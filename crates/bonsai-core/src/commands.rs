//! The command registry. Every user action is a [`CommandId`]; buttons, menus, shortcuts and
//! the command bar all dispatch through [`Invocation`]s of these ids. A button without a
//! command is a bug.

use std::collections::{BTreeMap, HashMap};

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
    /// an installed plugin
    Plugin,
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
    /// Text that [`Shortcut::parse`] reads back: `Ctrl+Shift+K`, `Alt+Slash`, `G H`.
    pub fn spec(&self) -> String {
        match self {
            Shortcut::Chord {
                ctrl,
                shift,
                alt,
                key,
            } => {
                let mut s = String::new();
                for (on, name) in [(ctrl, "Ctrl+"), (alt, "Alt+"), (shift, "Shift+")] {
                    if *on {
                        s.push_str(name);
                    }
                }
                s.push_str(key);
                s
            }
            Shortcut::Sequence(a, b) => format!("{a} {b}"),
        }
    }

    /// Reads a shortcut: `Ctrl+Shift+K` (Cmd counts as Ctrl) or two plain keys in a row, `G H`.
    /// Key names are bonsai-core's (`K`, `1`, `Slash`, `PageDown`); single characters are upper-cased.
    pub fn parse(text: &str) -> Result<Shortcut, String> {
        fn key(k: &str) -> Result<String, String> {
            let k = k.trim();
            let ok = !k.is_empty() && k.len() <= 16 && k.chars().all(|c| c.is_ascii_graphic() && c != '+');
            if !ok {
                return Err(format!("\"{k}\" isn't a key."));
            }
            Ok(if k.chars().count() == 1 {
                k.to_ascii_uppercase()
            } else {
                k.to_owned()
            })
        }
        let parts: Vec<&str> = text.split_whitespace().collect();
        match parts.as_slice() {
            [a, b] if !a.contains('+') && !b.contains('+') => Ok(Shortcut::Sequence(key(a)?, key(b)?)),
            [chord] => {
                let mut mods: Vec<&str> = chord.split('+').collect();
                // A trailing "+" is the Plus key: `Ctrl++`.
                let last = if chord.ends_with("++") {
                    mods.truncate(mods.len() - 2);
                    "Plus"
                } else {
                    mods.pop().unwrap_or_default()
                };
                let (mut ctrl, mut shift, mut alt) = (false, false, false);
                for m in mods {
                    match m.to_ascii_lowercase().as_str() {
                        "ctrl" | "control" | "cmd" | "meta" => ctrl = true,
                        "shift" => shift = true,
                        "alt" | "option" => alt = true,
                        _ => return Err(format!("\"{m}\" isn't Ctrl, Shift or Alt.")),
                    }
                }
                Ok(Shortcut::Chord {
                    ctrl,
                    shift,
                    alt,
                    key: key(last)?,
                })
            }
            _ => Err("Type a key with Ctrl, Shift or Alt (Ctrl+K), or two keys in a row (G H).".into()),
        }
    }

    /// True when pressing one would get in the way of the other: the same keys, or a plain key
    /// that starts a sequence.
    pub fn clashes(&self, other: &Shortcut) -> bool {
        let plain = |s: &Shortcut| match s {
            Shortcut::Chord {
                ctrl: false,
                alt: false,
                key,
                ..
            } => Some(key.clone()),
            _ => None,
        };
        let first = |s: &Shortcut| match s {
            Shortcut::Sequence(a, _) => Some(a.clone()),
            _ => None,
        };
        self == other
            || (plain(self).is_some() && plain(self) == first(other))
            || (first(self).is_some() && first(self) == plain(other))
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

/// How risky a write is. Decides what a plugin may do with it; see `docs/plugins.md`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tier {
    /// Reversible and seen only by you: wear an item, favorite, join a game, a note.
    Harmless = 1,
    /// Visible to other people: friends, follows, blocks, chat, communities.
    Social = 2,
    /// Robux or items can be lost: buying, trades, payouts, private servers.
    Value = 3,
    /// Sign-in, security and privacy. Never available to plugins.
    Account = 4,
}

/// What running a command does.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Effect {
    /// Opens, shows or copies something. Changes nothing.
    Read,
    /// Changes this app on this computer: settings, themes, layout, tabs, account labels.
    /// Never available to plugins.
    Local,
    /// Acts as an account, on Roblox or in its per-account data here.
    Write(Tier),
    /// Acts as the independent Bonsai publisher. Never available to plugins.
    Hub,
}

impl Effect {
    /// The tier of a write, or `None` for reads and local changes.
    pub fn tier(self) -> Option<Tier> {
        match self {
            Effect::Write(t) => Some(t),
            Effect::Read | Effect::Local | Effect::Hub => None,
        }
    }
}

macro_rules! commands {
    ($($id:ident => $key:literal, $title:literal, [$($kw:literal),*], $arg:ident, $effect:expr, [$($sc:expr),*];)*) => {
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
            use Effect::*;
            use Tier::*;
            vec![$(Command {
                id: CommandId::$id,
                title: $title,
                keywords: &[$($kw),*],
                arg: ArgKind::$arg,
                effect: $effect,
                default_shortcuts: vec![$($sc),*],
            }),*]
        }
    };
}

