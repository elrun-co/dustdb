use std::collections::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValueEntry {
    pub value: String,
    pub ttl: Option<u64>,
}

#[derive(Clone)]
pub struct Database {
    data: Arc<RwLock<HashMap<String, ValueEntry>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn set(&self, key: &str, value: &str) {
        self.data.write().insert(key.to_string(), ValueEntry { value: value.to_string(), ttl: None });
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.read().get(key).map(|v| v.value.clone())
    }

    pub fn del(&self, key: &str) -> bool {
        self.data.write().remove(key).is_some()
    }

    pub fn dump(&self) -> Vec<u8> {
        let map = self.data.read();
        bincode::serialize(&*map).unwrap_or_default()
    }

    pub fn load(&self, bytes: &[u8]) {
        if let Ok(map) = bincode::deserialize::<HashMap<String, ValueEntry>>(bytes) {
            *self.data.write() = map;
        }
    }

    pub fn keys(&self) -> Vec<String> {
        self.data.read().keys().cloned().collect()
    }
}

