//! `ctx.store`: what a plugin keeps between runs, in `plugin-data/<plugin>/<account>.json`.
//! Account 0 is the plugin's own storage for calls with no account. Each file is capped, and
//! a plugin only reaches the accounts it was granted.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use serde_json::Value;

use crate::AccountId;
use crate::core::Inner;

use super::Status;

/// Most bytes one plugin may keep for one account.
pub(super) const MAX_BYTES: usize = 256 * 1024;

/// Loaded storage files, by plugin and account.
pub(super) type Data = std::collections::HashMap<(String, u64), BTreeMap<String, Value>>;

impl Inner {
    fn store_file(&self, plugin: &str, account: u64) -> PathBuf {
        self.paths
            .plugin_data_dir()
            .join(plugin)
            .join(format!("{account}.json"))
    }

    /// The plugin is running and may use `account`, or it is 0.
    fn may_store(&self, plugin: &str, account: u64) -> Result<(), String> {
        let p = self.plugins.lock();
        let e = p
            .entries
            .iter()
            .find(|e| e.info.id == plugin)
            .ok_or("This plugin isn't installed.")?;
        if !matches!(e.info.status, Status::Running { .. }) {
            return Err("This plugin isn't running.".into());
        }
        if account != 0 && !e.info.accounts.contains(&AccountId(account)) {
            return Err("It may not use this account. Allow it in Settings > Plugins.".into());
        }
        Ok(())
    }

    /// Reads the file once; later calls use the copy in memory. On the plugin's thread.
    fn with_store<T>(
        &self,
        plugin: &str,
        account: u64,
        f: impl FnOnce(&mut BTreeMap<String, Value>) -> T,
    ) -> T {
        let key = (plugin.to_owned(), account);
        let loaded = self.plugins.lock().data.contains_key(&key);
        if !loaded {
            let map: BTreeMap<String, Value> = std::fs::read_to_string(self.store_file(plugin, account))
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default();
            self.plugins.lock().data.entry(key.clone()).or_insert(map);
        }
        f(self.plugins.lock().data.entry(key).or_default())
    }

    pub(super) fn plugin_store_get(&self, plugin: &str, account: u64, key: &str) -> Result<Value, String> {
        self.may_store(plugin, account)?;
        Ok(self.with_store(plugin, account, |m| m.get(key).cloned().unwrap_or(Value::Null)))
    }

    pub(super) fn plugin_store_set(
        self: &Arc<Self>,
        plugin: &str,
        account: u64,
        key: &str,
        value: Value,
    ) -> Result<(), String> {
        self.may_store(plugin, account)?;
        let json = self.with_store(plugin, account, |m| {
            let old = if value.is_null() {
                m.remove(key)
            } else {
                m.insert(key.to_owned(), value)
            };
            let json = serde_json::to_string(m).unwrap_or_default();
            if json.len() > MAX_BYTES {
                // Put it back as it was.
                match old {
                    Some(v) => m.insert(key.to_owned(), v),
                    None => m.remove(key),
                };
                return Err(format!(
                    "ctx.store is full: at most {} KB per account.",
                    MAX_BYTES / 1024
                ));
            }
            Ok(json)
        })?;
        let path = self.store_file(plugin, account);
        let write = || -> std::io::Result<()> {
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir)?;
            }
            let tmp = path.with_extension("json.new");
            std::fs::write(&tmp, json)?;
            std::fs::rename(&tmp, &path)
        };
        write().map_err(|e| format!("Can't save: {e}"))
    }

    /// Forgets and deletes everything a plugin kept, when it is removed.
    pub(super) fn drop_plugin_data(&self, plugin: &str) -> std::io::Result<()> {
        self.plugins.lock().data.retain(|(p, _), _| p != plugin);
        super::remove_dir(&self.paths.plugin_data_dir().join(plugin))
    }
}
