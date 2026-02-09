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

        // Regex to find headings (h1, h2, h3, etc.) with id attributes
        let heading_regex = Regex::new(r#"<h([1-6])[^>]*id="([^"]+)"[^>]*>(.*?)</h[1-6]>"#)?;
        // Regex to find snippet card titles
        let snippet_card_regex = Regex::new(r#"<h4 class="embedded-snippet-title"[^>]*>(.*?)</h4>"#)?;

        // Collect all snippet card IDs to filter their internal headers
        let mut snippet_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
        for mat in snippet_card_regex.captures_iter(html) {
            if let Some(title) = mat.get(1) {
                let snippet_id = crate::content::rst::embedded_snippet_cards::embedded_snippet_card_renderer::EmbeddedSnippetCardRenderer::generate_snippet_id(title.as_str().trim());
                snippet_ids.insert(snippet_id);
            }
        }

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

        // Build result structure
        let mut result: Vec<TocEntry> = Vec::new();
        let mut header_stack: Vec<(usize, usize)> = Vec::new(); // (level, index in result)
        let mut last_header_idx: Option<usize> = None;

        for (_pos, is_heading, cap) in all_matches {
            if is_heading {
                // Process heading
                if let (Some(level), Some(id), Some(title)) = (cap.get(1), cap.get(2), cap.get(3)) {
                    let level = level.as_str().parse::<usize>().unwrap_or(1);
                    let title = title.as_str().trim().to_string();
                    let id_str = id.as_str();

                    // Skip empty headings or headings with excluded text
                    // Also skip snippet card internal headings (scoped with snippet_id-heading_id pattern)
                    if !title.is_empty() &&
                       !title.contains("Referenced Snippet:") &&
                       !title.contains("Table of Contents") &&
                       !title.contains("Articles") &&
                       !title.contains("Tags") &&
                       !snippet_ids.iter().any(|sid| id_str.starts_with(&format!("{}-", sid))) {
                        
                        let entry = TocEntry {
                            level,
                            title,
                            anchor: id_str.to_string(),
                            children: Vec::new(),
                        };

                        // Build hierarchy
                        while let Some(&(stack_level, _)) = header_stack.last() {
                            if stack_level < level {
                                break;
                            }
                            header_stack.pop();
                        }

                        if let Some((_, parent_idx)) = header_stack.last() {
                            result[*parent_idx].children.push(entry);
                        } else {
                            result.push(entry);
                            header_stack.push((level, result.len() - 1));
                        }

                        // Update last header index
                        last_header_idx = Some(result.len() - 1);
                    }
                }
            } else {
                // Process snippet card
                if let Some(title) = cap.get(1) {
                    let snippet_title = title.as_str().trim().to_string();
                    // Generate snippet ID that matches the renderer's ID generation
                    let snippet_id = crate::content::rst::embedded_snippet_cards::embedded_snippet_card_renderer::EmbeddedSnippetCardRenderer::generate_snippet_id(&snippet_title);

                    let snippet_entry = TocEntry {
                        level: 1,
                        title: format!("Snippet: {}", snippet_title),
                        anchor: format!("snippet-{}", snippet_id),
                        children: Vec::new(),
                    };

                    // Nest snippet under the most recent header if available
                    if let Some(header_idx) = last_header_idx {
                        // Find the header in the result structure
                        if let Some(header) = result.get_mut(header_idx) {
                            header.children.push(snippet_entry);
                        } else {
                            // Header not found, add as top-level
                            result.push(snippet_entry);
                        }
                    } else {
                        // No header yet, add as top-level
                        result.push(snippet_entry);
                    }
                }
            }
        }

        Ok(result)
    }

    /// Generate table of contents with embedded snippet cards
    pub fn generate_with_snippets(&self, html: &str) -> Result<Vec<TocEntry>> {
        use regex::Regex;

        let mut entries = Vec::new();

        // Regex to find headings (h1, h2, h3, etc.) with id attributes
        let heading_regex = Regex::new(r#"<h([1-6])[^>]*id="([^"]+)"[^>]*>(.*?)</h[1-6]>"#)?;
        // Regex to find snippet card titles
        let snippet_card_regex = Regex::new(r#"<h4 class="embedded-snippet-title"[^>]*>(.*?)</h4>"#)?;

        // Collect all snippet card IDs to filter their internal headers
        let mut snippet_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
        for mat in snippet_card_regex.captures_iter(html) {
            if let Some(title) = mat.get(1) {
                let snippet_id = crate::content::rst::embedded_snippet_cards::embedded_snippet_card_renderer::EmbeddedSnippetCardRenderer::generate_snippet_id(title.as_str().trim());
                snippet_ids.insert(snippet_id);
            }
        }

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
                    // Also skip snippet card internal headings (scoped with snippet_id-heading_id pattern)
                    if !title.is_empty() &&
                       !title.contains("Referenced Snippet:") &&
                       !title.contains("Table of Contents") &&
                       !title.contains("Articles") &&
                       !title.contains("Tags") &&
                       !snippet_ids.iter().any(|sid| id_str.starts_with(&format!("{}-", sid))) {
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
                if let Some(title) = cap.get(2) {
                    let snippet_title = title.as_str().trim().to_string();
                    // Generate snippet ID that matches the renderer's ID generation
                    let snippet_id = crate::content::rst::embedded_snippet_cards::embedded_snippet_card_renderer::EmbeddedSnippetCardRenderer::generate_snippet_id(&snippet_title);

                    // Add snippet card entry as top-level entry
                    entries.push(TocEntry {
                        level: 1,
                        title: format!("Snippet: {}", snippet_title),
                        anchor: format!("snippet-{}", snippet_id),
                        children: Vec::new(),
                    });
                }
            }
        }

        // Build hierarchy from flat list
        Ok(self.build_hierarchy(entries))
    }

    /// Build hierarchical structure from headers and snippets based on document position
    /// Build hierarchical structure from flat header list
    fn build_hierarchy(&self, entries: Vec<TocEntry>) -> Vec<TocEntry> {
        if entries.is_empty() {
            return Vec::new();
        }

        let mut result: Vec<TocEntry> = Vec::new();
        let mut stack: Vec<(usize, usize)> = Vec::new(); // (level, index in result)

        for entry in entries {
            let level = entry.level;
            let is_snippet = entry.title.starts_with("Snippet:");

            if is_snippet {
                // Snippet cards should be nested under the most recent non-snippet header
                // Find the most recent non-snippet entry in the stack
                let mut parent_idx: Option<usize> = None;
                for &(_, entry_idx) in stack.iter().rev() {
                    if result[entry_idx].title.starts_with("Snippet:") {
                        continue;
                    }
                    parent_idx = Some(entry_idx);
                    break;
                }

                if let Some(idx) = parent_idx {
                    // Add snippet as child of the most recent non-snippet header
                    // Set snippet level to parent level + 1 for proper CSS styling
                    let mut snippet_entry = entry;
                    snippet_entry.level = result[idx].level + 1;
                    result[idx].children.push(snippet_entry);
                } else {
                    // No header yet, add as top-level with level 1
                    let mut snippet_entry = entry;
                    snippet_entry.level = 1;
                    result.push(snippet_entry);
                }
            } else {
                // Regular header - use normal level-based hierarchy
                while let Some(&(stack_level, _)) = stack.last() {
                    if stack_level < level {
                        break;
                    }
                    stack.pop();
                }

                if let Some((_, parent_idx)) = stack.last() {
                    // Add as child of parent
                    result[*parent_idx].children.push(entry);
                } else {
                    // Add as top-level header
                    result.push(entry);
                    stack.push((level, result.len() - 1));
                }
            }
        }

        result
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
    fn render_children_html(&self, children: &[TocEntry], _parent_level: usize) -> String {
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