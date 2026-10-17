//! Typed IPC between the UI and `bonsai-core`. Every command and event here is exported to
//! `ui/src/lib/ipc/bindings.ts` by tauri-specta; never call `invoke()` by hand in the UI.
//!
//! Commands run on the main thread, so they only take short locks or send to `bonsai-core`.
//! Types here are UI-facing views of `bonsai-core` data, not the store itself.

use std::sync::Arc;

use bonsai_core::commands::{ArgKind, CommandId, CommandRef, Registry, Shortcut};
use bonsai_core::discover::CONTINUE_TOPIC;
use bonsai_core::experience::ServerChange;
use bonsai_core::launcher::JoinTarget;
use bonsai_core::links;
use bonsai_core::people::FriendState;
use bonsai_core::res::{Page, Res};
use bonsai_core::store::{LoginState, PresenceKind, ServerPick, SessionState};
use bonsai_core::{AccountId, Cmd, ColorTag, Core};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager, State};
use tauri_specta::{Builder, Event, collect_commands, collect_events};

mod account;
mod approve;
mod avatar;
mod challenge;
mod communities;
mod discover;
mod inbox;
mod market;
mod plugins;
mod security;
mod trades;
mod transactions;

/// A Roblox user, game or item id, or a local id. Roblox ids stay far below 2^53, so a
/// JS number holds them exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(transparent)]
#[specta(transparent)]
pub struct Id(#[specta(type = specta_typescript::Number)] pub u64);

/// Sent when the store changed. The UI refetches what it shows.
#[derive(Clone, Debug, Serialize, Deserialize, Type, Event)]
pub struct StoreChanged;

/// Shell state every screen needs: accounts, toasts, connectivity.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Shell {
    pub accounts: Vec<Account>,
    pub active: Option<Id>,
    pub offline: bool,
    pub jobs: u32,
    pub toasts: Vec<Toast>,
    pub keychain_warning: Option<String>,
    /// Robux of the active account, once known.
    pub robux: Option<f64>,
    /// Unread counts of the active account.
    pub unread: Unread,
    /// The add-account flow, while it is open.
    pub login: Option<Login>,
    /// A two-step or captcha check holding an action, while it is open.
    pub challenge: Option<challenge::ChallengeView>,
    /// A theme waiting in the import dialog.
    pub theme_import: Option<account::ThemeImportView>,
    /// A plugin folder waiting in the install dialog.
    pub plugin_install: Option<plugins::PluginInstallView>,
    /// Actions a plugin proposed, waiting in the action dialog.
    pub plugin_actions: Option<plugins::PluginActionsView>,
    /// A screen a plugin asked to open. The UI runs it once per `seq`.
    pub plugin_open: Option<plugins::PluginOpenView>,
    /// Changes when plugin commands do; the UI reloads the command list.
    pub plugin_rev: Id,
    /// The account in the "Allow automation" dialog, while it is open.
    pub plugin_automation: Option<plugins::AutomationView>,
    /// When a plugin last wrote, in Unix seconds, for the stop indicator.
    pub plugin_wrote: Option<Id>,
    /// The "Sign in another device" dialog, once a code was entered.
    pub approve: Option<approve::ApproveView>,
    /// The authenticator setup or recovery codes dialog, while it is open.
    pub security: Option<security::SecurityFlowView>,
    /// Item values from Rolimons are on.
    pub item_values: bool,
    /// Avatar viewers open in 3D instead of on the image.
    pub avatar_3d: bool,
    /// Window zoom, 1 = 100%.
    pub ui_scale: f64,
    /// How the user arranged the sidebar.
    pub sidebar: SidebarView,
    pub look: Look,
}

/// Display settings the whole window follows. Values as `settings.set` takes them.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Look {
    /// `comfortable` or `compact`.
    pub density: String,
    /// BCP 47 tag for numbers and dates; empty follows the system.
    pub format_locale: String,
    /// `system`, `reduce` or `full`.
    pub motion: String,
    /// `auto`, `12h` or `24h`.
    pub time_format: String,
    pub relative_dates: bool,
    pub status_bar: bool,
    /// A screen name, or `last` to reopen the tabs saved with `save_tabs`.
    pub start_screen: String,
    /// `auto`, `expanded` or `collapsed`.
    pub sidebar_mode: String,
    pub drawer: bool,
    pub home_friends: bool,
    pub home_continue: bool,
    pub home_favorites: bool,
    pub home_recommended: bool,
    /// Use the active account's color tag as the accent.
    pub account_accent: bool,
}

/// Sidebar order and hidden screens, named by `nav.*` command without the prefix.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SidebarView {
    pub order: Vec<String>,
    pub hidden: Vec<String>,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Unread {
    pub messages: u32,
    pub friend_requests: u32,
    pub trades: u32,
    pub notifications: u32,
    pub chat: u32,
}

impl From<bonsai_core::store::Unread> for Unread {
    fn from(u: bonsai_core::store::Unread) -> Self {
        let n = |x: i64| x.clamp(0, u32::MAX as i64) as u32;
        Unread {
            messages: n(u.messages),
            friend_requests: n(u.friend_requests),
            trades: n(u.trades),
            notifications: n(u.notifications),
            chat: n(u.chat),
        }
    }
}

/// Where adding an account is (pasted cookie, quick sign-in or the sign-in window). The cookie
/// itself never comes back to the UI.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Login {
    /// The sign-in window is open.
    Browser,
    QuickStarting,
    /// Quick sign-in code to approve on another device. `expiresAt` is unix seconds.
    #[serde(rename_all = "camelCase")]
    QuickCode {
        code: String,
        expires_at: Id,
        linked: Option<String>,
    },
    Validating,
    #[serde(rename_all = "camelCase")]
    Confirm {
        user: User,
        already_added: bool,
    },
    Failed {
        message: String,
    },
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Id,
    pub name: String,
    pub display_name: String,
}

/// Whether an account's saved cookie still works.
#[derive(Clone, Copy, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum Session {
    Unknown,
    Ok,
    Expired,
    NoCookie,
}

impl From<&SessionState> for Session {
    fn from(s: &SessionState) -> Self {
        match s {
            SessionState::Unknown => Session::Unknown,
            SessionState::Ok => Session::Ok,
            SessionState::Expired => Session::Expired,
            SessionState::NoCookie => Session::NoCookie,
        }
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: Id,
    pub name: String,
    pub display_name: String,
    pub label: Option<String>,
    pub color: Color,
    pub session: Session,
    /// Everything unread for this account (messages, chat, friend requests, trades,
    /// notifications), for the rail badge.
    pub unread: u32,
}

#[derive(Clone, Copy, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum Color {
    None,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
}

impl From<ColorTag> for Color {
    fn from(c: ColorTag) -> Self {
        match c {
            ColorTag::None => Color::None,
            ColorTag::Red => Color::Red,
            ColorTag::Orange => Color::Orange,
            ColorTag::Yellow => Color::Yellow,
            ColorTag::Green => Color::Green,
            ColorTag::Blue => Color::Blue,
            ColorTag::Purple => Color::Purple,
            ColorTag::Pink => Color::Pink,
        }
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Toast {
    pub id: Id,
    pub account: Option<Id>,
    pub text: String,
    pub error: bool,
    pub host: Option<String>,
    pub retry_after_secs: Option<u32>,
}

/// One entry of the command registry, for the palette, menus and shortcut handling.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CommandInfo {
    /// Stable key, like `nav.home`.
    pub id: String,
    pub title: String,
    pub keywords: Vec<String>,
    pub arg: Arg,
    pub shortcuts: Vec<Keys>,
    /// The user changed the shortcuts from the defaults.
    pub rebound: bool,
    /// For plugin commands: the menus it shows in (`user`, `game`, `item` or `group`).
    pub context: Option<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum Arg {
    None,
    Account,
    GameOrFriend,
    User,
    SettingsSection,
    Text,
    Color,
    Outfit,
    Mode,
    Theme,
    Font,
    Tab,
    Screen,
    Setting,
    Plugin,
}

impl From<ArgKind> for Arg {
    fn from(a: ArgKind) -> Self {
        match a {
            ArgKind::None => Arg::None,
            ArgKind::Account => Arg::Account,
            ArgKind::GameOrFriend => Arg::GameOrFriend,
            ArgKind::User => Arg::User,
            ArgKind::SettingsSection => Arg::SettingsSection,
            ArgKind::Text => Arg::Text,
            ArgKind::Color => Arg::Color,
            ArgKind::Outfit => Arg::Outfit,
            ArgKind::Mode => Arg::Mode,
            ArgKind::Theme => Arg::Theme,
            ArgKind::Font => Arg::Font,
            ArgKind::Tab => Arg::Tab,
            ArgKind::Screen => Arg::Screen,
            ArgKind::Setting => Arg::Setting,
            ArgKind::Plugin => Arg::Plugin,
        }
    }
}

/// A shortcut. Key names follow `bonsai-core` (`"K"`, `"1"`, `"Slash"`, `"PageDown"`).
#[derive(Clone, Debug, Serialize, Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Keys {
    Chord {
        ctrl: bool,
        shift: bool,
        alt: bool,
        key: String,
        label: String,
    },
    Sequence {
        first: String,
        second: String,
        label: String,
    },
}

impl From<&Shortcut> for Keys {
    fn from(s: &Shortcut) -> Self {
        let label = s.label();
        match s.clone() {
            Shortcut::Chord {
                ctrl,
                shift,
                alt,
                key,
            } => Keys::Chord {
                ctrl,
                shift,
                alt,
                key,
                label,
            },
            Shortcut::Sequence(first, second) => Keys::Sequence { first, second, label },
        }
    }
}

/// A request to run a command.
#[derive(Clone, Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Invocation {
    pub id: String,
    pub arg: Option<String>,
    pub as_account: Option<Id>,
    /// The page it ran from, for commands about "this" thing: `game:<placeId>`.
    pub context: Option<String>,
}

/// How a user shows up: matches the `Presence` type of the UI's `Avatar`.
#[derive(Clone, Copy, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum Presence {
    Offline,
    Online,
    Game,
    Studio,
}

impl From<PresenceKind> for Presence {
    fn from(k: PresenceKind) -> Self {
        match k {
            PresenceKind::Online => Presence::Online,
            PresenceKind::InGame => Presence::Game,
            PresenceKind::InStudio => Presence::Studio,
            PresenceKind::Offline | PresenceKind::Invisible => Presence::Offline,
        }
    }
}

/// The active account's friends, in the order every friends list shows them.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Friends {
    pub loading: bool,
    pub error: Option<String>,
    pub friends: Vec<Friend>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Friend {
    pub id: Id,
    /// @name without the @; empty until the user is known.
    pub name: String,
    pub display_name: String,
    pub presence: Presence,
    /// The game's name in game, else the presence ("Online", "In Studio", "Offline").
    pub status: String,
    /// The experience's start place, for opening it. In game only.
    pub place_id: Option<Id>,
    /// For the experience's icon.
    pub universe_id: Option<Id>,
    /// What the active account calls them in this app.
    pub nickname: Option<String>,
}

