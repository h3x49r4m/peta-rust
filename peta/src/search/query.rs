//! Search query processor

use crate::search::indexer::SearchDocument;
use crate::core::Result;

/// Query processor
pub struct QueryProcessor;

impl QueryProcessor {
    /// Create a new query processor
    pub fn new() -> Self {
        Self
    }
    
    /// Process search query
    pub fn process(&self, query: &str, documents: &[SearchDocument]) -> Result<Vec<SearchResult>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }
        
        let terms = self.tokenize(query);
        let mut results = Vec::new();
        
        for (_idx, document) in documents.iter().enumerate() {
            let score = self.calculate_score(document, &terms);
            
            if score > 0.0 {
                results.push(SearchResult {
                    document: document.clone(),
                    score,
                    highlights: self.generate_highlights(document, &terms),
                });
            }
        }
        
        // Sort by score
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        
        Ok(results)
    }
    
    /// Tokenize query
    fn tokenize(&self, query: &str) -> Vec<String> {
        query.to_lowercase()
            .split_whitespace()
            .map(|s| s.trim_matches(&['.', ',', ';', ':', '!', '?', '(', ')', '[', ']', '{', '}'][..]))
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }
    
    /// Calculate relevance score
    fn calculate_score(&self, document: &SearchDocument, terms: &[String]) -> f32 {
        let mut score = 0.0;
        let content = format!("{} {} {}", document.title, document.excerpt, document.tags.join(" "));
        let content_lower = content.to_lowercase();
        
        for term in terms {
            // Title matches are worth more
            if document.title.to_lowercase().contains(term) {
                score += 10.0;
            }
            
            // Tag matches
            if document.tags.iter().any(|tag| tag.to_lowercase().contains(term)) {
                score += 5.0;
            }
            
            // Content matches
            if content_lower.contains(term) {
                score += 1.0;
            }
        }
        
        score
    }
    
    /// Generate highlights
    fn generate_highlights(&self, document: &SearchDocument, terms: &[String]) -> Vec<String> {
        let mut highlights = Vec::new();
        
        // Highlight title matches
        for term in terms {
            if document.title.to_lowercase().contains(term) {
                highlights.push(format!("Title: {}", document.title));
                break;
            }
        }
        
        // Highlight content matches
        let excerpt_lower = document.excerpt.to_lowercase();
        for term in terms {
            if let Some(pos) = excerpt_lower.find(term) {
                let start = if pos > 50 { pos - 50 } else { 0 };
                let end = if pos + term.len() + 50 < document.excerpt.len() {
                    pos + term.len() + 50
                } else {
                    document.excerpt.len()
                };
                
                highlights.push(format!("...{}...", &document.excerpt[start..end]));
                break;
            }
        }
        
        highlights
    }
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub document: SearchDocument,
    pub score: f32,
    pub highlights: Vec<String>,
}

impl Default for QueryProcessor {
    fn default() -> Self {
        Self::new()
    }
}