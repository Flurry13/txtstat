use crate::analysis::{bpe, tokenizer};
use crate::output::ResultTable;
use crate::utils::format::format_num;
use anyhow::Result;

pub fn run(text: &str, source_name: &str, model: Option<&str>) -> Result<ResultTable> {
    let words = tokenizer::words(text);
    let sentences = tokenizer::sentence_count(text);
    let chars = tokenizer::char_count(text);

    let whitespace_tokens = words.len();

    let mut table = ResultTable::new(source_name, vec!["Tokenizer", "Tokens"]);
    table.add_row(vec!["Whitespace".into(), format_num(whitespace_tokens)]);
    table.add_row(vec!["Sentences".into(), format_num(sentences)]);
    table.add_row(vec!["Characters".into(), format_num(chars)]);

    if let Some(model_str) = model {
        if model_str == "all" {
            let results = bpe::count_all_models(text)?;
            for r in results {
                table.add_row(vec![
                    format!("BPE ({})", r.model),
                    format_num(r.token_count),
                ]);
            }
        } else {
            let m = bpe::TokenizerModel::from_str(model_str)?;
            let r = bpe::count_tokens(text, &m)?;
            table.add_row(vec![m.label().to_string(), format_num(r.token_count)]);
        }
    }

    Ok(table)
}
