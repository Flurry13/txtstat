use unicode_segmentation::UnicodeSegmentation;

/// Extract words from text using Unicode word boundaries.
/// Filters out whitespace and punctuation-only segments.
pub fn words(text: &str) -> Vec<&str> {
    text.unicode_words().collect()
}

/// Count sentences. Uses Unicode sentence boundaries.
/// Returns at least 1 for non-empty text.
pub fn sentence_count(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    let count = text
        .split_sentence_bounds()
        .filter(|s| s.contains(|c: char| c.is_alphanumeric()))
        .count();
    count.max(1)
}

/// Estimate syllable count for a word using vowel-group heuristic.
pub fn syllable_count(word: &str) -> usize {
    let word = word.to_lowercase();
    if word.is_empty() {
        return 0;
    }
    if word.len() <= 3 {
        return 1;
    }
    let vowels = "aeiouy";
    let mut count = 0;
    let mut prev_vowel = false;
    for ch in word.chars() {
        let is_vowel = vowels.contains(ch);
        if is_vowel && !prev_vowel {
            count += 1;
        }
        prev_vowel = is_vowel;
    }
    // Silent 'e' at end
    if word.ends_with('e') && count > 1 {
        count -= 1;
    }
    count.max(1)
}

/// Count unicode characters (not bytes).
pub fn char_count(text: &str) -> usize {
    text.chars().count()
}
