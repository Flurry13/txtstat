use pyo3::prelude::*;
use rustc_hash::FxHashSet;

fn read_text(_py: Python<'_>, path: Option<String>, text: Option<String>) -> PyResult<String> {
    match (path, text) {
        (_, Some(t)) => Ok(t),
        (Some(p), None) => std::fs::read_to_string(&p)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}: {}", p, e))),
        (None, None) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Either path or text argument is required",
        )),
    }
}

fn parse_stopwords(stopwords: Option<Vec<String>>) -> Option<FxHashSet<String>> {
    stopwords.map(|words| words.into_iter().map(|w| w.to_lowercase()).collect())
}

fn to_json(value: &impl serde::Serialize) -> PyResult<String> {
    serde_json::to_string(value)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

fn result_to_dict<'py>(py: Python<'py>, json_str: &str) -> PyResult<Bound<'py, PyAny>> {
    let json_mod = py.import("json")?;
    json_mod.call_method1("loads", (json_str,))
}

/// Compute text statistics: token/type counts, TTR, hapax legomena, sentence count.
///
/// Args:
///     path: Path to a text file.
///     text: Text string to analyze.
///     stopwords: Optional list of stopwords to filter out.
///
/// Returns:
///     Dict with tokens, types, characters, sentences, type_token_ratio,
///     hapax_legomena, hapax_percentage, avg_sentence_length, stopwords_removed.
#[pyfunction]
#[pyo3(signature = (path=None, *, text=None, stopwords=None))]
fn stats(
    py: Python<'_>,
    path: Option<String>,
    text: Option<String>,
    stopwords: Option<Vec<String>>,
) -> PyResult<Bound<'_, PyAny>> {
    let t = read_text(py, path, text)?;
    let sw = parse_stopwords(stopwords);
    let result = corpa_core::results::compute_stats(&t, sw.as_ref());
    result_to_dict(py, &to_json(&result)?)
}

/// Compute n-gram frequencies from text.
///
/// Args:
///     path: Path to a text file.
///     text: Text string to analyze.
///     n: N-gram size (default 2 for bigrams).
///     top: Number of top n-grams to return (default 10).
///     min_freq: Minimum frequency threshold to include an n-gram.
///     case_insensitive: Fold case before counting (default False).
///     stopwords: Optional list of stopwords to filter out before n-gram extraction.
///
/// Returns:
///     List of dicts with ngram, frequency, relative_pct.
#[pyfunction]
#[pyo3(signature = (path=None, *, text=None, n=2, top=10, min_freq=None, case_insensitive=false, stopwords=None))]
fn ngrams(
    py: Python<'_>,
    path: Option<String>,
    text: Option<String>,
    n: usize,
    top: usize,
    min_freq: Option<usize>,
    case_insensitive: bool,
    stopwords: Option<Vec<String>>,
) -> PyResult<Bound<'_, PyAny>> {
    let t = read_text(py, path, text)?;
    let sw = parse_stopwords(stopwords);
    let result = corpa_core::results::compute_ngrams(&t, n, top, min_freq, case_insensitive, sw.as_ref());
    result_to_dict(py, &to_json(&result)?)
}

/// Compute Shannon entropy at orders 1-3, entropy rate, and redundancy.
///
/// Args:
///     path: Path to a text file.
///     text: Text string to analyze.
///
/// Returns:
///     Dict with h1, h2, h3, entropy_rate, vocabulary_size, redundancy.
#[pyfunction]
#[pyo3(signature = (path=None, *, text=None))]
fn entropy(
    py: Python<'_>,
    path: Option<String>,
    text: Option<String>,
) -> PyResult<Bound<'_, PyAny>> {
    let t = read_text(py, path, text)?;
    let result = corpa_core::results::compute_entropy(&t);
    result_to_dict(py, &to_json(&result)?)
}

