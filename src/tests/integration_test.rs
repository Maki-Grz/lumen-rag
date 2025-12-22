use candle_core::Device;
use lumen_rag::{ingestion::segment_text, utils::load_bert_model_and_tokenizer, VectorStore};

mod common;
use common::MockStore;

#[tokio::test]
async fn test_full_pipeline_ingestion() {
    let device = Device::Cpu;
    let (_, tokenizer) =
        load_bert_model_and_tokenizer(&device, None).expect("Failed to load model");

    let mock_store = MockStore::new();

    let text = "This is a test. It should be segmented into passages.";

    let passages = segment_text(text, None, &tokenizer);
    assert!(!passages.is_empty(), "The text should generate passages");

    let result = mock_store.add_passages(passages.clone()).await;

    assert!(result.is_ok());

    let stored_data = mock_store.storage.lock().unwrap();
    assert_eq!(stored_data.len(), passages.len());
    assert_eq!(stored_data[0].text, "This is a test.");
}
