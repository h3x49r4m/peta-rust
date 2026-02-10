//! RST directive handlers

use crate::core::Result;
use std::collections::HashMap;
use std::any::Any;

/// Trait for handling RST directives
pub trait DirectiveHandler: Any {
    /// Handle a directive and return the generated content
    fn handle(&mut self, directive_type: &str, content: &str, options: &HashMap<String, String>) -> Result<String>;

    /// Helper for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any where Self: Sized {
        self
    }
}

/// Code block directive handler
pub struct CodeBlockHandler {
    renderer: crate::content::rst::CodeBlockRenderer,
}

impl CodeBlockHandler {
    pub fn new() -> Self {
        Self {
            renderer: crate::content::rst::CodeBlockRenderer::new()
                .expect("Failed to create CodeBlockRenderer"),
        }
    }
}

impl Default for CodeBlockHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DirectiveHandler for CodeBlockHandler {
    fn handle(&mut self, language: &str, content: &str, _options: &std::collections::HashMap<String, String>) -> Result<String> {
        // Language is passed directly from the directive (e.g., "python", "rust", "typescript")
        // If no language specified, default to "text"
        let language = if language.is_empty() {
            "text"
        } else {
            language
        };

        // Clean up the code content
        let mut code = content.to_string();

        // Remove paragraph tags that might have been added by the paragraph converter
        code = code.replace("<p>", "").replace("</p>", "\n");

        // Use the new CodeBlockRenderer
        self.renderer.render(&code, language, None)
    }
}

/// Snippet card directive handler
pub struct SnippetCardHandler {
    snippet_index: std::collections::HashMap<String, usize>,
}

impl SnippetCardHandler {
    pub fn new() -> Self {
        Self {
            snippet_index: std::collections::HashMap::new(),
        }
    }

    pub fn with_snippet_index(snippet_index: std::collections::HashMap<String, usize>) -> Self {
        Self {
            snippet_index,
        }
    }

    pub fn set_snippet_index(&mut self, snippet_index: std::collections::HashMap<String, usize>) {
        self.snippet_index = snippet_index;
    }
}

impl Default for SnippetCardHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DirectiveHandler for SnippetCardHandler {
    fn handle(&mut self, directive_type: &str, content: &str, _options: &std::collections::HashMap<String, String>) -> Result<String> {
        // For snippet-card, directive_type is actually the snippet ID (language parameter)
        // content is empty for snippet-card
        let snippet_id = if content.trim().is_empty() {
            // Use directive_type as the snippet ID
            directive_type.trim()
        } else {
            // Fallback to content
            content.trim()
        };

        // Generate a simple placeholder that will be replaced later
        // The placeholder includes the snippet ID as a data attribute
        Ok(format!(
            r#"<div class="embedded-snippet-card" data-snippet="{}"></div>"#,
            snippet_id
        ))
    }
}

/// TocTree directive handler
pub struct TocTreeHandler;

impl DirectiveHandler for TocTreeHandler {
    fn handle(&mut self, _directive_type: &str, _content: &str, _options: &std::collections::HashMap<String, String>) -> Result<String> {
        // Return empty HTML since the TOC is already generated separately
        // in the book_toc component sidebar
        Ok(String::new())
    }
}

/// ArticleParts directive handler
pub struct ArticlePartsHandler;

impl DirectiveHandler for ArticlePartsHandler {
    fn handle(&mut self, _directive_type: &str, _content: &str, _options: &std::collections::HashMap<String, String>) -> Result<String> {
        // Return empty HTML since the TOC is already generated separately
        // similar to toctree for books
        Ok(String::new())
    }
}

/// Include directive handler
pub struct IncludeHandler {
    article_dir: Option<std::path::PathBuf>,
}

impl IncludeHandler {
    pub fn new() -> Self {
        Self {
            article_dir: None,
        }
    }

    pub fn with_article_dir(article_dir: std::path::PathBuf) -> Self {
        Self {
            article_dir: Some(article_dir),
        }
    }

    pub fn set_article_dir(&mut self, article_dir: std::path::PathBuf) {
        self.article_dir = Some(article_dir);
    }
}

impl Default for IncludeHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DirectiveHandler for IncludeHandler {
    fn handle(&mut self, _directive_type: &str, file_ref: &str, _options: &std::collections::HashMap<String, String>) -> Result<String> {
        use std::fs;
        use std::path::Path;

        let file_ref = file_ref.trim();
        
        // Get the article directory
        let article_dir = if let Some(ref dir) = self.article_dir {
            dir.clone()
        } else {
            return Ok(String::new()); // No article directory, can't resolve include
        };

        // Resolve the file path
        let file_path = if file_ref.starts_with("/") {
            // Absolute path from articles directory
            article_dir.join(file_ref.trim_start_matches("/"))
        } else {
            // Relative path
            article_dir.join(file_ref)
        };

        // Try both .rst and /index.rst
        let target_path = if file_path.exists() && file_path.extension().map_or(false, |e| e == "rst") {
            file_path.clone()
        } else {
            let index_path = file_path.join("index.rst");
            if index_path.exists() {
                index_path
            } else {
                // Try adding .rst extension
                let rst_path = format!("{}.rst", file_path.to_string_lossy());
                let rst_full_path = Path::new(&rst_path);
                if rst_full_path.exists() {
                    rst_full_path.to_path_buf()
                } else {
                    return Ok(String::new()); // File not found, return empty
                }
            }
        };

        // Read the file content
        let content = fs::read_to_string(&target_path)
            .map_err(|e| crate::core::Error::content(format!("Failed to read include file {}: {}", target_path.display(), e)))?;

        // Remove frontmatter from included content
        let content_without_frontmatter = if content.starts_with("---") {
            let parts: Vec<&str> = content.split("---").collect();
            if parts.len() >= 3 {
                // Skip first two "---" delimiters and their content
                parts[2..].join("\n")
            } else {
                content
            }
        } else {
            content
        };

        Ok(content_without_frontmatter)
    }
}

