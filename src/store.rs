use crate::types::Passage;
use anyhow::Result;
use async_trait::async_trait;

/// A trait defining the capabilities of a vector database backend.
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Ingests a list of passages into the store.
    async fn add_passages(&self, passages: Vec<Passage>) -> Result<Vec<String>>;

    /// Searches for the nearest neighbors given a query embedding.
    async fn search(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<Passage>>;
}
