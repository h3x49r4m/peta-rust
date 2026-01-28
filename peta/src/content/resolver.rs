//! Content resolver for cross-references and snippet resolution

use crate::content::{ContentType, RstContent};
use crate::core::Result;
use std::collections::HashMap;

/// Content resolver
pub struct ContentResolver {
    content_index: HashMap<String, usize>,
    snippets: HashMap<String, RstContent>,
}

impl ContentResolver {
    /// Create a new content resolver
    pub fn new() -> Self {
        Self {
            content_index: HashMap::new(),
            snippets: HashMap::new(),
        }
    }

    /// Build content index from processed content
    pub fn build_index(&mut self, content: &[RstContent]) -> Result<()> {
        self.content_index.clear();
        self.snippets.clear();

        for (idx, item) in content.iter().enumerate() {
            // Index by title
            self.content_index.insert(item.metadata.title.clone(), idx);

            // Index by ID
            self.content_index.insert(item.metadata.id.clone(), idx);

            // Extract snippets
            self.extract_snippets(item)?;
        }

        Ok(())
    }

    /// Extract snippets from content
    fn extract_snippets(&mut self, content: &RstContent) -> Result<()> {
        // Cache full snippet content
        if content.metadata.content_type == ContentType::Snippet {
            self.snippets.insert(
                content.metadata.id.clone(),
                content.clone()
            );
        }

        Ok(())
    }

    /// Resolve cross-references in content
    pub fn resolve_references(&self, content: &str) -> Result<String> {
        // For now, return content as-is
        // In a real implementation, you would resolve internal links and snippet references
        Ok(content.to_string())
    }

    /// Get content by reference
    pub fn get_content_by_reference(&self, _reference: &str) -> Option<&RstContent> {
        // This would need access to the content array
        // For now, return None
        None
    }

    /// Get snippet by ID
    pub fn get_snippet_by_id(&self, id: &str) -> Option<&RstContent> {
        // Try exact match first
        if let Some(snippet) = self.snippets.get(id) {
            return Some(snippet);
        }
        
        // Try with underscores replaced by hyphens
        let normalized_id = id.replace('_', "-");
        if let Some(snippet) = self.snippets.get(&normalized_id) {
            return Some(snippet);
        }
        
        // Try with hyphens replaced by underscores
        let alt_id = id.replace('-', "_");
        if let Some(snippet) = self.snippets.get(&alt_id) {
            return Some(snippet);
        }
        
        None
    }
    /// Get snippet by filename or title (flexible lookup)
    /// Tries multiple strategies to find a snippet:
    /// 1. Exact match on ID
    /// 2. Filename match (e.g., "uncertainty-principle" matches file uncertainty-principle.rst)
    /// 3. Slugified match
    pub fn find_snippet(&self, reference: &str) -> Option<&RstContent> {
        // Try exact match first
        if let Some(snippet) = self.snippets.get(reference) {
            return Some(snippet);
        }

        // Try to find by filename pattern (match against URL or ID variations)
        let ref_lower = reference.to_lowercase();
        for snippet in self.snippets.values() {
            // Check if reference matches the ID
            if snippet.metadata.id.to_lowercase() == ref_lower {
                return Some(snippet);
            }
            // Check if reference matches the URL (without .html extension)
            let url_stem = snippet.metadata.url.trim_end_matches(".html");
            if url_stem.to_lowercase() == ref_lower || url_stem.ends_with(&format!("/{}", ref_lower)) {
                return Some(snippet);
            }
        }

        None
    }

    /// Get snippet index (ID -> index mapping)
    pub fn get_snippet_index(&self) -> HashMap<String, usize> {
        let mut index = HashMap::new();
        for (idx, snippet_id) in self.snippets.keys().enumerate() {
            index.insert(snippet_id.clone(), idx);
        }
        index
    }

    /// Check if a snippet exists
    pub fn has_snippet(&self, id: &str) -> bool {
        self.snippets.contains_key(id)
    }
}

impl Default for ContentResolver {
    fn default() -> Self {
        Self::new()
    }
}