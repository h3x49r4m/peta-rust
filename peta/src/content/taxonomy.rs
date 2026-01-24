//! Taxonomy and tag management

use crate::content::RstContent;
use crate::core::Result;
use std::collections::HashMap;

/// Taxonomy manager
pub struct TaxonomyManager {
    tag_counts: HashMap<String, usize>,
    category_counts: HashMap<String, usize>,
}

impl TaxonomyManager {
    /// Create a new taxonomy manager
    pub fn new() -> Self {
        Self {
            tag_counts: HashMap::new(),
            category_counts: HashMap::new(),
        }
    }
    
    /// Build taxonomy from content
    pub fn build_taxonomy(&mut self, content: &[RstContent]) -> Result<()> {
        self.tag_counts.clear();
        self.category_counts.clear();
        
        for item in content {
            // Count tags
            for tag in &item.metadata.tags {
                *self.tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
            
            // Count categories (if any)
            if let Some(category) = self.extract_category(&item.metadata) {
                *self.category_counts.entry(category).or_insert(0) += 1;
            }
        }
        
        Ok(())
    }
    
    /// Extract category from metadata
    fn extract_category(&self, metadata: &crate::content::ContentMetadata) -> Option<String> {
        // For now, use content type as category
        Some(metadata.content_type.to_string())
    }
    
    /// Get all tags
    pub fn get_all_tags(&self) -> Vec<&String> {
        let mut tags: Vec<&String> = self.tag_counts.keys().collect();
        tags.sort();
        tags
    }
    
    /// Get tag count
    pub fn get_tag_count(&self, tag: &str) -> usize {
        self.tag_counts.get(tag).copied().unwrap_or(0)
    }
    
    /// Get all categories
    pub fn get_all_categories(&self) -> Vec<&String> {
        let mut categories: Vec<&String> = self.category_counts.keys().collect();
        categories.sort();
        categories
    }
    
    /// Get category count
    pub fn get_category_count(&self, category: &str) -> usize {
        self.category_counts.get(category).copied().unwrap_or(0)
    }
    
    /// Get popular tags
    pub fn get_popular_tags(&self, limit: usize) -> Vec<(&String, usize)> {
        let mut tags: Vec<(&String, usize)> = self.tag_counts.iter()
            .map(|(k, v)| (k, *v))
            .collect();
        tags.sort_by(|a, b| b.1.cmp(&a.1));
        tags.truncate(limit);
        tags
    }
}

impl Default for TaxonomyManager {
    fn default() -> Self {
        Self::new()
    }
}