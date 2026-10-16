//! The app background: a picture the user picks from disk or links to, shown behind every page.
//! It is copied (or downloaded through `rbx-api`) into `Paths::backgrounds_dir()` under a name
//! made from its contents, so the original can move and the webview loads it over `bg://`.
//! Only PNG, JPEG, GIF and WebP files are kept, checked by their header.

use std::path::{Path, PathBuf};

use crate::Core;
use crate::core::{Cmd, Inner};
use crate::settings::Background;
use sha2::{Digest, Sha256};

/// Large enough for a long GIF; small enough to keep the webview happy.
pub const MAX_BYTES: usize = 32 << 20;
const EXTS: [&str; 4] = ["png", "jpg", "gif", "webp"];

/// Where a new background comes from.
#[derive(Clone, Debug)]
pub enum Source {
    File(PathBuf),
    /// An `https://` link.
    Url(String),
}

impl Source {
    /// A pasted link or path. Quotes around it (as file managers copy paths) are dropped.
    pub fn parse(s: &str) -> Result<Source, String> {
        let s = s.trim().trim_matches(['"', '\'']);
        if s.is_empty() {
            return Err("Give a link or a file path.".into());
        }
        if s.starts_with("https://") {
            return Ok(Source::Url(s.to_owned()));
        }
        if s.starts_with("http://") {
            return Err("Use an https link.".into());
        }
        let path = match url::Url::parse(s) {
            Ok(u) if u.scheme() == "file" => u.to_file_path().map_err(|_| "Not a file path.".to_owned())?,
            _ => PathBuf::from(s),
        };
        if !path.is_absolute() {
            return Err("Give the full path to the file.".into());
        }
        Ok(Source::File(path))
    }
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

/// Whether `name` is a file this module writes (`<16 hex>.<ext>`), so `bg://` can serve it
/// without leaving the folder.
pub fn valid_file(name: &str) -> bool {
    let Some((stem, ext)) = name.split_once('.') else {
        return false;
    };
    stem.len() == 16
        && stem
            .bytes()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
        && EXTS.contains(&ext)
}

/// The Content-Type for a file name from [`valid_file`].
pub fn content_type(name: &str) -> &'static str {
    match name.rsplit_once('.').map(|(_, e)| e) {
        Some("png") => "image/png",
        Some("jpg") => "image/jpeg",
        Some("gif") => "image/gif",
        _ => "image/webp",
    }
}

/// Checks the bytes and writes them under their content name. Returns that name.
pub fn save(dir: &Path, bytes: &[u8]) -> Result<String, String> {
    if bytes.len() > MAX_BYTES {
        return Err(format!("The image is over {} MB.", MAX_BYTES >> 20));
    }
    let ext = image_ext(bytes).ok_or("That isn't a PNG, JPEG, GIF or WebP image.")?;
    let hash = Sha256::digest(bytes);
    let stem: String = hash[..8].iter().map(|b| format!("{b:02x}")).collect();
    let name = format!("{stem}.{ext}");
    let fail = |_| "Couldn't save the image.".to_owned();
    std::fs::create_dir_all(dir).map_err(fail)?;
    let path = dir.join(&name);
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, bytes).map_err(fail)?;
    std::fs::rename(tmp, path).map_err(fail)?;
    Ok(name)
}

/// Deletes every picture in `dir` except `keep`.
pub fn prune(dir: &Path, keep: Option<&str>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let name = e.file_name();
        let name = name.to_string_lossy();
        if Some(name.as_ref()) != keep && (valid_file(&name) || name.ends_with(".tmp")) {
            let _ = std::fs::remove_file(e.path());
        }
    }
}

async fn read(inner: &Inner, src: Source) -> Result<Vec<u8>, String> {
    match src {
        Source::File(path) => tokio::task::spawn_blocking(move || {
            let size = std::fs::metadata(&path)
                .map_err(|_| "Couldn't open that file.".to_owned())?
                .len();
            if size > MAX_BYTES as u64 {
                return Err(format!("The image is over {} MB.", MAX_BYTES >> 20));
            }
            std::fs::read(&path).map_err(|_| "Couldn't read that file.".to_owned())
        })
        .await
        .unwrap_or_else(|_| Err("Couldn't read that file.".into())),
        Source::Url(url) => inner
            .anon
            .fetch_user_image(&url, MAX_BYTES)
            .await
            .map_err(|e| format!("Couldn't download the image: {}", e.user_message())),
    }
}

