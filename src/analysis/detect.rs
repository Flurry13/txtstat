/// Result of language detection on a text sample.
pub struct LangResult {
    pub language: String,
    pub code: String,
    pub script: String,
    pub confidence: f64,
    pub is_reliable: bool,
}

/// Detect the language of the given text.
/// Returns `None` if the text is too short or ambiguous for detection.
pub fn detect(text: &str) -> Option<LangResult> {
    let info = whatlang::detect(text)?;
    Some(LangResult {
        language: format!("{}", info.lang()),
        code: info.lang().code().to_string(),
        script: format!("{}", info.script()),
        confidence: info.confidence() as f64,
        is_reliable: info.is_reliable(),
    })
}
