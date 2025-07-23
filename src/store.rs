use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct SharedStore {
    pub inner: Mutex<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct KV {
    pub key: String,
    pub value: String,
}

impl SharedStore {
    pub fn insert(&self, key: String, value: String) {
        let mut map = self.inner.lock().unwrap();
        map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let map = self.inner.lock().unwrap();
        map.get(key).cloned()
    }
}

