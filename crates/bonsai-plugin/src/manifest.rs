//! `plugin.toml`. Read tolerantly like themes: unknown keys are ignored, unknown permissions and
//! actions are kept aside as [`Refused`] for the install dialog, and only a missing `id` or
//! `entry`, or a newer `bonsai`, refuse the whole plugin.

use serde::Deserialize;

use crate::{API_VERSION, Error, is_plugin_id};

/// What a plugin may ask for. See `docs/plugins.md#permissions`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Permission {
    ReadPublic,
    ReadAccount,
    Notify,
    ClipboardWrite,
    Net,
}

impl Permission {
    pub const ALL: [Permission; 5] = [
        Permission::ReadPublic,
        Permission::ReadAccount,
        Permission::Notify,
        Permission::ClipboardWrite,
        Permission::Net,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Permission::ReadPublic => "read.public",
            Permission::ReadAccount => "read.account",
            Permission::Notify => "notify",
            Permission::ClipboardWrite => "clipboard.write",
            Permission::Net => "net",
        }
    }

    /// `read.public` is shared; the rest are granted per account.
    pub fn per_account(self) -> bool {
        self != Permission::ReadPublic
    }
}

/// A permission or action the manifest asked for that the plugin won't get.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refused {
    pub what: String,
    pub why: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Manifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    /// Path of the entry module inside the plugin folder, like `main.luau`.
    pub entry: String,
    /// Plugin API version it was written for.
    pub api: u32,
    pub permissions: Vec<Permission>,
    /// Exact public DNS hosts permitted for credential-free HTTPS.
    pub network_hosts: Vec<String>,
    /// Built-in command keys it may run or propose. Checked against tiers by the host with
    /// [`Manifest::refuse_actions`].
    pub actions: Vec<String>,
    /// Shown under Settings > Plugins and read with `ctx.settings`.
    pub settings: Vec<Setting>,
    pub refused: Vec<Refused>,
}

/// Most settings a manifest may declare.
const MAX_SETTINGS: usize = 50;
/// Longest text setting value.
const MAX_TEXT: usize = 500;

/// One `[[settings]]` entry in the manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Setting {
    pub key: String,
    pub title: String,
    pub description: String,
    pub kind: SettingKind,
    /// A valid value for `kind`.
    pub default: serde_json::Value,
}

/// What control a setting takes, like `PrefKind` in bonsai-core.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SettingKind {
    Bool,
    /// `(value, label)` pairs.
    Choice(Vec<(String, String)>),
    Range {
        min: i64,
        max: i64,
        step: i64,
    },
    Text {
        placeholder: String,
    },
}

impl Setting {
    /// Whether `v` is a value this setting can hold.
    pub fn valid(&self, v: &serde_json::Value) -> bool {
        use serde_json::Value as J;
        match (&self.kind, v) {
            (SettingKind::Bool, J::Bool(_)) => true,
            (SettingKind::Choice(o), J::String(s)) => o.iter().any(|(k, _)| k == s),
            (SettingKind::Range { min, max, step }, J::Number(n)) => n
                .as_i64()
                .is_some_and(|n| n >= *min && n <= *max && (n - min) % step == 0),
            (SettingKind::Text { .. }, J::String(s)) => s.chars().count() <= MAX_TEXT,
            _ => false,
        }
    }

