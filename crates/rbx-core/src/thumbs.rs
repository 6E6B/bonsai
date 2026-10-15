//! Thumbnails: batched lookups through the thumbnails batch endpoint, `Pending` polling, and a
//! disk cache of image bytes keyed by URL hash. The encoded bytes go to the store for the UI,
//! which serves them as they are.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use rbx_api::generated::thumbnails;
use rbx_api::{Client, Error};
use sha2::{Digest, Sha256};

/// What kind of image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ThumbKind {
    Headshot,
    Avatar,
    GameIcon,
    GameThumbnail,
    Asset,
    Bundle,
    GroupIcon,
    BadgeIcon,
    GamePass,
    Outfit,
}

impl ThumbKind {
    fn api_type(self) -> &'static str {
        match self {
            ThumbKind::Headshot => "AvatarHeadShot",
            ThumbKind::Avatar => "Avatar",
            ThumbKind::GameIcon => "GameIcon",
            ThumbKind::GameThumbnail => "GameThumbnail",
            ThumbKind::Asset => "Asset",
            ThumbKind::Bundle => "BundleThumbnail",
            ThumbKind::GroupIcon => "GroupIcon",
            ThumbKind::BadgeIcon => "BadgeIcon",
            ThumbKind::GamePass => "GamePass",
            ThumbKind::Outfit => "Outfit",
        }
    }
    /// Sizes the API accepts for this type; we use one per kind per use.
    pub fn size(self, large: bool) -> &'static str {
        match (self, large) {
            (ThumbKind::Headshot, false) => "48x48",
            (ThumbKind::Headshot, true) => "150x150",
            (ThumbKind::Avatar, _) => "420x420",
            (ThumbKind::GameIcon, false) => "50x50",
            (ThumbKind::GameIcon, true) => "256x256",
            (ThumbKind::GameThumbnail, _) => "768x432",
            (ThumbKind::Asset | ThumbKind::Bundle | ThumbKind::Outfit, false) => "150x150",
            (ThumbKind::Asset | ThumbKind::Bundle | ThumbKind::Outfit, true) => "420x420",
            (ThumbKind::GroupIcon, false) => "150x150",
            (ThumbKind::GroupIcon, true) => "420x420",
            (ThumbKind::BadgeIcon | ThumbKind::GamePass, _) => "150x150",
        }
    }
}

/// One image: kind, target id and size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ThumbKey {
    pub kind: ThumbKind,
    pub id: u64,
    pub large: bool,
}

impl ThumbKey {
    pub fn small(kind: ThumbKind, id: u64) -> ThumbKey {
        ThumbKey {
            kind,
            id,
            large: false,
        }
    }
    pub fn large(kind: ThumbKind, id: u64) -> ThumbKey {
        ThumbKey {
            kind,
            id,
            large: true,
        }
    }
    /// The batch request id, also the key of its cached URL.
    pub fn request_id(&self) -> String {
        format!(
            "{}:{}:{}",
            self.kind.api_type(),
            self.id,
            self.kind.size(self.large)
        )
    }
}