commands! {
    OpenCommandBar => "palette.open", "Search or run a command", ["palette", "search", "find"], None, Read, [Shortcut::ctrl("K"), Shortcut::key("Slash")];
    ShowShortcuts => "help.shortcuts", "Show keyboard shortcuts", ["help", "keys", "keyboard"], None, Read, [Shortcut::key("Questionmark")];

    SwitchAccount => "account.switch", "Switch account", ["switch", "account", "acc"], Account, Local, [];
    SwitchToLast => "account.switch_last", "Switch to last used account", ["switch", "previous", "back"], None, Local, [Shortcut::ctrl("Tab")];
    SwitchTo1 => "account.switch_1", "Switch to account 1", ["switch"], None, Local, [Shortcut::ctrl("1")];
    SwitchTo2 => "account.switch_2", "Switch to account 2", ["switch"], None, Local, [Shortcut::ctrl("2")];
    SwitchTo3 => "account.switch_3", "Switch to account 3", ["switch"], None, Local, [Shortcut::ctrl("3")];
    SwitchTo4 => "account.switch_4", "Switch to account 4", ["switch"], None, Local, [Shortcut::ctrl("4")];
    SwitchTo5 => "account.switch_5", "Switch to account 5", ["switch"], None, Local, [Shortcut::ctrl("5")];
    SwitchTo6 => "account.switch_6", "Switch to account 6", ["switch"], None, Local, [Shortcut::ctrl("6")];
    SwitchTo7 => "account.switch_7", "Switch to account 7", ["switch"], None, Local, [Shortcut::ctrl("7")];
    SwitchTo8 => "account.switch_8", "Switch to account 8", ["switch"], None, Local, [Shortcut::ctrl("8")];
    SwitchTo9 => "account.switch_9", "Switch to account 9", ["switch"], None, Local, [Shortcut::ctrl("9")];
    AddAccount => "account.add", "Add account", ["sign in", "login", "cookie", "new account"], None, Write(Account), [];
    QuickSignIn => "account.quick_sign_in", "Add account with quick sign-in", ["sign in", "login", "code", "quick", "new account"], None, Write(Account), [];
    ApproveCode => "account.approve_code", "Sign in another device with a code", ["quick sign-in", "code", "another device", "approve", "login", "cross-device"], Text, Write(Account), [];
    SignInWithBrowser => "account.sign_in_browser", "Add account with Roblox sign-in page", ["sign in", "login", "browser", "password", "new account"], None, Write(Account), [];
    RenameAccount => "account.rename", "Rename account (local label)", ["label", "rename", "account"], Text, Local, [];
    SetAccountColor => "account.color", "Set account color tag", ["color", "colour", "tag"], Color, Local, [];
    MoveAccountUp => "account.move_up", "Move account up in the rail", ["reorder", "order"], None, Local, [];
    MoveAccountDown => "account.move_down", "Move account down in the rail", ["reorder", "order"], None, Local, [];
    CopyAccountName => "account.copy_name", "Copy account username", ["copy", "username", "name"], None, Local, [];
    RemoveAccount => "account.remove", "Remove account from this app", ["delete", "forget", "remove"], Account, Write(Account), [];
    SignOut => "account.sign_out", "Sign out", ["logout", "log out", "sign out"], None, Write(Account), [];
    SignOutOthers => "account.sign_out_others", "Sign out of all other sessions", ["logout", "sessions", "devices", "security"], None, Write(Account), [];
    UnlockPin => "account.unlock_pin", "Unlock account PIN", ["pin", "unlock", "parental", "settings"], None, Write(Account), [];
    LockPin => "account.lock_pin", "Lock account PIN", ["pin", "lock", "parental"], None, Write(Account), [];
    ChangePassword => "security.change_password", "Change password", ["password", "security", "sign in"], None, Write(Account), [];
    AuthenticatorOn => "security.authenticator_on", "Turn on authenticator app codes", ["two-step", "2sv", "2fa", "totp", "authenticator", "security"], None, Write(Account), [];
    AuthenticatorOff => "security.authenticator_off", "Turn off authenticator app codes", ["two-step", "2sv", "2fa", "totp", "authenticator", "security"], None, Write(Account), [];
    EmailCodesOn => "security.email_codes_on", "Turn on email codes", ["two-step", "2sv", "2fa", "email", "security"], None, Write(Account), [];
    EmailCodesOff => "security.email_codes_off", "Turn off email codes", ["two-step", "2sv", "2fa", "email", "security"], None, Write(Account), [];
    RecoveryCodes => "security.recovery_codes", "Make new recovery codes", ["two-step", "2sv", "backup codes", "recovery", "security"], None, Write(Account), [];
    RemovePasskey => "security.remove_passkey", "Remove a passkey (name)", ["passkey", "security key", "webauthn", "security"], Text, Write(Account), [];
    SetDisplayName => "account.display_name", "Change display name", ["display name", "nickname", "name"], Text, Write(Social), [];
    SetPrivacy => "privacy.set", "Change a privacy setting (setting, then value)", ["privacy", "who can", "inventory", "trade", "chat", "join"], Text, Write(Account), [];
    ReplaceCookie => "account.replace_cookie", "Paste a new cookie for this account", ["expired", "session", "cookie"], None, Write(Account), [];

    GoHome => "nav.home", "Go to Home", ["home"], None, Read, [Shortcut::seq("G", "H")];
    GoDiscover => "nav.discover", "Go to Discover", ["discover", "charts", "sorts"], None, Read, [Shortcut::seq("G", "D")];
    GoAvatar => "nav.avatar", "Go to Avatar", ["avatar", "outfit", "wear"], None, Read, [Shortcut::seq("G", "A")];
    GoMarketplace => "nav.marketplace", "Go to Marketplace", ["catalog", "shop", "items"], None, Read, [Shortcut::seq("G", "M")];
    GoInventory => "nav.inventory", "Go to Inventory", ["inventory", "items"], None, Read, [Shortcut::seq("G", "I")];
    GoTrades => "nav.trades", "Go to Trades", ["trades", "trading"], None, Read, [Shortcut::seq("G", "T")];
    GoValues => "nav.values", "Go to Values (Rolimons)", ["values", "rolimons", "limiteds", "rap", "demand", "worth"], None, Read, [Shortcut::seq("G", "V")];
    GoTransactions => "nav.transactions", "Go to Transactions", ["transactions", "robux", "sales", "purchases", "spending", "income", "stipend", "ledger", "money"], None, Read, [Shortcut::seq("G", "R")];
    GoMessages => "nav.messages", "Go to Messages", ["messages", "inbox", "pm"], None, Read, [Shortcut::seq("G", "N")];
    GoCommunities => "nav.communities", "Go to Communities", ["groups", "communities"], None, Read, [Shortcut::seq("G", "C")];
    GoProfile => "nav.profile", "Go to your profile", ["profile", "me"], None, Read, [Shortcut::seq("G", "P")];
    GoFriends => "nav.friends", "Go to Friends", ["friends", "connections", "requests", "followers"], None, Read, [Shortcut::seq("G", "F")];
    OpenSettings => "nav.settings", "Settings", ["settings", "preferences", "options"], SettingsSection, Read, [Shortcut::ctrl("Comma")];
    OpenGame => "game.open", "Open experience", ["game", "experience", "place", "open"], Text, Read, [];
    OpenItem => "market.open", "Open item", ["item", "asset", "bundle", "pass", "open"], Text, Read, [];
    PreviewItem => "market.preview", "Preview item in a side panel", ["item", "asset", "bundle", "preview", "peek", "details", "inspect", "side"], Text, Read, [];
    OpenCommunity => "group.open", "Open community", ["group", "community", "open"], Text, Read, [];
    GoWiki => "nav.wiki", "Go to Wiki", ["wiki", "fandom", "article", "lore", "trivia", "guide", "history"], None, Read, [Shortcut::seq("G", "W")];
    WikiOpen => "wiki.open", "Open wiki article", ["wiki", "fandom", "article", "page"], Text, Read, [];
    WikiSearch => "wiki.search", "Search the wiki", ["wiki", "fandom", "search", "look up", "lookup"], Text, Read, [];
    WikiArticle => "wiki.article", "Read the wiki article about this", ["wiki", "fandom", "article", "about", "history", "trivia", "lore"], None, Read, [];
    WikiOpenSite => "wiki.open_site", "Open wiki article on Fandom", ["wiki", "fandom", "browser", "web", "site"], Text, Read, [];
    ToggleWiki => "wiki.toggle", "Turn wiki articles from Fandom on or off", ["wiki", "fandom", "article", "enable", "disable"], None, Local, [];
    GoComponents => "nav.components", "Go to Components (UI gallery)", ["components", "gallery", "ui", "design"], None, Read, [];
    GoOverview => "nav.overview", "Go to Overview (console clone test)", ["overview", "console", "clone", "test", "card"], None, Read, [];
    GoHub => "nav.hub", "Go to Theme and Plugin Hub", ["hub", "themes", "plugins", "browse", "community"], None, Read, [];
    HubRefresh => "hub.refresh", "Refresh hub", ["hub", "reload", "search"], None, Read, [];
    HubDetails => "hub.details", "View hub release details", ["hub", "release", "permissions"], Text, Read, [];
    HubPublish => "hub.publish", "Publish to hub", ["hub", "publish", "theme", "plugin", "upload"], Text, Local, [];
    HubPublishTheme => "hub.publish_theme", "Publish theme release", ["hub", "theme", "release"], Text, Hub, [];
    HubPublishPlugin => "hub.publish_plugin", "Submit plugin release for review", ["hub", "plugin", "review"], Text, Hub, [];
    HubInstall => "hub.install", "Review hub installation or update", ["hub", "install", "update", "download"], Text, Local, [];
    HubSignIn => "hub.sign_in", "Sign into Bonsai Hub with GitHub", ["hub", "account", "oauth", "publisher", "github"], None, Hub, [];
    HubSignOut => "hub.sign_out", "Sign out of Bonsai Hub", ["hub", "logout", "session"], None, Hub, [];
    HubSubmissions => "hub.submissions", "Refresh your hub submissions", ["hub", "pending", "status", "moderation"], None, Read, [];
    HubDelete => "hub.delete", "Delete your hub content", ["hub", "delete", "publisher"], Text, Hub, [];
    HubUnlist => "hub.unlist", "Unlist your hub content", ["hub", "unlist", "publisher"], Text, Hub, [];
    HubReport => "hub.report", "Report hub content", ["hub", "report", "moderation", "abuse"], Text, Hub, [];
    HubRevocations => "hub.revocations", "Check installed plugins for revocations", ["hub", "plugin", "revoked", "security"], None, Read, [];
    GoThemes => "nav.themes", "Go to Themes", ["themes", "appearance", "colors", "palette", "editor", "customize"], None, Read, [Shortcut::seq("G", "E")];

    ToggleDrawer => "view.drawer", "Toggle friends and chat drawer", ["friends", "chat", "drawer", "panel"], None, Local, [Shortcut::ctrl("Backslash")];
    ToggleSidebar => "view.sidebar", "Collapse or expand the sidebar", ["sidebar", "collapse"], None, Local, [Shortcut::ctrl("B")];
    ToggleSidebarScreen => "view.sidebar_toggle", "Show or hide in the sidebar", ["sidebar", "hide", "show", "remove", "customize", "screens"], Screen, Local, [];
    SidebarMoveUp => "view.sidebar_up", "Move up in the sidebar", ["sidebar", "reorder", "order", "customize"], Screen, Local, [];
    SidebarMoveDown => "view.sidebar_down", "Move down in the sidebar", ["sidebar", "reorder", "order", "customize"], Screen, Local, [];
    SetSidebarOrder => "view.sidebar_order", "Set sidebar order (screens, in order)", ["sidebar", "reorder", "order", "customize"], Text, Local, [];
    ResetSidebar => "view.sidebar_reset", "Reset sidebar order and hidden screens", ["sidebar", "reset", "default", "customize"], None, Local, [];
    SetMode => "view.theme", "Light, dark or follow the system", ["theme", "mode", "dark", "light", "system", "appearance"], Mode, Local, [];
    ApplyTheme => "theme.apply", "Use theme", ["theme", "color", "colour", "appearance", "skin", "palette"], Theme, Local, [];
    CycleTheme => "theme.cycle", "Next theme", ["theme", "cycle", "switch", "appearance"], None, Local, [];
    DeleteTheme => "theme.delete", "Delete theme", ["theme", "remove"], Theme, Local, [];
    DuplicateTheme => "theme.duplicate", "Duplicate theme", ["theme", "copy", "clone", "new"], Theme, Local, [];
    SetThemeFont => "theme.font", "Theme font", ["theme", "font", "typeface", "google", "text"], Font, Local, [];
    SetThemeMonoFont => "theme.mono_font", "Theme code font", ["theme", "font", "mono", "monospace", "code", "google"], Font, Local, [];
    NewTheme => "theme.new", "New theme", ["theme", "create", "make", "editor", "custom"], None, Local, [];
    EditTheme => "theme.edit", "Edit theme", ["theme", "editor", "customize", "colors", "change"], Theme, Local, [];
    SaveTheme => "theme.save", "Save theme", ["theme", "save", "editor"], Text, Local, [];
    RenameTheme => "theme.rename", "Rename theme", ["theme", "name", "rename"], Theme, Local, [];
    RevertTheme => "theme.revert", "Discard unsaved theme changes", ["theme", "discard", "revert", "reset", "editor"], None, Local, [];
    UndoTheme => "theme.undo", "Undo theme change", ["theme", "undo", "editor"], None, Local, [];
    RedoTheme => "theme.redo", "Redo theme change", ["theme", "redo", "editor"], None, Local, [];
    FixThemeContrast => "theme.fix_contrast", "Fix low contrast in the theme", ["theme", "contrast", "readable", "accessibility", "fix", "wcag"], None, Local, [];
    ImportTheme => "theme.import", "Import theme (paste a code or a file path)", ["theme", "import", "install", "add", "load", "file", "code", "paste", "share", "bonsaitheme"], Text, Local, [];
    ExportTheme => "theme.export", "Save theme to a file", ["theme", "export", "save", "file", "share", "bonsaitheme", "backup"], Theme, Local, [];
    CopyThemeCode => "theme.copy_code", "Copy theme code", ["theme", "share", "code", "copy", "export", "send"], Theme, Local, [];
    InstallPlugin => "plugin.install", "Install plugin (a .bonsaiplugin file or folder path)", ["plugin", "install", "add", "extension", "addon", "luau", "folder", "package", "bonsaiplugin"], Text, Local, [];
    LoadUnpackedPlugin => "plugin.load_unpacked", "Load unpacked plugin for development (a folder path)", ["plugin", "unpacked", "develop", "dev", "load", "folder", "luau"], Text, Local, [];
    EnablePlugin => "plugin.enable", "Turn on plugin", ["plugin", "enable", "on", "start", "extension"], Plugin, Local, [];
    DisablePlugin => "plugin.disable", "Turn off plugin", ["plugin", "disable", "off", "stop", "extension"], Plugin, Local, [];
    ReloadPlugin => "plugin.reload", "Reload plugin", ["plugin", "reload", "restart", "refresh", "develop"], Plugin, Local, [];
    ReviewPlugin => "plugin.review", "Review plugin permissions", ["plugin", "permissions", "review", "update", "agree"], Plugin, Local, [];
    PackPlugin => "plugin.pack", "Pack plugin into a .bonsaiplugin file", ["plugin", "pack", "package", "export", "share", "zip", "bonsaiplugin", "develop"], Plugin, Local, [];
    RemovePlugin => "plugin.remove", "Remove plugin", ["plugin", "remove", "uninstall", "delete"], Plugin, Local, [];
    GrantPlugin => "plugin.grant", "Let plugin use this account", ["plugin", "account", "allow", "grant", "permission"], Plugin, Local, [];
    RevokePlugin => "plugin.revoke", "Stop plugin using this account", ["plugin", "account", "revoke", "deny", "permission"], Plugin, Local, [];
    SetPluginSetting => "plugin.set", "Change plugin setting (plugin, setting, value; no value resets it)", ["plugin", "setting", "option", "config", "preference"], Text, Local, [];
    SetPluginCap => "plugin.cap", "Lower a plugin's hourly cap (plugin, action, number)", ["plugin", "cap", "limit", "rate", "hour", "automation"], Text, Local, [];
    SetPluginBudget => "plugin.budget", "Set a plugin's daily budget on this account", ["plugin", "budget", "robux", "spend", "limit", "day", "trade", "automation"], Plugin, Local, [];
    AllowAutomation => "plugins.allow_automation", "Allow plugin automation on this account", ["plugin", "automation", "automate", "unattended", "bot", "allow"], None, Local, [];
    BlockAutomation => "plugins.block_automation", "Turn off plugin automation on this account", ["plugin", "automation", "automate", "unattended", "bot", "block", "off"], None, Local, [];
    StopPlugins => "plugins.stop_all", "Stop all plugins", ["plugin", "stop", "kill", "panic", "disable", "halt", "automation"], None, Local, [Shortcut::ctrl_shift("Backspace")];
    PluginActivity => "plugins.activity", "Show plugin activity", ["plugin", "activity", "log", "history", "automation", "writes"], None, Read, [];
    SetBackground => "view.background", "Set background picture (HTTPS image link)", ["background", "wallpaper", "picture", "image", "photo", "gif", "url", "link", "appearance"], Text, Local, [];
    ClearBackground => "view.background_clear", "Remove background picture", ["background", "wallpaper", "picture", "remove", "clear", "appearance"], None, Local, [];
    SetBackgroundLevel => "view.background_level", "Background level (image, panels, blur, dim, saturation, tint, vignette, scanlines or grain, then a number)", ["background", "wallpaper", "opacity", "transparency", "see-through", "fade", "blur", "dim", "saturation", "tint", "vignette", "scanlines", "crt", "grain", "noise", "effect", "appearance"], Text, Local, [];
    ResetBackgroundEffects => "view.background_effects_reset", "Reset background effects", ["background", "wallpaper", "effects", "blur", "scanlines", "reset", "clear", "appearance"], None, Local, [];
    SetDensity => "view.density", "Density (compact or comfortable)", ["density", "compact", "comfortable", "dense", "spacing"], Text, Local, [];
    ZoomIn => "view.zoom_in", "Zoom in", ["scale", "bigger", "font"], None, Local, [Shortcut::ctrl("Plus"), Shortcut::ctrl("Equals")];
    ZoomOut => "view.zoom_out", "Zoom out", ["scale", "smaller", "font"], None, Local, [Shortcut::ctrl("Minus")];
    ZoomReset => "view.zoom_reset", "Reset zoom", ["scale"], None, Local, [Shortcut::ctrl("0")];
    MinimizeWindow => "window.minimize", "Minimize window", ["minimize", "hide"], None, Local, [];
    MaximizeWindow => "window.maximize", "Maximize or restore window", ["maximize", "restore", "fullscreen"], None, Local, [];

    NewTab => "tab.new", "New tab", ["tab", "open"], None, Local, [Shortcut::ctrl("T")];
    SwitchTab => "tab.switch", "Switch to tab", ["tab", "go", "find"], Tab, Local, [];
    CloseTab => "tab.close", "Close tab", ["close"], None, Local, [Shortcut::ctrl("W")];
    CloseOtherTabs => "tab.close_others", "Close other tabs", ["close", "tabs", "others"], None, Local, [];
    CloseTabsRight => "tab.close_right", "Close tabs to the right", ["close", "tabs", "right"], None, Local, [];
    PinTab => "tab.pin", "Pin or unpin tab", ["pin", "unpin", "tab"], None, Local, [];
    DuplicateTab => "tab.duplicate", "Duplicate tab", ["copy", "clone", "tab"], None, Local, [];
    NextTab => "tab.next", "Next tab", ["tab"], None, Local, [Shortcut::ctrl("PageDown")];
    PrevTab => "tab.prev", "Previous tab", ["tab"], None, Local, [Shortcut::ctrl("PageUp")];
    MoveTabRight => "tab.move_right", "Move tab right", ["reorder", "tab"], None, Local, [Shortcut::ctrl_shift("PageDown")];
    MoveTabLeft => "tab.move_left", "Move tab left", ["reorder", "tab"], None, Local, [Shortcut::ctrl_shift("PageUp")];
    GoTab1 => "tab.go_1", "Go to tab 1", ["tab"], None, Local, [Shortcut::alt("1")];
    GoTab2 => "tab.go_2", "Go to tab 2", ["tab"], None, Local, [Shortcut::alt("2")];
    GoTab3 => "tab.go_3", "Go to tab 3", ["tab"], None, Local, [Shortcut::alt("3")];
    GoTab4 => "tab.go_4", "Go to tab 4", ["tab"], None, Local, [Shortcut::alt("4")];
    GoTab5 => "tab.go_5", "Go to tab 5", ["tab"], None, Local, [Shortcut::alt("5")];
    GoTab6 => "tab.go_6", "Go to tab 6", ["tab"], None, Local, [Shortcut::alt("6")];
    GoTab7 => "tab.go_7", "Go to tab 7", ["tab"], None, Local, [Shortcut::alt("7")];
    GoTab8 => "tab.go_8", "Go to tab 8", ["tab"], None, Local, [Shortcut::alt("8")];
    GoTabLast => "tab.go_last", "Go to last tab", ["tab", "end"], None, Local, [Shortcut::alt("9")];
    ReopenTab => "tab.reopen", "Reopen closed tab", ["undo", "tab"], None, Local, [Shortcut::ctrl_shift("T")];

    Join => "game.join", "Join", ["join", "play", "launch"], GameOrFriend, Write(Harmless), [];
    JoinAs => "game.join_as", "Join as another account", ["join", "alt", "as"], GameOrFriend, Write(Harmless), [];
    JoinServer => "game.join_server", "Join selected server", ["server", "join"], None, Write(Harmless), [];
    JoinBestServer => "game.join_best", "Join best server for me", ["best", "ping", "server"], None, Write(Harmless), [];
    JoinSmallestServer => "game.join_smallest", "Join smallest server", ["small", "empty", "server"], None, Write(Harmless), [];
    JoinRandomServer => "game.join_random", "Join random server", ["random", "server"], None, Write(Harmless), [];
    JoinJobId => "game.join_job", "Join server by job id", ["job", "instance", "server id"], Text, Write(Harmless), [];
    CopyServerLink => "game.copy_server_link", "Copy server invite link", ["invite", "share", "server"], None, Read, [];
    FindUser => "user.find", "Find which server a user is in", ["find", "where", "server"], User, Read, [];
    OpenProfile => "user.open", "Open profile", ["profile", "user", "player"], User, Read, [];
    ShowFriendsOf => "user.friends", "Show a user's friends", ["friends", "connections", "mutuals", "mutual friends"], User, Read, [];
    ShowFollowersOf => "user.followers", "Show a user's followers", ["followers", "fans"], User, Read, [];
    ShowFollowingOf => "user.following", "Show who a user follows", ["following", "followings", "follows"], User, Read, [];
    AddFriend => "user.add_friend", "Add friend", ["friend", "connect", "request"], User, Write(Social), [];
    AddFriendAs => "user.add_friend_as", "Add friend as another account", ["friend", "connect", "request", "alt", "as"], User, Write(Social), [];
    Unfriend => "user.unfriend", "Remove friend", ["unfriend", "remove", "connection"], User, Write(Social), [];
    AcceptFriend => "user.accept_friend", "Accept friend request", ["accept", "request"], User, Write(Social), [];
    DeclineFriend => "user.decline_friend", "Decline friend request", ["decline", "ignore", "request"], User, Write(Social), [];
    DeclineAllFriends => "friends.decline_all", "Ignore all friend requests", ["decline", "ignore", "requests"], None, Write(Social), [];
    Follow => "user.follow", "Follow", ["follow"], User, Write(Social), [];
    Unfollow => "user.unfollow", "Unfollow", ["unfollow"], User, Write(Social), [];
    SetNickname => "user.nickname", "Set a nickname for someone (user, then nickname; empty clears)", ["nickname", "alias", "name", "friend"], Text, Write(Harmless), [];
    SetNote => "user.note", "Write a private note about someone (user, then note; empty clears)", ["note", "notes", "memo", "friend"], Text, Write(Harmless), [];
    Block => "user.block", "Block user", ["block"], User, Write(Social), [];
    Unblock => "user.unblock", "Unblock user", ["unblock"], User, Write(Social), [];
    LikeGame => "game.like", "Like this experience", ["like", "thumbs up", "vote", "upvote"], None, Write(Harmless), [];
    DislikeGame => "game.dislike", "Dislike this experience", ["dislike", "thumbs down", "vote", "downvote"], None, Write(Harmless), [];
    FavoriteGame => "game.favorite", "Favorite or unfavorite this experience", ["favorite", "favourite", "star"], None, Write(Harmless), [];
    NotifyGame => "game.notify", "Turn update notifications on or off", ["notify", "follow", "bell", "updates"], None, Write(Harmless), [];
    NotifyEvent => "game.event_notify", "Notify me about an event", ["event", "rsvp", "notify", "remind"], Text, Write(Harmless), [];
    BuyPass => "game.buy_pass", "Buy a pass", ["pass", "gamepass", "store", "buy", "purchase"], Text, Write(Value), [];
    BuyPrivateServer => "game.buy_private_server", "Buy a private server", ["private server", "vip", "buy", "create"], Text, Write(Value), [];
    JoinPrivateServer => "game.join_private_server", "Join a private server", ["private server", "vip", "join"], Text, Write(Harmless), [];
    RenamePrivateServer => "server.rename", "Rename private server (id, then name)", ["private server", "vip", "name"], Text, Write(Value), [];
    ToggleServerActive => "server.toggle_active", "Turn private server on or off", ["private server", "vip", "active", "disable"], Text, Write(Value), [];
    NewServerLink => "server.new_link", "Make a new private server link", ["private server", "vip", "link", "invite", "regenerate"], Text, Write(Value), [];
    ToggleServerFriends => "server.toggle_friends", "Let friends join private server", ["private server", "vip", "friends", "allow"], Text, Write(Value), [];
    AddServerUser => "server.add_user", "Let someone into a private server (id, then user)", ["private server", "vip", "whitelist", "invite", "allow"], Text, Write(Value), [];
    RemoveServerUser => "server.remove_user", "Take someone off a private server (id, then user)", ["private server", "vip", "whitelist", "remove"], Text, Write(Value), [];
    ToggleServerRenew => "server.toggle_renew", "Turn private server renewal on or off", ["private server", "vip", "subscription", "renew", "cancel"], Text, Write(Value), [];
    LoadMore => "list.more", "Load more", ["more", "next page", "load"], Text, Read, [];
    RefreshServers => "game.refresh_servers", "Refresh server list", ["reload", "servers"], None, Read, [Shortcut::key("R")];

    WearItem => "avatar.wear", "Wear or take off an item", ["wear", "equip", "remove", "take off", "avatar"], Text, Write(Harmless), [];
    WearOutfit => "avatar.outfit", "Wear an outfit", ["outfit", "wear", "costume", "avatar"], Outfit, Write(Harmless), [];
    SaveOutfit => "avatar.save_outfit", "Save current look as an outfit", ["outfit", "save", "create", "avatar"], Text, Write(Harmless), [];
    SetRig => "avatar.rig", "Switch body type (R6 or R15)", ["r6", "r15", "rig", "body", "avatar"], Text, Write(Harmless), [];
    UpdateOutfit => "avatar.update_outfit", "Update outfit with current look", ["outfit", "overwrite", "replace", "save", "avatar"], Outfit, Write(Harmless), [];
    RenameOutfit => "avatar.rename_outfit", "Rename outfit (id, then name)", ["outfit", "rename", "name", "avatar"], Text, Write(Harmless), [];
    DeleteOutfit => "avatar.delete_outfit", "Delete outfit", ["outfit", "delete", "remove", "avatar"], Outfit, Write(Harmless), [];
    MoveLayer => "avatar.layer", "Move layered clothing up or down (item id, then up or down)", ["layer", "order", "layered clothing", "on top", "under", "avatar"], Text, Write(Harmless), [];
    SetBodyColor => "avatar.color", "Set body color (color id, or part:color id)", ["skin", "color", "colour", "tone", "body", "avatar"], Text, Write(Harmless), [];
    SetScale => "avatar.scale", "Set body scale (scale, then value)", ["height", "width", "head", "proportion", "body type", "scale", "size", "avatar"], Text, Write(Harmless), [];
    SetEmote => "avatar.emote", "Put an emote in a hotbar slot (slot 1-8, then item id; slot alone clears it)", ["emote", "hotbar", "slot", "dance", "avatar"], Text, Write(Harmless), [];
    RedrawAvatar => "avatar.redraw", "Redraw avatar image", ["redraw", "refresh", "thumbnail", "render", "avatar"], None, Write(Harmless), [];
    Toggle3d => "avatar.view_3d", "Show the avatar in 3D or as an image", ["3d", "2d", "model", "render", "preview", "spin", "avatar"], None, Local, [];
    SetAvatarView => "avatar.default_view", "Open avatars in 3D or as an image (3d or 2d)", ["3d", "2d", "default", "model", "image", "setting", "avatar"], Text, Local, [];
    PlayAnimation => "avatar.play", "Play an animation in the 3D view (idle, walk, run, jump, fall, climb, swim, or an emote id)", ["animation", "emote", "dance", "play", "pose", "3d", "avatar"], Text, Read, [];
    Orbit3d => "avatar.orbit", "Move the 3D camera (left, right, up, down, in, out or reset)", ["camera", "rotate", "orbit", "zoom", "turn", "reset", "3d", "avatar"], Text, Read, [];

    SearchMarketplace => "market.search", "Search the Marketplace", ["catalog", "shop", "items", "find item"], Text, Read, [];
    BuyItem => "market.buy", "Buy this item", ["buy", "purchase", "get"], None, Write(Value), [];
    BuyItemAs => "market.buy_as", "Buy this item as another account", ["buy", "purchase", "alt", "as"], Account, Write(Value), [];
    BuyResale => "market.buy_resale", "Buy a reseller's copy", ["buy", "resale", "reseller", "limited", "copy"], Text, Write(Value), [];
    FavoriteItem => "market.favorite", "Favorite or unfavorite this item", ["favorite", "favourite", "star"], None, Write(Harmless), [];
    WishlistItem => "market.wishlist", "Add to or remove from wishlist", ["wishlist", "want", "save", "watch"], None, Write(Harmless), [];
    DownloadTemplate => "market.template", "Download clothing template", ["template", "download", "save", "shirt", "pants", "t-shirt", "clothing", "image"], Text, Local, [];
    DownloadMaskedTemplate => "market.template_masked", "Download clothing template, masked", ["template", "masked", "mask", "download", "save", "shirt", "pants", "clothing", "clean"], Text, Local, [];
    ToggleItemValues => "market.values", "Turn item values from Rolimons on or off", ["values", "rolimons", "worth", "rap"], None, Local, [];
    PriceHistoryRange => "market.history_range", "Set the price history range (7d, 30d, 90d, 1y, all; no argument cycles)", ["price history", "chart", "range", "rap", "sales", "limited"], Text, Local, [];
    PriceHistoryZoom => "market.history_zoom", "Zoom the price history chart (in, out, reset)", ["price history", "chart", "zoom", "pan", "limited"], Text, Local, [];
    TryOn => "market.try_on", "Try on this item in 3D", ["try on", "preview", "wear", "fit", "3d", "avatar"], None, Read, [];
    PlayAudio => "market.play_audio", "Play or pause this sound", ["play", "pause", "audio", "sound", "music", "listen"], None, Read, [];
    PriceHistoryScale => "market.history_scale", "Show the price history on a log or linear scale (log, linear; no argument toggles)", ["price history", "chart", "log", "scale", "limited"], Text, Local, [];
    PriceHistoryView => "market.history_view", "Show price history as a chart or a table (chart, table; no argument toggles)", ["price history", "chart", "table", "sales", "limited"], Text, Local, [];
    SearchAuditLog => "group.audit_search", "Search the community audit log", ["audit", "log", "search", "performer", "community", "group"], Text, Read, [];
    ViewInventory => "inventory.view", "Open a user's inventory", ["inventory", "items", "collectibles", "limiteds"], User, Read, [];
    DeleteInventoryItem => "inventory.delete", "Delete an item from your inventory", ["delete", "remove", "inventory"], Text, Write(Value), [];
    SortInventory => "inventory.sort", "Sort the inventory (newest, oldest, price_asc, price_desc, rap, value, demand)", ["inventory", "sort", "order", "price", "rap", "oldest", "newest"], Text, Local, [];
    FilterInventory => "inventory.filter", "Filter the inventory (all, limited, wearing, copies, for_sale)", ["inventory", "filter", "show only", "limited", "wearing", "duplicates"], Text, Local, [];
    InspectInventoryItem => "inventory.inspect", "Inspect an inventory item", ["inventory", "inspect", "details", "drawer", "info"], Text, Read, [];
    TradeWith => "trade.new", "Send a trade to", ["trade", "offer", "new trade"], User, Read, [];
    AcceptTrade => "trade.accept", "Accept trade", ["accept", "trade"], Text, Write(Value), [];
    DeclineTrade => "trade.decline", "Decline trade", ["decline", "trade", "cancel"], Text, Write(Social), [];
    CounterTrade => "trade.counter", "Counter trade", ["counter", "trade"], Text, Write(Value), [];
    SendTrade => "trade.send", "Send the trade you're making", ["send", "trade", "offer"], None, Write(Value), [];
    CopyTradesCsv => "trade.export", "Copy the trade list as CSV", ["export", "csv", "history", "copy"], None, Read, [];
    TxPeriod => "transactions.period", "Set the transactions period (day, week, month, year, all; no argument cycles)", ["transactions", "period", "range", "month", "year", "all time"], Text, Local, [];
    TxType => "transactions.type", "Show one transaction type (sales, purchases, stipends, ...; empty shows all)", ["transactions", "type", "filter", "sales", "purchases", "stipends", "payouts"], Text, Local, [];
    TxScope => "transactions.scope", "Show your transactions or a community's (me, or a community name or ID)", ["transactions", "community", "group", "revenue", "funds"], Text, Local, [];
    TxRange => "transactions.range", "Show transactions between two dates (2026-01-01..2026-03-31; empty clears)", ["transactions", "dates", "from", "to", "between"], Text, Local, [];
    TxPending => "transactions.pending", "Show only pending Robux, or everything (no argument toggles)", ["transactions", "pending", "held", "escrow"], Text, Local, [];
    TxSync => "transactions.sync", "Sync transactions now", ["transactions", "refresh", "reload", "sync"], None, Read, [];
    TxLoadOlder => "transactions.load_older", "Fetch older transaction history", ["transactions", "history", "older", "backfill", "all"], None, Read, [];
    TxStop => "transactions.stop", "Stop fetching older transactions", ["transactions", "stop", "cancel"], None, Read, [];
    TxExport => "transactions.export", "Save the shown transactions as CSV", ["transactions", "export", "csv", "save", "download"], None, Local, [];
    TxCopyCsv => "transactions.copy_csv", "Copy the shown transactions as CSV", ["transactions", "export", "csv", "copy"], None, Read, [];

    MessageUser => "user.message", "Message (chat)", ["chat", "message", "dm", "pm", "whisper"], User, Read, [];
    NewGroupChat => "chat.new_group", "Start a chat or group chat", ["new chat", "group chat", "compose", "new message"], None, Read, [];
    SendChat => "chat.send", "Send chat message", ["send", "say"], Text, Write(Social), [];
    ResendChat => "chat.resend", "Send a failed chat message again", ["resend", "retry", "failed", "chat"], Text, Write(Social), [];
    DiscardChat => "chat.discard", "Discard a failed chat message", ["discard", "delete", "failed", "chat"], Text, Write(Harmless), [];
    ChatMembers => "chat.members", "View chat members", ["members", "participants", "people", "group"], None, Read, [];
    RenameChat => "chat.rename", "Rename this group chat", ["rename", "title", "chat"], Text, Write(Social), [];
    AddToChat => "chat.add_people", "Add people to this chat", ["add", "invite", "chat"], User, Write(Social), [];
    RemoveFromChat => "chat.remove_person", "Remove someone from this chat", ["remove", "kick", "chat"], User, Write(Social), [];
    LeaveChat => "chat.leave", "Leave this group chat", ["leave", "exit", "chat"], None, Write(Social), [];
    PinChat => "chat.pin", "Pin or unpin this chat", ["pin", "unpin", "chat"], None, Write(Harmless), [];
    ShareExperience => "chat.share_experience", "Share an experience in this chat (link or ID)", ["share", "experience", "game", "link", "chat"], Text, Write(Social), [];
    ChatEmoji => "chat.emoji", "Insert an emoji", ["emoji", "emoticon", "smiley", "chat"], None, Read, [Shortcut::ctrl("Period")];
    ReplyMessage => "message.reply", "Reply to the message (in chat)", ["reply", "respond", "message"], Text, Write(Social), [Shortcut::shift("R")];
    ToggleMessageRead => "message.toggle_read", "Mark message read or unread", ["read", "unread", "message"], Text, Write(Harmless), [Shortcut::key("U")];
    ArchiveMessage => "message.archive", "Archive message, or move it back to Inbox", ["archive", "unarchive", "message"], Text, Write(Harmless), [Shortcut::key("E")];
    SelectAllMessages => "message.select_all", "Select all loaded messages, or clear the selection", ["select all", "bulk", "messages"], None, Read, [];
    MarkAllMessagesRead => "message.mark_all_read", "Mark all messages read", ["read all", "messages", "inbox"], None, Write(Harmless), [];
    FilterNotifications => "notifications.filter", "Show all or only unread notifications (all, unread; no argument toggles)", ["notifications", "unread", "filter", "bell"], Text, Local, [];
    FilterNotificationCategory => "notifications.category", "Show one notification category (friends, trades, communities, experiences, account, messages, other; no argument shows all)", ["notifications", "category", "filter", "friends", "trades", "security", "robux", "group", "experience"], Text, Local, [];
    ClearNotifications => "notifications.mark_read", "Mark notifications read", ["notifications", "bell", "clear", "read"], None, Write(Harmless), [];
    GoChat => "nav.chat", "Go to Chat", ["chat", "conversations"], None, Read, [];
    GoNotifications => "nav.notifications", "Go to Notifications", ["notifications", "bell", "alerts"], None, Read, [];

    SearchCommunities => "group.search", "Search communities", ["groups", "communities", "find group"], Text, Read, [];
    JoinCommunity => "group.join", "Join this community", ["join", "group", "community"], Text, Write(Social), [];
    LeaveCommunity => "group.leave", "Leave a community", ["leave", "group", "community"], Text, Write(Social), [];
    SetPrimaryCommunity => "group.primary", "Make or clear primary community", ["primary", "group", "community", "main"], Text, Write(Social), [];
    PostShout => "group.shout", "Post a shout", ["shout", "status", "announce", "group"], Text, Write(Social), [];
    AcceptJoinRequest => "group.accept_request", "Accept join request", ["accept", "join request", "group"], User, Write(Social), [];
    DeclineJoinRequest => "group.decline_request", "Decline join request", ["decline", "join request", "group"], User, Write(Social), [];
    AcceptSelectedRequests => "group.accept_selected", "Accept selected join requests", ["accept", "bulk", "join requests"], None, Write(Social), [];
    DeclineSelectedRequests => "group.decline_selected", "Decline selected join requests", ["decline", "bulk", "join requests"], None, Write(Social), [];
    ChangeRank => "group.change_rank", "Change a member's rank", ["rank", "role", "promote", "demote", "group"], Text, Write(Social), [];
    KickMember => "group.kick", "Remove a member from the community", ["kick", "exile", "remove", "group"], User, Write(Social), [];
    NewForumPost => "group.new_post", "New forum post", ["forum", "post", "wall", "group"], Text, Write(Social), [];
    ReplyForumPost => "group.reply", "Reply to the open forum post", ["reply", "forum", "comment"], Text, Write(Social), [];
    DeleteForumPost => "group.delete_post", "Delete a forum post", ["delete", "forum", "post"], Text, Write(Social), [];
    DeleteForumReply => "group.delete_reply", "Delete a forum reply", ["delete", "forum", "reply", "comment"], Text, Write(Social), [];
    LeaveSelectedCommunities => "group.leave_selected", "Leave the selected communities", ["leave", "cleanup", "bulk", "groups"], None, Write(Social), [];
    CopyMembersCsv => "group.export_members", "Copy loaded members as CSV", ["export", "csv", "members", "group"], None, Read, [];
    ToggleCommunitySetting => "group.setting", "Turn a community setting on or off", ["settings", "approval", "group", "community"], Text, Write(Social), [];
    PayOut => "group.payout", "Pay out Robux to a member (user, then amount)", ["payout", "pay", "robux", "funds", "revenue", "group"], Text, Write(Value), [];
    SetRecurringPayouts => "group.recurring_payouts", "Set recurring payouts (user:percent, …)", ["payout", "recurring", "split", "percentage", "revenue", "funds", "group"], Text, Write(Value), [];
    SendAllyRequest => "group.ally", "Send an ally request to a community", ["ally", "allies", "affiliate", "group", "community"], Text, Write(Social), [];
    RemoveAlly => "group.remove_ally", "Remove an ally", ["ally", "allies", "affiliate", "remove", "group"], Text, Write(Social), [];
    DeclareEnemy => "group.enemy", "Declare a community an enemy", ["enemy", "enemies", "affiliate", "group", "community"], Text, Write(Social), [];
    RemoveEnemy => "group.remove_enemy", "Remove an enemy", ["enemy", "enemies", "affiliate", "remove", "group"], Text, Write(Social), [];

    SearchRoblox => "search.open", "Search Roblox (results page)", ["search", "find", "results", "people", "experiences", "groups", "marketplace"], Text, Read, [];
    ClearSearchHistory => "search.clear_history", "Clear recent searches", ["search", "history", "recent", "clear"], None, Local, [];

    CopyId => "entity.copy_id", "Copy ID", ["copy", "id"], None, Read, [];
    CopyLink => "entity.copy_link", "Copy link", ["copy", "link", "url"], None, Read, [];
    OpenOnWebsite => "entity.open_website", "Open on website", ["browser", "web", "site"], None, Read, [Shortcut::ctrl_shift("O")];
    Report => "entity.report", "Report (on the website)", ["report", "abuse"], None, Read, [];
    Refresh => "view.refresh", "Refresh", ["reload"], None, Read, [Shortcut::key("F5")];
    SetPlayerCommand => "app.player_command", "Set how Roblox is launched (empty for the default)", ["launch", "player", "sober", "flatpak", "wine", "command"], Text, Local, [];
    SetFormatLocale => "app.format_locale", "Set number and date format (e.g. de-DE; empty for the system's)", ["language", "locale", "region", "format", "date", "number"], Text, Local, [];
    SetNotify => "app.notify", "Turn a notification kind on or off (friend_joins, trades, chats, friend_requests, experience_updates, sales, plugins)", ["notifications", "alerts", "desktop", "mute", "sales"], Text, Local, [];
    SetSetting => "settings.set", "Change a setting", ["setting", "preference", "option", "change", "turn on", "turn off"], Setting, Local, [];
    ResetSettings => "settings.reset", "Reset settings to their defaults", ["reset", "default", "restore", "settings"], Setting, Local, [];
    SetShortcut => "shortcuts.set", "Change a command's shortcut", ["shortcut", "keybinding", "hotkey", "rebind", "keys"], Text, Local, [];
    ResetShortcuts => "shortcuts.reset", "Reset all shortcuts", ["shortcut", "keybinding", "hotkey", "default", "reset"], None, Local, [];
    ExportSettings => "settings.export", "Save settings to a file", ["export", "backup", "save", "settings", "file", "move"], None, Local, [];
    ImportSettings => "settings.import", "Load settings from a file", ["import", "restore", "load", "settings", "file", "backup"], None, Local, [];
    CopyDiagnostics => "app.copy_diagnostics", "Copy diagnostics for a bug report", ["diagnostics", "debug", "bug", "report", "version", "about", "support"], None, Read, [];
    ClearCache => "app.clear_cache", "Clear cached data and images", ["cache", "clear", "disk", "storage"], None, Local, [];

    NextSubTab => "view.next_subtab", "Next sub-tab", ["tab", "next"], None, Read, [Shortcut::key("CloseBracket")];
    PrevSubTab => "view.prev_subtab", "Previous sub-tab", ["tab", "previous"], None, Read, [Shortcut::key("OpenBracket")];
    ListDown => "list.down", "Move down", ["next"], None, Read, [Shortcut::key("J")];
    ListUp => "list.up", "Move up", ["previous"], None, Read, [Shortcut::key("K")];
    ListOpen => "list.open", "Open selected", ["open"], None, Read, [Shortcut::key("Enter")];
    ListSelect => "list.select", "Select for bulk action", ["select", "mark"], None, Read, [Shortcut::key("X")];

    Quit => "app.quit", "Quit", ["exit", "close"], None, Local, [Shortcut::ctrl("Q")];
}

