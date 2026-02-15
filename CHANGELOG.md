# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0](https://github.com/Maki-Grz/lumen-rag/compare/v0.1.0...v0.2.0) (2026-02-15)


### Features

* Add ARM64 config and improve error handling ([998909e](https://github.com/Maki-Grz/lumen-rag/commit/998909e1d70c6c08264ba7a932624437e9335be9))
* add SAP HANA Cloud support ([d6e8eff](https://github.com/Maki-Grz/lumen-rag/commit/d6e8efff9f13077fb583a3398c7ac04822b6dac1))


### Bug Fixes

* add actix-rt dependency and resolve unused variable in HANA example ([dfbbb7c](https://github.com/Maki-Grz/lumen-rag/commit/dfbbb7c66880f13bc9192402a5e12da17fe284bf))
* Add Debug derive to HanaStore ([30055b6](https://github.com/Maki-Grz/lumen-rag/commit/30055b6f42ae4282c2c3f84591541a7b95692d09))
* Refactor HanaStore to use async tasks ([87bd796](https://github.com/Maki-Grz/lumen-rag/commit/87bd796dd07432d14eb95b71ed82a782ea4c1c2f))
* resolve build errors in Qdrant and SAP HANA stores ([491eec9](https://github.com/Maki-Grz/lumen-rag/commit/491eec9202d4e0bfbf66f83c100b1d0eb12e9337))
* resolve type mismatch in Qdrant store and update HANA row handling ([45f1051](https://github.com/Maki-Grz/lumen-rag/commit/45f1051e4a296a269c698364484f78fd3f70bdc5))
* Update release-please-action to googleapis/release-please-action ([9d1c36b](https://github.com/Maki-Grz/lumen-rag/commit/9d1c36be70830b1e189360dd77dd5ad1e87d5c88))

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
