# Fix All Known Bugs — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix all confirmed bugs found during the comprehensive code review.

**Architecture:** Seven independent bug fixes across core analysis, output, streaming, Python bindings, and WASM bindings. Each fix is self-contained and testable.

**Tech Stack:** Rust, PyO3, wasm-bindgen, serde_json

---

### Task 1: Guard against n=0 panic in ngram_frequencies and streaming

**Files:**
- Modify: `src/analysis/ngram.rs:12-15`
- Modify: `src/results.rs:145-152`
- Modify: `src/streaming.rs:120-125`
- Test: `tests/ngram_test.rs`
- Test: `tests/streaming_test.rs`

**Step 1: Write failing tests**

In `tests/ngram_test.rs`, add:

```rust
#[test]
fn test_ngram_zero_returns_empty() {
    let tokens: Vec<&str> = vec!["hello", "world"];
    let result = corpa::analysis::ngram::ngram_frequencies(&tokens, 0);
    assert!(result.is_empty());
}
```

In `tests/streaming_test.rs`, add:

```rust
#[test]
fn test_stream_ngrams_n_zero_errors() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_corpa"))
        .args(&["ngrams", "-n", "0", "--stream"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn");
    child.stdin.take().unwrap().write_all(b"hello world\n").unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(!output.status.success(), "should fail for n=0");
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test test_ngram_zero_returns_empty test_stream_ngrams_n_zero_errors -- --nocapture 2>&1`
Expected: FAIL — first test panics on `windows(0)`, second test panics in child process

**Step 3: Implement the fixes**

In `src/analysis/ngram.rs`, add early return for `n == 0`:

```rust
pub fn ngram_frequencies(tokens: &[&str], n: usize) -> FxHashMap<String, usize> {
    if n == 0 || tokens.len() < n {
        return FxHashMap::default();
    }
    // ... rest unchanged
}
```

Also fix the `ngrams` iterator function at the top of the same file:

```rust
pub fn ngrams<'a>(tokens: &'a [&str], n: usize) -> impl Iterator<Item = String> + 'a {
    tokens.windows(n.max(1)).map(|window| window.join(" "))
}
```

In `src/results.rs`, add guard at the top of `compute_ngrams`:

```rust
pub fn compute_ngrams(
    text: &str,
    n: usize,
    // ...
) -> Vec<NgramEntry> {
    if n == 0 {
        return Vec::new();
    }
    // ... rest unchanged
}
```

In `src/streaming.rs`, add validation at the top of `stream_ngrams`:

```rust
pub fn stream_ngrams(
    format: &OutputFormat,
    chunk_lines: usize,
    n: usize,
    top: usize,
) -> Result<()> {
    anyhow::ensure!(n >= 1, "n-gram size must be at least 1");
    // ... rest unchanged
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test test_ngram_zero test_stream_ngrams_n_zero -- --nocapture 2>&1`
Expected: PASS

**Step 5: Commit**

```bash
git add src/analysis/ngram.rs src/results.rs src/streaming.rs tests/ngram_test.rs tests/streaming_test.rs
git commit -m "fix: guard against n=0 panic in ngram_frequencies and streaming path"
```

---

### Task 2: Emit typed JSON values instead of strings

**Files:**
- Modify: `src/output.rs:63-80`
- Test: `tests/cli_test.rs`

**Step 1: Write failing test**

In `tests/cli_test.rs`, add:

```rust
#[test]
fn test_stats_json_numeric_values() {
    let out = corpa(&["stats", "tests/fixtures/prose.txt", "--format", "json"]);
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&out).unwrap();
    // Find the "Tokens (words)" row and check its value is a number, not a string
    let tokens_row = parsed.iter().find(|r| r["metric"] == "Tokens (words)").unwrap();
    assert!(tokens_row["value"].is_number(), "token count should be a JSON number, got: {}", tokens_row["value"]);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_stats_json_numeric_values -- --nocapture 2>&1`
Expected: FAIL — value is `"126"` (string), not `126` (number)

**Step 3: Implement the fix**

In `src/output.rs`, replace `render_json` to attempt parsing values as numbers:

