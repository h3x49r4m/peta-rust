//! Parser for grid and simple RST tables

use super::{ParsedTable, TableType, TableDetector, ColumnAlignment};
use crate::core::Result;

/// Parser for inline RST tables (grid and simple)
pub struct TableParser;

impl TableParser {
    /// Parse any inline table type from lines
    pub fn parse(lines: &[&str], start_idx: usize) -> Result<ParsedTable> {
        if start_idx >= lines.len() {
            return Err(crate::core::Error::rst_parse("Table start index out of bounds"));
        }

        // Use the same detection logic as the main parser
        let line = lines[start_idx];
        let next_line_is_simple_sep = start_idx + 1 < lines.len() && 
            TableDetector::is_simple_separator(lines[start_idx + 1].trim());
        
        // For simple tables, we need at least 3 columns to distinguish from headings
        let current_line_has_multiple_columns = line.trim().split_whitespace().count() >= 3;
        
        let table_type = if let Some(tt) = TableDetector::detect(line) {
            Some(tt)
        } else if next_line_is_simple_sep && current_line_has_multiple_columns {
            // Current line might be a simple table header row
            Some(TableType::SimpleTable)
        } else {
            None
        };

        let table_type = match table_type {
            Some(tt) => tt,
            None => return Err(crate::core::Error::rst_parse("Not a valid table")),
        };

        match table_type {
            TableType::GridTable => Self::parse_grid_table(lines, start_idx),
            TableType::SimpleTable => Self::parse_simple_table(lines, start_idx),
            _ => Err(crate::core::Error::rst_parse(
                "Directives should be handled by directive parser",
            )),
        }
    }

    /// Parse grid table: +---+---+
    pub fn parse_grid_table(lines: &[&str], start_idx: usize) -> Result<ParsedTable> {
        let mut table = ParsedTable::new(TableType::GridTable);
        let mut table_lines = Vec::new();
        let mut i = start_idx;

        // Collect table lines until empty line or non-table content
        while i < lines.len() {
            let line = lines[i].trim();

            if line.is_empty() {
                break;
            }

            // Check if line could be part of a grid table
            if !line.starts_with('+') && !line.contains('|') {
                break;
            }

            table_lines.push(line.to_string());
            i += 1;
        }

        if table_lines.is_empty() {
            return Err(crate::core::Error::rst_parse("Empty table"));
        }

        // Parse grid table structure
        Self::parse_grid_structure(&mut table, &table_lines)?;

        Ok(table)
    }

    /// Parse grid table structure from collected lines
    fn parse_grid_structure(table: &mut ParsedTable, lines: &[String]) -> Result<()> {
        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut header_sep_idx: Option<usize> = None;

        for (idx, line) in lines.iter().enumerate() {
            // Skip separator lines
            if TableDetector::is_grid_separator(line) {
                // Check if this is a header separator (contains =)
                if line.contains('=') && idx > 0 && header_sep_idx.is_none() {
                    header_sep_idx = Some(idx);
                }
                continue;
            }

            // Extract cells from row: | cell1 | cell2 |
            if line.contains('|') {
                let cells: Vec<String> = line
                    .split('|')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                if !cells.is_empty() {
                    rows.push(cells);
                }
            }
        }

        if rows.is_empty() {
            return Err(crate::core::Error::rst_parse("No rows found in table"));
        }

        // Determine header and body
        if let Some(sep_idx) = header_sep_idx {
            if sep_idx > 0 && rows.len() > 1 {
                table.headers = rows[0].clone();
                table.rows = rows[1..].to_vec();
                table.has_header = true;
            } else {
                table.rows = rows;
                table.has_header = false;
            }
        } else {
            table.rows = rows;
            table.has_header = false;
        }

        // Set column alignments (default left)
        let col_count = table.column_count();
        table.column_alignments = vec![ColumnAlignment::Left; col_count];

        Ok(())
    }

    /// Parse simple table: Header 1  Header 2
    pub fn parse_simple_table(lines: &[&str], start_idx: usize) -> Result<ParsedTable> {
        let mut table = ParsedTable::new(TableType::SimpleTable);
        let mut table_lines = Vec::new();
        let mut i = start_idx;

        // Collect table lines
        while i < lines.len() {
            let line = lines[i].trim();

            if line.is_empty() {
                break;
            }

            table_lines.push(line.to_string());
            i += 1;
        }

        if table_lines.is_empty() {
            return Err(crate::core::Error::rst_parse("Empty table"));
        }

        // Parse simple table structure
        Self::parse_simple_structure(&mut table, &table_lines)?;

        Ok(table)
    }

