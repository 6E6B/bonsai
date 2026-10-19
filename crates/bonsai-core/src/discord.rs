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

#[cfg(test)]
mod tests {
    use super::invite_url;

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
