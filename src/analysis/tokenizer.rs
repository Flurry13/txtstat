/// Extract words from text using fast whitespace splitting with punctuation trimming.
/// For the vast majority of text, this produces identical results to unicode_words()
/// but runs significantly faster on large inputs.
pub fn words(text: &str) -> Vec<&str> {
    text.split_whitespace()
        .map(|w| w.trim_matches(|c: char| c.is_ascii_punctuation() || c == '\u{2014}' || c == '\u{2013}'))
        .filter(|w| !w.is_empty())
        .collect()
}

/// Common abbreviations (without trailing period).
const ABBREVIATIONS: &[&str] = &[
    "mr", "mrs", "ms", "dr", "prof", "sr", "jr", "st", "ave", "blvd",
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    "vs", "etc", "approx", "dept", "est", "govt", "inc", "ltd", "co", "corp",
    "gen", "sgt", "cpl", "pvt", "capt", "col", "maj", "lt", "fig", "vol", "no",
];

/// Check if a dot at `dot_pos` in `chars` follows an abbreviation.
fn is_abbreviation(chars: &[char], dot_pos: usize) -> bool {
    if dot_pos == 0 {
        return false;
    }
    // Walk backward to find the preceding alphabetic word
    let mut end = dot_pos;
    while end > 0 && !chars[end - 1].is_alphabetic() {
        end -= 1;
    }
    if end == 0 {
        return false;
    }
    let mut start = end;
    while start > 0 && chars[start - 1].is_alphabetic() {
        start -= 1;
    }
    let word_len = end - start;
    // Single letter before dot = abbreviation (U.S.A., e.g., i.e.)
    if word_len == 1 {
        return true;
    }
    // Check against known abbreviations (case-insensitive)
    let word: String = chars[start..end].iter().collect();
    let lower = word.to_lowercase();
    ABBREVIATIONS.contains(&lower.as_str())
}

/// Count sentences using a state machine that collapses consecutive
/// sentence-ending punctuation and skips abbreviations.
/// Returns at least 1 for non-empty text.
pub fn sentence_count(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    let chars: Vec<char> = text.chars().collect();
    let mut count = 0;
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '.' || chars[i] == '!' || chars[i] == '?' {
            let boundary_start = i;
            // Skip all consecutive sentence-ending punctuation
            while i < chars.len() && (chars[i] == '.' || chars[i] == '!' || chars[i] == '?') {
                i += 1;
            }
            // If boundary starts with '.', check for abbreviation
            if chars[boundary_start] == '.' && is_abbreviation(&chars, boundary_start) {
                continue;
            }
            // Punctuation directly followed by an alphabetic char is a pause, not a boundary
            if i < chars.len() && chars[i].is_alphabetic() {
                continue;
            }
            count += 1;
        } else {
            i += 1;
        }
    }
    count.max(1)
}

/// Check if a character is a vowel, including accented Latin vowels.
fn is_vowel(c: char) -> bool {
    let lower = c.to_lowercase().next().unwrap_or(c);
    matches!(
        lower,
        'a' | 'e' | 'i' | 'o' | 'u' | 'y'
            | '\u{00E0}' | '\u{00E1}' | '\u{00E2}' | '\u{00E3}' | '\u{00E4}' | '\u{00E5}' // à á â ã ä å
            | '\u{00E8}' | '\u{00E9}' | '\u{00EA}' | '\u{00EB}' // è é ê ë
            | '\u{00EC}' | '\u{00ED}' | '\u{00EE}' | '\u{00EF}' // ì í î ï
            | '\u{00F2}' | '\u{00F3}' | '\u{00F4}' | '\u{00F5}' | '\u{00F6}' // ò ó ô õ ö
            | '\u{00F9}' | '\u{00FA}' | '\u{00FB}' | '\u{00FC}' // ù ú û ü
            | '\u{00FD}' | '\u{00FF}' // ý ÿ
    )
}

/// Estimate syllable count for a word using vowel-group heuristic.
pub fn syllable_count(word: &str) -> usize {
    if word.is_empty() {
        return 0;
    }
    let chars: Vec<char> = word.chars().collect();
    if chars.len() <= 3 {
        return 1;
    }
    let mut count = 0;
    let mut prev_vowel = false;
    for &c in &chars {
        let v = is_vowel(c);
        if v && !prev_vowel {
            count += 1;
        }
        prev_vowel = v;
    }
    // Silent 'e' at end — only ASCII e (accented-e endings are pronounced)
    let last = *chars.last().unwrap();
    if (last == 'e' || last == 'E') && count > 1 {
        count -= 1;
    }
    count.max(1)
}

/// Count unicode characters (not bytes).
pub fn char_count(text: &str) -> usize {
    text.chars().count()
}
