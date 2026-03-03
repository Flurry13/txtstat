use corpa::analysis::tokenizer;

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
fn test_sentence_count_ellipsis() {
    assert_eq!(tokenizer::sentence_count("Wait... really?"), 2);
}

#[test]
fn test_sentence_count_interrobang() {
    assert_eq!(tokenizer::sentence_count("What?! That's crazy."), 2);
}

#[test]
fn test_sentence_count_multiple_exclamation() {
    assert_eq!(tokenizer::sentence_count("Wow!!! Amazing!!!"), 2);
}

#[test]
fn test_sentence_count_abbreviation() {
    assert_eq!(tokenizer::sentence_count("Dr. Smith went home."), 1);
}

#[test]
fn test_sentence_count_multiple_abbreviations() {
    assert_eq!(tokenizer::sentence_count("Mr. and Mrs. Smith arrived."), 1);
}

#[test]
fn test_sentence_count_initialism() {
    assert_eq!(tokenizer::sentence_count("He moved to the U.S.A. last year."), 1);
}

#[test]
fn test_sentence_count_mixed() {
    assert_eq!(
        tokenizer::sentence_count("Mr. Smith arrived. He shouted hello! Wait...really?!"),
        3,
    );
}

#[test]
fn test_sentence_count_only_punctuation() {
    assert_eq!(tokenizer::sentence_count("..."), 1);
}

#[test]
fn test_syllable_count_unicode() {
    // Accented-e ending is pronounced (not silent)
    assert_eq!(tokenizer::syllable_count("caf\u{00E9}"), 2);
    // Short word by char count (2 chars, 3+ bytes)
    assert_eq!(tokenizer::syllable_count("n\u{00E9}"), 1);
    // Umlaut vowel
    assert_eq!(tokenizer::syllable_count("\u{00FC}ber"), 2);
    // Multiple accented vowels
    assert_eq!(tokenizer::syllable_count("r\u{00E9}sum\u{00E9}"), 3);
    // Uppercase accented vowels must also be recognized
    assert_eq!(tokenizer::syllable_count("CAF\u{00C9}"), 2);
    assert_eq!(tokenizer::syllable_count("R\u{00C9}SUM\u{00C9}"), 3);
}

#[test]
fn test_syllable_count_ascii_regression() {
    assert_eq!(tokenizer::syllable_count("beautiful"), 3);
    assert_eq!(tokenizer::syllable_count("hello"), 2);
    assert_eq!(tokenizer::syllable_count("the"), 1);
    assert_eq!(tokenizer::syllable_count(""), 0);
    assert_eq!(tokenizer::syllable_count("a"), 1);
}

#[test]
fn test_char_count() {
    assert_eq!(tokenizer::char_count("hello"), 5);
}
