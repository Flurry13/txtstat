<div align="center">

# lexis

**Blazing-fast text analysis for the command line, Python, and the browser.**

A unified tool for corpus-level NLP statistics — n-gram frequencies, readability scores, entropy analysis, language detection, BPE token counting, and more — written in Rust for performance, with bindings for Python and JavaScript/WASM.

[Installation](#installation) · [Quick Start](#quick-start) · [Commands](#commands) · [Documentation](#documentation) · [Contributing](#contributing)

</div>

---

## Highlights

- **High performance** — Parallel processing via `rayon`. Analyzes multi-GB corpora in seconds.
- **Composable** — Unix-friendly design with structured output (JSON, CSV, table). Pipes seamlessly with `jq`, `awk`, and standard tooling.
- **Comprehensive** — Nine analysis commands covering vocabulary statistics, n-gram frequencies, readability indices, Shannon entropy, Zipf's law, language model perplexity, language detection, and BPE tokenization.
- **Multi-platform** — Available as a native CLI binary, a Python package via PyO3, and an npm/WASM module for browser and Node.js environments.
- **Streaming** — Process unbounded stdin streams with incremental chunk-based output for `stats`, `ngrams`, and `entropy`.

---

## Installation

### CLI

```bash
cargo install lexis
```

Or build from source:

```bash
git clone https://github.com/Flurry13/lexis
cd lexis
cargo build --release
```

### Python

```bash
pip install lexis
```

### JavaScript / WASM

```bash
npm install lexis
```

---

## Quick Start

### CLI

```bash
lexis stats corpus.txt
lexis ngrams -n 2 --top 20 corpus.txt
lexis readability essay.txt
lexis entropy corpus.txt
lexis perplexity corpus.txt --smoothing laplace
lexis lang mystery.txt
lexis tokens corpus.txt --model gpt4
lexis zipf corpus.txt --top 10
```

All commands accept file paths, directories (with `--recursive`), or stdin. Output format is controlled with `--format` (`table`, `json`, `csv`).

### Python

```python
import lexis

lexis.stats(text="The quick brown fox jumps over the lazy dog.")
# {'tokens': 9, 'types': 8, 'sentences': 1, 'type_token_ratio': 0.8889, ...}

lexis.ngrams("corpus.txt", n=2, top=10)
# [{'ngram': 'of the', 'frequency': 4521, 'relative_pct': 2.09}, ...]

lexis.lang(text="Bonjour le monde")
# {'language': 'Français', 'code': 'fra', 'script': 'Latin', 'confidence': 0.99}
```

All functions accept a file path as the first argument or a `text=` keyword argument for direct string input.

### JavaScript / WASM

```javascript
import { stats, lang, entropy } from 'lexis';

const result = stats("The quick brown fox jumps over the lazy dog.");
// { tokens: 9, types: 8, sentences: 1, type_token_ratio: 0.8889, ... }

const detected = lang("Bonjour le monde");
// { language: 'Français', code: 'fra', script: 'Latin', confidence: 0.99 }
```

All functions accept text strings directly and return plain JavaScript objects.

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
$ lexis stats prose.txt

  lexis · prose.txt
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
$ lexis readability prose.txt

  lexis · prose.txt
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
$ lexis tokens prose.txt --model all

  lexis · prose.txt
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
cat huge_corpus.txt | lexis stats --stream --chunk-lines 500 --format json
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

| Command | lexis | Python (NLTK) | Speedup |
|---------|---------|---------------|---------|
| Word count | 0.8s | 34s | **42x** |
| Bigram frequency | 1.2s | 89s | **74x** |
| Readability | 0.9s | 41s | **45x** |

> Benchmarks are targets and will be validated with formal benchmarking infrastructure.

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

- Custom vocabulary and dictionary support
- Concordance / KWIC (keyword in context) search
- Collocation analysis (PMI, chi-squared)
- Sentiment lexicon scoring
- Diff mode for comparing two corpora

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

This project is licensed under the [MIT License](LICENSE).

---

## Acknowledgments

- [rayon](https://github.com/rayon-rs/rayon) — Data parallelism
- [clap](https://github.com/clap-rs/clap) — CLI argument parsing
- [comfy-table](https://github.com/nuber-io/comfy-table) — Terminal table rendering
- [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) — Unicode text segmentation
- [whatlang](https://github.com/grstreten/whatlang-rs) — Language detection
- [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) — BPE tokenization for GPT models
- [PyO3](https://github.com/PyO3/pyo3) — Rust bindings for Python
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) — Rust/WebAssembly interop
