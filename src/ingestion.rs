use crate::types::{Metadata, Passage};
use regex::Regex;
use tokenizers::Tokenizer;

/// Counts the number of tokens in a string using the provided tokenizer.
fn count_tokens(tokenizer: &Tokenizer, text: &str) -> usize {
    tokenizer.encode(text, true).map(|e| e.len()).unwrap_or(0)
}

/// Keeps the last `n` tokens of a text string. Used for overlap.
fn keep_last_tokens(tokenizer: &Tokenizer, text: &str, n: usize) -> String {
    let encoding = tokenizer.encode(text, true).unwrap();
    let ids = encoding.get_ids();
    let start = if ids.len() > n { ids.len() - n } else { 0 };
    let slice = &ids[start..];
    tokenizer.decode(slice, true).unwrap_or_default()
}

/// Splits text into sections based on numbered lists (e.g., "1. Introduction").
fn split_sections(text: &str) -> Vec<String> {
    let re = Regex::new(r"(?m)^(\d+\.\s.*)").unwrap();
    let mut sections = Vec::new();
    let mut current = String::new();

    for line in text.lines() {
        if re.is_match(line) && !current.trim().is_empty() {
            sections.push(current.trim().to_string());
            current.clear();
        }
        current.push_str(line);
        current.push('\n');
    }

    if !current.trim().is_empty() {
        sections.push(current.trim().to_string());
    }

    sections
}

fn make_passage(text: &str, metadata: &Option<Metadata>) -> Passage {
    Passage {
        id: None,
        text: text.to_string(),
        embedding: vec![],
        metadata: metadata.clone(),
        hash: None,
    }
}

