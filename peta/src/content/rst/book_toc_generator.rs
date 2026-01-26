//! Book table of contents generator

use crate::core::Result;
use std::fs;
use std::path::Path;

/// Book chapter entry
#[derive(Debug, Clone)]
pub struct BookChapter {
    pub title: String,
    pub url: String,
    pub slug: String,
    pub order: usize,
}

/// Book table of contents generator
pub struct BookTocGenerator {
    #[allow(dead_code)]
    max_depth: usize,
}

impl BookTocGenerator {
    /// Create a new book TOC generator
    pub fn new() -> Self {
        Self {
            max_depth: 3,
        }
    }

    /// Generate book TOC from index.rst file
    pub fn generate(&self, book_dir: &Path) -> Result<Vec<BookChapter>> {
        let index_path = book_dir.join("index.rst");
        
        if !index_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&index_path)?;
        
        // Parse toctree directive
        let chapters = self.parse_toctree(&content, book_dir)?;
        
        Ok(chapters)
    }

    /// Parse toctree directive from RST content
    fn parse_toctree(&self, content: &str, book_dir: &Path) -> Result<Vec<BookChapter>> {
        let mut chapters = Vec::new();
        
        // Find toctree directive
        let lines: Vec<&str> = content.lines().collect();
        let mut in_toctree = false;
        let mut indent_level: usize = 0;
        let mut current_order = 0;

        for line in lines {
            let trimmed: &str = line.trim();
            
            // Check for toctree start
            if trimmed.starts_with(".. toctree::") {
                in_toctree = true;
                // Determine the indentation level of the toctree directive
                indent_level = line.len() - line.trim_start().len();
                continue;
            }

            // Exit toctree when we return to the same or lower indentation
            if in_toctree {
                let line_indent = line.len() - line.trim_start().len();
                if trimmed.is_empty() {
                    continue;
                }
                
                if line_indent <= indent_level && !trimmed.starts_with(":") {
                    // We've exited the toctree
                    in_toctree = false;
                    continue;
                }

                // Skip toctree options (lines starting with :)
                if trimmed.starts_with(":") {
                    continue;
                }

                // Parse chapter entry
                if let Some(chapter_slug) = trimmed.split_whitespace().next() {
                    let chapter_path = book_dir.join(format!("{}.rst", chapter_slug));
                    
                    if chapter_path.exists() {
                        // Read chapter file to get title
                        let title = self.extract_chapter_title(&chapter_path)?;
                        
                        chapters.push(BookChapter {
                            title,
                            url: format!("{}.html", chapter_slug),
                            slug: chapter_slug.to_string(),
                            order: current_order,
                        });
                        
                        current_order += 1;
                    }
                }
            }
        }

        Ok(chapters)
    }

    /// Extract title from chapter RST file
    fn extract_chapter_title(&self, chapter_path: &Path) -> Result<String> {
        let content = fs::read_to_string(chapter_path)?;
        
        // Try to extract title from frontmatter first
        if let Some(frontmatter_start) = content.find("---") {
            if let Some(frontmatter_end) = content[frontmatter_start + 3..].find("---") {
                let frontmatter = &content[frontmatter_start + 3..frontmatter_start + 3 + frontmatter_end];
                
                // Parse frontmatter as YAML-like key-value pairs
                for line in frontmatter.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("title:") {
                        let title = trimmed["title:".len()..].trim();
                        // Remove quotes if present
                        let title = title.trim_matches('"').trim_matches('\'');
                        return Ok(title.to_string());
                    }
                }
            }
        }

        // Fallback: Extract title from first heading
        if let Some(heading_start) = content.find("\n=") {
            let before_equals = &content[..heading_start];
            if let Some(title_end) = before_equals.rfind('\n') {
                let title = before_equals[title_end + 1..].trim();
                if !title.is_empty() {
                    return Ok(title.to_string());
                }
            }
        }

        // Last resort: Use filename as title
        if let Some(stem) = chapter_path.file_stem() {
            let title = stem.to_string_lossy()
                .replace('-', " ")
                .replace('_', " ");
            return Ok(title.to_string());
        }

        Ok("Untitled".to_string())
    }

    /// Convert book chapters to HTML
    pub fn render_html(&self, chapters: &[BookChapter]) -> String {
        if chapters.is_empty() {
            return String::new();
        }

        let mut html = String::from("<div class=\"toc-tree\">\n");

        for chapter in chapters {
            html.push_str(&format!(
                "  <div class=\"toc-item\"><a href=\"{}\">{}</a></div>\n",
                chapter.url, chapter.title
            ));
        }

        html.push_str("</div>\n");
        html
    }
}

impl Default for BookTocGenerator {
    fn default() -> Self {
        Self::new()
    }
}