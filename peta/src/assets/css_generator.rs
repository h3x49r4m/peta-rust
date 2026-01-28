//! CSS generator for code blocks

use crate::core::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// CSS generator for code block styling
pub struct CssGenerator {
    /// Configuration
    config: CssConfig,
    /// Token colors from theme
    token_colors: HashMap<String, String>,
}

/// Configuration for CSS generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssConfig {
    /// Theme name
    pub theme: String,
    /// Font family
    pub font_family: String,
    /// Font size
    pub font_size: String,
    /// Line height
    pub line_height: String,
    /// Border radius
    pub border_radius: String,
    /// Background colors
    pub background_gradient_start: String,
    pub background_gradient_end: String,
    /// Header background
    pub header_background: String,
}

impl Default for CssConfig {
    fn default() -> Self {
        Self {
            theme: "one-dark".to_string(),
            font_family: "SF Mono, Monaco, 'Inconsolata', 'Roboto Mono', 'Source Code Pro', monospace".to_string(),
            font_size: "0.9rem".to_string(),
            line_height: "1.6".to_string(),
            border_radius: "1rem".to_string(),
            background_gradient_start: "#1e293b".to_string(),
            background_gradient_end: "#0f172a".to_string(),
            header_background: "rgba(15, 23, 42, 0.6)".to_string(),
        }
    }
}

impl CssGenerator {
    /// Create a new CSS generator with default configuration
    pub fn new() -> Self {
        Self::with_config(CssConfig::default())
    }

    /// Create a CSS generator with custom configuration
    pub fn with_config(config: CssConfig) -> Self {
        let mut generator = Self {
            config,
            token_colors: HashMap::new(),
        };

        // Initialize token colors based on theme
        generator.init_token_colors();

        generator
    }

    /// Generate complete CSS for code blocks
    pub fn generate(&self) -> Result<String> {
        let mut css = String::new();

        css.push_str("/* Code Block Styles */\n\n");

        // Base styles
        css.push_str(&self.generate_base_styles());

        // Header styles
        css.push_str(&self.generate_header_styles());

        // Content styles
        css.push_str(&self.generate_content_styles());

        // Line number styles
        css.push_str(&self.generate_line_number_styles());

        // Copy button styles
        css.push_str(&self.generate_copy_button_styles());

        // Token colors
        css.push_str(&self.generate_token_colors());

        // Responsive styles
        css.push_str(&self.generate_responsive_styles());

        Ok(css)
    }

    /// Initialize token colors based on theme
    fn init_token_colors(&mut self) {
        match self.config.theme.as_str() {
            "one-dark" => {
                self.token_colors.insert("keyword".to_string(), "#c678dd".to_string());
                self.token_colors.insert("string".to_string(), "#98c379".to_string());
                self.token_colors.insert("comment".to_string(), "#5c6370".to_string());
                self.token_colors.insert("number".to_string(), "#d19a66".to_string());
                self.token_colors.insert("function".to_string(), "#61afef".to_string());
                self.token_colors.insert("variable".to_string(), "#e06c75".to_string());
                self.token_colors.insert("type".to_string(), "#e5c07b".to_string());
                self.token_colors.insert("operator".to_string(), "#56b6c2".to_string());
                self.token_colors.insert("punctuation".to_string(), "#abb2bf".to_string());
                self.token_colors.insert("property".to_string(), "#d19a66".to_string());
                self.token_colors.insert("tag".to_string(), "#e06c75".to_string());
                self.token_colors.insert("attribute".to_string(), "#d19a66".to_string());
                self.token_colors.insert("selector".to_string(), "#c678dd".to_string());
            }
            "solarized" => {
                self.token_colors.insert("keyword".to_string(), "#859900".to_string());
                self.token_colors.insert("string".to_string(), "#2aa198".to_string());
                self.token_colors.insert("comment".to_string(), "#93a1a1".to_string());
                self.token_colors.insert("number".to_string(), "#2aa198".to_string());
                self.token_colors.insert("function".to_string(), "#268bd2".to_string());
                self.token_colors.insert("variable".to_string(), "#b58900".to_string());
                self.token_colors.insert("type".to_string(), "#cb4b16".to_string());
                self.token_colors.insert("operator".to_string(), "#6c71c4".to_string());
                self.token_colors.insert("punctuation".to_string(), "#839496".to_string());
                self.token_colors.insert("property".to_string(), "#b58900".to_string());
                self.token_colors.insert("tag".to_string(), "#268bd2".to_string());
                self.token_colors.insert("attribute".to_string(), "#b58900".to_string());
                self.token_colors.insert("selector".to_string(), "#6c71c4".to_string());
            }
            _ => {
                // Default colors
                self.token_colors.insert("keyword".to_string(), "#c678dd".to_string());
                self.token_colors.insert("string".to_string(), "#98c379".to_string());
                self.token_colors.insert("comment".to_string(), "#5c6370".to_string());
                self.token_colors.insert("number".to_string(), "#d19a66".to_string());
                self.token_colors.insert("function".to_string(), "#61afef".to_string());
                self.token_colors.insert("variable".to_string(), "#e06c75".to_string());
                self.token_colors.insert("type".to_string(), "#e5c07b".to_string());
                self.token_colors.insert("operator".to_string(), "#56b6c2".to_string());
                self.token_colors.insert("punctuation".to_string(), "#abb2bf".to_string());
            }
        }
    }

