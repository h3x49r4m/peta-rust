//! Site representation and metadata

use crate::content::RstContent;
use crate::core::config::SiteConfig;
use std::collections::HashMap;

/// Represents a complete static site
#[derive(Debug, Clone)]
pub struct Site {
    pub config: SiteConfig,
    pub content: Vec<RstContent>,
    pub search_index: Option<String>,
    pub tag_counts: HashMap<String, usize>,
}

impl Site {
    /// Create a new site
    pub fn new(config: SiteConfig) -> Self {
        Self {
            config,
            content: Vec::new(),
            search_index: None,
            tag_counts: HashMap::new(),
        }
    }
    
    /// Create a site with content
    pub fn with_content(config: SiteConfig, content: Vec<RstContent>) -> Self {
        let mut site = Self::new(config);
        site.content = content;
        site.calculate_tag_counts();
        site
    }
    
    /// Calculate tag counts from content
    fn calculate_tag_counts(&mut self) {
        self.tag_counts.clear();
        
        for content in &self.content {
            for tag in &content.metadata.tags {
                *self.tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
    }
    
    /// Get content by type
    pub fn get_content_by_type(&self, content_type: &str) -> Vec<&RstContent> {
        let content_type_enum = crate::content::ContentType::from_string(content_type);
        self.content
            .iter()
            .filter(|c| c.metadata.content_type == content_type_enum)
            .collect()
    }
    
    /// Get content by tag
    pub fn get_content_by_tag(&self, tag: &str) -> Vec<&RstContent> {
        self.content
            .iter()
            .filter(|c| c.metadata.tags.contains(&tag.to_string()))
            .collect()
    }
    
    /// Get all unique tags
    pub fn get_all_tags(&self) -> Vec<&String> {
        let mut tags: Vec<&String> = self.tag_counts.keys().collect();
        tags.sort();
        tags
    }
    
    /// Get recent content
    pub fn get_recent_content(&self, limit: usize) -> Vec<&RstContent> {
        let mut content: Vec<&RstContent> = self.content.iter().collect();
        content.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));
        content.truncate(limit);
        content
    }
}