/// What Home shows for the active account (or signed out).
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Home {
    pub loading: bool,
    pub error: Option<String>,
    /// Games this account launched from here, newest first.
    pub recent: Vec<GameCard>,
    /// Experiences this account favorited.
    pub favorites: Vec<GameCard>,
    pub sorts: Vec<Sort>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Sort {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub games: Vec<GameCard>,
}

/// The few fields a game tile needs.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GameCard {
    pub universe_id: Id,
    pub place_id: Id,
    pub name: String,
    pub creator: String,
    pub playing: u32,
    /// Like ratio in percent, if anyone voted.
    pub rating: Option<u8>,
    pub sponsored: bool,
    /// Empty when unknown.
    pub genre: String,
}

impl From<&bonsai_core::store::GameCard> for GameCard {
    fn from(g: &bonsai_core::store::GameCard) -> Self {
        GameCard {
            universe_id: Id(g.universe_id),
            place_id: Id(g.place_id),
            name: g.name.clone(),
            creator: g.creator.clone(),
            playing: g.playing.clamp(0, u32::MAX as i64) as u32,
            rating: g.rating(),
            sponsored: g.sponsored,
            genre: g.genre.clone(),
        }
    }
}

impl From<&bonsai_core::store::GameInfo> for GameCard {
    fn from(g: &bonsai_core::store::GameInfo) -> Self {
        let up = g.up_votes.unwrap_or(0);
        let total = up + g.down_votes.unwrap_or(0);
        GameCard {
            universe_id: Id(g.universe_id),
            place_id: Id(g.root_place_id),
            name: g.name.clone(),
            creator: g.creator_name.clone(),
            playing: g.playing.clamp(0, u32::MAX as i64) as u32,
            rating: (total > 0).then(|| ((up * 100 + total / 2) / total) as u8),
            sponsored: false,
            genre: g.genre.clone(),
        }
    }
}

/// An experience page, by place id: everything above its tabs.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub place_id: Id,
    /// None until the place's experience is known.
    pub details: Option<GameDetails>,
    /// The active account's vote, favorite and notify state. None signed out or loading.
    pub me: Option<MyGame>,
    /// "Mild", "Moderate", …; None until known.
    pub maturity: Option<String>,
    /// What the rating is for: "Mild Violence", "Paid Random Items", ….
    pub maturity_descriptors: Vec<String>,
    pub media: Vec<Media>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GameDetails {
    pub universe_id: Id,
    pub root_place_id: Id,
    pub name: String,
    pub description: String,
    pub creator_id: Id,
    pub creator_name: String,
    pub creator_is_group: bool,
    pub creator_verified: bool,
    #[specta(type = specta_typescript::Number)]
    pub playing: u64,
    #[specta(type = specta_typescript::Number)]
    pub visits: u64,
    #[specta(type = specta_typescript::Number)]
    pub favorites: u64,
    #[specta(type = Option<specta_typescript::Number>)]
    pub up_votes: Option<u64>,
    #[specta(type = Option<specta_typescript::Number>)]
    pub down_votes: Option<u64>,
    pub max_players: u32,
    /// ISO dates.
    pub created: String,
    pub updated: String,
    pub genre: String,
    /// The site's genre and subgenre; empty when unset.
    pub genre_l1: String,
    pub genre_l2: String,
    pub copying_allowed: bool,
    pub private_servers_allowed: bool,
    /// "R6", "R15" or "Player choice"; empty when unknown.
    pub avatar_type: String,
    /// Gear categories players may bring in.
    pub gear: Vec<String>,
}

impl From<&bonsai_core::store::GameInfo> for GameDetails {
    fn from(g: &bonsai_core::store::GameInfo) -> Self {
        GameDetails {
            universe_id: Id(g.universe_id),
            root_place_id: Id(g.root_place_id),
            name: g.name.clone(),
            description: g.description.clone(),
            creator_id: Id(g.creator_id),
            creator_name: g.creator_name.clone(),
            creator_is_group: g.creator_is_group,
            creator_verified: g.creator_verified,
            playing: g.playing.max(0) as u64,
            visits: g.visits.max(0) as u64,
            favorites: g.favorites.max(0) as u64,
            up_votes: g.up_votes.map(|v| v.max(0) as u64),
            down_votes: g.down_votes.map(|v| v.max(0) as u64),
            max_players: g.max_players.max(0) as u32,
            created: g.created.clone(),
            updated: g.updated.clone(),
            genre: g.genre.clone(),
            genre_l1: g.genre_l1.clone(),
            genre_l2: g.genre_l2.clone(),
            copying_allowed: g.copying_allowed,
            private_servers_allowed: g.private_servers_allowed,
            avatar_type: g.avatar_type.clone(),
            gear: g.gear.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MyGame {
    pub can_vote: bool,
    /// true liked, false disliked.
    pub vote: Option<bool>,
    /// Why voting is off, in plain words.
    pub vote_hint: String,
    pub favorited: bool,
    /// The "Notify" bell.
    pub following: bool,
    pub can_follow: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    /// Shown through `thumb://game_thumbnail`.
    pub image_id: Id,
    pub video: bool,
    /// Empty for images.
    pub video_title: String,
    pub alt: String,
}

/// A list on one of the experience tabs.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct List<T> {
    pub loading: bool,
    pub error: Option<String>,
    pub items: Vec<T>,
    /// More pages exist.
    pub more: bool,
}

impl<T> List<T> {
    fn loading() -> Self {
        List {
            loading: true,
            error: None,
            items: Vec::new(),
            more: false,
        }
    }

    fn of<S>(r: Res<Vec<S>>, f: impl FnMut(&S) -> T) -> Self {
        List {
            loading: r.loading,
            error: r.error,
            items: r.data.map(|d| d.iter().map(f).collect()).unwrap_or_default(),
            more: false,
        }
    }

    fn paged<S>(r: Res<Page<S>>, f: impl FnMut(&S) -> T) -> Self {
        List {
            loading: r.loading,
            error: r.error,
            more: r.data.as_ref().is_some_and(|p| p.next.is_some()),
            items: r
                .data
                .map(|d| d.items.iter().map(f).collect())
                .unwrap_or_default(),
        }
    }
}

/// A place's public servers.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Servers {
    pub loading: bool,
    pub error: Option<String>,
    /// When the list was fetched, in ms since the epoch.
    pub fetched_at: Option<f64>,
    pub servers: Vec<ServerRow>,
    /// More pages exist.
    pub more: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ServerRow {
    pub job_id: String,
    pub playing: u32,
    pub max_players: u32,
    pub ping: Option<u32>,
    pub fps: Option<f64>,
    /// Friends of the active account in it.
    pub friends: Vec<Id>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PrivateServerRow {
    pub id: Id,
    pub name: String,
    pub owner_id: Id,
    /// Empty until the owner is known.
    pub owner_name: String,
    pub mine: bool,
    pub playing: u32,
    pub max_players: u32,
}

/// A user's profile as the active account sees it.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ProfileView {
    pub loading: bool,
    pub error: Option<String>,
    pub user: Option<ProfileHead>,
    pub presence: Presence,
    /// The experience they're in, when visible.
    pub place: Option<String>,
    pub place_id: Option<Id>,
    /// For the experience's icon.
    pub universe_id: Option<Id>,
    /// `None` on your own profiles and when signed out.
    pub relation: Option<RelationView>,
    /// One of your accounts.
    pub mine: bool,
    /// Usernames they had before, newest first.
    pub past_names: Vec<String>,
    /// When they were last seen online (unix ms): by this app for the active account, or by
    /// Rolimons when values are on and its sighting is newer.
    pub last_online: Option<f64>,
    pub last_online_by: Option<SeenBy>,
    /// Where Rolimons saw them ("Website"), when the sighting is Rolimons'.
    pub last_location: Option<String>,
    /// When the active account noticed they removed it as a friend (unix ms).
    pub unfriended_at: Option<f64>,
    /// Friends they share with the active account, among the pages of their friends loaded so
    /// far. `None` on your own profiles, when signed out, or before the first page loads.
    pub mutual_friends: Option<u32>,
    /// More pages of their friends exist, so there may be more mutual friends.
    pub mutual_more: bool,
    /// Visits across their public experiences. `None` until loaded.
    pub visits: Option<f64>,
    /// Favorites across their public experiences. `None` until every experience's details
    /// have loaded.
    pub favorites_received: Option<f64>,
    /// What the active account calls them, and its private note about them.
    pub nickname: Option<String>,
    pub note: Option<String>,
}

/// Who saw a user online last.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum SeenBy {
    App,
    Rolimons,
}

/// Rolimons' trading stats for a profile.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TraderView {
    /// Item values from Rolimons are on. Nothing else is set while off.
    pub enabled: bool,
    pub loading: bool,
    pub error: Option<String>,
    pub stats: Option<TraderStats>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TraderStats {
    /// Rolimons has scanned them; when false the rest is empty.
    pub tracked: bool,
    pub value: Option<f64>,
    pub rap: Option<f64>,
    /// Leaderboard place by value.
    pub rank: Option<u32>,
    pub premium: bool,
    /// Inventory hidden: the numbers are from before.
    pub private: bool,
    pub terminated: bool,
    /// When Rolimons last updated the numbers, unix ms.
    pub updated_at: Option<f64>,
    pub badges: Vec<TraderBadge>,
}

#[derive(Clone, Copy, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum TraderBadgeGroup {
    Value,
    Item,
    Collection,
    Trading,
    Community,
    Other,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TraderBadge {
    pub key: String,
    pub name: String,
    pub group: TraderBadgeGroup,
    /// Tiers of one goal share a family ("value" for 100K+ to 20M+), highest first. Empty for
    /// one-offs.
    pub family: String,
    /// unix ms.
    pub earned: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ProfileHead {
    pub id: Id,
    pub name: String,
    pub display_name: String,
    pub description: String,
    /// ISO date-time.
    pub created: String,
    pub verified: bool,
    pub banned: bool,
    /// Counts; `None` when Roblox didn't say.
    pub friends: Option<u32>,
    pub followers: Option<u32>,
    pub following: Option<u32>,
}

#[derive(Clone, Copy, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum FriendStatus {
    None,
    Friends,
    /// You sent them a request.
    Sent,
    /// They sent you one.
    Received,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RelationView {
    pub friend: FriendStatus,
    pub following: bool,
    pub followed_by: bool,
    pub blocked: bool,
}

/// A badge a user has.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct UserBadge {
    pub id: Id,
    pub name: String,
    pub description: String,
    /// Share of players who have it, 0 to 1.
    pub win_rate: f64,
    pub place_id: Id,
}

/// A community a user is in.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupCard {
    pub id: Id,
    pub name: String,
    pub members: u32,
    pub verified: bool,
    /// Their role in it.
    pub role: String,
    pub primary: bool,
}

