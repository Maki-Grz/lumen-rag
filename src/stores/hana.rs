use crate::store::VectorStore;
use crate::types::{Metadata, Passage};
use crate::utils::compute_hash;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use hdbconnect::{Connection, IntoConnectParams};
use regex::Regex;
use std::sync::Arc;
use tokio::task;

pub struct HanaStore {
    connection: Arc<Connection>,
    table_name: String,
}

impl HanaStore {
    pub fn new<P>(params: P, table_name: String) -> Result<Self>
    where
        P: IntoConnectParams,
    {
        // Validate table name to prevent SQL injection
        let re = Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*$")?;
        if !re.is_match(&table_name) {
            return Err(anyhow!("Invalid table name: {}", table_name));
        }

        let connection = Connection::new(params)?;
        Ok(Self {
            connection: Arc::new(connection),
            table_name,
        })
    }
}

#[async_trait]
impl VectorStore for HanaStore {
    async fn add_passages(&self, passages: Vec<Passage>) -> Result<Vec<String>> {
        let conn = Arc::clone(&self.connection);
        let table_name = self.table_name.clone();

        task::spawn_blocking(move || {
            let mut inserted_ids = Vec::new();

            // Re-creating the check inside spawn_blocking for convenience
            let create_query = format!(
                "CREATE COLUMN TABLE {} (ID NVARCHAR(36) PRIMARY KEY, TEXT NCLOB, EMBEDDING REAL_VECTOR, HASH BIGINT, METADATA NCLOB)",
                table_name
            );
            match conn.exec(&create_query) {
                Ok(_) => (),
                Err(e) => {
                    if let Some(code) = e.server_error_code() {
                        if code != 288 && code != 337 {
                            return Err(anyhow!("Failed to create table: {}", e));
                        }
                    } else {
                        return Err(anyhow!("Failed to create table: {}", e));
                    }
                }
            }

            for p in passages {
                let hash = compute_hash(&p.text);
                // Use hash as stable ID if no ID is provided to prevent duplicates
                let id = p.id.clone().unwrap_or_else(|| hash.to_string());

                let vec_str = format!("{:?}", p.embedding);
                let metadata_json = serde_json::to_string(&p.metadata)?;

                let query = format!(
                    "UPSERT {} (ID, TEXT, EMBEDDING, HASH, METADATA) VALUES (?, ?, TO_REAL_VECTOR(?), ?, ?) WITH PRIMARY KEY",
                    table_name
                );

                let mut stmt = conn.prepare(&query)?;
                stmt.execute(&(&id, &p.text, &vec_str, hash as i64, &metadata_json))?;

                inserted_ids.push(id);
            }
            Ok(inserted_ids)
        })
        .await?
    }

    async fn search(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<Passage>> {
        let conn = Arc::clone(&self.connection);
        let table_name = self.table_name.clone();
        let vec_str = format!("{:?}", query_embedding);

        task::spawn_blocking(move || {
            let query = format!(
                "SELECT TOP {} ID, TEXT, TO_NVARCHAR(EMBEDDING), HASH, METADATA FROM {} ORDER BY COSINE_SIMILARITY(EMBEDDING, TO_REAL_VECTOR(?)) DESC",
                limit, table_name
            );

            let mut stmt = conn.prepare(&query)?;
            let result = stmt.execute(&(&vec_str,))?;
            let rows = result.into_result_set()?;

            let mut passages = Vec::new();
            for row in rows {
                let row = row?;
                let (id, text, emb_str, hash, metadata_str): (String, String, String, i64, Option<String>) = row.try_into()?;

                // Robust embedding parsing
                let embedding: Result<Vec<f32>> = emb_str
                    .trim_matches(|c| c == '[' || c == ']')
                    .split(',')
                    .map(|s| {
                        s.trim()
                            .parse::<f32>()
                            .map_err(|e| anyhow!("Failed to parse embedding element '{}': {}", s, e))
                    })
                    .collect();

                let embedding = embedding?;

                let metadata: Option<Metadata> = metadata_str
                    .and_then(|s| serde_json::from_str(&s).ok());

                passages.push(Passage {
                    id: Some(id),
                    text,
                    embedding,
                    metadata,
                    hash: Some(hash),
                });
            }

            Ok(passages)
        })
        .await?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hana_store_new_invalid_url() {
        let result = HanaStore::new("hdb://invalid:1234", "TEST_TABLE".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_hana_store_invalid_table_name() {
        let result = HanaStore::new("hdb://localhost:30015", "INVALID TABLE;".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid table name"));
    }
}