/// A registered command.
#[derive(Clone, Debug)]
pub struct Command {
    pub id: CommandId,
    pub title: &'static str,
    pub keywords: &'static [&'static str],
    pub arg: ArgKind,
    pub effect: Effect,
    pub default_shortcuts: Vec<Shortcut>,
}

/// A request to run a command, with an optional argument and acting account.
#[derive(Clone, Debug, PartialEq)]
pub struct Invocation {
    pub id: CommandId,
    pub arg: Option<String>,
    /// "Act as": run as this account instead of the active one.
    pub as_account: Option<bonsai_api::AccountId>,
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

impl CommandId {
    pub fn from_key(key: &str) -> Option<CommandId> {
        CommandId::ALL.iter().copied().find(|c| c.key() == key)
    }
}

/// A command a plugin added: `plugin.<plugin>.<id>` in the registry.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PluginCmd {
    pub plugin: String,
    pub id: String,
}

impl PluginCmd {
    pub fn key(&self) -> String {
        format!("plugin.{}.{}", self.plugin, self.id)
    }
}

pub use bonsai_plugin::is_plugin_id;

/// A built-in command or one a plugin added. Both cross IPC as their key.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CommandRef {
    Builtin(CommandId),
    Plugin(PluginCmd),
}

impl CommandRef {
    pub fn key(&self) -> String {
        match self {
            CommandRef::Builtin(id) => id.key().to_owned(),
            CommandRef::Plugin(p) => p.key(),
        }
    }

