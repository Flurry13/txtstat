use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn stats(text: &str) -> Result<JsValue, JsValue> {
    let result = corpa::results::compute_stats(text, None);
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn ngrams(text: &str, n: usize, top: usize) -> Result<JsValue, JsValue> {
    let result = corpa::results::compute_ngrams(text, n, top, None, false, None);
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn entropy(text: &str) -> Result<JsValue, JsValue> {
    let result = corpa::results::compute_entropy(text);
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn readability(text: &str) -> Result<JsValue, JsValue> {
    let result = corpa::results::compute_readability(text);
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn lang(text: &str) -> Result<JsValue, JsValue> {
    match corpa::results::compute_lang(text) {
        Some(result) => {
            serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
        }
        None => Ok(JsValue::NULL),
    }
}

#[wasm_bindgen]
pub fn perplexity(text: &str, order: usize, smoothing: &str, k: f64) -> Result<JsValue, JsValue> {
    let result = corpa::results::compute_perplexity(text, order, smoothing, k);
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn zipf(text: &str, top: usize) -> Result<JsValue, JsValue> {
    let result = corpa::results::compute_zipf(text, top);
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn tokens(text: &str) -> Result<JsValue, JsValue> {
    let result = corpa::results::compute_tokens(text, false); // no BPE in WASM
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}
