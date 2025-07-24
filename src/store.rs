use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct SharedStore {
    inner: Arc<RwLock<HashMap<String, String>>>,
}

impl SharedStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, key: String, value: String) {
        let mut store = self.inner.write().await;
        store.insert(key, value);
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let store = self.inner.read().await;
        store.get(key).cloned()
    }
}

