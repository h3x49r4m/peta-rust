//! Math processing for RST content with KaTeX integration

use crate::core::{Error, Result};
use regex::Regex;
use std::collections::HashMap;

/// Math block extracted from RST content
#[derive(Debug, Clone)]
pub struct MathBlock {
    /// Original content
    pub original: String,
    /// Math formula content
    pub formula: String,
    /// Whether it's display math
    pub display: bool,
    /// Start position in original text
    pub start_pos: usize,
    /// End position in original text
    pub end_pos: usize,
}

/// Math processor for handling LaTeX formulas in RST content
pub struct MathProcessor {
    /// KaTeX renderer
    katex_renderer: KatexRenderer,
    /// Regex patterns for math detection
    display_math_regex: Regex,
    inline_math_regex: Regex,
    /// Cached rendered formulas
    render_cache: HashMap<String, String>,
    /// Configuration
    config: MathConfig,
}

/// Configuration for math processing
#[derive(Debug, Clone)]
pub struct MathConfig {
    /// Enable math rendering
    pub enabled: bool,
    /// Use KaTeX for rendering
    pub use_katex: bool,
    /// Fallback to MathJax
    pub fallback_mathjax: bool,
    /// Cache rendered formulas
    pub cache_rendered: bool,
    /// Display math delimiters
    pub display_delimiters: (String, String),
    /// Inline math delimiters
    pub inline_delimiters: (String, String),
}

impl Default for MathConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            use_katex: true,
            fallback_mathjax: true,
            cache_rendered: true,
            display_delimiters: ("$$".to_string(), "$$".to_string()),
            inline_delimiters: ("$".to_string(), "$".to_string()),
        }
    }
}

impl MathProcessor {
    /// Create a new math processor
    pub fn new() -> Result<Self> {
        Self::with_config(MathConfig::default())
    }
    
    /// Create a math processor with custom configuration
    pub fn with_config(config: MathConfig) -> Result<Self> {
        let display_math_regex = Regex::new(r"\$\$([^$]*(?:\$\$[^$]*\$\$)*[^$]*)\$\$")
            .map_err(|e| Error::math(format!("Invalid display math regex: {}", e)))?;
        
        let inline_math_regex = Regex::new(r"\$([^$\n]*(?:\$\$[^$\n]*\$\$)*[^$\n]*)\$")
            .map_err(|e| Error::math(format!("Invalid inline math regex: {}", e)))?;
        
        Ok(Self {
            katex_renderer: KatexRenderer::new()?,
            display_math_regex,
            inline_math_regex,
            render_cache: HashMap::new(),
            config,
        })
    }
    
    /// Process RST content with math rendering
    pub fn process_rst_with_math(&self, content: &str) -> Result<String> {
        if !self.config.enabled {
            return Ok(content.to_string());
        }
        
        // 1. Extract math blocks
        let math_blocks = self.extract_math_blocks(content)?;
        
        // 2. Process RST structure (handled by RST parser)
        let processed_content = content.to_string();
        
        // 3. Reinsert rendered math formulas
        let final_content = self.reinsert_rendered_math(processed_content, math_blocks)?;
        
        Ok(final_content)
    }
    
    /// Extract math blocks from content
    fn extract_math_blocks(&self, content: &str) -> Result<Vec<MathBlock>> {
        let mut math_blocks = Vec::new();
        
        // Extract display math blocks
        for caps in self.display_math_regex.captures_iter(content) {
            let full_match = caps.get(0).unwrap();
            let formula_match = caps.get(1).unwrap();
            
            math_blocks.push(MathBlock {
                original: full_match.as_str().to_string(),
                formula: self.clean_formula(formula_match.as_str()),
                display: true,
                start_pos: full_match.start(),
                end_pos: full_match.end(),
            });
        }
        
        // Extract inline math blocks (excluding those already captured as display math)
        let display_positions: HashSet<usize> = math_blocks
            .iter()
            .flat_map(|block| block.start_pos..=block.end_pos)
            .collect();
        
        for caps in self.inline_math_regex.captures_iter(content) {
            let full_match = caps.get(0).unwrap();
            
            // Skip if this is part of a display math block
            if display_positions.contains(&full_match.start()) {
                continue;
            }
            
            let formula_match = caps.get(1).unwrap();
            
            // Skip if this looks like it should be display math
            let formula = formula_match.as_str();
            if self.should_be_display_math(formula) {
                continue;
            }
            
            math_blocks.push(MathBlock {
                original: full_match.as_str().to_string(),
                formula: self.clean_formula(formula),
                display: false,
                start_pos: full_match.start(),
                end_pos: full_match.end(),
            });
        }
        
        Ok(math_blocks)
    }
    
    /// Clean formula by removing extra delimiters and fixing common issues
    fn clean_formula(&self, formula: &str) -> String {
        let mut cleaned = formula.trim().to_string();
        
        // Remove extra delimiters
        cleaned = cleaned.replace("$$", "");
        cleaned = cleaned.replace("$", "");
        
        // Fix common LaTeX syntax issues
        cleaned = regex::Regex::new(r"\\frac\{([^}]*)\}\{([^}]*)\}")
            .unwrap()
            .replace_all(&cleaned, r"\frac{$1}{$2}")
            .to_string();
        
        // Fix exponents
        cleaned = regex::Regex::new(r"\^([^\s\}]+)")
            .unwrap()
            .replace_all(&cleaned, "^{$1}")
            .to_string();
        
        // Fix subscripts
        cleaned = regex::Regex::new(r"_\{([^}]*)\}")
            .unwrap()
            .replace_all(&cleaned, "_{$1}")
            .to_string();
        
        cleaned.trim().to_string()
    }
    
