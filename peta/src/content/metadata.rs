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
        Self::extract_with_path(frontmatter, None)
    }
    
    /// Extract metadata from frontmatter with optional file path for book chapters
    pub fn extract_with_path(frontmatter: &HashMap<String, serde_json::Value>, file_path: Option<&std::path::Path>) -> Result<ContentMetadata> {
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
        
        // Generate URL from title, considering file path for book chapters
        let url = Self::generate_url_with_path(&title, &content_type, file_path);
        
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
        Self::generate_url_with_path(title, content_type, None)
    }
    
    /// Generate URL from title, content type, and optional file path
    fn generate_url_with_path(title: &str, content_type: &ContentType, file_path: Option<&std::path::Path>) -> String {
        let slug = Self::slugify(title);
        
        // For books, check if this is a chapter file
        if *content_type == ContentType::Book {
            if let Some(path) = file_path {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name != "index.rst" {
                        // This is a chapter file
                        if let Some(parent) = path.parent() {
                            if let Some(book_dir_name) = parent.file_name().and_then(|n| n.to_str()) {
                                let book_slug = Self::slugify(book_dir_name);
                                // Generate URL: books/{book-name}/{chapter-name}.html
                                return format!("books/{}/{}.html", book_slug, slug);
                            }
                        }
                    }
                }
            }
        }
        
        // Default URL generation
        match content_type {
            ContentType::Article => format!("articles/{}.html", slug),
            ContentType::Book => format!("books/{}/index.html", slug),
            ContentType::Snippet => format!("snippets/{}.html", slug),
            ContentType::Project => format!("projects/{}.html", slug),
        }
    }
    
    /// Generate URL for book chapters
    pub fn generate_chapter_url(book_slug: &str, chapter_slug: &str) -> String {
        format!("books/{}/{}.html", book_slug, chapter_slug)
    }
    
    /// Generate ID from title
    fn generate_id(title: &str) -> String {
        Self::slugify(title)
    }
    
    /// Convert title to URL-friendly slug
    fn slugify(title: &str) -> String {
        let mut result = title.to_lowercase();
        
        // Handle common programming language notations first
        result = result.replace("c++", "cpp");
        result = result.replace("c#", "csharp");
        result = result.replace("f#", "fsharp");
        result = result.replace("c++/cli", "cpp-cli");
        result = result.replace(".net", "dotnet");
        result = result.replace("node.js", "nodejs");
        result = result.replace("node.js", "nodejs");
        result = result.replace("react.js", "reactjs");
        result = result.replace("vue.js", "vuejs");
        result = result.replace("angular.js", "angularjs");
        
        // Replace common symbols with words
        result = result.replace("++", "plus");
        result = result.replace("--", "minus");
        result = result.replace("==", "equals");
        result = result.replace("!=", "not-equals");
        result = result.replace("<=", "less-equal");
        result = result.replace(">=", "greater-equal");
        result = result.replace("->", "arrow");
        result = result.replace("=>", "fat-arrow");
        result = result.replace("&&", "and");
        result = result.replace("||", "or");
        
        // Replace spaces and punctuation with dashes
        result = result.replace(&[' ', '-', '_', '.', ',', ';', ':', '!', '?', '@', '#', '$', '%', '^', '&', '*', '(', ')', '=', '[', ']', '{', '}', '\\', '|', '<', '>', '/', '"', '\''][..], "-");
        
        // Remove quotes completely
        result = result.replace(['"', '\''], "");
        
        // Filter to only keep alphanumeric characters and dashes
        result = result.chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();
        
        // Collapse multiple dashes into single dashes
        while result.contains("--") {
            result = result.replace("--", "-");
        }
        
        // Trim leading/trailing dashes
        result.trim_matches('-').to_string()
    }
}

impl Default for MetadataExtractor {
    fn default() -> Self {
        Self
    }
}