//! Raw asset files (models, meshes, textures, animations) for the 3D avatar viewer. Assets are
//! public, so files are cached on disk once for every account; the request that fetches one
//! still goes out as the account that asked.

use rbx_api::generated::assetdelivery;
use rbx_api::{AccountId, Client, Error};
use tokio::sync::oneshot;

use crate::core::{Core, Inner};

/// One asset file. `format` and `representations` pick a variant of the same asset, as the
/// avatar renderer asks for them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AssetReq {
    pub id: u64,
    /// `Roblox-AssetFormat`, like `avatar_meshpart_accessory`. Empty for the default.
    pub format: String,
    /// `contentRepresentationPriorityList`, base64 JSON as the website sends it. Empty for none.
    pub representations: String,
}

impl AssetReq {
    fn cache_key(&self) -> String {
        format!("asset:{}:{}:{}", self.id, self.format, self.representations)
    }

    /// Whether the variant fields hold only what Roblox uses there: format names and base64.
    pub fn is_valid(&self) -> bool {
        let format = self.format.len() <= 64
            && self
                .format
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-');
        let reps = self.representations.len() <= 1024
            && self
                .representations
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"+/=".contains(&b));
        self.id > 0 && format && reps
    }
}

impl Core {
    /// An asset file's bytes, from disk or Roblox. Resolves to `None` when Roblox won't serve it.
    pub fn asset_file(&self, account: AccountId, req: AssetReq) -> oneshot::Receiver<Option<Vec<u8>>> {
        let (tx, rx) = oneshot::channel();
        let inner = self.inner.clone();
        self.inner.rt.spawn(async move {
            let _ = tx.send(load(&inner, account, &req).await);
        });
        rx
    }
}

async fn load(inner: &Inner, account: AccountId, req: &AssetReq) -> Option<Vec<u8>> {
    let key = req.cache_key();
    if let Some(b) = inner.disk.get(&key) {
        return Some(b);
    }
    let client = inner.client(account).unwrap_or_else(|| inner.anon.clone());
    match fetch(&client, req).await {
        Ok(b) => {
            inner.disk.put(&key, &b);
            Some(b)
        }
        Err(e) => {
            tracing::debug!("asset {}: {e}", req.id);
            None
        }
    }
}

async fn fetch(c: &Client, req: &AssetReq) -> Result<Vec<u8>, Error> {
    let params = assetdelivery::v2::GetAssetIdByAssetIdParams {
        roblox_asset_format: req.format.clone(),
        content_representation_priority_list: (!req.representations.is_empty())
            .then(|| req.representations.clone()),
        ..Default::default()
    };
    let r = assetdelivery::v2::get_asset_id_by_asset_id(c, req.id as i64, &params).await?;
    let url = r
        .locations
        .unwrap_or_default()
        .into_iter()
        .find_map(|l| l.location)
        .ok_or_else(|| Error::Message("no location".into()))?;
    c.fetch_cdn(&url).await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req(format: &str, reps: &str) -> AssetReq {
        AssetReq {
            id: 1,
            format: format.into(),
            representations: reps.into(),
        }
    }

    #[test]
    fn validates_variants() {
        assert!(req("", "").is_valid());
        assert!(req("avatar_meshpart_head", "W3siZm9ybWF0Ijoi+/=").is_valid());
        assert!(!req("a b", "").is_valid());
        assert!(!req("", "a&b=c").is_valid());
        assert!(!AssetReq::default().is_valid());
    }

    #[test]
    fn variants_cache_apart() {
        assert_ne!(
            req("", "").cache_key(),
            req("avatar_meshpart_head", "").cache_key()
        );
        assert_ne!(req("", "").cache_key(), req("", "eA==").cache_key());
    }
}