    /// Check if formula should be display math
    fn should_be_display_math(&self, formula: &str) -> bool {
        // Contains display-style operators
        let display_indicators = [
            r"\int_", r"\sum_", r"\prod_", r"\lim_", r"\begin{", r"\end{",
            r"\\[", r"\\]", r"\begin{align}", r"\end{align}",
            r"\begin{equation}", r"\end{equation}",
        ];
        
        for indicator in &display_indicators {
            if formula.contains(indicator) {
                return true;
            }
        }
        
        // Contains multiple lines
        if formula.contains('\n') {
            return true;
        }
        
        // Contains fractions with complex numerators/denominators
        if regex::Regex::new(r"\\frac\{[^}]*\\[a-zA-Z]+\}")
            .unwrap()
            .is_match(formula)
        {
            return true;
        }
        
        false
    }
    
    /// Reinsert rendered math formulas into content
    fn reinsert_rendered_math(&self, mut content: String, math_blocks: Vec<MathBlock>) -> Result<String> {
        // Sort blocks by position in reverse order to maintain positions
        let mut sorted_blocks = math_blocks;
        sorted_blocks.sort_by_key(|b| std::cmp::Reverse(b.start_pos));
        
        for block in sorted_blocks {
            let rendered = self.render_math_block(&block)?;
            content.replace_range(block.start_pos..block.end_pos, &rendered);
        }
        
        Ok(content)
    }
    
    /// Render a single math block
    fn render_math_block(&self, block: &MathBlock) -> Result<String> {
        // Check cache first
        if self.config.cache_rendered {
            if let Some(cached) = self.render_cache.get(&block.formula) {
                return Ok(cached.clone());
            }
        }
        
        let rendered = if self.config.use_katex {
            self.katex_renderer.render(&block.formula, block.display)?
        } else {
            // Fallback to MathJax or plain text
            self.render_fallback(&block.formula, block.display)?
        };
        
        // Cache the result
        if self.config.cache_rendered {
            self.render_cache.insert(block.formula.clone(), rendered.clone());
        }
        
        Ok(rendered)
    }
    
    /// Render math with fallback method
    fn render_fallback(&self, formula: &str, display: bool) -> Result<String> {
        if self.config.fallback_mathjax {
            // Wrap in MathJax delimiters
            let delimiters = if display {
                ("\\[".to_string(), "\\]".to_string())
            } else {
                ("\\(".to_string(), "\\)".to_string())
            };
            
            Ok(format!("{}{}{}", delimiters.0, formula, delimiters.1))
        } else {
            // Plain text fallback
            Ok(format!("<span class=\"math-fallback\">{}</span>", formula))
        }
    }
    
    /// Clear render cache
    pub fn clear_cache(&mut self) {
        self.render_cache.clear();
    }
    
    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.render_cache.len(), self.render_cache.capacity())
    }
}

/// KaTeX renderer for math formulas
pub struct KatexRenderer {
    /// Whether KaTeX is available
    available: bool,
}

impl KatexRenderer {
    /// Create a new KaTeX renderer
    pub fn new() -> Result<Self> {
        // In a real implementation, this would check if KaTeX is available
        // For now, we'll assume it's always available for the static site generator
        Ok(Self { available: true })
    }
    
    /// Render a math formula using KaTeX
    pub fn render(&self, formula: &str, display: bool) -> Result<String> {
        if !self.available {
            return Err(Error::math("KaTeX renderer not available".to_string()));
        }
        
        // In a real implementation, this would call the KaTeX rendering engine
        // For now, we'll generate appropriate HTML structure
        let tag = if display { "div" } else { "span" };
        let class = if display { "math-display" } else { "math-inline" };
        
        Ok(format!(
            "<{} class=\"katex-eq {}\" data-formula=\"{}\">{}</{}>",
            tag, class, formula, formula, tag
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_math_extraction() {
        let processor = MathProcessor::new().unwrap();
        let content = "The formula $E = mc^2$ is famous. And $$\\int_0^\\infty e^{-x} dx = 1$$ is an integral.";
        
        let blocks = processor.extract_math_blocks(content).unwrap();
        assert_eq!(blocks.len(), 2);
        
        assert_eq!(blocks[0].formula, "E = mc^2");
        assert!(!blocks[0].display);
        
        assert_eq!(blocks[1].formula, "\\int_0^\\infty e^{-x} dx = 1");
        assert!(blocks[1].display);
    }
    
    #[test]
    fn test_formula_cleaning() {
        let processor = MathProcessor::new().unwrap();
        
        assert_eq!(processor.clean_formula("$$E = mc^2$$"), "E = mc^2");
        assert_eq!(processor.clean_formula("$x^2$"), "x^2");
        assert_eq!(processor.clean_formula("x^2"), "x^2");
    }
    
    #[test]
    fn test_display_math_detection() {
        let processor = MathProcessor::new().unwrap();
        
        assert!(processor.should_be_display_math("\\int_0^1 f(x) dx"));
        assert!(processor.should_be_display_math("\\sum_{i=1}^n i"));
        assert!(!processor.should_be_display_math("x^2 + y^2"));
    }
}
