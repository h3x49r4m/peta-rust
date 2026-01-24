//! Code highlighting for RST content using Syntect

use crate::core::Result;
use std::collections::HashMap;
use syntect::highlighting::{ThemeSet, Theme};
use syntect::parsing::SyntaxSet;
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;
use syntect::util::as_24_bit_terminal_escaped;

/// Code highlighter for syntax highlighting
pub struct CodeHighlighter {
    /// Syntax set for language definitions
    syntax_set: SyntaxSet,
    /// Theme set for color schemes
    theme_set: ThemeSet,
    /// Current theme
    theme: Theme,
    /// Configuration
    config: CodeConfig,
    /// Language aliases
    language_aliases: HashMap<String, String>,
}

/// Configuration for code highlighting
#[derive(Debug, Clone)]
pub struct CodeConfig {
    /// Enable syntax highlighting
    pub enabled: bool,
    /// Show line numbers
    pub line_numbers: bool,
    /// Show copy button
    pub copy_button: bool,
    /// Theme name
    pub theme: String,
    /// Tab size
    pub tab_size: usize,
    /// Wrap long lines
    pub wrap_lines: bool,
    /// Highlight specific lines
    pub highlight_lines: Option<Vec<usize>>,
}

impl Default for CodeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            line_numbers: true,
            copy_button: true,
            theme: "one-dark".to_string(),
            tab_size: 4,
            wrap_lines: false,
            highlight_lines: None,
        }
    }
}

impl CodeHighlighter {
    /// Create a new code highlighter
    pub fn new() -> Result<Self> {
        Self::with_config(CodeConfig::default())
    }
    
    /// Create a code highlighter with custom configuration
    pub fn with_config(config: CodeConfig) -> Result<Self> {
        // Load syntax definitions
        let syntax_set = SyntaxSet::load_defaults_newlines();
        
        // Load themes
        let theme_set = ThemeSet::load_defaults();
        
        // Try to load custom theme
        let theme = theme_set.themes.get(&config.theme)
            .cloned()
            .unwrap_or_else(|| {
                // Fallback to base16-ocean.dark if theme not found
                theme_set.themes.get("base16-ocean.dark").unwrap().clone()
            });
        
        // Set up language aliases
        let mut language_aliases = HashMap::new();
        language_aliases.insert("js".to_string(), "javascript".to_string());
        language_aliases.insert("ts".to_string(), "typescript".to_string());
        language_aliases.insert("py".to_string(), "python".to_string());
        language_aliases.insert("rb".to_string(), "ruby".to_string());
        language_aliases.insert("sh".to_string(), "bash".to_string());
        language_aliases.insert("shell".to_string(), "bash".to_string());
        language_aliases.insert("zsh".to_string(), "bash".to_string());
        language_aliases.insert("fish".to_string(), "fish".to_string());
        language_aliases.insert("rs".to_string(), "rust".to_string());
        language_aliases.insert("go".to_string(), "go".to_string());
        language_aliases.insert("cpp".to_string(), "c++".to_string());
        language_aliases.insert("cxx".to_string(), "c++".to_string());
        language_aliases.insert("cc".to_string(), "c++".to_string());
        language_aliases.insert("c".to_string(), "c".to_string());
        language_aliases.insert("h".to_string(), "c".to_string());
        language_aliases.insert("hpp".to_string(), "c++".to_string());
        language_aliases.insert("java".to_string(), "java".to_string());
        language_aliases.insert("kt".to_string(), "kotlin".to_string());
        language_aliases.insert("scala".to_string(), "scala".to_string());
        language_aliases.insert("cs".to_string(), "csharp".to_string());
        language_aliases.insert("php".to_string(), "php".to_string());
        language_aliases.insert("html".to_string(), "html".to_string());
        language_aliases.insert("htm".to_string(), "html".to_string());
        language_aliases.insert("xml".to_string(), "xml".to_string());
        language_aliases.insert("css".to_string(), "css".to_string());
        language_aliases.insert("scss".to_string(), "scss".to_string());
        language_aliases.insert("sass".to_string(), "sass".to_string());
        language_aliases.insert("less".to_string(), "less".to_string());
        language_aliases.insert("json".to_string(), "json".to_string());
        language_aliases.insert("yaml".to_string(), "yaml".to_string());
        language_aliases.insert("yml".to_string(), "yaml".to_string());
        language_aliases.insert("toml".to_string(), "toml".to_string());
        language_aliases.insert("sql".to_string(), "sql".to_string());
        language_aliases.insert("dockerfile".to_string(), "dockerfile".to_string());
        language_aliases.insert("makefile".to_string(), "makefile".to_string());
        language_aliases.insert("cmake".to_string(), "cmake".to_string());
        language_aliases.insert("diff".to_string(), "diff".to_string());
        language_aliases.insert("patch".to_string(), "diff".to_string());
        language_aliases.insert("log".to_string(), "log".to_string());
        
        Ok(Self {
            syntax_set,
            theme_set,
            theme,
            config,
            language_aliases,
        })
    }
    
