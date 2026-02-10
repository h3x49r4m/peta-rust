//! RST table parsing and rendering module
//!
//! Supports all standard RST table types:
//! - Grid tables (with + borders)
//! - Simple tables (with = underlines)
//! - CSV table directive
//! - List table directive

pub mod parser;
pub mod directive_parser;
pub mod html_generator;

pub use parser::TableParser;
pub use directive_parser::DirectiveParser;
pub use html_generator::TableHtmlGenerator;

/// RST table types
#[derive(Debug, Clone, PartialEq)]
pub enum TableType {
    GridTable,
    SimpleTable,
    CsvDirective,
    ListDirective,
}

/// Column alignment
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnAlignment {
    Left,
    Center,
    Right,
}

/// Parsed table structure
#[derive(Debug, Clone)]
pub struct ParsedTable {
    pub table_type: TableType,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub caption: Option<String>,
    pub column_widths: Option<Vec<usize>>,
    pub column_alignments: Vec<ColumnAlignment>,
    pub has_header: bool,
}

impl ParsedTable {
    /// Create a new empty parsed table
    pub fn new(table_type: TableType) -> Self {
        Self {
            table_type,
            headers: Vec::new(),
            rows: Vec::new(),
            caption: None,
            column_widths: None,
            column_alignments: Vec::new(),
            has_header: false,
        }
    }

    /// Get the number of columns
    pub fn column_count(&self) -> usize {
        if self.has_header {
            self.headers.len()
        } else if !self.rows.is_empty() {
            self.rows[0].len()
        } else {
            0
        }
    }

    /// Get the number of data rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

/// Table detector for identifying table types
pub struct TableDetector;

impl TableDetector {
    /// Detect table type from a line
    pub fn detect(line: &str) -> Option<TableType> {
        let trimmed = line.trim();

        // Check for directive
        if trimmed.contains(".. csv-table::") {
            return Some(TableType::CsvDirective);
        }
        if trimmed.contains(".. list-table::") {
            return Some(TableType::ListDirective);
        }

        // Check for grid table (starts with + and contains +)
        if trimmed.starts_with('+') && trimmed.contains('+') {
            let plus_count = trimmed.chars().filter(|&c| c == '+').count();
            let dash_count = trimmed.chars().filter(|&c| c == '-').count();
            let equals_count = trimmed.chars().filter(|&c| c == '=').count();

            // Must have at least 2 + signs and some dashes or equals
            if plus_count >= 2 && (dash_count >= 1 || equals_count >= 1) {
                return Some(TableType::GridTable);
            }
        }

        // Check for simple table (all characters are =, -, or whitespace)
        let is_simple_sep = trimmed
            .chars()
            .all(|c| c == '=' || c == '-' || c.is_whitespace());

        // For simple table detection, require at least 2 groups to distinguish from headings
        let has_multiple_groups = trimmed.split_whitespace().count() >= 2;

        if is_simple_sep && has_multiple_groups && (trimmed.chars().filter(|&c| c == '=').count() >= 3
            || trimmed.chars().filter(|&c| c == '-').count() >= 3) {
            return Some(TableType::SimpleTable);
        }

        None
    }

    /// Check if a line is a grid table separator
    pub fn is_grid_separator(line: &str) -> bool {
        let trimmed = line.trim();
        if !trimmed.starts_with('+') || !trimmed.ends_with('+') {
            return false;
        }

        let inner = trimmed.trim_matches('+');
        inner.chars().all(|c| c == '-' || c == '=' || c.is_whitespace())
    }

    /// Check if a line is a simple table separator
    pub fn is_simple_separator(line: &str) -> bool {
        let trimmed = line.trim();
        if !trimmed.chars().all(|c| c == '=' || c == '-' || c.is_whitespace()) {
            return false;
        }
        if !(trimmed.contains('=') || trimmed.chars().filter(|&c| c == '-').count() >= 3) {
            return false;
        }
        
        // Additional check: the separator should have at least 2 groups of = or -
        // This distinguishes table separators from heading underlines
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        parts.len() >= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_grid_table() {
        assert_eq!(
            TableDetector::detect("+---+---+"),
            Some(TableType::GridTable)
        );
        assert_eq!(
            TableDetector::detect("+====+====+"),
            Some(TableType::GridTable)
        );
    }

    #[test]
    fn test_detect_simple_table() {
        assert_eq!(
            TableDetector::detect("====  ===="),
            Some(TableType::SimpleTable)
        );
        assert_eq!(
            TableDetector::detect("------"),
            Some(TableType::SimpleTable)
        );
    }

    #[test]
    fn test_detect_csv_directive() {
        assert_eq!(
            TableDetector::detect(".. csv-table:: Title"),
            Some(TableType::CsvDirective)
        );
    }

    #[test]
    fn test_detect_list_directive() {
        assert_eq!(
            TableDetector::detect(".. list-table:: Title"),
            Some(TableType::ListDirective)
        );
    }

    #[test]
    fn test_is_grid_separator() {
        assert!(TableDetector::is_grid_separator("+---+---+"));
        assert!(TableDetector::is_grid_separator("+====+====+"));
        assert!(!TableDetector::is_grid_separator("| a | b |"));
    }

    #[test]
    fn test_is_simple_separator() {
        assert!(TableDetector::is_simple_separator("====  ===="));
        assert!(TableDetector::is_simple_separator("------"));
        assert!(!TableDetector::is_simple_separator("Hello World"));
    }
}