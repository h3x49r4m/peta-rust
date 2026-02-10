//! HTML generator for RST tables

use super::{ParsedTable, TableType, ColumnAlignment};
use crate::core::Result;

/// HTML generator for tables
pub struct TableHtmlGenerator;

impl TableHtmlGenerator {
    /// Generate HTML from parsed table
    pub fn generate(table: &ParsedTable) -> Result<String> {
        let mut html = String::new();

        // Table type attribute
        let type_str = match table.table_type {
            TableType::GridTable => "grid",
            TableType::SimpleTable => "simple",
            TableType::CsvDirective => "csv",
            TableType::ListDirective => "list",
        };

        html.push_str("<div class=\"rst-table\" data-type=\"");
        html.push_str(type_str);
        html.push_str("\">\n");

        // Controls (search, copy)
        html.push_str("  <div class=\"table-controls\">\n");
        html.push_str("    <input type=\"search\" class=\"table-search\" placeholder=\"Search table...\">\n");
        html.push_str("    <button class=\"table-copy\" title=\"Copy table\">📋</button>\n");
        html.push_str("  </div>\n");

        // Wrapper for responsive scrolling
        html.push_str("  <div class=\"table-wrapper\">\n");
        html.push_str("    <table>\n");

        // Caption
        if let Some(caption) = &table.caption {
            html.push_str(&format!("      <caption>{}</caption>\n", caption));
        }

        // Header
        if table.has_header {
            html.push_str("      <thead>\n        <tr>\n");
            for (idx, header) in table.headers.iter().enumerate() {
                let align = Self::get_alignment_str(table.column_alignments.get(idx));
                html.push_str(&format!(
                    "          <th style=\"text-align: {}\" data-sortable=\"true\">{}</th>\n",
                    align, header
                ));
            }
            html.push_str("        </tr>\n      </thead>\n");
        }

        // Body
        html.push_str("      <tbody>\n");
        for row in &table.rows {
            html.push_str("        <tr>\n");
            for (idx, cell) in row.iter().enumerate() {
                let align = Self::get_alignment_str(table.column_alignments.get(idx));
                html.push_str(&format!(
                    "          <td style=\"text-align: {}\">{}</td>\n",
                    align, cell
                ));
            }
            html.push_str("        </tr>\n");
        }
        html.push_str("      </tbody>\n");

        html.push_str("    </table>\n");
        html.push_str("  </div>\n");

        // Footer
        let row_count = table.row_count();
        let col_count = table.column_count();
        html.push_str(&format!(
            "  <div class=\"table-footer\">{} rows × {} columns</div>\n",
            row_count, col_count
        ));

        html.push_str("</div>");

        Ok(html)
    }

    /// Get alignment string for CSS
    fn get_alignment_str(alignment: Option<&ColumnAlignment>) -> &'static str {
        match alignment {
            Some(ColumnAlignment::Left) => "left",
            Some(ColumnAlignment::Center) => "center",
            Some(ColumnAlignment::Right) => "right",
            None => "left",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_html() {
        let mut table = ParsedTable::new(TableType::GridTable);
        table.headers = vec!["A".to_string(), "B".to_string()];
        table.rows = vec![vec!["1".to_string(), "2".to_string()]];
        table.has_header = true;
        table.column_alignments = vec![ColumnAlignment::Left, ColumnAlignment::Left];

        let html = TableHtmlGenerator::generate(&table).unwrap();

        assert!(html.contains("class=\"rst-table\""));
        assert!(html.contains("data-type=\"grid\""));
        assert!(html.contains("<th>A</th>"));
        assert!(html.contains("<td>1</td>"));
        assert!(html.contains("table-controls"));
        assert!(html.contains("table-footer"));
    }
}