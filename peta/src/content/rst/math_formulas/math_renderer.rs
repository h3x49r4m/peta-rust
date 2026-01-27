//! Math rendering using KaTeX

use crate::core::Result;

/// Math renderer for LaTeX equations
pub struct MathRenderer {
    cache: std::collections::HashMap<String, String>,
}

impl MathRenderer {
    /// Create a new math renderer
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }

    /// Check if content needs math rendering
    pub fn should_render(&self, content: &str) -> bool {
        content.contains("$") ||
        content.contains("\\[") ||
        content.contains("\\(") ||
        content.contains("data-latex")
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
        // Use non-greedy matching to correctly capture content between delimiters
        static DISPLAY_REGEX: once_cell::sync::Lazy<regex::Regex> = 
            once_cell::sync::Lazy::new(|| regex::Regex::new(r"\$\$(.*?)\$\$").unwrap());
        static LATEX_REGEX: once_cell::sync::Lazy<regex::Regex> = 
            once_cell::sync::Lazy::new(|| regex::Regex::new(r"\\\[(.*?)\\\]").unwrap());
        
        let mut result = content.to_string();
        
        // Handle $$...$$ delimiters
        result = DISPLAY_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let equation = caps.get(1).unwrap().as_str().trim();
            self.render_equation(equation, true).unwrap_or_else(|_| format!("<span class=\"math-error\">{}</span>", equation))
        }).to_string();
        
        // Handle \[...\] delimiters
        result = LATEX_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let equation = caps.get(1).unwrap().as_str().trim();
            self.render_equation(equation, true).unwrap_or_else(|_| format!("<span class=\"math-error\">{}</span>", equation))
        }).to_string();
        
        Ok(result)
    }
    
    /// Render inline math equations
    fn render_inline_math(&mut self, content: &str) -> Result<String> {
        // Pre-compile regex patterns once
        static LATEX_INLINE_REGEX: once_cell::sync::Lazy<regex::Regex> = 
            once_cell::sync::Lazy::new(|| regex::Regex::new(r"\\\((.*?)\\\)").unwrap());
        
        let mut result = content.to_string();
        
        // Handle \(...\) delimiters first
        result = LATEX_INLINE_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let equation = caps.get(1).unwrap().as_str().trim();
            self.render_equation(equation, false).unwrap_or_else(|_| format!("<span class=\"math-error\">{}</span>", equation))
        }).to_string();
        
        // Handle $...$ delimiters with a custom approach to avoid $$...$$
        // First, we'll mark all display math blocks temporarily
        let display_marker = uuid::Uuid::new_v4().to_string();
        static DISPLAY_TEMP_REGEX: once_cell::sync::Lazy<regex::Regex> = 
            once_cell::sync::Lazy::new(|| regex::Regex::new(r"\$\$(.*?)\$\$").unwrap());
        
        result = DISPLAY_TEMP_REGEX.replace_all(&result, |caps: &regex::Captures| {
            format!("__DISPLAY_MATH_{}__{}", display_marker, caps.get(0).unwrap().as_str())
        }).to_string();
        
        // Now process inline math (display math is temporarily removed)
        static INLINE_REGEX: once_cell::sync::Lazy<regex::Regex> = 
            once_cell::sync::Lazy::new(|| regex::Regex::new(r"\$([^$\n]+?)\$").unwrap());
        
        result = INLINE_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let equation = caps.get(1).unwrap().as_str().trim();
            self.render_equation(equation, false).unwrap_or_else(|_| format!("<span class=\"math-error\">{}</span>", equation))
        }).to_string();
        
        // Restore display math blocks
        static RESTORE_REGEX: once_cell::sync::Lazy<regex::Regex> = 
            once_cell::sync::Lazy::new(|| regex::Regex::new(r"__DISPLAY_MATH_[^_]+__\$\$(.*?)\$\$").unwrap());
        
        result = RESTORE_REGEX.replace_all(&result, "$$$1$$").to_string();
        
        Ok(result)
    }
    
    /// Render a single equation
    fn render_equation(&mut self, equation: &str, display: bool) -> Result<String> {
        // Check cache first
        let cache_key = format!("{}:{}", equation, display);
        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(cached.clone());
        }
        
        // Create HTML elements with data-latex attributes for on-demand rendering
        let rendered = if display {
            format!(
                r#"<div class="math-display" data-latex="{}"></div>"#,
                equation
            )
        } else {
            format!(
                r#"<span class="math-inline" data-latex="{}"></span>"#,
                equation
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