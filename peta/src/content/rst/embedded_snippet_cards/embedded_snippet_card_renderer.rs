//! Embedded snippet card renderer

use super::config::EmbeddedSnippetCardConfig;
use crate::content::RstContent;
use crate::core::Result;

/// Embedded snippet card renderer
pub struct EmbeddedSnippetCardRenderer {
    /// Configuration
    config: EmbeddedSnippetCardConfig,
    /// Base URL for generating links
    base_url: String,
}

impl EmbeddedSnippetCardRenderer {
    /// Create a new embedded snippet card renderer
    pub fn new() -> Result<Self> {
        Ok(Self {
            config: EmbeddedSnippetCardConfig::default(),
            base_url: String::new(),
        })
    }

    /// Create a new embedded snippet card renderer with base URL
    pub fn with_base_url(base_url: String) -> Result<Self> {
        Ok(Self {
            config: EmbeddedSnippetCardConfig::default(),
            base_url,
        })
    }

    /// Create a new embedded snippet card renderer with custom configuration
    pub fn with_config(config: EmbeddedSnippetCardConfig) -> Result<Self> {
        Ok(Self {
            config,
            base_url: String::new(),
        })
    }

    /// Render an embedded snippet card
    pub fn render(&self, snippet: &RstContent) -> Result<String> {
        let mut html = String::new();
        
        // Generate a unique ID for this snippet card based on snippet ID
        let snippet_id = Self::generate_snippet_id(&snippet.metadata.title);

        // Start card container with ID for TOC linking
        // Add "snippet-" prefix to avoid conflicts with article headings
        html.push_str(&format!(r#"<div class="embedded-snippet-card" id="snippet-{}">"#, snippet_id));
        html.push('\n');

        // Card header with metadata
        if self.config.show_metadata {
            html.push_str("  <div class=\"embedded-snippet-header\">\n");
            html.push_str(&format!("    <h4 class=\"embedded-snippet-title\">{}</h4>\n", snippet.metadata.title));

            // Tags
            if !snippet.metadata.tags.is_empty() {
                html.push_str("    <div class=\"embedded-snippet-tags\">\n");
                for tag in &snippet.metadata.tags {
                    html.push_str(&format!("      <span class=\"embedded-snippet-tag\">{}</span>\n", tag));
                }
                html.push_str("    </div>\n");
            }

            // Date
            if !snippet.metadata.date.is_empty() {
                html.push_str(&format!("    <span class=\"embedded-snippet-date\">{}</span>\n", snippet.metadata.date));
            }

            html.push_str("  </div>\n");
        }

        // Card content with full snippet HTML
        html.push_str("  <div class=\"embedded-snippet-content\">\n");
        
        // Adjust heading hierarchy and scope IDs to avoid conflicts with parent page
        let adjusted_content = self.adjust_heading_hierarchy(&snippet.html, &snippet_id);
        html.push_str(&adjusted_content);
        
        html.push_str("  </div>\n");

        // Card footer with link
        if self.config.show_footer {
            html.push_str("  <div class=\"embedded-snippet-footer\">\n");
            let url = crate::utils::url::build_url(&self.base_url, &snippet.metadata.url);
            html.push_str(&format!(
                r#"    <a href="{}" class="embedded-snippet-link">📄 View full snippet →</a>"#,
                url
            ));
            html.push('\n');
            html.push_str("  </div>\n");
        }

        // End card container
        html.push_str("</div>");

        Ok(html)
    }

// Adjust heading hierarchy to avoid conflicts with parent page
        // Since embedded snippet cards have distinct visual styling, we keep original header levels
        // and only scope the IDs to avoid conflicts
        fn adjust_heading_hierarchy(&self, html: &str, snippet_id: &str) -> String {
            let mut adjusted = html.to_string();

            // Keep original header levels (h1, h2, h3, h4, h5, h6)
            // But scope all IDs to avoid conflicts between snippets and parent page
            // Pattern: id="something" -> id="snippet_id-something"
            adjusted = regex::Regex::new(r#"id="([^"]+)""#)
                .unwrap()
                .replace_all(&adjusted, &format!(r#"id="{}-$1""#, snippet_id))
                .to_string();

            adjusted
        }
    
    /// Generate a snippet ID from title (for ID scoping)
    pub fn generate_snippet_id(title: &str) -> String {
        title.to_lowercase()
            .replace(&[' ', '-', '_', '.', ',', ';', ':', '!', '?', '@', '#', '$', '%', '^', '&', '*', '(', ')', '=', '[', ']', '{', '}', '\\', '|', '<', '>', '/', '"', '\''][..], "-")
            .split(|c: char| !c.is_alphanumeric() && c != '-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }

    /// Render error card for missing snippets
    pub fn render_error(&self, snippet_id: &str) -> Result<String> {
        Ok(format!(
            r#"<div class="embedded-snippet-card error">
  <div class="embedded-snippet-header">
    <h4 class="embedded-snippet-title">⚠️ Snippet Not Found: {}</h4>
  </div>
  <div class="embedded-snippet-content">
    <p>The referenced snippet could not be found. Please check the snippet ID.</p>
  </div>
</div>"#,
            snippet_id
        ))
    }

    /// Set configuration
    pub fn set_config(&mut self, config: EmbeddedSnippetCardConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn config(&self) -> &EmbeddedSnippetCardConfig {
        &self.config
    }
}

impl Default for EmbeddedSnippetCardRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create EmbeddedSnippetCardRenderer")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{ContentMetadata, ContentType};
    use std::collections::HashMap;

    #[test]
    fn test_render_basic_card() {
        let renderer = EmbeddedSnippetCardRenderer::new().unwrap();

        let metadata = ContentMetadata {
            id: "test".to_string(),
            title: "Test Snippet".to_string(),
            content_type: ContentType::Snippet,
            date: "2023-01-01".to_string(),
            tags: vec!["test".to_string(), "example".to_string()],
            author: None,
            excerpt: None,
            url: "snippets/test.html".to_string(),
            extra: HashMap::new(),
        };

        let snippet = RstContent {
            metadata,
            html: "<p>Test content</p>".to_string(),
            toc: Vec::new(),
            toc_html: String::new(),
            frontmatter: HashMap::new(),
            has_math_formulas: false,
            math_formula_count: 0,
        };

        let html = renderer.render(&snippet).unwrap();

        assert!(html.contains("embedded-snippet-card"));
        assert!(html.contains("Test Snippet"));
        assert!(html.contains("test"));
        assert!(html.contains("example"));
        assert!(html.contains("2023-01-01"));
        assert!(html.contains("Test content"));
        assert!(html.contains("View full snippet"));
    }

    #[test]
    fn test_heading_hierarchy_adjustment() {
        let renderer = EmbeddedSnippetCardRenderer::new().unwrap();

        let html_with_headings = "<h1>Level 1</h1><h2>Level 2</h2><h3>Level 3</h3>";
        let adjusted = renderer.adjust_heading_hierarchy(html_with_headings);

        assert!(adjusted.contains("<h3>Level 1</h3>"));
        assert!(adjusted.contains("<h4>Level 2</h4>"));
        assert!(adjusted.contains("<h5>Level 3</h5>"));
        assert!(!adjusted.contains("<h1"));
        assert!(!adjusted.contains("<h2"));
    }

    #[test]
    fn test_render_error_card() {
        let renderer = EmbeddedSnippetCardRenderer::new().unwrap();

        let html = renderer.render_error("missing-snippet").unwrap();

        assert!(html.contains("embedded-snippet-card"));
        assert!(html.contains("error"));
        assert!(html.contains("⚠️ Snippet Not Found"));
        assert!(html.contains("missing-snippet"));
    }

    #[test]
    fn test_card_without_metadata() {
        let mut config = EmbeddedSnippetCardConfig::default();
        config.show_metadata = false;
        config.show_footer = false;

        let renderer = EmbeddedSnippetCardRenderer::with_config(config).unwrap();

        let metadata = ContentMetadata {
            id: "test".to_string(),
            title: "Test".to_string(),
            content_type: ContentType::Snippet,
            date: String::new(),
            tags: Vec::new(),
            author: None,
            excerpt: None,
            url: "snippets/test.html".to_string(),
            extra: HashMap::new(),
        };

        let snippet = RstContent {
            metadata,
            html: "<p>Content</p>".to_string(),
            toc: Vec::new(),
            toc_html: String::new(),
            frontmatter: HashMap::new(),
            has_math_formulas: false,
            math_formula_count: 0,
        };

        let html = renderer.render(&snippet).unwrap();

        assert!(!html.contains("embedded-snippet-header"));
        assert!(!html.contains("embedded-snippet-footer"));
        assert!(html.contains("embedded-snippet-content"));
        assert!(html.contains("Content"));
    }
}
