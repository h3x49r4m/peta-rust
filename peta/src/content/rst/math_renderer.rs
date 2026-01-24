//! Math rendering using KaTeX and MathJax

use crate::core::Result;


/// Math renderer for LaTeX equations
pub struct MathRenderer {
    #[allow(dead_code)]
    katex_delimiters: Vec<String>,
    #[allow(dead_code)]
    fallback_mathjax: bool,
    cache: std::collections::HashMap<String, String>,
}

impl MathRenderer {
    /// Create a new math renderer
    pub fn new() -> Self {
        Self {
            katex_delimiters: vec![
                "$$".to_string(),
                "$".to_string(),
                "\\[".to_string(),
                "\\]".to_string(),
            ],
            fallback_mathjax: true,
            cache: std::collections::HashMap::new(),
        }
    }
    
    /// Render math equations in HTML content
    pub fn render(&mut self, content: &str) -> Result<String> {
        let mut result = content.to_string();
        
        // Render display math ($$...$$ or \[...\])
        result = self.render_display_math(&result)?;
        
        // Render inline math ($...$ or \(...\))
        result = self.render_inline_math(&result)?;
        
        Ok(result)
    }
    
    /// Render display math equations
    fn render_display_math(&mut self, content: &str) -> Result<String> {
        // Pre-compile regex patterns once
        static DISPLAY_REGEX: once_cell::sync::Lazy<regex::Regex> = 
            once_cell::sync::Lazy::new(|| regex::Regex::new(r"\$\$([^$]+)\$\$").unwrap());
        static LATEX_REGEX: once_cell::sync::Lazy<regex::Regex> = 
            once_cell::sync::Lazy::new(|| regex::Regex::new(r"\\\[(.*?)\\\]").unwrap());
        
        let mut result = content.to_string();
        
        // Handle $$...$$ delimiters
        result = DISPLAY_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let equation = caps.get(1).unwrap().as_str();
            self.render_equation(equation, true).unwrap_or_else(|_| format!("<span class=\"math-error\">{}</span>", equation))
        }).to_string();
        
        // Handle \[...\] delimiters
        result = LATEX_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let equation = caps.get(1).unwrap().as_str();
            self.render_equation(equation, true).unwrap_or_else(|_| format!("<span class=\"math-error\">{}</span>", equation))
        }).to_string();
        
        Ok(result)
    }
    
    /// Render inline math equations
    fn render_inline_math(&mut self, content: &str) -> Result<String> {
        // Pre-compile regex patterns once
        static INLINE_REGEX: once_cell::sync::Lazy<regex::Regex> = 
            once_cell::sync::Lazy::new(|| regex::Regex::new(r"\$([^$]+)\$").unwrap());
        static LATEX_INLINE_REGEX: once_cell::sync::Lazy<regex::Regex> = 
            once_cell::sync::Lazy::new(|| regex::Regex::new(r"\\\((.*?)\\\)").unwrap());
        
        let mut result = content.to_string();
        
        // Handle $...$ delimiters (but avoid $$...$$ which are already handled)
        result = INLINE_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let equation = caps.get(1).unwrap().as_str();
            self.render_equation(equation, false).unwrap_or_else(|_| format!("<span class=\"math-error\">{}</span>", equation))
        }).to_string();
        
        // Handle \(...\) delimiters
        result = LATEX_INLINE_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let equation = caps.get(1).unwrap().as_str();
            self.render_equation(equation, false).unwrap_or_else(|_| format!("<span class=\"math-error\">{}</span>", equation))
        }).to_string();
        
        Ok(result)
    }
    
    /// Render a single equation
    fn render_equation(&mut self, equation: &str, display: bool) -> Result<String> {
        // Check cache first
        let cache_key = format!("{}:{}", equation, display);
        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(cached.clone());
        }
        
        // For now, create KaTeX-compatible HTML
        // In a real implementation, you would use the KaTeX library
        let rendered = if display {
            format!(
                r#"<div class="math-display" data-latex="{}">
    <span class="katex-display">{}</span>
</div>"#,
                equation, equation
            )
        } else {
            format!(
                r#"<span class="math-inline" data-latex="{}">
    <span class="katex">{}</span>
</span>"#,
                equation, equation
            )
        };
        
        // Cache the result
        self.cache.insert(cache_key, rendered.clone());
        
        Ok(rendered)
    }
}

impl Default for MathRenderer {
    fn default() -> Self {
        Self::new()
    }
}