    /// Reads a key back. Built-in keys win, so `plugin.install` stays a built-in command.
    pub fn parse(key: &str) -> Option<CommandRef> {
        if let Some(id) = CommandId::from_key(key) {
            return Some(CommandRef::Builtin(id));
        }
        let (plugin, id) = key.strip_prefix("plugin.")?.split_once('.')?;
        (is_plugin_id(plugin) && is_plugin_id(id)).then(|| {
            CommandRef::Plugin(PluginCmd {
                plugin: plugin.to_owned(),
                id: id.to_owned(),
            })
        })
    }
}

/// A command a plugin registered with `bonsai.command`. No default shortcuts.
#[derive(Clone, Debug)]
pub struct PluginCommand {
    pub cmd: PluginCmd,
    pub title: String,
    pub keywords: Vec<String>,
    pub arg: ArgKind,
}

/// All commands plus the user's rebindings.
pub struct Registry {
    commands: Vec<Command>,
    plugins: Vec<PluginCommand>,
    overrides: HashMap<CommandId, Vec<Shortcut>>,
    /// Shortcuts the user gave plugin commands, by key. They have none by default. Kept while
    /// the plugin is off, so they come back with it.
    plugin_shortcuts: BTreeMap<String, Vec<Shortcut>>,
}

impl Default for Registry {
    fn default() -> Self {
        Registry {
            commands: build(),
            plugins: Vec::new(),
            overrides: HashMap::new(),
            plugin_shortcuts: BTreeMap::new(),
        }
    }
}

