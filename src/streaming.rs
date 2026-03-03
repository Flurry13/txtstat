use crate::analysis::{counter, entropy, ngram, tokenizer};
use crate::cli::OutputFormat;
use crate::output::ResultTable;
use crate::utils::format::format_num;
use anyhow::{bail, Result};
use rustc_hash::FxHashMap;
use std::io::{self, BufRead};

struct StreamState {
    word_freqs: FxHashMap<String, usize>,
    total_tokens: usize,
    total_chars: usize,
    total_sentences: usize,
    chunk_count: usize,
}

impl StreamState {
    fn new() -> Self {
        Self {
            word_freqs: FxHashMap::default(),
            total_tokens: 0,
            total_chars: 0,
            total_sentences: 0,
            chunk_count: 0,
        }
    }
}

pub fn stream_stats(format: &OutputFormat, chunk_lines: usize) -> Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut state = StreamState::new();
    let mut line_buf = Vec::new();
    let mut first_csv = true;

    for line in reader.lines() {
        line_buf.push(line?);
        if line_buf.len() >= chunk_lines {
            let chunk = line_buf.join("\n");
            process_stats_chunk(&chunk, &mut state);
            emit_stats(&state, format, &mut first_csv)?;
            line_buf.clear();
        }
    }
    if !line_buf.is_empty() {
        let chunk = line_buf.join("\n");
        process_stats_chunk(&chunk, &mut state);
        emit_stats(&state, format, &mut first_csv)?;
    }
    Ok(())
}

fn process_stats_chunk(chunk: &str, state: &mut StreamState) {
    let freqs = counter::word_frequencies(chunk);
    for (word, count) in &freqs {
        *state.word_freqs.entry(word.clone()).or_insert(0) += count;
    }
    state.total_tokens += counter::token_count(&freqs);
    state.total_chars += tokenizer::char_count(chunk);
    state.total_sentences += tokenizer::sentence_count(chunk);
    state.chunk_count += 1;
}

fn emit_stats(state: &StreamState, format: &OutputFormat, first_csv: &mut bool) -> Result<()> {
    let types = state.word_freqs.len();
    let ttr = if state.total_tokens > 0 {
        types as f64 / state.total_tokens as f64
    } else {
        0.0
    };

    match format {
        OutputFormat::Json => {
            let obj = serde_json::json!({
                "chunk": state.chunk_count,
                "tokens": state.total_tokens,
                "types": types,
                "characters": state.total_chars,
                "sentences": state.total_sentences,
                "type_token_ratio": format!("{:.4}", ttr),
            });
            println!("{}", serde_json::to_string(&obj)?);
        }
        OutputFormat::Csv => {
            if *first_csv {
                println!("chunk,tokens,types,characters,sentences,type_token_ratio");
                *first_csv = false;
            }
            println!(
                "{},{},{},{},{},{:.4}",
                state.chunk_count, state.total_tokens, types, state.total_chars,
                state.total_sentences, ttr
            );
        }
        OutputFormat::Table => {
            let mut table = ResultTable::new(
                format!("stream (chunk {})", state.chunk_count),
                vec!["Metric", "Value"],
            );
            table.add_row(vec![
                "Tokens (words)".into(),
                format_num(state.total_tokens),
            ]);
            table.add_row(vec!["Types (unique)".into(), format_num(types)]);
            table.add_row(vec![
                "Characters".into(),
                format_num(state.total_chars),
            ]);
            table.add_row(vec![
                "Sentences".into(),
                format_num(state.total_sentences),
            ]);
            table.add_row(vec!["Type-Token Ratio".into(), format!("{:.4}", ttr)]);
            print!("{}", table.render(format)?);
        }
    }
    Ok(())
}

