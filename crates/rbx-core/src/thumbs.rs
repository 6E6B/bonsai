//! Thumbnails: batched lookups through the thumbnails batch endpoint, `Pending` polling, and a
//! disk cache of image bytes keyed by URL hash. Decoded RGBA goes to the store for the UI.

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
    fn request_id(&self) -> String {
        format!(
            "{}:{}:{}",
            self.kind.api_type(),
            self.id,
            self.kind.size(self.large)
        )
    }
}

/// Decoded RGBA8.
#[derive(Debug, PartialEq, Eq)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
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

/// Bytes on disk keyed by URL hash.
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
    pub fn get(&self, url: &str) -> Option<Vec<u8>> {
        std::fs::read(self.path(url)).ok()
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
    pub fn put(&self, url: &str, bytes: &[u8]) {
        let p = self.path(url);
        let tmp = p.with_extension("tmp");
        if std::fs::write(&tmp, bytes).is_ok() {
            let _ = std::fs::rename(tmp, p);
        }
    }
}

pub fn decode(bytes: &[u8]) -> Option<Image> {
    let img = image::load_from_memory(bytes).ok()?.to_rgba8();
    Some(Image {
        width: img.width(),
        height: img.height(),
        rgba: img.into_raw(),
    })
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
    fn decodes_png() {
        let mut buf = std::io::Cursor::new(Vec::new());
        image::RgbaImage::from_pixel(2, 3, image::Rgba([1, 2, 3, 255]))
            .write_to(&mut buf, image::ImageFormat::Png)
            .unwrap();
        let i = decode(buf.get_ref()).unwrap();
        assert_eq!((i.width, i.height, i.rgba.len()), (2, 3, 24));
    }
}
