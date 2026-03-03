use rustc_hash::FxHashMap;

/// Shannon entropy of a frequency distribution.
/// H = -sum(p * log2(p)) where p = count / total.
pub fn shannon_entropy(freqs: &FxHashMap<String, usize>) -> f64 {
    let total: usize = freqs.values().sum();
    if total == 0 {
        return 0.0;
    }
    let total_f = total as f64;
    let mut h = 0.0;
    for &count in freqs.values() {
        if count > 0 {
            let p = count as f64 / total_f;
            h -= p * p.log2();
        }
    }
    h
}

/// Entropy rate estimated as H(trigram) - H(bigram).
pub fn entropy_rate(h_bigram: f64, h_trigram: f64) -> f64 {
    h_trigram - h_bigram
}

/// Redundancy: 1 - rate / log2(vocab_size).
/// Returns 0.0 if vocab_size <= 1.
pub fn redundancy(rate: f64, vocab_size: usize) -> f64 {
    if vocab_size <= 1 {
        return 0.0;
    }
    let max_entropy = (vocab_size as f64).log2();
    if max_entropy == 0.0 {
        return 0.0;
    }
    (1.0 - rate / max_entropy).clamp(0.0, 1.0)
}
