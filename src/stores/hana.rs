use crate::store::VectorStore;
use crate::types::Passage;
use crate::utils::compute_hash;
use anyhow::Result;
use async_trait::async_trait;
use hdbconnect::{Connection, IntoConnectParams};

pub struct HanaStore {
    connection: Connection,
    table_name: String,
}

impl HanaStore {
    pub fn new<P>(params: P, table_name: String) -> Result<Self>
    where
        P: IntoConnectParams,
    {
        let connection = Connection::new(params)?;
        Ok(Self {
            connection,
            table_name,
        })
    }

    fn create_table_if_not_exists(&self) -> Result<()> {
        let query = format!(
            "CREATE COLUMN TABLE {} (ID NVARCHAR(36) PRIMARY KEY, TEXT NCLOB, EMBEDDING REAL_VECTOR, HASH BIGINT)",
            self.table_name
        );
        let _ = self.connection.exec(&query);
        Ok(())
    }
}

#[async_trait]
impl VectorStore for HanaStore {
    async fn add_passages(&self, passages: Vec<Passage>) -> Result<Vec<String>> {
        let mut inserted_ids = Vec::new();
        self.create_table_if_not_exists()?;

        for p in passages {
            let hash = compute_hash(&p.text);
            let id =
                p.id.clone()
                    .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

            let vec_str = format!("{:?}", p.embedding);

            let query = format!(
                "UPSERT {} (ID, TEXT, EMBEDDING, HASH) VALUES (?, ?, TO_REAL_VECTOR(?), ?) WHERE HASH = ?",
                self.table_name
            );

            let mut stmt = self.connection.prepare(&query)?;
            stmt.execute((&id, &p.text, &vec_str, hash as i64, hash as i64))?;

            inserted_ids.push(id);
        }
        Ok(inserted_ids)
    }

    async fn search(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<Passage>> {
        let vec_str = format!("{:?}", query_embedding);
        let query = format!(
            "SELECT TOP {} ID, TEXT, TO_NVARCHAR(EMBEDDING), HASH FROM {} ORDER BY COSINE_SIMILARITY(EMBEDDING, TO_REAL_VECTOR(?)) DESC",
            limit, self.table_name
        );

        let mut stmt = self.connection.prepare(&query)?;
        let result = stmt.execute((&vec_str,))?;
        let rows = result.into_result_set()?;

        let mut passages = Vec::new();
        for row in rows {
            let (_id, text, emb_str, hash): (String, String, String, i64) = row.try_into()?;

            let embedding: Vec<f32> = emb_str
                .trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            passages.push(Passage {
                id: Some(_id),
                text,
                embedding,
                metadata: None,
                hash: Some(hash),
            });
        }

        Ok(passages)
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
}
