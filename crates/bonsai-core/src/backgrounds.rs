//! Theme backgrounds: external HTTPS links downloaded without credentials through `bonsai-api`.
//! Images are cached locally and served through `bg://`.
//! Only PNG, JPEG, GIF and WebP files are kept, checked by their header.

use crate::Core;
use crate::settings::Background;
use sha2::{Digest, Sha256};

/// Large enough for a long GIF; small enough to keep the webview happy.
pub const MAX_BYTES: usize = 32 << 20;
/// The external link in a bg protocol request. URL parsing stays in core.
pub fn url_from_query(query: &str) -> Option<String> {
    url::form_urlencoded::parse(query.as_bytes())
        .find(|(key, _)| key == "url")
        .map(|(_, value)| value.into_owned())
}

/// Theme backgrounds must remain portable: only external HTTPS URLs, never local files or data.
pub fn validate_url(value: &str) -> Result<(), String> {
    let url = url::Url::parse(value).map_err(|_| "Use an external https image link.".to_owned())?;
    if !value.starts_with("https://")
        || value.len() > 4096
        || url.scheme() != "https"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err("Use an external https image link.".into());
    }
    // The download also refuses names that resolve to these; this catches them early.
    let local = match url.host() {
        Some(url::Host::Ipv4(ip)) => !bonsai_api::transport::is_public_ip(ip.into()),
        Some(url::Host::Ipv6(ip)) => !bonsai_api::transport::is_public_ip(ip.into()),
        Some(url::Host::Domain(d)) => d == "localhost" || d.ends_with(".localhost"),
        None => true,
    };
    if local {
        return Err("Use an external https image link, not a local or private address.".into());
    }
    Ok(())
}

/// The file extension for an image by its header, or None when it isn't one we show.
pub fn image_ext(b: &[u8]) -> Option<&'static str> {
    if b.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some("png")
    } else if b.starts_with(&[0xff, 0xd8, 0xff]) {
        Some("jpg")
    } else if b.starts_with(b"GIF87a") || b.starts_with(b"GIF89a") {
        Some("gif")
    } else if b.len() >= 12 && &b[..4] == b"RIFF" && &b[8..12] == b"WEBP" {
        Some("webp")
    } else {
        None
    }
}

/// The Content-Type for an image extension.
pub fn content_type(name: &str) -> &'static str {
    match name.rsplit_once('.').map(|(_, e)| e) {
        Some("png") => "image/png",
        Some("jpg") => "image/jpeg",
        Some("gif") => "image/gif",
        _ => "image/webp",
    }
}

impl Core {
    /// Fetches an image without credentials, caching it locally for offline reuse. Called only
    /// from the async bg protocol; the webview never contacts the external image host.
    pub async fn background_image(&self, url: &str) -> Result<(Vec<u8>, &'static str), String> {
        validate_url(url)?;
        if !self.is_shown_background(url) {
            return Err("That image isn't the background of a theme in use.".into());
        }
        let hash = Sha256::digest(url.as_bytes());
        let key: String = hash.iter().map(|b| format!("{b:02x}")).collect();
        let path = self.paths().backgrounds_dir().join(format!("{key}.image"));
        let cached = path.clone();
        if let Ok(Ok(bytes)) = tokio::task::spawn_blocking(move || std::fs::read(cached)).await
            && bytes.len() <= MAX_BYTES
            && let Some(ext) = image_ext(&bytes)
        {
            return Ok((bytes, content_type(&format!("image.{ext}"))));
        }
        let bytes = self
            .inner
            .anon
            .fetch_user_image(url, MAX_BYTES)
            .await
            .map_err(|e| format!("Couldn't download the image: {}", e.user_message()))?;
        let ext = image_ext(&bytes).ok_or("That isn't a PNG, JPEG, GIF or WebP image.")?;
        let cached = bytes.clone();
        let _ = tokio::task::spawn_blocking(move || {
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir)?;
            }
            // Concurrent requests write identical content, each through its own temporary file.
            let tmp = path.with_extension(format!("{}.tmp", fastrand::u64(..)));
            std::fs::write(&tmp, cached)?;
            std::fs::rename(tmp, path)
        })
        .await;
        Ok((bytes, content_type(&format!("image.{ext}"))))
    }

    /// Whether `url` is the background of the dark or light theme in use. `bg://` serves only
    /// these, so it can't fetch arbitrary links for whoever builds the URL.
    fn is_shown_background(&self, url: &str) -> bool {
        [crate::themes::Base::Dark, crate::themes::Base::Light]
            .into_iter()
            .any(|b| {
                self.appearance_theme(b)
                    .background
                    .is_some_and(|bg| bg.url == url)
            })
    }

    /// Edits the current theme, copying a built-in first, while preserving System mode.
    fn edit_background(&self, edit: impl FnOnce(&mut Option<Background>)) -> Result<(), String> {
        let mut theme = self.appearance_theme(self.shown_base());
        if self.inner.themes.read().is_builtin(&theme.id) {
            let id = self.duplicate_theme(&theme.id)?;
            theme = self.theme_or_shown(&id)?;
        }
        edit(&mut theme.background);
        let (id, base) = (theme.id.clone(), theme.base);
        self.save_theme(theme)?;
        self.update_settings(|s| match base {
            crate::themes::Base::Dark => s.appearance.dark = id,
            crate::themes::Base::Light => s.appearance.light = id,
        });
        Ok(())
    }

    pub fn set_background(&self, source: &str) -> Result<(), String> {
        let url = source.trim();
        validate_url(url)?;
        self.edit_background(|background| {
            let old = background.take().unwrap_or_default();
            *background = Some(Background {
                url: url.to_owned(),
                ..old
            });
        })
    }

    pub fn clear_background(&self) -> Result<(), String> {
        if self.appearance_theme(self.shown_base()).background.is_none() {
            return Ok(());
        }
        self.edit_background(|background| *background = None)
    }

    pub fn set_background_level(&self, arg: &str) -> Result<(), String> {
        let (name, n) = parse_level(arg)?;
        if self.appearance_theme(self.shown_base()).background.is_none() {
            return Err("Set a background picture first.".into());
        }
        self.edit_background(|background| {
            if let Some(slot) = background.as_mut().and_then(|b| b.level_mut(name)) {
                *slot = n;
            }
        })
    }

    pub fn reset_background_effects(&self) -> Result<(), String> {
        if self.appearance_theme(self.shown_base()).background.is_none() {
            return Err("Set a background picture first.".into());
        }
        self.edit_background(|background| {
            if let Some(b) = background {
                b.reset_effects();
            }
        })
    }
}

