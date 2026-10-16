//! Google Fonts for themes. A theme names a family (`"font": {"google": "Space Grotesk"}`), and
//! this module downloads its files to `Paths::fonts_dir()` so the webview can load them from
//! disk over `font://`. The webview itself never reaches the network.
//!
//! The catalog (every family with its weights) is fetched at most weekly and cached like any
//! shared resource. A family's files are fetched once: its CSS from `fonts.googleapis.com` names
//! one TrueType file per weight on `fonts.gstatic.com`, and each is written as
//! `<slug>/<weight>.ttf`. Requests go out with the anonymous client, and the transport never
//! sends Roblox credentials to a non-Roblox host anyway.

use std::path::{Path, PathBuf};

use bonsai_api::Error;
use bonsai_api::generated::{fonts_google_com, fonts_googleapis_com};
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::res::{Key, Res};

/// The catalog changes a few times a week; picking from a week-old one is fine.
const CATALOG_AGE: i64 = 7 * 24 * 3600;
/// Font files rarely change. Refetching monthly picks up fixes.
const FILES_AGE: i64 = 30 * 24 * 3600;
/// The weights the app uses. A family without one gets its nearest weight instead.
const WANT: [u16; 4] = [400, 500, 600, 700];
/// Larger than any static Google font file (CJK families are about 5 MB a weight).
const MAX_FILE: usize = 16 << 20;
const MAX_FAMILY: usize = 64;

/// One family in the catalog.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FontInfo {
    pub family: String,
    /// Sans Serif, Serif, Display, Handwriting or Monospace.
    pub category: String,
    /// Upright weights it comes in, ascending.
    pub weights: Vec<u16>,
}

impl FontInfo {
    pub fn monospace(&self) -> bool {
        self.category == "Monospace"
    }
}

/// A downloaded family: one file per weight, relative to `Paths::fonts_dir()`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FontFiles {
    pub family: String,
    pub files: Vec<FontFile>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FontFile {
    pub weight: u16,
    /// `<slug>/<weight>.ttf`
    pub path: String,
}

/// A family name as Google writes it: letters, digits and single spaces.
pub fn valid_family(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= MAX_FAMILY
        && s.trim() == s
        && !s.contains("  ")
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == ' ')
}

/// The folder name for a family: `Space Grotesk` → `space-grotesk`.
pub fn slug(family: &str) -> String {
    family.to_ascii_lowercase().replace(' ', "-")
}

/// Whether `rel` is a path this module writes (`<slug>/<weight>.ttf`), so the `font://`
/// protocol can serve it without walking out of the folder.
pub fn valid_file(rel: &str) -> bool {
    let Some((dir, file)) = rel.split_once('/') else {
        return false;
    };
    let Some(weight) = file.strip_suffix(".ttf") else {
        return false;
    };
    !dir.is_empty()
        && dir.len() <= MAX_FAMILY
        && !dir.starts_with('-')
        && dir
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && (1..=4).contains(&weight.len())
        && weight.chars().all(|c| c.is_ascii_digit())
}

/// For each weight the app uses, the family's nearest one (ties go heavier), without repeats.
fn pick_weights(have: &[u16]) -> Vec<u16> {
    let mut out: Vec<u16> = WANT
        .iter()
        .filter_map(|&w| {
            have.iter()
                .copied()
                .min_by_key(|&h| (h.abs_diff(w), std::cmp::Reverse(h)))
        })
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

fn catalog(r: fonts_google_com::undocumented::FontCatalog) -> Vec<FontInfo> {
    let mut rows: Vec<(i64, FontInfo)> = r
        .family_metadata_list
        .unwrap_or_default()
        .into_iter()
        .filter_map(|f| {
            let family = f.family.filter(|n| valid_family(n))?;
            let mut weights: Vec<u16> = f
                .fonts
                .unwrap_or_default()
                .keys()
                .filter_map(|k| k.parse().ok())
                .filter(|w| (1..=1000).contains(w))
                .collect();
            if weights.is_empty() {
                return None;
            }
            weights.sort_unstable();
            let info = FontInfo {
                family,
                category: f.category.unwrap_or_default(),
                weights,
            };
            Some((f.popularity.unwrap_or(i64::MAX), info))
        })
        .collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.family.cmp(&b.1.family)));
    rows.into_iter().map(|(_, f)| f).collect()
}