/// Compute readability scores (Flesch-Kincaid, Flesch Reading Ease, etc.).
///
/// Args:
///     path: Path to a text file.
///     text: Text string to analyze.
///
/// Returns:
///     Dict with flesch_kincaid_grade, flesch_reading_ease, coleman_liau,
///     gunning_fog, smog.
#[pyfunction]
#[pyo3(signature = (path=None, *, text=None))]
fn readability(
    py: Python<'_>,
    path: Option<String>,
    text: Option<String>,
) -> PyResult<Bound<'_, PyAny>> {
    let t = read_text(py, path, text)?;
    let result = corpa_core::results::compute_readability(&t);
    result_to_dict(py, &to_json(&result)?)
}

/// Compute perplexity under an n-gram language model.
///
/// Args:
///     path: Path to a text file.
///     text: Text string to analyze.
///     order: N-gram order (default 3).
///     smoothing: Smoothing method — "none", "laplace", or "backoff" (default "laplace").
///     k: Smoothing constant for add-k smoothing (default 1.0).
///
/// Returns:
///     Dict with order, vocab_size, ngram_counts, smoothing, perplexity.
#[pyfunction]
#[pyo3(signature = (path=None, *, text=None, order=3, smoothing="laplace", k=1.0))]
fn perplexity<'py>(
    py: Python<'py>,
    path: Option<String>,
    text: Option<String>,
    order: usize,
    smoothing: &str,
    k: f64,
) -> PyResult<Bound<'py, PyAny>> {
    let t = read_text(py, path, text)?;
    let result = corpa_core::results::compute_perplexity(&t, order, smoothing, k);
    result_to_dict(py, &to_json(&result)?)
}

/// Detect the language of the given text.
///
/// Args:
///     path: Path to a text file.
///     text: Text string to analyze.
///
/// Returns:
///     Dict with language, code, script, confidence, is_reliable.
///     Returns None if text is too short or ambiguous.
#[pyfunction]
#[pyo3(signature = (path=None, *, text=None))]
fn lang(py: Python<'_>, path: Option<String>, text: Option<String>) -> PyResult<Bound<'_, PyAny>> {
    let t = read_text(py, path, text)?;
    match corpa_core::results::compute_lang(&t) {
        Some(result) => result_to_dict(py, &to_json(&result)?),
        None => Ok(py.None().into_bound(py)),
    }
}

/// Count tokens using whitespace, sentence, character, and BPE tokenizers.
///
/// Args:
///     path: Path to a text file.
///     text: Text string to analyze.
///     include_bpe: Include BPE token counts for GPT models (default True).
///
/// Returns:
///     Dict with whitespace, sentences, characters, bpe_gpt4, bpe_gpt4o, bpe_gpt3.
#[pyfunction]
#[pyo3(signature = (path=None, *, text=None, include_bpe=true))]
fn tokens(
    py: Python<'_>,
    path: Option<String>,
    text: Option<String>,
    include_bpe: bool,
) -> PyResult<Bound<'_, PyAny>> {
    let t = read_text(py, path, text)?;
    let result = corpa_core::results::compute_tokens(&t, include_bpe);
    result_to_dict(py, &to_json(&result)?)
}

/// Compute Zipf's law analysis: word frequency distribution with exponent.
///
/// Args:
///     path: Path to a text file.
///     text: Text string to analyze.
///     top: Number of top entries to return (default 20).
///
/// Returns:
///     Dict with entries (list of {rank, word, frequency}), alpha, r_squared.
#[pyfunction]
#[pyo3(signature = (path=None, *, text=None, top=20))]
fn zipf(
    py: Python<'_>,
    path: Option<String>,
    text: Option<String>,
    top: usize,
) -> PyResult<Bound<'_, PyAny>> {
    let t = read_text(py, path, text)?;
    let result = corpa_core::results::compute_zipf(&t, top);
    result_to_dict(py, &to_json(&result)?)
}

#[pymodule]
mod corpa {
    #[pymodule_export]
    use super::stats;
    #[pymodule_export]
    use super::ngrams;
    #[pymodule_export]
    use super::entropy;
    #[pymodule_export]
    use super::readability;
    #[pymodule_export]
    use super::perplexity;
    #[pymodule_export]
    use super::lang;
    #[pymodule_export]
    use super::tokens;
    #[pymodule_export]
    use super::zipf;
}
