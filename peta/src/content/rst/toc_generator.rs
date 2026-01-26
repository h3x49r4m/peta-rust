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
                
                // Generate an anchor from the title using the same slugify logic as the parser
                let anchor = self.slugify(&title);
                
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
    
    /// Convert title to URL-friendly slug (matches parser's slugify logic)
    fn slugify(&self, title: &str) -> String {
        let mut result = title.to_lowercase();
        
        // Handle common programming language notations first
        result = result.replace("c++", "cpp");
        result = result.replace("c#", "csharp");
        result = result.replace("f#", "fsharp");
        result = result.replace("c++/cli", "cpp-cli");
        result = result.replace(".net", "dotnet");
        result = result.replace("node.js", "nodejs");
        result = result.replace("react.js", "reactjs");
        result = result.replace("vue.js", "vuejs");
        result = result.replace("angular.js", "angularjs");
        
        // Replace common symbols with words
        result = result.replace("++", "plus");
        result = result.replace("--", "minus");
        result = result.replace("==", "equals");
        result = result.replace("!=", "not-equals");
        result = result.replace("<=", "less-equal");
        result = result.replace(">=", "greater-equal");
        result = result.replace("->", "arrow");
        result = result.replace("=>", "fat-arrow");
        result = result.replace("&&", "and");
        result = result.replace("||", "or");
        
        // Replace spaces and punctuation with dashes
        result = result.replace(&[' ', '-', '_', '.', ',', ';', ':', '!', '?', '@', '#', '$', '%', '^', '&', '*', '(', ')', '=', '[', ']', '{', '}', '\\', '|', '<', '>', '/', '"', '\''][..], "-");
        
        // Remove quotes completely
        result = result.replace(['"', '\''], "");
        
        // Filter to only keep alphanumeric characters and dashes
        result = result.chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();
        
        // Collapse multiple dashes into single dashes
        while result.contains("--") {
            result = result.replace("--", "-");
        }
        
        // Trim leading/trailing dashes
        result.trim_matches('-').to_string()
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