impl Registry {
    pub fn all(&self) -> &[Command] {
        &self.commands
    }

    /// Commands from enabled plugins.
    pub fn plugin_commands(&self) -> &[PluginCommand] {
        &self.plugins
    }

    /// Replaces one plugin's commands, e.g. after it loads, reloads or is disabled (empty).
    pub fn set_plugin_commands(&mut self, plugin: &str, commands: Vec<PluginCommand>) {
        self.plugins.retain(|c| c.cmd.plugin != plugin);
        self.plugins
            .extend(commands.into_iter().filter(|c| c.cmd.plugin == plugin));
    }

    /// Every command, built-in first.
    pub fn refs(&self) -> impl Iterator<Item = CommandRef> + '_ {
        let builtin = self.commands.iter().map(|c| CommandRef::Builtin(c.id));
        builtin.chain(self.plugins.iter().map(|c| CommandRef::Plugin(c.cmd.clone())))
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

    /// True when the user changed this command's shortcuts.
    pub fn is_rebound(&self, id: CommandId) -> bool {
        self.overrides.contains_key(&id)
    }

    pub fn plugin_shortcuts(&self) -> &BTreeMap<String, Vec<Shortcut>> {
        &self.plugin_shortcuts
    }

    pub fn load_plugin_shortcuts(&mut self, s: BTreeMap<String, Vec<Shortcut>>) {
        self.plugin_shortcuts = s;
    }