    /// Reads a value typed as text, like `settings.set`: `on` or `off`, an option's value, a
    /// whole number, or the text itself.
    pub fn parse(&self, text: &str) -> Result<serde_json::Value, String> {
        let t = text.trim();
        let v = match &self.kind {
            SettingKind::Bool => match t {
                "on" | "true" | "yes" => serde_json::Value::Bool(true),
                "off" | "false" | "no" => serde_json::Value::Bool(false),
                _ => return Err(format!("{} is on or off.", self.title)),
            },
            SettingKind::Choice(o) => {
                if !o.iter().any(|(k, _)| k == t) {
                    let keys: Vec<&str> = o.iter().map(|(k, _)| k.as_str()).collect();
                    return Err(format!("{} is one of {}.", self.title, keys.join(", ")));
                }
                t.into()
            }
            SettingKind::Range { min, max, step } => {
                let n = t.parse::<i64>().ok().map(serde_json::Value::from);
                match n {
                    Some(n) if self.valid(&n) => n,
                    _ => {
                        return Err(format!(
                            "{} is a whole number from {min} to {max} in steps of {step}.",
                            self.title
                        ));
                    }
                }
            }
            // Text keeps its spaces; only the command bar's split trims it.
            SettingKind::Text { .. } => {
                let v = serde_json::Value::from(text);
                if !self.valid(&v) {
                    return Err(format!("{} is at most {MAX_TEXT} characters.", self.title));
                }
                v
            }
        };
        Ok(v)
    }

    /// A value as the text [`Setting::parse`] reads back.
    pub fn text(v: &serde_json::Value) -> String {
        match v {
            serde_json::Value::Bool(true) => "on".into(),
            serde_json::Value::Bool(false) => "off".into(),
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        }
    }
}

/// Reads one `[[settings]]` table, or says why it is refused.
fn setting(v: &toml::Value) -> Result<Setting, (String, String)> {
    let t = v
        .as_table()
        .ok_or_else(|| ("settings".to_owned(), "Each setting is a table.".to_owned()))?;
    let text = |k: &str| t.get(k).and_then(|v| v.as_str()).map(|s| s.trim().to_owned());
    let key = text("key").unwrap_or_default();
    let what = format!("setting {key}");
    let no = |why: &str| (what.clone(), why.to_owned());
    if !key
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        || key.is_empty()
        || key.len() > 64
    {
        return Err(no("Its key needs lowercase letters, digits and underscores."));
    }
    let title = text("title")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| key.clone());
    let title: String = title.chars().take(120).collect();
    let description: String = text("description")
        .unwrap_or_default()
        .chars()
        .take(300)
        .collect();
    let int = |k: &str| t.get(k).and_then(|v| v.as_integer());
    let kind = match text("kind").as_deref() {
        Some("bool") => SettingKind::Bool,
        Some("choice") => {
            let options: Vec<(String, String)> = t
                .get("options")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|o| match o {
                            toml::Value::String(s) => Some((s.clone(), s.clone())),
                            toml::Value::Table(o) => {
                                let value = o.get("value")?.as_str()?.to_owned();
                                let label = o
                                    .get("label")
                                    .and_then(|l| l.as_str())
                                    .map_or_else(|| value.clone(), str::to_owned);
                                Some((value, label))
                            }
                            _ => None,
                        })
                        .take(50)
                        .collect()
                })
                .unwrap_or_default();
            if options.is_empty() {
                return Err(no("A choice needs options."));
            }
            SettingKind::Choice(options)
        }
        Some("range") => {
            let (min, max, step) = (
                int("min").unwrap_or(0),
                int("max").unwrap_or(100),
                int("step").unwrap_or(1),
            );
            if min >= max || step < 1 {
                return Err(no("A range needs min below max and a step of at least 1."));
            }
            SettingKind::Range { min, max, step }
        }
        Some("text") => SettingKind::Text {
            placeholder: text("placeholder").unwrap_or_default(),
        },
        _ => return Err(no("Its kind is bool, choice, range or text.")),
    };
    let default = match (&kind, t.get("default")) {
        (_, None) => match &kind {
            SettingKind::Bool => false.into(),
            SettingKind::Choice(o) => o[0].0.clone().into(),
            SettingKind::Range { min, .. } => (*min).into(),
            SettingKind::Text { .. } => "".into(),
        },
        (_, Some(toml::Value::Boolean(b))) => (*b).into(),
        (_, Some(toml::Value::Integer(n))) => (*n).into(),
        (_, Some(toml::Value::String(s))) => s.clone().into(),
        _ => serde_json::Value::Null,
    };
    let s = Setting {
        key,
        title,
        description,
        kind,
        default,
    };
    if !s.valid(&s.default) {
        return Err(no("Its default doesn't fit its kind."));
    }
    Ok(s)
}

