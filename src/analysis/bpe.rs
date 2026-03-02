use anyhow::{anyhow, Result};

/// Supported BPE tokenizer models.
pub enum TokenizerModel {
    /// GPT-4 / GPT-3.5-turbo (cl100k_base)
    Gpt4,
    /// GPT-4o (o200k_base)
    Gpt4o,
    /// GPT-3 / text-davinci (p50k_base)
    Gpt3,
}

impl TokenizerModel {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "gpt4" => Ok(Self::Gpt4),
            "gpt4o" => Ok(Self::Gpt4o),
            "gpt3" => Ok(Self::Gpt3),
            _ => Err(anyhow!("Unknown model '{}'. Use: gpt4, gpt4o, gpt3", s)),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Gpt4 => "BPE (GPT-4)",
            Self::Gpt4o => "BPE (GPT-4o)",
            Self::Gpt3 => "BPE (GPT-3)",
        }
    }
}

/// Result of BPE tokenization.
pub struct BpeResult {
    pub model: String,
    pub token_count: usize,
}

/// Count tokens for a specific model.
pub fn count_tokens(text: &str, model: &TokenizerModel) -> Result<BpeResult> {
    let (bpe, label) = match model {
        TokenizerModel::Gpt4 => (
            tiktoken_rs::cl100k_base().map_err(|e| anyhow!("{e}"))?,
            "GPT-4",
        ),
        TokenizerModel::Gpt4o => (
            tiktoken_rs::o200k_base().map_err(|e| anyhow!("{e}"))?,
            "GPT-4o",
        ),
        TokenizerModel::Gpt3 => (
            tiktoken_rs::p50k_base().map_err(|e| anyhow!("{e}"))?,
            "GPT-3",
        ),
    };
    let tokens = bpe.encode_with_special_tokens(text);
    Ok(BpeResult {
        model: label.to_string(),
        token_count: tokens.len(),
    })
}

/// Count tokens for all supported models.
pub fn count_all_models(text: &str) -> Result<Vec<BpeResult>> {
    let models = [TokenizerModel::Gpt4, TokenizerModel::Gpt4o, TokenizerModel::Gpt3];
    models.iter().map(|m| count_tokens(text, m)).collect()
}
