//! Enhanced search indexer for building client-side search index

use crate::content::RstContent;
use crate::core::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Search document representing indexed content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocument {
    /// Document ID
    pub id: String,
    /// Document title
    pub title: String,
    /// Document excerpt
    pub excerpt: String,
    /// Document URL
    pub url: String,
    /// Content type
    pub content_type: String,
    /// Tags
    pub tags: Vec<String>,
    /// Publication date
    pub date: String,
    /// Author
    pub author: Option<String>,
    /// Full content for search
    pub content: String,
    /// Word count
    pub word_count: usize,
    /// Reading time in minutes
    pub reading_time: usize,
}

/// Search index for client-side search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndex {
    /// Indexed documents
    pub documents: Vec<SearchDocument>,
    /// Term index for fast lookup
    pub terms: HashMap<String, Vec<usize>>,
    /// Tag index
    pub tags: HashMap<String, Vec<usize>>,
    /// Content type index
    pub content_types: HashMap<String, Vec<usize>>,
    /// Index metadata
    pub metadata: SearchMetadata,
}

/// Search metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMetadata {
    /// Index version
    pub version: String,
    /// Build timestamp
    pub build_timestamp: String,
    /// Total documents
    pub total_documents: usize,
    /// Total terms
    pub total_terms: usize,
    /// Average document length
    pub avg_document_length: f64,
}

/// Search indexer configuration
#[derive(Debug, Clone)]
pub struct IndexerConfig {
    /// Minimum term length to index
    pub min_term_length: usize,
    /// Maximum term length to index
    pub max_term_length: usize,
    /// Stop words to exclude
    pub stop_words: Vec<String>,
    /// Include content in index
    pub index_content: bool,
    /// Include metadata in index
    pub index_metadata: bool,
    /// Stemming enabled
    pub stemming: bool,
    /// Fuzzy search enabled
    pub fuzzy_search: bool,
}

impl Default for IndexerConfig {
    fn default() -> Self {
        Self {
            min_term_length: 2,
            max_term_length: 50,
            stop_words: vec![
                "a".to_string(), "an".to_string(), "and".to_string(), "are".to_string(), "as".to_string(), "at".to_string(), "be".to_string(), "but".to_string(), "by".to_string(), "for".to_string(), "if".to_string(),
                "in".to_string(), "into".to_string(), "is".to_string(), "it".to_string(), "no".to_string(), "not".to_string(), "of".to_string(), "on".to_string(), "or".to_string(), "such".to_string(), "that".to_string(),
                "the".to_string(), "their".to_string(), "then".to_string(), "there".to_string(), "these".to_string(), "they".to_string(), "this".to_string(), "to".to_string(), "was".to_string(),
                "will".to_string(), "with".to_string(), "the".to_string(), "this".to_string(), "that".to_string(), "these".to_string(), "those".to_string(), "is".to_string(), "are".to_string(),
                "was".to_string(), "were".to_string(), "been".to_string(), "being".to_string(), "have".to_string(), "has".to_string(), "had".to_string(), "do".to_string(), "does".to_string(), "did".to_string(),
            ],
            index_content: true,
            index_metadata: true,
            stemming: true,
            fuzzy_search: true,
        }
    }
}