    /// Generate base styles
    fn generate_base_styles(&self) -> String {
        format!(
            r#"
.code-block {{
  margin: 2rem 0;
  border-radius: {};
  overflow: hidden;
  background: linear-gradient(135deg, {} 0%, {} 100%);
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  border: 1px solid rgba(255, 255, 255, 0.1);
  position: relative;
  transition: all 0.3s ease;
  max-width: 100%;
  width: 100%;
  box-sizing: border-box;
}}

.code-block::before {{
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: linear-gradient(90deg, #3b82f6, #8b5cf6, #ec4899);
  opacity: 0.8;
}}

.code-block:hover {{
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25), 0 0 0 1px rgba(59, 130, 246, 0.1);
  transform: translateY(-2px);
}}

.code-block:focus-within {{
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25), 0 0 0 2px rgba(59, 130, 246, 0.3);
}}
"#,
            self.config.border_radius,
            self.config.background_gradient_start,
            self.config.background_gradient_end
        )
    }

    /// Generate header styles
    fn generate_header_styles(&self) -> String {
        format!(
            r#"
.code-header {{
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  background: {};
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}}

.code-info {{
  display: flex;
  align-items: center;
  gap: 0.75rem;
}}

.code-title {{
  font-size: 0.875rem;
  font-weight: 600;
  color: #f1f5f9;
}}

.code-language {{
  font-size: 0.875rem;
  font-weight: 600;
  color: #e2e8f0;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}}

.code-language::before {{
  content: '';
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  box-shadow: 0 0 10px rgba(59, 130, 246, 0.5);
}}
"#,
            self.config.header_background
        )
    }

    /// Generate content styles
    fn generate_content_styles(&self) -> String {
        format!(
            r#"
.code-content {{
  padding: 1.5rem;
  overflow-x: auto;
  background: rgba(15, 23, 42, 0.3);
  position: relative;
}}

.code-content pre {{
  margin: 0;
  padding: 0;
  background: none;
  color: #e2e8f0;
  font-family: {};
  font-size: {};
  line-height: {};
  white-space: pre;
  word-wrap: normal;
  tab-size: 4;
}}

.code-content::-webkit-scrollbar {{
  height: 8px;
}}

.code-content::-webkit-scrollbar-track {{
  background: rgba(30, 41, 59, 0.5);
  border-radius: 4px;
}}

.code-content::-webkit-scrollbar-thumb {{
  background: rgba(148, 163, 184, 0.3);
  border-radius: 4px;
}}

.code-content::-webkit-scrollbar-thumb:hover {{
  background: rgba(148, 163, 184, 0.5);
}}
"#,
            self.config.font_family,
            self.config.font_size,
            self.config.line_height
        )
    }

    /// Generate line number styles
    fn generate_line_number_styles(&self) -> String {
        r#"
.code-content.with-line-numbers {
  padding-left: 0;
}

.line-number {
  display: inline-block;
  width: 3.5rem;
  padding: 0 1rem;
  text-align: right;
  color: #64748b;
  background: rgba(30, 41, 59, 0.5);
  border-right: 1px solid rgba(255, 255, 255, 0.1);
  user-select: none;
  font-weight: 500;
  transition: background-color 0.2s ease;
  cursor: pointer;
}

.line-number:hover {
  background: rgba(59, 130, 246, 0.1);
  color: #e2e8f0;
}

.line-highlight {
  background: linear-gradient(90deg, rgba(59, 130, 246, 0.2), rgba(139, 92, 246, 0.1));
  display: block;
  border-left: 3px solid #3b82f6;
  margin-left: -3px;
  position: relative;
}

.line-highlight .line-number {
  background: rgba(59, 130, 246, 0.3);
  color: #e2e8f0;
}
"#
        .to_string()
    }

    /// Generate copy button styles
    fn generate_copy_button_styles(&self) -> String {
        r#"
.code-copy-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: #e2e8f0;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
}

.code-copy-button:hover {
  background: rgba(255, 255, 255, 0.2);
  border-color: rgba(255, 255, 255, 0.3);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.code-copy-button:active {
  transform: translateY(0);
}

.code-copy-button:focus {
  outline: 2px solid #3b82f6;
  outline-offset: 2px;
}

.code-icon {
  width: 1rem;
  height: 1rem;
  transition: transform 0.2s ease;
}

.code-copy-button:hover .code-icon {
  transform: scale(1.1);
}

@keyframes copySuccess {
  0% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.05);
  }
  100% {
    transform: scale(1);
  }
}

.code-copy-button.copied {
  animation: copySuccess 0.3s ease;
  background: rgba(16, 185, 129, 0.2);
  border-color: rgba(16, 185, 129, 0.5);
}
"#
        .to_string()
    }

    /// Generate token colors
    fn generate_token_colors(&self) -> String {
        let mut css = String::new();
        css.push_str("/* Syntax Highlighting Colors */\n\n");

        for (token_type, color) in &self.token_colors {
            css.push_str(&format!(
                ".token-{} {{\n  color: {};\n}}\n\n",
                token_type, color
            ));
        }

        // Additional token styles
        css.push_str(r#"
.token.italic {
  font-style: italic;
}

.token.bold {
  font-weight: bold;
}

.token.underline {
  text-decoration: underline;
}

.token.inserted {
  background-color: rgba(134, 239, 172, 0.2);
  color: #86efac;
  border-radius: 2px;
  padding: 0 2px;
}

.token.deleted {
  background-color: rgba(248, 113, 113, 0.2);
  color: #f87171;
  border-radius: 2px;
  padding: 0 2px;
}

/* Syntect wrapper classes - inherit styling from child elements */
.meta,
.source,
.text {
  color: inherit;
}
"#);

        css
    }

    /// Generate responsive styles
    fn generate_responsive_styles(&self) -> String {
        r#"
@media (max-width: 768px) {
  .code-block {
    margin: 1.5rem 0;
    border-radius: 0.75rem;
  }

  .code-header {
    padding: 0.75rem 1rem;
  }

  .code-content {
    padding: 1rem;
  }

  .code-language {
    font-size: 0.75rem;
  }

  .code-copy-button {
    padding: 0.25rem 0.75rem;
    font-size: 0.75rem;
  }

  .line-number {
    width: 2.5rem;
    padding: 0 0.5rem;
  }

  .code-content pre {
    font-size: 0.8rem;
  }
}
"#
        .to_string()
    }

    /// Set theme
    pub fn set_theme(&mut self, theme: &str) {
        self.config.theme = theme.to_string();
        self.init_token_colors();
    }

    /// Get configuration
    pub fn config(&self) -> &CssConfig {
        &self.config
    }
}