    /// Highlight code with syntax highlighting
    pub fn highlight_code(&self, code: &str, language: &str) -> Result<HighlightedCode> {
        if !self.config.enabled {
            return Ok(HighlightedCode {
                html: self.escape_html(code),
                language: language.to_string(),
                line_count: code.lines().count(),
            });
        }
        
        // Resolve language
        let resolved_language = self.resolve_language(language);
        
        // Find syntax definition
        let syntax = self.syntax_set
            .find_syntax_by_token(&resolved_language)
            .or_else(|| self.syntax_set.find_syntax_by_extension(&resolved_language))
            .or_else(|| self.syntax_set.find_syntax_by_first_line(code))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        
        // Create highlighter
        let mut highlighter = HighlightLines::new(syntax, &self.theme);
        
        // Process each line
        let mut lines = Vec::new();
        let mut line_number = 1;
        
        for line in LinesWithEndings::from(code) {
            let ranges = highlighter.highlight_line(line, &self.syntax_set)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            
            let highlighted_line = if self.config.line_numbers {
                let line_number_html = format!(
                    "<span class=\"line-number\" data-line=\"{}\">{}</span>",
                    line_number, line_number
                );
                format!("{} {}", line_number_html, as_24_bit_terminal_escaped(&ranges[..], false))
            } else {
                as_24_bit_terminal_escaped(&ranges[..], false)
            };
            
            // Highlight specific lines if configured
            let final_line = if let Some(ref highlight_lines) = self.config.highlight_lines {
                if highlight_lines.contains(&line_number) {
                    format!("<span class=\"line-highlight\">{}</span>", highlighted_line)
                } else {
                    highlighted_line
                }
            } else {
                highlighted_line
            };
            
            lines.push(final_line);
            line_number += 1;
        }
        
        let html = lines.join("\n");
        
        Ok(HighlightedCode {
            html,
            language: resolved_language.to_string(),
            line_count: code.lines().count(),
        })
    }
    
    /// Highlight code and return complete HTML block
    pub fn highlight_code_block(&self, code: &str, language: &str) -> Result<String> {
        let highlighted = self.highlight_code(code, language)?;
        
        let mut html = String::new();
        
        // Start code block
        html.push_str("<div class=\"code-block\">");
        
        // Header with language and copy button
        html.push_str("<div class=\"code-header\">");
        html.push_str(&format!("<span class=\"code-language\">{}</span>", highlighted.language));
        
        if self.config.copy_button {
            html.push_str("<button class=\"code-copy-button\" onclick=\"copyCode(this)\">");
            html.push_str("<svg class=\"code-icon\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\">");
            html.push_str("<rect x=\"9\" y=\"9\" width=\"13\" height=\"13\" rx=\"2\" ry=\"2\"></rect>");
            html.push_str("<path d=\"M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1\"></path>");
            html.push_str("</svg>");
            html.push_str("<span class=\"copy-text\">Copy</span>");
            html.push_str("</button>");
        }
        
        html.push_str("</div>");
        
        // Code content
        let class = if self.config.line_numbers {
            "code-content with-line-numbers"
        } else {
            "code-content"
        };
        
        html.push_str(&format!(
            "<pre class=\"{}\"><code class=\"language-{}\">{}</code></pre>",
            class, highlighted.language, highlighted.html
        ));
        
        html.push_str("</div>");
        
        Ok(html)
    }
    
