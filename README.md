# Lumen RAG Framework

[![Crates.io](https://img.shields.io/crates/v/lumen-rag.svg)](https://crates.io/crates/lumen-rag)
[![Documentation](https://docs.rs/lumen-rag/badge.svg)](https://docs.rs/lumen-rag)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Project Overview & Abstract

The Lumen framework is a modular, database-agnostic architecture designed for high-throughput, low-latency Retrieval-Augmented Generation (RAG) operations. The system addresses the computational challenges of context ingestion, text segmentation, high-dimensional vector space generation, and distributed index querying. In standard production RAG pipelines, tight coupling between application layers and specific vector search engines limits portability and system resilience. Lumen solves this problem by establishing an abstract interface layer that standardizes vector operations across heterogeneous storage architectures, including document-oriented databases, native vector search engines, and relational column-store systems.

The core objective of the framework is to minimize context retrieval latency $L_{\text{retrieve}}$ and optimize semantic search recall under heavy concurrent query loads. The data processing and retrieval pipelines are mathematically modeled to bound computational overhead. The framework leverages a multi-threaded, asynchronous runtime environment to isolate compute-bound tokenization tasks from I/O-bound storage operations, providing a predictable execution pipeline for enterprise-grade artificial intelligence workflows.

---

## Core Architecture & Design Decisions

Lumen is engineered in Rust to leverage its zero-cost abstractions, compile-time memory safety guarantees, and deterministic control over resource allocation. The architecture completely avoids garbage collection pauses, which ensures predictable latency curves at high percentiles ($p99$).

```
                +-----------------------------------------+
                |          Lumen Orchestration            |
                +-----------------------------------------+
                                     |
               Polymorphic Dispatch via VectorStore Trait
                                     |
         +---------------------------+---------------------------+
         |                           |                           |
         v                           v                           v
  [Feature: qdrant]          [Feature: mongodb]           [Feature: hana]
         |                           |                           |
         v                           v                           v
  Remote Qdrant Node         MongoDB / CosmosDB            SAP HANA Cloud
  (Native HNSW Index)       (In-Memory Cosine Sim)       (Native REAL_VECTOR)

```

### 1. Asynchronous and Parallel Execution Model

The system architecture decouples I/O-bound tasks from compute-bound tasks by leveraging two distinct concurrency primitives:

* **`tokio` Execution Matrix:** Manages non-blocking asynchronous execution over an $M:N$ work-stealing green-thread scheduler. This matrix governs all network operations, database driver sessions, and downstream Large Language Model (LLM) API communication.
* **`rayon` Work-Stealing Pool:** Offloads CPU-bound data operations, specifically text chunking, document parsing, and tokenization matrix transformations, to a dedicated thread pool to prevent blocking the asynchronous executor loops.

### 2. Localized Tensor Embeddings via Candle

To mitigate the latency overhead, network volatility, and data sovereignty compliance concerns associated with external Software-as-a-Service (SaaS) embedding endpoints, Lumen integrates an on-device tensor execution sub-engine using the Hugging Face `candle` minimalist machine learning framework. The framework compiles Bidirectional Encoder Representations from Transformers (BERT) models directly into native machine code, optimizing vector calculation routines over target instruction sets.

### 3. Decoupled Storage Engine Polymorphism

The database layer is abstracted via a static or dynamic polymorphic dispatch contract using the `VectorStore` trait. The architectural trade-offs of the supported database backends are configured via compile-time feature flags:

* **Qdrant (`features = ["qdrant"]`):** Dispatches queries to a native vector database. It utilizes Hierarchical Navigable Small World (HNSW) graph networks, bounding search time to sub-linear time complexity.
* **SAP HANA Cloud (`features = ["hana"]`):** Integrates vector search inside a columnar relational engine via the native `REAL_VECTOR` data type. This design supports Hybrid Transactional/Analytical Processing (HTAP), allowing concurrent execution of structural SQL queries and high-dimensional cosine distance evaluations.
* **MongoDB / CosmosDB (`features = ["mongodb"]`):** Designed for deployments lacking native vector database infrastructure. The system executes an architecture fallback: it performs coarse unstructured document retrieval from the remote database and delegates the high-dimensional cosine similarity calculations to local in-memory CPU registers.

---

## Algorithmic Design & Data Flow

The operational data pipeline executes in two primary phases: Parallel Document Ingestion and Synchronous Contextual Retrieval.

```
[Raw Document Stream] 
         |
         v
[Rayon Parallel Ingestion Pool] ---> Linear / Slidewindow Smart Chunking
         |
         v
[Candle Embedded Subsystem]    ---> Inference Matrix: E = Local_BERT(Chunks)
         |
         v
[Polymorphic Trait Vector]     ---> Conditional Feature Flag Storage Mapping

```

### 1. Mathematical Formulation of the In-Memory Fallback

When utilizing the `mongodb` fallback architecture, the framework retrieves a candidate document set $D$. Each document $d_i \in D$ possesses a stored embedding vector $v_i \in \mathbb{R}^d$. Given a query vector $q \in \mathbb{R}^d$, the local CPU pool evaluates the Cosine Similarity across the matrix stack:

$$\text{Similarity}(q, v_i) = \frac{q \cdot v_i}{\|q\|_2 \|v_i\|_2} = \frac{\sum_{j=1}^{d} q_j v_{ij}}{\sqrt{\sum_{j=1}^{d} q_j^2} \sqrt{\sum_{j=1}^{d} v_{ij}^2}}$$

### 2. Trait Specification

The execution boundary between the orchestration layer and storage engine backends is governed by the following strict architectural contract:

```rust
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Ingests segmented passages into the target repository, 
    /// returning an array of unique document identifiers.
    async fn add_passages(&self, passages: Vec<Passage>) -> Result<Vec<String>, FrameworkError>;
    
    /// Executes a mathematical distance query across the target indexing structure.
    async fn search(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<Passage>, FrameworkError>;
}

```

---

## Technical Specifications & Performance Metrics

### Algorithmic Complexity Mapping

The performance vectors of the framework scale differently depending on the chosen database engine flag. The computational and space complexities are categorized as follows:

| Database Target | Algorithmic Paradigm | Time Complexity (Query) | Space Complexity (Index) |
| --- | --- | --- | --- |
| **Qdrant** | HNSW Graph Traversal | $\mathcal{O}(\log N)$ | $\mathcal{O}(N \cdot M)$ |
| **SAP HANA Cloud** | Native Columnar Indexing | $\mathcal{O}(N \cdot d)$ | $\mathcal{O}(N \cdot d)$ |
| **MongoDB / Fallback** | Linear Fetch + Local Compute | $\mathcal{O}(N \cdot d)$ | $\mathcal{O}(1) \text{ (Heap-allocated)}$ |

*Where $N$ represents the total number of items in the vector collection, $d$ represents the vector dimensionality (e.g., $d = 768$ for BERT-base architectures), and $M$ represents the bi-directional link density factor per graph node.*

### Critical Engineering Trade-offs

* **In-Memory Compute Fallback:** The `mongodb` strategy introduces an $\mathcal{O}(N \cdot d)$ computational bottleneck. While it eliminates the operational cost of dedicated vector databases, it binds system RAM scaling directly to dataset size, introducing out-of-memory (OOM) risks if collection parsing bounds are omitted.
* **HNSW Quantization:** The `qdrant` search path exchanges absolute precision for deterministic execution speed. Graph configurations parameterize the search space, introducing a negligible drop in semantic recall in exchange for predictable sub-millisecond query execution timelines.

---

## Deployment & Computational Requirements

### Compilation Flag Configuration Matrix

Lumen leverages conditional compilation to eliminate unnecessary driver code from the compiled executable, minimizing the final binary footprint and optimization overhead.

```bash
# Build profile optimized for native distributed graph index testing
cargo build --release --features qdrant

# Build profile optimized for enterprise SAP hybrid transactional integration architectures
cargo build --release --features hana

# Build profile optimized for standard document-store operational constraints
cargo build --release --features mongodb

```

### Environment Control Declarations

The internal state machines, connection pools, and downstream endpoints are controlled via uniform environment schemas verified at initialization runtime:

```ini
# Core LLM Interface Specifications
LLM_URI=https://api.internal-orchestrator.telecom/v1/chat/completions
MODEL=gpt-4-deduplicated
LLM_API_KEY=v1_sec_crypto_payload_stream

# Distributed Node Routing Coordinates
QDRANT_URI=http://10.240.0.12:6334
COSMOS_URI=mongodb://root_admin:secure_entropy_string@10.240.0.14:27017
HANA_URL=hdb://db_runtime_user:vector_pass_string@10.240.1.50:30015
HANA_TABLE=TELECOM_PARIS_LUMEN_INDEX

```

### Runtime Target Validation

To deploy the local embedding subsystem via `candle`, the host compilation target must feature hardware-specific extensions to accelerate linear algebra calculations. For `x86_64` execution nodes, target extensions should prioritize Advanced Vector Extensions (`AVX2`, `FMA`). For `aarch64` architectures, targets must pass underlying native Neon instruction feature flags during the execution loop bootstrap phase.
