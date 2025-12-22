# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-22

### Added
- **Core Framework**: Initial release of Lumen RAG, a modular framework for Retrieval-Augmented Generation in Rust.
- **Vector Store Trait**: Abstract `VectorStore` trait to support multiple database backends easily.
- **MongoDB Support**: Implementation of `MongoStore` with hybrid search capabilities (Cosine Similarity computed via Rayon).
- **Qdrant Support**: Implementation of `QdrantStore` using the official Rust client (v1.7+) and HNSW vector search.
- **Local Embeddings**: Integrated `candle-core` and `candle-transformers` for local BERT embeddings (no API required).
- **Text Ingestion**: Smart text segmentation logic with configurable token overlap and minimum token limits.
- **LLM Integration**: Streaming response support (Server-Sent Events) for OpenAI-compatible APIs (Mistral, Ollama, etc.).
- **Examples**: Full Actix-web server examples for both MongoDB (`examples/server_mongo.rs`) and Qdrant (`examples/server_qdrant.rs`).

### Changed
- **Architecture**: Separated core logic from HTTP framework (Actix is now a dev-dependency).
- **Performance**: Optimized embedding normalization using scalar division to prevent shape mismatch errors.
- **Configuration**: Unified configuration management via `.env` files.

### Fixed
- Resolved dependency conflicts between `qdrant-client` versions.
- Fixed BSON deserialization issues by excluding `_id` field from MongoDB search results.
