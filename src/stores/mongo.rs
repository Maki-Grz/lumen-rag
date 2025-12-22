use crate::store::VectorStore;
use crate::types::Passage;
use crate::utils::compute_hash;
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::{FuturesUnordered, StreamExt};
use mongodb::bson::{doc, oid::ObjectId, Bson};
use mongodb::{Client, Collection};
use rayon::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

pub struct MongoStore {
    client: Client,
    db_name: String,
    collection_name: String,
    fetch_limit: i64,
}

impl MongoStore {
    pub fn new(client: Client, db_name: String, collection_name: String) -> Self {
        Self {
            client,
            db_name,
            collection_name,
            fetch_limit: 2000,
        }
    }

    fn get_collection(&self) -> Collection<Passage> {
        self.client
            .database(&self.db_name)
            .collection(&self.collection_name)
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot / (norm_a * norm_b)
        }
    }
}

#[async_trait]
impl VectorStore for MongoStore {
    async fn add_passages(&self, passages: Vec<Passage>) -> Result<Vec<String>> {
        let collection = self.get_collection();
        let mut inserted_ids = Vec::new();
        let mut tasks = FuturesUnordered::new();
        let coll_arc = Arc::new(collection);

        for mut p in passages {
            let coll = Arc::clone(&coll_arc);
            tasks.push(async move {
                let hash = compute_hash(&p.text);
                p.hash = Some(hash as i64);

                if let Ok(Some(existing)) = coll.find_one(doc! { "hash": hash as i64 }).await {
                    if let Some(id_str) = existing.id {
                        return Ok(id_str);
                    }
                }

                let _id_filter = if let Some(ref id_str) = p.id {
                    if let Ok(oid) = ObjectId::from_str(id_str) {
                        Some(doc! { "_id": oid })
                    } else {
                        None
                    }
                } else {
                    None
                };

                match coll.insert_one(p).await {
                    Ok(res) => match res.inserted_id {
                        Bson::ObjectId(oid) => Ok(oid.to_string()),
                        _ => Ok("unknown_id".to_string()),
                    },
                    Err(e) => Err(anyhow::anyhow!("DB Error: {}", e)),
                }
            });
        }

        while let Some(res) = tasks.next().await {
            if let Ok(id) = res {
                inserted_ids.push(id);
            }
        }
        Ok(inserted_ids)
    }

    async fn search(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<Passage>> {
        let collection = self.get_collection();

        let find_opts = mongodb::options::FindOptions::builder()
            .limit(self.fetch_limit)
            .projection(doc! {
                "text": 1,
                "embedding": 1,
                "metadata": 1,
                "_id": 0
            })
            .build();

        let mut cursor = collection.find(doc! {}).with_options(find_opts).await?;
        let mut candidates = Vec::new();

        while cursor.advance().await? {
            candidates.push(cursor.deserialize_current()?);
        }

        let mut scored_passages: Vec<_> = candidates
            .par_iter()
            .map(|p| {
                let sim = Self::cosine_similarity(query_embedding, &p.embedding);
                (p.clone(), sim)
            })
            .collect();

        scored_passages
            .sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let result: Vec<Passage> = scored_passages
            .into_iter()
            .take(limit)
            .map(|(p, _)| p)
            .collect();

        Ok(result)
    }
}
