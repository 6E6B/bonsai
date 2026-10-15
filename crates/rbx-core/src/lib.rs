//! Domain layer between the UI and the API: accounts, sessions, caches, search, commands and
//! background jobs. The UI talks to [`Core`] and renders [`StoreData`].

pub mod account;
pub mod avatar;
pub mod cache;
pub mod chat;
pub mod commands;
pub mod communities;
pub mod core;
pub mod discover;
pub mod experience;
pub mod fmt;
pub mod inventory;
pub mod launcher;
pub mod links;
pub mod market;
pub mod me;
pub mod messages;
pub mod people;
pub mod purchase;
mod remote;
pub mod res;
pub mod resolve;
pub mod search;
pub mod seen;
pub mod settings;
pub mod signin;
pub mod store;
pub mod template;
pub mod text;
pub mod thumbs;
pub mod trades;
pub mod transactions;
pub mod values;

pub use crate::core::{Cmd, Core, CoreConfig, Repaint};
pub use account::{AccountMeta, ColorTag, Paths};
pub use rbx_api::{AccountId, Secret};
pub use store::StoreData;