/// Cleans raw text by normalizing newlines and trimming whitespace.
fn clean_text(text: &str) -> String {
    text.replace("\r\n", "\n")
        .replace('\r', "\n")
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn split_sentences(text: &str) -> Vec<String> {
    let re = Regex::new(r"(?m)([^.!?]*[.!?](?:\s|$))").unwrap();
    let mut sentences = Vec::new();
    let mut last_index = 0;

    for mat in re.find_iter(text) {
        let sentence = text[last_index..mat.end()].trim();
        if !sentence.is_empty() && sentence.len() > 10 {
            sentences.push(sentence.to_string());
        }
        last_index = mat.end();
    }

    if last_index < text.len() {
        let rest = text[last_index..].trim();
        if !rest.is_empty() {
            sentences.push(rest.to_string());
        }
    }

    // Fallback if regex failed to find sentence structures
    if sentences.is_empty() && !text.trim().is_empty() {
        let words: Vec<&str> = text.split_whitespace().collect();
        let chunk_size = 25;
        for chunk in words.chunks(chunk_size) {
            sentences.push(chunk.join(" "));
        }
    }

    sentences
}

fn split_large_text(
    text: &str,
    tokenizer: &Tokenizer,
    max_tokens: usize,
    overlap_tokens: usize,
) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut chunks = Vec::new();
    let mut start_idx = 0;

    while start_idx < words.len() {
        let mut end_idx = start_idx;
        let mut current_chunk = String::new();

        while end_idx < words.len() {
            let test_text = if current_chunk.is_empty() {
                words[end_idx].to_string()
            } else {
                format!("{} {}", current_chunk, words[end_idx])
            };

            if count_tokens(tokenizer, &test_text) > max_tokens && !current_chunk.is_empty() {
                break;
            }

            current_chunk = test_text;
            end_idx += 1;
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        if end_idx < words.len() {
            // Check overlap
            let overlap_text = keep_last_tokens(tokenizer, chunks.last().unwrap(), overlap_tokens);
            let overlap_words: Vec<&str> = overlap_text.split_whitespace().collect();
            // Move start_idx back by the number of overlapping words
            start_idx = end_idx.saturating_sub(overlap_words.len());
            // Avoid infinite loop if overlap is entire chunk
            if start_idx == end_idx {
                start_idx += 1;
            }
        } else {
            break;
        }
    }
    chunks
}

/// Main entry point for text segmentation.
/// Splits text into semantic chunks suitable for embedding.
pub fn segment_text(text: &str, metadata: Option<Metadata>, tokenizer: &Tokenizer) -> Vec<Passage> {
    let max_tokens = 200;
    let overlap_tokens = 30;
    let min_tokens = 3;

    let text = clean_text(text);
    let sections = split_sections(&text);
    let mut passages = Vec::new();

    for section in sections {
        let section_token_count = count_tokens(tokenizer, &section);
        if section_token_count > max_tokens * 3 {
            let chunks = split_large_text(&section, tokenizer, max_tokens, overlap_tokens);
            for chunk in chunks {
                if count_tokens(tokenizer, &chunk) >= min_tokens {
                    passages.push(make_passage(&chunk, &metadata));
                }
            }
            continue;
        }

        let paragraphs: Vec<&str> = section
            .split("\n\n")
            .filter(|p| !p.trim().is_empty())
            .collect();

        for paragraph in paragraphs {
            let paragraph_tokens = count_tokens(tokenizer, paragraph);

            if paragraph_tokens > max_tokens {
                let chunks = split_large_text(paragraph, tokenizer, max_tokens, overlap_tokens);
                for chunk in chunks {
                    if count_tokens(tokenizer, &chunk) >= min_tokens {
                        passages.push(make_passage(&chunk, &metadata));
                    }
                }
                continue;
            }

            let sentences = split_sentences(paragraph);
            let mut buffer = String::new();
            let mut token_count = 0;

            for sentence in sentences {
                let sentence_tokens = count_tokens(tokenizer, &sentence);

                if sentence_tokens > max_tokens {
                    if !buffer.is_empty() && token_count >= min_tokens {
                        passages.push(make_passage(&buffer, &metadata));
                        buffer.clear();
                        token_count = 0;
                    }
                    let sentence_chunks =
                        split_large_text(&sentence, tokenizer, max_tokens, overlap_tokens);
                    for chunk in sentence_chunks {
                        if count_tokens(tokenizer, &chunk) >= min_tokens {
                            passages.push(make_passage(&chunk, &metadata));
                        }
                    }
                    continue;
                }

                if token_count + sentence_tokens > max_tokens {
                    if token_count >= min_tokens {
                        passages.push(make_passage(&buffer, &metadata));
                    }

                    let overlap_text = keep_last_tokens(tokenizer, &buffer, overlap_tokens);
                    buffer = if overlap_text.is_empty() {
                        sentence.clone()
                    } else {
                        format!("{} {}", overlap_text, sentence)
                    };
                    token_count = count_tokens(tokenizer, &buffer);
                } else {
                    if !buffer.is_empty() {
                        buffer.push(' ');
                    }
                    buffer.push_str(&sentence);
                    token_count += sentence_tokens;
                }
            }

            if !buffer.is_empty() && token_count >= min_tokens {
                passages.push(make_passage(&buffer, &metadata));
            }
        }
    }

    if passages.is_empty() && !text.trim().is_empty() {
        passages.push(make_passage(&text, &metadata));
    }

    passages
}

#[cfg(test)]
mod tests {
    use super::*;
    use hf_hub::{api::sync::Api, Repo, RepoType};
    use tokenizers::Tokenizer;

    /// Helper to fetch a real tokenizer for tests (avoids mocking complexity)
    fn get_tokenizer() -> Tokenizer {
        let model_id = "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2";
        let repo = Repo::with_revision(model_id.to_string(), RepoType::Model, "main".to_string());
        let api = Api::new().expect("Failed to create API").repo(repo);
        let tokenizer_file = api
            .get("tokenizer.json")
            .expect("Failed to download tokenizer");

        Tokenizer::from_file(tokenizer_file).expect("Failed to load tokenizer")
    }

    #[test]
    fn test_clean_text() {
        let raw = "Hello  \r\n World";
        assert_eq!(clean_text(raw), "Hello\nWorld");
    }

    #[test]
    #[ignore] // Ignored by default in CI to avoid network calls, run with `cargo test -- --ignored`
    fn test_segmentation_logic() {
        let tokenizer = get_tokenizer();
        let text = "First sentence. Second sentence. ".repeat(20);
        let passages = segment_text(&text, None, &tokenizer);

        assert!(!passages.is_empty());
        // Should be split into multiple chunks due to length
        assert!(passages.len() > 1);
    }
}