```rust
fn render_json(&self) -> Result<String> {
    let records: Vec<serde_json::Value> = self
        .rows
        .iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            for (header, value) in self.headers.iter().zip(row.iter()) {
                let key = header.to_lowercase().replace(' ', "_");
                // Try to parse as numeric types, fall back to string
                let json_value = Self::parse_value(value);
                map.insert(key, json_value);
            }
            serde_json::Value::Object(map)
        })
        .collect();

    Ok(serde_json::to_string_pretty(&records)?)
}

/// Try to parse a display string as a JSON number, falling back to string.
/// Handles comma-formatted numbers ("1,234") and floats ("0.5429").
fn parse_value(s: &str) -> serde_json::Value {
    // Strip commas from comma-formatted numbers (e.g., "1,234" -> "1234")
    let stripped = s.replace(',', "");
    // Try integer first
    if let Ok(n) = stripped.parse::<u64>() {
        return serde_json::Value::Number(n.into());
    }
    // Try float
    if let Ok(f) = stripped.parse::<f64>() {
        if let Some(n) = serde_json::Number::from_f64(f) {
            return serde_json::Value::Number(n);
        }
    }
    // Fall back to string for non-numeric values
    serde_json::Value::String(s.to_string())
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test -- --nocapture 2>&1`
Expected: ALL PASS (including the new test and existing tests — existing tests check for key presence, not value types)

**Step 5: Commit**

```bash
git add src/output.rs tests/cli_test.rs
git commit -m "fix: emit typed JSON values instead of strings for jq interoperability"
```

---

### Task 3: Clamp entropy redundancy to [0, 1]

**Files:**
- Modify: `src/analysis/entropy.rs:28-37`
- Test: `tests/entropy_test.rs`

**Step 1: Write failing test**

In `tests/entropy_test.rs`, add:

```rust
#[test]
fn test_redundancy_clamped() {
    // rate > max_entropy should clamp to 0.0, not go negative
    let r = corpa::analysis::entropy::redundancy(10.0, 4); // max_entropy = log2(4) = 2.0
    assert!(r >= 0.0 && r <= 1.0, "redundancy should be clamped to [0,1], got {}", r);

    // negative rate should clamp to 1.0 at most
    let r2 = corpa::analysis::entropy::redundancy(-1.0, 4);
    assert!(r2 >= 0.0 && r2 <= 1.0, "redundancy should be clamped to [0,1], got {}", r2);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_redundancy_clamped -- --nocapture 2>&1`
Expected: FAIL — `r = 1.0 - 10.0/2.0 = -4.0`

**Step 3: Implement the fix**

In `src/analysis/entropy.rs`, clamp the result:

```rust
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
```

**Step 4: Run tests to verify they pass**

Run: `cargo test test_redundancy -- --nocapture 2>&1`
Expected: ALL redundancy tests PASS

**Step 5: Commit**

```bash
git add src/analysis/entropy.rs tests/entropy_test.rs
git commit -m "fix: clamp entropy redundancy to [0, 1] for degenerate inputs"
```

---

### Task 4: Rewrite streaming entropy to use incremental frequency maps

**Files:**
- Modify: `src/streaming.rs:258-318`
- Test: `tests/streaming_test.rs`

**Step 1: Write test**

In `tests/streaming_test.rs`, add:

```rust
#[test]
fn test_stream_entropy_json() {
    let input = "the cat sat on the mat\nthe dog sat on the rug\n".repeat(100);
    let out = corpa_stdin(
        &["entropy", "--stream", "--chunk-lines", "50", "--format", "json"],
        &input,
    );
    let lines: Vec<&str> = out.trim().lines().collect();
    assert!(lines.len() >= 2, "should emit multiple JSON lines, got {}", lines.len());
    for line in &lines {
        let parsed: serde_json::Value = serde_json::from_str(line).unwrap();
        assert!(parsed.get("h1").is_some());
        assert!(parsed.get("vocabulary_size").is_some());
        assert!(parsed.get("chunk").is_some());
    }
}
```

**Step 2: Run test**

Run: `cargo test test_stream_entropy_json -- --nocapture 2>&1`
Expected: Should pass (existing code works, just poorly). This test is for regression.

