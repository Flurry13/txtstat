use std::process::Command;

fn txtstat(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_txtstat"))
        .args(args)
        .output()
        .expect("failed to run txtstat");
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn test_stats_json() {
    let out = txtstat(&["stats", "tests/fixtures/small.txt", "--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    let tokens_row = records
        .iter()
        .find(|r| r.get("metric").and_then(|m| m.as_str()) == Some("Tokens (words)"))
        .expect("missing Tokens row");
    let token_val: usize = tokens_row["value"]
        .as_str()
        .unwrap()
        .replace(',', "")
        .parse()
        .unwrap();
    assert!(token_val > 0);
}

#[test]
fn test_stats_table_output() {
    let out = txtstat(&["stats", "tests/fixtures/small.txt"]);
    assert!(out.contains("Tokens"));
    assert!(out.contains("Types"));
    assert!(out.contains("Type-Token Ratio"));
}

#[test]
fn test_stats_csv() {
    let out = txtstat(&["stats", "tests/fixtures/small.txt", "--format", "csv"]);
    assert!(out.contains("Metric,Value"));
}

#[test]
fn test_ngrams_bigrams_json() {
    let out = txtstat(&[
        "ngrams",
        "-n",
        "2",
        "--top",
        "3",
        "tests/fixtures/small.txt",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    assert!(records.len() <= 3);
}

#[test]
fn test_ngrams_case_insensitive() {
    let out = txtstat(&[
        "ngrams",
        "--case-insensitive",
        "tests/fixtures/small.txt",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert!(!parsed.as_array().unwrap().is_empty());
}

#[test]
fn test_tokens_command() {
    let out = txtstat(&[
        "tokens",
        "tests/fixtures/small.txt",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    assert!(records.len() >= 3);
}

#[test]
fn test_stats_empty_file() {
    let out = txtstat(&["stats", "tests/fixtures/empty.txt", "--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert!(!parsed.as_array().unwrap().is_empty());
}

#[test]
fn test_stats_single_word() {
    let out = txtstat(&["stats", "tests/fixtures/single-word.txt", "--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    let tokens_row = records.iter().find(|r| {
        r.get("metric").and_then(|m| m.as_str()) == Some("Tokens (words)")
    }).unwrap();
    assert_eq!(tokens_row["value"].as_str().unwrap(), "1");
}

#[test]
fn test_stdin_input() {
    use std::io::Write;
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_txtstat"))
        .args(&["stats", "--format", "json"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    child.stdin.take().unwrap().write_all(b"hello world hello").unwrap();
    let output = child.wait_with_output().unwrap();
    let out = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert!(!parsed.as_array().unwrap().is_empty());
}

// --- v0.2.0 integration tests ---

#[test]
fn test_readability_json() {
    let out = txtstat(&["readability", "tests/fixtures/prose.txt", "--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    assert_eq!(records.len(), 5, "expected 5 readability metrics");
    let metrics: Vec<&str> = records.iter().map(|r| r["metric"].as_str().unwrap()).collect();
    assert!(metrics.contains(&"Flesch-Kincaid Grade"));
    assert!(metrics.contains(&"Flesch Reading Ease"));
    assert!(metrics.contains(&"Coleman-Liau Index"));
    assert!(metrics.contains(&"Gunning Fog Index"));
    assert!(metrics.contains(&"SMOG Index"));
}

#[test]
fn test_readability_table() {
    let out = txtstat(&["readability", "tests/fixtures/prose.txt"]);
    assert!(out.contains("Flesch-Kincaid Grade"));
    assert!(out.contains("Score"));
    assert!(out.contains("Grade"));
}

#[test]
fn test_entropy_json() {
    let out = txtstat(&["entropy", "tests/fixtures/prose.txt", "--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    assert!(records.len() >= 6, "expected at least 6 entropy rows");
}

#[test]
fn test_zipf_json() {
    let out = txtstat(&["zipf", "tests/fixtures/prose.txt", "--format", "json", "--top", "5"]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    assert!(records.len() >= 5, "expected at least 5 zipf rows, got {}", records.len());
}

#[test]
fn test_zipf_plot() {
    let out = txtstat(&["zipf", "--plot", "tests/fixtures/prose.txt"]);
    assert!(out.contains("Distribution"));
    assert!(out.contains("Zipf Exponent"));
}

#[test]
fn test_ngrams_with_stopwords() {
    let out = txtstat(&[
        "ngrams",
        "--stopwords",
        "english",
        "tests/fixtures/small.txt",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    for record in records {
        let word = record["unigram"].as_str().unwrap();
        let lower = word.trim_matches('"').to_lowercase();
        assert!(
            lower != "the" && lower != "on" && lower != "in" && lower != "was",
            "stopword '{}' should be filtered",
            lower
        );
    }
}

#[test]
fn test_stats_with_stopwords() {
    let out = txtstat(&[
        "stats",
        "--stopwords",
        "english",
        "tests/fixtures/small.txt",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    let sw_row = records
        .iter()
        .find(|r| r.get("metric").and_then(|m| m.as_str()) == Some("Stopwords Removed"));
    assert!(sw_row.is_some(), "should have Stopwords Removed row");
    let removed: usize = sw_row.unwrap()["value"]
        .as_str()
        .unwrap()
        .replace(',', "")
        .parse()
        .unwrap();
    assert!(removed > 0, "should have removed some stopwords");
}

#[test]
fn test_ngrams_with_custom_stopwords_file() {
    let out = txtstat(&[
        "ngrams",
        "--stopwords",
        "tests/fixtures/stopwords_test.txt",
        "tests/fixtures/small.txt",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    for record in records {
        let word = record["unigram"].as_str().unwrap();
        let lower = word.trim_matches('"').to_lowercase();
        assert!(
            lower != "cat" && lower != "dog" && lower != "the",
            "custom stopword '{}' should be filtered",
            lower
        );
    }
}

// --- v0.3.0 integration tests ---

#[test]
fn test_perplexity_json() {
    let out = txtstat(&[
        "perplexity",
        "tests/fixtures/prose.txt",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    let pp_row = records
        .iter()
        .find(|r| r.get("metric").and_then(|m| m.as_str()) == Some("Perplexity"))
        .expect("missing Perplexity row");
    let pp_val: f64 = pp_row["value"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    assert!(pp_val.is_finite(), "perplexity should be finite");
    assert!(pp_val > 0.0, "perplexity should be positive");
}

#[test]
fn test_perplexity_smoothing_options() {
    // Test laplace
    let out_laplace = txtstat(&[
        "perplexity",
        "tests/fixtures/prose.txt",
        "--smoothing",
        "laplace",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out_laplace).unwrap();
    let records = parsed.as_array().unwrap();
    let smoothing_row = records
        .iter()
        .find(|r| r.get("metric").and_then(|m| m.as_str()) == Some("Smoothing"))
        .unwrap();
    assert!(smoothing_row["value"].as_str().unwrap().contains("Add-k"));

    // Test backoff
    let out_backoff = txtstat(&[
        "perplexity",
        "tests/fixtures/prose.txt",
        "--smoothing",
        "backoff",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out_backoff).unwrap();
    let records = parsed.as_array().unwrap();
    let smoothing_row = records
        .iter()
        .find(|r| r.get("metric").and_then(|m| m.as_str()) == Some("Smoothing"))
        .unwrap();
    assert!(smoothing_row["value"]
        .as_str()
        .unwrap()
        .contains("Stupid Backoff"));
}

#[test]
fn test_lang_json() {
    let out = txtstat(&["lang", "tests/fixtures/prose.txt", "--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    let lang_row = records
        .iter()
        .find(|r| r.get("metric").and_then(|m| m.as_str()) == Some("Language"))
        .expect("missing Language row");
    assert_eq!(lang_row["value"].as_str().unwrap(), "English");
}

#[test]
fn test_lang_french() {
    let out = txtstat(&["lang", "tests/fixtures/french.txt", "--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    let lang_row = records
        .iter()
        .find(|r| r.get("metric").and_then(|m| m.as_str()) == Some("Language"))
        .expect("missing Language row");
    assert_eq!(lang_row["value"].as_str().unwrap(), "Français");
}

#[test]
fn test_tokens_with_bpe() {
    let out = txtstat(&[
        "tokens",
        "tests/fixtures/prose.txt",
        "--model",
        "gpt4",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    let bpe_row = records
        .iter()
        .find(|r| {
            r.get("tokenizer")
                .and_then(|t| t.as_str())
                .map(|s| s.contains("GPT-4"))
                .unwrap_or(false)
        })
        .expect("missing BPE (GPT-4) row");
    let count: usize = bpe_row["tokens"]
        .as_str()
        .unwrap()
        .replace(',', "")
        .parse()
        .unwrap();
    assert!(count > 0, "BPE token count should be positive");
}

#[test]
fn test_tokens_backward_compatible() {
    let out = txtstat(&[
        "tokens",
        "tests/fixtures/prose.txt",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    assert_eq!(records.len(), 3, "without --model, should have exactly 3 rows");
    let tokenizers: Vec<&str> = records
        .iter()
        .map(|r| r["tokenizer"].as_str().unwrap())
        .collect();
    assert!(!tokenizers.iter().any(|t| t.contains("BPE")), "no BPE rows without --model");
}
