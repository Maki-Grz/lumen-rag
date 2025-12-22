use anyhow::{anyhow, Error as E, Result};
use candle_core::{DType, Device, Tensor};
use candle_transformers::models::bert::BertModel;
use tokenizers::Tokenizer;
use twox_hash::XxHash3_64;

/// Computes the BERT embedding for a given text.
/// Optimized to use scalar division for normalization to avoid shape mismatches.
pub async fn compute_text_embedding(
    model: &BertModel,
    tokenizer: &Tokenizer,
    device: &Device,
    text: &str,
) -> Result<Tensor> {
    let encoding = tokenizer.encode(text, true).map_err(|e| anyhow!(e))?;
    let ids = encoding.get_ids().to_vec();
    let mask = encoding.get_attention_mask().to_vec();

    // Shapes: [1, seq_len]
    let ids_tensor = Tensor::new(&*ids, device)?.unsqueeze(0)?;
    let mask_tensor = Tensor::new(&*mask, device)?.unsqueeze(0)?;

    let output = model.forward(&ids_tensor, &mask_tensor, None)?;

    let mask_f = mask_tensor.to_dtype(DType::F32)?;
    let mask_sum = mask_f.sum_all()?.to_scalar::<f32>()?;

    let mask_3d = mask_f.unsqueeze(2)?;
    let masked_hidden = output.broadcast_mul(&mask_3d)?;
    let summed = masked_hidden.sum(1)?; // Shape: [1, 384]

    let mean_pooled = (summed / (mask_sum as f64))?;

    let norm = mean_pooled
        .clone()
        .powf(2.0)?
        .sum_all()?
        .to_scalar::<f32>()?
        .sqrt();

    let normalized = (mean_pooled / (norm as f64))?;

    Ok(normalized)
}

/// Loads a BERT model and tokenizer from Hugging Face Hub.
/// Allows specifying a model_id optionally.
pub fn load_bert_model_and_tokenizer(
    device: &Device,
    model_id: Option<&str>,
) -> Result<(BertModel, Tokenizer), E> {
    use candle_nn::VarBuilder;
    use candle_transformers::models::bert::{Config, HiddenAct, DTYPE};
    use hf_hub::{api::sync::Api, Repo, RepoType};

    let model_id =
        model_id.unwrap_or("sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2");
    let revision = "main";

    let repo = Repo::with_revision(model_id.to_string(), RepoType::Model, revision.parse()?);
    let api = Api::new()?.repo(repo);

    let config_file = api.get("config.json")?;
    let tokenizer_file = api.get("tokenizer.json")?;
    let weights_file = api.get("model.safetensors")?;

    let config_str = std::fs::read_to_string(config_file)?;
    let mut config: Config = serde_json::from_str(&config_str)?;
    config.hidden_act = HiddenAct::GeluApproximate;

    let tokenizer = Tokenizer::from_file(tokenizer_file).map_err(|e| anyhow!(e))?;

    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_file], DTYPE, device)? };
    let model = BertModel::load(vb, &config)?;

    Ok((model, tokenizer))
}

pub fn compute_hash(s: &str) -> u64 {
    XxHash3_64::oneshot(s.as_bytes())
}
