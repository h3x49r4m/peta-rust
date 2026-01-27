//! Unified code block renderer using SyntaxHighlighter

use crate::content::rst::{SyntaxHighlighter, HighlighterConfig};
use crate::core::Result;
use serde::{Deserialize, Serialize};

/// Code block renderer that generates complete HTML structure
pub struct CodeBlockRenderer {
    /// Syntax highlighter
    highlighter: SyntaxHighlighter,
    /// Configuration
    config: CodeBlockConfig,
}

/// Configuration for code block rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlockConfig {
    /// Enable line numbers
    pub line_numbers: bool,
    /// Enable copy button
    pub copy_button: bool,
    /// Highlight specific lines
    pub highlight_lines: Vec<usize>,
    /// Theme name
    pub theme: String,
    /// Font family
    pub font_family: Option<String>,
    /// Font size
    pub font_size: Option<String>,
}

impl Default for CodeBlockConfig {
    fn default() -> Self {
        Self {
            line_numbers: true,
            copy_button: true,
            highlight_lines: Vec::new(),
            theme: "one-dark".to_string(),
            font_family: None,
            font_size: None,
        }
    }
}

impl CodeBlockRenderer {
    /// Create a new code block renderer with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(CodeBlockConfig::default())
    }

    /// Create a code block renderer with custom configuration
    pub fn with_config(config: CodeBlockConfig) -> Result<Self> {
        let highlighter_config = HighlighterConfig {
            theme: config.theme.clone(),
            line_numbers: config.line_numbers,
            highlight_lines: config.highlight_lines.clone(),
        };

        let highlighter = SyntaxHighlighter::with_config(highlighter_config)?;

        Ok(Self {
            highlighter,
            config,
        })
    }

    /// Render a code block to HTML
    pub fn render(&self, code: &str, language: &str, title: Option<&str>) -> Result<String> {
        // Highlight code
        let (highlighted_code, line_count) = if self.config.line_numbers {
            self.highlighter
                .highlight_with_line_numbers(code, language, &self.config.highlight_lines)?
        } else {
            let highlighted = self.highlighter.highlight(code, language)?;
            let line_count = code.lines().count();
            (highlighted, line_count)
        };

        // Generate HTML structure
        let mut html = String::new();

        // Start code block container
        html.push_str(r#"<div class="code-block""#);

        // Add data attributes
        html.push_str(&format!(r#" data-language="{}""#, language));
        html.push_str(&format!(r#" data-theme="{}""#, self.config.theme));
        html.push_str(&format!(r#" data-line-count="{}""#, line_count));

        if self.config.line_numbers {
            html.push_str(r#" data-line-numbers="true""#);
        }

        html.push('>');
        html.push('\n');

        // Code header
        html.push_str("  <div class=\"code-header\">\n");
        html.push_str("    <div class=\"code-info\">\n");

        if let Some(t) = title {
            html.push_str(&format!(r#"      <span class="code-title">{}</span>"#, t));
            html.push('\n');
        }

        html.push_str(&format!(
            r#"      <span class="code-language">{}</span>"#,
            language.to_uppercase()
        ));
        html.push('\n');
        html.push_str("    </div>\n");

        // Copy button
        if self.config.copy_button {
            html.push_str(r#"    <button class="code-copy-button" onclick="copyCode(this)" aria-label="Copy code">"#);
            html.push('\n');
            html.push_str(r#"      <svg class="code-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">"#);
            html.push('\n');
            html.push_str(r#"        <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>"#);
            html.push('\n');
            html.push_str(r#"        <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>"#);
            html.push('\n');
            html.push_str("      </svg>\n");
            html.push_str("      <span class=\"copy-text\">Copy</span>\n");
            html.push_str("    </button>\n");
        }

        html.push_str("  </div>\n");

        // Code content
        let content_class = if self.config.line_numbers {
            "code-content with-line-numbers"
        } else {
            "code-content"
        };

        html.push_str(&format!("  <div class=\"{}\">\n", content_class));
        html.push_str("    <pre><code class=\"language-");
        html.push_str(language);
        html.push_str("\">\n");
        html.push_str(&highlighted_code);
        html.push_str("\n    </code></pre>\n");
        html.push_str("  </div>\n");

        // End code block container
        html.push_str("</div>");

        Ok(html)
    }

    /// Set theme
    pub fn set_theme(&mut self, theme: &str) -> Result<()> {
        self.highlighter.set_theme(theme)?;
        self.config.theme = theme.to_string();
        Ok(())
    }

    /// Get current theme
    pub fn theme(&self) -> &str {
        &self.config.theme
    }

    /// Set line numbers
    pub fn set_line_numbers(&mut self, enabled: bool) {
        self.config.line_numbers = enabled;
    }

    /// Set copy button
    pub fn set_copy_button(&mut self, enabled: bool) {
        self.config.copy_button = enabled;
    }

    /// Set highlight lines
    pub fn set_highlight_lines(&mut self, lines: Vec<usize>) {
        self.config.highlight_lines = lines;
    }

    /// Get configuration
    pub fn config(&self) -> &CodeBlockConfig {
        &self.config
    }
}

impl Default for CodeBlockRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create CodeBlockRenderer")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_block_rendering() {
        let renderer = CodeBlockRenderer::new().unwrap();
        let code = "def hello():\n    print('Hello, world!')";

        let html = renderer.render(code, "python", None).unwrap();

        assert!(html.contains("code-block"));
        assert!(html.contains("data-language=\"python\""));
        assert!(html.contains("code-header"));
        assert!(html.contains("code-content"));
        assert!(html.contains("data-line-numbers=\"true\""));
    }

    #[test]
    fn test_code_block_with_title() {
        let renderer = CodeBlockRenderer::new().unwrap();
        let code = "const x = 42;";

        let html = renderer.render(code, "javascript", Some("Example Code")).unwrap();

        assert!(html.contains("code-title"));
        assert!(html.contains("Example Code"));
    }

    #[test]
    fn test_code_block_without_line_numbers() {
        let mut config = CodeBlockConfig::default();
        config.line_numbers = false;
        let renderer = CodeBlockRenderer::with_config(config).unwrap();
        let code = "line1\nline2";

        let html = renderer.render(code, "text", None).unwrap();

        assert!(!html.contains("data-line-numbers=\"true\""));
        assert!(!html.contains("with-line-numbers"));
    }

    #[test]
    fn test_code_block_without_copy_button() {
        let mut config = CodeBlockConfig::default();
        config.copy_button = false;
        let renderer = CodeBlockRenderer::with_config(config).unwrap();
        let code = "code";

        let html = renderer.render(code, "text", None).unwrap();

        assert!(!html.contains("code-copy-button"));
    }

    #[test]
    fn test_code_block_with_highlight_lines() {
        let mut config = CodeBlockConfig::default();
        config.highlight_lines = vec![2, 3];
        let renderer = CodeBlockRenderer::with_config(config).unwrap();
        let code = "line1\nline2\nline3\nline4";

        let html = renderer.render(code, "text", None).unwrap();

        assert!(html.contains("line-highlight"));
    }

    #[test]
    fn test_set_theme() {
        let mut renderer = CodeBlockRenderer::new().unwrap();
        // Use base16-ocean.dark which is available in syntect
        renderer.set_theme("base16-ocean.dark").unwrap();

        assert_eq!(renderer.theme(), "base16-ocean.dark");
    }

    #[test]
    fn test_set_line_numbers() {
        let mut renderer = CodeBlockRenderer::new().unwrap();
        renderer.set_line_numbers(false);

        assert!(!renderer.config().line_numbers);
    }

    #[test]
    fn test_set_copy_button() {
        let mut renderer = CodeBlockRenderer::new().unwrap();
        renderer.set_copy_button(false);

        assert!(!renderer.config().copy_button);
    }

    #[test]
    fn test_set_highlight_lines() {
        let mut renderer = CodeBlockRenderer::new().unwrap();
        renderer.set_highlight_lines(vec![1, 3, 5]);

        assert_eq!(renderer.config().highlight_lines, vec![1, 3, 5]);
    }
}