/// Each upright `@font-face`'s weight and file URL. Only `fonts.gstatic.com` URLs are kept.
fn parse_css(css: &str) -> Vec<(u16, String)> {
    let mut out = Vec::new();
    for block in css.split("@font-face").skip(1) {
        let block = block.split('}').next().unwrap_or("");
        let prop = |name: &str| {
            block.split(';').find_map(|d| {
                let (k, v) = d.split_once(':')?;
                (k.trim() == name).then(|| v.trim())
            })
        };
        if prop("font-style").is_some_and(|s| s != "normal") {
            continue;
        }
        let Some(weight) = prop("font-weight").and_then(|w| w.parse::<u16>().ok()) else {
            continue;
        };
        let Some(url) = block
            .split("url(")
            .nth(1)
            .and_then(|r| r.split(')').next())
            .map(|u| u.trim_matches(['\'', '"']))
        else {
            continue;
        };
        if url.starts_with("https://fonts.gstatic.com/") && !out.iter().any(|(w, _)| *w == weight) {
            out.push((weight, url.to_owned()));
        }
    }
    out
}

/// TrueType, OpenType or a TrueType collection.
fn is_font(b: &[u8]) -> bool {
    matches!(b.get(..4), Some([0, 1, 0, 0] | b"OTTO" | b"true" | b"ttcf"))
}

fn save(dir: &Path, rel: &str, bytes: &[u8]) -> std::io::Result<()> {
    let path = dir.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(tmp, path)
}

async fn download(f: &crate::res::Fetch, dir: PathBuf, info: FontInfo) -> Result<FontFiles, Error> {
    let weights = pick_weights(&info.weights);
    let list: Vec<String> = weights.iter().map(u16::to_string).collect();
    let params = fonts_googleapis_com::undocumented::GetFontCssParams {
        family: format!("{}:wght@{}", info.family, list.join(";")),
    };
    let r = f
        .client
        .send(fonts_googleapis_com::undocumented::get_font_css_request(&params)?)
        .await?;
    let faces = parse_css(&String::from_utf8_lossy(&r.body));
    if faces.is_empty() {
        return Err(Error::Message("Google Fonts sent no files for this font.".into()));
    }
    let mut files = Vec::new();
    for (weight, url) in faces {
        let bytes = f.client.fetch_font(&url).await?;
        if bytes.len() > MAX_FILE || !is_font(&bytes) {
            return Err(Error::Message(
                "Google Fonts sent a file that isn't a font.".into(),
            ));
        }
        let path = format!("{}/{weight}.ttf", slug(&info.family));
        save(&dir, &path, &bytes).map_err(|_| Error::Message("Couldn't save the font.".into()))?;
        files.push(FontFile { weight, path });
    }
    Ok(FontFiles {
        family: info.family,
        files,
    })
}

impl Core {
    /// Every Google Fonts family, most popular first.
    pub fn font_catalog(&self) -> Res<Vec<FontInfo>> {
        self.inner
            .res(Key::shared("font_catalog", ""), CATALOG_AGE, |f| async move {
                let r = fonts_google_com::undocumented::get_font_catalog(&f.client).await?;
                Ok(catalog(r))
            })
    }

    /// A family from the catalog by name, ignoring case. `None` while the catalog loads too.
    pub fn font_info(&self, family: &str) -> Option<FontInfo> {
        let c = self.font_catalog().data?;
        c.iter().find(|f| f.family.eq_ignore_ascii_case(family)).cloned()
    }

