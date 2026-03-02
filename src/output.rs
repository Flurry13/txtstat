use crate::cli::OutputFormat;
use anyhow::Result;
#[cfg(feature = "comfy-table")]
use comfy_table::{presets::UTF8_FULL_CONDENSED, Table, ContentArrangement, Cell, CellAlignment};
use serde::Serialize;

/// A generic result table that can be rendered in any format
#[derive(Serialize)]
pub struct ResultTable {
    pub title: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl ResultTable {
    pub fn new(title: impl Into<String>, headers: Vec<&str>) -> Self {
        Self {
            title: title.into(),
            headers: headers.into_iter().map(String::from).collect(),
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    pub fn render(&self, format: &OutputFormat) -> Result<String> {
        match format {
            #[cfg(feature = "comfy-table")]
            OutputFormat::Table => Ok(self.render_table()),
            #[cfg(not(feature = "comfy-table"))]
            OutputFormat::Table => self.render_json(),
            OutputFormat::Json => self.render_json(),
            OutputFormat::Csv => self.render_csv(),
        }
    }

    #[cfg(feature = "comfy-table")]
    fn render_table(&self) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL_CONDENSED)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(
                self.headers.iter().map(|h| Cell::new(h)).collect::<Vec<_>>(),
            );

        // Right-align numeric columns (all except first)
        for i in 1..self.headers.len() {
            if let Some(col) = table.column_mut(i) {
                col.set_cell_alignment(CellAlignment::Right);
            }
        }

        for row in &self.rows {
            table.add_row(row.iter().map(|c| Cell::new(c)).collect::<Vec<_>>());
        }

        format!("\n  lexis · {}\n{}\n", self.title, table)
    }

    fn render_json(&self) -> Result<String> {
        let records: Vec<serde_json::Value> = self
            .rows
            .iter()
            .map(|row| {
                let mut map = serde_json::Map::new();
                for (header, value) in self.headers.iter().zip(row.iter()) {
                    map.insert(
                        header.to_lowercase().replace(' ', "_"),
                        serde_json::Value::String(value.clone()),
                    );
                }
                serde_json::Value::Object(map)
            })
            .collect();

        Ok(serde_json::to_string_pretty(&records)?)
    }

    fn render_csv(&self) -> Result<String> {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        wtr.write_record(&self.headers)?;
        for row in &self.rows {
            wtr.write_record(row)?;
        }
        let bytes = wtr.into_inner()?;
        Ok(String::from_utf8(bytes)?)
    }
}