impl SearchIndex {
    /// Create a new search index
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
            terms: HashMap::new(),
            tags: HashMap::new(),
            content_types: HashMap::new(),
            metadata: SearchMetadata {
                version: "1.0.0".to_string(),
                build_timestamp: chrono::Utc::now().to_rfc3339(),
                total_documents: 0,
                total_terms: 0,
                avg_document_length: 0.0,
            },
        }
    }
    
    /// Build search index from RST content
    pub fn build(&mut self, content: &[RstContent]) -> Result<()> {
        let config = IndexerConfig::default();
        self.build_with_config(content, &config)
    }
    
    /// Build search index with custom configuration
    pub fn build_with_config(&mut self, content: &[RstContent], config: &IndexerConfig) -> Result<()> {
        // Clear existing index
        self.documents.clear();
        self.terms.clear();
        self.tags.clear();
        self.content_types.clear();
        
        // Process each content item
        for (idx, item) in content.iter().enumerate() {
            let document = self.create_document(item, config)?;
            self.documents.push(document);
            
            // Index document
            self.index_document(idx, config)?;
        }
        
        // Update metadata
        self.update_metadata();
        
        Ok(())
    }
    
    /// Create search document from RST content
    fn create_document(&self, content: &RstContent, config: &IndexerConfig) -> Result<SearchDocument> {
        // Extract text content from HTML
        let text_content = self.html_to_text(&content.html);
        
        // Calculate word count
        let word_count = text_content.split_whitespace().count();
        
        // Calculate reading time (assuming 200 words per minute)
        let reading_time = (word_count + 199) / 200;
        
        // Get author from frontmatter
        let author = content.frontmatter
            .get("author")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        Ok(SearchDocument {
            id: content.metadata.id.clone(),
            title: content.metadata.title.clone(),
            excerpt: content.metadata.excerpt.clone().unwrap_or_else(|| {
                self.generate_excerpt(&text_content, 150)
            }),
            url: content.metadata.url.clone(),
            content_type: content.metadata.content_type.to_string(),
            tags: content.metadata.tags.clone(),
            date: content.metadata.date.clone(),
            author,
            content: if config.index_content { text_content } else { String::new() },
            word_count,
            reading_time,
        })
    }
    
    /// Index a document
    fn index_document(&mut self, doc_idx: usize, config: &IndexerConfig) -> Result<()> {
        let document = self.documents[doc_idx].clone();
        
        // Index title (always indexed)
        self.index_text(&document.title, doc_idx, config)?;
        
        // Index content if enabled
        if config.index_content {
            self.index_text(&document.content, doc_idx, config)?;
        }
        
        // Index tags
        for tag in &document.tags {
            self.tags
                .entry(tag.to_lowercase())
                .or_insert_with(Vec::new)
                .push(doc_idx);
        }
        
        // Index content type
        self.content_types
            .entry(document.content_type.clone())
            .or_insert_with(Vec::new)
            .push(doc_idx);
        
        Ok(())
    }
    
    /// Index text content
    fn index_text(&mut self, text: &str, doc_idx: usize, config: &IndexerConfig) -> Result<()> {
        // Tokenize text
        let terms = self.tokenize(text, config);
        
        // Index each term
        for term in terms {
            self.terms
                .entry(term.to_lowercase())
                .or_insert_with(Vec::new)
                .push(doc_idx);
        }
        
        Ok(())
    }
    
    /// Tokenize text into terms
    fn tokenize(&self, text: &str, config: &IndexerConfig) -> Vec<String> {
        let mut terms = Vec::new();
        
        // Split on whitespace and punctuation
        let raw_terms: Vec<String> = text
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        
        for term in raw_terms {
            // Filter by length
            if term.len() < config.min_term_length || term.len() > config.max_term_length {
                continue;
            }
            
            // Filter stop words
            if config.stop_words.contains(&term.to_string()) {
                continue;
            }
            
            // Apply stemming if enabled
            let processed_term = if config.stemming {
                self.stem_term(&term)
            } else {
                term.to_string()
            };
            
            terms.push(processed_term);
        }
        
        terms
    }
    
    /// Simple stemming (placeholder implementation)
    fn stem_term(&self, term: &str) -> String {
        // This is a very basic stemming implementation
        // In a real implementation, you'd use a proper stemming library
        let mut stemmed = term.to_string();
        
        // Remove common suffixes
        let suffixes = ["ing", "ed", "er", "est", "ly", "s"];
        for suffix in &suffixes {
            if stemmed.ends_with(suffix) && stemmed.len() > suffix.len() + 2 {
                stemmed.truncate(stemmed.len() - suffix.len());
                break;
            }
        }
        
        stemmed
    }
    
    /// Convert HTML to plain text
    fn html_to_text(&self, html: &str) -> String {
        // Simple HTML to text conversion
        // In a real implementation, you'd use a proper HTML parser
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
    
    /// Generate excerpt from text
    fn generate_excerpt(&self, text: &str, max_length: usize) -> String {
        if text.len() <= max_length {
            return text.to_string();
        }
        
        // Find the last complete word within max_length
        let mut excerpt = text[..max_length].to_string();
        
        // Remove trailing incomplete word
        while let Some(last_char) = excerpt.chars().last() {
            if last_char.is_whitespace() {
                break;
            }
            excerpt.pop();
        }
        
        if !excerpt.is_empty() {
            excerpt.push_str("...");
        }
        
        excerpt
    }
    
    /// Update search metadata
    fn update_metadata(&mut self) {
        self.metadata.total_documents = self.documents.len();
        self.metadata.total_terms = self.terms.len();
        
        if !self.documents.is_empty() {
            let total_length: usize = self.documents
                .iter()
                .map(|d| d.content.len())
                .sum();
            self.metadata.avg_document_length = total_length as f64 / self.documents.len() as f64;
        }
        
        self.metadata.build_timestamp = chrono::Utc::now().to_rfc3339();
    }
    
    /// Generate client-side search data
    pub fn generate_client_search(&self) -> Result<String> {
        let search_data = ClientSearchData {
            documents: self.documents.clone(),
            terms: self.terms.clone(),
            tags: self.tags.clone(),
            content_types: self.content_types.clone(),
            metadata: self.metadata.clone(),
        };
        
        serde_json::to_string(&search_data)
            .map_err(|e| Error::search(format!("Failed to serialize search data: {}", e)))
    }
    
    /// Save search index to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = self.generate_client_search()?;
        fs::write(path, json)
            .map_err(|e| Error::search(format!("Failed to write search index: {}", e)))?;
        Ok(())
    }
    
    /// Load search index from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let json = fs::read_to_string(path)
            .map_err(|e| Error::search(format!("Failed to read search index: {}", e)))?;
        
        serde_json::from_str(&json)
            .map_err(|e| Error::search(format!("Failed to parse search index: {}", e)))
    }
    
    /// Search the index
    pub fn search(&self, query: &str, limit: Option<usize>) -> Vec<&SearchDocument> {
        if query.is_empty() {
            return Vec::new();
        }
        
        let terms = self.tokenize(query, &IndexerConfig::default());
        let mut document_scores: HashMap<usize, f64> = HashMap::new();
        
        // Score documents based on term matches
        for term in terms {
            if let Some(document_indices) = self.terms.get(&term) {
                for &doc_idx in document_indices {
                    let score = document_scores.entry(doc_idx).or_insert(0.0);
                    *score += 1.0; // Simple scoring: each term match adds 1 point
                }
            }
        }
        
        // Sort documents by score
        let mut scored_docs: Vec<(usize, f64)> = document_scores.into_iter().collect();
        scored_docs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Return documents up to limit
        let limit = limit.unwrap_or(10);
        scored_docs
            .into_iter()
            .take(limit)
            .filter_map(|(idx, _)| self.documents.get(idx))
            .collect()
    }
    
    /// Get documents by tag
    pub fn get_by_tag(&self, tag: &str) -> Vec<&SearchDocument> {
        self.tags
            .get(&tag.to_lowercase())
            .map(|indices| {
                indices.iter()
                    .filter_map(|&idx| self.documents.get(idx))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get documents by content type
    pub fn get_by_content_type(&self, content_type: &str) -> Vec<&SearchDocument> {
        self.content_types
            .get(content_type)
            .map(|indices| {
                indices.iter()
                    .filter_map(|&idx| self.documents.get(idx))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get all tags
    pub fn get_all_tags(&self) -> Vec<&String> {
        let mut tags: Vec<&String> = self.tags.keys().collect();
        tags.sort();
        tags
    }
    
    /// Get popular tags (with document counts)
    pub fn get_popular_tags(&self, limit: Option<usize>) -> Vec<(String, usize)> {
        let mut tag_counts: Vec<(String, usize)> = self.tags
            .iter()
            .map(|(tag, indices)| (tag.clone(), indices.len()))
            .collect();
        
        tag_counts.sort_by(|a, b| b.1.cmp(&a.1));
        
        if let Some(limit) = limit {
            tag_counts.truncate(limit);
        }
        
        tag_counts
    }
}

/// Client-side search data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientSearchData {
    pub documents: Vec<SearchDocument>,
    pub terms: HashMap<String, Vec<usize>>,
    pub tags: HashMap<String, Vec<usize>>,
    pub content_types: HashMap<String, Vec<usize>>,
    pub metadata: SearchMetadata,
}

impl Default for SearchIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{ContentMetadata, ContentType};
    
    #[test]
    fn test_search_index_building() {
        let mut index = SearchIndex::new();
        
        let content = vec![
            create_test_content(
                "test-1",
                "Test Article 1",
                "This is a test article about Rust programming",
                "/articles/test-1",
                ContentType::Article,
                vec!["rust".to_string(), "programming".to_string()],
            ),
            create_test_content(
                "test-2",
                "Test Article 2",
                "This is a test article about Python programming",
                "/articles/test-2",
                ContentType::Article,
                vec!["python".to_string(), "programming".to_string()],
            ),
        ];
        
        index.build(&content).unwrap();
        
        assert_eq!(index.documents.len(), 2);
        assert!(index.terms.contains_key("rust"));
        assert!(index.terms.contains_key("python"));
        assert!(index.tags.contains_key("rust"));
    }
    
    #[test]
    fn test_search_functionality() {
        let mut index = SearchIndex::new();
        
        let content = vec![
            create_test_content(
                "test-1",
                "Rust Programming Guide",
                "Learn Rust programming with this comprehensive guide",
                "/articles/rust-guide",
                ContentType::Article,
                vec!["rust".to_string(), "programming".to_string()],
            ),
        ];
        
        index.build(&content).unwrap();
        
        let results = index.search("rust", None);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Rust Programming Guide");
    }
    
    fn create_test_content(
        id: &str,
        title: &str,
        content: &str,
        url: &str,
        content_type: ContentType,
        tags: Vec<String>,
    ) -> RstContent {
        RstContent::new(
            ContentMetadata {
                id: id.to_string(),
                title: title.to_string(),
                content_type,
                date: "2023-01-01".to_string(),
                tags,
                author: None,
                excerpt: None,
                url: url.to_string(),
                extra: std::collections::HashMap::new(),
            },
            format!("<p>{}</p>", content),
            Vec::new(),
            HashMap::new(),
        )
    }
}