#[derive(Deserialize)]
struct Raw {
    id: Option<String>,
    name: Option<String>,
    version: Option<String>,
    author: Option<String>,
    entry: Option<String>,
    bonsai: Option<u32>,
    #[serde(default)]
    permissions: Vec<toml::Value>,
    #[serde(default)]
    network_hosts: Vec<String>,
    #[serde(default)]
    actions: Vec<toml::Value>,
    #[serde(default)]
    settings: Vec<toml::Value>,
}

impl Manifest {
    pub fn parse(text: &str) -> Result<Manifest, Error> {
        let raw: Raw =
            toml::from_str(text).map_err(|e| Error::Manifest(format!("plugin.toml: {}", e.message())))?;
        let id = raw
            .id
            .ok_or_else(|| Error::Manifest("plugin.toml has no id.".into()))?;
        if !is_plugin_id(&id) {
            return Err(Error::Manifest(format!(
                "\"{id}\" isn't a plugin id. Use lowercase letters, digits and dashes."
            )));
        }
        let entry = raw
            .entry
            .ok_or_else(|| Error::Manifest("plugin.toml has no entry.".into()))?;
        if module_name(&entry).is_none() {
            return Err(Error::Manifest(format!(
                "entry \"{entry}\" must be a .luau file inside the plugin folder."
            )));
        }
        let api = raw.bonsai.unwrap_or(1);
        if api > API_VERSION {
            return Err(Error::Manifest(format!(
                "Needs plugin API {api}; this version of Bonsai has {API_VERSION}. Update Bonsai."
            )));
        }

        let mut refused = Vec::new();
        let mut permissions = Vec::new();
        for v in raw.permissions {
            let key = value_text(&v);
            match Permission::ALL.into_iter().find(|p| p.key() == key) {
                Some(p) if !permissions.contains(&p) => permissions.push(p),
                Some(_) => {}
                // `actions` is a permission in the docs' table but its own key in the manifest.
                None if key == "actions" => {}
                None => refused.push(Refused {
                    what: key,
                    why: "Unknown permission.".into(),
                }),
            }
        }
        let mut network_hosts = Vec::new();
        if raw.network_hosts.len() > 32 {
            return Err(Error::Manifest("At most 32 network hosts.".into()));
        }
        for host in raw.network_hosts {
            let host = host.to_ascii_lowercase();
            if !valid_network_host(&host) {
                return Err(Error::Manifest("Network hosts must be exact public DNS names outside Roblox, without wildcards, ports or URLs.".into()));
            }
            if !network_hosts.contains(&host) {
                network_hosts.push(host);
            }
        }
        if permissions.contains(&Permission::Net) && network_hosts.is_empty() {
            return Err(Error::Manifest("The net permission needs network_hosts.".into()));
        }
        if !network_hosts.is_empty() && !permissions.contains(&Permission::Net) {
            return Err(Error::Manifest("network_hosts needs the net permission.".into()));
        }
        let mut actions = Vec::new();
        for v in raw.actions {
            let key = value_text(&v);
            let ok = !key.is_empty()
                && key.contains('.')
                && key
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'.');
            if !ok {
                refused.push(Refused {
                    what: key,
                    why: "Not a command.".into(),
                });
            } else if !actions.contains(&key) {
                actions.push(key);
            }
        }

        let mut settings: Vec<Setting> = Vec::new();
        for v in &raw.settings {
            match setting(v) {
                Ok(_) if settings.len() >= MAX_SETTINGS => refused.push(Refused {
                    what: "settings".into(),
                    why: format!("At most {MAX_SETTINGS} settings."),
                }),
                Ok(s) if settings.iter().any(|x| x.key == s.key) => refused.push(Refused {
                    what: format!("setting {}", s.key),
                    why: "Listed twice.".into(),
                }),
                Ok(s) => settings.push(s),
                Err((what, why)) => refused.push(Refused { what, why }),
            }
        }

