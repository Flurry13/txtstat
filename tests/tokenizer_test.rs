use txtstat::analysis::tokenizer;

#[test]
fn test_word_tokenize_simple() {
    let words = tokenizer::words("Hello world");
    assert_eq!(words, vec!["Hello", "world"]);
}

#[test]
fn test_word_tokenize_punctuation() {
    let words = tokenizer::words("It's a test, isn't it?");
    assert!(words.contains(&"It's"));
    assert!(words.contains(&"test"));
    assert!(words.contains(&"isn't"));
    assert!(words.contains(&"it"));
}

#[test]
fn test_word_tokenize_empty() {
    let words = tokenizer::words("");
    assert!(words.is_empty());
}

#[test]
fn test_sentence_count() {
    let count = tokenizer::sentence_count("Hello world. How are you? I'm fine!");
    assert_eq!(count, 3);
}

#[test]
fn test_sentence_count_empty() {
    assert_eq!(tokenizer::sentence_count(""), 0);
}

#[test]
fn test_sentence_count_no_terminal() {
    assert_eq!(tokenizer::sentence_count("Hello world"), 1);
}

#[test]
fn test_syllable_count() {
    assert_eq!(tokenizer::syllable_count("hello"), 2);
    assert_eq!(tokenizer::syllable_count("the"), 1);
    assert_eq!(tokenizer::syllable_count("a"), 1);
}

#[test]
fn test_char_count() {
    assert_eq!(tokenizer::char_count("hello"), 5);
}
