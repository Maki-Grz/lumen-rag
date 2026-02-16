//! # Lumen RAG Framework
//!
//! Lumen is a modular Retrieval-Augmented Generation framework for Rust.
//! It allows you to build RAG pipelines using interchangeable vector stores (MongoDB, Qdrant, SAP HANA Cloud).
//!
//! ## Example
//! ```rust,no_run
//! use lumen_rag::{VectorStore, types::Passage};
//! // Initialize a specific store (e.g. HanaStore) via feature flags
//! ```

// Test comment for CI/CD workflow validation
pub mod config;
pub mod generation;
pub mod ingestion;
pub mod store;
pub mod types;
pub mod utils;

pub mod stores;

pub use store::VectorStore;
pub use types::{IngestRequest, IngestResponse, Metadata, Passage, QuestionRequest};

#[cfg(feature = "mongodb")]
pub use stores::mongo::MongoStore;

#[cfg(feature = "qdrant")]
pub use stores::qdrant::QdrantStore;

#[cfg(feature = "hana")]
pub use stores::hana::HanaStore;
