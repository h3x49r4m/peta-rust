//! Parser for RST table directives (csv-table and list-table)

use super::{ParsedTable, TableType, ColumnAlignment};
use crate::core::Result;
use std::collections::HashMap;

/// Parser for table directives
pub struct DirectiveParser;

impl DirectiveParser {
    /// Parse csv-table directive
    pub fn parse_csv(content: &str, options: &HashMap<String, String>) -> Result<ParsedTable> {
        let mut table = ParsedTable::new(TableType::CsvDirective);

        // Parse options
        if let Some(header) = options.get("header") {
            table.headers = header
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            table.has_header = true;
        }

        if let Some(widths) = options.get("widths") {
            table.column_widths = Some(
                widths
                    .split(',')
                    .map(|s| s.trim().parse::<usize>().unwrap_or(0))
                    .collect(),
            );
        }

        // Parse CSV data
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let cells = Self::parse_csv_line(line)?;
            if !cells.is_empty() {
                table.rows.push(cells);
            }
        }

        // Set default alignments
        let col_count = table.column_count();
        table.column_alignments = vec![ColumnAlignment::Left; col_count];

        Ok(table)
    }

    /// Parse a single CSV line (handles quoted values)
    fn parse_csv_line(line: &str) -> Result<Vec<String>> {
        let mut cells = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '"' if in_quotes => {
                    // Check for escaped quote
                    if chars.peek() == Some(&'"') {
                        current.push('"');
                        chars.next();
                    } else {
                        in_quotes = false;
                    }
                }
                '"' => {
                    in_quotes = true;
                }
                ',' if !in_quotes => {
                    cells.push(current.trim().to_string());
                    current = String::new();
                }
                _ => {
                    current.push(c);
                }
            }
        }

        cells.push(current.trim().to_string());
        Ok(cells)
    }

    /// Parse list-table directive
    pub fn parse_list(content: &str, options: &HashMap<String, String>) -> Result<ParsedTable> {
        let mut table = ParsedTable::new(TableType::ListDirective);

        // Parse options
        let header_rows = options
            .get("header-rows")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);

        if let Some(widths) = options.get("widths") {
            table.column_widths = Some(
                widths
                    .split(',')
                    .map(|s| s.trim().parse::<usize>().unwrap_or(0))
                    .collect(),
            );
        }

        // Parse list items
        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut current_row: Vec<String> = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines
            if trimmed.is_empty() {
                if !current_row.is_empty() {
                    rows.push(current_row.clone());
                    current_row.clear();
                }
                continue;
            }

            // Check for new row (starts with * or -)
            if trimmed.starts_with('*') {
                if !current_row.is_empty() {
                    rows.push(current_row.clone());
                    current_row.clear();
                }

                // Extract the first cell from the row
                // Format: * - Feature
                if let Some(cell) = trimmed.strip_prefix('*').map(|s| s.trim()) {
                    if let Some(cell_content) = cell.strip_prefix('-').map(|s| s.trim()) {
                        current_row.push(cell_content.to_string());
                    }
                }
            } else if trimmed.starts_with('-') {
                // Continuation of current row - new cell
                if let Some(cell_content) = trimmed.strip_prefix('-').map(|s| s.trim()) {
                    current_row.push(cell_content.to_string());
                }
            }
        }

        // Don't forget the last row
        if !current_row.is_empty() {
            rows.push(current_row);
        }

        // Split headers and body
        if header_rows > 0 && rows.len() > header_rows {
            table.headers = rows[0].clone();
            table.rows = rows[header_rows..].to_vec();
            table.has_header = true;
        } else {
            table.rows = rows;
            table.has_header = false;
        }

        // Set default alignments
        let col_count = table.column_count();
        table.column_alignments = vec![ColumnAlignment::Left; col_count];

        Ok(table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_line() {
        let cells = DirectiveParser::parse_csv_line("a,b,c").unwrap();
        assert_eq!(cells, vec!["a", "b", "c"]);

        let cells = DirectiveParser::parse_csv_line("\"a,b\",c").unwrap();
        assert_eq!(cells, vec!["a,b", "c"]);

        let cells = DirectiveParser::parse_csv_line("\"a\"\"b\",c").unwrap();
        assert_eq!(cells, vec!["a\"b", "c"]);
    }

    #[test]
    fn test_parse_csv_table() {
        let content = "1,\"one\"\n2,\"two\"";
        let mut options = HashMap::new();
        options.insert("header".to_string(), "Number,Name".to_string());

        let table = DirectiveParser::parse_csv(content, &options).unwrap();

        assert!(table.has_header);
        assert_eq!(table.headers, vec!["Number", "Name"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec!["1", "one"]);
        assert_eq!(table.rows[1], vec!["2", "two"]);
    }

    #[test]
    fn test_parse_list_table() {
        let content = "* - Header1 - Header2\n* - Data1 - Data2";
        let mut options = HashMap::new();
        options.insert("header-rows".to_string(), "1".to_string());

        let table = DirectiveParser::parse_list(content, &options).unwrap();

        assert!(table.has_header);
        assert_eq!(table.headers, vec!["Header1", "Header2"]);
        assert_eq!(table.rows.len(), 1);
        assert_eq!(table.rows[0], vec!["Data1", "Data2"]);
    }
}