**Step 3: Implement the fix**

Replace `stream_entropy` in `src/streaming.rs` with an incremental approach that maintains running bigram and trigram frequency maps instead of accumulating all words. Keep overlap tokens for cross-chunk boundary n-grams (same pattern as `stream_ngrams`):

```rust
pub fn stream_entropy(format: &OutputFormat, chunk_lines: usize) -> Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut word_freqs: FxHashMap<String, usize> = FxHashMap::default();
    let mut bigram_freqs: FxHashMap<String, usize> = FxHashMap::default();
    let mut trigram_freqs: FxHashMap<String, usize> = FxHashMap::default();
    let mut bigram_overlap: Vec<String> = Vec::new();
    let mut trigram_overlap: Vec<String> = Vec::new();
    let mut chunk_count = 0usize;
    let mut line_buf = Vec::new();
    let mut first_csv = true;

    for line in reader.lines() {
        line_buf.push(line?);
        if line_buf.len() >= chunk_lines {
            let chunk = line_buf.join("\n");
            process_entropy_chunk(
                &chunk,
                &mut word_freqs,
                &mut bigram_freqs,
                &mut trigram_freqs,
                &mut bigram_overlap,
                &mut trigram_overlap,
            );
            chunk_count += 1;

            let h1 = entropy::shannon_entropy(&word_freqs);
            let h2 = entropy::shannon_entropy(&bigram_freqs);
            let h3 = entropy::shannon_entropy(&trigram_freqs);
            let rate = entropy::entropy_rate(h2, h3);
            let vocab = word_freqs.len();
            let redund = entropy::redundancy(rate, vocab);

            emit_entropy(
                chunk_count, h1, h2, h3, rate, vocab, redund, format, &mut first_csv,
            )?;
            line_buf.clear();
        }
    }
    if !line_buf.is_empty() {
        let chunk = line_buf.join("\n");
        process_entropy_chunk(
            &chunk,
            &mut word_freqs,
            &mut bigram_freqs,
            &mut trigram_freqs,
            &mut bigram_overlap,
            &mut trigram_overlap,
        );
        chunk_count += 1;

        let h1 = entropy::shannon_entropy(&word_freqs);
        let h2 = entropy::shannon_entropy(&bigram_freqs);
        let h3 = entropy::shannon_entropy(&trigram_freqs);
        let rate = entropy::entropy_rate(h2, h3);
        let vocab = word_freqs.len();
        let redund = entropy::redundancy(rate, vocab);

        emit_entropy(
            chunk_count, h1, h2, h3, rate, vocab, redund, format, &mut first_csv,
        )?;
    }
    Ok(())
}

fn process_entropy_chunk(
    chunk: &str,
    word_freqs: &mut FxHashMap<String, usize>,
    bigram_freqs: &mut FxHashMap<String, usize>,
    trigram_freqs: &mut FxHashMap<String, usize>,
    bigram_overlap: &mut Vec<String>,
    trigram_overlap: &mut Vec<String>,
) {
    // Update unigram frequencies
    let chunk_freqs = counter::word_frequencies(chunk);
    for (word, count) in &chunk_freqs {
        *word_freqs.entry(word.clone()).or_insert(0) += count;
    }

    let chunk_words = tokenizer::words(chunk);

    // Update bigram frequencies with overlap
    let mut bi_tokens: Vec<&str> = bigram_overlap.iter().map(|s| s.as_str()).collect();
    bi_tokens.extend(chunk_words.iter());
    let new_bigrams = ngram::ngram_frequencies(&bi_tokens, 2);
    for (ng, count) in &new_bigrams {
        *bigram_freqs.entry(ng.clone()).or_insert(0) += count;
    }
    *bigram_overlap = if chunk_words.len() >= 1 {
        chunk_words[chunk_words.len() - 1..].iter().map(|s| s.to_string()).collect()
    } else {
        chunk_words.iter().map(|s| s.to_string()).collect()
    };

    // Update trigram frequencies with overlap
    let mut tri_tokens: Vec<&str> = trigram_overlap.iter().map(|s| s.as_str()).collect();
    tri_tokens.extend(chunk_words.iter());
    let new_trigrams = ngram::ngram_frequencies(&tri_tokens, 3);
    for (ng, count) in &new_trigrams {
        *trigram_freqs.entry(ng.clone()).or_insert(0) += count;
    }
    *trigram_overlap = if chunk_words.len() >= 2 {
        chunk_words[chunk_words.len() - 2..].iter().map(|s| s.to_string()).collect()
    } else {
        chunk_words.iter().map(|s| s.to_string()).collect()
    };
}
```

