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
}

impl Permission {
    pub const ALL: [Permission; 4] = [
        Permission::ReadPublic,
        Permission::ReadAccount,
        Permission::Notify,
        Permission::ClipboardWrite,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Permission::ReadPublic => "read.public",
            Permission::ReadAccount => "read.account",
            Permission::Notify => "notify",
            Permission::ClipboardWrite => "clipboard.write",
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
    /// Built-in command keys it may run or propose. Checked against tiers by the host with
    /// [`Manifest::refuse_actions`].
    pub actions: Vec<String>,
    pub refused: Vec<Refused>,
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
    actions: Vec<toml::Value>,
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
                None if key == "net" => refused.push(Refused {
                    what: key,
                    why: "Network access isn't available to plugins yet.".into(),
                }),
                // `actions` is a permission in the docs' table but its own key in the manifest.
                None if key == "actions" => {}
                None => refused.push(Refused {
                    what: key,
                    why: "Unknown permission.".into(),
                }),
            }
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
            actions,
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
                Permission::Notify
            ]
        );
        assert_eq!(m.actions, vec!["user.unfriend", "user.decline_friend"]);
        let refused: Vec<_> = m.refused.iter().map(|r| r.what.as_str()).collect();
        assert_eq!(refused, vec!["net", "files.read", "Not A Command"]);
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
