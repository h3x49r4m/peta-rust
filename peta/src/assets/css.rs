//! CSS processing

use crate::core::Result;

/// CSS processor
pub struct CssProcessor {
    minify: bool,
}

impl CssProcessor {
    /// Create a new CSS processor
    pub fn new(minify: bool) -> Self {
        Self { minify }
    }
    
    /// Process CSS content
    pub fn process(&self, css: &str) -> Result<String> {
        if self.minify {
            self.minify_css(css)
        } else {
            Ok(css.to_string())
        }
    }
    
    /// Minify CSS
    fn minify_css(&self, css: &str) -> Result<String> {
        // Simple CSS minification
        let mut result = css.to_string();
        
        // Remove comments
        let comment_regex = regex::Regex::new(r"/\*.*?\*/").unwrap();
        result = comment_regex.replace_all(&result, "").to_string();
        
        // Remove whitespace
        let whitespace_regex = regex::Regex::new(r"\s+").unwrap();
        result = whitespace_regex.replace_all(&result, " ").to_string();
        
        // Remove unnecessary spaces
        result = result.replace(" {", "{").replace("{ ", "{");
        result = result.replace(" }", "}").replace(" }", "}");
        result = result.replace("; ", ";").replace(" ;", ";");
        result = result.replace(": ", ":").replace(" :", ":");
        result = result.replace(", ", ",").replace(" ,", ",");
        
        Ok(result.trim().to_string())
    }
}