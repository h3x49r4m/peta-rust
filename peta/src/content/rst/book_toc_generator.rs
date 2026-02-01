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
    pub headers: Vec<ChapterHeader>,
}

/// Chapter header entry
#[derive(Debug, Clone)]
pub struct ChapterHeader {
    pub level: usize,
    pub title: String,
    pub anchor: String,
    pub children: Vec<ChapterHeader>,
}

/// Book table of contents generator
pub struct BookTocGenerator {
    #[allow(dead_code)]
    max_depth: usize,
    base_url: String,
}

impl BookTocGenerator {
    /// Create a new book TOC generator
    pub fn new() -> Self {
        Self {
            max_depth: 3,
            base_url: String::new(),
        }
    }

    /// Create a new book TOC generator with base URL
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            max_depth: 3,
            base_url,
        }
    }

    /// Get full URL with base_url prefix
    fn get_full_url(&self, path: &str) -> String {
        crate::utils::url::build_url(&self.base_url, path)
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
        
        // Extract book slug from directory name
        let book_slug = book_dir.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("book");
        
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
                        // Read chapter file to get title and headers
                        let title = self.extract_chapter_title(&chapter_path)?;
                        let headers = self.extract_chapter_headers(&chapter_path)?;
                        
                        chapters.push(BookChapter {
                            title,
                            url: format!("books/{}/{}.html", book_slug, chapter_slug),
                            slug: chapter_slug.to_string(),
                            order: current_order,
                            headers,
                        });
                        
                        current_order += 1;
                    }
                }
            }
        }

        Ok(chapters)
    }

    /// Extract headers from chapter RST file
        fn extract_chapter_headers(&self, chapter_path: &Path) -> Result<Vec<ChapterHeader>> {
            let content = fs::read_to_string(chapter_path)?;
            let mut headers = Vec::new();
            let lines: Vec<&str> = content.lines().collect();
            
            let mut i = 0;
            let mut in_frontmatter = false;
            
            while i < lines.len() {
                let line = lines[i].trim();
                
                // Handle frontmatter delimiters
                if line == "---" {
                    if !in_frontmatter {
                        // Start of frontmatter
                        in_frontmatter = true;
                    } else {
                        // End of frontmatter
                        in_frontmatter = false;
                    }
                    i += 1;
                    continue;
                }
                
                // Skip all content inside frontmatter
                if in_frontmatter {
                    i += 1;
                    continue;
                }
                
                // Skip empty lines
                if line.is_empty() {
                    i += 1;
                    continue;
                }
                
                // Check if this is a heading (underlined with =, -, ~, or *)
                if i + 1 < lines.len() {
                    let underline = lines[i + 1].trim();
                    
                    // Determine the heading level based on the underline character
                    if !underline.is_empty() && 
                       (underline.starts_with("==") || 
                        underline.starts_with("--") || 
                        underline.starts_with("~~") || 
                        underline.starts_with("**")) &&
                       underline.chars().all(|c| c == '=' || c == '-' || c == '~' || c == '*') {
                        
                        // Calculate heading level based on character
                        let level = match underline.chars().next() {
                            Some('=') => 1,
                            Some('-') => 2,
                            Some('~') => 3,
                            Some('*') => 4,
                            _ => 2,
                        };
                        
                        // Skip the chapter title (level 1) to avoid duplication
                        if level > 1 {
                            let title = line.trim().to_string();
                            let anchor = self.slugify(&title);
                            
                            headers.push(ChapterHeader {
                                level,
                                title,
                                anchor,
                                children: Vec::new(),
                            });
                        }
                        
                        i += 2; // Skip both the heading and its underline
                        continue;
                    }
                }
                
                i += 1;
            }
    
            // Build hierarchy from flat list
            Ok(self.build_header_hierarchy(headers))
        }
        
        /// Build hierarchical structure from flat header list

            fn build_header_hierarchy(&self, headers: Vec<ChapterHeader>) -> Vec<ChapterHeader> {

                let mut result: Vec<ChapterHeader> = Vec::new();

                let mut stack: Vec<(usize, usize)> = Vec::new(); // (level, index in result)



                for header in headers {

                    let level = header.level;



                    // Find the correct parent based on level

                    while let Some(&(stack_level, _)) = stack.last() {

                        if stack_level < level {

                            break;

                        }

                        stack.pop();

                    }



                    if let Some((_, parent_idx)) = stack.last() {

                        // Add as child of parent

                        let parent = &mut result[*parent_idx];

                        // Use only the child's anchor, not concatenated with parent

                        parent.children.push(ChapterHeader {

                            level: header.level,

                            title: header.title,

                            anchor: header.anchor,

                            children: Vec::new(),

                        });

                    } else {

                        // Add as top-level header

                        result.push(header);

                        stack.push((level, result.len() - 1));

                    }

                }



                result

            }    /// Extract title from chapter RST file
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

        // Fallback: Extract title from first heading (underlined with =)
        // RST headings are followed by = signs on the next line
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("=") && !trimmed.starts_with("-") {
                // Check if the next line starts with = or - (heading underline)
                let lines: Vec<&str> = content.lines().collect();
                if let Some(pos) = lines.iter().position(|l| l.trim() == trimmed) {
                    if pos + 1 < lines.len() {
                        let next_line = lines[pos + 1].trim();
                        if next_line.starts_with("=") || next_line.starts_with("-") {
                            // This is a heading
                            return Ok(trimmed.to_string());
                        }
                    }
                }
                break;
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

    /// Convert title to URL-friendly slug
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

    /// Convert book chapters to HTML

        pub fn render_html(&self, chapters: &[BookChapter]) -> String {

            if chapters.is_empty() {

                return String::new();

            }

    

            let mut html = String::from("<div class=\"toc-tree\">\n");

    

            for chapter in chapters {

                html.push_str(&format!(

                    "  <div class=\"toc-item\" data-chapter=\"{}\">\n",

                    chapter.slug

                ));

                

                // Chapter link with toggle button

                html.push_str(&format!(

                    "    <div class=\"toc-item-header\">\n",

                ));

                html.push_str(&format!(

                    "      <button class=\"toc-toggle-btn\" data-target=\"{}-headers\" aria-expanded=\"false\" aria-label=\"Toggle {} headers\">\n",

                    chapter.slug, chapter.title

                ));

                html.push_str("        <svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke=\"currentColor\">\n");

                html.push_str("          <path stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M19 9l-7 7-7-7\" />\n");

                html.push_str("        </svg>\n");

                html.push_str("      </button>\n");

                html.push_str(&format!(

                                    "      <a href=\"{}\" class=\"toc-chapter-link\">{}</a>\n",

                

                                    self.get_full_url(&chapter.url), chapter.title

                

                                ));

                html.push_str("    </div>\n");

                

                // Chapter headers (nested)

                if !chapter.headers.is_empty() {

                    html.push_str(&format!(

                        "    <div class=\"toc-headers\" id=\"{}-headers\">\n",

                        chapter.slug

                    ));

                    html.push_str("      <ul class=\"toc-header-list\">\n");

                    

                    for header in &chapter.headers {

                        html.push_str(&self.render_header_html(header, &chapter.url));

                    }

                    

                    html.push_str("      </ul>\n");

                    html.push_str("    </div>\n");

                }

                

                html.push_str("  </div>\n");

            }

    

            html.push_str("</div>\n");

    

            html

        }

        

        /// Render header HTML recursively

        fn render_header_html(&self, header: &ChapterHeader, chapter_url: &str) -> String {

            let has_children = !header.children.is_empty();

            let header_id = format!("{}-{}", chapter_url.trim_end_matches(".html"), header.anchor);

            

            let mut html = String::new();

            html.push_str(&format!(

                "        <li class=\"toc-header-item toc-level-{}\">\n",

                header.level

            ));

            

            if has_children {

                html.push_str(&format!(

                    "          <div class=\"toc-header-item-header\">\n"

                ));

                html.push_str(&format!(

                    "            <button class=\"toc-toggle-btn\" data-target=\"{}-subheaders\" aria-expanded=\"false\" aria-label=\"Toggle {} subheaders\">\n",

                    header_id, header.title

                ));

                html.push_str("              <svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke=\"currentColor\">\n");

                html.push_str("                <path stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M19 9l-7 7-7-7\" />\n");

                html.push_str("              </svg>\n");

                html.push_str("            </button>\n");

                html.push_str(&format!(

                                    "            <a href=\"{}#{}\" class=\"toc-header-link\">{}</a>\n",

                

                                    self.get_full_url(chapter_url), header.anchor, header.title

                

                                ));

                html.push_str("          </div>\n");

                

                // Render children

                html.push_str(&format!(

                    "          <div class=\"toc-headers\" id=\"{}-subheaders\">\n",

                    header_id

                ));

                html.push_str("            <ul class=\"toc-header-list\">\n");

                for child in &header.children {

                    html.push_str(&self.render_header_html(child, chapter_url));

                }

                html.push_str("            </ul>\n");

                html.push_str("          </div>\n");

            } else {

                html.push_str(&format!(

                                    "          <a href=\"{}#{}\" class=\"toc-header-link\">{}</a>\n",

                

                                    self.get_full_url(chapter_url), header.anchor, header.title

                

                                ));

            }

            

            html.push_str("        </li>\n");

            html

        }
}

impl Default for BookTocGenerator {
    fn default() -> Self {
        Self::new()
    }
}