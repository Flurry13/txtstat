use rustc_hash::FxHashSet;
use lexis::utils::stopwords;

#[test]
fn test_default_english_not_empty() {
    let sw = stopwords::default_english();
    assert!(sw.len() > 100);
    assert!(sw.contains("the"));
    assert!(sw.contains("a"));
    assert!(sw.contains("is"));
}

#[test]
fn test_filter_words() {
    let mut sw = FxHashSet::default();
    sw.insert("the".to_string());
    sw.insert("a".to_string());
    sw.insert("is".to_string());

    let words = vec!["The", "cat", "is", "a", "happy", "animal"];
    let filtered = stopwords::filter_words(&words, &sw);
    assert_eq!(filtered, vec!["cat", "happy", "animal"]);
}

#[test]
fn test_filter_words_case_insensitive() {
    let mut sw = FxHashSet::default();
    sw.insert("the".to_string());

    let words = vec!["The", "THE", "the", "cat"];
    let filtered = stopwords::filter_words(&words, &sw);
    assert_eq!(filtered, vec!["cat"]);
}

#[test]
fn test_load_stopwords_from_file() {
    let sw = stopwords::load_stopwords(std::path::Path::new("data/stopwords/english.txt")).unwrap();
    assert!(sw.contains("the"));
    assert!(!sw.contains("#"));
    assert!(!sw.contains(""));
}
