//! Game playlists. Arguments start with the playlist id; `playlist.add` and `playlist.remove`
//! follow it with a place id, or `playlist.add` takes the experience page it ran from.

use std::sync::Arc;

use bonsai_core::commands::CommandId;
use bonsai_core::{AccountId, Core, playlists};
use serde::Serialize;
use specta::Type;
use tauri::State;

use super::{GameCard, Id, Invocation, viewer};

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistView {
    pub id: Id,
    pub name: String,
    pub games: Vec<GameCard>,
}

/// The active account's playlists, oldest first. Games show cached details, or the name saved
/// with them until details load.
#[tauri::command]
#[specta::specta]
pub fn playlists(core: State<'_, Arc<Core>>) -> Vec<PlaylistView> {
    let account = viewer(&core);
    let lists = core.playlists(account);
    let mut universes = Vec::new();
    let mut places = Vec::new();
    let view = {
        let s = core.store();
        lists
            .iter()
            .map(|p| PlaylistView {
                id: Id(p.id),
                name: p.name.clone(),
                games: p
                    .games
                    .iter()
                    .map(|g| match s.game_for_place(g.place_id) {
                        Some(info) => GameCard {
                            place_id: Id(g.place_id),
                            ..GameCard::from(info)
                        },
                        None => {
                            if g.universe_id > 0 {
                                universes.push(g.universe_id);
                            } else {
                                places.push(g.place_id);
                            }
                            GameCard {
                                universe_id: Id(g.universe_id),
                                place_id: Id(g.place_id),
                                name: g.name.clone(),
                                creator: String::new(),
                                playing: 0,
                                rating: None,
                                sponsored: false,
                                genre: String::new(),
                            }
                        }
                    })
                    .collect(),
            })
            .collect()
    };
    core.want_universes(&universes);
    for p in places {
        core.ensure_place(account, p);
    }
    view
}

fn ids(arg: Option<&str>) -> (Option<u64>, Option<u64>, String) {
    let raw = arg.unwrap_or("").trim();
    let (first, rest) = raw.split_once(char::is_whitespace).unwrap_or((raw, ""));
    let rest = rest.trim();
    (first.parse().ok(), rest.parse().ok(), rest.to_owned())
}

fn name_of(core: &Core, account: AccountId, id: u64) -> String {
    core.playlists(account)
        .into_iter()
        .find(|p| p.id == id)
        .map(|p| p.name)
        .unwrap_or_default()
}

pub fn run(
    core: &Arc<Core>,
    id: CommandId,
    inv: &Invocation,
    acting: Option<AccountId>,
) -> Option<Result<(), String>> {
    let acting = || acting.ok_or_else(|| "no account".to_owned());
    let (list, place, rest) = ids(inv.arg.as_deref());
    let list = || list.ok_or_else(|| "Which playlist?".to_owned());
    Some(match id {
        CommandId::NewPlaylist => (|| {
            let a = acting()?;
            let name = inv.arg.clone().unwrap_or_default();
            core.edit_playlists(a, |all| playlists::create(all, &name))?;
            core.notice(format!("Made playlist {}.", name.trim()), false);
            Ok(())
        })(),
        CommandId::AddToPlaylist => (|| {
            let a = acting()?;
            let l = list()?;
            let place = place
                .or_else(|| {
                    inv.context
                        .as_deref()
                        .and_then(|c| c.strip_prefix("game:"))
                        .and_then(|p| p.parse().ok())
                })
                .ok_or_else(|| "Open an experience first.".to_owned())?;
            let added = core.add_to_playlist(a, l, place)?;
            let name = name_of(core, a, l);
            let text = if added {
                format!("Added to {name}.")
            } else {
                format!("Already in {name}.")
            };
            core.notice(text, false);
            Ok(())
        })(),
        CommandId::RemoveFromPlaylist => (|| {
            let a = acting()?;
            let l = list()?;
            let place = place.ok_or_else(|| "Which experience?".to_owned())?;
            core.edit_playlists(a, |all| playlists::remove(all, l, place))
        })(),
        CommandId::RenamePlaylist => (|| {
            let a = acting()?;
            let l = list()?;
            core.edit_playlists(a, |all| playlists::rename(all, l, &rest))
        })(),
        CommandId::DeletePlaylist => (|| {
            let a = acting()?;
            let l = list()?;
            core.edit_playlists(a, |all| playlists::delete(all, l))
        })(),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_playlist_and_rest() {
        assert_eq!(ids(Some("3 1818")), (Some(3), Some(1818), "1818".to_owned()));
        assert_eq!(ids(Some(" 3  New name ")), (Some(3), None, "New name".to_owned()));
        assert_eq!(ids(Some("3")), (Some(3), None, String::new()));
        assert_eq!(ids(None), (None, None, String::new()));
    }
}
