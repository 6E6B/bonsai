//! Plugin manifests and the Luau sandbox. No network and no store: everything a plugin does
//! outside its VM goes through [`Host`], which `bonsai-core` implements. Tier and grant checks
//! live there, not here. See `docs/plugins.md`.

mod manifest;
mod sandbox;

pub use manifest::{Manifest, Permission, Refused};
pub use sandbox::{Action, Arg, Command, Ctx, Host, Limits, Outcome, Plugin, Read, TradeStatus};

/// The plugin API version this build provides. A manifest asking for a newer one is refused.
pub const API_VERSION: u32 = 1;

/// Plugin and plugin command ids: lowercase letters, digits and dashes.
pub fn is_plugin_id(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Manifest(String),
    #[error("Took longer than {0} ms.")]
    Timeout(u128),
    #[error("Used more than {0} MB of memory.")]
    Memory(usize),
    /// A Luau error, with the plugin's file and line.
    #[error("{0}")]
    Script(String),
    #[error("No command \"{0}\".")]
    NoCommand(String),
    /// It returned an action its manifest doesn't list.
    #[error("Isn't allowed to run \"{0}\".")]
    Undeclared(String),
    #[error("Can't read {0}: {1}")]
    Io(String, std::io::Error),
}
