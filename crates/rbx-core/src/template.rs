//! Clothing templates: the image behind a classic shirt, pants or T-shirt, saved to the
//! downloads folder. The clothing asset is a small model whose `ShirtTemplate`,
//! `PantsTemplate` or `Graphic` property points at the image asset.
//!
//! A masked template keeps only the parts of the image Roblox maps onto the body and clears the
//! rest, which drops watermarks and notes drawn outside the regions.

use std::path::{Path, PathBuf};

use image::{ImageFormat, RgbaImage};
use rbx_api::generated::assetdelivery;
use rbx_api::{AccountId, Client, Error};

use crate::Core;

pub const TSHIRT: i32 = 2;
pub const SHIRT: i32 = 11;
pub const PANTS: i32 = 12;

/// Whether an asset type has an image to download.
pub fn has_template(asset_type: i32) -> bool {
    matches!(asset_type, TSHIRT | SHIRT | PANTS)
}

/// Whether an asset type uses the shirt and pants template layout, so it can be masked.
pub fn can_mask(asset_type: i32) -> bool {
    matches!(asset_type, SHIRT | PANTS)
}

/// The size of a Roblox clothing template.
pub const SIZE: (u32, u32) = (585, 559);

/// The parts of a template Roblox maps onto the body: `(x, y, w, h)` at [`SIZE`].
pub const REGIONS: [(u32, u32, u32, u32); 18] = [
    // Torso: up, right, front, left, back, down
    (231, 8, 128, 64),
    (165, 74, 64, 128),
    (231, 74, 128, 128),
    (361, 74, 64, 128),
    (427, 74, 128, 128),
    (231, 204, 128, 64),
    // Right arm or leg: up, left, back, right, front, down
    (217, 289, 64, 64),
    (19, 355, 64, 128),
    (85, 355, 64, 128),
    (151, 355, 64, 128),
    (217, 355, 64, 128),
    (217, 485, 64, 64),
    // Left arm or leg: up, left, back, right, front, down
    (308, 289, 64, 64),
    (374, 355, 64, 128),
    (440, 355, 64, 128),
    (506, 355, 64, 128),
    (308, 355, 64, 128),
    (308, 485, 64, 64),
];

/// Keeps only the template regions of `img`, scaled to its size, and clears everything else.
pub fn mask(img: &RgbaImage) -> RgbaImage {
    let (w, h) = img.dimensions();
    let sx = w as f64 / SIZE.0 as f64;
    let sy = h as f64 / SIZE.1 as f64;
    let mut out = RgbaImage::new(w, h);
    for (x, y, rw, rh) in REGIONS {
        let x0 = ((x as f64 * sx).round() as u32).min(w);
        let y0 = ((y as f64 * sy).round() as u32).min(h);
        let x1 = (((x + rw) as f64 * sx).round() as u32).min(w);
        let y1 = (((y + rh) as f64 * sy).round() as u32).min(h);
        for py in y0..y1 {
            for px in x0..x1 {
                out.put_pixel(px, py, *img.get_pixel(px, py));
            }
        }
    }
    out
}

/// The image asset id a clothing model points at: the first `?id=` or `rbxassetid://` in its
/// XML or binary form.
pub fn texture_id(model: &[u8]) -> Option<u64> {
    let text = String::from_utf8_lossy(model);
    let lower = text.to_ascii_lowercase();
    ["rbxassetid://", "?id=", "&id="]
        .iter()
        .filter_map(|p| lower.find(p).map(|i| i + p.len()))
        .min()
        .and_then(|start| {
            let digits: String = lower[start..].chars().take_while(char::is_ascii_digit).collect();
            digits.parse().ok()
        })
}

/// A file name from the item's name: no path separators or other characters file systems
/// refuse, and not too long.
pub fn file_stem(name: &str, id: u64, masked: bool) -> String {
    let clean: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || matches!(c, ' ' | '-' | '_' | '.' | '(' | ')') {
                c
            } else {
                '_'
            }
        })
        .take(80)
        .collect();
    let clean = clean.trim().trim_matches('.').trim();
    let base = if clean.is_empty() {
        id.to_string()
    } else {
        format!("{clean} ({id})")
    };
    if masked { format!("{base} masked") } else { base }
}

/// `dir/stem.ext`, or `dir/stem 2.ext` and so on when that file exists.
pub(crate) fn free_path(dir: &Path, stem: &str, ext: &str) -> PathBuf {
    let mut p = dir.join(format!("{stem}.{ext}"));
    let mut n = 2;
    while p.exists() {
        p = dir.join(format!("{stem} {n}.{ext}"));
        n += 1;
    }
    p
}

