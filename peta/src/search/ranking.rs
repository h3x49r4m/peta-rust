//! Search ranking algorithm

use crate::search::indexer::SearchDocument;

/// Ranking algorithm
pub struct RankingAlgorithm;

impl RankingAlgorithm {
    /// Create a new ranking algorithm
    pub fn new() -> Self {
        Self
    }
    
    /// Rank search results
    pub fn rank(&self, documents: &[SearchDocument], query: &str) -> Vec<f32> {
        documents
            .iter()
            .map(|doc| self.calculate_rank(doc, query))
            .collect()
    }
    
    /// Calculate rank for a document
    fn calculate_rank(&self, document: &SearchDocument, query: &str) -> f32 {
        let mut score = 0.0;
        let binding = query.to_lowercase();
        let query_terms = binding.split_whitespace().collect::<Vec<_>>();
        
        // Title relevance (highest weight)
        let title_lower = document.title.to_lowercase();
        for term in &query_terms {
            if title_lower.contains(term) {
                score += 20.0;
            }
        }
        
        // Tag relevance (high weight)
        for term in &query_terms {
            if document.tags.iter().any(|tag| tag.to_lowercase().contains(term)) {
                score += 10.0;
            }
        }
        
        // Content relevance (medium weight)
        let content_lower = format!("{} {}", document.excerpt, document.tags.join(" ")).to_lowercase();
        for term in &query_terms {
            if content_lower.contains(term) {
                score += 5.0;
            }
        }
        
        // Recency bonus (smaller weight)
        if let Ok(date) = chrono::NaiveDate::parse_from_str(&document.date, "%Y-%m-%d") {
            let days_old = (chrono::Utc::now().date_naive() - date).num_days();
            if days_old < 30 {
                score += 2.0;
            } else if days_old < 365 {
                score += 1.0;
            }
        }
        
        score
    }
}

impl Default for RankingAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}