mod analysis;
mod cli;
mod commands;
mod input;
mod output;
mod utils;

use anyhow::Result;
use clap::Parser;
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
    }

    Ok(())
}