    /// Resolve language alias
    fn resolve_language(&self, language: &str) -> String {
        self.language_aliases
            .get(language)
            .cloned()
            .unwrap_or_else(|| language.to_string())
    }
    
    /// Escape HTML entities
    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('\"', "&quot;")
            .replace('\'', "&#x27;")
    }
    
    /// Get available themes
    pub fn available_themes(&self) -> Vec<&str> {
        self.theme_set.themes.keys().map(|s| s.as_str()).collect()
    }
    
    /// Get available languages
    pub fn available_languages(&self) -> Vec<String> {
        self.syntax_set.syntaxes()
            .iter()
            .filter_map(|s| s.file_extensions.first())
            .map(|s| s.to_string())
            .collect()
    }
    
    /// Set theme
    pub fn set_theme(&mut self, theme_name: &str) -> Result<()> {
        if let Some(theme) = self.theme_set.themes.get(theme_name) {
            self.theme = theme.clone();
            self.config.theme = theme_name.to_string();
            Ok(())
        } else {
            Err(crate::core::Error::Content(format!("Theme '{}' not found", theme_name)))
        }
    }
    
    /// Get current theme name
    pub fn current_theme(&self) -> &str {
        &self.config.theme
    }
}

/// Result of code highlighting
#[derive(Debug, Clone)]
pub struct HighlightedCode {
    /// Highlighted HTML
    pub html: String,
    /// Detected language
    pub language: String,
    /// Number of lines
    pub line_count: usize,
}

impl HighlightedCode {
    /// Create new highlighted code
    pub fn new(html: String, language: String, line_count: usize) -> Self {
        Self {
            html,
            language,
            line_count,
        }
    }
    
    /// Get HTML as string
    pub fn as_html(&self) -> &str {
        &self.html
    }
    
    /// Get language
    pub fn language(&self) -> &str {
        &self.language
    }
    
    /// Get line count
    pub fn line_count(&self) -> usize {
        self.line_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_code_highlighting() {
        let highlighter = CodeHighlighter::new().unwrap();
        let code = "fn main() {\n    println!(\"Hello, world!\");\n}";
        
        let result = highlighter.highlight_code(code, "rust").unwrap();
        assert_eq!(result.language, "rust");
        assert_eq!(result.line_count, 3);
        assert!(!result.html.is_empty());
    }
    
    #[test]
    fn test_language_resolution() {
        let highlighter = CodeHighlighter::new().unwrap();
        
        assert_eq!(highlighter.resolve_language("js"), "javascript");
        assert_eq!(highlighter.resolve_language("py"), "python");
        assert_eq!(highlighter.resolve_language("unknown"), "unknown");
    }
    
    #[test]
    fn test_code_block_generation() {
        let highlighter = CodeHighlighter::new().unwrap();
        let code = "print('Hello, world!')";
        
        let html = highlighter.highlight_code_block(code, "python").unwrap();
        assert!(html.contains("code-block"));
        assert!(html.contains("python"));
        assert!(html.contains("Copy"));
    }
    
    #[test]
    fn test_highlighting_disabled() {
        let mut config = CodeConfig::default();
        config.enabled = false;
        let highlighter = CodeHighlighter::with_config(config).unwrap();
        
        let code = "fn main() {}";
        let result = highlighter.highlight_code(code, "rust").unwrap();
        assert_eq!(result.html, highlighter.escape_html(code));
    }
}