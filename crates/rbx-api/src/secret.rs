//! A string that never shows up in `Debug`, `Display` or logs.

use std::sync::Arc;

/// Cookies, auth tickets and CSRF tokens. `Debug` prints `<redacted>`; there is no `Display`
/// and no `Serialize`, so a secret cannot end up in a log line or a config file by accident.
#[derive(Clone, PartialEq, Eq)]
pub struct Secret(Arc<str>);

impl Secret {
    pub fn new(s: impl AsRef<str>) -> Self {
        Self(Arc::from(s.as_ref()))
    }
    /// The raw value. Only for putting it on the wire or in the OS keychain.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<redacted>")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn debug_is_redacted() {
        let s = super::Secret::new("_|WARNING:-DO-NOT-SHARE-THIS.--abc");
        assert_eq!(format!("{s:?}"), "<redacted>");
        assert_eq!(format!("{:?}", Some(s)), "Some(<redacted>)");
    }
}
