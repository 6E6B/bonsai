//! The community invite is bundled at build time, never supplied by a command argument.

pub(crate) fn invite_url(code: &str) -> Result<String, String> {
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
    Ok(format!("https://discord.gg/{code}"))
}

#[cfg(test)]
mod tests {
    use super::invite_url;

    #[test]
    fn builds_an_https_invite_from_only_a_code() {
        assert_eq!(
            invite_url(" Abc-123_x\n").unwrap(),
            "https://discord.gg/Abc-123_x"
        );
        assert_eq!(invite_url("ARy4URNkFm").unwrap(), "https://discord.gg/ARy4URNkFm");
        for code in [
            "",
            " ",
            "../other",
            "https://evil.example",
            "abc?x=y",
            "abc/def",
            "a b",
        ] {
            assert!(invite_url(code).is_err());
        }
    }
}
