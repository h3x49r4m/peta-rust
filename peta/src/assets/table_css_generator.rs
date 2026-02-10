//! CSS generator for RST table styling

use crate::core::Result;
use serde::{Deserialize, Serialize};

/// Configuration for table CSS generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCssConfig {
    /// Border color
    pub border_color: String,
    /// Header background color
    pub header_bg: String,
    /// Stripe color for even rows
    pub stripe_color: String,
    /// Hover color
    pub hover_color: String,
    /// Background color
    pub background_color: String,
    /// Text color
    pub text_color: String,
    /// Border radius
    pub border_radius: String,
}

impl Default for TableCssConfig {
    fn default() -> Self {
        Self {
            border_color: "#e2e8f0".to_string(),
            header_bg: "#f8fafc".to_string(),
            stripe_color: "#f1f5f9".to_string(),
            hover_color: "#e0f2fe".to_string(),
            background_color: "#ffffff".to_string(),
            text_color: "#1e293b".to_string(),
            border_radius: "0.5rem".to_string(),
        }
    }
}

/// CSS generator for tables
pub struct TableCssGenerator {
    config: TableCssConfig,
}

impl TableCssGenerator {
    /// Create a new table CSS generator
    pub fn new() -> Self {
        Self {
            config: TableCssConfig::default(),
        }
    }

    /// Create a CSS generator with custom configuration
    pub fn with_config(config: TableCssConfig) -> Self {
        Self { config }
    }

