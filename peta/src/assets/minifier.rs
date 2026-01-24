//! Asset minification

use crate::core::Result;

/// Asset minifier
pub struct Minifier;

impl Minifier {
    /// Create a new minifier
    pub fn new() -> Self {
        Self
    }
    
    /// Minify HTML
    pub fn minify_html(&self, html: &str) -> Result<String> {
        // Simple HTML minification
        let mut result = html.to_string();
        
        // Remove comments
        let comment_regex = regex::Regex::new(r"<!--.*?-->").unwrap();
        result = comment_regex.replace_all(&result, "").to_string();
        
        // Remove whitespace between tags
        let tag_whitespace_regex = regex::Regex::new(r">\s+<").unwrap();
        result = tag_whitespace_regex.replace_all(&result, "><").to_string();
        
        // Remove leading/trailing whitespace
        result = result.trim().to_string();
        
        Ok(result)
    }
    
    /// Minify CSS
    pub fn minify_css(&self, css: &str) -> Result<String> {
        let processor = crate::assets::css::CssProcessor::new(true);
        processor.process(css)
    }
    
    /// Minify JavaScript
    pub fn minify_js(&self, js: &str) -> Result<String> {
        let processor = crate::assets::js::JsProcessor::new(true);
        processor.process(js)
    }
}

impl Default for Minifier {
    fn default() -> Self {
        Self::new()
    }
}