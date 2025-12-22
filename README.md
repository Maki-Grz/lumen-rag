# Lumen RAG Framework

[![Crates.io](https://img.shields.io/crates/v/lumen-rag.svg)](https://crates.io/crates/lumen-rag)
[![Documentation](https://docs.rs/lumen-rag/badge.svg)](https://docs.rs/lumen-rag)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Lumen** is a high-performance, modular, and database-agnostic **RAG (Retrieval-Augmented Generation)** framework written in Rust. 

It abstracts the complexity of vector storage and retrieval, allowing you to switch seamlessy between **MongoDB**, **CosmosDB**, and **Qdrant**, while providing built-in support for state-of-the-art embeddings (BERT) via `candle`.

## 🚀 Features

- **🔌 Modular Backends**: Switch between MongoDB (Atlas/Local) and Qdrant with Feature Flags.
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

# OR for Qdrant support
lumen-rag = { version = "0.1.0", features = ["qdrant"] }

```

## 🛠️ Configuration

Lumen uses environment variables for configuration. Create a `.env` file in your project root:

```ini
# --- LLM Settings ---
# URL to your LLM provider (Ollama, OpenAI, Groq, etc.)
LLM_URI=[https://api.openai.com/v1/chat/completions](https://api.openai.com/v1/chat/completions)
# Model name
MODEL=gpt-3.5-turbo
# (Optional) API Key if using a cloud provider
LLM_API_KEY=sk-your-api-key-here

# --- Database Settings ---
# MongoDB / CosmosDB
COSMOS_URI=mongodb://admin:password@localhost:27017
DATABASE=lumen_db
COLLECTION=knowledge_base

# Qdrant
QDRANT_URI=http://localhost:6334
```

## ⚡ Quick Start

### 1. Setup Infrastructure

You can easily start the required databases using Docker:

```bash
# Start MongoDB and Qdrant
docker run -d -p 27017:27017 -e MONGO_INITDB_ROOT_USERNAME=admin -e MONGO_INITDB_ROOT_PASSWORD=password mongo:latest
docker run -d -p 6333:6333 -p 6334:6334 qdrant/qdrant:latest

```

### 2. Run the Examples

We provide full server examples in the `examples/` folder.

**Using MongoDB:**

```bash
cargo run --example server_mongo --features mongodb

```

**Using Qdrant:**

```bash
cargo run --example server_qdrant --features qdrant

```

### 3. Test the API

#### Ingest a Document (Indexing)

```bash
curl -X POST [http://127.0.0.1:8080/ingest](http://127.0.0.1:8080/ingest) \
   -H "Content-Type: application/json" \
   -d '{
     "text": "Rust is a systems programming language focused on safety and performance.",
     "metadata": {"source": "wikipedia"}
   }'

```

#### Ask a Question (RAG)

```bash
curl -X POST [http://127.0.0.1:8080/ask](http://127.0.0.1:8080/ask) \
   -H "Content-Type: application/json" \
   -d '{
     "question": "What is Rust focused on?"
   }'

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

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.