//! Table of contents generator

use crate::content::TocEntry;
use crate::core::Result;

/// Table of contents generator
pub struct TocGenerator {
    #[allow(dead_code)]
    max_depth: usize,
}

impl TocGenerator {
    /// Create a new TOC generator
    pub fn new() -> Self {
        Self {
            max_depth: 3,
        }
    }
    
    /// Generate table of contents from HTML content
    pub fn generate(&self, _html: &str) -> Result<Vec<TocEntry>> {
        // Simple implementation for now
        Ok(vec![])
    }
}

impl Default for TocGenerator {
    fn default() -> Self {
        Self::new()
    }
}