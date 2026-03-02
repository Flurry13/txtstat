use lexis::results;

#[test]
fn test_compute_stats() {
    let r = results::compute_stats("hello world hello", None);
    assert_eq!(r.tokens, 3);
    assert_eq!(r.types, 2);
    assert!(r.type_token_ratio > 0.0);
}

#[test]
fn test_compute_stats_with_stopwords() {
    let mut sw = rustc_hash::FxHashSet::default();
    sw.insert("the".to_string());
    sw.insert("a".to_string());
    let r = results::compute_stats("the cat sat on a mat", Some(&sw));
    assert_eq!(r.stopwords_removed, Some(2));
    assert_eq!(r.tokens, 4); // "cat", "sat", "on", "mat"
}

#[test]
fn test_compute_stats_hapax() {
    let r = results::compute_stats("hello world hello", None);
    // "world" appears once → 1 hapax; 2 types total → 50%
    assert_eq!(r.hapax_legomena, 1);
    assert!((r.hapax_percentage - 50.0).abs() < 0.01);
}

#[test]
fn test_compute_ngrams() {
    let entries = results::compute_ngrams("the cat sat on the mat", 2, 10, None, false, None);
    assert!(!entries.is_empty());
    // "the cat" and "the mat" are bigrams; frequencies should be present
    let total_freq: usize = entries.iter().map(|e| e.frequency).sum();
    assert!(total_freq > 0);
    // Relative percentages should sum to roughly 100%
    let total_pct: f64 = entries.iter().map(|e| e.relative_pct).sum();
    assert!((total_pct - 100.0).abs() < 0.01);
}

#[test]
fn test_compute_ngrams_case_insensitive() {
    let entries = results::compute_ngrams("Hello hello HELLO", 1, 10, None, true, None);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].frequency, 3);
}

#[test]
fn test_compute_entropy() {
    let r = results::compute_entropy("the cat sat on the mat");
    assert!(r.h1 > 0.0);
    assert!(r.vocabulary_size > 0);
}

#[test]
fn test_compute_readability() {
    let text = "The quick brown fox jumps over the lazy dog. \
                Simple sentences are easy to read. \
                This is a basic readability test.";
    let r = results::compute_readability(text);
    // Flesch Reading Ease should be in a reasonable range
    assert!(r.flesch_reading_ease > 0.0);
    // All scores should be finite
    assert!(r.flesch_kincaid_grade.is_finite());
    assert!(r.coleman_liau.is_finite());
    assert!(r.gunning_fog.is_finite());
    assert!(r.smog.is_finite());
}

#[test]
fn test_compute_lang() {
    let r = results::compute_lang(
        "The quick brown fox jumps over the lazy dog and runs around the park all day long",
    )
    .unwrap();
    assert_eq!(r.code, "eng");
}

#[test]
fn test_compute_perplexity() {
    let r = results::compute_perplexity("the cat sat on the mat the cat sat", 2, "laplace", 1.0);
    assert!(r.perplexity.is_finite());
    assert!(r.perplexity > 0.0);
}

#[test]
fn test_compute_perplexity_smoothing_labels() {
    let r1 = results::compute_perplexity("the cat sat on the mat", 2, "none", 1.0);
    assert_eq!(r1.smoothing, "None (MLE)");

    let r2 = results::compute_perplexity("the cat sat on the mat", 2, "laplace", 1.0);
    assert_eq!(r2.smoothing, "Add-k (k=1)");

    let r3 = results::compute_perplexity("the cat sat on the mat", 2, "backoff", 1.0);
    assert_eq!(r3.smoothing, "Stupid Backoff (α=0.4)");
}

#[test]
fn test_compute_tokens() {
    let r = results::compute_tokens("hello world", false);
    assert_eq!(r.whitespace, 2);
    assert_eq!(r.sentences, 1);
    assert!(r.characters > 0);
    assert!(r.bpe_gpt4.is_none());
    assert!(r.bpe_gpt4o.is_none());
    assert!(r.bpe_gpt3.is_none());
}

#[test]
fn test_compute_zipf() {
    let r = results::compute_zipf("the cat sat on the mat the dog sat", 5);
    assert!(!r.entries.is_empty());
    // "the" should be rank 1
    assert_eq!(r.entries[0].rank, 1);
    assert_eq!(r.entries[0].word, "the");
    assert_eq!(r.entries[0].frequency, 3);
    // Alpha and R² should be finite
    assert!(r.alpha.is_finite());
    assert!(r.r_squared.is_finite());
}

#[test]
fn test_compute_zipf_top_limit() {
    let r = results::compute_zipf("a b c d e f g", 3);
    assert!(r.entries.len() <= 3);
}

#[test]
fn test_stats_serializable() {
    let r = results::compute_stats("hello world", None);
    let json = serde_json::to_string(&r).unwrap();
    assert!(json.contains("\"tokens\":2"));
}