/// Diagram directive handler
pub struct DiagramHandler {
    renderer: crate::content::rst::diagrams::DiagramRenderer,
}

impl DiagramHandler {
    pub fn new() -> Result<Self> {
        Ok(Self {
            renderer: crate::content::rst::diagrams::DiagramRenderer::new()?,
        })
    }
}

impl DirectiveHandler for DiagramHandler {
    fn handle(&mut self, diagram_type: &str, content: &str, options: &std::collections::HashMap<String, String>) -> Result<String> {
        // Clean up the diagram content
        let content = content
            .replace("<p>", "")
            .replace("</p>", "\n");

        // Extract title from options
        let title = options.get("title").map(|t| t.as_str());

        // Render the diagram
        self.renderer.render(diagram_type, &content, title)
    }
}

/// Music score directive handler
pub struct MusicScoreHandler {
    renderer: crate::content::rst::music_scores::MusicScoreRenderer,
}

impl MusicScoreHandler {
    pub fn new() -> Result<Self> {
        Ok(Self {
            renderer: crate::content::rst::music_scores::MusicScoreRenderer::new()?,
        })
    }
}

impl DirectiveHandler for MusicScoreHandler {
    fn handle(&mut self, score_type: &str, content: &str, options: &std::collections::HashMap<String, String>) -> Result<String> {
        // Clean up the music score content
        let content = content
            .replace("<p>", "")
            .replace("</p>", "\n");

        // Extract title from options
        let title = options.get("title").map(|t| t.as_str());

        // Render the music score
        self.renderer.render(score_type, &content, title)
    }
}

/// Math directive handler for official RST math syntax
pub struct MathDirectiveHandler {
    renderer: crate::content::rst::MathRenderer,
}

impl MathDirectiveHandler {
    pub fn new() -> Self {
        Self {
            renderer: crate::content::rst::MathRenderer::new(),
        }
    }
}

impl Default for MathDirectiveHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DirectiveHandler for MathDirectiveHandler {
    fn handle(&mut self, _directive_type: &str, content: &str, options: &std::collections::HashMap<String, String>) -> Result<String> {
        // Check for :label: option
        let empty_label = String::new();
        let label = options.get("label").unwrap_or(&empty_label);
        
        // Clean the math content:
        // 1. Remove <p> tags
        // 2. Remove common indentation from all lines
        let latex_cleaned = content
            .replace("<p>", "")
            .replace("</p>", "");
        
        // Remove common indentation (find minimum indentation and remove it from all lines)
        let lines: Vec<&str> = latex_cleaned.lines().collect();
        let min_indent = lines.iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.len() - line.trim_start().len())
            .min()
            .unwrap_or(0);
        
        let latex_dedented: String = lines.iter()
            .map(|line| {
                if line.len() >= min_indent && !line.trim().is_empty() {
                    &line[min_indent..]
                } else {
                    *line
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        
        let latex = latex_dedented.trim();
        
        // Render as display math
        let rendered = if let Ok(eq) = self.renderer.render_equation(latex, true) {
            eq
        } else {
            // Fallback if rendering fails
            format!(r#"<div class="math-error">Failed to render: {}</div>"#, latex)
        };
        
        // Add label if provided by inserting data-label attribute into the rendered HTML
        if !label.is_empty() {
            // The rendered HTML is <div class="math-display" data-latex="..."></div>
            // We need to add data-label="..." to it
            let labeled = rendered.replace(r#"<div class="math-display""#, &format!(r#"<div class="math-display" data-label="{}""#, label));
            Ok(labeled)
        } else {
            Ok(rendered)
        }
    }
}

/// Table directive handler for csv-table and list-table directives
pub struct TableDirectiveHandler;

impl TableDirectiveHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TableDirectiveHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DirectiveHandler for TableDirectiveHandler {
    fn handle(&mut self, directive_type: &str, content: &str, options: &std::collections::HashMap<String, String>) -> Result<String> {
        // Clean up the table content
        let content = content
            .replace("<p>", "")
            .replace("</p>", "\n");

        // Extract caption from options
        let caption = options.get("caption").map(|t| t.as_str());

        // Parse based on directive type
        let table = match directive_type {
            "csv-table" => crate::content::rst::tables::DirectiveParser::parse_csv(&content, options)?,
            "list-table" => crate::content::rst::tables::DirectiveParser::parse_list(&content, options)?,
            _ => return Err(crate::core::Error::rst_parse(format!("Unknown table directive: {}", directive_type))),
        };

        // Set caption if provided
        let mut table_with_caption = table;
        if let Some(cap) = caption {
            table_with_caption.caption = Some(cap.to_string());
        }

        // Generate HTML
        crate::content::rst::tables::TableHtmlGenerator::generate(&table_with_caption)
    }
}