    /// The shortcuts of a built-in or plugin command.
    pub fn shortcuts_of(&self, c: &CommandRef) -> &[Shortcut] {
        match c {
            CommandRef::Builtin(id) => self.shortcuts(*id),
            CommandRef::Plugin(p) => self
                .plugin_shortcuts
                .get(&p.key())
                .map(Vec::as_slice)
                .unwrap_or_default(),
        }
    }

    /// True when the user changed the command's shortcuts. A plugin command has none until
    /// they give it one.
    pub fn is_rebound_ref(&self, c: &CommandRef) -> bool {
        match c {
            CommandRef::Builtin(id) => self.is_rebound(*id),
            CommandRef::Plugin(p) => self.plugin_shortcuts.contains_key(&p.key()),
        }
    }

    /// Gives a command this one shortcut, or none, and takes it off any other command it
    /// clashes with (see [`Shortcut::clashes`]), built-in or plugin. Returns the commands it was
    /// taken from.
    pub fn assign(&mut self, target: &CommandRef, shortcut: Option<Shortcut>) -> Vec<CommandRef> {
        let mut taken = Vec::new();
        if let Some(s) = &shortcut {
            for other in CommandId::ALL.iter().copied() {
                if *target == CommandRef::Builtin(other) {
                    continue;
                }
                let had = self.shortcuts(other);
                if had.iter().any(|x| x.clashes(s)) {
                    let kept = had.iter().filter(|x| !x.clashes(s)).cloned().collect();
                    self.overrides.insert(other, kept);
                    taken.push(CommandRef::Builtin(other));
                }
            }
            let key = target.key();
            for (other, had) in self.plugin_shortcuts.iter_mut() {
                if *other != key && had.iter().any(|x| x.clashes(s)) {
                    had.retain(|x| !x.clashes(s));
                    taken.extend(CommandRef::parse(other));
                }
            }
        }
        match target {
            CommandRef::Builtin(id) => {
                self.overrides.insert(*id, shortcut.into_iter().collect());
            }
            CommandRef::Plugin(p) => {
                self.plugin_shortcuts
                    .insert(p.key(), shortcut.into_iter().collect());
            }
        }
        self.tidy();
        taken
    }

