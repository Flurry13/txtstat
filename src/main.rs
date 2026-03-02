mod analysis;
mod cli;
mod commands;
mod input;
mod output;
mod streaming;
mod utils;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use cli::Cli;
use rustc_hash::FxHashSet;
use std::path::Path;

fn load_stopwords(arg: &Option<String>) -> Result<Option<FxHashSet<String>>> {
    match arg {
        None => Ok(None),
        Some(val) if val == "english" => Ok(Some(utils::stopwords::default_english())),
        Some(path) => Ok(Some(utils::stopwords::load_stopwords(Path::new(path))?)),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.stream {
        match &cli.command {
            cli::Commands::Stats { .. } => {
                return streaming::stream_stats(&cli.format, cli.chunk_lines);
            }
            cli::Commands::Ngrams { n, top, .. } => {
                return streaming::stream_ngrams(&cli.format, cli.chunk_lines, *n, *top);
            }
            cli::Commands::Entropy { .. } => {
                return streaming::stream_entropy(&cli.format, cli.chunk_lines);
            }
            cli::Commands::Tokens { .. } => return streaming::unsupported("tokens"),
            cli::Commands::Readability { .. } => return streaming::unsupported("readability"),
            cli::Commands::Lang { .. } => return streaming::unsupported("lang"),
            cli::Commands::Perplexity { .. } => return streaming::unsupported("perplexity"),
            cli::Commands::Zipf { .. } => return streaming::unsupported("zipf"),
            cli::Commands::Completions { .. } => return streaming::unsupported("completions"),
        }
    }

    match &cli.command {
        cli::Commands::Stats {
            input,
            stopwords,
            recursive,
        } => {
            let sw = load_stopwords(stopwords)?;
            let inputs = input::resolve_input(input.as_ref(), *recursive)?;
            for (name, text) in &inputs {
                let table = commands::stats::run(text.as_str()?, &name, sw.as_ref())?;
                print!("{}", table.render(&cli.format)?);
            }
        }
        cli::Commands::Ngrams {
            input,
            n,
            top,
            min_freq,
            case_insensitive,
            stopwords,
            recursive,
        } => {
            let sw = load_stopwords(stopwords)?;
            let inputs = input::resolve_input(input.as_ref(), *recursive)?;
            for (name, text) in &inputs {
                let table = commands::ngrams::run(
                    text.as_str()?,
                    &name,
                    *n,
                    *top,
                    *min_freq,
                    *case_insensitive,
                    sw.as_ref(),
                )?;
                print!("{}", table.render(&cli.format)?);
            }
        }
        cli::Commands::Tokens {
            input,
            model,
            recursive,
        } => {
            let inputs = input::resolve_input(input.as_ref(), *recursive)?;
            for (name, text) in &inputs {
                let table =
                    commands::tokens::run(text.as_str()?, &name, model.as_deref())?;
                print!("{}", table.render(&cli.format)?);
            }
        }
        cli::Commands::Readability { input, recursive } => {
            let inputs = input::resolve_input(input.as_ref(), *recursive)?;
            for (name, text) in &inputs {
                let table = commands::readability::run(text.as_str()?, &name)?;
                print!("{}", table.render(&cli.format)?);
            }
        }
        cli::Commands::Entropy { input, recursive } => {
            let inputs = input::resolve_input(input.as_ref(), *recursive)?;
            for (name, text) in &inputs {
                let table = commands::entropy::run(text.as_str()?, &name)?;
                print!("{}", table.render(&cli.format)?);
            }
        }
        cli::Commands::Lang { input, recursive } => {
            let inputs = input::resolve_input(input.as_ref(), *recursive)?;
            for (name, text) in &inputs {
                let table = commands::lang::run(text.as_str()?, &name)?;
                print!("{}", table.render(&cli.format)?);
            }
        }
        cli::Commands::Perplexity {
            input,
            order,
            smoothing,
            k,
            recursive,
        } => {
            let inputs = input::resolve_input(input.as_ref(), *recursive)?;
            for (name, text) in &inputs {
                let table =
                    commands::perplexity::run(text.as_str()?, &name, *order, smoothing, *k)?;
                print!("{}", table.render(&cli.format)?);
            }
        }
        cli::Commands::Zipf {
            input,
            top,
            plot,
            recursive,
        } => {
            let inputs = input::resolve_input(input.as_ref(), *recursive)?;
            for (name, text) in &inputs {
                let table = commands::zipf::run(text.as_str()?, &name, *top, *plot)?;
                print!("{}", table.render(&cli.format)?);
            }
        }
        cli::Commands::Completions { shell } => {
            let mut cmd = cli::Cli::command();
            clap_complete::generate(*shell, &mut cmd, "txtstat", &mut std::io::stdout());
        }
    }

    Ok(())
}
