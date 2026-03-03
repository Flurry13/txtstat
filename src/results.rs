//! Typed result structs and compute functions for programmatic API access.
//!
//! These structs return proper typed values (not strings) and are suitable
//! for use by Python (PyO3), WASM, and other language bindings.

use crate::analysis::{counter, entropy, lm, ngram, readability, tokenizer};
use rustc_hash::FxHashSet;
use serde::Serialize;

// ---------------------------------------------------------------------------
// Result structs
// ---------------------------------------------------------------------------

#[derive(Serialize, Debug, Clone)]
pub struct StatsResult {
    pub tokens: usize,
    pub types: usize,
    pub characters: usize,
    pub sentences: usize,
    pub type_token_ratio: f64,
    pub hapax_legomena: usize,
    pub hapax_percentage: f64,
    pub avg_sentence_length: f64,
    pub stopwords_removed: Option<usize>,
}

#[derive(Serialize, Debug, Clone)]
pub struct NgramEntry {
    pub ngram: String,
    pub frequency: usize,
    pub relative_pct: f64,
}

#[derive(Serialize, Debug, Clone)]
pub struct EntropyResult {
    pub h1: f64,
    pub h2: f64,
    pub h3: f64,
    pub entropy_rate: f64,
    pub vocabulary_size: usize,
    pub redundancy: f64,
}

#[derive(Serialize, Debug, Clone)]
pub struct ReadabilityResult {
    pub flesch_kincaid_grade: f64,
    pub flesch_reading_ease: f64,
    pub coleman_liau: f64,
    pub gunning_fog: f64,
    pub smog: f64,
}

#[derive(Serialize, Debug, Clone)]
pub struct PerplexityResult {
    pub order: usize,
    pub vocab_size: usize,
    pub ngram_counts: Vec<usize>,
    pub smoothing: String,
    pub perplexity: f64,
}

