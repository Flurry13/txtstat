use lexis::analysis::lm::{NgramLM, Smoothing};

fn sample_tokens() -> Vec<&'static str> {
    "the cat sat on the mat the cat sat on the hat"
        .split_whitespace()
        .collect()
}

#[test]
fn test_train_counts() {
    let tokens = sample_tokens();
    let lm = NgramLM::train(&tokens, 3);
    let stats = lm.stats();
    assert_eq!(stats.order, 3);
    assert_eq!(stats.vocab_size, 6); // the, cat, sat, on, mat, hat
    assert!(stats.ngram_counts[0] > 0); // unigrams
    assert!(stats.ngram_counts[1] > 0); // bigrams
    assert!(stats.ngram_counts[2] > 0); // trigrams
}

#[test]
fn test_mle_unigram() {
    let tokens = sample_tokens();
    let lm = NgramLM::train(&tokens, 2);
    // "the" appears 4 times out of 12 tokens
    let p = lm.prob("the", &[], &Smoothing::None);
    assert!((p - 4.0 / 12.0).abs() < 1e-10);
}

#[test]
fn test_mle_bigram() {
    let tokens = sample_tokens();
    let lm = NgramLM::train(&tokens, 2);
    // P(cat | the) = C("the cat") / C("the") = 2 / 4
    let p = lm.prob("cat", &["the"], &Smoothing::None);
    assert!((p - 2.0 / 4.0).abs() < 1e-10);
}

#[test]
fn test_mle_unseen_is_zero() {
    let tokens = sample_tokens();
    let lm = NgramLM::train(&tokens, 2);
    let p = lm.prob("dog", &["the"], &Smoothing::None);
    assert_eq!(p, 0.0);
}

#[test]
fn test_laplace_nonzero_for_unseen() {
    let tokens = sample_tokens();
    let lm = NgramLM::train(&tokens, 2);
    let p = lm.prob("dog", &["the"], &Smoothing::AddK(1.0));
    assert!(p > 0.0, "Laplace should give non-zero for unseen");
}

#[test]
fn test_laplace_unigram() {
    let tokens = sample_tokens();
    let lm = NgramLM::train(&tokens, 2);
    // P_laplace(the) = (4 + 1) / (12 + 1*6) = 5/18
    let p = lm.prob("the", &[], &Smoothing::AddK(1.0));
    assert!((p - 5.0 / 18.0).abs() < 1e-10);
}

#[test]
fn test_stupid_backoff_uses_highest_order() {
    let tokens = sample_tokens();
    let lm = NgramLM::train(&tokens, 3);
    // "the cat" exists as a bigram, so trigram context ["on", "the"] should find "the cat" -> backs off
    // But "cat" given ["the"] directly: bigram "the cat" count=2, "the" count=3 -> 2/3
    let p_backoff = lm.prob("cat", &["the"], &Smoothing::StupidBackoff(0.4));
    let p_mle = lm.prob("cat", &["the"], &Smoothing::None);
    // With one-word context and order 3, max_ctx=1, trying ctx_len=1 first (bigram) -> 2/3
    // No backoff needed since bigram exists, so alpha^0 * MLE
    assert!((p_backoff - p_mle).abs() < 1e-10);
}

#[test]
fn test_stupid_backoff_decays() {
    let tokens = sample_tokens();
    let lm = NgramLM::train(&tokens, 3);
    // Ask for unseen bigram context, should back off to unigram with alpha penalty
    let p = lm.prob("the", &["xyz"], &Smoothing::StupidBackoff(0.4));
    let p_unigram = lm.prob("the", &[], &Smoothing::None);
    // Should be alpha * unigram probability since bigram doesn't exist
    assert!((p - 0.4 * p_unigram).abs() < 1e-10);
}

#[test]
fn test_perplexity_finite() {
    let tokens = sample_tokens();
    let lm = NgramLM::train(&tokens, 3);
    let pp = lm.perplexity(&tokens, &Smoothing::AddK(1.0));
    assert!(pp.is_finite(), "Perplexity should be finite");
    assert!(pp > 0.0, "Perplexity should be positive");
}

#[test]
fn test_perplexity_uniform_text() {
    // For perfectly uniform text, perplexity should approach vocab size
    let tokens: Vec<&str> = vec!["a", "b", "c", "d"];
    let repeated: Vec<&str> = tokens.iter().cycle().take(400).copied().collect();
    let lm = NgramLM::train(&repeated, 1);
    let pp = lm.perplexity(&repeated, &Smoothing::None);
    assert!(
        (pp - 4.0).abs() < 0.5,
        "Uniform unigram perplexity should be ~vocab_size, got {}",
        pp
    );
}

#[test]
fn test_perplexity_empty() {
    let tokens = sample_tokens();
    let lm = NgramLM::train(&tokens, 2);
    let pp = lm.perplexity(&[], &Smoothing::AddK(1.0));
    assert!(pp.is_infinite());
}

#[test]
fn test_single_word() {
    let tokens = vec!["hello"];
    let lm = NgramLM::train(&tokens, 2);
    let stats = lm.stats();
    assert_eq!(stats.vocab_size, 1);
    assert_eq!(stats.ngram_counts[0], 1);
    // Only unigrams exist (can't form bigrams from single token)
    assert_eq!(stats.ngram_counts[1], 0);
}

#[test]
fn test_log_prob_negative() {
    let tokens = sample_tokens();
    let lm = NgramLM::train(&tokens, 2);
    let lp = lm.log_prob("the", &[], &Smoothing::None);
    assert!(lp < 0.0, "log prob of probable word should be negative");
}

#[test]
fn test_log_prob_unseen() {
    let tokens = sample_tokens();
    let lm = NgramLM::train(&tokens, 2);
    let lp = lm.log_prob("xyz", &[], &Smoothing::None);
    assert!(lp.is_infinite() && lp < 0.0);
}