impl Default for CssGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_generation() {
        let generator = CssGenerator::new();
        let css = generator.generate().unwrap();

        assert!(css.contains(".code-block"));
        assert!(css.contains(".code-header"));
        assert!(css.contains(".code-content"));
        assert!(css.contains(".line-number"));
        assert!(css.contains(".code-copy-button"));
        assert!(css.contains(".token-keyword"));
    }

    #[test]
    fn test_theme_token_colors() {
        let mut generator = CssGenerator::new();
        generator.set_theme("solarized");

        assert_eq!(generator.token_colors.get("keyword"), Some(&"#859900".to_string()));
        assert_eq!(generator.token_colors.get("string"), Some(&"#2aa198".to_string()));
    }

    #[test]
    fn test_custom_config() {
        let config = CssConfig {
            theme: "custom".to_string(),
            font_family: "Custom Font".to_string(),
            font_size: "1rem".to_string(),
            ..Default::default()
        };

        let generator = CssGenerator::with_config(config);
        let css = generator.generate().unwrap();

        assert!(css.contains("Custom Font"));
        assert!(css.contains("1rem"));
    }
}

/// CSS generator for embedded snippet card styling
pub struct EmbeddedSnippetCardCssGenerator {
    /// Configuration
    config: EmbeddedSnippetCardConfig,
}

