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

    /// Suppress progress output
    #[arg(long, global = true)]
    pub quiet: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Full corpus statistics (word count, types, TTR, hapax, entropy)
    Stats {
        /// Input file(s) or directory
        #[arg()]
        input: Option<PathBuf>,

        /// Process directories recursively
        #[arg(long)]
        recursive: bool,
    },

    /// N-gram frequency analysis
    Ngrams {
        /// Input file(s) or directory
        #[arg()]
        input: Option<PathBuf>,

        /// N-gram size
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

        /// Process directories recursively
        #[arg(long)]
        recursive: bool,
    },

    /// Token counting
    Tokens {
        /// Input file(s) or directory
        #[arg()]
        input: Option<PathBuf>,

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
