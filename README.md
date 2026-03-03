<div align="center">

# corpa

**Blazing-fast text analysis for the command line, Python, and the browser.**

A unified tool for corpus-level NLP statistics — n-gram frequencies, readability scores, entropy analysis, language detection, BPE token counting, and more — written in Rust for performance, with bindings for Python and JavaScript/WASM.

[Installation](#installation) · [Quick Start](#quick-start) · [Commands](#commands) · [Documentation](#documentation) · [Contributing](#contributing)

</div>

---

## Highlights

- **High performance** — Parallel processing via `rayon`. Analyzes multi-GB corpora in seconds.
- **Composable** — Unix-friendly design with structured output (JSON, CSV, table). Pipes seamlessly with `jq`, `awk`, and standard tooling.
- **Comprehensive** — Eight analysis commands covering vocabulary statistics, n-gram frequencies, readability indices, Shannon entropy, Zipf's law, language model perplexity, language detection, and BPE tokenization.
- **Multi-platform** — Available as a native CLI binary, a Python package via PyO3, and an npm/WASM module for browser and Node.js environments.
- **Streaming** — Process unbounded stdin streams with incremental chunk-based output for `stats`, `ngrams`, and `entropy`.

---

## Installation

### CLI

```bash
cargo install corpa
```

Or build from source:

```bash
git clone https://github.com/Flurry13/corpa
cd corpa
cargo build --release
```

### Python

```bash
pip install corpa
```

### JavaScript / WASM

> npm support coming soon.

---

## Quick Start

### CLI

```bash
corpa stats corpus.txt
corpa ngrams -n 2 --top 20 corpus.txt
corpa readability essay.txt
corpa entropy corpus.txt
corpa perplexity corpus.txt --smoothing laplace
corpa lang mystery.txt
corpa tokens corpus.txt --model gpt4
corpa zipf corpus.txt --top 10
```

All commands accept file paths, directories (with `--recursive`), or stdin. Output format is controlled with `--format` (`table`, `json`, `csv`).

### Python

```python
import corpa

corpa.stats(text="The quick brown fox jumps over the lazy dog.")
# {'tokens': 9, 'types': 8, 'sentences': 1, 'type_token_ratio': 0.8889, ...}

corpa.ngrams("corpus.txt", n=2, top=10)
# [{'ngram': 'of the', 'frequency': 4521, 'relative_pct': 2.09}, ...]

corpa.lang(text="Bonjour le monde")
# {'language': 'Français', 'code': 'fra', 'script': 'Latin', 'confidence': 0.99}
```

All functions accept a file path as the first argument or a `text=` keyword argument for direct string input.

### JavaScript / WASM

> support coming soon.
---

## Commands

| Command | Description |
|---------|-------------|
| `stats` | Token, type, sentence counts, type-token ratio, hapax legomena, average sentence length |
| `ngrams` | N-gram frequency analysis with configurable N, top-K, minimum frequency, case folding, stopword filtering |
| `tokens` | Whitespace, sentence, and character tokenization; BPE token counts for GPT-3, GPT-4, and GPT-4o |
| `readability` | Flesch-Kincaid Grade, Flesch Reading Ease, Coleman-Liau Index, Gunning Fog Index, SMOG Index |
| `entropy` | Unigram, bigram, and trigram Shannon entropy; entropy rate; vocabulary redundancy |
| `perplexity` | N-gram language model perplexity with Laplace smoothing and Stupid Backoff |
| `lang` | Language and script detection with confidence scoring |
| `zipf` | Zipf's law rank-frequency distribution with exponent fitting and terminal sparkline plotting |
| `completions` | Shell completion generation for bash, zsh, and fish |

### Example Output

```
$ corpa stats prose.txt

  corpa · prose.txt
┌─────────────────────┬────────────┐
│ Metric              ┆      Value │
╞═════════════════════╪════════════╡
│ Tokens (words)      ┆        175 │
│ Types (unique)      ┆         95 │
│ Characters          ┆        805 │
│ Sentences           ┆          6 │
│ Type-Token Ratio    ┆     0.5429 │
│ Hapax Legomena      ┆ 70 (73.7%) │
│ Avg Sentence Length ┆ 29.2 words │
└─────────────────────┴────────────┘
```

```
$ corpa readability prose.txt

  corpa · prose.txt
┌──────────────────────┬───────┬─────────────┐
│ Metric               ┆ Score ┆       Grade │
╞══════════════════════╪═══════╪═════════════╡
│ Flesch-Kincaid Grade ┆ 12.73 ┆ High School │
│ Flesch Reading Ease  ┆ 41.16 ┆   Difficult │
│ Coleman-Liau Index   ┆ 13.82 ┆     College │
│ Gunning Fog Index    ┆ 16.97 ┆     College │
│ SMOG Index           ┆ 14.62 ┆     College │
└──────────────────────┴───────┴─────────────┘
```

```
$ corpa tokens prose.txt --model all

  corpa · prose.txt
┌──────────────┬────────┐
│ Tokenizer    ┆ Tokens │
╞══════════════╪════════╡
│ Whitespace   ┆    126 │
│ Sentences    ┆      6 │
│ Characters   ┆    805 │
│ BPE (GPT-4)  ┆    150 │
│ BPE (GPT-4o) ┆    148 │
│ BPE (GPT-3)  ┆    151 │
└──────────────┴────────┘
```

---

## Streaming

The `--stream` flag enables incremental processing of unbounded stdin, emitting cumulative results after each chunk. Chunk size is configurable with `--chunk-lines` (default: 1000).

```bash
cat huge_corpus.txt | corpa stats --stream --chunk-lines 500 --format json
```

Supported commands: `stats`, `ngrams`, `entropy`.

| Format | Behavior |
|--------|----------|
| `json` | JSON Lines — one object per chunk |
| `csv` | Header row once, data rows per chunk |
| `table` | Table per chunk with chunk number in title |

---

## Global Options

| Flag | Description |
|------|-------------|
| `--format <fmt>` | Output format: `table` (default), `json`, `csv` |
| `--recursive` | Process directories recursively |
| `--stream` | Process stdin incrementally, emitting results per chunk |
| `--chunk-lines <N>` | Lines per chunk in streaming mode (default: 1000) |

---

## Performance

Benchmarks on a 1GB English text corpus (Apple M2, 8 cores):

| Command | corpa | Python | Speedup |
|---------|---------|--------|---------|
| Word count | 1.9s | 11.5s | **6x** |
| Bigram frequency | 3.4s | 53.9s | **16x** |
| Readability | 5.4s | 107.9s | **20x** |

> Benchmarked on ~1 GB generated English text corpus (Apple M-series, 8 cores).

---

## Documentation

| Resource | Description |
|----------|-------------|
| [CLI Commands](#commands) | Full command reference with options and examples |
| [Streaming](#streaming) | Incremental stdin processing for large-scale analysis |
| [Python API](#python) | PyO3 bindings — all commands as native Python functions |
| [JavaScript API](#javascript--wasm) | WASM bindings for browser and Node.js environments |

---

## Roadmap

### Completed

- **v0.1.0 — Core CLI**: `stats`, `ngrams`, `tokens`, JSON/CSV/table output, stdin and file input, recursive directories
- **v0.2.0 — Analysis**: `readability`, `entropy`, `zipf`, stopword filtering, case folding, parallel processing
- **v0.3.0 — Language Models**: `perplexity` with Laplace/Stupid Backoff, `lang` detection, BPE token counting
- **v0.4.0 — Ecosystem**: Python bindings (PyO3), WASM/npm package, streaming mode, shell completions

### Planned

#### v0.5.0 — Robustness & Output Quality

- **Typed JSON output** — Emit numeric values as JSON numbers instead of comma-formatted strings for proper `jq` interoperability
- **Improved sentence detection** — Collapse consecutive sentence-ending punctuation (e.g., `...` = 1, `?!` = 1) and handle common abbreviations (Mr., Dr., U.S.A.)
- **Input validation hardening** — Guard against `n=0` panic in streaming n-gram path; clamp entropy redundancy to [0, 1]; validate parameters in public library API
- **Unicode-aware syllable counting** — Recognize accented vowels (e, i, o, u) for correct readability scores on non-English Latin-script text
- **Streaming entropy rewrite** — Replace O(n^2) accumulate-all-words approach with incremental entropy estimation that respects memory bounds

#### v0.6.0 — Bindings Parity

- **Python: expose missing parameters** — Stopword filtering, `min_freq`, `case_insensitive` for `ngrams`; stopwords for `stats`; BPE model selection for `tokens`
- **Python: replace `.unwrap()` with proper error propagation** — Prevent potential interpreter crashes on serialization failures
- **Python: add docstrings and `.pyi` type stubs** — Enable IDE autocompletion and `mypy`/`pyright` support
- **WASM: fix `package.json`** — Correct entry point filenames, add missing `corpa_wasm_bg.js`, set `"type": "module"`, reconcile scoped package name
- **WASM: expose filtering parameters** — Stopwords, `min_freq`, `case_insensitive` for `ngrams` and `stats`
- **WASM: TypeScript type definitions** — Generate proper interfaces via `tsify` instead of returning `any`
- **Version synchronization** — Align Python (0.4.1) and WASM (0.4.0) package versions with main crate

#### v0.7.0 — Corpus Comparison & Search

- Concordance / KWIC (keyword in context) search
- Diff mode for comparing two corpora — side-by-side statistics, vocabulary overlap, divergence metrics
- Collocation analysis (PMI, log-likelihood, chi-squared)
- Custom vocabulary and dictionary support

#### v0.8.0 — Advanced Analysis

- Sentiment lexicon scoring (AFINN, VADER-style)
- Topic segmentation and keyword extraction (TF-IDF)
- Text complexity profiling — combined readability + entropy + vocabulary richness report
- Configurable sentence tokenizer (regex-based or rule-based)

#### Future

- Language-specific stopword lists (bundled for top 10 languages)
- Plugin system for custom analysis modules
- Interactive TUI mode with live statistics
- Parallel streaming for multi-file batch processing
- Wasm streaming API for browser-based incremental analysis

---

## Contributing

Contributions are welcome. Please open an issue to discuss proposed changes before submitting a pull request.

```bash
cargo test            # Run test suite
cargo clippy          # Lint
cargo bench           # Run benchmarks
```

---

## License

This project is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.

---

## Acknowledgments

- [rayon](https://github.com/rayon-rs/rayon) — Data parallelism
- [clap](https://github.com/clap-rs/clap) — CLI argument parsing
- [comfy-table](https://github.com/nuber-io/comfy-table) — Terminal table rendering
- [whatlang](https://github.com/grstreten/whatlang-rs) — Language detection
- [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) — BPE tokenization for GPT models
- [PyO3](https://github.com/PyO3/pyo3) — Rust bindings for Python
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) — Rust/WebAssembly interop