/// Configuration for embedded snippet card CSS generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedSnippetCardConfig {
    /// Border radius
    pub border_radius: String,
    /// Border color
    pub border_color: String,
    /// Background color
    pub background_color: String,
    /// Shadow
    pub shadow: String,
    /// Header background
    pub header_background: String,
    /// Header border
    pub header_border: String,
    /// Title color
    pub title_color: String,
    /// Tag color
    pub tag_color: String,
    /// Content padding
    pub content_padding: String,
    /// Content background
    pub content_background: String,
    /// Footer background
    pub footer_background: String,
    /// Footer border
    pub footer_border: String,
    /// Link color
    pub link_color: String,
}

impl Default for EmbeddedSnippetCardConfig {
    fn default() -> Self {
        Self {
            border_radius: "0.5rem".to_string(),
            border_color: "#e2e8f0".to_string(),
            background_color: "#eff6ff".to_string(),
            shadow: "0 2px 4px rgba(0, 0, 0, 0.05)".to_string(),
            header_background: "#dbeafe".to_string(),
            header_border: "#bfdbfe".to_string(),
            title_color: "#1e40af".to_string(),
            tag_color: "#3b82f6".to_string(),
            content_padding: "1.5rem".to_string(),
            content_background: "#ffffff".to_string(),
            footer_background: "#dbeafe".to_string(),
            footer_border: "#bfdbfe".to_string(),
            link_color: "#3b82f6".to_string(),
        }
    }
}

impl EmbeddedSnippetCardCssGenerator {
    /// Create a new embedded snippet card CSS generator
    pub fn new() -> Result<Self> {
        Ok(Self {
            config: EmbeddedSnippetCardConfig::default(),
        })
    }

    /// Create a CSS generator with custom configuration
    pub fn with_config(config: EmbeddedSnippetCardConfig) -> Result<Self> {
        Ok(Self { config })
    }

    /// Generate complete CSS for embedded snippet cards
    pub fn generate(&self) -> Result<String> {
        let mut css = String::new();

        css.push_str("/* Embedded Snippet Card Styles */\n\n");

        // Base styles
        css.push_str(&self.generate_base_styles());

        // Header styles
        css.push_str(&self.generate_header_styles());

        // Content styles
        css.push_str(&self.generate_content_styles());

        // Footer styles
        css.push_str(&self.generate_footer_styles());

        // Tag styles
        css.push_str(&self.generate_tag_styles());

        // Error styles
        css.push_str(&self.generate_error_styles());

        // Responsive styles
        css.push_str(&self.generate_responsive_styles());

        Ok(css)
    }

