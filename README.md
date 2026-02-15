# Lumen RAG Framework

[![Crates.io](https://img.shields.io/crates/v/lumen-rag.svg)](https://crates.io/crates/lumen-rag)
[![Documentation](https://docs.rs/lumen-rag/badge.svg)](https://docs.rs/lumen-rag)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Lumen** is a high-performance, modular, and database-agnostic **RAG (Retrieval-Augmented Generation)** framework written in Rust. 

It abstracts the complexity of vector storage and retrieval, allowing you to switch seamlessy between **MongoDB**, **CosmosDB**, **Qdrant**, and **SAP HANA Cloud**, while providing built-in support for state-of-the-art embeddings (BERT) via `candle`.

## 🚀 Features

- **🔌 Modular Backends**: Switch between MongoDB, Qdrant, and SAP HANA Cloud with Feature Flags.
- **⚡ High Performance**: Built on `Tokio`, `Actix-web`, and `Rayon` for async and parallel processing.
- **🧠 Local Embeddings**: Integrated BERT support using Hugging Face's `candle` (no external API needed for embeddings).
- **📄 Smart Chunking**: Intelligent text segmentation preserving semantic context.
- **🤖 LLM Agnostic**: Compatible with any OpenAI-compatible API (Ollama, vLLM, OpenAI, Mistral, etc.).

## 📦 Installation

Add `lumen-rag` to your `Cargo.toml`. Select the database backend you need:

```toml
[dependencies]
# For MongoDB or CosmosDB support
lumen-rag = { version = "0.1.0", features = ["mongodb"] }

# For Qdrant support
lumen-rag = { version = "0.1.0", features = ["qdrant"] }

# For SAP HANA Cloud support
lumen-rag = { version = "0.1.0", features = ["hana"] }
```

## 🛠️ Configuration

Lumen uses environment variables for configuration. Create a `.env` file in your project root:

```ini
# --- LLM Settings ---
LLM_URI=https://api.openai.com/v1/chat/completions
MODEL=gpt-3.5-turbo
LLM_API_KEY=sk-your-api-key-here

# --- Database Settings ---
# MongoDB / CosmosDB
COSMOS_URI=mongodb://admin:password@localhost:27017
DATABASE=lumen_db
COLLECTION=knowledge_base

# Qdrant
QDRANT_URI=http://localhost:6334

# SAP HANA Cloud
HANA_URL=hdb://user:password@host:port
HANA_TABLE=LUMEN_RAG_TABLE
```

## 🏗️ Architecture

Lumen is built around the `VectorStore` trait, enabling easy integration of new vector databases.

```rust
#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn add_passages(&self, passages: Vec<Passage>) -> Result<Vec<String>>;
    async fn search(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<Passage>>;
}
```

### Supported Stores

| Database | Feature Flag | Search Type |
| --- | --- | --- |
| **MongoDB** | `mongodb` | Hybrid (Fetch + In-memory Cosine Similarity) |
| **CosmosDB** | `mongodb` | Hybrid (Mongo API Compatible) |
| **Qdrant** | `qdrant` | Native HNSW Vector Search |
| **SAP HANA Cloud** | `hana` | Native Vector Search (REAL_VECTOR) |

> [!WARNING]
> **Experimental Feature**: SAP HANA Cloud support is currently in beta and has not been fully validated against a live instance.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.