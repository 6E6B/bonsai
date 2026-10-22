//! Roblox web API: generated endpoint modules and the hand-written transport.
//!
//! This is the only crate that does network I/O. Endpoint URLs live only in `generated/`
//! (from the tracker specs and `specs/undocumented/`).

pub mod batch;
pub mod error;
pub mod fandom;
pub mod paging;
pub mod plugin_http;
pub mod realtime;
#[cfg(feature = "replay")]
pub mod replay;
pub mod rt;
pub mod secret;
pub mod transport;

#[rustfmt::skip]
#[allow(clippy::all, unreachable_patterns, non_camel_case_types, deprecated)]
pub mod generated;

pub use error::{ApiError, Challenge, Error, Result};
pub use secret::Secret;
pub use transport::{AccountId, Base, Client, RateLimiter, Response, RetryPolicy};
