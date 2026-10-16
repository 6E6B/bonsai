//! Coalesce single-key lookups made close together (one UI frame) into multi-get requests.

use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{mpsc, oneshot};

use crate::error::{Error, Result};

type Fetch<K, V> =
    Arc<dyn Fn(Vec<K>) -> Pin<Box<dyn Future<Output = Result<HashMap<K, V>>> + Send>> + Send + Sync>;
type Waiter<V> = oneshot::Sender<std::result::Result<Option<V>, String>>;

/// A batching front for a multi-get endpoint (users, thumbnails, presence, game details).
///
/// Keys requested within `window` are deduplicated and sent together in chunks of `max`.
pub struct Batcher<K, V> {
    tx: mpsc::UnboundedSender<(K, Waiter<V>)>,
}

impl<K, V> Clone for Batcher<K, V> {
    fn clone(&self) -> Self {
        Self { tx: self.tx.clone() }
    }
}

impl<K, V> Batcher<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Must be called inside a tokio runtime.
    pub fn new<F, Fut>(window: Duration, max: usize, fetch: F) -> Self
    where
        F: Fn(Vec<K>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<HashMap<K, V>>> + Send + 'static,
    {
        let fetch: Fetch<K, V> = Arc::new(move |keys| Box::pin(fetch(keys)));
        let (tx, mut rx) = mpsc::unbounded_channel::<(K, Waiter<V>)>();
        tokio::spawn(async move {
            while let Some(first) = rx.recv().await {
                let mut pending: Vec<(K, Waiter<V>)> = vec![first];
                let deadline = tokio::time::Instant::now() + window;
                while let Ok(Some(item)) = tokio::time::timeout_at(deadline, rx.recv()).await {
                    pending.push(item);
                }
                let mut waiters: HashMap<K, Vec<Waiter<V>>> = HashMap::new();
                let mut order = Vec::new();
                for (k, w) in pending {
                    let e = waiters.entry(k.clone()).or_default();
                    if e.is_empty() {
                        order.push(k);
                    }
                    e.push(w);
                }
                for chunk in order.chunks(max.max(1)) {
                    let res = fetch(chunk.to_vec()).await;
                    for k in chunk {
                        for w in waiters.remove(k).unwrap_or_default() {
                            let r = match &res {
                                Ok(map) => Ok(map.get(k).cloned()),
                                Err(e) => Err(e.to_string()),
                            };
                            let _ = w.send(r);
                        }
                    }
                }
            }
        });
        Self { tx }
    }

    /// Look up one key; resolves when its batch returns. `Ok(None)` means the server did not return it.
    pub async fn get(&self, key: K) -> Result<Option<V>> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send((key, tx))
            .map_err(|_| Error::Network("batcher stopped".into()))?;
        rx.await
            .map_err(|_| Error::Network("batcher stopped".into()))?
            .map_err(Error::Network)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[tokio::test]
    async fn coalesces_and_dedupes() {
        let calls = Arc::new(AtomicUsize::new(0));
        let c = calls.clone();
        let b: Batcher<u64, String> = Batcher::new(Duration::from_millis(20), 2, move |keys: Vec<u64>| {
            c.fetch_add(1, Ordering::SeqCst);
            async move {
                Ok(keys
                    .into_iter()
                    .filter(|k| *k != 9)
                    .map(|k| (k, format!("v{k}")))
                    .collect())
            }
        });
        let (a, b2, c2, d, e) = tokio::join!(b.get(1), b.get(2), b.get(1), b.get(3), b.get(9));
        assert_eq!(a.unwrap().as_deref(), Some("v1"));
        assert_eq!(b2.unwrap().as_deref(), Some("v2"));
        assert_eq!(c2.unwrap().as_deref(), Some("v1"));
        assert_eq!(d.unwrap().as_deref(), Some("v3"));
        assert_eq!(e.unwrap(), None);
        // 4 unique keys, chunks of 2
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }
}