    /// A family's files on disk, downloading them on first ask. Loading while the catalog loads.
    pub fn google_font(&self, family: &str) -> Res<FontFiles> {
        let cat = self.font_catalog();
        let Some(list) = cat.data else {
            return Res {
                data: None,
                loading: cat.loading,
                error: cat.error,
                fetched_at: 0,
            };
        };
        let Some(info) = list.iter().find(|f| f.family == family).cloned() else {
            return Res {
                error: Some(format!("Google Fonts has no font named {family}.")),
                ..Res::default()
            };
        };
        let dir = self.paths().fonts_dir();
        let key = Key::shared("google_font", family);
        let r = self.inner.res(key.clone(), FILES_AGE, {
            let dir = dir.clone();
            let info = info.clone();
            move |f| async move { download(&f, dir, info).await }
        });
        // The saved row outlives the files when the cache folder is cleared: fetch again.
        let missing = r
            .data
            .as_ref()
            .is_some_and(|d| d.files.iter().any(|x| !dir.join(&x.path).is_file()));
        if missing && !r.loading {
            return self
                .inner
                .res(key, 0, move |f| async move { download(&f, dir, info).await });
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn family_names_and_files() {
        assert!(valid_family("Space Grotesk"));
        assert!(valid_family("M PLUS 1p"));
        assert!(!valid_family(""));
        assert!(!valid_family(" Inter"));
        assert!(!valid_family("A  B"));
        assert!(!valid_family("Inter'; x"));
        assert_eq!(slug("Space Grotesk"), "space-grotesk");
        assert!(valid_file("space-grotesk/400.ttf"));
        assert!(!valid_file("../400.ttf"));
        assert!(!valid_file("a/b/400.ttf"));
        assert!(!valid_file("a/400.otf"));
        assert!(!valid_file("a/x.ttf"));
    }

    #[test]
    fn picks_nearest_weights() {
        assert_eq!(
            pick_weights(&[100, 400, 500, 600, 700, 900]),
            vec![400, 500, 600, 700]
        );
        assert_eq!(pick_weights(&[400]), vec![400]);
        assert_eq!(pick_weights(&[300, 500, 700]), vec![500, 700]);
        assert_eq!(pick_weights(&[300]), vec![300]);
        assert_eq!(pick_weights(&[400, 700]), vec![400, 700]);
    }

    #[test]
    fn reads_css_and_skips_italics_and_other_hosts() {
        let css = "@font-face {\n  font-family: 'Inter';\n  font-style: normal;\n  font-weight: 400;\n  src: url(https://fonts.gstatic.com/s/inter/a.ttf) format('truetype');\n}\n\
                   @font-face {\n  font-style: italic;\n  font-weight: 400;\n  src: url(https://fonts.gstatic.com/s/inter/i.ttf);\n}\n\
                   @font-face {\n  font-style: normal;\n  font-weight: 700;\n  src: url(https://evil.example/b.ttf);\n}\n\
                   @font-face {\n  font-style: normal;\n  font-weight: 600;\n  src: url('https://fonts.gstatic.com/s/inter/c.ttf');\n}";
        assert_eq!(
            parse_css(css),
            vec![
                (400, "https://fonts.gstatic.com/s/inter/a.ttf".to_owned()),
                (600, "https://fonts.gstatic.com/s/inter/c.ttf".to_owned()),
            ]
        );
    }

    #[test]
    fn catalog_sorts_by_popularity_and_drops_junk() {
        let r: fonts_google_com::undocumented::FontCatalog = serde_json::from_value(serde_json::json!({
            "familyMetadataList": [
                {"family": "B Font", "category": "Serif", "popularity": 9, "fonts": {"400": {}, "700": {}, "400i": {}}},
                {"family": "A Font", "category": "Monospace", "popularity": 2, "fonts": {"400": {}}},
                {"family": "Only Italic", "popularity": 1, "fonts": {"400i": {}}},
                {"family": "Bad\"Name", "popularity": 1, "fonts": {"400": {}}},
                {"category": "Serif"}
            ]
        }))
        .unwrap();
        let c = catalog(r);
        assert_eq!(c.len(), 2);
        assert_eq!(c[0].family, "A Font");
        assert!(c[0].monospace());
        assert_eq!(c[1].weights, vec![400, 700]);
    }

    #[test]
    fn font_magic() {
        assert!(is_font(&[0, 1, 0, 0, 9]));
        assert!(is_font(b"OTTO...."));
        assert!(!is_font(b"<html>"));
    }
}
