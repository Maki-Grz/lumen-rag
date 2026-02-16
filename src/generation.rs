use crate::types::{LLMMessage, LLMRequest, LLMStreamResponse, Passage};
use futures_util::{Stream, StreamExt};
use std::pin::Pin;

/// Generates an answer using an LLM (e.g., vLLM, Ollama) via an OpenAI-compatible API.
///
/// # Arguments
/// * `question` - The user's question.
/// * `context` - Retrieved passages to act as knowledge.
/// * `llm_uri` - The API endpoint (e.g., "http://localhost:8000/v1/chat/completions").
/// * `model` - The model name (e.g., "mistral-7b").
pub async fn generate_answer(
    question: &str,
    context: &[Passage],
    llm_uri: &str,
    model: &str,
) -> Result<
    Pin<Box<dyn Stream<Item = Result<String, Box<dyn std::error::Error>>> + Send>>,
    Box<dyn std::error::Error>,
> {
    let api_key = std::env::var("LLM_API_KEY").ok();
    let system_prompt_base =
        "You are an expert assistant. Answer strictly based on the provided context.";

    let context_text = context
        .iter()
        .enumerate()
        .map(|(i, passage)| format!("[PASSAGE {}]\n{}", i + 1, passage.text))
        .collect::<Vec<_>>()
        .join("\n\n");

    let system_prompt = format!("{}\n\n{}", system_prompt_base, context_text);
    let user_prompt = format!("QUESTION: {}\n", question);

    let llm_request = LLMRequest {
        model: model.to_string(),
        messages: vec![
            LLMMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            LLMMessage {
                role: "user".to_string(),
                content: user_prompt,
            },
        ],
        stream: true,
    };

    let client = reqwest::Client::new();
    let mut request_builder = client.post(llm_uri).json(&llm_request);

    if let Some(token) = api_key {
        request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
    }

    let response = request_builder.send().await?;

    if !response.status().is_success() {
        return Err(format!("LLM Error: {}", response.status()).into());
    }

    let byte_stream = response.bytes_stream();

    let mapped_stream = byte_stream.filter_map(|chunk| async {
        match chunk {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                let mut parts = Vec::new();

                for line in text.lines() {
                    if let Some(json_str) = line.strip_prefix("data: ") {
                        if json_str == "[DONE]" {
                            return Some(Ok("[DONE]".to_string()));
                        }
                        if let Ok(event) = serde_json::from_str::<LLMStreamResponse>(json_str) {
                            parts.push(event.choices[0].delta.content.clone());
                        }
                    }
                }
                if parts.is_empty() {
                    None
                } else {
                    Some(Ok(parts.join("")))
                }
            }
            Err(e) => Some(Err(Box::new(e) as Box<dyn std::error::Error>)),
        }
    });

    Ok(Box::pin(mapped_stream))
}