    /// Generate base styles
    fn generate_base_styles(&self) -> String {
        format!(
            r#"
.embedded-snippet-card {{
  margin: 2rem 0;
  border-radius: {};
  border: 1px solid #bfdbfe;
  border-left: 4px solid #3b82f6;
  background: {};
  box-shadow: {};
  overflow: hidden;
  position: relative;
  transition: all 0.3s ease;
}}

.embedded-snippet-card::before {{
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: linear-gradient(90deg, #3b82f6, #60a5fa);
  opacity: 0.6;
}}

.embedded-snippet-card:hover {{
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.15);
  transform: translateX(4px);
}}
"#,
            self.config.border_radius,
            self.config.background_color,
            self.config.shadow
        )
    }

    /// Generate header styles

        fn generate_header_styles(&self) -> String {

            format!(

                r#"

    .embedded-snippet-header {{

      padding: 1rem 1.25rem;

      background: {};

      border-bottom: 1px solid {};

      display: flex;

      justify-content: space-between;

      align-items: center;

      flex-wrap: wrap;

      gap: 0.5rem;

    }}

    

    .embedded-snippet-title {{

      font-size: 1.125rem;

      font-weight: 600;

      color: {};

      margin: 0;

      flex: 1;

      min-width: 0;

    }}

    

    .embedded-snippet-title a {{

      color: inherit;

      text-decoration: none;

      display: block;

      overflow: hidden;

      text-overflow: ellipsis;

      white-space: nowrap;

    }}

    

    .embedded-snippet-title a:hover {{

      text-decoration: underline;

    }}

    "#,

                self.config.header_background,

                self.config.header_border,

                self.config.title_color

            )

        }

/// Generate content styles
    fn generate_content_styles(&self) -> String {
        format!(
            r#"
.embedded-snippet-content {{
  padding: {};
  background: {};
}}

.embedded-snippet-content h1 {{
  color: #3b82f6;
  margin-top: 2rem;
  margin-bottom: 1rem;
  font-size: 2rem;
  font-weight: 700;
}}

.embedded-snippet-content h1:first-child {{
  margin-top: 0;
}}

.embedded-snippet-content h2 {{
  color: #3b82f6;
  margin-top: 1.5rem;
  margin-bottom: 1rem;
  font-size: 1.5rem;
  font-weight: 600;
}}

.embedded-snippet-content h2:first-child {{
  margin-top: 0;
}}

.embedded-snippet-content h3 {{
  color: #60a5fa;
  margin-top: 1.25rem;
  margin-bottom: 0.75rem;
  font-size: 1.25rem;
  font-weight: 600;
}}

.embedded-snippet-content h4 {{
  color: #60a5fa;
  margin-top: 1.25rem;
  margin-bottom: 0.75rem;
  font-size: 1.125rem;
  font-weight: 600;
}}

.embedded-snippet-content h5 {{
  color: #60a5fa;
  margin-top: 1rem;
  margin-bottom: 0.5rem;
  font-size: 1rem;
  font-weight: 600;
}}

.embedded-snippet-content h6 {{
  color: #60a5fa;
  margin-top: 1rem;
  margin-bottom: 0.5rem;
  font-size: 0.875rem;
  font-weight: 600;
}}

.embedded-snippet-content p {{
  line-height: 1.75;
  margin-bottom: 1rem;
}}

.embedded-snippet-content pre {{
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  border-radius: 0.375rem;
  padding: 1rem;
  overflow-x: auto;
  margin: 1rem 0;
}}

.embedded-snippet-content code {{
  background: #f1f5f9;
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-size: 0.875rem;
}}

.embedded-snippet-content pre code {{
  background: none;
  padding: 0;
  border-radius: 0;
}}

.embedded-snippet-content > *:first-child {{
  margin-top: 0;
}}

.embedded-snippet-content > *:last-child {{
  margin-bottom: 0;
}}
"#,
            self.config.content_padding,
            self.config.content_background
        )
    }

/// Generate footer styles
    fn generate_footer_styles(&self) -> String {
        format!(
            r#"
.embedded-snippet-footer {{
  padding: 0.75rem 1.25rem;
  background: {};
  border-top: 1px solid {};
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.875rem;
  color: #1e40af;
}}

.embedded-snippet-date {{
  display: flex;
  align-items: center;
  gap: 0.375rem;
}}

.embedded-snippet-date::before {{
  content: '📅';
}}

.embedded-snippet-actions {{
  display: flex;
  gap: 0.5rem;
}}

.embedded-snippet-toggle {{
  background: #3b82f6;
  color: white;
  border: none;
  padding: 0.375rem 0.75rem;
  border-radius: 0.375rem;
  font-size: 0.8125rem;
  cursor: pointer;
  transition: background 0.2s;
}}

.embedded-snippet-toggle:hover {{
  background: #2563eb;
}}

.embedded-snippet-link {{
  color: #3b82f6;
  text-decoration: none;
  font-weight: 500;
}}

.embedded-snippet-link:hover {{
  text-decoration: underline;
}}
"#,
            self.config.footer_background,
            self.config.footer_border
        )
    }

/// Generate tag styles
    fn generate_tag_styles(&self) -> String {
        format!(
            r#"
.embedded-snippet-tags {{
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}}

.embedded-snippet-tag {{
  background: #dbeafe;
  color: {};
  padding: 0.25rem 0.625rem;
  border-radius: 0.375rem;
  font-size: 0.75rem;
  font-weight: 500;
  white-space: nowrap;
}}

.embedded-snippet-tag:hover {{
  background: #bfdbfe;
}}
"#,
            self.config.tag_color
        )
    }

