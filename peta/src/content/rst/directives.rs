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
pub struct SnippetCardHandler;

impl DirectiveHandler for SnippetCardHandler {
    fn handle(&mut self, _directive_type: &str, content: &str) -> Result<String> {
        let snippet_name = content.trim();
        let snippet_link = format!("/snippets/{}.html", snippet_name.replace('_', "-").to_lowercase());
        
        Ok(format!(
            r#"<div class="snippet-card" data-snippet="{}">
    <div class="snippet-header">
        <h4>Referenced Snippet: {}</h4>
    </div>
    <div class="snippet-content">
        <p>See <a href="{}">{}</a> for the complete snippet.</p>
    </div>
</div>"#,
            snippet_name, snippet_name, snippet_link, snippet_name
        ))
    }
}

/// TocTree directive handler
pub struct TocTreeHandler;

impl DirectiveHandler for TocTreeHandler {
    fn handle(&mut self, _directive_type: &str, content: &str) -> Result<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut _maxdepth = 2;
        let mut caption = String::new();
        let mut entries = Vec::new();
        
        for line in lines {
            let trimmed = line.trim_start();
            if trimmed.starts_with(":maxdepth:") {
                _maxdepth = trimmed.split(':').nth(1).unwrap_or("2").trim().parse().unwrap_or(2);
            } else if trimmed.starts_with(":caption:") {
                caption = trimmed.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if !trimmed.starts_with(':') && !trimmed.is_empty() {
                entries.push(trimmed.to_string());
            }
        }
        
        let mut toc_html = String::new();
        
        if !caption.is_empty() {
            toc_html.push_str(&format!("<div class=\"toc-caption\">{}</div>", caption));
        }
        
        toc_html.push_str("<div class=\"toc-tree\">");
        
        for entry in entries {
            let title = entry.trim_end_matches(".rst");
            let url = title.replace('_', "-").to_lowercase() + ".html";
            toc_html.push_str(&format!(
                "<div class=\"toc-item\"><a href=\"{}\">{}</a></div>",
                url, title
            ));
        }
        
        toc_html.push_str("</div>");
        
        Ok(toc_html)
    }
}