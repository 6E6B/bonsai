//! The command registry. Every user action is a [`CommandId`]; buttons, menus, shortcuts and
//! the command bar all dispatch through [`Invocation`]s of these ids. A button without a
//! command is a bug.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// What argument a command takes, for completion in the command bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgKind {
    None,
    /// one of your accounts
    Account,
    /// an experience or a friend (join)
    GameOrFriend,
    /// any user
    User,
    /// a settings section
    SettingsSection,
    /// free text
    Text,
    /// a colour tag name
    Color,
    /// an outfit of the active account
    Outfit,
    /// system, dark or light
    Mode,
    /// an installed theme
    Theme,
    /// a Google Fonts family or a bundled font
    Font,
    /// one of the open tabs
    Tab,
    /// a screen that can sit in the sidebar
    Screen,
    /// a setting from [`crate::prefs::PREFS`], then its value
    Setting,
}

/// A keyboard shortcut in egui key names (`"K"`, `"1"`, `"Tab"`, `"Backslash"`, ...).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Shortcut {
    Chord {
        ctrl: bool,
        shift: bool,
        alt: bool,
        key: String,
    },
    /// Two plain keys in a row, like `g h`.
    Sequence(String, String),
}

impl Shortcut {
    pub fn ctrl(key: &str) -> Shortcut {
        Shortcut::Chord {
            ctrl: true,
            shift: false,
            alt: false,
            key: key.into(),
        }
    }
    pub fn ctrl_shift(key: &str) -> Shortcut {
        Shortcut::Chord {
            ctrl: true,
            shift: true,
            alt: false,
            key: key.into(),
        }
    }
    /// Shift plus a letter, for keys whose plain form is taken.
    pub fn shift(key: &str) -> Shortcut {
        Shortcut::Chord {
            ctrl: false,
            shift: true,
            alt: false,
            key: key.into(),
        }
    }
    pub fn alt(key: &str) -> Shortcut {
        Shortcut::Chord {
            ctrl: false,
            shift: false,
            alt: true,
            key: key.into(),
        }
    }
    pub fn key(key: &str) -> Shortcut {
        Shortcut::Chord {
            ctrl: false,
            shift: false,
            alt: false,
            key: key.into(),
        }
    }
    pub fn seq(a: &str, b: &str) -> Shortcut {
        Shortcut::Sequence(a.into(), b.into())
    }
    /// For display, e.g. `Ctrl+K` or `G H`.
    pub fn label(&self) -> String {
        match self {
            Shortcut::Chord {
                ctrl,
                shift,
                alt,
                key,
            } => {
                let mut s = String::new();
                if *ctrl {
                    s.push_str(if cfg!(target_os = "macos") {
                        "Cmd+"
                    } else {
                        "Ctrl+"
                    });
                }
                if *alt {
                    s.push_str("Alt+");
                }
                if *shift {
                    s.push_str("Shift+");
                }
                s.push_str(match key.as_str() {
                    "Backslash" => "\\",
                    "Slash" => "/",
                    "Questionmark" => "?",
                    "PageDown" => "PgDn",
                    "PageUp" => "PgUp",
                    "Period" => ".",
                    k => k,
                });
                s
            }
            Shortcut::Sequence(a, b) => format!("{} {}", a.to_lowercase(), b.to_lowercase()),
        }
    }
}

macro_rules! commands {
    ($($id:ident => $key:literal, $title:literal, [$($kw:literal),*], $arg:ident, [$($sc:expr),*];)*) => {
        /// Every action in the app.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum CommandId { $($id),* }

        impl CommandId {
            pub const ALL: &'static [CommandId] = &[$(CommandId::$id),*];
            /// Stable id for settings (rebinding) and tests.
            pub fn key(self) -> &'static str {
                match self { $(CommandId::$id => $key),* }
            }
        }

        fn build() -> Vec<Command> {
            vec![$(Command {
                id: CommandId::$id,
                title: $title,
                keywords: &[$($kw),*],
                arg: ArgKind::$arg,
                default_shortcuts: vec![$($sc),*],
            }),*]
        }
    };
}

