use crate::analysis::detect;
use crate::output::ResultTable;
use anyhow::Result;

pub fn run(text: &str, source_name: &str) -> Result<ResultTable> {
    let mut table = ResultTable::new(source_name, vec!["Metric", "Value"]);

    match detect::detect(text) {
        Some(result) => {
            table.add_row(vec!["Language".into(), result.language]);
            table.add_row(vec!["Code".into(), result.code]);
            table.add_row(vec!["Script".into(), result.script]);
            table.add_row(vec!["Confidence".into(), format!("{:.4}", result.confidence)]);
            table.add_row(vec![
                "Reliable".into(),
                if result.is_reliable {
                    "Yes".into()
                } else {
                    "No".into()
                },
            ]);
        }
        None => {
            table.add_row(vec![
                "Language".into(),
                "Unknown (text too short or ambiguous)".into(),
            ]);
        }
    }

    Ok(table)
}
