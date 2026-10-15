//! Discover: the site's Charts (sorts with device and location filters, "see all"), the
//! signed-in Home feed, and favorite / followed experiences.

use rbx_api::AccountId;
use rbx_api::generated::{apis, followings, games};

use crate::Core;
use crate::res::{Key, Page, Res};
use crate::store::{ChartFilter, GameCard, HomeSort};

const AGE: i64 = 600;

/// The Home feed topic with games the account played recently, here or on the website.
pub const CONTINUE_TOPIC: &str = "100000003";

impl Core {
    /// Chart sorts for a device and location, a few at a time. The first page also has a
    /// sort with `filters` and no games: the filter options.
    pub fn charts(&self, account: AccountId, device: &str, country: &str, more: bool) -> Res<Page<HomeSort>> {
        let (device, country) = (device.to_string(), country.to_string());
        self.inner.paged(
            Key::private(account, "charts", format!("{device}:{country}")),
            AGE,
            more,
            move |f, cursor| {
                let session = f.inner.session_id.clone();
                let (device, country) = (device.clone(), country.clone());
                async move {
                    let p = apis::undocumented::GetExploreSortsParams {
                        session_id: session,
                        device: Some(device),
                        country: Some(country),
                        sorts_page_token: cursor,
                    };
                    let r = apis::undocumented::get_explore_sorts(&f.client, &p).await?;
                    Ok(Page {
                        items: r.sorts.unwrap_or_default().into_iter().filter_map(sort).collect(),
                        next: r.next_sorts_page_token.filter(|t| !t.is_empty()),
                    })
                }
            },
        )
    }

    /// Every game in one chart ("see all").
    pub fn chart(&self, account: AccountId, sort_id: &str, device: &str, country: &str) -> Res<HomeSort> {
        let (id, device, country) = (sort_id.to_string(), device.to_string(), country.to_string());
        self.inner.res(
            Key::private(account, "chart", format!("{id}:{device}:{country}")),
            AGE,
            move |f| async move {
                let p = apis::undocumented::GetExploreSortContentParams {
                    session_id: f.inner.session_id.clone(),
                    sort_id: id,
                    device: Some(device),
                    country: Some(country),
                };
                let r = apis::undocumented::get_explore_sort_content(&f.client, &p).await?;
                Ok(sort(r).unwrap_or_default())
            },
        )
    }

    /// The signed-in Home feed (Recommended For You, Continue, Today's Picks, ...), in the
    /// site's order. Empty when signed out.
    pub fn home_feed(&self, account: AccountId) -> Res<Vec<HomeSort>> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner
            .res(Key::private(account, "home_feed", ""), AGE, move |f| async move {
                let body = apis::undocumented::OmniRecommendationRequest {
                    page_type: Some("Home".into()),
                    session_id: Some(f.inner.session_id.clone()),
                };
                let r = apis::undocumented::post_omni_recommendation(&f.client, &body).await?;
                let meta = r.content_metadata.and_then(|m| m.game).unwrap_or_default();
                let mut out: Vec<HomeSort> = Vec::new();
                for t in r.sorts.unwrap_or_default() {
                    let games: Vec<GameCard> = t
                        .recommendation_list
                        .unwrap_or_default()
                        .into_iter()
                        .filter(|i| i.content_type.as_deref() == Some("Game"))
                        .filter_map(|i| meta.get(&i.content_id?.to_string()).cloned())
                        .filter_map(GameCard::from_api)
                        .filter(|g| !g.sponsored)
                        .collect();
                    let title = t.topic.unwrap_or_default();
                    if games.is_empty() || out.iter().any(|s| s.title == title) {
                        continue;
                    }
                    out.push(HomeSort {
                        id: t.topic_id.unwrap_or(0).to_string(),
                        title,
                        games,
                        ..Default::default()
                    });
                }
                Ok(out)
            })
    }

    /// Experiences a user favorited.
    pub fn favorite_games(&self, user_id: u64, more: bool) -> Res<Page<GameCard>> {
        self.inner.paged(
            Key::shared("favorite_games", user_id),
            AGE,
            more,
            move |f, cursor| async move {
                let p = games::v2::GetUsersByUserIdFavoriteGamesParams {
                    limit: Some(50),
                    cursor,
                    ..Default::default()
                };
                // Deprecated in the tracker with no replacement; the site still uses it (DECISIONS #38).
                #[allow(deprecated)]
                let r = games::v2::get_users_by_user_id_favorite_games(&f.client, user_id as i64, &p).await?;
                Ok(Page {
                    items: r
                        .data
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|g| {
                            Some(GameCard {
                                universe_id: g.id? as u64,
                                place_id: g.root_place.and_then(|p| p.id)? as u64,
                                name: g.name.unwrap_or_default(),
                                creator: g.creator.and_then(|c| c.name).unwrap_or_default(),
                                ..Default::default()
                            })
                        })
                        .collect(),
                    next: r.next_page_cursor,
                })
            },
        )
    }

    /// Experiences you follow (notifications on). Only your own list is visible.
    pub fn followed_games(&self, account: AccountId) -> Res<Vec<GameCard>> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "followed_games", ""),
            AGE,
            move |f| async move {
                let r = followings::v2::get_users_by_user_id_universes(&f.client, account.0 as i64).await?;
                let mut ids: Vec<i64> = r
                    .followed_sources
                    .unwrap_or_default()
                    .keys()
                    .filter_map(|k| k.parse().ok())
                    .collect();
                if ids.is_empty() {
                    return Ok(Vec::new());
                }
                ids.sort_unstable();
                ids.truncate(50);
                let p = games::v1::GetGamesParams { universe_ids: ids };
                let g = games::v1::get_games(&f.client, &p).await?;
                Ok(g.data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|g| {
                        Some(GameCard {
                            universe_id: g.id? as u64,
                            place_id: g.root_place_id? as u64,
                            name: g.name.unwrap_or_default(),
                            playing: g.playing.unwrap_or(0),
                            creator: g.creator.and_then(|c| c.name).unwrap_or_default(),
                            genre: g.genre.unwrap_or_default(),
                            ..Default::default()
                        })
                    })
                    .collect())
            },
        )
    }
}

fn sort(s: apis::undocumented::ExploreSort) -> Option<HomeSort> {
    let filters: Vec<ChartFilter> = s
        .filters
        .unwrap_or_default()
        .into_iter()
        .map(|f| ChartFilter {
            kind: f.filter_type.unwrap_or_default(),
            title: f.filter_display_name.unwrap_or_default(),
            selected: f.selected_option_id.unwrap_or_default(),
            options: f
                .filter_options
                .unwrap_or_default()
                .into_iter()
                .filter_map(|o| Some((o.option_id?, o.option_display_name.unwrap_or_default())))
                .collect(),
        })
        .collect();
    let games: Vec<GameCard> = s
        .games
        .unwrap_or_default()
        .into_iter()
        .filter_map(GameCard::from_api)
        .filter(|g| !g.sponsored)
        .collect();
    if games.is_empty() && filters.is_empty() {
        return None;
    }
    Some(HomeSort {
        id: s.sort_id.unwrap_or_default(),
        title: s.sort_display_name.unwrap_or_default(),
        subtitle: s.subtitle.unwrap_or_default(),
        info: s.topic_layout_data.and_then(|t| t.info_text).unwrap_or_default(),
        games,
        filters,
    })
}