**Step 4: Run all tests**

Run: `cargo test 2>&1`
Expected: ALL PASS

**Step 5: Commit**

```bash
git add src/streaming.rs tests/streaming_test.rs
git commit -m "fix: rewrite streaming entropy to use incremental frequency maps instead of O(n^2) word accumulation"
```

---

### Task 5: Replace .unwrap() with proper error propagation in Python bindings

**Files:**
- Modify: `corpa-python/src/lib.rs`

**Step 1: Implement the fix**

Replace every `serde_json::to_string(&result).unwrap()` with:

```rust
serde_json::to_string(&result)
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?
```

There are 8 occurrences (stats, ngrams, entropy, readability, perplexity, lang, tokens, zipf).

**Step 2: Verify it compiles**

Run: `cargo check -p corpa-python 2>&1`
Expected: Compiles without errors (note: full build requires Python dev headers, `check` may suffice)

**Step 3: Commit**

```bash
git add corpa-python/src/lib.rs
git commit -m "fix: replace .unwrap() with proper error propagation in Python bindings"
```

---

### Task 6: Fix WASM package.json

**Files:**
- Modify: `corpa-wasm/package.json`

**Step 1: Implement the fix**

Replace the contents of `corpa-wasm/package.json` with correct filenames matching the generated `pkg/` output, add `"type": "module"`, `sideEffects`, and fix license:

```json
{
  "name": "@flurry13/corpa",
  "version": "0.4.11",
  "description": "Blazing-fast text analysis powered by Rust, compiled to WebAssembly.",
  "type": "module",
  "main": "corpa_wasm.js",
  "types": "corpa_wasm.d.ts",
  "files": [
    "corpa_wasm_bg.wasm",
    "corpa_wasm.js",
    "corpa_wasm_bg.js",
    "corpa_wasm.d.ts"
  ],
  "sideEffects": [
    "./corpa_wasm.js",
    "./snippets/*"
  ],
  "keywords": ["nlp", "text-analysis", "wasm", "rust"],
  "license": "MIT OR Apache-2.0"
}
```

**Step 2: Commit**

```bash
git add corpa-wasm/package.json
git commit -m "fix: correct WASM package.json filenames, add type module, fix license"
```

---

### Task 7: Run full test suite and verify all fixes

**Step 1: Run all tests**

Run: `cargo test -p corpa 2>&1`
Expected: ALL PASS

**Step 2: Manually verify key fixes**

```bash
# Verify n=0 streaming no longer panics
echo "hello world" | cargo run -- ngrams -n 0 --stream 2>&1
# Expected: Error message, not a panic

# Verify JSON output has typed values
cargo run -- stats tests/fixtures/prose.txt --format json 2>&1
# Expected: "value": 126 (number), not "value": "126" (string)

# Verify entropy redundancy is clamped
cargo run -- entropy tests/fixtures/single-word.txt --format json 2>&1
# Expected: redundancy between 0 and 1
```

**Step 3: Commit all verified**

No additional commit needed — each task committed independently.

---

## Summary of All Bugs Fixed

| # | Bug | Severity | File(s) |
|---|-----|----------|---------|
| 1 | n=0 panic in ngram_frequencies and streaming | Medium | ngram.rs, results.rs, streaming.rs |
| 2 | JSON output all values as strings | Medium | output.rs |
| 3 | Entropy redundancy unclamped | Medium | entropy.rs |
| 4 | Streaming entropy O(n^2) memory | High | streaming.rs |
| 5 | Python .unwrap() crash risk | Medium | corpa-python/src/lib.rs |
| 6 | WASM package.json wrong filenames | Critical | corpa-wasm/package.json |
