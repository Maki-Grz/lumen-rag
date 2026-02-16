use anyhow::Result;
use async_trait::async_trait;
use lumen_rag::{Passage, VectorStore};
use std::sync::{Arc, Mutex};

pub struct MockStore {
    pub storage: Arc<Mutex<Vec<Passage>>>,
}

impl MockStore {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl VectorStore for MockStore {
    async fn add_passages(&self, passages: Vec<Passage>) -> Result<Vec<String>> {
        let mut store = self.storage.lock().unwrap();
        let mut ids = Vec::new();
        for p in passages {
            ids.push("mock_id".to_string());
            store.push(p);
        }
        Ok(ids)
    }

    async fn search(&self, _query_embedding: &[f32], _limit: usize) -> Result<Vec<Passage>> {
        let store = self.storage.lock().unwrap();
        Ok(store.clone())
    }
}