    /// Generate error styles
    fn generate_error_styles(&self) -> String {
        r#"
.embedded-snippet-card.error {
  background: #fef2f2;
  border-left-color: #ef4444;
}

.embedded-snippet-card.error::before {
  background: linear-gradient(90deg, #ef4444, #f87171);
}

.embedded-snippet-card.error .embedded-snippet-header {
  background: #fee2e2;
  border-bottom-color: #fecaca;
}

.embedded-snippet-card.error .embedded-snippet-title {
  color: #991b1b;
}

.embedded-snippet-card.error .embedded-snippet-title::before {
  content: '⚠️';
}

.embedded-snippet-card.error .embedded-snippet-footer {
  background: #fee2e2;
  border-top-color: #fecaca;
}

.embedded-snippet-card.error .embedded-snippet-link {
  color: #dc2626;
}
"#
        .to_string()
    }

    /// Generate responsive styles
    fn generate_responsive_styles(&self) -> String {
        r#"
@media (max-width: 768px) {
  .embedded-snippet-card {
    margin: 1.5rem 0;
    border-radius: 0.5rem;
  }

  .embedded-snippet-header {
    padding: 0.75rem 1rem;
  }

  .embedded-snippet-title {
    font-size: 1rem;
  }

  .embedded-snippet-content {
    padding: 1rem;
  }

  .embedded-snippet-content h3 {
    font-size: 1.125rem;
  }

  .embedded-snippet-content h4 {
    font-size: 1rem;
  }

  .embedded-snippet-content h5 {
    font-size: 0.875rem;
  }

  .embedded-snippet-footer {
    padding: 0.75rem 1rem;
  }
}

@media (max-width: 480px) {
  .embedded-snippet-card {
    margin: 1rem 0;
    border-radius: 0.375rem;
  }

  .embedded-snippet-header {
    padding: 0.5rem 0.75rem;
  }

  .embedded-snippet-title {
    font-size: 0.875rem;
  }

  .embedded-snippet-content {
    padding: 0.75rem;
  }

  .embedded-snippet-footer {
    padding: 0.5rem 0.75rem;
  }
}
"#
        .to_string()
    }

    /// Set configuration
    pub fn set_config(&mut self, config: EmbeddedSnippetCardConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn config(&self) -> &EmbeddedSnippetCardConfig {
        &self.config
    }
}

impl Default for EmbeddedSnippetCardCssGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create EmbeddedSnippetCardCssGenerator")
    }
}