commands! {
    OpenCommandBar => "palette.open", "Search or run a command", ["palette", "search", "find"], None, [Shortcut::ctrl("K"), Shortcut::key("Slash")];
    ShowShortcuts => "help.shortcuts", "Show keyboard shortcuts", ["help", "keys", "keyboard"], None, [Shortcut::key("Questionmark")];

    SwitchAccount => "account.switch", "Switch account", ["switch", "account", "acc"], Account, [];
    SwitchToLast => "account.switch_last", "Switch to last used account", ["switch", "previous", "back"], None, [Shortcut::ctrl("Tab")];
    SwitchTo1 => "account.switch_1", "Switch to account 1", ["switch"], None, [Shortcut::ctrl("1")];
    SwitchTo2 => "account.switch_2", "Switch to account 2", ["switch"], None, [Shortcut::ctrl("2")];
    SwitchTo3 => "account.switch_3", "Switch to account 3", ["switch"], None, [Shortcut::ctrl("3")];
    SwitchTo4 => "account.switch_4", "Switch to account 4", ["switch"], None, [Shortcut::ctrl("4")];
    SwitchTo5 => "account.switch_5", "Switch to account 5", ["switch"], None, [Shortcut::ctrl("5")];
    SwitchTo6 => "account.switch_6", "Switch to account 6", ["switch"], None, [Shortcut::ctrl("6")];
    SwitchTo7 => "account.switch_7", "Switch to account 7", ["switch"], None, [Shortcut::ctrl("7")];
    SwitchTo8 => "account.switch_8", "Switch to account 8", ["switch"], None, [Shortcut::ctrl("8")];
    SwitchTo9 => "account.switch_9", "Switch to account 9", ["switch"], None, [Shortcut::ctrl("9")];
    AddAccount => "account.add", "Add account", ["sign in", "login", "cookie", "new account"], None, [];
    QuickSignIn => "account.quick_sign_in", "Add account with quick sign-in", ["sign in", "login", "code", "quick", "new account"], None, [];
    ApproveCode => "account.approve_code", "Sign in another device with a code", ["quick sign-in", "code", "another device", "approve", "login", "cross-device"], Text, [];
    SignInWithBrowser => "account.sign_in_browser", "Add account with Roblox sign-in page", ["sign in", "login", "browser", "password", "new account"], None, [];
    RenameAccount => "account.rename", "Rename account (local label)", ["label", "rename", "account"], Text, [];
    SetAccountColor => "account.color", "Set account color tag", ["color", "colour", "tag"], Color, [];
    MoveAccountUp => "account.move_up", "Move account up in the rail", ["reorder", "order"], None, [];
    MoveAccountDown => "account.move_down", "Move account down in the rail", ["reorder", "order"], None, [];
    RemoveAccount => "account.remove", "Remove account from this app", ["delete", "forget", "remove"], Account, [];
    SignOut => "account.sign_out", "Sign out", ["logout", "log out", "sign out"], None, [];
    SignOutOthers => "account.sign_out_others", "Sign out of all other sessions", ["logout", "sessions", "devices", "security"], None, [];
    UnlockPin => "account.unlock_pin", "Unlock account PIN", ["pin", "unlock", "parental", "settings"], None, [];
    LockPin => "account.lock_pin", "Lock account PIN", ["pin", "lock", "parental"], None, [];
    SetDisplayName => "account.display_name", "Change display name", ["display name", "nickname", "name"], Text, [];
    SetPrivacy => "privacy.set", "Change a privacy setting (setting, then value)", ["privacy", "who can", "inventory", "trade", "chat", "join"], Text, [];
    ReplaceCookie => "account.replace_cookie", "Paste a new cookie for this account", ["expired", "session", "cookie"], None, [];

    GoHome => "nav.home", "Go to Home", ["home"], None, [Shortcut::seq("G", "H")];
    GoDiscover => "nav.discover", "Go to Discover", ["discover", "charts", "sorts"], None, [Shortcut::seq("G", "D")];
    GoAvatar => "nav.avatar", "Go to Avatar", ["avatar", "outfit", "wear"], None, [Shortcut::seq("G", "A")];
    GoMarketplace => "nav.marketplace", "Go to Marketplace", ["catalog", "shop", "items"], None, [Shortcut::seq("G", "M")];
    GoInventory => "nav.inventory", "Go to Inventory", ["inventory", "items"], None, [Shortcut::seq("G", "I")];
    GoTrades => "nav.trades", "Go to Trades", ["trades", "trading"], None, [Shortcut::seq("G", "T")];
    GoValues => "nav.values", "Go to Values (Rolimons)", ["values", "rolimons", "limiteds", "rap", "demand", "worth"], None, [Shortcut::seq("G", "V")];
    GoTransactions => "nav.transactions", "Go to Transactions", ["transactions", "robux", "sales", "purchases", "spending", "income", "stipend", "ledger", "money"], None, [Shortcut::seq("G", "R")];
    GoMessages => "nav.messages", "Go to Messages", ["messages", "inbox", "pm"], None, [Shortcut::seq("G", "N")];
    GoCommunities => "nav.communities", "Go to Communities", ["groups", "communities"], None, [Shortcut::seq("G", "C")];
    GoProfile => "nav.profile", "Go to your profile", ["profile", "me"], None, [Shortcut::seq("G", "P")];
    GoFriends => "nav.friends", "Go to Friends", ["friends", "connections", "requests", "followers"], None, [Shortcut::seq("G", "F")];
    OpenSettings => "nav.settings", "Settings", ["settings", "preferences", "options"], SettingsSection, [Shortcut::ctrl("Comma")];
    OpenGame => "game.open", "Open experience", ["game", "experience", "place", "open"], Text, [];
    OpenItem => "market.open", "Open item", ["item", "asset", "bundle", "pass", "open"], Text, [];
    PreviewItem => "market.preview", "Preview item in a side panel", ["item", "asset", "bundle", "preview", "peek", "details", "inspect", "side"], Text, [];
    OpenCommunity => "group.open", "Open community", ["group", "community", "open"], Text, [];
    GoComponents => "nav.components", "Go to Components (UI gallery)", ["components", "gallery", "ui", "design"], None, [];
    GoOverview => "nav.overview", "Go to Overview (console clone test)", ["overview", "console", "clone", "test", "card"], None, [];
    GoThemes => "nav.themes", "Go to Themes", ["themes", "appearance", "colors", "palette", "editor", "customize"], None, [Shortcut::seq("G", "E")];

    ToggleDrawer => "view.drawer", "Toggle friends and chat drawer", ["friends", "chat", "drawer", "panel"], None, [Shortcut::ctrl("Backslash")];
    ToggleSidebar => "view.sidebar", "Collapse or expand the sidebar", ["sidebar", "collapse"], None, [Shortcut::ctrl("B")];
    ToggleSidebarScreen => "view.sidebar_toggle", "Show or hide in the sidebar", ["sidebar", "hide", "show", "remove", "customize", "screens"], Screen, [];
    SidebarMoveUp => "view.sidebar_up", "Move up in the sidebar", ["sidebar", "reorder", "order", "customize"], Screen, [];
    SidebarMoveDown => "view.sidebar_down", "Move down in the sidebar", ["sidebar", "reorder", "order", "customize"], Screen, [];
    SetSidebarOrder => "view.sidebar_order", "Set sidebar order (screens, in order)", ["sidebar", "reorder", "order", "customize"], Text, [];
    ResetSidebar => "view.sidebar_reset", "Reset sidebar order and hidden screens", ["sidebar", "reset", "default", "customize"], None, [];
    SetMode => "view.theme", "Light, dark or follow the system", ["theme", "mode", "dark", "light", "system", "appearance"], Mode, [];
    ApplyTheme => "theme.apply", "Use theme", ["theme", "color", "colour", "appearance", "skin", "palette"], Theme, [];
    CycleTheme => "theme.cycle", "Next theme", ["theme", "cycle", "switch", "appearance"], None, [];
    DeleteTheme => "theme.delete", "Delete theme", ["theme", "remove"], Theme, [];
    DuplicateTheme => "theme.duplicate", "Duplicate theme", ["theme", "copy", "clone", "new"], Theme, [];
    SetThemeFont => "theme.font", "Theme font", ["theme", "font", "typeface", "google", "text"], Font, [];
    SetThemeMonoFont => "theme.mono_font", "Theme code font", ["theme", "font", "mono", "monospace", "code", "google"], Font, [];
    NewTheme => "theme.new", "New theme", ["theme", "create", "make", "editor", "custom"], None, [];
    EditTheme => "theme.edit", "Edit theme", ["theme", "editor", "customize", "colors", "change"], Theme, [];
    SaveTheme => "theme.save", "Save theme", ["theme", "save", "editor"], Text, [];
    RenameTheme => "theme.rename", "Rename theme", ["theme", "name", "rename"], Theme, [];
    RevertTheme => "theme.revert", "Discard unsaved theme changes", ["theme", "discard", "revert", "reset", "editor"], None, [];
    UndoTheme => "theme.undo", "Undo theme change", ["theme", "undo", "editor"], None, [];
    RedoTheme => "theme.redo", "Redo theme change", ["theme", "redo", "editor"], None, [];
    InspectTheme => "theme.inspect", "Inspect: pick part of the preview to edit its tokens", ["theme", "inspect", "pick", "token", "element", "editor"], None, [];
    FixThemeContrast => "theme.fix_contrast", "Fix low contrast in the theme", ["theme", "contrast", "readable", "accessibility", "fix", "wcag"], None, [];
    ThemeLivePreview => "theme.live", "Show theme edits across the whole app (on or off)", ["theme", "live", "preview", "apply", "editor"], None, [];
    SetBackground => "view.background", "Set background picture (https link or file path)", ["background", "wallpaper", "picture", "image", "photo", "gif", "url", "link", "appearance"], Text, [];
    PickBackground => "view.background_file", "Choose background picture from files", ["background", "wallpaper", "picture", "image", "photo", "gif", "file", "upload", "appearance"], None, [];
    ClearBackground => "view.background_clear", "Remove background picture", ["background", "wallpaper", "picture", "remove", "clear", "appearance"], None, [];
    SetBackgroundLevel => "view.background_level", "Background level (image, panels, blur, dim, saturation, tint, vignette, scanlines or grain, then a number)", ["background", "wallpaper", "opacity", "transparency", "see-through", "fade", "blur", "dim", "saturation", "tint", "vignette", "scanlines", "crt", "grain", "noise", "effect", "appearance"], Text, [];
    ResetBackgroundEffects => "view.background_effects_reset", "Reset background effects", ["background", "wallpaper", "effects", "blur", "scanlines", "reset", "clear", "appearance"], None, [];
    SetDensity => "view.density", "Density (compact or comfortable)", ["density", "compact", "comfortable", "dense", "spacing"], Text, [];
    ZoomIn => "view.zoom_in", "Zoom in", ["scale", "bigger", "font"], None, [Shortcut::ctrl("Plus"), Shortcut::ctrl("Equals")];
    ZoomOut => "view.zoom_out", "Zoom out", ["scale", "smaller", "font"], None, [Shortcut::ctrl("Minus")];
    ZoomReset => "view.zoom_reset", "Reset zoom", ["scale"], None, [Shortcut::ctrl("0")];
    MinimizeWindow => "window.minimize", "Minimize window", ["minimize", "hide"], None, [];
    MaximizeWindow => "window.maximize", "Maximize or restore window", ["maximize", "restore", "fullscreen"], None, [];

    NewTab => "tab.new", "New tab", ["tab", "open"], None, [Shortcut::ctrl("T")];
    SwitchTab => "tab.switch", "Switch to tab", ["tab", "go", "find"], Tab, [];
    CloseTab => "tab.close", "Close tab", ["close"], None, [Shortcut::ctrl("W")];
    CloseOtherTabs => "tab.close_others", "Close other tabs", ["close", "tabs", "others"], None, [];
    CloseTabsRight => "tab.close_right", "Close tabs to the right", ["close", "tabs", "right"], None, [];
    PinTab => "tab.pin", "Pin or unpin tab", ["pin", "unpin", "tab"], None, [];
    DuplicateTab => "tab.duplicate", "Duplicate tab", ["copy", "clone", "tab"], None, [];
    NextTab => "tab.next", "Next tab", ["tab"], None, [Shortcut::ctrl("PageDown")];
    PrevTab => "tab.prev", "Previous tab", ["tab"], None, [Shortcut::ctrl("PageUp")];
    MoveTabRight => "tab.move_right", "Move tab right", ["reorder", "tab"], None, [Shortcut::ctrl_shift("PageDown")];
    MoveTabLeft => "tab.move_left", "Move tab left", ["reorder", "tab"], None, [Shortcut::ctrl_shift("PageUp")];
    GoTab1 => "tab.go_1", "Go to tab 1", ["tab"], None, [Shortcut::alt("1")];
    GoTab2 => "tab.go_2", "Go to tab 2", ["tab"], None, [Shortcut::alt("2")];
    GoTab3 => "tab.go_3", "Go to tab 3", ["tab"], None, [Shortcut::alt("3")];
    GoTab4 => "tab.go_4", "Go to tab 4", ["tab"], None, [Shortcut::alt("4")];
    GoTab5 => "tab.go_5", "Go to tab 5", ["tab"], None, [Shortcut::alt("5")];
    GoTab6 => "tab.go_6", "Go to tab 6", ["tab"], None, [Shortcut::alt("6")];
    GoTab7 => "tab.go_7", "Go to tab 7", ["tab"], None, [Shortcut::alt("7")];
    GoTab8 => "tab.go_8", "Go to tab 8", ["tab"], None, [Shortcut::alt("8")];
    GoTabLast => "tab.go_last", "Go to last tab", ["tab", "end"], None, [Shortcut::alt("9")];
    ReopenTab => "tab.reopen", "Reopen closed tab", ["undo", "tab"], None, [Shortcut::ctrl_shift("T")];

    Join => "game.join", "Join", ["join", "play", "launch"], GameOrFriend, [];
    JoinAs => "game.join_as", "Join as another account", ["join", "alt", "as"], GameOrFriend, [];
    JoinServer => "game.join_server", "Join selected server", ["server", "join"], None, [];
    JoinBestServer => "game.join_best", "Join best server for me", ["best", "ping", "server"], None, [];
    JoinSmallestServer => "game.join_smallest", "Join smallest server", ["small", "empty", "server"], None, [];
    JoinRandomServer => "game.join_random", "Join random server", ["random", "server"], None, [];
    JoinJobId => "game.join_job", "Join server by job id", ["job", "instance", "server id"], Text, [];
    CopyServerLink => "game.copy_server_link", "Copy server invite link", ["invite", "share", "server"], None, [];
    FindUser => "user.find", "Find which server a user is in", ["find", "where", "server"], User, [];
    OpenProfile => "user.open", "Open profile", ["profile", "user", "player"], User, [];
    ShowFriendsOf => "user.friends", "Show a user's friends", ["friends", "connections", "mutuals", "mutual friends"], User, [];
    ShowFollowersOf => "user.followers", "Show a user's followers", ["followers", "fans"], User, [];
    ShowFollowingOf => "user.following", "Show who a user follows", ["following", "followings", "follows"], User, [];
    AddFriend => "user.add_friend", "Add friend", ["friend", "connect", "request"], User, [];
    AddFriendAs => "user.add_friend_as", "Add friend as another account", ["friend", "connect", "request", "alt", "as"], User, [];
    Unfriend => "user.unfriend", "Remove friend", ["unfriend", "remove", "connection"], User, [];
    AcceptFriend => "user.accept_friend", "Accept friend request", ["accept", "request"], User, [];
    DeclineFriend => "user.decline_friend", "Decline friend request", ["decline", "ignore", "request"], User, [];
    DeclineAllFriends => "friends.decline_all", "Ignore all friend requests", ["decline", "ignore", "requests"], None, [];
    Follow => "user.follow", "Follow", ["follow"], User, [];
    Unfollow => "user.unfollow", "Unfollow", ["unfollow"], User, [];
    SetNickname => "user.nickname", "Set a nickname for someone (user, then nickname; empty clears)", ["nickname", "alias", "name", "friend"], Text, [];
    SetNote => "user.note", "Write a private note about someone (user, then note; empty clears)", ["note", "notes", "memo", "friend"], Text, [];
    Block => "user.block", "Block user", ["block"], User, [];
    Unblock => "user.unblock", "Unblock user", ["unblock"], User, [];
    LikeGame => "game.like", "Like this experience", ["like", "thumbs up", "vote", "upvote"], None, [];
    DislikeGame => "game.dislike", "Dislike this experience", ["dislike", "thumbs down", "vote", "downvote"], None, [];
    FavoriteGame => "game.favorite", "Favorite or unfavorite this experience", ["favorite", "favourite", "star"], None, [];
    NotifyGame => "game.notify", "Turn update notifications on or off", ["notify", "follow", "bell", "updates"], None, [];
    NotifyEvent => "game.event_notify", "Notify me about an event", ["event", "rsvp", "notify", "remind"], Text, [];
    BuyPass => "game.buy_pass", "Buy a pass", ["pass", "gamepass", "store", "buy", "purchase"], Text, [];
    BuyPrivateServer => "game.buy_private_server", "Buy a private server", ["private server", "vip", "buy", "create"], Text, [];
    JoinPrivateServer => "game.join_private_server", "Join a private server", ["private server", "vip", "join"], Text, [];
    RenamePrivateServer => "server.rename", "Rename private server (id, then name)", ["private server", "vip", "name"], Text, [];
    ToggleServerActive => "server.toggle_active", "Turn private server on or off", ["private server", "vip", "active", "disable"], Text, [];
    NewServerLink => "server.new_link", "Make a new private server link", ["private server", "vip", "link", "invite", "regenerate"], Text, [];
    ToggleServerFriends => "server.toggle_friends", "Let friends join private server", ["private server", "vip", "friends", "allow"], Text, [];
    AddServerUser => "server.add_user", "Let someone into a private server (id, then user)", ["private server", "vip", "whitelist", "invite", "allow"], Text, [];
    RemoveServerUser => "server.remove_user", "Take someone off a private server (id, then user)", ["private server", "vip", "whitelist", "remove"], Text, [];
    ToggleServerRenew => "server.toggle_renew", "Turn private server renewal on or off", ["private server", "vip", "subscription", "renew", "cancel"], Text, [];
    LoadMore => "list.more", "Load more", ["more", "next page", "load"], Text, [];
    RefreshServers => "game.refresh_servers", "Refresh server list", ["reload", "servers"], None, [Shortcut::key("R")];

    WearItem => "avatar.wear", "Wear or take off an item", ["wear", "equip", "remove", "take off", "avatar"], Text, [];
    WearOutfit => "avatar.outfit", "Wear an outfit", ["outfit", "wear", "costume", "avatar"], Outfit, [];
    SaveOutfit => "avatar.save_outfit", "Save current look as an outfit", ["outfit", "save", "create", "avatar"], Text, [];
    SetRig => "avatar.rig", "Switch body type (R6 or R15)", ["r6", "r15", "rig", "body", "avatar"], Text, [];
    UpdateOutfit => "avatar.update_outfit", "Update outfit with current look", ["outfit", "overwrite", "replace", "save", "avatar"], Outfit, [];
    RenameOutfit => "avatar.rename_outfit", "Rename outfit (id, then name)", ["outfit", "rename", "name", "avatar"], Text, [];
    DeleteOutfit => "avatar.delete_outfit", "Delete outfit", ["outfit", "delete", "remove", "avatar"], Outfit, [];
    MoveLayer => "avatar.layer", "Move layered clothing up or down (item id, then up or down)", ["layer", "order", "layered clothing", "on top", "under", "avatar"], Text, [];
    SetBodyColor => "avatar.color", "Set body color (color id, or part:color id)", ["skin", "color", "colour", "tone", "body", "avatar"], Text, [];
    SetScale => "avatar.scale", "Set body scale (scale, then value)", ["height", "width", "head", "proportion", "body type", "scale", "size", "avatar"], Text, [];
    SetEmote => "avatar.emote", "Put an emote in a hotbar slot (slot 1-8, then item id; slot alone clears it)", ["emote", "hotbar", "slot", "dance", "avatar"], Text, [];
    RedrawAvatar => "avatar.redraw", "Redraw avatar image", ["redraw", "refresh", "thumbnail", "render", "avatar"], None, [];
    Toggle3d => "avatar.view_3d", "Show the avatar in 3D or as an image", ["3d", "2d", "model", "render", "preview", "spin", "avatar"], None, [];
    SetAvatarView => "avatar.default_view", "Open avatars in 3D or as an image (3d or 2d)", ["3d", "2d", "default", "model", "image", "setting", "avatar"], Text, [];
    PlayAnimation => "avatar.play", "Play an animation in the 3D view (idle, walk, run, jump, fall, climb, swim, or an emote id)", ["animation", "emote", "dance", "play", "pose", "3d", "avatar"], Text, [];
    Orbit3d => "avatar.orbit", "Move the 3D camera (left, right, up, down, in, out or reset)", ["camera", "rotate", "orbit", "zoom", "turn", "reset", "3d", "avatar"], Text, [];

    SearchMarketplace => "market.search", "Search the Marketplace", ["catalog", "shop", "items", "find item"], Text, [];
    BuyItem => "market.buy", "Buy this item", ["buy", "purchase", "get"], None, [];
    BuyItemAs => "market.buy_as", "Buy this item as another account", ["buy", "purchase", "alt", "as"], Account, [];
    BuyResale => "market.buy_resale", "Buy a reseller's copy", ["buy", "resale", "reseller", "limited", "copy"], Text, [];
    FavoriteItem => "market.favorite", "Favorite or unfavorite this item", ["favorite", "favourite", "star"], None, [];
    WishlistItem => "market.wishlist", "Add to or remove from wishlist", ["wishlist", "want", "save", "watch"], None, [];
    DownloadTemplate => "market.template", "Download clothing template", ["template", "download", "save", "shirt", "pants", "t-shirt", "clothing", "image"], Text, [];
    DownloadMaskedTemplate => "market.template_masked", "Download clothing template, masked", ["template", "masked", "mask", "download", "save", "shirt", "pants", "clothing", "clean"], Text, [];
    ToggleItemValues => "market.values", "Turn item values from Rolimons on or off", ["values", "rolimons", "worth", "rap"], None, [];
    PriceHistoryRange => "market.history_range", "Set the price history range (7d, 30d, 90d, 1y, all; no argument cycles)", ["price history", "chart", "range", "rap", "sales", "limited"], Text, [];
    PriceHistoryZoom => "market.history_zoom", "Zoom the price history chart (in, out, reset)", ["price history", "chart", "zoom", "pan", "limited"], Text, [];
    TryOn => "market.try_on", "Try on this item in 3D", ["try on", "preview", "wear", "fit", "3d", "avatar"], None, [];
    PriceHistoryScale => "market.history_scale", "Show the price history on a log or linear scale (log, linear; no argument toggles)", ["price history", "chart", "log", "scale", "limited"], Text, [];
    PriceHistoryView => "market.history_view", "Show price history as a chart or a table (chart, table; no argument toggles)", ["price history", "chart", "table", "sales", "limited"], Text, [];
    SearchAuditLog => "group.audit_search", "Search the community audit log", ["audit", "log", "search", "performer", "community", "group"], Text, [];
    ViewInventory => "inventory.view", "Open a user's inventory", ["inventory", "items", "collectibles", "limiteds"], User, [];
    DeleteInventoryItem => "inventory.delete", "Delete an item from your inventory", ["delete", "remove", "inventory"], Text, [];
    SortInventory => "inventory.sort", "Sort the inventory (newest, oldest, price_asc, price_desc, rap, value, demand)", ["inventory", "sort", "order", "price", "rap", "oldest", "newest"], Text, [];
    FilterInventory => "inventory.filter", "Filter the inventory (all, limited, wearing, copies, for_sale)", ["inventory", "filter", "show only", "limited", "wearing", "duplicates"], Text, [];
    InspectInventoryItem => "inventory.inspect", "Inspect an inventory item", ["inventory", "inspect", "details", "drawer", "info"], Text, [];
    TradeWith => "trade.new", "Send a trade to", ["trade", "offer", "new trade"], User, [];
    AcceptTrade => "trade.accept", "Accept trade", ["accept", "trade"], Text, [];
    DeclineTrade => "trade.decline", "Decline trade", ["decline", "trade", "cancel"], Text, [];
    CounterTrade => "trade.counter", "Counter trade", ["counter", "trade"], Text, [];
    SendTrade => "trade.send", "Send the trade you're making", ["send", "trade", "offer"], None, [];
    CopyTradesCsv => "trade.export", "Copy the trade list as CSV", ["export", "csv", "history", "copy"], None, [];
    TxPeriod => "transactions.period", "Set the transactions period (day, week, month, year, all; no argument cycles)", ["transactions", "period", "range", "month", "year", "all time"], Text, [];
    TxType => "transactions.type", "Show one transaction type (sales, purchases, stipends, ...; empty shows all)", ["transactions", "type", "filter", "sales", "purchases", "stipends", "payouts"], Text, [];
    TxScope => "transactions.scope", "Show your transactions or a community's (me, or a community name or ID)", ["transactions", "community", "group", "revenue", "funds"], Text, [];
    TxRange => "transactions.range", "Show transactions between two dates (2026-01-01..2026-03-31; empty clears)", ["transactions", "dates", "from", "to", "between"], Text, [];
    TxPending => "transactions.pending", "Show only pending Robux, or everything (no argument toggles)", ["transactions", "pending", "held", "escrow"], Text, [];
    TxSync => "transactions.sync", "Sync transactions now", ["transactions", "refresh", "reload", "sync"], None, [];
    TxLoadOlder => "transactions.load_older", "Fetch older transaction history", ["transactions", "history", "older", "backfill", "all"], None, [];
    TxStop => "transactions.stop", "Stop fetching older transactions", ["transactions", "stop", "cancel"], None, [];
    TxExport => "transactions.export", "Save the shown transactions as CSV", ["transactions", "export", "csv", "save", "download"], None, [];
    TxCopyCsv => "transactions.copy_csv", "Copy the shown transactions as CSV", ["transactions", "export", "csv", "copy"], None, [];

    MessageUser => "user.message", "Message (chat)", ["chat", "message", "dm", "pm", "whisper"], User, [];
    NewGroupChat => "chat.new_group", "Start a chat or group chat", ["new chat", "group chat", "compose", "new message"], None, [];
    SendChat => "chat.send", "Send chat message", ["send", "say"], Text, [];
    ResendChat => "chat.resend", "Send a failed chat message again", ["resend", "retry", "failed", "chat"], Text, [];
    DiscardChat => "chat.discard", "Discard a failed chat message", ["discard", "delete", "failed", "chat"], Text, [];
    RenameChat => "chat.rename", "Rename this group chat", ["rename", "title", "chat"], Text, [];
    AddToChat => "chat.add_people", "Add people to this chat", ["add", "invite", "chat"], User, [];
    RemoveFromChat => "chat.remove_person", "Remove someone from this chat", ["remove", "kick", "chat"], User, [];
    LeaveChat => "chat.leave", "Leave this group chat", ["leave", "exit", "chat"], None, [];
    PinChat => "chat.pin", "Pin or unpin this chat", ["pin", "unpin", "chat"], None, [];
    ShareExperience => "chat.share_experience", "Share an experience in this chat (link or ID)", ["share", "experience", "game", "link", "chat"], Text, [];
    ChatEmoji => "chat.emoji", "Insert an emoji", ["emoji", "emoticon", "smiley", "chat"], None, [Shortcut::ctrl("Period")];
    ReplyMessage => "message.reply", "Reply to the message (in chat)", ["reply", "respond", "message"], Text, [Shortcut::shift("R")];
    ToggleMessageRead => "message.toggle_read", "Mark message read or unread", ["read", "unread", "message"], Text, [Shortcut::key("U")];
    ArchiveMessage => "message.archive", "Archive message, or move it back to Inbox", ["archive", "unarchive", "message"], Text, [Shortcut::key("E")];
    SelectAllMessages => "message.select_all", "Select all loaded messages, or clear the selection", ["select all", "bulk", "messages"], None, [];
    MarkAllMessagesRead => "message.mark_all_read", "Mark all messages read", ["read all", "messages", "inbox"], None, [];
    FilterNotifications => "notifications.filter", "Show all or only unread notifications (all, unread; no argument toggles)", ["notifications", "unread", "filter", "bell"], Text, [];
    FilterNotificationCategory => "notifications.category", "Show one notification category (friends, trades, communities, experiences, account, messages, other; no argument shows all)", ["notifications", "category", "filter", "friends", "trades", "security", "robux", "group", "experience"], Text, [];
    ClearNotifications => "notifications.mark_read", "Mark notifications read", ["notifications", "bell", "clear", "read"], None, [];
    GoChat => "nav.chat", "Go to Chat", ["chat", "conversations"], None, [];
    GoNotifications => "nav.notifications", "Go to Notifications", ["notifications", "bell", "alerts"], None, [];

    SearchCommunities => "group.search", "Search communities", ["groups", "communities", "find group"], Text, [];
    JoinCommunity => "group.join", "Join this community", ["join", "group", "community"], Text, [];
    LeaveCommunity => "group.leave", "Leave a community", ["leave", "group", "community"], Text, [];
    SetPrimaryCommunity => "group.primary", "Make or clear primary community", ["primary", "group", "community", "main"], Text, [];
    PostShout => "group.shout", "Post a shout", ["shout", "status", "announce", "group"], Text, [];
    AcceptJoinRequest => "group.accept_request", "Accept join request", ["accept", "join request", "group"], User, [];
    DeclineJoinRequest => "group.decline_request", "Decline join request", ["decline", "join request", "group"], User, [];
    AcceptSelectedRequests => "group.accept_selected", "Accept selected join requests", ["accept", "bulk", "join requests"], None, [];
    DeclineSelectedRequests => "group.decline_selected", "Decline selected join requests", ["decline", "bulk", "join requests"], None, [];
    ChangeRank => "group.change_rank", "Change a member's rank", ["rank", "role", "promote", "demote", "group"], Text, [];
    KickMember => "group.kick", "Remove a member from the community", ["kick", "exile", "remove", "group"], User, [];
    NewForumPost => "group.new_post", "New forum post", ["forum", "post", "wall", "group"], Text, [];
    ReplyForumPost => "group.reply", "Reply to the open forum post", ["reply", "forum", "comment"], Text, [];
    DeleteForumPost => "group.delete_post", "Delete a forum post", ["delete", "forum", "post"], Text, [];
    DeleteForumReply => "group.delete_reply", "Delete a forum reply", ["delete", "forum", "reply", "comment"], Text, [];
    LeaveSelectedCommunities => "group.leave_selected", "Leave the selected communities", ["leave", "cleanup", "bulk", "groups"], None, [];
    CopyMembersCsv => "group.export_members", "Copy loaded members as CSV", ["export", "csv", "members", "group"], None, [];
    ToggleCommunitySetting => "group.setting", "Turn a community setting on or off", ["settings", "approval", "group", "community"], Text, [];
    PayOut => "group.payout", "Pay out Robux to a member (user, then amount)", ["payout", "pay", "robux", "funds", "revenue", "group"], Text, [];
    SetRecurringPayouts => "group.recurring_payouts", "Set recurring payouts (user:percent, …)", ["payout", "recurring", "split", "percentage", "revenue", "funds", "group"], Text, [];
    SendAllyRequest => "group.ally", "Send an ally request to a community", ["ally", "allies", "affiliate", "group", "community"], Text, [];
    RemoveAlly => "group.remove_ally", "Remove an ally", ["ally", "allies", "affiliate", "remove", "group"], Text, [];
    DeclareEnemy => "group.enemy", "Declare a community an enemy", ["enemy", "enemies", "affiliate", "group", "community"], Text, [];
    RemoveEnemy => "group.remove_enemy", "Remove an enemy", ["enemy", "enemies", "affiliate", "remove", "group"], Text, [];

    SearchRoblox => "search.open", "Search Roblox (results page)", ["search", "find", "results", "people", "experiences", "groups", "marketplace"], Text, [];
    ClearSearchHistory => "search.clear_history", "Clear recent searches", ["search", "history", "recent", "clear"], None, [];

    CopyId => "entity.copy_id", "Copy ID", ["copy", "id"], None, [];
    CopyLink => "entity.copy_link", "Copy link", ["copy", "link", "url"], None, [];
    OpenOnWebsite => "entity.open_website", "Open on website", ["browser", "web", "site"], None, [Shortcut::ctrl_shift("O")];
    Report => "entity.report", "Report (on the website)", ["report", "abuse"], None, [];
    Refresh => "view.refresh", "Refresh", ["reload"], None, [Shortcut::key("F5")];
    SetPlayerCommand => "app.player_command", "Set how Roblox is launched (empty for the default)", ["launch", "player", "sober", "flatpak", "wine", "command"], Text, [];
    SetFormatLocale => "app.format_locale", "Set number and date format (e.g. de-DE; empty for the system's)", ["language", "locale", "region", "format", "date", "number"], Text, [];
    SetNotify => "app.notify", "Turn a notification kind on or off (friend_joins, trades, chats, friend_requests, experience_updates, sales)", ["notifications", "alerts", "desktop", "mute", "sales"], Text, [];
    SetSetting => "settings.set", "Change a setting", ["setting", "preference", "option", "change", "turn on", "turn off"], Setting, [];
    ResetSettings => "settings.reset", "Reset settings to their defaults", ["reset", "default", "restore", "settings"], Setting, [];
    ClearCache => "app.clear_cache", "Clear cached data and images", ["cache", "clear", "disk", "storage"], None, [];

    NextSubTab => "view.next_subtab", "Next sub-tab", ["tab", "next"], None, [Shortcut::key("CloseBracket")];
    PrevSubTab => "view.prev_subtab", "Previous sub-tab", ["tab", "previous"], None, [Shortcut::key("OpenBracket")];
    ListDown => "list.down", "Move down", ["next"], None, [Shortcut::key("J")];
    ListUp => "list.up", "Move up", ["previous"], None, [Shortcut::key("K")];
    ListOpen => "list.open", "Open selected", ["open"], None, [Shortcut::key("Enter")];
    ListSelect => "list.select", "Select for bulk action", ["select", "mark"], None, [Shortcut::key("X")];

    Quit => "app.quit", "Quit", ["exit", "close"], None, [Shortcut::ctrl("Q")];
}