/// An item in a grid: catalog, inventory or a profile.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ItemCard {
    pub id: Id,
    pub bundle: bool,
    pub name: String,
    /// Empty when unknown.
    pub creator: String,
    pub creator_verified: bool,
    /// Copies owned, for inventory lists; 1 elsewhere.
    pub copies: u32,
    /// 0 for bundles and when unknown.
    pub asset_type: i32,
    /// Robux; `None` when it can't be bought.
    pub price: Option<u32>,
    pub limited: bool,
    pub serial: Option<u32>,
    /// Recent average price, for collectibles.
    pub rap: Option<u32>,
    /// Rolimons' data, for collectibles when values are on.
    pub values: Option<market::ValueTag>,
}

fn count(n: i64) -> Option<u32> {
    (n >= 0).then(|| n.min(u32::MAX as i64) as u32)
}

/// Someone in a people list: a friend request, a follower, a blocked user.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: Id,
    /// Empty until the user is known.
    pub name: String,
    pub display_name: String,
    /// Mutual friends, for friend requests.
    pub mutuals: Vec<String>,
    /// ISO date a friend request was sent.
    pub sent_at: Option<String>,
    /// Whether the active account follows them, for followers. `None` when not known yet.
    pub following: Option<bool>,
}

/// Someone in a profile's friends, followers or following list.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub id: Id,
    /// Empty until the user is known.
    pub name: String,
    pub display_name: String,
    pub verified: bool,
    /// A friend of the active account (a mutual friend, on a friends list).
    pub friend: bool,
    /// Known only for the active account's friends; offline for everyone else.
    pub presence: Presence,
    /// Whether the active account follows them. `None` for yourself, when signed out, or until
    /// it has loaded.
    pub following: Option<bool>,
}

/// Buying a private server here, when you can.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ServerOffer {
    /// Robux per month; 0 is free.
    pub price: u32,
}