/// `"<name> <number>"` to a known level name and its value, clamped to that level's range.
fn parse_level(arg: &str) -> Result<(&'static str, u8), String> {
    let names = || {
        Background::LEVELS
            .iter()
            .map(|(n, _)| *n)
            .collect::<Vec<_>>()
            .join(", ")
    };
    let mut parts = arg.split_whitespace();
    let which = parts.next().unwrap_or_default().to_ascii_lowercase();
    let Some(&(name, max)) = Background::LEVELS.iter().find(|(n, _)| *n == which) else {
        return Err(format!("Give one of {}, then a number.", names()));
    };
    let n = parts
        .next()
        .and_then(|n| n.trim_end_matches("px").trim_end_matches('%').parse::<f32>().ok())
        .filter(|n| n.is_finite())
        .ok_or_else(|| format!("Give a number from 0 to {max} for {name}."))?;
    Ok((name, n.round().clamp(0.0, f32::from(max)) as u8))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_levels() {
        assert_eq!(parse_level("blur 12"), Ok(("blur", 12)));
        assert_eq!(parse_level("Blur 99"), Ok(("blur", 40)));
        assert_eq!(parse_level("saturation 150%"), Ok(("saturation", 150)));
        assert_eq!(parse_level("panels -5"), Ok(("panels", 0)));
        assert_eq!(parse_level("blur 8px"), Ok(("blur", 8)));
        assert!(parse_level("glow 5").is_err());
        assert!(parse_level("dim").is_err());
        assert!(parse_level("").is_err());
    }

    #[test]
    fn sniffs_images() {
        assert_eq!(image_ext(b"\x89PNG\r\n\x1a\nrest"), Some("png"));
        assert_eq!(image_ext(&[0xff, 0xd8, 0xff, 0xe0]), Some("jpg"));
        assert_eq!(image_ext(b"GIF89a..."), Some("gif"));
        assert_eq!(image_ext(b"RIFF\0\0\0\0WEBPVP8 "), Some("webp"));
        assert_eq!(image_ext(b"<svg>"), None);
        assert_eq!(image_ext(b""), None);
    }

    #[test]
    fn protocol_query_preserves_image_query_parameters() {
        assert_eq!(
            url_from_query("url=https%3A%2F%2Fexample.com%2Fa.png%3Fx%3D1%26y%3Dtwo%2Bthree").as_deref(),
            Some("https://example.com/a.png?x=1&y=two+three")
        );
        assert!(url_from_query("file=local.png").is_none());
    }

    #[test]
    fn parses_sources() {
        assert!(validate_url("https://x.example/a.gif?x=1&y=2").is_ok());
        for value in [
            "http://x.example/a.gif",
            "pics/a.png",
            "",
            "/home/a.png",
            "file:///a.png",
            "data:image/png;base64,abc",
            "https://user:pass@example.com/a",
            "https://localhost/a.png",
            "https://127.0.0.1/a.png",
            "https://192.168.1.10:8443/a.png",
            "https://169.254.169.254/latest",
            "https://[::1]/a.png",
            "https://[fd00::1]/a.png",
        ] {
            assert!(validate_url(value).is_err(), "{value}");
        }
    }
}
