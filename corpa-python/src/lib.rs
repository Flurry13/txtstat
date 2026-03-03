use pyo3::prelude::*;

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

fn result_to_dict<'py>(py: Python<'py>, json_str: &str) -> PyResult<Bound<'py, PyAny>> {
    let json_mod = py.import("json")?;
    json_mod.call_method1("loads", (json_str,))
}

#[pyfunction]
#[pyo3(signature = (path=None, *, text=None))]
fn stats(py: Python<'_>, path: Option<String>, text: Option<String>) -> PyResult<Bound<'_, PyAny>> {
    let t = read_text(py, path, text)?;
    let result = corpa_core::results::compute_stats(&t, None);
    let json = serde_json::to_string(&result).unwrap();
    result_to_dict(py, &json)
}

#[pyfunction]
#[pyo3(signature = (path=None, *, text=None, n=2, top=10))]
fn ngrams(
    py: Python<'_>,
    path: Option<String>,
    text: Option<String>,
    n: usize,
    top: usize,
) -> PyResult<Bound<'_, PyAny>> {
    let t = read_text(py, path, text)?;
    let result = corpa_core::results::compute_ngrams(&t, n, top, None, false, None);
    let json = serde_json::to_string(&result).unwrap();
    result_to_dict(py, &json)
}

#[pyfunction]
#[pyo3(signature = (path=None, *, text=None))]
fn entropy(
    py: Python<'_>,
    path: Option<String>,
    text: Option<String>,
) -> PyResult<Bound<'_, PyAny>> {
    let t = read_text(py, path, text)?;
    let result = corpa_core::results::compute_entropy(&t);
    let json = serde_json::to_string(&result).unwrap();
    result_to_dict(py, &json)
}

#[pyfunction]
#[pyo3(signature = (path=None, *, text=None))]
fn readability(
    py: Python<'_>,
    path: Option<String>,
    text: Option<String>,
) -> PyResult<Bound<'_, PyAny>> {
    let t = read_text(py, path, text)?;
    let result = corpa_core::results::compute_readability(&t);
    let json = serde_json::to_string(&result).unwrap();
    result_to_dict(py, &json)
}

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
    let json = serde_json::to_string(&result).unwrap();
    result_to_dict(py, &json)
}

#[pyfunction]
#[pyo3(signature = (path=None, *, text=None))]
fn lang(py: Python<'_>, path: Option<String>, text: Option<String>) -> PyResult<Bound<'_, PyAny>> {
    let t = read_text(py, path, text)?;
    match corpa_core::results::compute_lang(&t) {
        Some(result) => {
            let json = serde_json::to_string(&result).unwrap();
            result_to_dict(py, &json)
        }
        None => Ok(py.None().into_bound(py)),
    }
}

#[pyfunction]
#[pyo3(signature = (path=None, *, text=None))]
fn tokens(
    py: Python<'_>,
    path: Option<String>,
    text: Option<String>,
) -> PyResult<Bound<'_, PyAny>> {
    let t = read_text(py, path, text)?;
    let result = corpa_core::results::compute_tokens(&t, true);
    let json = serde_json::to_string(&result).unwrap();
    result_to_dict(py, &json)
}

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
    let json = serde_json::to_string(&result).unwrap();
    result_to_dict(py, &json)
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