#[derive(Serialize, Debug, Clone)]
pub struct LangResult {
    pub language: String,
    pub code: String,
    pub script: String,
    pub confidence: f64,
    pub is_reliable: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct TokensResult {
    pub whitespace: usize,
    pub sentences: usize,
    pub characters: usize,
    pub bpe_gpt4: Option<usize>,
    pub bpe_gpt4o: Option<usize>,
    pub bpe_gpt3: Option<usize>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ZipfEntry {
    pub rank: usize,
    pub word: String,
    pub frequency: usize,
}

#[derive(Serialize, Debug, Clone)]
pub struct ZipfResult {
    pub entries: Vec<ZipfEntry>,
    pub alpha: f64,
    pub r_squared: f64,
}

// ---------------------------------------------------------------------------
// Compute functions
// ---------------------------------------------------------------------------

/// Compute basic text statistics (tokens, types, TTR, hapax, etc.).
pub fn compute_stats(text: &str, stopwords: Option<&FxHashSet<String>>) -> StatsResult {
    let words = tokenizer::words(text);
    let filtered: Vec<&str>;
    let stopwords_removed;

    let effective_words: &[&str] = if let Some(sw) = stopwords {
        filtered = crate::utils::stopwords::filter_words(&words, sw);
        stopwords_removed = Some(words.len() - filtered.len());
        &filtered
    } else {
        stopwords_removed = None;
        &words
    };

    let freqs = counter::word_frequencies_from_slice(effective_words);
    let sentences = tokenizer::sentence_count(text);
    let tokens = counter::token_count(&freqs);
    let types = counter::type_count(&freqs);
    let hapax = counter::hapax_count(&freqs);

    StatsResult {
        tokens,
        types,
        characters: tokenizer::char_count(text),
        sentences,
        type_token_ratio: counter::type_token_ratio(&freqs),
        hapax_legomena: hapax,
        hapax_percentage: if types > 0 {
            hapax as f64 / types as f64 * 100.0
        } else {
            0.0
        },
        avg_sentence_length: if sentences > 0 {
            tokens as f64 / sentences as f64
        } else {
            0.0
        },
        stopwords_removed,
    }
}

/// Compute n-gram frequencies from text.
///
/// Returns a sorted (by frequency descending) vector of `NgramEntry` values,
/// truncated to `top` entries. Optionally filters by `min_freq` threshold.
pub fn compute_ngrams(
    text: &str,
    n: usize,
    top: usize,
    min_freq: Option<usize>,
    case_insensitive: bool,
    stopwords: Option<&FxHashSet<String>>,
) -> Vec<NgramEntry> {
    if n == 0 {
        return Vec::new();
    }
    let words = tokenizer::words(text);

    // Apply stopword filtering before n-gram extraction
    let filtered: Vec<&str>;
    let words_ref: &[&str] = if let Some(sw) = stopwords {
        filtered = crate::utils::stopwords::filter_words(&words, sw);
        &filtered
    } else {
        &words
    };

    let freqs = if case_insensitive {
        let lowered: Vec<String> = words_ref.iter().map(|w| w.to_lowercase()).collect();
        let refs: Vec<&str> = lowered.iter().map(|s| s.as_str()).collect();
        ngram::ngram_frequencies(&refs, n)
    } else {
        ngram::ngram_frequencies(words_ref, n)
    };

    let total: usize = freqs.values().sum();

    let mut entries: Vec<(&str, usize)> = freqs.iter().map(|(k, &v)| (k.as_str(), v)).collect();

    if let Some(min) = min_freq {
        entries.retain(|&(_, freq)| freq >= min);
    }

    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    entries.truncate(top);

    entries
        .into_iter()
        .map(|(ngram_str, freq)| {
            let pct = if total > 0 {
                freq as f64 / total as f64 * 100.0
            } else {
                0.0
            };
            NgramEntry {
                ngram: ngram_str.to_string(),
                frequency: freq,
                relative_pct: pct,
            }
        })
        .collect()
}

/// Compute Shannon entropy at orders 1-3, entropy rate, and redundancy.
pub fn compute_entropy(text: &str) -> EntropyResult {
    let unigram_freqs = counter::word_frequencies(text);
    let words = tokenizer::words(text);
    let bigram_freqs = ngram::ngram_frequencies(&words, 2);
    let trigram_freqs = ngram::ngram_frequencies(&words, 3);
    let h1 = entropy::shannon_entropy(&unigram_freqs);
    let h2 = entropy::shannon_entropy(&bigram_freqs);
    let h3 = entropy::shannon_entropy(&trigram_freqs);
    let rate = entropy::entropy_rate(h2, h3);
    let vocab_size = unigram_freqs.len();
    EntropyResult {
        h1,
        h2,
        h3,
        entropy_rate: rate,
        vocabulary_size: vocab_size,
        redundancy: entropy::redundancy(rate, vocab_size),
    }
}

/// Compute readability scores (Flesch-Kincaid, Flesch Reading Ease, etc.).
pub fn compute_readability(text: &str) -> ReadabilityResult {
    let m = readability::compute_metrics(text);
    ReadabilityResult {
        flesch_kincaid_grade: readability::flesch_kincaid_grade(&m),
        flesch_reading_ease: readability::flesch_reading_ease(&m),
        coleman_liau: readability::coleman_liau(&m),
        gunning_fog: readability::gunning_fog(&m),
        smog: readability::smog(&m),
    }
}

/// Detect the language of the given text.
/// Returns `None` if the text is too short or ambiguous.
pub fn compute_lang(text: &str) -> Option<LangResult> {
    crate::analysis::detect::detect(text).map(|r| LangResult {
        language: r.language,
        code: r.code,
        script: r.script,
        confidence: r.confidence,
        is_reliable: r.is_reliable,
    })
}

/// Compute perplexity of the text under an n-gram language model.
///
/// `smoothing` should be one of: `"none"`, `"laplace"`, `"backoff"`.
/// `k` is the smoothing constant for add-k smoothing.
pub fn compute_perplexity(text: &str, order: usize, smoothing: &str, k: f64) -> PerplexityResult {
    let words = tokenizer::words(text);
    let token_refs: Vec<&str> = words.iter().copied().collect();
    let model = lm::NgramLM::train(&token_refs, order);
    let stats = model.stats();
    let sm = match smoothing {
        "none" => lm::Smoothing::None,
        "laplace" => lm::Smoothing::AddK(k),
        "backoff" => lm::Smoothing::StupidBackoff(0.4),
        _ => lm::Smoothing::AddK(k),
    };
    let pp = model.perplexity(&token_refs, &sm);
    let smoothing_label = match &sm {
        lm::Smoothing::None => "None (MLE)".to_string(),
        lm::Smoothing::AddK(k) => format!("Add-k (k={k})"),
        lm::Smoothing::StupidBackoff(a) => format!("Stupid Backoff (α={a})"),
    };
    PerplexityResult {
        order: stats.order,
        vocab_size: stats.vocab_size,
        ngram_counts: stats.ngram_counts,
        smoothing: smoothing_label,
        perplexity: pp,
    }
}

/// Compute token counts from various tokenizers.
///
/// BPE counts are only available when compiled with the `tiktoken-rs` feature.
/// When the feature is absent, the BPE fields will be `None`.
pub fn compute_tokens(text: &str, include_bpe: bool) -> TokensResult {
    let words = tokenizer::words(text);
    let sentences = tokenizer::sentence_count(text);
    let chars = tokenizer::char_count(text);

    let (bpe_gpt4, bpe_gpt4o, bpe_gpt3) = if include_bpe {
        #[cfg(feature = "tiktoken-rs")]
        {
            use crate::analysis::bpe;
            let gpt4 = bpe::count_tokens(text, &bpe::TokenizerModel::Gpt4)
                .ok()
                .map(|r| r.token_count);
            let gpt4o = bpe::count_tokens(text, &bpe::TokenizerModel::Gpt4o)
                .ok()
                .map(|r| r.token_count);
            let gpt3 = bpe::count_tokens(text, &bpe::TokenizerModel::Gpt3)
                .ok()
                .map(|r| r.token_count);
            (gpt4, gpt4o, gpt3)
        }
        #[cfg(not(feature = "tiktoken-rs"))]
        {
            (None, None, None)
        }
    } else {
        (None, None, None)
    };

    TokensResult {
        whitespace: words.len(),
        sentences,
        characters: chars,
        bpe_gpt4,
        bpe_gpt4o,
        bpe_gpt3,
    }
}

/// Compute Zipf's law analysis: word frequency distribution with exponent.
///
/// Returns the top `top` entries by frequency along with the Zipf exponent (alpha)
/// and R-squared goodness of fit.
pub fn compute_zipf(text: &str, top: usize) -> ZipfResult {
    use crate::analysis::zipf;

    let freqs = counter::word_frequencies(text);
    let sorted = counter::top_n(&freqs, freqs.len());

    let rank_freq: Vec<(usize, usize)> = sorted
        .iter()
        .enumerate()
        .map(|(i, &(_, f))| (i + 1, f))
        .collect();

    let (alpha, r_squared) = zipf::zipf_exponent(&rank_freq);

    let display_count = top.min(sorted.len());
    let entries = sorted
        .iter()
        .take(display_count)
        .enumerate()
        .map(|(i, &(word, freq))| ZipfEntry {
            rank: i + 1,
            word: word.to_string(),
            frequency: freq,
        })
        .collect();

    ZipfResult {
        entries,
        alpha,
        r_squared,
    }
}
