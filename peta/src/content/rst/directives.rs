//! RST directive handlers

use crate::core::Result;

/// Trait for RST directive handlers
pub trait DirectiveHandler {
    /// Handle the directive and return HTML
    fn handle(&mut self, content: &str) -> Result<String>;
}

/// Code block directive handler
pub struct CodeBlockHandler;

impl DirectiveHandler for CodeBlockHandler {
    fn handle(&mut self, content: &str) -> Result<String> {
        // Extract language from first line
        let lines: Vec<&str> = content.lines().collect();
        let language = if let Some(first_line) = lines.first() {
            first_line.trim()
        } else {
            "text"
        };
        
        // Collect code lines (skip first line which is language)
        let code_lines: Vec<&str> = lines.iter().skip(1).copied().collect();
        let mut code = code_lines.join("\n");
        
        // Remove paragraph tags that might have been added by the paragraph converter
        code = code.replace("<p>", "").replace("</p>", "\n");
        
        Ok(format!(
            r#"<div class="code-block">
    <div class="code-header">
        <span class="language">{}</span>
        <button class="copy-button" onclick="copyCode(this)">Copy</button>
    </div>
    <pre><code class="language-{}">{}</code></pre>
</div>"#,
            language, language, code
        ))
    }
}

/// Snippet card directive handler
pub struct SnippetCardHandler;

impl DirectiveHandler for SnippetCardHandler {
    fn handle(&mut self, content: &str) -> Result<String> {
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
    fn handle(&mut self, content: &str) -> Result<String> {
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