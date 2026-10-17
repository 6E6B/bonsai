//! What the UI renders from. Background tasks write here and request a repaint; the UI reads
//! under a short read lock every frame and never waits on the network.

use std::collections::HashMap;
use std::sync::Arc;

use bonsai_api::AccountId;
use serde::{Deserialize, Serialize};

use crate::account::{AccountList, AccountMeta};
use crate::thumbs::{ThumbKey, ThumbState};

/// A value that may be cached, loading, or failed — all at once. Screens render `data` when it
/// exists and show `loading`/`error` without covering it.
#[derive(Clone, Debug)]
pub struct Loadable<T> {
    pub data: Option<T>,
    pub loading: bool,
    pub error: Option<String>,
    pub fetched_at: i64,
}

impl<T> Default for Loadable<T> {
    fn default() -> Self {
        Self {
            data: None,
            loading: false,
            error: None,
            fetched_at: 0,
        }
    }
}

impl<T> Loadable<T> {
    pub fn cached(data: T, fetched_at: i64) -> Self {
        Self {
            data: Some(data),
            loading: false,
            error: None,
            fetched_at,
        }
    }
    pub fn start(&mut self) {
        self.loading = true;
    }
    pub fn done(&mut self, data: T) {
        self.data = Some(data);
        self.loading = false;
        self.error = None;
        self.fetched_at = crate::cache::now();
    }
    pub fn fail(&mut self, e: String) {
        self.loading = false;
        self.error = Some(e);
    }
    pub fn age_secs(&self) -> i64 {
        crate::cache::now() - self.fetched_at
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    pub verified: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GameInfo {
    pub universe_id: u64,
    pub root_place_id: u64,
    pub name: String,
    pub description: String,
    pub creator_id: u64,
    pub creator_name: String,
    pub creator_is_group: bool,
    pub creator_verified: bool,
    pub playing: i64,
    pub visits: i64,
    pub favorites: i64,
    pub max_players: i32,
    pub created: String,
    pub updated: String,
    pub genre: String,
    pub price: Option<i64>,
    pub private_servers_allowed: bool,
    pub up_votes: Option<i64>,
    pub down_votes: Option<i64>,
    /// The site's genre and subgenre, like "Roleplay & Avatar Sim" / "Pet Care".
    #[serde(default)]
    pub genre_l1: String,
    #[serde(default)]
    pub genre_l2: String,
    /// Others may copy the place in Studio.
    #[serde(default)]
    pub copying_allowed: bool,
    /// "R6", "R15" or "Player choice"; empty when unknown.
    #[serde(default)]
    pub avatar_type: String,
    /// Gear categories players may bring in.
    #[serde(default)]
    pub gear: Vec<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PresenceKind {
    #[default]
    Offline,
    Online,
    InGame,
    InStudio,
    Invisible,
}

impl PresenceKind {
    pub fn from_code(c: i64) -> PresenceKind {
        match c {
            1 => PresenceKind::Online,
            2 => PresenceKind::InGame,
            3 => PresenceKind::InStudio,
            4 => PresenceKind::Invisible,
            _ => PresenceKind::Offline,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            PresenceKind::Offline => "Offline",
            PresenceKind::Online => "Online",
            PresenceKind::InGame => "In game",
            PresenceKind::InStudio => "In Studio",
            PresenceKind::Invisible => "Invisible",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Presence {
    pub kind: PresenceKind,
    pub place_id: Option<u64>,
    pub root_place_id: Option<u64>,
    pub universe_id: Option<u64>,
    /// Server job id, when visible to this account.
    pub job_id: Option<String>,
    pub last_location: String,
}

impl Presence {
    pub(crate) fn from_api(p: bonsai_api::generated::presence::v1::UserPresence) -> Option<(u64, Presence)> {
        let uid = p.user_id? as u64;
        Some((
            uid,
            Presence {
                kind: PresenceKind::from_code(p.user_presence_type.unwrap_or(0) as i64),
                place_id: p.place_id.filter(|x| *x > 0).map(|x| x as u64),
                root_place_id: p.root_place_id.filter(|x| *x > 0).map(|x| x as u64),
                universe_id: p.universe_id.filter(|x| *x > 0).map(|x| x as u64),
                job_id: p.game_id.filter(|g| !g.is_empty()),
                last_location: p.last_location.unwrap_or_default(),
            },
        ))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Server {
    pub job_id: String,
    pub playing: i32,
    pub max_players: i32,
    pub ping: Option<i32>,
    pub fps: Option<f64>,
    /// Friends of the viewing account in this server (from presence).
    #[serde(default)]
    pub friends: Vec<u64>,
    #[serde(default)]
    pub player_tokens: Vec<String>,
}

impl Server {
    pub fn free_slots(&self) -> i32 {
        (self.max_players - self.playing).max(0)
    }
    pub fn is_full(&self) -> bool {
        self.free_slots() == 0
    }
}

/// How "join a server for me" chooses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServerPick {
    /// Most friends, then lowest ping.
    Best,
    /// Fewest players.
    Smallest,
    Random,
}

/// The server a pick lands on, among ones with room. Unknown ping counts as worst.
pub fn pick_server(list: &[Server], how: ServerPick) -> Option<&Server> {
    let ping = |s: &Server| s.ping.unwrap_or(i32::MAX);
    let open = list.iter().filter(|s| !s.is_full());
    match how {
        ServerPick::Best => open.min_by_key(|s| (std::cmp::Reverse(s.friends.len()), ping(s))),
        ServerPick::Smallest => open.min_by_key(|s| (s.playing, ping(s))),
        ServerPick::Random => {
            let open: Vec<&Server> = open.collect();
            (!open.is_empty()).then(|| open[fastrand::usize(..open.len())])
        }
    }
}

/// Unread counts for the rail badges.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unread {
    pub messages: i64,
    pub friend_requests: i64,
    pub trades: i64,
    pub notifications: i64,
    /// Chat conversations with unread messages.
    #[serde(default)]
    pub chat: i64,
}

impl Unread {
    pub fn total(&self) -> i64 {
        self.messages + self.friend_requests + self.trades + self.notifications + self.chat
    }
}

/// Session health of one account.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum SessionState {
    #[default]
    Unknown,
    Ok,
    /// 401: show "session expired, paste a new cookie" on this account only.
    Expired,
    /// The keychain had no cookie (e.g. Secret Service missing after restart).
    NoCookie,
}

/// Private, per-account data. Never shared with another account.
#[derive(Clone, Debug, Default)]
pub struct AccountData {
    pub session: SessionState,
    pub robux: Option<i64>,
    pub unread: Unread,
    pub friends: Loadable<Vec<u64>>,
    /// Presence as seen by this account.
    pub presence: HashMap<u64, Presence>,
    /// When this account last saw each user online (unix seconds). Local; Roblox doesn't say.
    pub last_seen: HashMap<u64, i64>,
    /// Friends who removed this account, and when it noticed (unix seconds). Local.
    pub unfriended: HashMap<u64, i64>,
    /// Friends this account removed, until the friends list stops showing them.
    pub removed_by_me: std::collections::HashSet<u64>,
    pub home: Loadable<Vec<HomeSort>>,
    pub recent: Vec<u64>,
    pub favorites: Loadable<Vec<u64>>,
    /// Local wishlist, newest first.
    pub wishlist: Vec<crate::market::WishItem>,
    pub last_launch: Option<String>,
    /// Connected to the realtime hub.
    pub realtime: bool,
    /// Who is typing, per conversation: user id -> until (unix ms). From realtime.
    pub typing: HashMap<String, HashMap<u64, i64>>,
    /// Chat messages sent from here that failed, per conversation, until resent or discarded.
    pub failed_chat: HashMap<String, Vec<crate::chat::ChatMessage>>,
    /// A conversation the UI should open (set after "Chat" on a profile creates one).
    pub chat_focus: Option<String>,
    /// The trade being put together. Not saved.
    pub trade_draft: Option<crate::trades::Draft>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HomeSort {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub games: Vec<GameCard>,
    /// What the sort means ("Top experiences with the largest ...").
    #[serde(default)]
    pub info: String,
    /// Charts send their filters as a sort of their own.
    #[serde(default)]
    pub filters: Vec<ChartFilter>,
}

/// A Charts filter (device, location) and its options as (id, label).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChartFilter {
    pub kind: String,
    pub title: String,
    pub selected: String,
    pub options: Vec<(String, String)>,
}

/// The few fields a game tile needs; comes with sorts and search results.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GameCard {
    pub universe_id: u64,
    pub place_id: u64,
    pub name: String,
    pub playing: i64,
    pub up_votes: i64,
    pub down_votes: i64,
    pub creator: String,
    pub sponsored: bool,
    #[serde(default)]
    pub genre: String,
}

impl GameCard {
    /// Like ratio in percent, if anyone voted.
    pub fn rating(&self) -> Option<u8> {
        let total = self.up_votes + self.down_votes;
        (total > 0).then(|| ((self.up_votes * 100 + total / 2) / total) as u8)
    }

    pub(crate) fn from_api(g: bonsai_api::generated::apis::undocumented::GameCard) -> Option<GameCard> {
        Some(GameCard {
            universe_id: g.universe_id? as u64,
            place_id: g.root_place_id? as u64,
            name: g.name.unwrap_or_default(),
            playing: g.player_count.unwrap_or(0),
            up_votes: g.total_up_votes.unwrap_or(0),
            down_votes: g.total_down_votes.unwrap_or(0),
            creator: g.creator_name.unwrap_or_default(),
            sponsored: g.is_sponsored.unwrap_or(false),
            genre: g.genre_l1.unwrap_or_default(),
        })
    }
}

/// Adding an account: a pasted cookie, quick sign-in or the sign-in window. All three end in
/// `Validating` → `Confirm`.
#[derive(Clone, Debug, PartialEq)]
pub enum LoginState {
    /// The sign-in window is open; the user signs in there.
    Browser,
    /// Quick sign-in: asking Roblox for a code.
    QuickStarting,
    /// Quick sign-in: the code to approve on a signed-in device.
    QuickCode {
        code: String,
        /// Unix seconds.
        expires_at: i64,
        /// The account that entered the code, before it is approved.
        linked: Option<String>,
    },
    Validating,
    /// Valid: show avatar and names, ask the user to confirm before saving.
    Confirm {
        user: UserInfo,
        already_added: bool,
    },
    Failed(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
    pub id: u64,
    pub account: Option<AccountId>,
    pub text: String,
    pub error: bool,
    pub host: Option<String>,
    pub retry_after_secs: Option<u32>,
}

/// A remote search result row (merged into the command bar).
#[derive(Clone, Debug, PartialEq)]
pub struct RemoteResults {
    pub query: String,
    pub rows: Vec<crate::search::Entry>,
}

/// Where a friend shows up in a friends list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FriendGroup {
    InGame,
    Online,
    Offline,
}

impl FriendGroup {
    pub fn label(self) -> &'static str {
        match self {
            FriendGroup::InGame => "In game",
            FriendGroup::Online => "Online",
            FriendGroup::Offline => "Offline",
        }
    }
}

/// One friend in a friends list (the drawer and the Friends page show the same rows).
#[derive(Clone, Debug, PartialEq)]
pub struct FriendRow {
    pub id: u64,
    pub title: String,
    /// @name without the @; empty until the user is known.
    pub name: String,
    pub kind: PresenceKind,
    /// The game's name in game, else the presence ("Online", "In Studio", "Offline").
    pub status: String,
    /// The experience's start place, for opening it. In game only.
    pub place_id: Option<u64>,
    pub universe_id: Option<u64>,
}

impl FriendRow {
    pub fn group(&self) -> FriendGroup {
        match self.kind {
            PresenceKind::InGame => FriendGroup::InGame,
            PresenceKind::Online | PresenceKind::InStudio => FriendGroup::Online,
            PresenceKind::Offline | PresenceKind::Invisible => FriendGroup::Offline,
        }
    }
}

#[derive(Default)]
pub struct StoreData {
    pub accounts: AccountList,
    pub data: HashMap<AccountId, AccountData>,
    // shared public data
    pub users: HashMap<u64, UserInfo>,
    pub games: HashMap<u64, GameInfo>,
    pub place_universe: HashMap<u64, u64>,
    pub servers: HashMap<u64, Loadable<Vec<Server>>>,
    pub server_cursor: HashMap<u64, Option<String>>,
    pub thumbs: HashMap<ThumbKey, ThumbState>,
    pub remote: Loadable<RemoteResults>,
    pub login: Option<LoginState>,
    /// The "Sign in another device" dialog (see [`crate::signin`]).
    pub approve: Option<crate::signin::ApprovePrompt>,
    /// A two-step or captcha check waiting for the user (see [`crate::challenge`]).
    pub challenge: Option<crate::challenge::ChallengePrompt>,
    /// A theme read from a file or a code, waiting for the user to install it.
    pub theme_import: Option<crate::themes::PendingImport>,
    /// A plugin folder waiting in the install dialog.
    pub plugin_install: Option<crate::plugins::PendingInstall>,
    /// Turning on the authenticator app or making recovery codes (see [`crate::security`]).
    pub security: Option<crate::security::SecurityFlow>,
    pub toasts: Vec<Toast>,
    pub jobs: u32,
    pub offline: bool,
    pub keychain_warning: Option<String>,
    /// Disk cache size, once measured (Settings → Advanced).
    pub cache_usage: Option<crate::me::CacheUsage>,
}

impl StoreData {
    pub fn active(&self) -> Option<&AccountMeta> {
        self.accounts.active.and_then(|id| self.accounts.get(id))
    }
    pub fn active_id(&self) -> Option<AccountId> {
        self.accounts.active
    }
    pub fn account_data(&self, id: AccountId) -> Option<&AccountData> {
        self.data.get(&id)
    }
    pub fn active_data(&self) -> Option<&AccountData> {
        self.active_id().and_then(|id| self.data.get(&id))
    }
    pub fn game_for_place(&self, place_id: u64) -> Option<&GameInfo> {
        self.place_universe.get(&place_id).and_then(|u| self.games.get(u))
    }
    pub fn thumb(&self, k: &ThumbKey) -> Option<&Arc<crate::thumbs::Image>> {
        match self.thumbs.get(k) {
            Some(ThumbState::Ready(i)) => Some(i),
            _ => None,
        }
    }
    /// Friends of `account` that are in a game, with presence.
    pub fn friends_in_game(&self, account: AccountId) -> Vec<(u64, Presence)> {
        let Some(d) = self.data.get(&account) else {
            return Vec::new();
        };
        let mut v: Vec<(u64, Presence)> = d
            .friends
            .data
            .iter()
            .flatten()
            .filter_map(|f| {
                d.presence
                    .get(f)
                    .filter(|p| p.kind == PresenceKind::InGame)
                    .map(|p| (*f, p.clone()))
            })
            .collect();
        v.sort_by_key(|(id, _)| *id);
        v
    }
    /// Friends of `account` as every friends list shows them: in game, in Studio, online, then
    /// offline, each by name. `filter` matches display name or @name, ignoring case.
    pub fn friend_rows(&self, account: AccountId, filter: &str) -> Vec<FriendRow> {
        let Some(d) = self.data.get(&account) else {
            return Vec::new();
        };
        let q = filter.trim().to_lowercase();
        let mut rows: Vec<FriendRow> = d
            .friends
            .data
            .iter()
            .flatten()
            .filter_map(|id| {
                let name = self.users.get(id).map(|u| u.name.clone()).unwrap_or_default();
                let title = self.user_title(*id);
                if !q.is_empty() && !name.to_lowercase().contains(&q) && !title.to_lowercase().contains(&q) {
                    return None;
                }
                let p = d.presence.get(id).cloned().unwrap_or_default();
                let status = match p.kind {
                    PresenceKind::InGame => p
                        .universe_id
                        .and_then(|u| self.games.get(&u))
                        .map(|g| g.name.clone())
                        .or_else(|| (!p.last_location.is_empty()).then(|| p.last_location.clone()))
                        .unwrap_or_else(|| "In a game".into()),
                    PresenceKind::Invisible => PresenceKind::Offline.label().to_string(),
                    k => k.label().to_string(),
                };
                Some(FriendRow {
                    id: *id,
                    title,
                    name,
                    kind: p.kind,
                    status,
                    place_id: p.root_place_id.or(p.place_id),
                    universe_id: p.universe_id,
                })
            })
            .collect();
        rows.sort_by(|a, b| {
            a.group()
                .cmp(&b.group())
                .then_with(|| a.title.to_lowercase().cmp(&b.title.to_lowercase()))
                .then_with(|| a.id.cmp(&b.id))
        });
        rows
    }
    pub fn user_title(&self, id: u64) -> String {
        self.users
            .get(&id)
            .map(|u| u.display_name.clone())
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| format!("User {id}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn server(job: &str, playing: i32, ping: Option<i32>, friends: usize) -> Server {
        Server {
            job_id: job.into(),
            playing,
            max_players: 10,
            ping,
            friends: (0..friends as u64).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn picks_servers() {
        let list = [
            server("full", 10, Some(5), 3),
            server("far", 2, Some(200), 0),
            server("near", 8, Some(40), 0),
            server("pals", 9, None, 1),
        ];
        let job = |how| pick_server(&list, how).map(|s| s.job_id.as_str());
        assert_eq!(
            job(ServerPick::Best),
            Some("pals"),
            "friends beat ping; full servers skipped"
        );
        assert_eq!(job(ServerPick::Smallest), Some("far"));
        for _ in 0..50 {
            assert!(job(ServerPick::Random).is_some_and(|j| j != "full"));
        }
        assert_eq!(pick_server(&list[..1], ServerPick::Random), None);
        let no_friends = &list[1..3];
        assert_eq!(
            pick_server(no_friends, ServerPick::Best).map(|s| s.job_id.as_str()),
            Some("near")
        );
    }

    fn store() -> (StoreData, AccountId) {
        let acc = AccountId(1);
        let mut s = StoreData::default();
        for (id, name, display) in [
            (10, "zed", "Zed"),
            (11, "amy", "Amy"),
            (12, "bob", "Bob"),
            (13, "cal", "Cal"),
        ] {
            s.users.insert(
                id,
                UserInfo {
                    id,
                    name: name.into(),
                    display_name: display.into(),
                    ..Default::default()
                },
            );
        }
        let mut d = AccountData::default();
        d.friends.data = Some(vec![10, 11, 12, 13]);
        let p = |kind, loc: &str| Presence {
            kind,
            last_location: loc.into(),
            ..Default::default()
        };
        d.presence.insert(10, p(PresenceKind::InGame, "Crossroads"));
        d.presence.insert(11, p(PresenceKind::Offline, ""));
        d.presence.insert(12, p(PresenceKind::InStudio, ""));
        d.presence.insert(13, p(PresenceKind::Online, "Website"));
        s.data.insert(acc, d);
        (s, acc)
    }

    #[test]
    fn friend_rows_group_then_sort_by_name() {
        let (s, acc) = store();
        let rows = s.friend_rows(acc, "");
        let order: Vec<(u64, FriendGroup, &str)> = rows
            .iter()
            .map(|r| (r.id, r.group(), r.status.as_str()))
            .collect();
        assert_eq!(
            order,
            vec![
                (10, FriendGroup::InGame, "Crossroads"),
                (12, FriendGroup::Online, "In Studio"),
                (13, FriendGroup::Online, "Online"),
                (11, FriendGroup::Offline, "Offline"),
            ]
        );
    }

    #[test]
    fn friend_rows_filter_by_either_name() {
        let (s, acc) = store();
        assert_eq!(
            s.friend_rows(acc, "AM").iter().map(|r| r.id).collect::<Vec<_>>(),
            vec![11]
        );
        assert_eq!(
            s.friend_rows(acc, " cal ")
                .iter()
                .map(|r| r.id)
                .collect::<Vec<_>>(),
            vec![13]
        );
        assert!(s.friend_rows(acc, "nobody").is_empty());
    }
}
