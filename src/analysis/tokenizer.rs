/// Extract words from text using fast whitespace splitting with punctuation trimming.
/// For the vast majority of text, this produces identical results to unicode_words()
/// but runs significantly faster on large inputs.
pub fn words(text: &str) -> Vec<&str> {
    text.split_whitespace()
        .map(|w| w.trim_matches(|c: char| c.is_ascii_punctuation() || c == '\u{2014}' || c == '\u{2013}'))
        .filter(|w| !w.is_empty())
        .collect()
}

/// Count sentences by counting sentence-ending punctuation.
/// Returns at least 1 for non-empty text.
pub fn sentence_count(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    let count = text.chars().filter(|&c| c == '.' || c == '!' || c == '?').count();
    count.max(1)
}

/// Estimate syllable count for a word using vowel-group heuristic.
pub fn syllable_count(word: &str) -> usize {
    if word.is_empty() {
        return 0;
    }
    if word.len() <= 3 {
        return 1;
    }
    let vowels = b"aeiouyAEIOUY";
    let bytes = word.as_bytes();
    let mut count = 0;
    let mut prev_vowel = false;
    for &b in bytes {
        let is_vowel = vowels.contains(&b);
        if is_vowel && !prev_vowel {
            count += 1;
        }
        prev_vowel = is_vowel;
    }
    // Silent 'e' at end
    if (bytes.last() == Some(&b'e') || bytes.last() == Some(&b'E')) && count > 1 {
        count -= 1;
    }
    count.max(1)
}

/// Count unicode characters (not bytes).
pub fn char_count(text: &str) -> usize {
    text.chars().count()
}