/// A registered command.
#[derive(Clone, Debug)]
pub struct Command {
    pub id: CommandId,
    pub title: &'static str,
    pub keywords: &'static [&'static str],
    pub arg: ArgKind,
    pub default_shortcuts: Vec<Shortcut>,
}

/// A request to run a command, with an optional argument and acting account.
#[derive(Clone, Debug, PartialEq)]
pub struct Invocation {
    pub id: CommandId,
    pub arg: Option<String>,
    /// "Act as": run as this account instead of the active one.
    pub as_account: Option<rbx_api::AccountId>,
}

impl From<CommandId> for Invocation {
    fn from(id: CommandId) -> Self {
        Invocation {
            id,
            arg: None,
            as_account: None,
        }
    }
}

impl Invocation {
    pub fn with_arg(id: CommandId, arg: impl Into<String>) -> Self {
        Invocation {
            id,
            arg: Some(arg.into()),
            as_account: None,
        }
    }
}

/// All commands plus the user's rebindings.
pub struct Registry {
    commands: Vec<Command>,
    overrides: HashMap<CommandId, Vec<Shortcut>>,
}

impl Default for Registry {
    fn default() -> Self {
        Registry {
            commands: build(),
            overrides: HashMap::new(),
        }
    }
}

impl Registry {
    pub fn all(&self) -> &[Command] {
        &self.commands
    }

