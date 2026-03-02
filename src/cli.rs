use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "txtstat", version, about = "The ripgrep of text analysis.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output format
    #[arg(long, global = true, default_value = "table")]
    pub format: OutputFormat,

}

#[derive(Subcommand)]
pub enum Commands {
    /// Full corpus statistics (word count, types, TTR, hapax, entropy)
    Stats {
        /// Input file(s) or directory
        #[arg()]
        input: Option<PathBuf>,

        /// Filter stopwords (path to file, or "english" for built-in list)
        #[arg(long)]
        stopwords: Option<String>,

        /// Process directories recursively
        #[arg(long)]
        recursive: bool,
    },

    /// N-gram frequency analysis
    Ngrams {
        /// Input file(s) or directory
        #[arg()]
        input: Option<PathBuf>,

        /// N-gram size (must be >= 1)
        #[arg(short, default_value = "1")]
        n: usize,

        /// Show top K results
        #[arg(long, default_value = "10")]
        top: usize,

        /// Minimum frequency threshold
        #[arg(long)]
        min_freq: Option<usize>,

        /// Fold case before counting
        #[arg(long)]
        case_insensitive: bool,

        /// Filter stopwords (path to file, or "english" for built-in list)
        #[arg(long)]
        stopwords: Option<String>,

        /// Process directories recursively
        #[arg(long)]
        recursive: bool,
    },

    /// Token counting (whitespace + optional BPE)
    Tokens {
        /// Input file(s) or directory
        #[arg()]
        input: Option<PathBuf>,

        /// BPE model: gpt4, gpt4o, gpt3, all
        #[arg(long)]
        model: Option<String>,

        /// Process directories recursively
        #[arg(long)]
        recursive: bool,
    },

    /// Readability scoring (Flesch-Kincaid, Coleman-Liau, Gunning Fog, SMOG)
    Readability {
        /// Input file(s) or directory
        #[arg()]
        input: Option<PathBuf>,

        /// Process directories recursively
        #[arg(long)]
        recursive: bool,
    },

    /// Shannon entropy analysis
    Entropy {
        /// Input file(s) or directory
        #[arg()]
        input: Option<PathBuf>,

        /// Process directories recursively
        #[arg(long)]
        recursive: bool,
    },

    /// Language detection
    Lang {
        /// Input file(s) or directory
        #[arg()]
        input: Option<PathBuf>,

        /// Process directories recursively
        #[arg(long)]
        recursive: bool,
    },

    /// N-gram language model perplexity
    Perplexity {
        /// Input file(s) or directory
        #[arg()]
        input: Option<PathBuf>,

        /// N-gram order
        #[arg(short = 'n', long, default_value = "3")]
        order: usize,

        /// Smoothing method: none, laplace, backoff
        #[arg(long, default_value = "laplace")]
        smoothing: String,

        /// Smoothing parameter k (for laplace/add-k)
        #[arg(long, default_value = "1.0")]
        k: f64,

        /// Process directories recursively
        #[arg(long)]
        recursive: bool,
    },

    /// Zipf's law analysis (rank-frequency distribution)
    Zipf {
        /// Input file(s) or directory
        #[arg()]
        input: Option<PathBuf>,

        /// Show top K ranked words
        #[arg(long, default_value = "20")]
        top: usize,

        /// Show sparkline plot instead of rank table
        #[arg(long)]
        plot: bool,

        /// Process directories recursively
        #[arg(long)]
        recursive: bool,
    },
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}
