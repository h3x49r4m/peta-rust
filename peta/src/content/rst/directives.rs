//! RST directive handlers

use crate::core::Result;


/// Trait for RST directive handlers
pub trait DirectiveHandler {
    /// Handle the directive and return HTML
    fn handle(&mut self, directive_type: &str, content: &str) -> Result<String>;
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
    fn handle(&mut self, language: &str, content: &str) -> Result<String> {
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
    fn handle(&mut self, directive_type: &str, content: &str) -> Result<String> {
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
    fn handle(&mut self, _directive_type: &str, _content: &str) -> Result<String> {
        // Return empty HTML since the TOC is already generated separately
        // in the book_toc component sidebar
        Ok(String::new())
    }
}