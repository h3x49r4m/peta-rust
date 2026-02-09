//! Article TOC generator that parses RST content directly
//! Extracts hierarchical structure from RST with snippet-to-header relationships

use crate::content::TocEntry;
use crate::core::Result;

/// Article TOC generator for RST-based content
pub struct ArticleTocGenerator {
    #[allow(dead_code)]
    max_depth: usize,
}

impl ArticleTocGenerator {
    /// Create a new article TOC generator
    pub fn new() -> Self {
        Self {
            max_depth: 3,
        }
    }

    /// Generate table of contents from RST content
    pub fn generate_from_rst(&self, rst_content: &str) -> Result<Vec<TocEntry>> {
        let mut result: Vec<TocEntry> = Vec::new();
        let mut header_stack: Vec<(usize, usize)> = Vec::new(); // (level, index in result)
        let mut pending_snippet: Option<String> = None;
        let mut line_buffer: Vec<String> = Vec::new();

        for line in rst_content.lines() {
            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }

            // Check for snippet directive
            if line.trim().starts_with(".. snippet-card::") {
                // Clear line_buffer before processing snippet directive
                line_buffer.clear();
                
                let snippet_id = line.trim().split("::").nth(1).unwrap_or("").trim();
                pending_snippet = Some(snippet_id.to_string());
                continue;
            } else if self.is_header_underline(line) {
                // This is handled in the next block
            } else if !line.trim().is_empty() {
                line_buffer.push(line.to_string());
            }

            // Check for header (underline markers)
            if self.is_header_underline(line) {
                if let Some(header_line) = line_buffer.last() {
                    let header_text = header_line.trim();
                    
                    // Check if header line is valid (not empty, not an underline)
                    if !header_text.is_empty() && !self.is_header_underline(header_line) {
                        // Check if underline is at least as long as the title
                        if line.trim().len() >= header_text.len() {
                            let level = self.get_header_level(line);
                            let title = header_text.to_string();
                            let anchor = self.slugify(&title);

                            // Create header entry
                            let header_entry = TocEntry {
                                level,
                                title: title.clone(),
                                anchor,
                                children: Vec::new(),
                            };

                            // Add header to hierarchy
                            while let Some(&(stack_level, _)) = header_stack.last() {
                                if stack_level < level {
                                    break;
                                }
                                header_stack.pop();
                            }

                            if let Some((_, parent_idx)) = header_stack.last() {
                                result[*parent_idx].children.push(header_entry);
                            } else {
                                result.push(header_entry);
                                header_stack.push((level, result.len() - 1));
                            }

                            line_buffer.clear();
                            continue;
                        }
                    }
                }
            }

            // Check for pending snippet (snippet appears after the most recent header)
            // Check on blank lines OR non-empty, non-directive, non-underline lines
            if let Some(snippet_id) = &pending_snippet {
                if line.trim().is_empty() || 
                   (!line.trim().starts_with(".. snippet-card::") && !self.is_header_underline(line)) {
                    
                    // Add snippet as child of the most recent header
                    if let Some((_, parent_idx)) = header_stack.last() {
                        let snippet_title = self.get_snippet_title(snippet_id);
                        let parent_level = result[*parent_idx].level;
                        
                        let snippet_entry = TocEntry {
                            level: parent_level + 1,
                            title: format!("Snippet: {}", snippet_title),
                            anchor: format!("snippet-{}", snippet_id),
                            children: Vec::new(),
                        };
                        
                        result[*parent_idx].children.push(snippet_entry);
                    } else {
                        // No header yet, add snippet as top-level
                        let snippet_title = self.get_snippet_title(snippet_id);
                        
                        let snippet_entry = TocEntry {
                            level: 1,
                            title: format!("Snippet: {}", snippet_title),
                            anchor: format!("snippet-{}", snippet_id),
                            children: Vec::new(),
                        };
                        
                        result.push(snippet_entry);
                    }
                    
                    pending_snippet = None;
                }
            }
            // Add line to buffer (might be a header title)
            line_buffer.push(line.to_string());
        }

        // Handle any remaining pending snippet (shouldn't happen normally)
        if let Some(snippet_id) = pending_snippet {
            let snippet_title = self.get_snippet_title(&snippet_id);
            result.push(TocEntry {
                level: 1,
                title: format!("Snippet: {}", snippet_title),
                anchor: format!("snippet-{}", snippet_id),
                children: Vec::new(),
            });
        }

        Ok(result)
    }

    /// Check if a line is a header underline marker
    fn is_header_underline(&self, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.len() < 3 {
            return false;
        }

        let chars: Vec<char> = trimmed.chars().collect();
        let first_char = chars[0];

        // Valid underline characters: =, -, ~, ", ', `, ^, *, +, #
        let valid_chars = ['=', '-', '~', '"', '\'', '`', '^', '*', '+', '#'];
        
        if !valid_chars.contains(&first_char) {
            return false;
        }

        // All characters must be the same
        chars.iter().all(|c| *c == first_char)
    }

    /// Get header level from underline marker
    fn get_header_level(&self, underline: &str) -> usize {
        match underline.trim().chars().next() {
            Some('=') => 1,
            Some('-') => 2,
            Some('~') => 3,
            Some('"') | Some('\'') | Some('`') => 4,
            Some('^') | Some('*') => 5,
            Some('+') | Some('#') => 6,
            _ => 1,
        }
    }

    /// Convert title to slug for anchor
    fn slugify(&self, title: &str) -> String {
        title.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-")
    }

    /// Get snippet title from snippet ID
    /// This is a placeholder - in real implementation, you'd read the snippet file
    fn get_snippet_title(&self, snippet_id: &str) -> String {
        // Convert kebab-case to title case
        snippet_id
            .split('-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl Default for ArticleTocGenerator {
    fn default() -> Self {
        Self::new()
    }
}