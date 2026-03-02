use crate::analysis::tokenizer;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use rustc_hash::FxHashMap;

/// Threshold in bytes above which we use parallel processing.
#[cfg(feature = "rayon")]
const PARALLEL_THRESHOLD: usize = 64 * 1024;

/// Count word frequencies from text. Case-sensitive.
/// Uses parallel chunk processing for texts larger than 64KB.
pub fn word_frequencies(text: &str) -> FxHashMap<String, usize> {
    #[cfg(feature = "rayon")]
    {
        if text.len() < PARALLEL_THRESHOLD {
            word_frequencies_sequential(text)
        } else {
            word_frequencies_parallel(text)
        }
    }
    #[cfg(not(feature = "rayon"))]
    {
        word_frequencies_sequential(text)
    }
}

/// Count word frequencies from a pre-tokenized slice (avoids re-tokenization).
/// Used by the library API (results.rs).
#[allow(dead_code)]
pub fn word_frequencies_from_slice(words: &[&str]) -> FxHashMap<String, usize> {
    let mut freqs = FxHashMap::default();
    for &word in words {
        *freqs.entry(word.to_string()).or_insert(0) += 1;
    }
    freqs
}

fn word_frequencies_sequential(text: &str) -> FxHashMap<String, usize> {
    let words = tokenizer::words(text);
    let mut freqs = FxHashMap::default();
    for word in words {
        *freqs.entry(word.to_string()).or_insert(0) += 1;
    }
    freqs
}

#[cfg(feature = "rayon")]
fn word_frequencies_parallel(text: &str) -> FxHashMap<String, usize> {
    let num_chunks = rayon::current_num_threads().max(2);
    let chunks = split_at_word_boundaries(text, num_chunks);

    chunks
        .par_iter()
        .map(|chunk| word_frequencies_sequential(chunk))
        .reduce(FxHashMap::default, |mut acc, map| {
            for (word, count) in map {
                *acc.entry(word).or_insert(0) += count;
            }
            acc
        })
}

/// Split text into N chunks at word boundaries (UTF-8 safe).
#[cfg(feature = "rayon")]
fn split_at_word_boundaries(text: &str, n: usize) -> Vec<&str> {
    if n <= 1 || text.is_empty() {
        return vec![text];
    }

    let chunk_size = text.len() / n;
    let mut chunks = Vec::with_capacity(n);
    let mut start = 0;

    for i in 1..n {
        let mut pos = (chunk_size * i).min(text.len());
        // Find a valid char boundary
        while pos < text.len() && !text.is_char_boundary(pos) {
            pos += 1;
        }
        // Advance to next whitespace to avoid splitting words
        while pos < text.len() && !text.as_bytes()[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if pos >= text.len() {
            break;
        }
        chunks.push(&text[start..pos]);
        start = pos;
    }
    if start < text.len() {
        chunks.push(&text[start..]);
    }
    chunks
}

/// Return top N entries sorted by frequency (descending), then alphabetically.
pub fn top_n(freqs: &FxHashMap<String, usize>, n: usize) -> Vec<(&str, usize)> {
    let mut entries: Vec<(&str, usize)> = freqs.iter().map(|(k, &v)| (k.as_str(), v)).collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    entries.truncate(n);
    entries
}

/// Number of unique word types.
pub fn type_count(freqs: &FxHashMap<String, usize>) -> usize {
    freqs.len()
}

/// Total token (word) count.
pub fn token_count(freqs: &FxHashMap<String, usize>) -> usize {
    freqs.values().sum()
}

/// Count of words appearing exactly once (hapax legomena).
pub fn hapax_count(freqs: &FxHashMap<String, usize>) -> usize {
    freqs.values().filter(|&&v| v == 1).count()
}

/// Type-token ratio.
pub fn type_token_ratio(freqs: &FxHashMap<String, usize>) -> f64 {
    let types = type_count(freqs);
    let tokens = token_count(freqs);
    if tokens == 0 {
        return 0.0;
    }
    types as f64 / tokens as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "rayon")]
    #[test]
    fn test_split_at_word_boundaries() {
        let text = "hello world foo bar baz qux";
        let chunks = split_at_word_boundaries(text, 3);
        assert!(chunks.len() >= 2);
        let rejoined: String = chunks.join("");
        // All characters preserved
        assert_eq!(rejoined.replace(' ', "").len() + rejoined.matches(' ').count(), text.len());
    }

    #[cfg(feature = "rayon")]
    #[test]
    fn test_parallel_matches_sequential() {
        // Create text >64KB to exercise parallel path
        let sentence = "The quick brown fox jumps over the lazy dog. ";
        let text: String = sentence.repeat(2000); // ~90KB
        assert!(text.len() > PARALLEL_THRESHOLD);

        let seq = word_frequencies_sequential(&text);
        let par = word_frequencies_parallel(&text);

        assert_eq!(seq.len(), par.len());
        for (word, count) in &seq {
            assert_eq!(par.get(word), Some(count), "mismatch for '{}'", word);
        }
    }
}
