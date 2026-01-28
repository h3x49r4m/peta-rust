//! Configuration for embedded snippet card rendering

use serde::{Deserialize, Serialize};

/// Configuration for embedded snippet card rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedSnippetCardConfig {
    /// Card border radius
    pub border_radius: String,
    /// Card border color
    pub border_color: String,
    /// Card background color
    pub background_color: String,
    /// Card shadow
    pub shadow: String,
    /// Header background
    pub header_background: String,
    /// Header border
    pub header_border: String,
    /// Title color
    pub title_color: String,
    /// Tag color
    pub tag_color: String,
    /// Content padding
    pub content_padding: String,
    /// Content background
    pub content_background: String,
    /// Footer background
    pub footer_background: String,
    /// Footer border
    pub footer_border: String,
    /// Link color
    pub link_color: String,
    /// Show metadata (title, tags, date)
    pub show_metadata: bool,
    /// Show footer with link
    pub show_footer: bool,
    /// Enable collapsible content
    pub collapsible: bool,
}

impl Default for EmbeddedSnippetCardConfig {
    fn default() -> Self {
        Self {
            border_radius: "0.75rem".to_string(),
            border_color: "#e2e8f0".to_string(),
            background_color: "#ffffff".to_string(),
            shadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)".to_string(),
            header_background: "#f8fafc".to_string(),
            header_border: "#e2e8f0".to_string(),
            title_color: "#1e293b".to_string(),
            tag_color: "#3b82f6".to_string(),
            content_padding: "1.5rem".to_string(),
            content_background: "#ffffff".to_string(),
            footer_background: "#f8fafc".to_string(),
            footer_border: "#e2e8f0".to_string(),
            link_color: "#3b82f6".to_string(),
            show_metadata: true,
            show_footer: true,
            collapsible: false,
        }
    }
}