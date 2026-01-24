//! Content resolver for cross-references and snippet resolution

use crate::content::RstContent;
use crate::core::Result;
use std::collections::HashMap;

/// Content resolver
pub struct ContentResolver {
    content_index: HashMap<String, usize>,
    snippets: HashMap<String, String>,
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
        // For now, just index snippet metadata
        // In a real implementation, you would extract actual snippet content
        if content.metadata.content_type == crate::content::ContentType::Snippet {
            self.snippets.insert(
                content.metadata.id.clone(),
                content.html.clone()
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
}

impl Default for ContentResolver {
    fn default() -> Self {
        Self::new()
    }
}