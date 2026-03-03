use std::io::Write;
use std::process::{Command, Stdio};

fn corpa_stdin(args: &[&str], input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_corpa"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn test_stream_stats_json() {
    let input = "hello world hello\nfoo bar baz\n".repeat(100);
    let out = corpa_stdin(
        &["stats", "--stream", "--chunk-lines", "50", "--format", "json"],
        &input,
    );
    let lines: Vec<&str> = out.trim().lines().collect();
    assert!(
        lines.len() >= 2,
        "should emit multiple JSON lines, got {}",
        lines.len()
    );
    for line in &lines {
        let parsed: serde_json::Value = serde_json::from_str(line).unwrap();
        assert!(parsed.get("tokens").is_some());
        assert!(parsed.get("chunk").is_some());
    }
}

#[test]
fn test_stream_unsupported_command() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_corpa"))
        .args(&["readability", "--stream"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"test")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("not supported") || !output.status.success());
}

#[test]
fn test_stream_ngrams_n_zero_errors() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_corpa"))
        .args(&["ngrams", "-n", "0", "--stream"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"hello world\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(!output.status.success(), "should fail for n=0");
}

#[test]
fn test_stream_entropy_json() {
    let input = "the cat sat on the mat\nthe dog sat on the rug\n".repeat(100);
    let out = corpa_stdin(
        &[
            "entropy",
            "--stream",
            "--chunk-lines",
            "50",
            "--format",
            "json",
        ],
        &input,
    );
    let lines: Vec<&str> = out.trim().lines().collect();
    assert!(
        lines.len() >= 2,
        "should emit multiple JSON lines, got {}",
        lines.len()
    );
    for line in &lines {
        let parsed: serde_json::Value = serde_json::from_str(line).unwrap();
        assert!(parsed.get("h1").is_some());
        assert!(parsed.get("vocabulary_size").is_some());
        assert!(parsed.get("chunk").is_some());
    }
}