    /// Parse simple table structure from collected lines
    fn parse_simple_structure(table: &mut ParsedTable, lines: &[String]) -> Result<()> {
        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut header_sep_idx: Option<usize> = None;
        let mut column_alignments: Vec<ColumnAlignment> = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            if TableDetector::is_simple_separator(line) {
                // Parse column alignments from separator
                if idx == 1 || header_sep_idx.is_none() {
                    column_alignments = Self::parse_alignments(line);
                    header_sep_idx = Some(idx);
                }
                continue;
            }

            // For simple tables, we need to parse by column widths
            // This is a simplified approach - split by whitespace
            let cells: Vec<String> = line
                .split_whitespace()
                .map(|s| s.trim().to_string())
                .collect();

            if !cells.is_empty() {
                rows.push(cells);
            }
        }

        if rows.is_empty() {
            return Err(crate::core::Error::rst_parse("No rows found in table"));
        }

        // Determine header and body
        if let Some(sep_idx) = header_sep_idx {
            if sep_idx > 0 && rows.len() > 1 {
                table.headers = rows[0].clone();
                table.rows = rows[1..].to_vec();
                table.has_header = true;
            } else {
                table.rows = rows;
                table.has_header = false;
            }
        } else {
            table.rows = rows;
            table.has_header = false;
        }

        // Set column alignments
        if column_alignments.is_empty() {
            let col_count = table.column_count();
            table.column_alignments = vec![ColumnAlignment::Left; col_count];
        } else {
            table.column_alignments = column_alignments;
        }

        Ok(())
    }

    /// Parse column alignments from separator line
    fn parse_alignments(line: &str) -> Vec<ColumnAlignment> {
        let mut alignments = Vec::new();
        let parts: Vec<&str> = line.split_whitespace().collect();

        for part in parts {
            if part.starts_with(':') && part.ends_with(':') {
                alignments.push(ColumnAlignment::Center);
            } else if part.starts_with(':') {
                alignments.push(ColumnAlignment::Left);
            } else if part.ends_with(':') {
                alignments.push(ColumnAlignment::Right);
            } else {
                alignments.push(ColumnAlignment::Left);
            }
        }

        alignments
    }

    /// Count lines consumed by a table
    pub fn count_table_lines(lines: &[&str], start_idx: usize) -> Result<usize> {
        let mut count = 0;
        let mut i = start_idx;

        // First, check if we're starting at a simple table header row
        // (using look-ahead logic similar to parse())
        let next_line_is_simple_sep = start_idx + 1 < lines.len() && 
            TableDetector::is_simple_separator(lines[start_idx + 1].trim());
        let current_line_has_multiple_columns = lines[start_idx].trim().split_whitespace().count() >= 3;
        let is_simple_table_header = next_line_is_simple_sep && current_line_has_multiple_columns;
        
        while i < lines.len() {
            let line = lines[i].trim();

            if line.is_empty() {
                break;
            }

            let table_type = TableDetector::detect(line);
            
            // Check if this line is part of the table
            let is_table_line = if table_type.is_some() {
                true
            } else if line.contains('|') || line.contains('+') {
                true
            } else if is_simple_table_header {
                // For simple tables, data rows are also part of the table
                // But stop if we hit a separator that's not a valid simple table separator
                if TableDetector::is_grid_separator(line) || TableDetector::is_simple_separator(line) {
                    // This is a separator line, include it
                    true
                } else if line.chars().all(|c| c == '=' || c == '-' || c.is_whitespace()) {
                    // This looks like a separator but doesn't have multiple parts
                    // It's a heading underline, not a table separator
                    break;
                } else {
                    // This is a data row
                    true
                }
            } else {
                false
            };
            
            if !is_table_line {
                break;
            }

            count += 1;
            i += 1;
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid_table() {
        let lines = vec![
            "+---+---+",
            "| A | B |",
            "+===+===+",
            "| 1 | 2 |",
            "+---+---+",
        ];

        let table = TableParser::parse_grid_table(&lines, 0).unwrap();

        assert!(table.has_header);
        assert_eq!(table.headers, vec!["A", "B"]);
        assert_eq!(table.rows.len(), 1);
        assert_eq!(table.rows[0], vec!["1", "2"]);
    }

    #[test]
    fn test_parse_simple_table() {
        let lines = vec![
            "A  B",
            "== ==",
            "1  2",
        ];

        let table = TableParser::parse_simple_table(&lines, 0).unwrap();

        assert!(table.has_header);
        assert_eq!(table.headers, vec!["A", "B"]);
        assert_eq!(table.rows.len(), 1);
        assert_eq!(table.rows[0], vec!["1", "2"]);
    }

    #[test]
    fn test_parse_alignments() {
        assert_eq!(
            TableParser::parse_alignments(":--  :-:  --:"),
            vec![ColumnAlignment::Left, ColumnAlignment::Center, ColumnAlignment::Right]
        );
    }
}