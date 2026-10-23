//! User-requested exports of rendered thumbnails. Original textures/models use asset delivery
//! separately. Export requests never replace the small images used by the UI.

use std::io::Cursor;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use bonsai_api::generated::thumbnails;
use bonsai_api::{AccountId, Error};

use crate::Core;
use crate::cache::Kind;
use crate::core::thumb_client;
use crate::thumbs::{Image, ThumbKey, ThumbState, sniff};

pub struct ThumbnailImage {
    pub image: Arc<Image>,
    /// Public Roblox CDN URL, never the local thumb:// URL.
    pub url: Option<String>,
}

impl Image {
    pub fn extension(&self) -> &'static str {
        match self.mime {
            "image/webp" => "webp",
            "image/jpeg" => "jpg",
            _ => "png",
        }
    }

    /// Bounded decoding also checks truncated/corrupt cached images before saving or copying.
    pub fn rgba(&self) -> Result<image::RgbaImage, String> {
        let mut reader = image::ImageReader::new(Cursor::new(&self.bytes))
            .with_guessed_format()
            .map_err(|e| format!("Could not read image: {e}"))?;
        let mut limits = image::Limits::default();
        limits.max_image_width = Some(4096);
        limits.max_image_height = Some(4096);
        limits.max_alloc = Some(64 * 1024 * 1024);
        reader.limits(limits);
        reader
            .decode()
            .map(|i| i.into_rgba8())
            .map_err(|e| format!("Could not decode image: {e}"))
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        std::fs::write(path, &self.bytes).map_err(|e| format!("Could not save image: {e}"))
    }
}

impl Core {
    /// Supported renders in descending resolution. If Roblox cannot
    /// render one, try the next size. All network traffic uses the existing API client/limiter;
    /// public URLs and image bytes share the thumb:// disk cache.
    pub async fn thumbnail_image(&self, key: ThumbKey) -> Result<ThumbnailImage, String> {
        let sizes = key.kind.export_sizes();
        let client = thumb_client(&self.inner);
        let result = async {
            for &size in sizes {
                let request_id = key.request_id_for(size);
                let cached =
                    self.inner
                        .cache
                        .get::<String>(Kind::ThumbUrl, AccountId::ANONYMOUS, &request_id);
                if let Some(cached) = &cached
                    && cached.is_fresh(key.kind.url_age_secs())
                    && let Some(image) = self.cached_export_image(&cached.value)
                {
                    return Ok(image);
                }
                // Pending/InReview renders get a short chance to finish before falling back.
                for attempt in 0..3 {
                    let request = vec![thumbnails::undocumented::BatchRequest {
                        request_id: Some(request_id.clone()),
                        target_id: Some(key.id as i64),
                        type_: Some(key.kind.api_type().into()),
                        size: Some(size.into()),
                        format: Some("webp".into()),
                        is_circular: Some(false),
                    }];
                    let mut response = thumbnails::undocumented::post_batch(&client, &request).await;
                    if matches!(response, Err(Error::Unauthorized)) && client.signed_in() {
                        response = thumbnails::undocumented::post_batch(&self.inner.anon, &request).await;
                    }
                    let Ok(response) = response else { break };
                    let Some(item) = response
                        .data
                        .unwrap_or_default()
                        .into_iter()
                        .find(|i| i.request_id.as_deref() == Some(&request_id))
                    else {
                        break;
                    };
                    match item.state.as_deref() {
                        Some("Completed") => {
                            let Some(url) = item.image_url.filter(|u| !u.is_empty()) else {
                                break;
                            };
                            let image = if let Some(image) = self.cached_export_image(&url) {
                                Some(image)
                            } else if let Ok(bytes) = self.inner.anon.fetch_cdn(&url).await {
                                sniff(bytes).filter(|i| i.rgba().is_ok()).map(|image| {
                                    self.inner.disk.put(&url, &image.bytes);
                                    ThumbnailImage {
                                        image: Arc::new(image),
                                        url: Some(url.clone()),
                                    }
                                })
                            } else {
                                None
                            };
                            if let Some(image) = image {
                                self.inner
                                    .cache
                                    .put(Kind::ThumbUrl, AccountId::ANONYMOUS, &request_id, &url);
                                return Ok(image);
                            }
                            break;
                        }
                        Some("Pending" | "InReview" | "TemporarilyUnavailable") if attempt < 2 => {
                            tokio::time::sleep(Duration::from_millis(500)).await;
                        }
                        _ => break,
                    }
                }
                // A URL may still work when the lookup service is down.
                if let Some(cached) = cached
                    && let Ok(bytes) = self.inner.anon.fetch_cdn(&cached.value).await
                    && let Some(image) = sniff(bytes).filter(|i| i.rgba().is_ok())
                {
                    self.inner.disk.put(&cached.value, &image.bytes);
                    return Ok(ThumbnailImage {
                        image: Arc::new(image),
                        url: Some(cached.value),
                    });
                }
            }
            Err("This thumbnail is unavailable.".to_owned())
        };
        // An offline/rate-limited export must still be able to use what is already displayed.
        let fetched = tokio::time::timeout(Duration::from_secs(12), result).await;
        if let Ok(Ok(image)) = fetched {
            return Ok(image);
        }
        for &size in sizes {
            let cached = self.inner.cache.get::<String>(
                Kind::ThumbUrl,
                AccountId::ANONYMOUS,
                &key.request_id_for(size),
            );
            if let Some(cached) = cached
                && let Some(image) = self.cached_export_image(&cached.value)
            {
                return Ok(image);
            }
        }
        for key in [
            ThumbKey::large(key.kind, key.id),
            ThumbKey::small(key.kind, key.id),
        ] {
            let cached =
                self.inner
                    .cache
                    .get::<String>(Kind::ThumbUrl, AccountId::ANONYMOUS, &key.request_id());
            if let Some(cached) = &cached
                && let Some(image) = self.cached_export_image(&cached.value)
            {
                return Ok(image);
            }
            let image = match self.store().thumbs.get(&key) {
                Some(ThumbState::Ready(image)) => Some(image.clone()),
                _ => None,
            };
            if let Some(image) = image.filter(|i| i.rgba().is_ok()) {
                return Ok(ThumbnailImage {
                    image,
                    url: cached.map(|c| c.value),
                });
            }
        }
        Err("This thumbnail is unavailable. Try again when connected.".into())
    }

    fn cached_export_image(&self, url: &str) -> Option<ThumbnailImage> {
        let image = sniff(self.inner.disk.get(url)?)?;
        image.rgba().ok()?;
        Some(ThumbnailImage {
            image: Arc::new(image),
            url: Some(url.to_owned()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_encoded_bytes_and_decodes_clipboard_pixels() {
        let expected = image::RgbaImage::from_pixel(2, 3, image::Rgba([10, 20, 30, 40]));
        let mut out = Cursor::new(Vec::new());
        expected.write_to(&mut out, image::ImageFormat::Png).unwrap();
        let bytes = out.into_inner();
        let image = sniff(bytes.clone()).unwrap();
        assert_eq!(image.extension(), "png");
        assert_eq!(image.rgba().unwrap(), expected);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("thumbnail.png");
        image.save(&path).unwrap();
        assert_eq!(std::fs::read(path).unwrap(), bytes);
        // A valid file header alone must not make a corrupt cached render exportable.
        let truncated = Image {
            mime: "image/png",
            bytes: b"\x89PNG\r\n\x1a\n".to_vec(),
        };
        assert!(truncated.rgba().is_err());
    }
}
