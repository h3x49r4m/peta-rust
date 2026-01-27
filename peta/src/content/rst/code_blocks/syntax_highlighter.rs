//! Syntax highlighting using Syntect for code blocks

use crate::core::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::{ClassedHTMLGenerator, ClassStyle};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

/// Syntax highlighter for code blocks
pub struct SyntaxHighlighter {
    /// Syntax set for language definitions
    syntax_set: SyntaxSet,
    /// Theme set for color schemes
    theme_set: ThemeSet,
    /// Current theme
    theme: Theme,
    /// Language aliases
    language_aliases: HashMap<String, String>,
    /// Current theme name
    theme_name: String,
}

/// Configuration for syntax highlighting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlighterConfig {
    /// Theme name
    pub theme: String,
    /// Enable line numbers
    pub line_numbers: bool,
    /// Highlight specific lines
    pub highlight_lines: Vec<usize>,
}

impl Default for HighlighterConfig {
    fn default() -> Self {
        Self {
            theme: "one-dark".to_string(),
            line_numbers: true,
            highlight_lines: Vec::new(),
        }
    }
}

impl SyntaxHighlighter {
    /// Create a new syntax highlighter with default theme
    pub fn new() -> Result<Self> {
        Self::with_config(HighlighterConfig::default())
    }