        Ok(Manifest {
            name: raw
                .name
                .filter(|n| !n.trim().is_empty())
                .unwrap_or_else(|| id.clone()),
            id,
            version: raw.version.unwrap_or_default(),
            author: raw.author,
            entry,
            api,
            permissions,
            network_hosts,
            actions,
            settings,
            refused,
        })
    }

    /// Moves actions the host won't allow into [`Manifest::refused`]. `check` returns why one is
    /// refused (unknown command, not a write, tier 4), or `None` to keep it.
    pub fn refuse_actions(&mut self, check: impl Fn(&str) -> Option<String>) {
        let mut kept = Vec::new();
        for a in std::mem::take(&mut self.actions) {
            match check(&a) {
                Some(why) => self.refused.push(Refused { what: a, why }),
                None => kept.push(a),
            }
        }
        self.actions = kept;
    }

    pub fn has(&self, p: Permission) -> bool {
        self.permissions.contains(&p)
    }

    pub fn declares(&self, action: &str) -> bool {
        self.actions.iter().any(|a| a == action)
    }
}

fn valid_network_host(host: &str) -> bool {
    host.len() <= 253
        && host.contains('.')
        && !["roblox.com", "rbxcdn.com", "roblox.cn", "rbx.com"]
            .iter()
            .any(|d| host == *d || host.ends_with(&format!(".{d}")))
        && !host.split('.').all(|p| p.bytes().all(|b| b.is_ascii_digit()))
        && host.split('.').all(|p| {
            !p.is_empty()
                && p.len() <= 63
                && !p.starts_with('-')
                && !p.ends_with('-')
                && p.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        })
}

fn value_text(v: &toml::Value) -> String {
    match v {
        toml::Value::String(s) => s.trim().to_owned(),
        other => format!("{other:?}"),
    }
}

/// `main.luau` or `lib/util.luau` → the module name without `.luau`. Only lowercase letters,
/// digits, `_` and `-` in each part, so a name can't leave the plugin folder.
pub(crate) fn module_name(path: &str) -> Option<&str> {
    let name = path.strip_suffix(".luau")?;
    valid_module(name).then_some(name)
}