/// An encoded image (WebP, PNG or JPEG) as Roblox sent it.
#[derive(Debug, PartialEq, Eq)]
pub struct Image {
    pub mime: &'static str,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug)]
pub enum ThumbState {
    Loading,
    Ready(Arc<Image>),
    /// Blocked, moderated, or failed; the UI shows a placeholder.
    Unavailable,
}

/// Look up URLs for keys. Returns `(key, Some(url))` for completed ones, `(key, None)` for
/// ones that are still pending, and leaves out the unavailable ones.
pub async fn lookup(
    client: &Client,
    keys: &[ThumbKey],
) -> Result<Vec<(ThumbKey, Option<String>, bool)>, Error> {
    // Keys whose kind uses one size for both (game thumbnails) share a request id. Ask once
    // and answer every key with it; otherwise all but the first would look unavailable.
    let mut seen = std::collections::HashSet::new();
    let body: Vec<thumbnails::undocumented::BatchRequest> = keys
        .iter()
        .filter(|k| seen.insert(k.request_id()))
        .map(|k| thumbnails::undocumented::BatchRequest {
            request_id: Some(k.request_id()),
            target_id: Some(k.id as i64),
            type_: Some(k.kind.api_type().into()),
            size: Some(k.kind.size(k.large).into()),
            format: Some("webp".into()),
            is_circular: Some(false),
        })
        .collect();
    let resp = thumbnails::undocumented::post_batch(client, &body).await?;
    let mut out = Vec::new();
    for item in resp.data.unwrap_or_default() {
        for k in keys.iter().filter(|k| Some(k.request_id()) == item.request_id) {
            match item.state.as_deref() {
                Some("Completed") => out.push((*k, item.image_url.clone().filter(|u| !u.is_empty()), false)),
                Some("Pending") | Some("InReview") | Some("TemporarilyUnavailable") => {
                    out.push((*k, None, false))
                }
                _ => out.push((*k, None, true)),
            }
        }
    }
    Ok(out)
}

/// Bytes on disk keyed by URL hash. Reads bump a file's modified time, so pruning drops the
/// least recently used images first.
pub struct DiskCache {
    dir: PathBuf,
}

impl DiskCache {
    pub fn new(dir: &Path) -> DiskCache {
        let _ = std::fs::create_dir_all(dir);
        DiskCache {
            dir: dir.to_path_buf(),
        }
    }
    fn path(&self, url: &str) -> PathBuf {
        let h = Sha256::digest(url.as_bytes());
        let hex: String = h.iter().take(16).map(|b| format!("{b:02x}")).collect();
        self.dir.join(hex)
    }
    pub fn has(&self, url: &str) -> bool {
        self.path(url).is_file()
    }
    pub fn get(&self, url: &str) -> Option<Vec<u8>> {
        let p = self.path(url);
        let bytes = std::fs::read(&p).ok()?;
        if let Ok(f) = std::fs::File::options().write(true).open(&p) {
            let _ = f.set_modified(std::time::SystemTime::now());
        }
        Some(bytes)
    }
    /// Bytes of every cached image.
    pub fn size_bytes(&self) -> u64 {
        std::fs::read_dir(&self.dir)
            .map(|d| {
                d.flatten()
                    .filter_map(|e| e.metadata().ok())
                    .map(|m| m.len())
                    .sum()
            })
            .unwrap_or(0)
    }
    /// Delete every cached image.
    pub fn clear(&self) {
        if let Ok(d) = std::fs::read_dir(&self.dir) {
            for e in d.flatten() {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
    /// Delete images not used since `cutoff`, then the least recently used ones until the rest
    /// fit in `max_bytes`. Returns how many files went.
    pub fn prune(&self, cutoff: std::time::SystemTime, max_bytes: u64) -> usize {
        let Ok(d) = std::fs::read_dir(&self.dir) else {
            return 0;
        };
        let mut files: Vec<(std::time::SystemTime, u64, PathBuf)> = d
            .flatten()
            .filter_map(|e| {
                let m = e.metadata().ok()?;
                m.is_file()
                    .then(|| (m.modified().unwrap_or(std::time::UNIX_EPOCH), m.len(), e.path()))
            })
            .collect();
        // Newest first; keep from the front while there is room.
        files.sort_by_key(|f| std::cmp::Reverse(f.0));
        let mut total = 0;
        let mut removed = 0;
        for (modified, len, path) in files {
            total += len;
            if modified < cutoff || total > max_bytes {
                if std::fs::remove_file(path).is_ok() {
                    removed += 1;
                }
                total -= len;
            }
        }
        removed
    }
    pub fn put(&self, url: &str, bytes: &[u8]) {
        let p = self.path(url);
        let tmp = p.with_extension("tmp");
        if std::fs::write(&tmp, bytes).is_ok() {
            let _ = std::fs::rename(tmp, p);
        }
    }
}

/// Which loaded images stay in memory: the most recently loaded ones up to a byte budget.
/// Evicted ones load again from disk when asked for.
#[derive(Default)]
pub(crate) struct Budget {
    seq: u64,
    by_key: std::collections::HashMap<ThumbKey, (u64, usize)>,
    order: std::collections::BTreeMap<u64, ThumbKey>,
    total: usize,
}

impl Budget {
    /// Note a loaded image; returns the keys to drop to stay within `max_bytes`.
    pub(crate) fn insert(&mut self, k: ThumbKey, bytes: usize, max_bytes: usize) -> Vec<ThumbKey> {
        self.remove(&k);
        self.seq += 1;
        self.by_key.insert(k, (self.seq, bytes));
        self.order.insert(self.seq, k);
        self.total += bytes;
        let mut out = Vec::new();
        // Never drop the one just loaded; it is about to be shown.
        while self.total > max_bytes && self.order.len() > 1 {
            let Some((_, old)) = self.order.pop_first() else {
                break;
            };
            if let Some((_, b)) = self.by_key.remove(&old) {
                self.total -= b;
            }
            out.push(old);
        }
        out
    }

    pub(crate) fn remove(&mut self, k: &ThumbKey) {
        if let Some((seq, b)) = self.by_key.remove(k) {
            self.order.remove(&seq);
            self.total -= b;
        }
    }

    pub(crate) fn clear(&mut self) {
        *self = Budget::default();
    }
}

/// Check that `bytes` are an image the webview can show and say which kind.
pub fn sniff(bytes: Vec<u8>) -> Option<Image> {
    let mime = match image::guess_format(&bytes).ok()? {
        image::ImageFormat::WebP => "image/webp",
        image::ImageFormat::Png => "image/png",
        image::ImageFormat::Jpeg => "image/jpeg",
        _ => return None,
    };
    Some(Image { mime, bytes })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disk_cache_round_trip() {
        let d = tempfile::tempdir().unwrap();
        let c = DiskCache::new(d.path());
        assert!(c.get("https://tr.rbxcdn.com/a").is_none());
        c.put("https://tr.rbxcdn.com/a", b"xyz");
        assert_eq!(c.get("https://tr.rbxcdn.com/a").unwrap(), b"xyz");
    }

    #[test]
    fn prune_drops_old_then_least_recently_used() {
        use std::time::{Duration, SystemTime};
        let d = tempfile::tempdir().unwrap();
        let c = DiskCache::new(d.path());
        let t = SystemTime::now();
        let age = |url: &str, secs: u64| {
            let f = std::fs::File::options().write(true).open(c.path(url)).unwrap();
            f.set_modified(t - Duration::from_secs(secs)).unwrap();
        };
        for (url, secs) in [("a", 100), ("b", 200), ("c", 300), ("old", 10_000)] {
            c.put(url, &[0; 10]);
            age(url, secs);
        }
        // Reading "c" makes it the most recently used.
        assert!(c.get("c").is_some());
        assert_eq!(c.prune(t - Duration::from_secs(1000), 20), 2);
        assert!(c.get("old").is_none());
        assert!(c.get("b").is_none());
        assert!(c.get("a").is_some() && c.get("c").is_some());
        assert_eq!(c.size_bytes(), 20);
    }

    #[test]
    fn budget_drops_oldest_loaded_first() {
        let k = |id| ThumbKey::small(ThumbKind::Headshot, id);
        let mut b = Budget::default();
        assert!(b.insert(k(1), 10, 25).is_empty());
        assert!(b.insert(k(2), 10, 25).is_empty());
        // Loading 1 again makes it the newest.
        assert!(b.insert(k(1), 10, 25).is_empty());
        assert_eq!(b.insert(k(3), 10, 25), vec![k(2)]);
        b.remove(&k(1));
        assert!(b.insert(k(4), 10, 25).is_empty());
        // One image over budget on its own is still kept.
        assert_eq!(b.insert(k(5), 100, 25), vec![k(3), k(4)]);
    }

    #[test]
    fn sniffs_images() {
        let mut buf = std::io::Cursor::new(Vec::new());
        image::RgbaImage::from_pixel(2, 3, image::Rgba([1, 2, 3, 255]))
            .write_to(&mut buf, image::ImageFormat::Png)
            .unwrap();
        assert_eq!(sniff(buf.into_inner()).unwrap().mime, "image/png");
        assert!(sniff(b"<html>".to_vec()).is_none());
    }
}
