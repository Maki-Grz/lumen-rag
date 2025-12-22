use serde::{Deserialize, Serialize};

/// Generic configuration for the RAG Pipeline.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RagConfig {
    /// Name of the database (Mongo / Qdrant collection)
    pub database_name: String,
    /// Name of the collection
    pub collection_name: String,
    /// LLM API URI (e.g., Ollama, vLLM)
    pub llm_uri: Option<String>,
    /// LLM model to use
    pub llm_model: Option<String>,
}

impl Default for RagConfig {
    fn default() -> Self {
        Self {
            database_name: "knowledge_base".to_string(),
            collection_name: "vectors".to_string(),
            llm_uri: Some("http://localhost:11434/api/chat".to_string()),
            llm_model: Some("mistral".to_string()),
        }
    }
}