    /// Create a syntax highlighter with custom configuration
    pub fn with_config(config: HighlighterConfig) -> Result<Self> {
        // Load syntax definitions
        let syntax_set = SyntaxSet::load_defaults_newlines();

        // Load themes
        let theme_set = ThemeSet::load_defaults();

        // Try to load theme
        let theme = theme_set
            .themes
            .get(&config.theme)
            .cloned()
            .unwrap_or_else(|| {
                // Fallback to base16-ocean.dark if theme not found
                theme_set
                    .themes
                    .get("base16-ocean.dark")
                    .unwrap()
                    .clone()
            });

        // Set up language aliases
        let mut language_aliases = HashMap::new();
        language_aliases.insert("js".to_string(), "javascript".to_string());
        language_aliases.insert("ts".to_string(), "typescript".to_string());
        language_aliases.insert("py".to_string(), "python".to_string());
        language_aliases.insert("py3".to_string(), "python".to_string());
        language_aliases.insert("python3".to_string(), "python".to_string());
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
            language_aliases,
            theme_name: config.theme,
        })
    }

    /// Highlight code and return HTML with syntax highlighting
    pub fn highlight(&self, code: &str, language: &str) -> Result<String> {
        // Resolve language
        let resolved_language = self.resolve_language(language);

        // Find syntax definition
        let syntax = self
            .syntax_set
            .find_syntax_by_token(&resolved_language)
            .or_else(|| self.syntax_set.find_syntax_by_extension(&resolved_language))
            .or_else(|| self.syntax_set.find_syntax_by_first_line(code))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        // Create HTML generator
        let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
            syntax,
            &self.syntax_set,
            ClassStyle::Spaced,
        );

        // Process each line
        for line in LinesWithEndings::from(code) {
            html_generator
                .parse_html_for_line_which_includes_newline(line)
                .map_err(|e| {
                    crate::core::Error::content(format!(
                        "Failed to parse line for syntax highlighting: {}",
                        e
                    ))
                })?;
        }

        let html = html_generator.finalize();

        // Convert syntect classes to our token classes
        let converted_html = self.convert_classes(&html);

        Ok(converted_html)
    }
    
    /// Convert syntect class names to our token classes
    /// Convert syntect class names to our token classes
    fn convert_classes(&self, html: &str) -> String {
        use regex::Regex;
        
        let mut result = html.to_string();
        
        // Keywords - must be after more specific patterns
        result = Regex::new(r#"class="keyword storage type function(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-keyword""#).to_string();
        result = Regex::new(r#"class="keyword storage type(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-type""#).to_string();
        result = Regex::new(r#"class="keyword storage modifier(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-keyword""#).to_string();
        result = Regex::new(r#"class="keyword control(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-keyword""#).to_string();
        result = Regex::new(r#"class="keyword operator(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-operator""#).to_string();
        result = Regex::new(r#"class="keyword(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-keyword""#).to_string();
        
        // Storage types
        result = Regex::new(r#"class="storage type(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-type""#).to_string();
        result = Regex::new(r#"class="storage modifier(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-keyword""#).to_string();
        
        // Entity names
        result = Regex::new(r#"class="entity name function(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-function""#).to_string();
        result = Regex::new(r#"class="entity name type(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-type""#).to_string();
        result = Regex::new(r#"class="entity name struct(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-type""#).to_string();
        result = Regex::new(r#"class="entity name class(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-type""#).to_string();
        result = Regex::new(r#"class="entity name variable(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-variable""#).to_string();
        result = Regex::new(r#"class="entity name constant(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-variable""#).to_string();
        result = Regex::new(r#"class="entity name label(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-variable""#).to_string();
        result = Regex::new(r#"class="entity name(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-variable""#).to_string();
        
        // Support
        result = Regex::new(r#"class="support function(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-function""#).to_string();
        result = Regex::new(r#"class="support type(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-type""#).to_string();
        result = Regex::new(r#"class="support class(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-type""#).to_string();
        result = Regex::new(r#"class="support constant(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-variable""#).to_string();
        
        // Variables
        result = Regex::new(r#"class="variable function(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-function""#).to_string();
        result = Regex::new(r#"class="variable parameter(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-variable""#).to_string();
        result = Regex::new(r#"class="variable other(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-variable""#).to_string();
        result = Regex::new(r#"class="variable(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-variable""#).to_string();
        
        // Constants
        result = Regex::new(r#"class="constant numeric(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-number""#).to_string();
        result = Regex::new(r#"class="constant other(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-variable""#).to_string();
        result = Regex::new(r#"class="constant(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-variable""#).to_string();
        
        // Strings
        result = Regex::new(r#"class="string(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-string""#).to_string();
        
        // Comments
        result = Regex::new(r#"class="comment(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-comment""#).to_string();
        
        // Punctuation
        result = Regex::new(r#"class="punctuation(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-punctuation""#).to_string();
        
        // Plain text
        result = Regex::new(r#"class="source(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-plain""#).to_string();
        result = Regex::new(r#"class="text(\s+[^"]*)?""#).unwrap().replace_all(&result, r#"class="token-plain""#).to_string();
        
        result
    }

    /// Highlight code with line numbers
    pub fn highlight_with_line_numbers(
        &self,
        code: &str,
        language: &str,
        highlight_lines: &[usize],
    ) -> Result<(String, usize)> {
        let highlighted = self.highlight(code, language)?;

        // Split into lines
        let lines: Vec<&str> = highlighted.lines().collect();
        let line_count = lines.len();

        // Add line numbers
        let mut result = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            let line_num = i + 1;
            let is_highlighted = highlight_lines.contains(&line_num);

            let highlight_class = if is_highlighted {
                "line-highlight"
            } else {
                ""
            };

            result.push(format!(
                r#"<span class="line-number {}" data-line="{}">{}</span>{}"#,
                highlight_class, line_num, line_num, line
            ));
        }

        Ok((result.join("\n"), line_count))
    }

    /// Resolve language alias
    fn resolve_language(&self, language: &str) -> String {
        let resolved = self.language_aliases
            .get(language)
            .cloned()
            .unwrap_or_else(|| language.to_string());

        // TypeScript isn't in syntect's default set, use JavaScript as fallback
        if resolved == "typescript" || resolved == "ts" {
            "javascript".to_string()
        } else {
            resolved
        }
    }

    /// Get available themes
    pub fn available_themes(&self) -> Vec<&str> {
        self.theme_set.themes.keys().map(|s| s.as_str()).collect()
    }

    /// Get available languages
    pub fn available_languages(&self) -> Vec<String> {
        self.syntax_set
            .syntaxes()
            .iter()
            .filter_map(|s| s.file_extensions.first())
            .map(|s| s.to_string())
            .collect()
    }

    /// Set theme
    pub fn set_theme(&mut self, theme_name: &str) -> Result<()> {
        if let Some(theme) = self.theme_set.themes.get(theme_name) {
            self.theme = theme.clone();
            self.theme_name = theme_name.to_string();
            Ok(())
        } else {
            Err(crate::core::Error::content(format!(
                "Theme '{}' not found",
                theme_name
            )))
        }
    }

    /// Get current theme name
    pub fn current_theme(&self) -> &str {
        &self.theme_name
    }

    /// Escape HTML entities
    #[allow(dead_code)]
    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new().expect("Failed to create SyntaxHighlighter")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_highlighting() {
        let highlighter = SyntaxHighlighter::new().unwrap();
        let code = "def hello():\n    print('Hello, world!')";

        let result = highlighter.highlight(code, "python").unwrap();
        // syntect uses different class names with spaces
        assert!(result.contains("class="));
        assert!(result.len() > 0);
    }

    #[test]
    fn test_language_resolution() {
        let highlighter = SyntaxHighlighter::new().unwrap();

        assert_eq!(highlighter.resolve_language("js"), "javascript");
        assert_eq!(highlighter.resolve_language("py"), "python");
        assert_eq!(highlighter.resolve_language("rs"), "rust");
        assert_eq!(highlighter.resolve_language("unknown"), "unknown");
    }

    #[test]
    fn test_line_numbers() {
        let highlighter = SyntaxHighlighter::new().unwrap();
        let code = "line1\nline2\nline3";

        let (html, line_count) = highlighter
            .highlight_with_line_numbers(code, "text", &[2])
            .unwrap();

        assert_eq!(line_count, 3);
        assert!(html.contains(r#"data-line="1""#));
        assert!(html.contains(r#"data-line="2""#));
        assert!(html.contains(r#"data-line="3""#));
        assert!(html.contains("line-highlight"));
    }

    #[test]
    fn test_available_themes() {
        let highlighter = SyntaxHighlighter::new().unwrap();
        let themes = highlighter.available_themes();

        assert!(!themes.is_empty());
        assert!(themes.contains(&"base16-ocean.dark"));
    }

    #[test]
    fn test_available_languages() {
        let highlighter = SyntaxHighlighter::new().unwrap();
        let languages = highlighter.available_languages();

        assert!(!languages.is_empty());
        assert!(languages.contains(&"rs".to_string()));
        assert!(languages.contains(&"py".to_string()));
        assert!(languages.contains(&"js".to_string()));
    }
}