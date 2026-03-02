mod analysis;
mod cli;
mod input;
mod output;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        cli::Commands::Stats { .. } => {
            eprintln!("stats: not yet implemented");
        }
        cli::Commands::Ngrams { .. } => {
            eprintln!("ngrams: not yet implemented");
        }
        cli::Commands::Tokens { .. } => {
            eprintln!("tokens: not yet implemented");
        }
    }
}
