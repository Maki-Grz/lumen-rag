use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use lumen_rag::stores::hana::HanaStore;
use lumen_rag::{
    types::{IngestRequest, Passage, QuestionRequest},
    VectorStore,
};
use std::env;

#[post("/ingest")]
async fn ingest(store: web::Data<HanaStore>, req: web::Json<IngestRequest>) -> impl Responder {
    let passage = Passage {
        id: None,
        text: req.text.clone(),
        embedding: vec![0.1, 0.2, 0.3],
        metadata: req.metadata.clone(),
        hash: None,
    };

    match store.add_passages(vec![passage]).await {
        Ok(ids) => HttpResponse::Ok().json(ids),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/search")]
async fn search(store: web::Data<HanaStore>, _req: web::Json<QuestionRequest>) -> impl Responder {
    let query_embedding = vec![0.1, 0.2, 0.3];
    match store.search(&query_embedding, 5).await {
        Ok(passages) => HttpResponse::Ok().json(passages),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let hana_url = env::var("HANA_URL").expect("HANA_URL must be set");
    let table_name = env::var("HANA_TABLE").unwrap_or_else(|_| "LUMEN_RAG".to_string());

    let store = HanaStore::new(hana_url, table_name).expect("Failed to connect to HANA");
    let store_data = web::Data::new(store);

    HttpServer::new(move || {
        App::new()
            .app_data(store_data.clone())
            .service(ingest)
            .service(search)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
