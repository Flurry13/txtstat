use rustc_hash::FxHashSet;
use wasm_bindgen::prelude::*;

fn parse_stopwords(stopwords: JsValue) -> Option<FxHashSet<String>> {
    if stopwords.is_null() || stopwords.is_undefined() {
        return None;
    }
    let words: Vec<String> = serde_wasm_bindgen::from_value(stopwords).ok()?;
    Some(words.into_iter().map(|w| w.to_lowercase()).collect())
}

fn parse_min_freq(min_freq: JsValue) -> Option<usize> {
    min_freq.as_f64().map(|v| v as usize)
}

fn parse_bool(val: JsValue) -> bool {
    val.as_bool().unwrap_or(false)
}

fn to_js(value: &impl serde::Serialize) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(value).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn stats(text: &str, stopwords: JsValue) -> Result<JsValue, JsValue> {
    let sw = parse_stopwords(stopwords);
    let result = corpa::results::compute_stats(text, sw.as_ref());
    to_js(&result)
}

#[wasm_bindgen]
pub fn ngrams(
    text: &str,
    n: usize,
    top: usize,
    min_freq: JsValue,
    case_insensitive: JsValue,
    stopwords: JsValue,
) -> Result<JsValue, JsValue> {
    let sw = parse_stopwords(stopwords);
    let result = corpa::results::compute_ngrams(
        text,
        n,
        top,
        parse_min_freq(min_freq),
        parse_bool(case_insensitive),
        sw.as_ref(),
    );
    to_js(&result)
}

#[wasm_bindgen]
pub fn entropy(text: &str) -> Result<JsValue, JsValue> {
    to_js(&corpa::results::compute_entropy(text))
}

#[wasm_bindgen]
pub fn readability(text: &str) -> Result<JsValue, JsValue> {
    to_js(&corpa::results::compute_readability(text))
}

#[wasm_bindgen]
pub fn lang(text: &str) -> Result<JsValue, JsValue> {
    match corpa::results::compute_lang(text) {
        Some(result) => to_js(&result),
        None => Ok(JsValue::NULL),
    }
}

#[wasm_bindgen]
pub fn perplexity(text: &str, order: usize, smoothing: &str, k: f64) -> Result<JsValue, JsValue> {
    to_js(&corpa::results::compute_perplexity(text, order, smoothing, k))
}

#[wasm_bindgen]
pub fn zipf(text: &str, top: usize) -> Result<JsValue, JsValue> {
    to_js(&corpa::results::compute_zipf(text, top))
}

#[wasm_bindgen]
pub fn tokens(text: &str) -> Result<JsValue, JsValue> {
    to_js(&corpa::results::compute_tokens(text, false)) // no BPE in WASM
}
