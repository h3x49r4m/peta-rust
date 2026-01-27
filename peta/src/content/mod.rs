//! Content processing module

pub mod rst;
pub mod metadata;
pub mod resolver;
pub mod taxonomy;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Processed article with math detection
#[derive(Debug, Clone)]
pub struct ProcessedArticle {
    pub content: String,
    pub metadata: ContentMetadata,
    pub has_math_formulas: bool,
    pub math_formula_count: usize,
    pub toc: Option<String>,
}

/// Content metadata extracted from frontmatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub id: String,
    pub title: String,
    pub content_type: ContentType,
    pub date: String,
    pub tags: Vec<String>,
    pub author: Option<String>,
    pub excerpt: Option<String>,
    pub url: String,
    #[serde(default)]
    pub extra: HashMap<String, String>,
}

/// Content types supported by the site generator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Article,
    Book,
    Snippet,
    Project,
}

impl ContentType {
    /// Convert string to ContentType
    pub fn from_string(s: &str) -> Self {
        match s {
            "article" => ContentType::Article,
            "book" => ContentType::Book,
            "snippet" => ContentType::Snippet,
            "project" => ContentType::Project,
            _ => ContentType::Article,
        }
    }
    
    /// Convert ContentType to string
    pub fn to_string(&self) -> String {
        match self {
            ContentType::Article => "article".to_string(),
            ContentType::Book => "book".to_string(),
            ContentType::Snippet => "snippet".to_string(),
            ContentType::Project => "project".to_string(),
        }
    }
}

/// Table of contents entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocEntry {
    pub level: usize,
    pub title: String,
    pub anchor: String,
    pub children: Vec<TocEntry>,
}

/// Processed RST content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RstContent {
    pub metadata: ContentMetadata,
    pub html: String,
    pub toc: Vec<TocEntry>,
    pub toc_html: String,
    pub frontmatter: HashMap<String, serde_json::Value>,
    pub has_math_formulas: bool,
    pub math_formula_count: usize,
}

impl RstContent {
    /// Create new RST content
    pub fn new(
        metadata: ContentMetadata,
        html: String,
        toc: Vec<TocEntry>,
        frontmatter: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self {
            metadata,
            html,
            toc,
            toc_html: String::new(),
            frontmatter,
            has_math_formulas: false,
            math_formula_count: 0,
        }
    }

    /// Create new RST content with math detection
    pub fn new_with_math(
        metadata: ContentMetadata,
        html: String,
        toc: Vec<TocEntry>,
        frontmatter: HashMap<String, serde_json::Value>,
        has_math_formulas: bool,
        math_formula_count: usize,
    ) -> Self {
        Self {
            metadata,
            html,
            toc,
            toc_html: String::new(),
            frontmatter,
            has_math_formulas,
            math_formula_count,
        }
    }
    
    /// Get excerpt from content
    pub fn get_excerpt(&self, max_length: usize) -> String {
        if let Some(excerpt) = &self.metadata.excerpt {
            return excerpt.clone();
        }
        
        // Generate excerpt from HTML content
        let text = html_to_text(&self.html);
        if text.len() <= max_length {
            text
        } else {
            format!("{}...", &text[..max_length])
        }
    }
}

/// Convert HTML to plain text for excerpt generation
fn html_to_text(html: &str) -> String {
    // Simple HTML to text conversion
    // In a real implementation, you might use a proper HTML parser
    let mut text = html.to_string();
    
    // Remove HTML tags
    text = regex::Regex::new(r"<[^>]*>")
        .unwrap()
        .replace_all(&text, "")
        .to_string();
    
    // Normalize whitespace
    text = regex::Regex::new(r"\s+")
        .unwrap()
        .replace_all(&text.trim(), " ")
        .to_string();
    
    text
}