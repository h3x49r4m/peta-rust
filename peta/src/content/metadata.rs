//! Metadata extraction and processing

use crate::content::ContentMetadata;
use crate::content::ContentType;
use crate::core::Result;
use chrono::{DateTime, Utc};
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
        // For book chapters without frontmatter, use filename as title
        let title = if frontmatter.get("title").is_none() && file_path.is_some() {
            file_path
                .and_then(|p| p.file_stem())
                .and_then(|s| s.to_str())
                .map(|s| s.replace('-', " ").replace('_', " ")) // Convert kebab-case to title case
                .unwrap_or_else(|| "Untitled".to_string())
        } else {
            frontmatter.get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("Untitled")
                .to_string()
        };
        
        let content_type = frontmatter.get("type")
            .and_then(|v| v.as_str())
            .map(|s| ContentType::from_string(s))
            .unwrap_or(ContentType::Article);
        
        let date_str = frontmatter.get("date")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        // Parse datetime from date string
        let date_time = Self::parse_datetime(&date_str);
        
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
            .map(|s| s.to_string())
            .or_else(|| frontmatter.get("description")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()));
        
        // Generate URL from title, considering file path for book chapters
        let url = Self::generate_url_with_path(&title, &content_type, file_path);
        
        // Generate ID from title
        let id = Self::generate_id(&title);
        
        Ok(ContentMetadata {
            id,
            title,
            content_type,
            date: date_str,
            date_time,
            tags,
            author,
            excerpt,
            url,
            extra: HashMap::new(),
        })
    }
    
    /// Parse datetime from date string
    /// Supports formats: "YYYY-MM-DD" and "YYYY-MM-DDTHH:MM:SS"
    fn parse_datetime(date_str: &str) -> Option<DateTime<Utc>> {
        if date_str.is_empty() {
            return None;
        }
        
        // Try ISO 8601 format with time (YYYY-MM-DDTHH:MM:SS)
        if let Ok(dt) = DateTime::parse_from_rfc3339(&format!("{}Z", date_str)) {
            return Some(dt.with_timezone(&Utc));
        }
        
        // Try simple date format (YYYY-MM-DD) and append midnight time
        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            let naive_datetime = naive_date.and_hms_opt(0, 0, 0)?;
            return Some(DateTime::from_naive_utc_and_offset(naive_datetime, Utc));
        }
        
        None
    }
    
    /// Generate URL from title and content type
    #[allow(dead_code)]
    fn generate_url(title: &str, content_type: &ContentType) -> String {
        Self::generate_url_with_path(title, content_type, None)
    }
    
    /// Generate URL from title, content type, and optional file path
    fn generate_url_with_path(title: &str, content_type: &ContentType, file_path: Option<&std::path::Path>) -> String {
        // For books, check if this is a chapter file
        if *content_type == ContentType::Book {
            if let Some(path) = file_path {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    // Check if this is an index.rst file (could be book index or chapter index)
                    if file_name == "index.rst" {
                        // Determine if this is a book index or chapter index
                        if let Some(parent) = path.parent() {
                            // Check if parent is a book directory (direct child of books/)
                            if let Some(grandparent) = parent.parent() {
                                if let Some(grandparent_name) = grandparent.file_name().and_then(|n| n.to_str()) {
                                    if grandparent_name == "books" {
                                        // This is a book index: books/{book}/index.html
                                        let book_slug = Self::slugify(
                                            parent.file_name()
                                                .and_then(|n| n.to_str())
                                                .unwrap_or("book")
                                        );
                                        return format!("books/{}/index.html", book_slug);
                                    } else {
                                        // This is a chapter index in a folder: books/{book}/{chapter}/index.html
                                        let book_slug = Self::slugify(grandparent_name);
                                        let chapter_slug = Self::slugify(
                                            parent.file_name()
                                                .and_then(|n| n.to_str())
                                                .unwrap_or("chapter")
                                        );
                                        return format!("books/{}/{}/index.html", book_slug, chapter_slug);
                                    }
                                }
                            }
                        }
                    } else {
                        // This is a chapter file in flat structure: books/{book}/{chapter}.html
                        let fallback_slug = Self::slugify(title);
                        let chapter_slug = path.file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or(fallback_slug.as_str())
                            .to_string();
                        
                        if let Some(parent) = path.parent() {
                            if let Some(book_dir_name) = parent.file_name().and_then(|n| n.to_str()) {
                                let book_slug = Self::slugify(book_dir_name);
                                return format!("books/{}/{}.html", book_slug, chapter_slug);
                            }
                        }
                    }
                }
            }
        }
        
        // Default URL generation for index pages and other content types
        match content_type {
            ContentType::Article => {
                let slug = Self::slugify(title);
                format!("articles/{}.html", slug)
            }
            ContentType::Book => {
                // Fallback to title-based slug
                let slug = Self::slugify(title);
                format!("books/{}/index.html", slug)
            }
            ContentType::Snippet => {
                let slug = Self::slugify(title);
                format!("snippets/{}.html", slug)
            }
            ContentType::Project => {
                let slug = Self::slugify(title);
                format!("projects/{}.html", slug)
            }
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