pub fn stream_ngrams(
    format: &OutputFormat,
    chunk_lines: usize,
    n: usize,
    top: usize,
) -> Result<()> {
    anyhow::ensure!(n >= 1, "n-gram size must be at least 1");
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut ngram_freqs: FxHashMap<String, usize> = FxHashMap::default();
    let mut overlap_tokens: Vec<String> = Vec::new();
    let mut chunk_count = 0usize;
    let mut line_buf = Vec::new();
    let mut first_csv = true;

    for line in reader.lines() {
        line_buf.push(line?);
        if line_buf.len() >= chunk_lines {
            let chunk = line_buf.join("\n");
            process_ngram_chunk(&chunk, &mut ngram_freqs, &mut overlap_tokens, n);
            chunk_count += 1;
            emit_ngrams(&ngram_freqs, format, chunk_count, n, top, &mut first_csv)?;
            line_buf.clear();
        }
    }
    if !line_buf.is_empty() {
        let chunk = line_buf.join("\n");
        process_ngram_chunk(&chunk, &mut ngram_freqs, &mut overlap_tokens, n);
        chunk_count += 1;
        emit_ngrams(&ngram_freqs, format, chunk_count, n, top, &mut first_csv)?;
    }
    Ok(())
}

fn process_ngram_chunk(
    chunk: &str,
    ngram_freqs: &mut FxHashMap<String, usize>,
    overlap_tokens: &mut Vec<String>,
    n: usize,
) {
    let chunk_words = tokenizer::words(chunk);
    let mut all_tokens: Vec<&str> = overlap_tokens.iter().map(|s| s.as_str()).collect();
    all_tokens.extend(chunk_words.iter());

    let new_freqs = ngram::ngram_frequencies(&all_tokens, n);
    for (ngram_str, count) in &new_freqs {
        *ngram_freqs.entry(ngram_str.clone()).or_insert(0) += count;
    }

    // Save last n-1 tokens for next chunk overlap
    *overlap_tokens = if chunk_words.len() >= n.saturating_sub(1) {
        chunk_words[chunk_words.len() - (n.saturating_sub(1))..]
            .iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        chunk_words.iter().map(|s| s.to_string()).collect()
    };
}

fn emit_ngrams(
    freqs: &FxHashMap<String, usize>,
    format: &OutputFormat,
    chunk_count: usize,
    n: usize,
    top: usize,
    first_csv: &mut bool,
) -> Result<()> {
    let mut sorted: Vec<_> = freqs.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    sorted.truncate(top);

    let total: usize = freqs.values().sum();
    let label = match n {
        1 => "Unigram",
        2 => "Bigram",
        3 => "Trigram",
        _ => "N-gram",
    };

    match format {
        OutputFormat::Json => {
            let entries: Vec<serde_json::Value> = sorted
                .iter()
                .map(|(ng, &freq)| {
                    let pct = if total > 0 {
                        freq as f64 / total as f64 * 100.0
                    } else {
                        0.0
                    };
                    serde_json::json!({
                        "chunk": chunk_count,
                        "ngram": ng,
                        "frequency": freq,
                        "relative_pct": format!("{:.2}", pct),
                    })
                })
                .collect();
            for entry in entries {
                println!("{}", serde_json::to_string(&entry)?);
            }
        }
        OutputFormat::Csv => {
            if *first_csv {
                println!("chunk,{},frequency,relative_pct", label.to_lowercase());
                *first_csv = false;
            }
            for (ng, &freq) in &sorted {
                let pct = if total > 0 {
                    freq as f64 / total as f64 * 100.0
                } else {
                    0.0
                };
                println!("{},{},{},{:.2}", chunk_count, ng, freq, pct);
            }
        }
        OutputFormat::Table => {
            let mut table = ResultTable::new(
                format!("stream (chunk {})", chunk_count),
                vec![label, "Freq", "Rel %"],
            );
            for (ng, &freq) in &sorted {
                let pct = if total > 0 {
                    freq as f64 / total as f64 * 100.0
                } else {
                    0.0
                };
                table.add_row(vec![
                    format!("\"{}\"", ng),
                    format_num(freq),
                    format!("{:.2}%", pct),
                ]);
            }
            print!("{}", table.render(format)?);
        }
    }
    Ok(())
}

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
    let chunk_freqs = counter::word_frequencies(chunk);
    for (word, count) in &chunk_freqs {
        *word_freqs.entry(word.clone()).or_insert(0) += count;
    }

    let chunk_words = tokenizer::words(chunk);

    // Update bigram frequencies with overlap for cross-chunk boundary
    let mut bi_tokens: Vec<&str> = bigram_overlap.iter().map(|s| s.as_str()).collect();
    bi_tokens.extend(chunk_words.iter());
    let new_bigrams = ngram::ngram_frequencies(&bi_tokens, 2);
    for (ng, count) in &new_bigrams {
        *bigram_freqs.entry(ng.clone()).or_insert(0) += count;
    }
    *bigram_overlap = if chunk_words.len() >= 1 {
        chunk_words[chunk_words.len() - 1..]
            .iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        chunk_words.iter().map(|s| s.to_string()).collect()
    };

    // Update trigram frequencies with overlap for cross-chunk boundary
    let mut tri_tokens: Vec<&str> = trigram_overlap.iter().map(|s| s.as_str()).collect();
    tri_tokens.extend(chunk_words.iter());
    let new_trigrams = ngram::ngram_frequencies(&tri_tokens, 3);
    for (ng, count) in &new_trigrams {
        *trigram_freqs.entry(ng.clone()).or_insert(0) += count;
    }
    *trigram_overlap = if chunk_words.len() >= 2 {
        chunk_words[chunk_words.len() - 2..]
            .iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        chunk_words.iter().map(|s| s.to_string()).collect()
    };
}

