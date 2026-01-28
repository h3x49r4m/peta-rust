//! Table of contents generator

use crate::content::TocEntry;
use crate::core::Result;

/// Table of contents generator
pub struct TocGenerator {
    #[allow(dead_code)]
    max_depth: usize,
    current_section_level: usize,
}

impl TocGenerator {
    /// Create a new TOC generator
    pub fn new() -> Self {
        Self {
            max_depth: 3,
            current_section_level: 0,
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
    
    /// Generate table of contents with embedded snippet cards
    pub fn generate_with_snippets(&self, html: &str) -> Result<Vec<TocEntry>> {
        use regex::Regex;
        
        let mut entries = Vec::new();
        
        // Regex to find headings (h1, h2, h3, etc.)
        let heading_regex = Regex::new(r#"<h([1-6])[^>]*id="([^"]+)"[^>]*>(.*?)</h[1-6]>"#)?;
        // Regex to find snippet card titles
        let snippet_card_regex = Regex::new(r#"<h4 class="embedded-snippet-title">(.*?)</h4>"#)?;
        
        let mut current_heading_level: Option<usize> = None;
        let mut current_heading_id: Option<String> = None;
        
        // Collect all matches with their positions
        let mut all_matches: Vec<(usize, bool, regex::Captures)> = Vec::new();
        
        // Find all heading matches
        for mat in heading_regex.find_iter(html) {
            if let Some(cap) = heading_regex.captures(mat.as_str()) {
                all_matches.push((mat.start(), true, cap));
            }
        }
        
        // Find all snippet card matches
        for mat in snippet_card_regex.find_iter(html) {
            if let Some(cap) = snippet_card_regex.captures(mat.as_str()) {
                all_matches.push((mat.start(), false, cap));
            }
        }
        
        // Sort matches by position
        all_matches.sort_by_key(|(pos, _, _)| *pos);
        
        // Process matches in order
        for (_, is_heading, cap) in all_matches {
            if is_heading {
                // Process heading
                if let (Some(level), Some(id), Some(title)) = (cap.get(1), cap.get(2), cap.get(3)) {
                    let level = level.as_str().parse::<usize>().unwrap_or(1);
                    let title = title.as_str().trim().to_string();
                    let id_str = id.as_str();
                    
                    // Skip empty headings or headings with excluded text
                    // Also skip scoped snippet card internal headings (level 6 with hyphenated IDs)
                    if !title.is_empty() && 
                       !title.contains("Referenced Snippet:") &&
                       !title.contains("Table of Contents") &&
                       !title.contains("Articles") &&
                       !title.contains("Tags") &&
                       !(level == 6 && id_str.contains('-')) {
                        current_heading_level = Some(level);
                        current_heading_id = Some(id_str.to_string());
                        
                        entries.push(TocEntry {
                            level,
                            title,
                            anchor: id_str.to_string(),
                            children: Vec::new(),
                        });
                    }
                }
            } else {
                // Process snippet card
                if let Some(title) = cap.get(1) {
                    let snippet_title = title.as_str().trim().to_string();
                    // Generate snippet ID that matches the renderer's ID generation
                    let snippet_id = crate::content::rst::embedded_snippet_cards::embedded_snippet_card_renderer::EmbeddedSnippetCardRenderer::generate_snippet_id(&snippet_title);
                    
                    // Add snippet card entry under current heading
                    if let (Some(level), Some(heading_id)) = (current_heading_level, &current_heading_id) {
                        let snippet_entry = TocEntry {
                            level: level + 1,
                            title: format!("Snippet: {}", snippet_title),
                            anchor: snippet_id.clone(),
                            children: Vec::new(),
                        };
                        
                        // Find the entry that matches the current heading ID and add as child
                        // Only add to entries that are actual headings (not snippets)
                        // We can distinguish by checking if the entry doesn't start with "Snippet:"
                        if let Some(entry) = entries.iter_mut().find(|e| e.anchor == *heading_id && !e.title.starts_with("Snippet:")) {
                            entry.children.push(snippet_entry);
                        }
                    } else {
                        // No current heading, add as top-level entry with snippet ID as anchor
                        entries.push(TocEntry {
                            level: 1,
                            title: format!("Snippet: {}", snippet_title),
                            anchor: snippet_id.clone(),
                            children: Vec::new(),
                        });
                    }
                }
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
            
            // Render children if present
            if !entry.children.is_empty() {
                html.push_str(&self.render_children_html(&entry.children, entry.level + 1));
            }
            
            html.push_str("  </li>\n");
        }
        
        html.push_str("</ul>\n");
        html
    }
    
    /// Render children TOC entries recursively
    fn render_children_html(&self, children: &[TocEntry], parent_level: usize) -> String {
        let mut html = String::from("<ul class=\"toc-sublist\">\n");
        
        for entry in children {
            html.push_str(&format!(
                "  <li class=\"toc-item toc-level-{}\">\n",
                entry.level
            ));
            html.push_str(&format!(
                "    <a href=\"#{}\" class=\"toc-link\">{}</a>\n",
                entry.anchor, entry.title
            ));
            
            // Render nested children if present
            if !entry.children.is_empty() {
                html.push_str(&self.render_children_html(&entry.children, entry.level + 1));
            }
            
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