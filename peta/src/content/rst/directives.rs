//! RST directive handlers

use crate::core::Result;


/// Trait for RST directive handlers
pub trait DirectiveHandler {
    /// Handle the directive and return HTML
    /// 
    /// # Arguments
    /// * `directive_type` - The type of directive (e.g., "code-block", "diagram")
    /// * `content` - The content of the directive
    /// * `options` - Field list options (key-value pairs, e.g., {"title": "My Title"})
    fn handle(&mut self, directive_type: &str, content: &str, options: &std::collections::HashMap<String, String>) -> Result<String>;
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