fn emit_entropy(
    chunk: usize,
    h1: f64,
    h2: f64,
    h3: f64,
    rate: f64,
    vocab: usize,
    redund: f64,
    format: &OutputFormat,
    first_csv: &mut bool,
) -> Result<()> {
    match format {
        OutputFormat::Json => {
            let obj = serde_json::json!({
                "chunk": chunk,
                "h1": format!("{:.4}", h1),
                "h2": format!("{:.4}", h2),
                "h3": format!("{:.4}", h3),
                "entropy_rate": format!("{:.4}", rate),
                "vocabulary_size": vocab,
                "redundancy": format!("{:.4}", redund),
            });
            println!("{}", serde_json::to_string(&obj)?);
        }
        OutputFormat::Csv => {
            if *first_csv {
                println!("chunk,h1,h2,h3,entropy_rate,vocabulary_size,redundancy");
                *first_csv = false;
            }
            println!(
                "{},{:.4},{:.4},{:.4},{:.4},{},{:.4}",
                chunk, h1, h2, h3, rate, vocab, redund
            );
        }
        OutputFormat::Table => {
            let mut table = ResultTable::new(
                format!("stream (chunk {})", chunk),
                vec!["Metric", "Value"],
            );
            table.add_row(vec![
                "H1 (Unigram Entropy)".into(),
                format!("{:.4}", h1),
            ]);
            table.add_row(vec![
                "H2 (Bigram Entropy)".into(),
                format!("{:.4}", h2),
            ]);
            table.add_row(vec![
                "H3 (Trigram Entropy)".into(),
                format!("{:.4}", h3),
            ]);
            table.add_row(vec!["Entropy Rate".into(), format!("{:.4}", rate)]);
            table.add_row(vec!["Vocabulary Size".into(), format_num(vocab)]);
            table.add_row(vec!["Redundancy".into(), format!("{:.4}", redund)]);
            print!("{}", table.render(format)?);
        }
    }
    Ok(())
}

pub fn unsupported(command_name: &str) -> Result<()> {
    bail!(
        "--stream is not supported for the '{}' command. Supported: stats, ngrams, entropy",
        command_name
    )
}
