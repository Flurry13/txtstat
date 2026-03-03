#[cfg(feature = "rayon")]
use rayon::prelude::*;
use rustc_hash::FxHashMap;

/// Iterator over n-grams from a token slice. Yields joined strings.
pub fn ngrams<'a>(tokens: &'a [&str], n: usize) -> impl Iterator<Item = String> + 'a {
    tokens.windows(n).map(|window| window.join(" "))
}

/// Count n-gram frequencies from a token slice.
/// Uses parallel processing for large inputs.
pub fn ngram_frequencies(tokens: &[&str], n: usize) -> FxHashMap<String, usize> {
    if n == 0 || tokens.len() < n {
        return FxHashMap::default();
    }
    #[cfg(feature = "rayon")]
    {
        if tokens.len() > 100_000 {
            return ngram_frequencies_parallel(tokens, n);
        }
    }
    ngram_frequencies_sequential(tokens, n)
}

fn ngram_frequencies_sequential(tokens: &[&str], n: usize) -> FxHashMap<String, usize> {
    let mut freqs = FxHashMap::default();
    for ngram in ngrams(tokens, n) {
        *freqs.entry(ngram).or_insert(0) += 1;
    }
    freqs
}

/// Count n-grams for a range of starting positions within the full token slice.
fn count_range(tokens: &[&str], n: usize, start: usize, end: usize) -> FxHashMap<String, usize> {
    let mut freqs = FxHashMap::default();
    for i in start..end {
        let ngram = tokens[i..i + n].join(" ");
        *freqs.entry(ngram).or_insert(0) += 1;
    }
    freqs
}

#[cfg(feature = "rayon")]
fn ngram_frequencies_parallel(tokens: &[&str], n: usize) -> FxHashMap<String, usize> {
    let total_ngrams = tokens.len() - n + 1;
    let num_chunks = rayon::current_num_threads().max(2);
    let chunk_size = (total_ngrams / num_chunks).max(1);

    (0..num_chunks)
        .into_par_iter()
        .map(|i| {
            let start = i * chunk_size;
            let end = if i == num_chunks - 1 {
                total_ngrams
            } else {
                ((i + 1) * chunk_size).min(total_ngrams)
            };
            if start >= total_ngrams {
                FxHashMap::default()
            } else {
                count_range(tokens, n, start, end)
            }
        })
        .reduce(FxHashMap::default, |mut acc, map| {
            for (ngram, count) in map {
                *acc.entry(ngram).or_insert(0) += count;
            }
            acc
        })
}
