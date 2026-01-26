//! Table of contents generator

use crate::content::TocEntry;
use crate::core::Result;

/// Table of contents generator
pub struct TocGenerator {
    #[allow(dead_code)]
    max_depth: usize,
}

impl TocGenerator {
    /// Create a new TOC generator
    pub fn new() -> Self {
        Self {
            max_depth: 3,
        }
    }
    
    /// Generate table of contents from HTML content
    pub fn generate(&self, html: &str) -> Result<Vec<TocEntry>> {
        use regex::Regex;
        
        let mut entries = Vec::new();
        
        // Regex to find headings (h1, h2, h3, etc.)
        let heading_regex = Regex::new(r"<h([1-6])[^>]*>(.*?)</h[1-6]>")?;
        
        for cap in heading_regex.captures_iter(html) {
            if let (Some(level), Some(title)) = (cap.get(1), cap.get(2)) {
                let level = level.as_str().parse::<usize>().unwrap_or(1);
                let title = title.as_str().trim().to_string();
                
                // Skip empty headings or headings with excluded text
                if title.is_empty() || 
                   title.contains("Referenced Snippet:") ||
                   title.contains("Table of Contents") ||
                   title.contains("Articles") ||
                   title.contains("Tags") {
                    continue;
                }
                
                // Generate an anchor from the title
                let anchor = title
                    .to_lowercase()
                    .replace(' ', "-")
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '-')
                    .collect::<String>();
                
                entries.push(TocEntry {
                    level,
                    title,
                    anchor,
                    children: Vec::new(),
                });
            }
        }
        
        Ok(entries)
    }
}

impl TocGenerator {
    /// Convert TOC entries to HTML
    pub fn render_html(&self, entries: &[TocEntry]) -> String {
        if entries.is_empty() {
            return String::new();
        }
        
        let mut html = String::from("<ul class=\"toc-list\">\n");
        
        for entry in entries {
            html.push_str(&format!(
                "  <li class=\"toc-item toc-level-{}\">\n",
                entry.level
            ));
            html.push_str(&format!(
                "    <a href=\"#{}\" class=\"toc-link\">{}</a>\n",
                entry.anchor, entry.title
            ));
            html.push_str("  </li>\n");
        }
        
        html.push_str("</ul>\n");
        html
    }
}

impl Default for TocGenerator {
    fn default() -> Self {
        Self::new()
    }
}