    /// Generate CSS for tables
    pub fn generate(&self) -> Result<String> {
        let css = format!(
            r#"/* RST Table Styles */

.rst-table {{
  margin: 2rem 0;
  border-radius: {};
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  background: {};
  border: 1px solid {};
}}

/* Table Controls - Collapsible */
.table-controls {{
  display: flex;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: transparent;
  border-bottom: 1px solid transparent;
  opacity: 0;
  transition: opacity 0.2s ease, background 0.2s ease, border-color 0.2s ease;
  max-height: 0;
  overflow: hidden;
}}

.rst-table:hover .table-controls,
.table-controls:focus-within {{
  opacity: 1;
  background: {};
  border-bottom: 1px solid {};
  max-height: 60px;
}}

.table-search {{
  flex: 1;
  padding: 0.25rem 0.5rem;
  border: 1px solid {};
  border-radius: 0.25rem;
  font-size: 0.8rem;
  transition: border-color 0.2s, box-shadow 0.2s;
}}

.table-search:focus {{
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}}

.table-copy {{
  padding: 0.25rem 0.75rem;
  background: #3b82f6;
  color: white;
  border: none;
  border-radius: 0.25rem;
  font-size: 0.8rem;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s, transform 0.1s;
}}

.table-copy:hover {{
  background: #2563eb;
}}

.table-copy:active {{
  transform: translateY(1px);
}}

/* Table Wrapper for Responsive Scrolling */
.table-wrapper {{
  overflow-x: auto;
  -webkit-overflow-scrolling: touch;
}}

/* Table Base Styles */
.rst-table table {{
  width: 100%;
  border-collapse: collapse;
  font-size: 0.9375rem;
  color: {};
}}

/* Table Caption */
.rst-table caption {{
  caption-side: top;
  padding: 1rem;
  font-size: 1rem;
  font-weight: 600;
  color: {};
  text-align: left;
  border-bottom: 1px solid {};
}}

/* Table Header */
.rst-table thead {{
  background: {};
}}

.rst-table th {{
  padding: 0.875rem 1rem;
  font-weight: 600;
  text-align: left;
  border-bottom: 2px solid {};
  white-space: nowrap;
}}

.rst-table th[data-sortable="true"] {{
  cursor: pointer;
  user-select: none;
  position: relative;
}}

.rst-table th[data-sortable="true"]:hover {{
  background: rgba(59, 130, 246, 0.1);
}}

.rst-table th[data-sortable="true"].sort-asc::after {{
  content: " ▲";
  font-size: 0.75rem;
}}

.rst-table th[data-sortable="true"].sort-desc::after {{
  content: " ▼";
  font-size: 0.75rem;
}}

/* Table Body */
.rst-table tbody tr {{
  border-bottom: 1px solid {};
  transition: background 0.15s;
}}

.rst-table tbody tr:hover {{
  background: {};
}}

.rst-table tbody tr:nth-child(even) {{
  background: {};
}}

.rst-table tbody tr:nth-child(even):hover {{
  background: {};
}}

.rst-table td {{
  padding: 0.75rem 1rem;
}}

/* Table Footer - Collapsible */
.table-footer {{
  padding: 0.375rem 1rem;
  font-size: 0.8125rem;
  color: #64748b;
  background: transparent;
  border-top: 1px solid transparent;
  opacity: 0;
  transition: opacity 0.2s ease;
}}

.rst-table:hover .table-footer {{
  opacity: 1;
  background: {};
  border-top: 1px solid {};
}}

/* Empty State */
.rst-table:empty {{
  display: none;
}}

/* Responsive Styles */
@media (max-width: 768px) {{
  .rst-table {{
    margin: 1rem 0;
    border-radius: 0.375rem;
  }}

  .table-controls {{
    flex-direction: row;
    padding: 0.375rem 0.75rem;
  }}

  .table-search {{
    width: auto;
    flex: 1;
  }}

  .table-copy {{
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
  }}

  .rst-table th,
  .rst-table td {{
    padding: 0.625rem 0.75rem;
    font-size: 0.875rem;
  }}

  .table-footer {{
    font-size: 0.75rem;
    padding: 0.25rem 0.75rem;
  }}

  /* Always show controls on touch devices */
  .table-controls,
  .table-footer {{
    opacity: 1;
    max-height: 60px;
    background: {};
    border-bottom: 1px solid {};
  }}

  .table-footer {{
    background: {};
    border-top: 1px solid {};
  }}
}}

@media (max-width: 480px) {{
  .rst-table {{
    margin: 0.75rem 0;
  }}

  .rst-table th,
  .rst-table td {{
    padding: 0.5rem 0.625rem;
    font-size: 0.8125rem;
  }}

  .table-search {{
    font-size: 0.75rem;
  }}

  .table-copy {{
    font-size: 0.7rem;
    padding: 0.2rem 0.4rem;
  }}
}}

/* Dark Mode Support */
@media (prefers-color-scheme: dark) {{
  .rst-table {{
    border-color: #374151;
    background: #1f2937;
  }}

  .table-controls {{
    border-color: transparent;
  }}

  .rst-table:hover .table-controls,
  .table-controls:focus-within {{
    background: #111827;
    border-bottom-color: #374151;
  }}

  .table-search {{
    background: #1f2937;
    border-color: #374151;
    color: #e5e7eb;
  }}

  .table-search:focus {{
    border-color: #60a5fa;
  }}

  .rst-table table {{
    color: #e5e7eb;
  }}

  .rst-table caption {{
    color: #f3f4f6;
    border-color: #374151;
  }}

  .rst-table thead {{
    background: #111827;
  }}

  .rst-table th {{
    border-color: #374151;
    color: #f3f4f6;
  }}

  .rst-table th[data-sortable="true"]:hover {{
    background: rgba(96, 165, 250, 0.1);
  }}

  .rst-table tbody tr {{
    border-color: #374151;
  }}

  .rst-table tbody tr:hover {{
    background: rgba(96, 165, 250, 0.1);
  }}

  .rst-table tbody tr:nth-child(even) {{
    background: #1f2937;
  }}

  .table-footer {{
    border-color: transparent;
  }}

  .rst-table:hover .table-footer {{
    background: #111827;
    border-top-color: #374151;
  }}

  /* Always show controls on mobile in dark mode */
  @media (max-width: 768px) {{
    .table-controls {{
      background: #111827;
      border-bottom-color: #374151;
    }}

    .table-footer {{
      background: #111827;
      border-top-color: #374151;
    }}
  }}
}}

/* Print Styles */
@media print {{
  .rst-table {{
    box-shadow: none;
    border: 1px solid #000;
  }}

  .table-controls,
  .table-footer {{
    display: none;
  }}

  .rst-table tbody tr:hover {{
    background: none;
  }}
}}
"#,
            self.config.border_radius,
            self.config.background_color,
            self.config.border_color,
            self.config.header_bg,
            self.config.border_color,
            self.config.border_color,
            self.config.text_color,
            self.config.text_color,
            self.config.border_color,
            self.config.header_bg,
            self.config.border_color,
            self.config.stripe_color,
            self.config.hover_color,
            self.config.hover_color,
            self.config.header_bg,
            self.config.border_color,
            self.config.header_bg,
            self.config.border_color,
            self.config.header_bg,
            self.config.border_color,
            self.config.header_bg
        );

        Ok(css)
    }

    /// Set configuration
    pub fn set_config(&mut self, config: TableCssConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn config(&self) -> &TableCssConfig {
        &self.config
    }
}

impl Default for TableCssGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_generation() {
        let generator = TableCssGenerator::new();
        let css = generator.generate().unwrap();

        assert!(css.contains(".rst-table"));
        assert!(css.contains(".table-controls"));
        assert!(css.contains(".table-search"));
        assert!(css.contains(".table-copy"));
        assert!(css.contains(".table-wrapper"));
        assert!(css.contains(".table-footer"));
    }

    #[test]
    fn test_custom_config() {
        let config = TableCssConfig {
            border_color: "#ff0000".to_string(),
            header_bg: "#00ff00".to_string(),
            ..Default::default()
        };

        let generator = TableCssGenerator::with_config(config);
        let css = generator.generate().unwrap();

        assert!(css.contains("#ff0000"));
        assert!(css.contains("#00ff00"));
    }
}