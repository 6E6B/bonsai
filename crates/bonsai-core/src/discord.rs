//! The community invite is bundled at build time, never supplied by a command argument.

pub(crate) fn invite_url(code: &str, browser: bool) -> Result<String, String> {
    let code = code.trim();
    if code.is_empty() {
        return Err("The Bonsai Discord invite is not available yet.".into());
    }
    if !code
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return Err("The Bonsai Discord invite code is invalid.".into());
    }
    let prefix = if browser {
        "https://discord.com/invite/"
    } else {
        "discord://-/invite/"
    };
    Ok(format!("{prefix}{code}"))
}

/// Keep protocol failures away from the browser's address bar.
pub(crate) fn launch_invite(
    opener: &dyn crate::launcher::Opener,
    code: &str,
    browser: bool,
) -> Result<(), String> {
    let web = invite_url(code, true)?;
    if !browser && opener.open_protocol(&invite_url(code, false)?).is_ok() {
        return Ok(());
    }
    opener.open_web(&web)
}

#[cfg(test)]
mod tests {
    use super::{invite_url, launch_invite};
    use crate::launcher::{Opener, RecordingOpener};

    struct Unavailable;

    impl Opener for Unavailable {
        fn open(&self, _: &bonsai_api::Secret) -> Result<(), String> {
            panic!("Discord must not use the Roblox player command");
        }

        fn open_protocol(&self, uri: &str) -> Result<(), String> {
            assert_eq!(uri, "discord://-/invite/Abc123");
            Err("No handler".into())
        }

        fn open_web(&self, url: &str) -> Result<(), String> {
            assert_eq!(url, "https://discord.com/invite/Abc123");
            Ok(())
        }
    }

    #[test]
    fn native_invite_uses_the_protocol_launcher() {
        let opener = RecordingOpener::default();
        launch_invite(&opener, "Abc123", false).unwrap();
        assert_eq!(*opener.protocols.lock(), ["discord://-/invite/Abc123"]);
        assert!(opener.web.lock().is_empty());
        assert!(opener.opened.lock().is_empty());
    }

    #[test]
    fn browser_invite_skips_the_protocol_launcher() {
        let opener = RecordingOpener::default();
        launch_invite(&opener, "Abc123", true).unwrap();
        assert_eq!(*opener.web.lock(), ["https://discord.com/invite/Abc123"]);
        assert!(opener.protocols.lock().is_empty());
        assert!(opener.opened.lock().is_empty());
    }

    #[test]
    fn missing_protocol_handler_falls_back_to_https() {
        launch_invite(&Unavailable, "Abc123", false).unwrap();
    }

    #[test]
    fn opens_only_an_invite_code() {
        assert_eq!(
            invite_url(" Abc-123_x\n", false).unwrap(),
            "discord://-/invite/Abc-123_x"
        );
        assert_eq!(
            invite_url("Abc123", true).unwrap(),
            "https://discord.com/invite/Abc123"
        );
        for code in [
            "",
            " ",
            "../other",
            "https://evil.example",
            "abc?x=y",
            "abc/def",
            "a b",
        ] {
            assert!(invite_url(code, false).is_err());
        }
    }
}
