use rustc_hash::FxHashMap;
use corpa::analysis::entropy;

fn make_freqs(pairs: &[(&str, usize)]) -> FxHashMap<String, usize> {
    pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
}

#[test]
fn test_uniform_distribution() {
    // 4 items, each freq 1 -> H = log2(4) = 2.0
    let freqs = make_freqs(&[("a", 1), ("b", 1), ("c", 1), ("d", 1)]);
    let h = entropy::shannon_entropy(&freqs);
    assert!((h - 2.0).abs() < 0.001, "got {}", h);
}

#[test]
fn test_single_item() {
    // 1 item -> H = 0
    let freqs = make_freqs(&[("a", 5)]);
    let h = entropy::shannon_entropy(&freqs);
    assert!((h - 0.0).abs() < 0.001, "got {}", h);
}

#[test]
fn test_known_distribution() {
    // {"a": 3, "b": 1} -> p(a)=0.75, p(b)=0.25
    // H = -(0.75*log2(0.75) + 0.25*log2(0.25))
    // H = -(0.75*(-0.4150) + 0.25*(-2.0))
    // H = -(−0.3113 + −0.5) = 0.8113
    let freqs = make_freqs(&[("a", 3), ("b", 1)]);
    let h = entropy::shannon_entropy(&freqs);
    assert!((h - 0.8113).abs() < 0.001, "got {}", h);
}

#[test]
fn test_empty_distribution() {
    let freqs: FxHashMap<String, usize> = FxHashMap::default();
    let h = entropy::shannon_entropy(&freqs);
    assert_eq!(h, 0.0);
}

#[test]
fn test_entropy_rate() {
    let rate = entropy::entropy_rate(3.0, 4.5);
    assert!((rate - 1.5).abs() < 0.001);
}

#[test]
fn test_redundancy() {
    // rate=1.0, vocab=4 -> log2(4)=2.0 -> redundancy = 1 - 1/2 = 0.5
    let r = entropy::redundancy(1.0, 4);
    assert!((r - 0.5).abs() < 0.001, "got {}", r);
}

#[test]
fn test_redundancy_single_vocab() {
    assert_eq!(entropy::redundancy(1.0, 1), 0.0);
}

#[test]
fn test_redundancy_clamped() {
    // rate > max_entropy should clamp to 0.0, not go negative
    let r = entropy::redundancy(10.0, 4); // max_entropy = log2(4) = 2.0
    assert!(r >= 0.0 && r <= 1.0, "redundancy should be clamped to [0,1], got {}", r);

    // negative rate should clamp to 1.0 at most
    let r2 = entropy::redundancy(-1.0, 4);
    assert!(r2 >= 0.0 && r2 <= 1.0, "redundancy should be clamped to [0,1], got {}", r2);
}
