# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0](https://github.com/Maki-Grz/lumen-rag/compare/lumen-rag-v0.2.1...lumen-rag-v0.3.0) (2026-02-16)


### Features

* Add ARM64 config and improve error handling ([998909e](https://github.com/Maki-Grz/lumen-rag/commit/998909e1d70c6c08264ba7a932624437e9335be9))
* Add permissive CORS support and update to `0.2.3-alpha` ([4ceffe6](https://github.com/Maki-Grz/lumen-rag/commit/4ceffe6c6018d2d210d3b2cf8f3aae931d7acb2d))
* Add SAP HANA Cloud support ([d6e8eff](https://github.com/Maki-Grz/lumen-rag/commit/d6e8efff9f13077fb583a3398c7ac04822b6dac1))
* Add system prompt and model hash handling, update dependencies ([2ee9949](https://github.com/Maki-Grz/lumen-rag/commit/2ee9949b412f40ad0534f0d0bcb5d101c5676018))
* Enhance retrieval and generation capabilities with BERT and LLM integration ([dc59314](https://github.com/Maki-Grz/lumen-rag/commit/dc593143cfe18694e60e4565251851a5a852f6d1))
* Enhance segmentation logic, retrieval thresholds, and prompt formatting ([8bd55c3](https://github.com/Maki-Grz/lumen-rag/commit/8bd55c3f2fa4b7af0b2d353e9b79a775d1917acb))
* Improve retrieval performance, segmentation, and response handling ([7631f3c](https://github.com/Maki-Grz/lumen-rag/commit/7631f3c26606c34fe25675267623214642850313))
* Improve retrieval performance, segmentation, and response handling ([c56b343](https://github.com/Maki-Grz/lumen-rag/commit/c56b343dd370ecb33df5640f7a48688a62d7526a))
* Initialize project with API endpoints for ingestion and Q&A ([5686d8e](https://github.com/Maki-Grz/lumen-rag/commit/5686d8e1226980b37e0d6d7c2f75bc421ee86958))
* Refactor and optimize ingestion, retrieval, and configuration handling ([0d0ce24](https://github.com/Maki-Grz/lumen-rag/commit/0d0ce24cfbfb7c76b4fd793d4c312b2620ebb258))
* Update text segmentation logic and model configuration ([9106403](https://github.com/Maki-Grz/lumen-rag/commit/91064035433d1f60f7b42b8d45bf4b30b6b8e167))


### Bug Fixes

* Add actix-rt dependency and resolve unused variable in HANA example ([dfbbb7c](https://github.com/Maki-Grz/lumen-rag/commit/dfbbb7c66880f13bc9192402a5e12da17fe284bf))
* Add Debug derive to HanaStore ([30055b6](https://github.com/Maki-Grz/lumen-rag/commit/30055b6f42ae4282c2c3f84591541a7b95692d09))
* Refactor HanaStore to use async tasks ([87bd796](https://github.com/Maki-Grz/lumen-rag/commit/87bd796dd07432d14eb95b71ed82a782ea4c1c2f))
* Remove nlp keyword from Cargo.toml ([241e285](https://github.com/Maki-Grz/lumen-rag/commit/241e2851ef5c7c87fc4bad2cd709a266b89c5b9f))
* Resolve build errors in Qdrant and SAP HANA stores ([491eec9](https://github.com/Maki-Grz/lumen-rag/commit/491eec9202d4e0bfbf66f83c100b1d0eb12e9337))
* Resolve type mismatch in Qdrant store and update HANA row handling ([45f1051](https://github.com/Maki-Grz/lumen-rag/commit/45f1051e4a296a269c698364484f78fd3f70bdc5))
* Update release-please-action to googleapis/release-please-action ([9d1c36b](https://github.com/Maki-Grz/lumen-rag/commit/9d1c36be70830b1e189360dd77dd5ad1e87d5c88))

## [0.2.1](https://github.com/Maki-Grz/lumen-rag/compare/v0.2.0...v0.2.1) (2026-02-15)


### Bug Fixes

* Remove nlp keyword from Cargo.toml ([241e285](https://github.com/Maki-Grz/lumen-rag/commit/241e2851ef5c7c87fc4bad2cd709a266b89c5b9f))

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
