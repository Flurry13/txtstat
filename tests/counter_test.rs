use lexis::analysis::counter;

#[test]
fn test_word_frequencies() {
    let text = "the cat sat on the mat the cat";
    let freqs = counter::word_frequencies(text);
    assert_eq!(freqs.get("the"), Some(&3));
    assert_eq!(freqs.get("cat"), Some(&2));
    assert_eq!(freqs.get("sat"), Some(&1));
    assert_eq!(freqs.get("on"), Some(&1));
    assert_eq!(freqs.get("mat"), Some(&1));
}

#[test]
fn test_word_frequencies_empty() {
    let freqs = counter::word_frequencies("");
    assert!(freqs.is_empty());
}

#[test]
fn test_top_n() {
    let text = "a a a b b c";
    let freqs = counter::word_frequencies(text);
    let top = counter::top_n(&freqs, 2);
    assert_eq!(top[0], ("a", 3));
    assert_eq!(top[1], ("b", 2));
}

#[test]
fn test_type_count() {
    let text = "the cat sat on the mat";
    let freqs = counter::word_frequencies(text);
    assert_eq!(counter::type_count(&freqs), 5); // the, cat, sat, on, mat
}

#[test]
fn test_hapax_count() {
    let text = "the cat sat on the mat";
    let freqs = counter::word_frequencies(text);
    // hapax = words appearing exactly once: cat, sat, on, mat = 4
    assert_eq!(counter::hapax_count(&freqs), 4);
}

#[test]
fn test_token_count() {
    let text = "the cat sat on the mat";
    let freqs = counter::word_frequencies(text);
    assert_eq!(counter::token_count(&freqs), 6);
}

#[test]
fn test_type_token_ratio() {
    let text = "the cat sat on the mat";
    let freqs = counter::word_frequencies(text);
    let ttr = counter::type_token_ratio(&freqs);
    assert!((ttr - 5.0 / 6.0).abs() < 0.001);
}