/// Settings of one of your private servers. The link itself stays in the backend; "Copy link"
/// copies it.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ServerSettings {
    pub name: String,
    pub active: bool,
    pub friends_allowed: bool,
    pub renews: bool,
    /// ISO date the subscription ends or renews, or empty.
    pub expires: String,
    /// Robux per month.
    pub price: u32,
    pub has_link: bool,
    /// Users let in besides friends.
    pub users: Vec<User>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PassRow {
    pub id: Id,
    pub name: String,
    pub description: String,
    /// None when off sale.
    #[specta(type = Option<specta_typescript::Number>)]
    pub price: Option<u64>,
    pub owned: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct BadgeRow {
    pub id: Id,
    pub name: String,
    pub description: String,
    /// Share of players who have it, 0 to 1.
    pub win_rate: f64,
    #[specta(type = specta_typescript::Number)]
    pub awarded: u64,
    /// ISO date the active account earned it.
    pub earned: Option<String>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct EventRow {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    /// ISO dates.
    pub start: String,
    pub end: String,
    pub host: String,
    pub going: bool,
}

#[tauri::command]
#[specta::specta]
pub fn shell(core: State<'_, Arc<Core>>) -> Shell {
    let s = core.store();
    Shell {
        accounts: s
            .accounts
            .accounts
            .iter()
            .map(|a| Account {
                id: Id(a.id.0),
                name: a.name.clone(),
                display_name: a.display_name.clone(),
                label: a.label.clone(),
                color: a.color.into(),
                session: s
                    .data
                    .get(&a.id)
                    .map_or(Session::Unknown, |d| (&d.session).into()),
                unread: s
                    .data
                    .get(&a.id)
                    .map_or(0, |d| d.unread.total().clamp(0, u32::MAX as i64) as u32),
            })
            .collect(),
        active: s.accounts.active.map(|a| Id(a.0)),
        offline: s.offline,
        jobs: s.jobs,
        toasts: s
            .toasts
            .iter()
            .map(|t| Toast {
                id: Id(t.id),
                account: t.account.map(|a| Id(a.0)),
                text: t.text.clone(),
                error: t.error,
                host: t.host.clone(),
                retry_after_secs: t.retry_after_secs,
            })
            .collect(),
        keychain_warning: s.keychain_warning.clone(),
        item_values: core.values_enabled(),
        avatar_3d: core.settings().avatar_3d,
        ui_scale: f64::from(core.settings().ui_scale),
        look: {
            let p = core.settings();
            Look {
                density: p.density.name().to_owned(),
                format_locale: p.format_locale,
                motion: p.motion,
                time_format: p.time_format,
                relative_dates: p.relative_dates,
                status_bar: p.status_bar,
                start_screen: p.start_screen,
                sidebar_mode: p.sidebar_mode,
                drawer: p.drawer,
                home_friends: p.home.friends,
                home_continue: p.home.continue_playing,
                home_favorites: p.home.favorites,
                home_recommended: p.home.recommended,
                account_accent: p.account_accent,
            }
        },
        sidebar: {
            let p = core.settings().sidebar;
            SidebarView {
                order: p.order,
                hidden: p.hidden,
            }
        },
        robux: s.active_data().and_then(|d| d.robux).map(|r| r as f64),
        unread: s.active_data().map(|d| d.unread.into()).unwrap_or_default(),
        challenge: s.challenge.as_ref().map(challenge::view),
        theme_import: s.theme_import.as_ref().map(account::theme_import_view),
        plugin_install: s.plugin_install.as_ref().map(plugins::install_view),
        plugin_actions: s.plugin_actions.as_ref().map(|p| plugins::actions_view(p, &s)),
        plugin_open: s.plugin_open.as_ref().map(plugins::open_view),
        plugin_rev: Id(s.plugin_rev),
        plugin_automation: s.plugin_automation.map(|a| plugins::automation_view(a, &s)),
        plugin_wrote: s.plugin_wrote.map(|t| Id(t.max(0) as u64)),
        approve: s.approve.as_ref().map(approve::view),
        security: s.security.as_ref().map(security::view),
        login: s.login.as_ref().map(|l| match l {
            LoginState::Browser => Login::Browser,
            LoginState::QuickStarting => Login::QuickStarting,
            LoginState::QuickCode {
                code,
                expires_at,
                linked,
            } => Login::QuickCode {
                code: code.clone(),
                expires_at: Id((*expires_at).max(0) as u64),
                linked: linked.clone(),
            },
            LoginState::Validating => Login::Validating,
            LoginState::Confirm { user, already_added } => Login::Confirm {
                user: User {
                    id: Id(user.id),
                    name: user.name.clone(),
                    display_name: user.display_name.clone(),
                },
                already_added: *already_added,
            },
            LoginState::Failed(message) => Login::Failed {
                message: message.clone(),
            },
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub fn commands(core: State<'_, Arc<Core>>) -> Vec<CommandInfo> {
    let mut registry = Registry::default();
    registry.load_overrides(core.settings().shortcuts);
    let builtin = registry.all().iter().map(|c| CommandInfo {
        id: c.id.key().to_owned(),
        title: c.title.to_owned(),
        keywords: c.keywords.iter().map(|k| (*k).to_owned()).collect(),
        arg: c.arg.into(),
        shortcuts: registry.shortcuts(c.id).iter().map(Keys::from).collect(),
        rebound: registry.is_rebound(c.id),
        context: None,
    });
    let plugin = core.plugin_commands().into_iter().map(plugins::command_info);
    builtin.chain(plugin).collect()
}

/// Runs a command that needs the backend. Commands that only change UI state (navigation,
/// zoom, tabs) are handled in the UI and never reach this.
#[tauri::command]
#[specta::specta]
pub fn run_command(app: AppHandle, core: State<'_, Arc<Core>>, inv: Invocation) -> Result<(), String> {
    if let Some(CommandRef::Plugin(cmd)) = CommandRef::parse(&inv.id) {
        let picked = inv.as_account.map(|a| AccountId(a.0));
        return core.run_plugin_command(&cmd, inv.arg, core.acting(picked), inv.context.as_deref());
    }
    dispatch(&app, core.inner(), inv)
}

/// Runs a built-in command. Plugin actions come here too, through the runner set in `lib.rs`,
/// after bonsai-core checked them.
pub fn dispatch(app: &AppHandle, core: &Arc<Core>, inv: Invocation) -> Result<(), String> {
    let Some(CommandRef::Builtin(id)) = CommandRef::parse(&inv.id) else {
        return Err(format!("unknown command: {}", inv.id));
    };
    let nth = |n: usize| core.store().accounts.accounts.get(n).map(|a| a.id);
    let switch = |target: Option<AccountId>| match target {
        Some(a) if core.switch(a) => Ok(()),
        _ => Err("no such account".to_owned()),
    };
    let picked = inv.as_account.map(|a| AccountId(a.0));
    let acting = || core.acting(picked).ok_or_else(|| "no account".to_owned());
    let arg = || inv.arg.clone().filter(|a| !a.trim().is_empty());
    let page = || page_game(core, inv.context.as_deref());
    let pick = |how: ServerPick| {
        let (place, _) = page()?;
        core.join_picked(acting()?, place, how)
    };
    if let Some(r) = communities::run(app, core, id, &inv, core.acting(picked)) {
        return r;
    }
    if let Some(r) = market::run(core, id, &inv, core.acting(picked)) {
        return r;
    }
    if let Some(r) = trades::run(app, core, id, &inv, core.acting(picked)) {
        return r;
    }
    if let Some(r) = transactions::run(app, core, id, &inv, core.acting(picked)) {
        return r;
    }
    if let Some(r) = inbox::run(core, id, &inv, core.acting(picked)) {
        return r;
    }
    if let Some(r) = discover::run(core, id, &inv, core.acting(picked)) {
        return r;
    }
    if let Some(r) = avatar::run(core, id, &inv, core.acting(picked)) {
        return r;
    }
    if let Some(r) = account::run(app, core, id, &inv, core.acting(picked)) {
        return r;
    }
    if let Some(r) = plugins::run(app, core, id, &inv, core.acting(picked)) {
        return r;
    }
    let move_by = |by: isize| {
        let a = acting()?;
        let at = core
            .store()
            .accounts
            .accounts
            .iter()
            .position(|m| m.id == a)
            .ok_or_else(|| "no such account".to_owned())?;
        core.reorder(a, at.saturating_add_signed(by));
        Ok(())
    };
    match id {
        CommandId::SwitchAccount => switch(inv.arg.and_then(|a| a.parse().ok()).map(AccountId)),
        CommandId::SwitchToLast => {
            core.switch_last();
            Ok(())
        }
        CommandId::SwitchTo1 => switch(nth(0)),
        CommandId::SwitchTo2 => switch(nth(1)),
        CommandId::SwitchTo3 => switch(nth(2)),
        CommandId::SwitchTo4 => switch(nth(3)),
        CommandId::SwitchTo5 => switch(nth(4)),
        CommandId::SwitchTo6 => switch(nth(5)),
        CommandId::SwitchTo7 => switch(nth(6)),
        CommandId::SwitchTo8 => switch(nth(7)),
        CommandId::SwitchTo9 => switch(nth(8)),
        CommandId::CopyAccountName => {
            let a = acting()?;
            let name = core
                .store()
                .accounts
                .accounts
                .iter()
                .find(|x| x.id == a)
                .map(|x| x.name.clone())
                .ok_or_else(|| "Unknown account.".to_owned())?;
            copy(&app, &core, name, "Copied username.")
        }
        CommandId::RemoveAccount => {
            let a = match arg() {
                Some(a) => AccountId(a.parse().map_err(|_| "bad account id".to_owned())?),
                None => acting()?,
            };
            core.send(Cmd::RemoveAccount(a));
            Ok(())
        }
        CommandId::SignOut => {
            core.send(Cmd::SignOut(acting()?));
            Ok(())
        }
        CommandId::RenameAccount => {
            core.rename(acting()?, arg());
            Ok(())
        }
        CommandId::SetAccountColor => {
            let c = arg()
                .and_then(|c| ColorTag::from_name(c.trim()))
                .ok_or_else(|| "unknown color".to_owned())?;
            core.set_color(acting()?, c);
            Ok(())
        }
        CommandId::MoveAccountUp => move_by(-1),
        CommandId::MoveAccountDown => move_by(1),
        CommandId::Refresh => {
            let a = acting()?;
            core.send(Cmd::Refresh(a));
            core.reload_home(a);
            Ok(())
        }
        CommandId::Join | CommandId::JoinAs => {
            // No argument joins the experience the command ran from.
            let target = arg()
                .or_else(|| page().ok().map(|(place, _)| place.to_string()))
                .as_deref()
                .and_then(join_target)
                .ok_or_else(|| "Nothing to join.".to_owned())?;
            core.send(Cmd::Launch {
                account: acting()?,
                target,
            });
            Ok(())
        }
        CommandId::LikeGame | CommandId::DislikeGame => {
            let (_, u) = page()?;
            core.try_vote(acting()?, u, id == CommandId::LikeGame)
        }
        CommandId::FavoriteGame => {
            let (_, u) = page()?;
            core.toggle_favorite(acting()?, u)
        }
        CommandId::NotifyGame => {
            let (_, u) = page()?;
            core.toggle_follow_game(acting()?, u)
        }
        CommandId::NotifyEvent => {
            let (_, u) = page()?;
            let event = arg().ok_or_else(|| "Which event?".to_owned())?;
            core.toggle_rsvp(acting()?, u, event.trim())
        }
        CommandId::RefreshServers => {
            let (place, _) = page()?;
            core.refresh_servers(acting()?, place);
            Ok(())
        }
        CommandId::JoinJobId => {
            let (place_id, _) = page()?;
            let job_id = arg().ok_or_else(|| "Which server?".to_owned())?.trim().to_owned();
            core.send(Cmd::Launch {
                account: acting()?,
                target: JoinTarget::Server { place_id, job_id },
            });
            Ok(())
        }
        CommandId::JoinBestServer => pick(ServerPick::Best),
        CommandId::JoinSmallestServer => pick(ServerPick::Smallest),
        CommandId::JoinRandomServer => pick(ServerPick::Random),
        CommandId::JoinPrivateServer => {
            // `<server> <place>` from lists outside an experience; else the page's place.
            let a = arg().ok_or_else(|| "Which private server?".to_owned())?;
            let mut parts = a.split_whitespace();
            let server = parse_id(parts.next().map(str::to_owned), "private server")?;
            let place = match parts.next() {
                Some(p) => parse_id(Some(p.to_owned()), "place")?,
                None => page()?.0,
            };
            core.join_private_server(acting()?, place, server)
        }
        CommandId::OpenOnWebsite => {
            // An argument names the target (from a menu); otherwise the page it ran from.
            let target = arg().or_else(|| inv.context.clone()).unwrap_or_default();
            // A full link (a notification's) opens as is; `open_web` only allows Roblox pages.
            if target.trim().starts_with("https://") {
                return core.open_web(target.trim());
            }
            let me = core.acting(picked).map(|a| a.0);
            let url = links::page(&target, me).ok_or_else(|| "Nothing to open.".to_owned())?;
            core.open_web(&url)
        }
        CommandId::Report => {
            let target = arg().or_else(|| inv.context.clone()).unwrap_or_default();
            let me = core.acting(picked).map(|a| a.0);
            let url = links::report_target(&target, me).ok_or_else(|| "Nothing to report.".to_owned())?;
            core.open_web(&url)
        }
        CommandId::CopyId => {
            let target = arg().or_else(|| inv.context.clone()).unwrap_or_default();
            let id = links::target_id(&target).ok_or_else(|| "Nothing to copy.".to_owned())?;
            copy(app, core, id.to_string(), "Copied ID.")
        }
        CommandId::CopyLink => {
            let target = arg().or_else(|| inv.context.clone()).unwrap_or_default();
            if let Some(server) = target.trim().strip_prefix("server:") {
                let link = core.server_link(acting()?, parse_id(Some(server.to_owned()), "server")?)?;
                return copy(app, core, link, "Copied server link.");
            }
            let me = core.acting(picked).map(|a| a.0);
            let url = links::page(&target, me).ok_or_else(|| "No link for this.".to_owned())?;
            copy(app, core, url, "Copied link.")
        }
        CommandId::CopyServerLink => {
            let (place, _) = page()?;
            let job = arg().ok_or_else(|| "Which server?".to_owned())?;
            copy(
                app,
                core,
                links::server_invite(place, job.trim()),
                "Copied invite link.",
            )
        }
        CommandId::BuyPrivateServer => Err("Open an experience first.".to_owned()),
        CommandId::RenamePrivateServer => {
            let a = arg().ok_or_else(|| "Which server, and what name?".to_owned())?;
            let (server, name) = a
                .trim()
                .split_once(' ')
                .ok_or_else(|| "Give a new name.".to_owned())?;
            let server = parse_id(Some(server.to_owned()), "server")?;
            let name = name.trim();
            if name.is_empty() {
                return Err("Give a new name.".to_owned());
            }
            core.change_server(acting()?, server, ServerChange::Rename(name.to_owned()));
            Ok(())
        }
        CommandId::ToggleServerActive => {
            core.toggle_server(acting()?, parse_id(arg(), "server")?, ServerChange::Active)
        }
        CommandId::ToggleServerFriends => core.toggle_server(
            acting()?,
            parse_id(arg(), "server")?,
            ServerChange::FriendsAllowed,
        ),
        CommandId::ToggleServerRenew => {
            core.toggle_server(acting()?, parse_id(arg(), "server")?, ServerChange::Renew)
        }
        CommandId::NewServerLink => {
            core.change_server(acting()?, parse_id(arg(), "server")?, ServerChange::NewLink);
            Ok(())
        }
        CommandId::AddFriend | CommandId::AddFriendAs => user_action(core, acting()?, &inv, Core::add_friend),
        CommandId::Unfriend => user_action(core, acting()?, &inv, Core::unfriend),
        CommandId::AcceptFriend => user_action(core, acting()?, &inv, Core::accept_friend),
        CommandId::DeclineFriend => user_action(core, acting()?, &inv, Core::decline_friend),
        CommandId::DeclineAllFriends => {
            core.decline_all_friends(acting()?);
            Ok(())
        }
        CommandId::Follow => user_action(core, acting()?, &inv, Core::follow),
        CommandId::Unfollow => user_action(core, acting()?, &inv, Core::unfollow),
        CommandId::Block => user_action(core, acting()?, &inv, Core::block),
        CommandId::Unblock => user_action(core, acting()?, &inv, Core::unblock),
        CommandId::FindUser => user_action(core, acting()?, &inv, Core::find_user),
        CommandId::SetNickname | CommandId::SetNote => {
            let a = acting()?;
            let raw = arg().ok_or_else(|| "Which user?".to_owned())?;
            let (who, text) = match raw.trim().split_once(char::is_whitespace) {
                Some((w, t)) => (w.to_owned(), t.trim().to_owned()),
                None => (raw.trim().to_owned(), String::new()),
            };
            let nickname = id == CommandId::SetNickname;
            core.with_user(&who, move |c, user| {
                let what = if nickname { "Nickname" } else { "Note" };
                if nickname {
                    c.set_nickname(a, user, &text);
                } else {
                    c.set_note(a, user, &text);
                }
                let done = if text.is_empty() { "cleared" } else { "saved" };
                c.notice(format!("{what} {done}."), false);
            })
        }
        CommandId::SearchRoblox => {
            core.note_search(viewer(core), &arg().unwrap_or_default());
            Ok(())
        }
        CommandId::ClearSearchHistory => {
            core.clear_searches(viewer(core));
            Ok(())
        }
        // The experience page handles this with its selected server; it only lands here without one.
        CommandId::JoinServer => Err("Open an experience and pick a server first.".to_owned()),
        // List and sub-tab keys are handled by the screen showing one. Elsewhere they do nothing.
        CommandId::ListDown
        | CommandId::ListUp
        | CommandId::ListOpen
        | CommandId::ListSelect
        | CommandId::NextSubTab
        | CommandId::PrevSubTab => Ok(()),
        CommandId::LoadMore if inv.context.as_deref().is_some_and(|c| c.starts_with("group:")) => {
            communities::more(core, &inv, core.acting(picked))
        }
        CommandId::LoadMore if inv.context.as_deref().is_some_and(|c| c.starts_with("user:")) => {
            let user = inv
                .context
                .as_deref()
                .and_then(links::target_id)
                .ok_or_else(|| "Open a profile first.".to_owned())?;
            match arg().as_deref().map(str::trim) {
                Some("badges") => drop(core.user_badges(viewer(core), user, true)),
                Some("favorites") => drop(core.favorite_games(user, true)),
                Some("outfits") => drop(core.user_outfits(user, true)),
                Some("requests") => drop(core.friend_requests(acting()?, true)),
                Some("followers") => drop(core.followers(acting()?, user, true)),
                Some("following") => drop(core.followings(acting()?, user, true)),
                Some("friends") => drop(core.friends_of(user, true)),
                // A friends search on a profile: `friends:<query>`.
                Some(a) if a.starts_with("friends:") => {
                    drop(core.search_friends_of(acting()?, user, &a[8..], true))
                }
                _ => return Err("Nothing more to load here.".to_owned()),
            }
            Ok(())
        }
        CommandId::LoadMore => {
            let which = arg().ok_or_else(|| "Which list?".to_owned())?;
            let (place, u) = page()?;
            let a = viewer(core);
            match which.trim() {
                "servers" => core.send(Cmd::LoadServers {
                    account: a,
                    place_id: place,
                    more: true,
                }),
                "private_servers" => drop(core.private_servers(acting()?, place, true)),
                "passes" => drop(core.game_passes(a, u, true)),
                "badges" => drop(core.game_badges(u, true)),
                _ => return Err("Nothing more to load here.".to_owned()),
            }
            Ok(())
        }
        // The experience page handles this with a confirm dialog that shows the price.
        CommandId::BuyPass => Err("Open the experience to buy a pass.".to_owned()),
        CommandId::QuickSignIn => {
            core.start_quick_sign_in();
            Ok(())
        }
        CommandId::SignInWithBrowser => crate::signin::open(app, core),
        CommandId::Quit => {
            app.exit(0);
            Ok(())
        }
        CommandId::MinimizeWindow | CommandId::MaximizeWindow => {
            let w = app
                .get_webview_window("main")
                .ok_or_else(|| "no window".to_owned())?;
            let r = if id == CommandId::MinimizeWindow {
                w.minimize()
            } else if w.is_maximized().unwrap_or(false) {
                w.unmaximize()
            } else {
                w.maximize()
            };
            r.map_err(|e| e.to_string())
        }
        other => Err(format!("not wired yet: {}", other.key())),
    }
}

#[tauri::command]
#[specta::specta]
pub fn friends(core: State<'_, Arc<Core>>, filter: String) -> Friends {
    let notes = core.acting(None).map(|a| core.user_notes(a)).unwrap_or_default();
    let (view, unnamed) = {
        let s = core.store();
        let Some(account) = s.active_id() else {
            return Friends {
                loading: false,
                error: None,
                friends: Vec::new(),
            };
        };
        let d = s.account_data(account);
        let rows = s.friend_rows(account, &filter);
        let unnamed: Vec<u64> = rows.iter().filter(|r| r.name.is_empty()).map(|r| r.id).collect();
        let view = Friends {
            loading: d.is_some_and(|d| d.friends.loading),
            error: d.and_then(|d| d.friends.error.clone()),
            friends: rows
                .into_iter()
                .map(|r| Friend {
                    id: Id(r.id),
                    name: r.name,
                    display_name: r.title,
                    presence: r.kind.into(),
                    status: r.status,
                    place_id: r.place_id.map(Id),
                    universe_id: r.universe_id.map(Id),
                    nickname: notes
                        .get(&r.id)
                        .map(|n| n.nickname.clone())
                        .filter(|n| !n.is_empty()),
                })
                .collect(),
        };
        (view, unnamed)
    };
    core.want_users(&unnamed);
    view
}

#[tauri::command]
#[specta::specta]
pub fn home(core: State<'_, Arc<Core>>) -> Home {
    let account = core.acting(None).unwrap_or(AccountId::ANONYMOUS);
    core.ensure_home(account);
    let (favorites, feed) = if account.is_anonymous() {
        (None, None)
    } else {
        (
            core.favorite_games(account.0, false).data,
            core.home_feed(account).data.filter(|f| !f.is_empty()),
        )
    };
    let (view, missing) = {
        let s = core.store();
        let d = s.account_data(account);
        let recent_ids = d.map(|d| d.recent.clone()).unwrap_or_default();
        let missing: Vec<u64> = recent_ids
            .iter()
            .filter(|u| !s.games.contains_key(u))
            .copied()
            .collect();
        let home = d.map(|d| &d.home);
        let card = |g: &bonsai_core::store::GameCard| {
            s.games
                .get(&g.universe_id)
                .map_or_else(|| GameCard::from(g), GameCard::from)
        };
        let sort = |h: &bonsai_core::store::HomeSort| Sort {
            id: h.id.clone(),
            title: h.title.clone(),
            subtitle: h.subtitle.clone(),
            games: h.games.iter().map(GameCard::from).collect(),
        };
        // Launched from here first, then what the website's feed says was played elsewhere.
        let mut recent: Vec<GameCard> = recent_ids
            .iter()
            .filter_map(|u| s.games.get(u))
            .map(GameCard::from)
            .collect();
        let played = feed
            .iter()
            .flat_map(|f| f.iter())
            .filter(|h| h.id == CONTINUE_TOPIC)
            .flat_map(|h| &h.games);
        for g in played {
            if !recent.iter().any(|r| r.universe_id.0 == g.universe_id) {
                recent.push(card(g));
            }
        }
        // Signed in, the site's own Home feed; otherwise (or until it loads) the explore sorts.
        let sorts = match &feed {
            Some(f) => f.iter().filter(|h| h.id != CONTINUE_TOPIC).map(sort).collect(),
            None => home
                .and_then(|h| h.data.as_ref())
                .into_iter()
                .flatten()
                .map(sort)
                .collect(),
        };
        let view = Home {
            loading: home.is_some_and(|h| h.loading),
            error: home.and_then(|h| h.error.clone()),
            recent,
            favorites: favorites.iter().flat_map(|p| &p.items).map(card).collect(),
            sorts,
        };
        (view, missing)
    };
    core.want_universes(&missing);
    view
}

/// Runs a one-click action on a user: the argument (an id or a username), else the profile the
/// command ran from.
fn user_action(
    core: &Core,
    account: AccountId,
    inv: &Invocation,
    f: fn(&Core, AccountId, u64),
) -> Result<(), String> {
    let who = inv
        .arg
        .clone()
        .filter(|a| !a.trim().is_empty())
        .or_else(|| {
            inv.context
                .as_deref()
                .filter(|c| c.starts_with("user:"))
                .and_then(links::target_id)
                .map(|id| id.to_string())
        })
        .ok_or_else(|| "Which user?".to_owned())?;
    core.with_user(&who, move |c, user| {
        if user == account.0 {
            c.notice("That's you.", true);
        } else {
            f(c, account, user);
        }
    })
}

/// A numeric id from a command argument; `what` names it in the error.
fn parse_id(arg: Option<String>, what: &str) -> Result<u64, String> {
    arg.and_then(|a| a.trim().parse().ok())
        .ok_or_else(|| format!("Which {what}?"))
}

/// Puts text on the system clipboard and says so.
fn copy(app: &AppHandle, core: &Core, text: String, done: &str) -> Result<(), String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    app.clipboard()
        .write_text(text)
        .map_err(|e| format!("Could not copy ({e})."))?;
    core.notice(done, false);
    Ok(())
}

/// The account an experience page reads as: the active one, or signed out.
fn viewer(core: &Core) -> AccountId {
    core.acting(None).unwrap_or(AccountId::ANONYMOUS)
}

/// The universe of a place, once known.
fn universe_of(core: &Core, place_id: u64) -> Option<u64> {
    core.store().place_universe.get(&place_id).copied()
}

/// The experience a command ran from, from `Invocation.context`: (place id, universe id).
fn page_game(core: &Core, context: Option<&str>) -> Result<(u64, u64), String> {
    let place = context
        .and_then(|c| c.strip_prefix("game:"))
        .and_then(|p| p.parse().ok())
        .ok_or_else(|| "Open an experience first.".to_owned())?;
    let universe = universe_of(core, place).ok_or_else(|| "Still loading this experience.".to_owned())?;
    Ok((place, universe))
}

#[tauri::command]
#[specta::specta]
pub fn game(core: State<'_, Arc<Core>>, place_id: Id) -> Game {
    let account = viewer(&core);
    core.ensure_place(account, place_id.0);
    let mut view = Game {
        place_id,
        details: None,
        me: None,
        maturity: None,
        maturity_descriptors: Vec::new(),
        media: Vec::new(),
    };
    let Some(u) = universe_of(&core, place_id.0) else {
        return view;
    };
    view.details = core.store().games.get(&u).map(GameDetails::from);
    view.me = core.my_game_state(account, u).data.map(|m| MyGame {
        can_vote: m.can_vote,
        vote: m.vote,
        vote_hint: m.vote_hint.clone(),
        favorited: m.favorited,
        following: m.following,
        can_follow: m.can_follow,
    });
    if let Some(m) = core.maturity(u).data {
        view.maturity = Some(m.label.clone()).filter(|l| !l.is_empty());
        view.maturity_descriptors = m.descriptors.clone();
    }
    view.media = core
        .game_media(u)
        .data
        .map(|v| {
            v.iter()
                .map(|m| Media {
                    image_id: Id(m.image_id),
                    video: m.video_title.is_some(),
                    video_title: m.video_title.clone().unwrap_or_default(),
                    alt: m.alt.clone(),
                })
                .collect()
        })
        .unwrap_or_default();
    view
}

#[tauri::command]
#[specta::specta]
pub fn game_servers(core: State<'_, Arc<Core>>, place_id: Id) -> Servers {
    core.ensure_servers(viewer(&core), place_id.0);
    let s = core.store();
    let Some(l) = s.servers.get(&place_id.0) else {
        return Servers {
            loading: true,
            error: None,
            fetched_at: None,
            servers: Vec::new(),
            more: false,
        };
    };
    Servers {
        loading: l.loading,
        error: l.error.clone(),
        fetched_at: (l.fetched_at > 0).then_some(l.fetched_at as f64 * 1000.0),
        more: s.server_cursor.get(&place_id.0).is_some_and(Option::is_some),
        servers: l
            .data
            .iter()
            .flatten()
            .map(|v| ServerRow {
                job_id: v.job_id.clone(),
                playing: v.playing.max(0) as u32,
                max_players: v.max_players.max(0) as u32,
                ping: v.ping.map(|p| p.max(0) as u32),
                fps: v.fps,
                friends: v.friends.iter().copied().map(Id).collect(),
            })
            .collect(),
    }
}

/// Remote results for the command bar.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RemoteSearch {
    pub loading: bool,
    pub rows: Vec<RemoteRow>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RemoteRow {
    /// "Experiences", "People", "Pasted link", ….
    pub group: String,
    pub title: String,
    pub subtitle: String,
    /// The command that opens it, and its argument.
    pub command: String,
    pub arg: String,
    pub kind: RemoteKind,
    /// The user, place, community, asset or bundle id.
    pub id: Id,
    /// An experience's universe id, for its icon. None for a bare place id.
    pub universe_id: Option<Id>,
}

/// What a command bar search result is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum RemoteKind {
    User,
    Experience,
    Community,
    Item,
    Bundle,
}

/// Roblox's search for a command bar query: people, experiences, communities and items, or one
/// of them with a scope prefix (`@`, `g:`, `#`, `i:`). An exact username, an id or a pasted link
/// also gets its own rows. Starts the search the first time a query is seen; rows are the results
/// for this exact query, empty until they arrive.
#[tauri::command]
#[specta::specta]
pub fn remote_search(core: State<'_, Arc<Core>>, query: String) -> RemoteSearch {
    core.ensure_remote_search(viewer(&core), &query);
    let s = core.store();
    let q = query.trim();
    let rows = s
        .remote
        .data
        .as_ref()
        .filter(|r| r.query == q)
        .map(|r| r.rows.iter().filter_map(remote_row).collect())
        .unwrap_or_default();
    RemoteSearch {
        loading: s.remote.loading,
        rows,
    }
}

fn remote_row(e: &bonsai_core::search::Entry) -> Option<RemoteRow> {
    use bonsai_core::search::Target;
    let (command, arg, kind, id, universe) = match e.target {
        Target::Experience {
            place_id,
            universe_id,
        } => (
            "game.open",
            place_id.to_string(),
            RemoteKind::Experience,
            place_id,
            Some(Id(universe_id)),
        ),
        Target::Place(id) => ("game.open", id.to_string(), RemoteKind::Experience, id, None),
        Target::User(id) => ("user.open", id.to_string(), RemoteKind::User, id, None),
        Target::Group(id) => ("group.open", id.to_string(), RemoteKind::Community, id, None),
        Target::Asset(id) => ("market.open", format!("item:{id}"), RemoteKind::Item, id, None),
        Target::Bundle(id) => (
            "market.open",
            format!("bundle:{id}"),
            RemoteKind::Bundle,
            id,
            None,
        ),
        _ => return None,
    };
    Some(RemoteRow {
        group: e.group.title().to_owned(),
        title: e.title.clone(),
        subtitle: e.subtitle.clone(),
        command: command.to_owned(),
        arg,
        kind,
        id: Id(id),
        universe_id: universe,
    })
}

/// Results for the search results page.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SearchResults {
    pub loading: bool,
    pub error: Option<String>,
    pub rows: Vec<RemoteRow>,
}

/// Roblox's search for the results page: `scope` is `all`, `experiences`, `people`,
/// `communities` or `items`. One kind gets more rows than all of them together.
#[tauri::command]
#[specta::specta]
pub fn search_results(core: State<'_, Arc<Core>>, query: String, scope: String) -> SearchResults {
    use bonsai_core::search::Scope;
    let scope = match scope.as_str() {
        "experiences" => Scope::Experiences,
        "people" => Scope::Users,
        "communities" => Scope::Communities,
        "items" => Scope::Items,
        _ => Scope::All,
    };
    let r = core.search_results(viewer(&core), scope, &query);
    SearchResults {
        loading: r.loading,
        error: r.error,
        rows: r
            .data
            .map(|v| v.iter().filter_map(remote_row).collect())
            .unwrap_or_default(),
    }
}

/// What the active account searched for lately, newest first.
#[tauri::command]
#[specta::specta]
pub fn recent_searches(core: State<'_, Arc<Core>>) -> Vec<String> {
    core.recent_searches(viewer(&core))
}

/// What the active account calls a user, and its note about them.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct UserNoteView {
    pub nickname: String,
    pub note: String,
}

#[tauri::command]
#[specta::specta]
pub fn user_note(core: State<'_, Arc<Core>>, user_id: Id) -> UserNoteView {
    let n = core
        .acting(None)
        .map(|a| core.user_note(a, user_id.0))
        .unwrap_or_default();
    UserNoteView {
        nickname: n.nickname,
        note: n.note,
    }
}

/// A creator's profile or community description, for an experience's About tab. None while
/// loading.
#[tauri::command]
#[specta::specta]
pub fn creator_bio(core: State<'_, Arc<Core>>, creator_id: Id, group: bool) -> Option<String> {
    if group {
        core.community(creator_id.0).data.map(|c| c.description.clone())
    } else {
        core.profile(creator_id.0).data.map(|p| p.description.clone())
    }
}

/// A user an `@mention` in some text names.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MentionView {
    /// The lowercased name the text used. Can be a past username.
    pub requested: String,
    pub id: Id,
    pub name: String,
    pub display_name: String,
    pub verified: bool,
}

/// The users the `@mentions` in a description name, looked up in one request. Names nobody
/// has are left out.
#[tauri::command]
#[specta::specta]
pub fn mentioned_users(core: State<'_, Arc<Core>>, names: Vec<String>) -> List<MentionView> {
    List::of(core.users_by_names(&names), |u| MentionView {
        requested: u.requested.clone(),
        id: Id(u.id),
        name: u.name.clone(),
        display_name: u.display_name.clone(),
        verified: u.verified,
    })
}

/// Servers of a place with friends of the active account in them.
#[tauri::command]
#[specta::specta]
pub fn game_friend_servers(core: State<'_, Arc<Core>>, place_id: Id) -> List<ServerRow> {
    let account = viewer(&core);
    if account.is_anonymous() {
        return List {
            loading: false,
            ..List::loading()
        };
    }
    List::of(core.friend_servers(account, place_id.0), |v| ServerRow {
        job_id: v.job_id.clone(),
        playing: v.playing.max(0) as u32,
        max_players: v.max_players.max(0) as u32,
        ping: v.ping.map(|p| p.max(0) as u32),
        fps: None,
        friends: v.friends.iter().copied().map(Id).collect(),
    })
}

#[tauri::command]
#[specta::specta]
pub fn game_private_servers(core: State<'_, Arc<Core>>, place_id: Id) -> List<PrivateServerRow> {
    let account = viewer(&core);
    if account.is_anonymous() {
        return List {
            loading: false,
            ..List::loading()
        };
    }
    let r = core.private_servers(account, place_id.0, false);
    let (view, unnamed) = {
        let s = core.store();
        let mut unnamed = Vec::new();
        let view = List::paged(r, |p| {
            let owner_name = s
                .users
                .get(&p.owner_id)
                .map(|u| u.display_name.clone())
                .unwrap_or_default();
            if owner_name.is_empty() {
                unnamed.push(p.owner_id);
            }
            PrivateServerRow {
                id: Id(p.id),
                name: p.name.clone(),
                owner_id: Id(p.owner_id),
                owner_name,
                mine: p.owner_id == account.0,
                playing: p.playing.max(0) as u32,
                max_players: p.max_players.max(0) as u32,
            }
        });
        (view, unnamed)
    };
    core.want_users(&unnamed);
    view
}

#[tauri::command]
#[specta::specta]
pub fn profile(core: State<'_, Arc<Core>>, user_id: Id) -> ProfileView {
    let uid = user_id.0;
    let account = viewer(&core);
    let r = core.profile(uid);
    let mine = core.store().accounts.accounts.iter().any(|a| a.id.0 == uid);
    let relation = (!mine)
        .then(|| core.relation(account, uid).data)
        .flatten()
        .map(|r| RelationView {
            friend: match r.friend {
                FriendState::Friends => FriendStatus::Friends,
                FriendState::RequestSent => FriendStatus::Sent,
                FriendState::RequestReceived => FriendStatus::Received,
                FriendState::NotFriends | FriendState::Unknown => FriendStatus::None,
            },
            following: r.following,
            followed_by: r.followed_by,
            blocked: r.blocked,
        });
    let p = core.presence_of(account, uid).data;
    let (presence, place, place_id, universe_id) = match p.as_deref() {
        Some(p) => {
            let place = (p.kind == PresenceKind::InGame)
                .then(|| {
                    let s = core.store();
                    p.universe_id
                        .and_then(|u| s.games.get(&u))
                        .map(|g| g.name.clone())
                        .or_else(|| (!p.last_location.is_empty()).then(|| p.last_location.clone()))
                })
                .flatten();
            (
                p.kind.into(),
                place,
                p.root_place_id.or(p.place_id).map(Id),
                p.universe_id.map(Id),
            )
        }
        None => (Presence::Offline, None, None, None),
    };
    let (seen, unfriended_at) = core.store().account_data(account).map_or((None, None), |d| {
        (
            d.last_seen.get(&uid).copied(),
            d.unfriended.get(&uid).map(|t| *t as f64 * 1000.0),
        )
    });
    // Rolimons sees people the app never did, and sometimes more recently.
    let roli = (!mine)
        .then(|| core.player_stats(uid).data)
        .flatten()
        .and_then(|p| p.last_online.map(|t| (t, p.last_location.clone())));
    let (last_online, last_online_by, last_location) = match (seen, roli) {
        (Some(a), Some((r, _))) if a >= r => (Some(a), Some(SeenBy::App), None),
        (_, Some((r, loc))) => (Some(r), Some(SeenBy::Rolimons), (!loc.is_empty()).then_some(loc)),
        (Some(a), None) => (Some(a), Some(SeenBy::App), None),
        (None, None) => (None, None, None),
    };
    let last_online = last_online.map(|t| t as f64 * 1000.0);
    let note = if account.is_anonymous() {
        Default::default()
    } else {
        core.user_note(account, uid)
    };
    let (mutual_friends, mutual_more) = if mine || account == AccountId::ANONYMOUS {
        (None, false)
    } else {
        match core.friends_of(uid, false).data {
            Some(page) => {
                let n = own_friends(&core, account)
                    .map(|own| page.items.iter().filter(|id| own.contains(id)).count() as u32);
                (n, page.next.is_some())
            }
            None => (None, false),
        }
    };
    let (visits, favorites_received) = match core.user_creations(uid).data {
        Some(list) => {
            let ids: Vec<u64> = list.iter().map(|c| c.universe_id).collect();
            let s = core.store();
            let favs: Option<i64> = ids.iter().map(|u| s.games.get(u).map(|g| g.favorites)).sum();
            drop(s);
            core.want_universes(&ids);
            (
                Some(list.iter().map(|c| c.visits).sum::<i64>() as f64),
                favs.map(|n| n as f64),
            )
        }
        None => (None, None),
    };
    ProfileView {
        loading: r.loading,
        error: r.error,
        user: r.data.map(|u| ProfileHead {
            id: Id(u.id),
            name: u.name.clone(),
            display_name: u.display_name.clone(),
            description: u.description.clone(),
            created: u.created.clone(),
            verified: u.verified,
            banned: u.banned,
            friends: count(u.friends),
            followers: count(u.followers),
            following: count(u.following),
        }),
        presence,
        place,
        place_id,
        universe_id,
        relation,
        mine,
        past_names: core
            .username_history(uid)
            .data
            .map(|v| v.to_vec())
            .unwrap_or_default(),
        last_online,
        last_online_by,
        last_location,
        unfriended_at,
        mutual_friends,
        mutual_more,
        visits,
        favorites_received,
        nickname: Some(note.nickname).filter(|n| !n.is_empty()),
        note: Some(note.note).filter(|n| !n.is_empty()),
    }
}

/// Rolimons' value, RAP, rank and badges for a user, when values are on.
#[tauri::command]
#[specta::specta]
pub fn profile_trading(core: State<'_, Arc<Core>>, user_id: Id) -> TraderView {
    use bonsai_core::values::BadgeGroup as G;
    let ms = |t: i64| (t > 0).then_some(t as f64 * 1000.0);
    let n = |v: Option<i64>| v.map(|v| v as f64);
    let r = core.player_stats(user_id.0);
    TraderView {
        enabled: core.values_enabled(),
        loading: r.loading,
        error: r.error,
        stats: r.data.map(|p| TraderStats {
            tracked: p.tracked,
            value: n(p.value),
            rap: n(p.rap),
            rank: p.rank.map(|r| r.clamp(0, u32::MAX as i64) as u32),
            premium: p.premium,
            private: p.private,
            terminated: p.terminated,
            updated_at: p.stats_updated.and_then(ms),
            badges: p
                .badges
                .iter()
                .map(|b| TraderBadge {
                    key: b.key.clone(),
                    name: b.name.clone(),
                    group: match b.group {
                        G::Value => TraderBadgeGroup::Value,
                        G::Item => TraderBadgeGroup::Item,
                        G::Collection => TraderBadgeGroup::Collection,
                        G::Trading => TraderBadgeGroup::Trading,
                        G::Community => TraderBadgeGroup::Community,
                        G::Unknown => TraderBadgeGroup::Other,
                    },
                    family: b.family.clone(),
                    earned: ms(b.earned),
                })
                .collect(),
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub fn profile_badges(core: State<'_, Arc<Core>>, user_id: Id) -> List<UserBadge> {
    List::paged(core.user_badges(viewer(&core), user_id.0, false), |b| UserBadge {
        id: Id(b.id),
        name: b.name.clone(),
        description: b.description.clone(),
        win_rate: b.win_rate,
        place_id: Id(b.place_id),
    })
}

#[tauri::command]
#[specta::specta]
pub fn profile_groups(core: State<'_, Arc<Core>>, user_id: Id) -> List<GroupCard> {
    List::of(core.user_groups(user_id.0), |g| GroupCard::from(g))
}

/// Experiences a user made.
#[tauri::command]
#[specta::specta]
pub fn profile_creations(core: State<'_, Arc<Core>>, user_id: Id) -> List<GameCard> {
    let r = core.user_creations(user_id.0);
    let (view, missing) = {
        let s = core.store();
        let mut missing = Vec::new();
        let view = List::of(r, |c| match s.games.get(&c.universe_id) {
            Some(g) => GameCard::from(g),
            None => {
                missing.push(c.universe_id);
                GameCard {
                    universe_id: Id(c.universe_id),
                    place_id: Id(c.place_id),
                    name: c.name.clone(),
                    creator: String::new(),
                    playing: 0,
                    rating: None,
                    sponsored: false,
                    genre: String::new(),
                }
            }
        });
        (view, missing)
    };
    core.want_universes(&missing);
    view
}

/// Experiences a user follows. Roblox shows this list only to the user, so it is empty for
/// anyone but your own accounts.
#[tauri::command]
#[specta::specta]
pub fn profile_followed_games(core: State<'_, Arc<Core>>, user_id: Id) -> List<GameCard> {
    let uid = user_id.0;
    if !core.store().accounts.accounts.iter().any(|a| a.id.0 == uid) {
        return List {
            loading: false,
            ..List::loading()
        };
    }
    let r = core.followed_games(AccountId(uid));
    let s = core.store();
    List::of(r, |g| {
        s.games
            .get(&g.universe_id)
            .map_or_else(|| GameCard::from(g), GameCard::from)
    })
}

/// Experiences a user favorited.
#[tauri::command]
#[specta::specta]
pub fn profile_favorites(core: State<'_, Arc<Core>>, user_id: Id) -> List<GameCard> {
    let r = core.favorite_games(user_id.0, false);
    let s = core.store();
    List::paged(r, |g| match s.games.get(&g.universe_id) {
        Some(full) => GameCard::from(full),
        None => GameCard::from(g),
    })
}

/// A user's collectibles, when the active account may see their inventory.
#[tauri::command]
#[specta::specta]
pub fn profile_collectibles(core: State<'_, Arc<Core>>, user_id: Id) -> List<ItemCard> {
    let account = viewer(&core);
    let can = core.can_view_inventory(account, user_id.0);
    match can.data.as_deref() {
        Some(true) => {}
        Some(false) => {
            return List {
                loading: false,
                error: Some("Their inventory is private.".to_owned()),
                items: Vec::new(),
                more: false,
            };
        }
        None => {
            return List {
                loading: can.error.is_none(),
                error: can.error,
                items: Vec::new(),
                more: false,
            };
        }
    }
    let values = core.item_values().data;
    List::of(core.collectibles(account, user_id.0), |i| {
        market::inv_card(i, 0, values.as_deref())
    })
}

/// The active account's friend requests, followers, following or blocked users (`which`).
#[tauri::command]
#[specta::specta]
pub fn people(core: State<'_, Arc<Core>>, which: String) -> List<Person> {
    let Some(account) = core.acting(None) else {
        return List {
            loading: false,
            ..List::loading()
        };
    };
    let me = account.0;
    let ids = |r: Res<Page<u64>>| List::paged(r, |id| *id);
    let list: List<u64> = match which.as_str() {
        "requests" => {
            // The requests endpoint often leaves names empty; fill them from the user store.
            let mut unnamed = Vec::new();
            let page = core.friend_requests(account, false);
            let view = {
                let s = core.store();
                List::paged(page, |r| {
                    let u = s.users.get(&r.user_id).filter(|_| r.name.is_empty());
                    if r.name.is_empty() && u.is_none() {
                        unnamed.push(r.user_id);
                    }
                    Person {
                        id: Id(r.user_id),
                        name: u.map_or_else(|| r.name.clone(), |u| u.name.clone()),
                        display_name: u.map_or_else(|| r.display_name.clone(), |u| u.display_name.clone()),
                        mutuals: r.mutuals.clone(),
                        sent_at: Some(r.sent_at.clone()).filter(|s| !s.is_empty()),
                        following: None,
                    }
                })
            };
            core.want_users(&unnamed);
            return view;
        }
        "followers" => ids(core.followers(account, me, false)),
        "following" => ids(core.followings(account, me, false)),
        "blocked" => List::of(core.blocked_users(account), |id| *id),
        _ => return List::loading(),
    };
    // Followers get a follow-back button, so ask which of them you follow.
    let follows = if which == "followers" {
        following_of(&core, account, &list.items)
    } else {
        Default::default()
    };
    let (view, unnamed) = {
        let s = core.store();
        let mut unnamed = Vec::new();
        let items = list
            .items
            .iter()
            .map(|id| {
                let u = s.users.get(id);
                if u.is_none() {
                    unnamed.push(*id);
                }
                Person {
                    id: Id(*id),
                    name: u.map(|u| u.name.clone()).unwrap_or_default(),
                    display_name: u.map(|u| u.display_name.clone()).unwrap_or_default(),
                    mutuals: Vec::new(),
                    sent_at: None,
                    following: follows.get(id).copied(),
                }
            })
            .collect();
        (
            List {
                loading: list.loading,
                error: list.error,
                items,
                more: list.more,
            },
            unnamed,
        )
    };
    core.want_users(&unnamed);
    view
}

/// Follow-request batch size for [`following_of`].
const FOLLOW_BATCH: usize = 100;

/// Which of `ids` `account` follows, as far as loaded. Asked a page at a time, so a page that
/// is already known isn't asked again as the list grows.
fn following_of(core: &Core, account: AccountId, ids: &[u64]) -> std::collections::HashMap<u64, bool> {
    ids.chunks(FOLLOW_BATCH)
        .filter_map(|c| core.follows(account, c).data)
        .flat_map(|m| m.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>())
        .collect()
}

/// The account's friends, once its friends list has loaded.
fn own_friends(core: &Core, account: AccountId) -> Option<std::collections::HashSet<u64>> {
    let s = core.store();
    let d = s.account_data(account)?;
    Some(d.friends.data.as_ref()?.iter().copied().collect())
}

/// A user's friends, followers or following (`which`), as the active account sees them.
/// `query` searches their friends on Roblox; it is ignored for the other lists. Friends lists
/// put mutual friends first.
#[tauri::command]
#[specta::specta]
pub fn profile_people(
    core: State<'_, Arc<Core>>,
    user_id: Id,
    which: String,
    query: String,
) -> List<Connection> {
    let uid = user_id.0;
    let account = core.acting(None);
    let signed_in = |r: &dyn Fn(AccountId) -> Res<Page<u64>>| match account {
        Some(a) => List::paged(r(a), |id| *id),
        None => List {
            loading: false,
            error: Some("Sign in to see this list.".to_owned()),
            ..List::loading()
        },
    };
    let own = account.and_then(|a| own_friends(&core, a)).unwrap_or_default();
    let mut list: List<u64> = match which.as_str() {
        "friends" if !query.trim().is_empty() => {
            signed_in(&|a| core.search_friends_of(a, uid, &query, false))
        }
        "friends" => {
            let mut l = List::paged(core.friends_of(uid, false), |id| *id);
            l.items = bonsai_core::people::mutuals_first(&l.items, &own).0;
            l
        }
        "followers" => signed_in(&|a| core.followers(a, uid, false)),
        "following" => signed_in(&|a| core.followings(a, uid, false)),
        _ => return List::loading(),
    };
    let ids = std::mem::take(&mut list.items);
    let follows = account.map(|a| following_of(&core, a, &ids)).unwrap_or_default();
    let (items, unnamed) = {
        let s = core.store();
        let presence = account.and_then(|a| s.account_data(a)).map(|d| &d.presence);
        let mut unnamed = Vec::new();
        let items = ids
            .iter()
            .map(|id| {
                let u = s.users.get(id);
                if u.is_none() {
                    unnamed.push(*id);
                }
                let friend = own.contains(id);
                Connection {
                    id: Id(*id),
                    name: u.map(|u| u.name.clone()).unwrap_or_default(),
                    display_name: u.map(|u| u.display_name.clone()).unwrap_or_default(),
                    verified: u.is_some_and(|u| u.verified),
                    friend,
                    presence: presence
                        .filter(|_| friend)
                        .and_then(|p| p.get(id))
                        .map_or(Presence::Offline, |p| p.kind.into()),
                    following: follows.get(id).copied(),
                }
            })
            .collect();
        (items, unnamed)
    };
    core.want_users(&unnamed);
    List {
        loading: list.loading,
        error: list.error,
        items,
        more: list.more,
    }
}

/// Whether the active account can buy a private server in this place, and the price.
#[tauri::command]
#[specta::specta]
pub fn game_server_offer(core: State<'_, Arc<Core>>, place_id: Id) -> Option<ServerOffer> {
    let account = core.acting(None)?;
    let u = universe_of(&core, place_id.0)?;
    let o = core.private_server_offer(account, u, place_id.0).data?;
    let price = o.price.filter(|_| o.enabled && o.can_create)?;
    Some(ServerOffer {
        price: price.clamp(0, u32::MAX as i64) as u32,
    })
}

/// Settings of one of the active account's private servers.
#[tauri::command]
#[specta::specta]
pub fn server_settings(core: State<'_, Arc<Core>>, server_id: Id) -> Option<ServerSettings> {
    let account = core.acting(None)?;
    let c = core.server_config(account, server_id.0).data?;
    let users: Vec<User> = {
        let s = core.store();
        c.users
            .iter()
            .map(|id| {
                let u = s.users.get(id);
                User {
                    id: Id(*id),
                    name: u.map(|u| u.name.clone()).unwrap_or_default(),
                    display_name: u.map(|u| u.display_name.clone()).unwrap_or_default(),
                }
            })
            .collect()
    };
    let unnamed: Vec<u64> = users
        .iter()
        .filter(|u| u.name.is_empty())
        .map(|u| u.id.0)
        .collect();
    core.want_users(&unnamed);
    Some(ServerSettings {
        name: c.name.clone(),
        active: c.active,
        friends_allowed: c.friends_allowed,
        renews: c.renews,
        expires: c.expires.clone(),
        price: c.price.clamp(0, u32::MAX as i64) as u32,
        has_link: !c.link.is_empty(),
        users,
    })
}

/// Buys a private server. Called only from the dialog `game.buy_private_server` opens, after the
/// user saw `price`; Roblox refuses the purchase if the price changed.
#[tauri::command]
#[specta::specta]
pub fn buy_private_server(
    core: State<'_, Arc<Core>>,
    place_id: Id,
    name: String,
    price: u32,
) -> Result<(), String> {
    let account = core
        .acting(None)
        .ok_or_else(|| "Add an account first.".to_owned())?;
    let u = universe_of(&core, place_id.0).ok_or_else(|| "Still loading this experience.".to_owned())?;
    let name = name.trim();
    if name.is_empty() {
        return Err("Give it a name.".to_owned());
    }
    core.buy_private_server(account, u, name.to_owned(), price.into());
    Ok(())
}

/// Buys a pass from the experience's store. Called only from the dialog `game.buy_pass` opens,
/// after the user saw `price`; nothing is bought if the store now says another price.
#[tauri::command]
#[specta::specta]
pub fn buy_pass(core: State<'_, Arc<Core>>, place_id: Id, pass_id: Id, price: u32) -> Result<(), String> {
    let account = core
        .acting(None)
        .ok_or_else(|| "Add an account first.".to_owned())?;
    let u = universe_of(&core, place_id.0).ok_or_else(|| "Still loading this experience.".to_owned())?;
    let passes = core
        .game_passes(account, u, false)
        .data
        .ok_or_else(|| "Still loading the store.".to_owned())?;
    let p = passes
        .items
        .iter()
        .find(|p| p.id == pass_id.0)
        .ok_or_else(|| "That pass isn't in this store.".to_owned())?;
    if p.owned {
        return Err("You already own this pass.".to_owned());
    }
    if p.price != Some(price.into()) {
        return Err("The price changed. Look again before buying.".to_owned());
    }
    if p.product_id == 0 {
        return Err("This pass can only be bought on the website.".to_owned());
    }
    core.buy_pass(account, p.product_id, p.name.clone(), price.into());
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn game_passes(core: State<'_, Arc<Core>>, place_id: Id) -> List<PassRow> {
    let Some(u) = universe_of(&core, place_id.0) else {
        return List::loading();
    };
    List::paged(core.game_passes(viewer(&core), u, false), |p| PassRow {
        id: Id(p.id),
        name: p.name.clone(),
        description: p.description.clone(),
        price: p.price.map(|x| x.max(0) as u64),
        owned: p.owned,
    })
}

#[tauri::command]
#[specta::specta]
pub fn game_badges(core: State<'_, Arc<Core>>, place_id: Id) -> List<BadgeRow> {
    let Some(u) = universe_of(&core, place_id.0) else {
        return List::loading();
    };
    let r = core.game_badges(u, false);
    let ids: Vec<u64> = r
        .data
        .as_ref()
        .map(|p| p.items.iter().map(|b| b.id).collect())
        .unwrap_or_default();
    let earned = core.badges_earned(viewer(&core), &ids).data.unwrap_or_default();
    List::paged(r, |b| BadgeRow {
        id: Id(b.id),
        name: b.name.clone(),
        description: b.description.clone(),
        win_rate: b.win_rate,
        awarded: b.awarded.max(0) as u64,
        earned: earned.get(&b.id).cloned(),
    })
}

#[tauri::command]
#[specta::specta]
pub fn game_events(core: State<'_, Arc<Core>>, place_id: Id) -> List<EventRow> {
    let Some(u) = universe_of(&core, place_id.0) else {
        return List::loading();
    };
    List::of(core.game_events(viewer(&core), u), |e| EventRow {
        id: e.id.clone(),
        title: e.title.clone(),
        subtitle: e.subtitle.clone(),
        description: e.description.clone(),
        start: e.start.clone(),
        end: e.end.clone(),
        host: e.host.clone(),
        going: e.going,
    })
}

#[tauri::command]
#[specta::specta]
pub fn game_recommended(core: State<'_, Arc<Core>>, place_id: Id) -> List<GameCard> {
    let Some(u) = universe_of(&core, place_id.0) else {
        return List::loading();
    };
    List::of(core.recommended_games(u), |g| GameCard::from(g))
}

/// Step 1 of adding an account: check a pasted cookie. Progress shows in `Shell.login`.
#[tauri::command]
#[specta::specta]
pub fn submit_cookie(core: State<'_, Arc<Core>>, text: String) -> bool {
    core.submit_cookie(&text)
}

/// Step 2: save the checked account and switch to it.
#[tauri::command]
#[specta::specta]
pub fn confirm_login(core: State<'_, Arc<Core>>) {
    core.send(Cmd::ConfirmLogin);
}

/// Stop adding an account: forget an unconfirmed one, cancel a quick sign-in code and close
/// the sign-in window.
#[tauri::command]
#[specta::specta]
pub fn cancel_login(app: AppHandle, core: State<'_, Arc<Core>>) {
    core.cancel_login();
    crate::signin::close(&app);
}

/// A join target from a command argument: `user:<id>` follows a user, a bare number is a place.
fn join_target(arg: &str) -> Option<JoinTarget> {
    let arg = arg.trim();
    match arg.strip_prefix("user:") {
        Some(user) => Some(JoinTarget::FollowUser {
            user_id: user.parse().ok()?,
        }),
        None => Some(JoinTarget::Place {
            place_id: arg.parse().ok()?,
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub fn dismiss_toast(core: State<'_, Arc<Core>>, id: Id) {
    core.dismiss_toast(id.0);
}

pub fn builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            shell,
            commands,
            run_command,
            dismiss_toast,
            friends,
            home,
            game,
            game_servers,
            game_private_servers,
            game_friend_servers,
            creator_bio,
            mentioned_users,
            remote_search,
            game_server_offer,
            server_settings,
            buy_private_server,
            buy_pass,
            game_passes,
            game_badges,
            game_events,
            game_recommended,
            profile,
            profile_badges,
            profile_trading,
            profile_groups,
            profile_creations,
            profile_favorites,
            profile_followed_games,
            profile_collectibles,
            search_results,
            recent_searches,
            user_note,
            people,
            profile_people,
            communities::my_communities,
            communities::community_search,
            communities::community,
            communities::community_games,
            communities::community_store,
            communities::community_passes,
            communities::community_revenue,
            communities::community_affiliates,
            communities::community_roles,
            communities::community_members,
            communities::community_requests,
            communities::community_audit,
            communities::community_settings,
            communities::forum_categories,
            communities::forum_posts,
            communities::forum_comments,
            communities::create_forum_post,
            market::catalog_categories,
            market::catalog_options,
            market::catalog_search,
            market::item_page,
            market::pass_page,
            market::item_resellers,
            market::buy_item,
            market::buy_resale,
            market::item_owners,
            market::item_price_history,
            market::item_recommendations,
            market::wishlist,
            market::find_user,
            market::inventory_categories,
            market::inventory_items,
            market::inventory_passes,
            market::inventory_badges,
            market::inventory_places,
            market::inventory_inspect,
            market::inventory_totals,
            market::values_status,
            market::values_table,
            trades::trade_list,
            trades::trade_detail,
            trades::trade_eligibility,
            trades::trade_draft,
            trades::tradable_items,
            trades::draft_toggle_item,
            trades::draft_set_robux,
            trades::discard_trade_draft,
            transactions::transactions_view,
            transactions::transaction_insights,
            transactions::transaction_accounts,
            inbox::messages,
            inbox::notifications,
            inbox::notification_categories,
            inbox::chat_list,
            inbox::chat_view,
            inbox::take_chat_focus,
            inbox::linked_game,
            inbox::mark_chat_read,
            inbox::chat_typing,
            inbox::start_chat,
            discover::discover,
            discover::discover_chart,
            avatar::avatar,
            avatar::outfits,
            avatar::wardrobe,
            avatar::body_palette,
            avatar::recent_items,
            avatar::profile_look,
            avatar::profile_outfits,
            avatar::outfit_look,
            avatar::avatar_3d,
            account::account_info,
            account::account_sessions,
            account::privacy_settings,
            account::my_private_servers,
            account::app_settings,
            account::prefs,
            account::start_tabs,
            account::save_tabs,
            account::themes,
            account::appearance,
            account::set_system_dark,
            account::font_catalog,
            account::font_files,
            account::theme_import_install,
            account::theme_import_cancel,
            plugins::plugins,
            plugins::plugin_install_finish,
            plugins::plugin_install_cancel,
            plugins::plugin_actions_run,
            plugins::plugin_actions_cancel,
            plugins::plugin_automation,
            plugins::plugin_automation_confirm,
            plugins::plugin_automation_cancel,
            plugins::plugin_activity,
            account::pin_status,
            account::unlock_pin,
            security::security_status,
            security::change_password,
            security::security_set,
            security::security_open,
            security::security_password,
            security::security_code,
            security::security_close,
            security::security_copy,
            challenge::challenge_submit_code,
            challenge::challenge_use_method,
            challenge::challenge_cancel,
            challenge::open_captcha,
            approve::approve_enter,
            approve::approve_confirm,
            approve::approve_close,
            submit_cookie,
            confirm_login,
            cancel_login
        ])
        .events(collect_events![StoreChanged])
}

pub const BINDINGS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/ui/src/lib/ipc/bindings.ts");

/// Writes the TypeScript bindings. Runs on debug startup and in `cargo test`.
pub fn export(builder: &Builder<tauri::Wry>) -> Result<(), String> {
    builder
        .export(specta_typescript::Typescript::default(), BINDINGS)
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Keeps the committed bindings in sync with the Rust types. Commit the file if this
    /// test changed it.
    #[test]
    fn join_targets() {
        assert!(matches!(
            join_target("user:7"),
            Some(JoinTarget::FollowUser { user_id: 7 })
        ));
        assert!(matches!(
            join_target(" 42 "),
            Some(JoinTarget::Place { place_id: 42 })
        ));
        assert!(join_target("user:x").is_none());
        assert!(join_target("game").is_none());
    }

    #[test]
    fn export_bindings() {
        export(&builder()).expect("export bindings");
    }
}
