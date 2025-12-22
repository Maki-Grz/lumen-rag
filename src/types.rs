use serde::{Deserialize, Serialize};

/// Represents a chunk of text processed by the RAG pipeline.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Passage {
    /// Unique identifier for the passage.
    /// If using MongoDB, this maps to the ObjectId string.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The content of the passage.
    pub text: String,

    /// The computed vector embedding.
    pub embedding: Vec<f32>,

    /// Optional metadata associated with the passage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// A hash of the text content, used for deduplication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<i64>,
}

/// Metadata associated with a source document.
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Request payload for ingesting text.
#[derive(Deserialize, Debug)]
pub struct IngestRequest {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IngestResponse {
    pub passage_ids: Vec<String>,
    pub count: usize,
}

/// Request payload for asking a question.
#[derive(Deserialize, Debug)]
pub struct QuestionRequest {
    pub question: String,
}

// Internal structures for LLM communication
#[derive(Serialize, Clone, Debug)]
pub struct LLMRequest {
    pub model: String,
    pub messages: Vec<LLMMessage>,
    pub stream: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LLMMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct LLMStreamResponse {
    pub choices: Vec<LLMChoice>,
}

#[derive(Deserialize, Debug)]
pub struct LLMChoice {
    pub delta: LLMStreamMessage,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LLMStreamMessage {
    pub content: String,
}
