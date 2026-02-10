use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Simple in-memory cache for entity lookups.
/// Session-scoped (lives as long as the MCP server process).
#[derive(Clone)]
pub struct EntityCache<T: Clone> {
    inner: Arc<RwLock<HashMap<String, T>>>,
}

impl<T: Clone> Default for EntityCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> EntityCache<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get a cached value or fetch and cache it.
    pub async fn get_or_fetch<F, Fut, E>(&self, key: &str, fetch: F) -> Result<T, E>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        // Check cache first (read lock)
        {
            let cache = self.inner.read().await;
            if let Some(val) = cache.get(key) {
                return Ok(val.clone());
            }
        }

        // Cache miss — fetch and store
        let val = fetch().await?;
        {
            let mut cache = self.inner.write().await;
            cache.insert(key.to_string(), val.clone());
        }
        Ok(val)
    }
}