/// Fetches an asset's file through asset delivery. Returns the bytes and the asset type.
async fn asset_file(c: &Client, id: u64) -> Result<(Vec<u8>, i32), Error> {
    let r = assetdelivery::v2::get_asset_id_by_asset_id(c, id as i64, &Default::default()).await?;
    if let Some(e) = r.errors.as_ref().and_then(|e| e.first()) {
        return Err(Error::Message(
            e.message
                .clone()
                .unwrap_or_else(|| "Roblox wouldn't hand over this asset.".into()),
        ));
    }
    let url = r
        .locations
        .unwrap_or_default()
        .into_iter()
        .find_map(|l| l.location)
        .ok_or_else(|| Error::Message("Roblox didn't say where this asset is.".into()))?;
    Ok((c.fetch_cdn(&url).await?, r.asset_type_id.unwrap_or(0)))
}

impl Core {
    /// Saves the image of a shirt, pants or T-shirt to the downloads folder, masked or not.
    /// `name` names the file; the id is used when it is empty.
    pub fn download_template(&self, account: AccountId, asset_id: u64, name: &str, masked: bool) {
        let name = name.to_owned();
        self.inner
            .mutate(account, "download the template", "", &[], move |f| async move {
                let (model, kind) = asset_file(&f.client, asset_id).await?;
                if !has_template(kind) {
                    return Err(Error::Message(
                        "Only shirts, pants and T-shirts have templates.".into(),
                    ));
                }
                if masked && !can_mask(kind) {
                    return Err(Error::Message("Only shirts and pants can be masked.".into()));
                }
                let tex = texture_id(&model)
                    .ok_or_else(|| Error::Message("This item has no template image.".into()))?;
                let (bytes, _) = asset_file(&f.client, tex).await?;
                let format = image::guess_format(&bytes)
                    .map_err(|_| Error::Message("The template isn't an image.".into()))?;
                let (bytes, ext) = if masked {
                    let img = image::load_from_memory_with_format(&bytes, format)
                        .map_err(|e| Error::Message(format!("Can't read the template: {e}")))?
                        .to_rgba8();
                    let mut out = std::io::Cursor::new(Vec::new());
                    mask(&img)
                        .write_to(&mut out, ImageFormat::Png)
                        .map_err(|e| Error::Message(format!("Can't encode the template: {e}")))?;
                    (out.into_inner(), "png")
                } else {
                    let ext = format.extensions_str().first().copied().unwrap_or("png");
                    (bytes, ext)
                };
                let dir = f.inner.paths.downloads.clone();
                let path = free_path(&dir, &file_stem(&name, asset_id, masked), ext);
                std::fs::create_dir_all(&dir)
                    .and_then(|()| std::fs::write(&path, bytes))
                    .map_err(|e| Error::Message(format!("Can't save to {}: {e}", dir.display())))?;
                f.inner
                    .toast(Some(account), format!("Saved {}", path.display()), false);
                Ok(())
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_texture_ids() {
        let xml = br#"<roblox><Item class="Shirt"><Properties>
            <Content name="ShirtTemplate"><url>http://www.roblox.com/asset/?id=1234567</url></Content>
            <string name="Name">Shirt</string></Properties></Item></roblox>"#;
        assert_eq!(texture_id(xml), Some(1234567));
        assert_eq!(texture_id(b"<url>rbxassetid://42</url>"), Some(42));
        assert_eq!(
            texture_id(b"<url>https://www.roblox.com/asset?ID=9</url>"),
            Some(9)
        );
        assert_eq!(texture_id(b"<roblox></roblox>"), None);
        assert_eq!(texture_id(b"?id=x"), None);
    }

    #[test]
    fn mask_keeps_regions_only() {
        let img = RgbaImage::from_pixel(SIZE.0, SIZE.1, image::Rgba([255, 0, 0, 255]));
        let m = mask(&img);
        // torso front, a right arm side, a left arm bottom
        assert_eq!(m.get_pixel(300, 140).0, [255, 0, 0, 255]);
        assert_eq!(m.get_pixel(20, 356).0, [255, 0, 0, 255]);
        assert_eq!(m.get_pixel(371, 548).0, [255, 0, 0, 255]);
        // corners and the gap between the torso and limbs
        assert_eq!(m.get_pixel(0, 0).0, [0; 4]);
        assert_eq!(m.get_pixel(584, 558).0, [0; 4]);
        assert_eq!(m.get_pixel(300, 280).0, [0; 4]);
    }

    #[test]
    fn mask_scales_with_the_image() {
        let img = RgbaImage::from_pixel(SIZE.0 * 2, SIZE.1 * 2, image::Rgba([0, 0, 255, 255]));
        let m = mask(&img);
        assert_eq!(m.dimensions(), (SIZE.0 * 2, SIZE.1 * 2));
        assert_eq!(m.get_pixel(600, 280).0, [0, 0, 255, 255]);
        assert_eq!(m.get_pixel(600, 560).0, [0; 4]);
    }

    #[test]
    fn file_names() {
        assert_eq!(file_stem("Cool Shirt", 5, false), "Cool Shirt (5)");
        assert_eq!(file_stem("a/b:c", 5, true), "a_b_c (5) masked");
        assert_eq!(file_stem("..", 5, false), "5");
        assert_eq!(file_stem("", 7, true), "7 masked");
    }
}
