//! # Lumen RAG Framework
//!
//! Lumen is a modular Retrieval-Augmented Generation framework for Rust.
//! It allows you to build RAG pipelines using interchangeable vector stores (MongoDB, Qdrant).
//!
//! ## Example
//! ```rust,no_run
//! use lumen_rag::{VectorStore, types::Passage};
//! // Initialize a specific store (e.g. MongoStore) via feature flags
//! ```

pub mod config;
pub mod generation;
pub mod ingestion;
pub mod store;
pub mod types;
pub mod utils;

mod stores {
    #[cfg(feature = "mongodb")]
    pub mod mongo;
    #[cfg(feature = "qdrant")]
    pub mod qdrant;
}

pub use store::VectorStore;
pub use types::{IngestRequest, IngestResponse, Metadata, Passage, QuestionRequest};

#[cfg(feature = "mongodb")]
pub use stores::mongo::MongoStore;

#[cfg(feature = "qdrant")]
pub use stores::qdrant::QdrantStore;