    /// Puts back the default shortcuts of a command, or of every command. Plugin commands go
    /// back to none.
    pub fn reset(&mut self, target: Option<&CommandRef>) {
        match target {
            Some(CommandRef::Builtin(id)) => {
                self.overrides.remove(id);
            }
            Some(CommandRef::Plugin(p)) => {
                self.plugin_shortcuts.remove(&p.key());
            }
            None => {
                self.overrides.clear();
                self.plugin_shortcuts.clear();
            }
        }
    }

    /// Drops rebindings that match the defaults again.
    fn tidy(&mut self) {
        self.plugin_shortcuts.retain(|_, list| !list.is_empty());
        let commands = &self.commands;
        self.overrides.retain(|id, list| {
            commands
                .iter()
                .find(|c| c.id == *id)
                .is_none_or(|c| c.default_shortcuts != *list)
        });
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
    fn account_control_is_tier_4() {
        for c in Registry::default().all() {
            let key = c.id.key();
            let control = key.starts_with("security.")
                || key.starts_with("privacy.")
                || matches!(
                    c.id,
                    CommandId::AddAccount
                        | CommandId::QuickSignIn
                        | CommandId::ApproveCode
                        | CommandId::SignInWithBrowser
                        | CommandId::RemoveAccount
                        | CommandId::SignOut
                        | CommandId::SignOutOthers
                        | CommandId::UnlockPin
                        | CommandId::LockPin
                        | CommandId::ReplaceCookie
                );
            assert_eq!(control, c.effect == Effect::Write(Tier::Account), "{key}");
        }
    }

    #[test]
    fn value_and_social_writes_have_their_tier() {
        let r = Registry::default();
        let tier = |id| r.get(id).effect.tier();
        for id in [
            CommandId::BuyItem,
            CommandId::BuyResale,
            CommandId::SendTrade,
            CommandId::AcceptTrade,
            CommandId::CounterTrade,
            CommandId::PayOut,
            CommandId::DeleteInventoryItem,
            CommandId::BuyPrivateServer,
        ] {
            assert_eq!(tier(id), Some(Tier::Value), "{}", id.key());
        }
        for id in [
            CommandId::AddFriend,
            CommandId::Unfriend,
            CommandId::Block,
            CommandId::SendChat,
            CommandId::PostShout,
        ] {
            assert_eq!(tier(id), Some(Tier::Social), "{}", id.key());
        }
        // Launching another program or reading files is never a plugin action.
        for id in [
            CommandId::SetPlayerCommand,
            CommandId::ImportSettings,
            CommandId::ImportTheme,
            CommandId::SetBackground,
        ] {
            assert_eq!(r.get(id).effect, Effect::Local, "{}", id.key());
        }
        // Navigation never changes anything.
        for c in r.all() {
            if c.id.key().starts_with("nav.") {
                assert_eq!(c.effect, Effect::Read, "{}", c.id.key());
            }
        }
    }

    #[test]
    fn command_refs_round_trip() {
        for id in CommandId::ALL {
            assert_eq!(CommandRef::parse(id.key()), Some(CommandRef::Builtin(*id)));
        }
        let p = CommandRef::parse("plugin.friend-cleanup.unfriend-inactive").unwrap();
        assert_eq!(
            p,
            CommandRef::Plugin(PluginCmd {
                plugin: "friend-cleanup".into(),
                id: "unfriend-inactive".into(),
            })
        );
        assert_eq!(p.key(), "plugin.friend-cleanup.unfriend-inactive");
        for bad in [
            "plugin.x",
            "plugin.X.y",
            "plugin.a.b.c",
            "plugin..y",
            "plugin.a.",
            "nope",
        ] {
            assert_eq!(CommandRef::parse(bad), None, "{bad}");
        }
    }

    #[test]
    fn plugin_commands_replace_per_plugin() {
        let cmd = |plugin: &str, id: &str| PluginCommand {
            cmd: PluginCmd {
                plugin: plugin.into(),
                id: id.into(),
            },
            title: id.into(),
            keywords: vec![],
            arg: ArgKind::None,
        };
        let mut r = Registry::default();
        r.set_plugin_commands("a", vec![cmd("a", "one"), cmd("a", "two")]);
        r.set_plugin_commands("b", vec![cmd("b", "one"), cmd("a", "sneaky")]);
        assert_eq!(r.plugin_commands().len(), 3);
        r.set_plugin_commands("a", vec![]);
        let left: Vec<_> = r.plugin_commands().iter().map(|c| c.cmd.key()).collect();
        assert_eq!(left, vec!["plugin.b.one"]);
        assert_eq!(r.refs().count(), CommandId::ALL.len() + 1);
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
    fn parse_and_spec_round_trip() {
        for text in ["Ctrl+K", "Ctrl+Shift+O", "Alt+Slash", "G H", "F5", "Ctrl+Plus"] {
            let s = Shortcut::parse(text).unwrap();
            assert_eq!(s.spec(), text);
        }
        assert_eq!(Shortcut::parse("cmd+k"), Ok(Shortcut::ctrl("K")));
        assert_eq!(Shortcut::parse("g h"), Ok(Shortcut::seq("G", "H")));
        assert_eq!(Shortcut::parse("Ctrl++"), Ok(Shortcut::ctrl("Plus")));
        assert!(Shortcut::parse("").is_err());
        assert!(Shortcut::parse("Hyper+K").is_err());
        assert!(Shortcut::parse("A B C").is_err());
        for c in Registry::default().all() {
            for s in &c.default_shortcuts {
                assert_eq!(Shortcut::parse(&s.spec()).as_ref(), Ok(s), "{}", s.spec());
            }
        }
    }

    #[test]
    fn assign_takes_clashing_shortcuts() {
        let mut r = Registry::default();
        let quit = CommandRef::Builtin(CommandId::Quit);
        let taken = r.assign(&quit, Some(Shortcut::ctrl("K")));
        assert_eq!(taken, vec![CommandRef::Builtin(CommandId::OpenCommandBar)]);
        assert!(
            !r.shortcuts(CommandId::OpenCommandBar)
                .contains(&Shortcut::ctrl("K"))
        );
        assert_eq!(r.bound_to(&Shortcut::ctrl("K")), vec![CommandId::Quit]);
        // Back to its own default: the rebinding is dropped.
        r.assign(&quit, Some(Shortcut::ctrl("Q")));
        assert!(!r.is_rebound(CommandId::Quit));
        r.reset(None);
        assert_eq!(r.bound_to(&Shortcut::ctrl("K")), vec![CommandId::OpenCommandBar]);
    }

    #[test]
    fn plugin_commands_can_be_rebound() {
        let mut r = Registry::default();
        let hello = CommandRef::parse("plugin.x.hello").unwrap();
        let other = CommandRef::parse("plugin.y.other").unwrap();
        assert!(r.shortcuts_of(&hello).is_empty());
        assert!(!r.is_rebound_ref(&hello));

        // Taking a built-in shortcut takes it from the built-in command.
        let taken = r.assign(&hello, Some(Shortcut::ctrl("K")));
        assert_eq!(taken, vec![CommandRef::Builtin(CommandId::OpenCommandBar)]);
        assert_eq!(r.shortcuts_of(&hello), &[Shortcut::ctrl("K")]);
        assert!(r.is_rebound_ref(&hello));
        assert!(r.bound_to(&Shortcut::ctrl("K")).is_empty());

        // And the other way round, and between plugins.
        r.assign(&other, Some(Shortcut::seq("G", "X")));
        assert!(r.assign(&hello, Some(Shortcut::key("G"))).contains(&other));
        assert!(!r.is_rebound_ref(&other), "an emptied binding is dropped");
        let taken = r.assign(&CommandRef::Builtin(CommandId::Quit), Some(Shortcut::key("G")));
        assert!(taken.contains(&hello));
        assert!(r.plugin_shortcuts().is_empty());

        r.assign(&hello, Some(Shortcut::ctrl("J")));
        r.assign(&hello, None);
        assert!(!r.is_rebound_ref(&hello));
        r.assign(&hello, Some(Shortcut::ctrl("J")));
        r.reset(Some(&hello));
        assert!(r.shortcuts_of(&hello).is_empty());
        r.assign(&hello, Some(Shortcut::ctrl("J")));
        r.reset(None);
        assert!(r.plugin_shortcuts().is_empty());
    }

    #[test]
    fn plain_key_clashes_with_sequence_start() {
        assert!(Shortcut::key("G").clashes(&Shortcut::seq("G", "H")));
        assert!(Shortcut::seq("G", "H").clashes(&Shortcut::key("G")));
        assert!(!Shortcut::ctrl("G").clashes(&Shortcut::seq("G", "H")));
        assert!(!Shortcut::seq("G", "H").clashes(&Shortcut::seq("G", "N")));
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
