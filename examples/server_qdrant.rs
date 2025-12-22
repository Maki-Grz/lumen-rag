use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use candle_core::Device;
use futures::StreamExt;
use qdrant_client::Qdrant;

use lumen_rag::{
    generation::generate_answer,
    ingestion::segment_text,
    store::VectorStore,
    types::{IngestRequest, IngestResponse, QuestionRequest},
    utils::{compute_text_embedding, load_bert_model_and_tokenizer},
    QdrantStore,
};

struct AppConfig {
    llm_uri: String,
    model: String,
}

struct AppState {
    model: candle_transformers::models::bert::BertModel,
    tokenizer: tokenizers::Tokenizer,
    device: Device,
    store: QdrantStore,
    config: AppConfig,
}

#[post("/ingest")]
async fn ingest(state: web::Data<AppState>, req: web::Json<IngestRequest>) -> impl Responder {
    println!("Ingesting text...");

    let passages = segment_text(&req.text, req.metadata.clone(), &state.tokenizer);
    println!("Passages générés : {}", passages.len());

    let mut passages_with_vectors = Vec::new();
    let model = &state.model;
    let tokenizer = &state.tokenizer;
    let device = &state.device;

    for mut p in passages {
        if let Ok(tensor) = compute_text_embedding(model, tokenizer, device, &p.text).await {
            if let Ok(vec) = tensor.to_vec2::<f32>() {
                if let Some(emb) = vec.first() {
                    p.embedding = emb.clone();
                    passages_with_vectors.push(p);
                }
            }
        }
    }

    if passages_with_vectors.is_empty() {
        return HttpResponse::BadRequest().body("Aucun vecteur généré (texte trop court ?)");
    }

    match state.store.add_passages(passages_with_vectors).await {
        Ok(ids) => HttpResponse::Ok().json(IngestResponse {
            count: ids.len(),
            passage_ids: ids,
        }),
        Err(e) => {
            eprintln!("Error adding passages: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[post("/ask")]
async fn ask(state: web::Data<AppState>, req: web::Json<QuestionRequest>) -> impl Responder {
    let q_tensor =
        compute_text_embedding(&state.model, &state.tokenizer, &state.device, &req.question)
            .await
            .unwrap();
    let q_vec = q_tensor.to_vec2::<f32>().unwrap()[0].clone();

    let passages = match state.store.search(&q_vec, 5).await {
        Ok(p) => p,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    if passages.is_empty() {
        return HttpResponse::Ok().body("Aucun résultat trouvé dans la base de connaissances.");
    }

    let stream_result = generate_answer(
        &req.question,
        &passages,
        &state.config.llm_uri,
        &state.config.model,
    )
    .await;

    match stream_result {
        Ok(stream) => {
            let sse_stream = stream.map(|chunk| match chunk {
                Ok(text) => Ok::<_, actix_web::Error>(web::Bytes::from(text)),
                Err(e) => Ok::<_, actix_web::Error>(web::Bytes::from(format!(" [Erreur: {}] ", e))),
            });

            HttpResponse::Ok()
                .append_header(("Content-Type", "text/event-stream"))
                .streaming(sse_stream)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let qdrant_uri =
        std::env::var("QDRANT_URI").unwrap_or_else(|_| "http://localhost:6334".to_string());
    let collection_name =
        std::env::var("COLLECTION").unwrap_or_else(|_| "lumen_collection".to_string());

    let app_config = AppConfig {
        llm_uri: std::env::var("LLM_URI").expect("LLM_URI must be set"),
        model: std::env::var("MODEL").unwrap_or("mistral".to_string()),
    };

    let device = Device::Cpu;
    let (model, tokenizer) = load_bert_model_and_tokenizer(&device, None)?;

    let client = Qdrant::from_url(&qdrant_uri).build()?;
    let store = QdrantStore::new(client, collection_name);

    let app_state = web::Data::new(AppState {
        model,
        tokenizer,
        device,
        store,
        config: app_config,
    });

    println!("Qdrant Server running on 0.0.0.0:8081");
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(app_state.clone())
            .service(ingest)
            .service(ask)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await?;

    Ok(())
}
