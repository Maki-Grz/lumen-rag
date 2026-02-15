use crate::store::VectorStore;
use crate::types::{Metadata, Passage};
use crate::utils::compute_hash;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use qdrant_client::qdrant::{
    vector_output, vectors_output, PointStruct, SearchPoints, UpsertPointsBuilder, Value,
};
use qdrant_client::{Payload, Qdrant};
use std::collections::HashMap;

/// A VectorStore implementation using Qdrant.
pub struct QdrantStore {
    client: Qdrant,
    collection_name: String,
}

impl QdrantStore {
    /// Creates a new QdrantStore instance.
    pub fn new(client: Qdrant, collection_name: String) -> Self {
        Self {
            client,
            collection_name,
        }
    }

    /// Helper to convert Qdrant Payload (JSON-like) back into our Metadata struct.
    fn extract_metadata(payload: &HashMap<String, Value>) -> Option<Metadata> {
        let title = payload
            .get("title")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let source = payload
            .get("source")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let date = payload
            .get("date")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let url = payload
            .get("url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        if title.is_none() && source.is_none() && date.is_none() && url.is_none() {
            None
        } else {
            Some(Metadata {
                title,
                source,
                date,
                url,
            })
        }
    }
}

#[async_trait]
impl VectorStore for QdrantStore {
    async fn add_passages(&self, passages: Vec<Passage>) -> Result<Vec<String>> {
        let mut points = Vec::new();
        let mut ids = Vec::new();

        for p in passages {
            let id_num = p.hash.unwrap_or_else(|| compute_hash(&p.text) as i64) as u64;
            let id_str = id_num.to_string();
            ids.push(id_str.clone());

            let mut payload = HashMap::new();
            payload.insert("text".to_string(), Value::from(p.text));

            if let Some(meta) = p.metadata {
                if let Some(v) = meta.title {
                    payload.insert("title".to_string(), Value::from(v));
                }
                if let Some(v) = meta.source {
                    payload.insert("source".to_string(), Value::from(v));
                }
                if let Some(v) = meta.date {
                    payload.insert("date".to_string(), Value::from(v));
                }
                if let Some(v) = meta.url {
                    payload.insert("url".to_string(), Value::from(v));
                }
            }

            let point = PointStruct::new(id_num, p.embedding, Payload::from(payload));
            points.push(point);
        }

        let request = UpsertPointsBuilder::new(&self.collection_name, points);
        self.client.upsert_points(request).await?;

        Ok(ids)
    }

    async fn search(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<Passage>> {
        let request = SearchPoints {
            collection_name: self.collection_name.clone(),
            vector: query_embedding.to_vec(),
            limit: limit as u64,
            with_payload: Some(true.into()),
            with_vectors: Some(true.into()),
            ..Default::default()
        };

        let search_result = self.client.search_points(request).await?;

        let mut passages = Vec::new();

        for scored_point in search_result.result {
            let payload = scored_point.payload;

            let text = payload
                .get("text")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Stored point missing 'text' field in payload"))?
                .to_string();

            let embedding = scored_point
                .vectors
                .and_then(|v| v.vectors_options)
                .and_then(|v| match v {
                    vectors_output::VectorsOptions::Vector(v) => match v.into_vector() {
                        vector_output::Vector::Dense(dv) => Some(dv.data),
                        _ => None,
                    },
                    _ => None,
                })
                .unwrap_or_default();

            let passage = Passage {
                id: None,
                text,
                embedding,
                metadata: Self::extract_metadata(&payload),
                hash: scored_point
                    .id
                    .and_then(|id| id.point_id_options)
                    .map(|id| match id {
                        qdrant_client::qdrant::point_id::PointIdOptions::Num(num) => num as i64,
                        _ => 0,
                    }),
            };
            passages.push(passage);
        }

        Ok(passages)
    }
}
