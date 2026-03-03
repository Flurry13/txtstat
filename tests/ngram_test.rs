use corpa::analysis::ngram;

#[test]
fn test_bigrams() {
    let tokens = vec!["the", "cat", "sat"];
    let bigrams: Vec<String> = ngram::ngrams(&tokens, 2).collect();
    assert_eq!(bigrams, vec!["the cat", "cat sat"]);
}

#[test]
fn test_trigrams() {
    let tokens = vec!["the", "cat", "sat", "on"];
    let trigrams: Vec<String> = ngram::ngrams(&tokens, 3).collect();
    assert_eq!(trigrams, vec!["the cat sat", "cat sat on"]);
}

#[test]
fn test_unigrams() {
    let tokens = vec!["hello", "world"];
    let unigrams: Vec<String> = ngram::ngrams(&tokens, 1).collect();
    assert_eq!(unigrams, vec!["hello", "world"]);
}

#[test]
fn test_ngrams_too_short() {
    let tokens = vec!["hello"];
    let bigrams: Vec<String> = ngram::ngrams(&tokens, 2).collect();
    assert!(bigrams.is_empty());
}

#[test]
fn test_ngram_frequencies() {
    let tokens = vec!["the", "cat", "the", "cat", "the", "dog"];
    let freqs = ngram::ngram_frequencies(&tokens, 2);
    assert_eq!(freqs.get("the cat"), Some(&2));
    assert_eq!(freqs.get("cat the"), Some(&2));
    assert_eq!(freqs.get("the dog"), Some(&1));
}

#[test]
fn test_ngram_zero_returns_empty() {
    let tokens: Vec<&str> = vec!["hello", "world"];
    let result = ngram::ngram_frequencies(&tokens, 0);
    assert!(result.is_empty());
}
