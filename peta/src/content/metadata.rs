//! Metadata extraction and processing

use crate::content::ContentMetadata;
use crate::content::ContentType;
use crate::core::Result;
use std::collections::HashMap;

/// Metadata extractor
pub struct MetadataExtractor;

impl MetadataExtractor {
    /// Extract metadata from frontmatter
    pub fn extract(frontmatter: &HashMap<String, serde_json::Value>) -> Result<ContentMetadata> {
        let title = frontmatter.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Untitled")
            .to_string();
        
        let content_type = frontmatter.get("type")
            .and_then(|v| v.as_str())
            .map(|s| ContentType::from_string(s))
            .unwrap_or(ContentType::Article);
        
        let date = frontmatter.get("date")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let tags = frontmatter.get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();
        
        let author = frontmatter.get("author")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let excerpt = frontmatter.get("excerpt")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        // Generate URL from title
        let url = Self::generate_url(&title, &content_type);
        
        // Generate ID from title
        let id = Self::generate_id(&title);
        
        Ok(ContentMetadata {
            id,
            title,
            content_type,
            date,
            tags,
            author,
            excerpt,
            url,
            extra: HashMap::new(),
        })
    }
    
    /// Generate URL from title and content type
    fn generate_url(title: &str, content_type: &ContentType) -> String {
        let slug = Self::slugify(title);
        match content_type {
            ContentType::Article => format!("articles/{}.html", slug),
            ContentType::Book => format!("books/{}/index.html", slug),
            ContentType::Snippet => format!("snippets/{}.html", slug),
            ContentType::Project => format!("projects/{}.html", slug),
        }
    }
    
    /// Generate ID from title
    fn generate_id(title: &str) -> String {
        Self::slugify(title)
    }
    
    /// Convert title to URL-friendly slug
    fn slugify(title: &str) -> String {
        title.to_lowercase()
            .replace(&[' ', '-', '_', '.', ',', ';', ':', '!', '?', '@', '#', '$', '%', '^', '&', '*', '(', ')', '+', '=', '[', ']', '{', '}', '\\', '|', '<', '>', '/', '"', '\''][..], "-")
            .replace(&['"', '\''][..], "")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>()
            .trim_matches('-')
            .to_string()
    }
}

impl Default for MetadataExtractor {
    fn default() -> Self {
        Self
    }
}