    pub fn get(&self, id: CommandId) -> &Command {
        // `build()` registers every id (checked by a test), so the fallback is unreachable.
        self.commands
            .iter()
            .find(|c| c.id == id)
            .unwrap_or(&self.commands[0])
    }

    pub fn shortcuts(&self, id: CommandId) -> &[Shortcut] {
        self.overrides
            .get(&id)
            .map(Vec::as_slice)
            .unwrap_or(&self.get(id).default_shortcuts)
    }

    /// First shortcut, for display next to buttons and in the palette.
    pub fn shortcut_label(&self, id: CommandId) -> Option<String> {
        self.shortcuts(id).first().map(Shortcut::label)
    }

    /// Rebind a command. An empty list removes its shortcuts.
    pub fn rebind(&mut self, id: CommandId, shortcuts: Vec<Shortcut>) {
        self.overrides.insert(id, shortcuts);
    }

    pub fn overrides(&self) -> &HashMap<CommandId, Vec<Shortcut>> {
        &self.overrides
    }

    pub fn load_overrides(&mut self, o: HashMap<CommandId, Vec<Shortcut>>) {
        self.overrides = o;
    }

    /// Commands bound to this shortcut.
    pub fn bound_to(&self, s: &Shortcut) -> Vec<CommandId> {
        CommandId::ALL
            .iter()
            .copied()
            .filter(|id| self.shortcuts(*id).contains(s))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn every_id_registered_once_with_unique_keys() {
        let r = Registry::default();
        assert_eq!(r.all().len(), CommandId::ALL.len());
        let ids: HashSet<_> = r.all().iter().map(|c| c.id).collect();
        assert_eq!(ids.len(), CommandId::ALL.len());
        let keys: HashSet<_> = CommandId::ALL.iter().map(|c| c.key()).collect();
        assert_eq!(keys.len(), CommandId::ALL.len());
    }

    #[test]
    fn no_default_shortcut_conflicts() {
        let r = Registry::default();
        let mut seen: HashMap<Shortcut, CommandId> = HashMap::new();
        for c in r.all() {
            for s in &c.default_shortcuts {
                if let Some(other) = seen.insert(s.clone(), c.id) {
                    panic!("{:?} and {:?} share {}", other, c.id, s.label());
                }
            }
        }
    }

    #[test]
    fn rebinding() {
        let mut r = Registry::default();
        assert_eq!(r.bound_to(&Shortcut::ctrl("K")), vec![CommandId::OpenCommandBar]);
        r.rebind(CommandId::OpenCommandBar, vec![Shortcut::ctrl("P")]);
        assert!(r.bound_to(&Shortcut::ctrl("K")).is_empty());
        assert!(
            r.shortcut_label(CommandId::OpenCommandBar)
                .unwrap()
                .ends_with('P')
        );
    }
}