impl Inner {
    pub(crate) async fn load_background(&self, src: Source) {
        let dir = self.paths.backgrounds_dir();
        let saved = match read(self, src).await {
            Ok(bytes) => tokio::task::spawn_blocking(move || save(&dir, &bytes))
                .await
                .unwrap_or_else(|_| Err("Couldn't save the image.".into())),
            Err(e) => Err(e),
        };
        let file = match saved {
            Ok(f) => f,
            Err(e) => {
                self.toast(None, e, true);
                return;
            }
        };
        let s = {
            let mut s = self.settings.write();
            let old = s.appearance.background.take().unwrap_or_default();
            s.appearance.background = Some(Background { file, ..old });
            s.clone()
        };
        if let Err(e) = s.save(&self.paths.settings_file()) {
            tracing::warn!("saving settings: {e}");
        }
        let keep = s.appearance.background.map(|b| b.file);
        prune(&self.paths.backgrounds_dir(), keep.as_deref());
    }
}

impl Core {
    /// Shows a picture behind the app, from a full file path or an https link. Copying or
    /// downloading happens on the worker; a failure shows as a toast.
    pub fn set_background(&self, source: &str) -> Result<(), String> {
        let src = Source::parse(source)?;
        if matches!(src, Source::Url(_)) {
            self.notice("Downloading the background…", false);
        }
        self.send(Cmd::LoadBackground(src));
        Ok(())
    }

    /// Takes the background picture away and deletes its copy.
    pub fn clear_background(&self) {
        self.update_settings(|s| s.appearance.background = None);
        self.send(Cmd::PruneBackgrounds);
    }

    /// Sets one level of the picture, as `"<name> <number>"`: `image` and `panels` (0–100),
    /// `frost` (blur behind cards, 0–40 px), or an effect (see [`Background::LEVELS`]). Out of range values are clamped.
    pub fn set_background_level(&self, arg: &str) -> Result<(), String> {
        let (name, n) = parse_level(arg)?;
        if self.settings().appearance.background.is_none() {
            return Err("Set a background picture first.".into());
        }
        self.update_settings(|s| {
            if let Some(slot) = s.appearance.background.as_mut().and_then(|b| b.level_mut(name)) {
                *slot = n;
            }
        });
        Ok(())
    }

    /// Turns off blur, dim, tint and the other effects on the background picture.
    pub fn reset_background_effects(&self) -> Result<(), String> {
        if self.settings().appearance.background.is_none() {
            return Err("Set a background picture first.".into());
        }
        self.update_settings(|s| {
            if let Some(b) = &mut s.appearance.background {
                b.reset_effects();
            }
        });
        Ok(())
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
    fn file_names() {
        assert!(valid_file("0123456789abcdef.gif"));
        assert!(!valid_file("0123456789ABCDEF.gif"));
        assert!(!valid_file("0123456789abcdef.svg"));
        assert!(!valid_file("../23456789abcdef.png"));
        assert!(!valid_file("0123456789abcdef.png.tmp"));
        assert!(!valid_file("abc.png"));
    }

    #[test]
    fn parses_sources() {
        assert!(matches!(
            Source::parse(" https://x.example/a.gif "),
            Ok(Source::Url(_))
        ));
        assert!(Source::parse("http://x.example/a.gif").is_err());
        assert!(Source::parse("pics/a.png").is_err());
        assert!(Source::parse("").is_err());
        #[cfg(unix)]
        {
            assert!(
                matches!(Source::parse("'/home/a/b c.png'"), Ok(Source::File(p)) if p == Path::new("/home/a/b c.png"))
            );
            assert!(
                matches!(Source::parse("file:///home/a/b%20c.png"), Ok(Source::File(p)) if p == Path::new("/home/a/b c.png"))
            );
        }
    }

    #[test]
    fn saves_by_content_and_prunes_the_rest() {
        let dir = tempfile::tempdir().unwrap();
        let a = save(dir.path(), b"GIF89a-one").unwrap();
        let b = save(dir.path(), b"\x89PNG\r\n\x1a\ntwo").unwrap();
        assert!(valid_file(&a) && a.ends_with(".gif"));
        assert_eq!(save(dir.path(), b"GIF89a-one").unwrap(), a);
        assert!(save(dir.path(), b"not an image").is_err());
        std::fs::write(dir.path().join("keep.txt"), "x").unwrap();
        prune(dir.path(), Some(&b));
        assert!(!dir.path().join(&a).exists());
        assert!(dir.path().join(&b).exists());
        assert!(dir.path().join("keep.txt").exists());
    }
}
