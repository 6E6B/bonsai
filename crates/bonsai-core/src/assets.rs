//! Raw asset files (models, meshes, textures, animations) for the 3D avatar viewer. Assets are
//! public, so files are cached on disk once for every account; the request that fetches one
//! still goes out as the account that asked.

use bonsai_api::generated::assetdelivery;
use bonsai_api::{AccountId, Client, Error};
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

const MAX_PRIVATE_ASSET_BYTES: usize = 32 * 1024 * 1024;

#[derive(Default)]
pub(crate) struct PrivateAssetCache {
    inner: parking_lot::Mutex<PrivateAssetsInner>,
}

#[derive(Default)]
struct PrivateAssetsInner {
    entries: std::collections::VecDeque<(AccountId, String, Vec<u8>)>,
    total_bytes: usize,
}

impl PrivateAssetCache {
    pub(crate) fn get(&self, account: AccountId, key: &str) -> Option<Vec<u8>> {
        let g = self.inner.lock();
        g.entries
            .iter()
            .find(|(acc, k, _)| *acc == account && k == key)
            .map(|(_, _, b)| b.clone())
    }

    pub(crate) fn put(&self, account: AccountId, key: String, bytes: &[u8]) {
        if bytes.len() > MAX_PRIVATE_ASSET_BYTES {
            return;
        }
        let mut g = self.inner.lock();
        if let Some(pos) = g
            .entries
            .iter()
            .position(|(acc, k, _)| *acc == account && k == &key)
            && let Some((_, _, old)) = g.entries.remove(pos)
        {
            g.total_bytes = g.total_bytes.saturating_sub(old.len());
        }
        while g.total_bytes + bytes.len() > MAX_PRIVATE_ASSET_BYTES {
            let Some((_, _, evicted)) = g.entries.pop_front() else {
                break;
            };
            g.total_bytes = g.total_bytes.saturating_sub(evicted.len());
        }
        g.total_bytes += bytes.len();
        g.entries.push_back((account, key, bytes.to_vec()));
    }

    pub(crate) fn forget_account(&self, account: AccountId) {
        let mut g = self.inner.lock();
        let mut removed = 0usize;
        g.entries.retain(|(acc, _, b)| {
            let keep = *acc != account;
            if !keep {
                removed += b.len();
            }
            keep
        });
        g.total_bytes = g.total_bytes.saturating_sub(removed);
    }

    pub(crate) fn clear(&self) {
        let mut g = self.inner.lock();
        g.entries.clear();
        g.total_bytes = 0;
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
    if !req.is_valid() {
        return None;
    }
    let key = req.cache_key();
    if account.is_anonymous() {
        if let Some(b) = inner.disk.get(&key) {
            return Some(b);
        }
        return match fetch(&inner.anon, req).await {
            Ok(b) => {
                inner.disk.put(&key, &b);
                Some(b)
            }
            Err(e) => {
                tracing::debug!("asset {}: {e}", req.id);
                None
            }
        };
    }
    if inner.vault.blocked()
        || inner.vault_busy.load(std::sync::atomic::Ordering::Acquire)
        || inner.vault_ending.load(std::sync::atomic::Ordering::Acquire)
    {
        return None;
    }
    if let Some(b) = inner.private_assets.get(account, &key) {
        return Some(b);
    }
    if let Some(b) = inner.disk.get(&key) {
        return Some(b);
    }
    let client = inner.client(account)?;
    match fetch(&client, req).await {
        Ok(b) => {
            inner.private_assets.put(account, key, &b);
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

    #[test]
    fn private_asset_cache_isolates_accounts_and_clears() {
        let cache = PrivateAssetCache::default();
        let key = req("", "").cache_key();
        cache.put(AccountId(1), key.clone(), b"secret-mesh");
        assert_eq!(
            cache.get(AccountId(1), &key).as_deref(),
            Some(b"secret-mesh".as_slice())
        );
        assert_eq!(cache.get(AccountId(2), &key), None);
        assert_eq!(cache.get(AccountId::ANONYMOUS, &key), None);
        cache.forget_account(AccountId(1));
        assert_eq!(cache.get(AccountId(1), &key), None);
    }
}