pub(crate) fn valid_module(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 128
        && name.split('/').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = r#"
id = "friend-cleanup"
name = "Friend cleanup"
version = "1.0.0"
author = "nick"
entry = "main.luau"
bonsai = 1
homepage = "ignored"
permissions = ["read.public", "read.account", "notify", "net", "files.read", "read.public"]
network_hosts = ["example.com"]
actions = ["user.unfriend", "user.decline_friend", "Not A Command", "user.unfriend"]
"#;

    #[test]
    fn reads_the_doc_example_tolerantly() {
        let m = Manifest::parse(DOC).unwrap();
        assert_eq!(m.id, "friend-cleanup");
        assert_eq!(m.name, "Friend cleanup");
        assert_eq!(m.author.as_deref(), Some("nick"));
        assert_eq!(
            m.permissions,
            vec![
                Permission::ReadPublic,
                Permission::ReadAccount,
                Permission::Notify,
                Permission::Net
            ]
        );
        assert_eq!(m.actions, vec!["user.unfriend", "user.decline_friend"]);
        let refused: Vec<_> = m.refused.iter().map(|r| r.what.as_str()).collect();
        assert_eq!(refused, vec!["files.read", "Not A Command"]);
        assert!(m.declares("user.unfriend"));
        assert!(!m.declares("market.buy"));
    }

    #[test]
    fn refuses_missing_id_entry_or_newer_api() {
        assert!(Manifest::parse("entry = \"main.luau\"").is_err());
        assert!(Manifest::parse("id = \"x\"").is_err());
        assert!(Manifest::parse("id = \"Bad Id\"\nentry = \"main.luau\"").is_err());
        assert!(Manifest::parse("id = \"x\"\nentry = \"main.luau\"\nbonsai = 2").is_err());
        assert!(Manifest::parse("id = \"x\"\nentry = \"../main.luau\"").is_err());
        assert!(Manifest::parse("id = \"x\"\nentry = \"/etc/main.luau\"").is_err());
        assert!(Manifest::parse("id = \"x\"\nentry = \"main.lua\"").is_err());
        let m = Manifest::parse("id = \"x\"\nentry = \"src/main.luau\"").unwrap();
        assert_eq!((m.name.as_str(), m.api), ("x", 1));
    }

    #[test]
    fn net_hosts_are_explicit_bounded_and_outside_roblox() {
        for host in [
            "*.example.com",
            "https://example.com",
            "example.com:443",
            "localhost",
            "127.0.0.1",
            "[::1]",
            "roblox.com",
            "auth.roblox.com",
            "a.rbxcdn.com",
            "example.com.",
            "a..example.com",
            "-a.example.com",
        ] {
            let doc = format!(
                "id = \"x\"\nentry = \"main.luau\"\npermissions = [\"net\"]\nnetwork_hosts = [\"{host}\"]"
            );
            assert!(Manifest::parse(&doc).is_err(), "{host}");
        }
        let doc = "id = \"x\"\nentry = \"main.luau\"\npermissions = [\"net\"]\nnetwork_hosts = [\"API.Example.com\", \"api.example.com\"]";
        assert_eq!(
            Manifest::parse(doc).unwrap().network_hosts,
            vec!["api.example.com"]
        );
        assert!(Manifest::parse("id = \"x\"\nentry = \"main.luau\"\npermissions = [\"net\"]").is_err());
    }

    #[test]
    fn host_can_refuse_actions() {
        let mut m = Manifest::parse(DOC).unwrap();
        m.refuse_actions(|a| (a == "user.decline_friend").then(|| "Not allowed.".to_owned()));
        assert_eq!(m.actions, vec!["user.unfriend"]);
        assert_eq!(
            m.refused.last().map(|r| r.what.as_str()),
            Some("user.decline_friend")
        );
    }

    #[test]
    fn reads_settings_and_refuses_bad_ones() {
        let m = Manifest::parse(
            r#"
id = "x"
entry = "main.luau"

[[settings]]
key = "min_age"
title = "Minimum age"
kind = "range"
min = 0
max = 30
default = 7

[[settings]]
key = "mode"
kind = "choice"
options = ["quiet", { value = "loud", label = "Loud" }]

[[settings]]
key = "on"
kind = "bool"
default = true

[[settings]]
key = "Bad Key"
kind = "bool"

[[settings]]
key = "far"
kind = "range"
max = 10
default = 11

[[settings]]
key = "what"
kind = "colour"
"#,
        )
        .unwrap();
        let keys: Vec<&str> = m.settings.iter().map(|s| s.key.as_str()).collect();
        assert_eq!(keys, vec!["min_age", "mode", "on"]);
        assert_eq!(m.settings[0].default, serde_json::json!(7));
        assert_eq!(m.settings[1].default, serde_json::json!("quiet"));
        assert_eq!(m.refused.len(), 3);

        let age = &m.settings[0];
        assert_eq!(age.parse("12"), Ok(serde_json::json!(12)));
        assert!(age.parse("31").is_err());
        assert!(age.parse("1.5").is_err());
        assert_eq!(m.settings[1].parse("loud"), Ok(serde_json::json!("loud")));
        assert!(m.settings[1].parse("Loud").is_err());
        assert_eq!(m.settings[2].parse("off"), Ok(serde_json::json!(false)));
        assert_eq!(Setting::text(&serde_json::json!(true)), "on");
    }

    #[test]
    fn module_names_stay_inside() {
        for ok in ["main.luau", "lib/util.luau", "a-b_c.luau"] {
            assert!(module_name(ok).is_some(), "{ok}");
        }
        for bad in [
            "../x.luau",
            "/x.luau",
            "a//b.luau",
            "./x.luau",
            "x",
            "X.luau",
            "a\\b.luau",
            ".luau",
        ] {
            assert!(module_name(bad).is_none(), "{bad}");
        }
    }
}
