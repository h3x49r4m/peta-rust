//! JavaScript processing

use crate::core::Result;

/// JavaScript processor
pub struct JsProcessor {
    minify: bool,
}

impl JsProcessor {
    /// Create a new JavaScript processor
    pub fn new(minify: bool) -> Self {
        Self { minify }
    }
    
    /// Process JavaScript content
    pub fn process(&self, js: &str) -> Result<String> {
        if self.minify {
            self.minify_js(js)
        } else {
            Ok(js.to_string())
        }
    }
    
    /// Minify JavaScript
    fn minify_js(&self, js: &str) -> Result<String> {
        // Simple JavaScript minification
        let mut result = js.to_string();
        
        // Remove single-line comments
        let single_line_regex = regex::Regex::new(r"//.*").unwrap();
        result = single_line_regex.replace_all(&result, "").to_string();
        
        // Remove multi-line comments
        let multi_line_regex = regex::Regex::new(r"/\*.*?\*/").unwrap();
        result = multi_line_regex.replace_all(&result, "").to_string();
        
        // Remove unnecessary whitespace
        let whitespace_regex = regex::Regex::new(r"\s+").unwrap();
        result = whitespace_regex.replace_all(&result, " ").to_string();
        
        // Remove spaces around operators
        result = result.replace(" = ", "=").replace(" =", "=");
        result = result.replace(" + ", "+").replace(" +", "+");
        result = result.replace(" - ", "-").replace(" -", "-");
        result = result.replace(" * ", "*").replace(" *", "*");
        result = result.replace(" / ", "/").replace(" /", "/");
        
        Ok(result.trim().to_string())
    }
}