//! HTML renderer

use crate::core::Result;
use crate::templates::TemplateEngine;
use crate::content::RstContent;
use tera::Context;

/// HTML renderer
pub struct HtmlRenderer {
    template_engine: TemplateEngine,
}

impl HtmlRenderer {
    /// Create a new HTML renderer
    pub fn new(template_engine: TemplateEngine) -> Self {
        Self {
            template_engine,
        }
    }
    
    /// Render content to HTML
    pub fn render_content(&self, content: &RstContent, site_config: &crate::core::config::SiteConfig) -> Result<String> {
        let mut context = Context::new();
        
        // Add site configuration
        context.insert("site", &site_config.site);
        context.insert("config", site_config);
        
        // Add content metadata
        context.insert("page", &content.metadata);
        context.insert("content", &content.html);
        context.insert("toc", &content.toc_html);
        context.insert("toc_html", &content.toc_html);
        context.insert("toc_entries", &content.toc);
        
        // Choose template based on content type
        let template_name = match content.metadata.content_type {
            crate::content::ContentType::Article => "article.html",
            crate::content::ContentType::Book => "book.html",
            crate::content::ContentType::Snippet => "snippet.html",
            crate::content::ContentType::Project => "project.html",
        };
        
        self.template_engine.render(template_name, &context)
    }
    
    /// Render index page
    pub fn render_index(&self, content: &[RstContent], site_config: &crate::core::config::SiteConfig) -> Result<String> {
        let mut context = Context::new();
        
        // Add site configuration
        context.insert("site", &site_config.site);
        context.insert("config", site_config);
        
        // Add content
        context.insert("content", content);
        
        // Add recent content
        let mut recent_content = content.to_vec();
        recent_content.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));
        recent_content.truncate(10);
        context.insert("recent_content", &recent_content);
        
        self.template_engine.render("index.html", &context)
    }
    
    /// Render search page
    pub fn render_search(&self, site_config: &crate::core::config::SiteConfig) -> Result<String> {
        let mut context = Context::new();
        
        // Add site configuration
        context.insert("site", &site_config.site);
        context.insert("config", site_config);
        
        self.template_engine.render("search.html", &context)
    }
}