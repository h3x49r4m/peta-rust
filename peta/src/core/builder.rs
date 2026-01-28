//! Site builder implementation following RST-first architecture

use crate::core::{Site, SiteConfig, Result, Error};
use crate::core::theme::{Theme, ThemeSystem};
use crate::content::{RstContent, ContentType};
use crate::search::SearchIndex;
use crate::templates::TemplateEngine;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use glob::glob;
use chrono::NaiveDate;

/// Main site builder that orchestrates the RST-to-HTML pipeline
pub struct SiteBuilder {
    config: SiteConfig,
    rst_content: Vec<RstContent>,
    theme_system: ThemeSystem,
    search_index: SearchIndex,
    component_registry: crate::components::ComponentRegistry,
}

impl SiteBuilder {
    /// Create a new site builder with configuration
    pub fn new(config: SiteConfig) -> Self {
        let component_registry = crate::components::ComponentRegistry::new();
        let mut theme_system = ThemeSystem::new("themes");
        
        // Initialize theme system
        if let Err(e) = theme_system.initialize(&config) {
            eprintln!("Warning: Failed to initialize theme system: {}", e);
        }
        
        Self {
            config,
            rst_content: Vec::new(),
            theme_system,
            search_index: SearchIndex::new(),
            component_registry,
        }
    }
    
    /// Build the complete static site following RST-first architecture
    pub async fn build(&mut self) -> Result<Site> {
        // 1. Load RST content
        self.load_rst_content().await?;
        
        // 2. Parse RST to HTML
        self.parse_rst_to_html().await?;
        
        // 3. Resolve snippet references
        self.resolve_references().await?;
        
        // 4. Build search index
        self.build_search_index().await?;
        
        // 5. Generate static site
        self.generate_static_site().await?;
        
        Ok(Site::with_content(
            self.config.clone(),
            self.rst_content.clone()
        ))
    }
    
    /// Load RST content from the content directory
    async fn load_rst_content(&mut self) -> Result<()> {
        let content_dir = PathBuf::from(&self.config.build.content_dir);
        
        // Load articles
        self.load_content_type(&content_dir.join("articles"), ContentType::Article).await?;
        
        // Load snippets
        self.load_content_type(&content_dir.join("snippets"), ContentType::Snippet).await?;
        
        // Load books
        self.load_content_type(&content_dir.join("books"), ContentType::Book).await?;
        
        // Load projects
        self.load_content_type(&content_dir.join("projects"), ContentType::Project).await?;
        
        Ok(())
    }
    
    /// Load content of a specific type from directory
    async fn load_content_type(&mut self, dir: &PathBuf, content_type: ContentType) -> Result<()> {
        if !dir.exists() {
            return Ok(());
        }
        
        use std::collections::HashSet;
        let mut loaded_files = HashSet::new();
        
        // First load index.rst files in subdirectories
        let pattern = dir.join("**/index.rst");
        
        if let Ok(paths) = glob(pattern.to_str().unwrap()) {
            for path in paths.flatten() {
                if let Ok(content) = self.load_rst_file(&path, content_type.clone()).await {
                    loaded_files.insert(path.clone());
                    self.rst_content.push(content);
                }
            }
        }
        
        // Then load direct .rst files in the directory (not in subdirectories)
        let pattern = dir.join("*.rst");
        
        if let Ok(paths) = glob(pattern.to_str().unwrap()) {
            for path in paths.flatten() {
                if let Ok(content) = self.load_rst_file(&path, content_type.clone()).await {
                    loaded_files.insert(path.clone());
                    self.rst_content.push(content);
                }
            }
        }
        
        // For books, also load non-index .rst files in subdirectories (chapters)
        if content_type == ContentType::Book {
            let pattern = dir.join("**/*.rst");
            
            if let Ok(paths) = glob(pattern.to_str().unwrap()) {
                for path in paths.flatten() {
                    // Skip files that were already loaded
                    if !loaded_files.contains(&path) {
                        if let Ok(content) = self.load_rst_file(&path, content_type.clone()).await {
                            loaded_files.insert(path.clone());
                            self.rst_content.push(content);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Load a single RST file
    async fn load_rst_file(&self, path: &PathBuf, content_type: ContentType) -> Result<RstContent> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::content(format!("Failed to read file {}: {}", path.display(), e)))?;
        
        // Parse RST content using the RST parser with content type override and file path
        let mut parser = crate::content::rst::parser::RstParser::new()?;
        parser.parse_with_type_and_path(&content, Some(content_type), Some(path))
            .map_err(|e| Error::rst_parse(format!("Failed to parse RST file {}: {}", path.display(), e)))
    }
    
    /// Parse RST content to HTML following RST-first architecture
    async fn parse_rst_to_html(&mut self) -> Result<()> {
        // This is handled during load_rst_file to maintain RST→HTML direct conversion
        // No intermediate JSON structures are created
        Ok(())
    }
    
    /// Resolve snippet references and cross-links
    async fn resolve_references(&mut self) -> Result<()> {
        use crate::content::resolver::ContentResolver;
        use regex::Regex;

        // Build resolver with all snippets
        let mut resolver = ContentResolver::new();
        resolver.build_index(&self.rst_content)?;

        // Regex to match snippet-card placeholders (with or without content)
        let card_regex = Regex::new(r#"<div class="embedded-snippet-card" data-snippet="([^"]+)">(.*?)</div>"#)
            .map_err(|e| Error::content(format!("Failed to compile snippet card regex: {}", e)))?;

// Resolve snippet-card directives in all content
        for content in self.rst_content.iter_mut() {
            let mut result = content.html.clone();
            let mut offset = 0;

            for mat in card_regex.find_iter(&content.html) {
                // Extract snippet_id from the data-snippet attribute
                let match_text = mat.as_str();
                let snippet_id = match_text
                    .split("data-snippet=\"")
                    .nth(1)
                    .and_then(|s| s.split("\"").next())
                    .unwrap_or("");
                
                if let Some(snippet) = resolver.find_snippet(snippet_id) {
                    // Render the actual snippet card
                    let renderer = crate::content::rst::EmbeddedSnippetCardRenderer::new()?;
                    let rendered_card = renderer.render(snippet)?;

                    // Replace the placeholder with the rendered card
                    let before = &result[..mat.start() + offset];
                    let after = &result[mat.end() + offset..];
                    result = format!("{}{}{}", before, rendered_card, after);
                    offset += rendered_card.len() - mat.len();
                }
            }

            content.html = result;
        }

        // Regenerate TOC for articles and projects to include embedded snippet cards
        let toc_generator = crate::content::rst::toc_generator::TocGenerator::new();
        for content in self.rst_content.iter_mut() {
            if content.metadata.content_type == crate::content::ContentType::Article || 
               content.metadata.content_type == crate::content::ContentType::Project {
                // Use enhanced TOC generator that includes embedded snippet cards
                let toc_entries = toc_generator.generate_with_snippets(&content.html)?;
                let toc_html = toc_generator.render_html(&toc_entries);
                content.toc = toc_entries;
                content.toc_html = toc_html;
            }
        }

        // Resolve internal links
        // Process toctree directives
        Ok(())
    }

    /// Get directive handler (placeholder for implementation)
    fn get_directive_handler(&mut self) -> Option<&mut crate::content::rst::directives::SnippetCardHandler> {
        // This would need to return the actual directive handler from the parser
        // For now, return None
        None
    }
    
    /// Build search index for client-side search
    async fn build_search_index(&mut self) -> Result<()> {
        if !self.config.search.enabled {
            return Ok(());
        }
        
        self.search_index.build(&self.rst_content)?;
        Ok(())
    }
    
    /// Generate static HTML site with V4 architecture support
    
        async fn generate_static_site(&mut self) -> Result<()> {
    
            let theme = Theme::load(
    
                self.theme_system.current_theme(),
    
                Path::new("themes").join(self.theme_system.current_theme())
    
            )?;
    
            let template_engine = TemplateEngine::new_with_components(&theme, self.component_registry.clone())?;        
    
            
    
            // Load components from theme
    
            self.load_components(&template_engine)?;
    
            
    
            // Create output directory
    
            let output_dir = PathBuf::from(&self.config.build.output_dir);
    
            std::fs::create_dir_all(&output_dir)?;
    
            
    
            // Generate contexts data (site.json, search.json, tags.json)
    
            self.generate_contexts_data(&output_dir)?;
    
            
    
            // Generate hooks JavaScript files
    
            self.generate_hooks_system(&output_dir)?;
    
            
    
            // Generate index page
    
            self.generate_index_page(&template_engine, &output_dir)?;
    
            
    
            // Generate pages for each content type
    
            self.generate_content_pages(&template_engine, &output_dir)?;
    
            
    
            // Generate search functionality
    
            
    
                        if self.config.search.enabled {
    
            
    
                            self.generate_search_page(&template_engine, &output_dir)?;
    
            
    
                        }
    
            
    
                        
    
            
    
                        // Generate 404 page
    
            
    
                        self.generate_404_page(&template_engine, &output_dir)?;
    
            
    
                        
    
            
    
                        // Copy and process assets
    
            
    
                        self.process_assets(&output_dir)?;
    
            
    
            Ok(())
    
        }
    
    /// Generate index page
    fn generate_index_page(&self, template_engine: &TemplateEngine, output_dir: &PathBuf) -> Result<()> {
        let mut context = self.create_base_context();
        
        // Add recent content to index context
        let recent_content: Vec<_> = self.rst_content
            .iter()
            .take(10) // Show last 10 items
            .map(|content| {
                serde_json::json!({
                    "title": content.metadata.title,
                    "url": content.metadata.url,
                    "date": content.metadata.date,
                    "tags": content.metadata.tags,
                    "content_type": content.metadata.content_type
                })
            })
            .collect();
        
        context.insert("recent_content", &recent_content);
        
        // Generate tags data
        let tags = self.generate_tags_data();
        context.insert("tags", &tags);
        
        // Add content type counts
        let books_count = self.rst_content.iter()
            .filter(|c| c.metadata.content_type == ContentType::Book && c.metadata.url.ends_with("index.html"))
            .count();
        let articles_count = self.rst_content.iter()
            .filter(|c| c.metadata.content_type == ContentType::Article)
            .count();
        let snippets_count = self.rst_content.iter()
            .filter(|c| c.metadata.content_type == ContentType::Snippet)
            .count();
        let projects_count = self.rst_content.iter()
            .filter(|c| c.metadata.content_type == ContentType::Project)
            .count();
        
        context.insert("books_count", &books_count);
        context.insert("articles_count", &articles_count);
        context.insert("snippets_count", &snippets_count);
        context.insert("projects_count", &projects_count);
        
        let html = template_engine.render("index.html", &context)?;
        std::fs::write(output_dir.join("index.html"), html)?;
        
        Ok(())
    }
    
    /// Generate pages for all content
    fn generate_content_pages(&self, template_engine: &TemplateEngine, output_dir: &PathBuf) -> Result<()> {
        // Generate individual content pages
        for content in &self.rst_content {
            let template_name = match content.metadata.content_type {
                ContentType::Article => "article.html",
                ContentType::Book => "book.html",
                ContentType::Snippet => "snippet.html",
                ContentType::Project => "project.html",
            };
            
            let html = template_engine.render(template_name, &self.create_template_context(content))?;
            
            let file_path = output_dir.join(&content.metadata.url);
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            std::fs::write(file_path, html)?;
        }
        
        // Generate listing pages for each content type
        self.generate_listing_pages(template_engine, output_dir)?;
        
        Ok(())
    }
    
    /// Generate listing pages for each content type
    fn generate_listing_pages(&self, template_engine: &TemplateEngine, output_dir: &PathBuf) -> Result<()> {
        // Group content by type
        let mut articles = Vec::new();
        let mut snippets = Vec::new();
        let mut books = Vec::new();
        let mut projects = Vec::new();
        
        for content in &self.rst_content {
            let excerpt = content.metadata.excerpt.as_deref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| content.get_excerpt(200));
            
            let mut item = serde_json::json!({
                "title": content.metadata.title,
                "url": content.metadata.url,
                "date": content.metadata.date,
                "tags": content.metadata.tags,
                "excerpt": excerpt,
                "author": content.metadata.author,
                "content_type": format!("{:?}", content.metadata.content_type)
            });
            
            // Add content field for snippets to enable modal display
            if content.metadata.content_type == ContentType::Snippet {
                item["content"] = serde_json::Value::String(content.html.clone());
                if let Some(language) = content.metadata.extra.get("language") {
                    item["language"] = serde_json::Value::String(language.clone());
                }
                item["slug"] = serde_json::Value::String(
                    content.metadata.url.split('/').last().unwrap_or("").replace(".html", "")
                );
            }
            
            match content.metadata.content_type {
                ContentType::Article => articles.push(item),
                ContentType::Snippet => snippets.push(item),
                ContentType::Book => {
                    // Only include book index files, not chapters
                    if content.metadata.url.ends_with("index.html") {
                        books.push(item);
                    }
                },
                ContentType::Project => projects.push(item),
            }
        }
        
        // Sort by date (newest first)
        self.sort_content_by_date(&mut articles);
        self.sort_content_by_date(&mut snippets);
        self.sort_content_by_date(&mut books);
        self.sort_content_by_date(&mut projects);
        
        // Generate articles listing
        let mut articles_context = self.create_base_context();
        articles_context.insert("articles", &articles);
        let articles_html = template_engine.render("articles.html", &articles_context)?;
        std::fs::write(output_dir.join("articles.html"), articles_html)?;
        
        // Generate snippets listing
        let mut snippets_context = self.create_base_context();
        snippets_context.insert("snippets", &snippets);
        let snippets_html = template_engine.render("snippets.html", &snippets_context)?;
        std::fs::write(output_dir.join("snippets.html"), snippets_html)?;
        
        // Generate books listing
        let mut books_context = self.create_base_context();
        books_context.insert("books", &books);
        let books_html = template_engine.render("books.html", &books_context)?;
        std::fs::write(output_dir.join("books.html"), books_html)?;
        
        // Generate projects listing
        let mut projects_context = self.create_base_context();
        projects_context.insert("projects", &projects);
        let projects_html = template_engine.render("projects.html", &projects_context)?;
        std::fs::write(output_dir.join("projects.html"), projects_html)?;
        
        Ok(())
    }
    
    /// Generate search page and index
    fn generate_search_page(&self, template_engine: &TemplateEngine, output_dir: &PathBuf) -> Result<()> {
        // Generate search.html
        let search_html = template_engine.render("search.html", &self.create_base_context())?;
        std::fs::write(output_dir.join("search.html"), search_html)?;
        
        // Generate search.json for client-side search
        let search_json = self.search_index.generate_client_search()?;
        std::fs::write(output_dir.join("search.json"), search_json)?;
        
        Ok(())
    }
    
    /// Generate 404 page
    fn generate_404_page(&self, template_engine: &TemplateEngine, output_dir: &PathBuf) -> Result<()> {
        // Generate 404.html
        let not_found_html = template_engine.render("404.html", &self.create_base_context())?;
        std::fs::write(output_dir.join("404.html"), not_found_html)?;
        
        Ok(())
    }
    
    /// Process and copy assets
    fn process_assets(&self, output_dir: &PathBuf) -> Result<()> {
        let assets_dir = output_dir.join("assets");
        std::fs::create_dir_all(&assets_dir)?;
        
        // Process theme assets using the asset pipeline
        let mut asset_pipeline = crate::assets::AssetPipeline::new(
            self.theme_system.current_theme(),
            &assets_dir,
        );
        
        asset_pipeline.set_component_registry(self.component_registry.clone());        if let Err(e) = asset_pipeline.process_assets() {
            eprintln!("Warning: Failed to process theme assets: {}", e);
            // Fallback to basic asset copying
            self.copy_theme_assets("css", &assets_dir)?;
            self.copy_theme_assets("js", &assets_dir)?;
            self.copy_theme_assets("images", &assets_dir)?;
        }
        
        Ok(())
    }
    
    /// Generate tags data for tag cloud
    fn generate_tags_data(&self) -> Vec<serde_json::Value> {
        use std::collections::HashMap;
        
        let mut tag_counts: HashMap<String, usize> = HashMap::new();
        
        // Count all tags across all content
        for content in &self.rst_content {
            for tag in &content.metadata.tags {
                *tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        
        // Convert to vector and sort by count (descending)
        let mut tags: Vec<_> = tag_counts
            .into_iter()
            .map(|(name, count)| {
                serde_json::json!({
                    "name": name,
                    "count": count
                })
            })
            .collect();
        
        tags.sort_by(|a, b| {
            b.get("count").unwrap_or(&0.into()).as_u64().unwrap_or(0)
                .cmp(&a.get("count").unwrap_or(&0.into()).as_u64().unwrap_or(0))
        });
        
        tags
    }
    
    /// Copy theme assets to output directory
    fn copy_theme_assets(&self, asset_type: &str, output_dir: &PathBuf) -> Result<()> {
        let theme = Theme::load(
            self.theme_system.current_theme(),
            Path::new("themes").join(self.theme_system.current_theme())
        )?;
        let theme_asset_dir = theme.assets_dir.join(asset_type);
        let output_asset_dir = output_dir.join(asset_type);
        
        if theme_asset_dir.exists() {
            std::fs::create_dir_all(&output_asset_dir)?;
            
            for entry in WalkDir::new(&theme_asset_dir) {
                let entry = entry.map_err(|e| Error::other(format!("Failed to walk directory: {}", e)))?;
                let path = entry.path();
                
                if path.is_file() {
                    let relative_path = path.strip_prefix(&theme_asset_dir)
                        .map_err(|e| Error::other(format!("Failed to get relative path: {}", e)))?;
                    let output_path = output_asset_dir.join(relative_path);
                    
                    if let Some(parent) = output_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    
                    std::fs::copy(path, &output_path)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Create template context for content rendering
    fn create_template_context(&self, content: &RstContent) -> tera::Context {
        let mut context = self.create_base_context();
        
        // Create page metadata with slug
        let mut page_metadata = content.metadata.clone();
        page_metadata.extra.insert("slug".to_string(), 
            content.metadata.url.split('/').last().unwrap_or("").replace(".html", "")
        );
        
        context.insert("page", &page_metadata);
        context.insert("content", &content.html);
        context.insert("toc", &content.toc);
        context.insert("has_math_formulas", &content.has_math_formulas);
        context.insert("math_formula_count", &content.math_formula_count);
        
        // Generate book TOC for book pages
        if content.metadata.content_type == ContentType::Book {
            if let Ok(book_toc) = self.generate_book_toc(&content) {
                context.insert("book_toc", &book_toc);
            }
            if let Ok(book_title) = self.get_book_title(&content) {
                context.insert("book_title", &book_title);
            }
            if let Ok(book_author) = self.get_book_author(&content) {
                context.insert("book_author", &book_author);
            }
        }
        
        // Add all snippets data if this is a snippet page
        if content.metadata.content_type == ContentType::Snippet {
            let mut snippets = Vec::new();
            for snippet_content in &self.rst_content {
                if snippet_content.metadata.content_type == ContentType::Snippet {
                    let mut item = serde_json::json!({
                        "title": snippet_content.metadata.title,
                        "url": snippet_content.metadata.url,
                        "date": snippet_content.metadata.date,
                        "tags": snippet_content.metadata.tags,
                        "excerpt": snippet_content.metadata.excerpt.as_deref().unwrap_or("No excerpt available"),
                        "author": snippet_content.metadata.author,
                        "content_type": format!("{:?}", snippet_content.metadata.content_type)
                    });
                    
                    // Add content field for snippets to enable modal display
                    item["content"] = serde_json::Value::String(snippet_content.html.clone());
                    if let Some(language) = snippet_content.metadata.extra.get("language") {
                        item["language"] = serde_json::Value::String(language.clone());
                    }
                    item["slug"] = serde_json::Value::String(
                        snippet_content.metadata.url.split('/').last().unwrap_or("").replace(".html", "")
                    );
                    
                    snippets.push(item);
                }
            }
            context.insert("snippets", &snippets);
        }
        
        context
    }
    
    /// Create base template context
    fn create_base_context(&self) -> tera::Context {
        let mut context = tera::Context::new();
        context.insert("site", &self.config.site);
        context.insert("config", &self.config);
        
        // Add component information
        let enabled_components = self.component_registry.get_enabled_components();
        let component_names: Vec<String> = enabled_components
            .iter()
            .map(|c| c.name.clone())
            .collect();
            
        context.insert("components", &serde_json::json!({
            "enabled": enabled_components,
            "names": component_names,
            "registry": self.component_registry.get_all_components(),
        }));
        
        context
    }
    
    /// Generate book table of contents for a book page
    fn generate_book_toc(&self, content: &RstContent) -> Result<String> {
        use crate::content::rst::book_toc_generator::BookTocGenerator;
        
        // Extract book directory from URL
        let url_parts: Vec<&str> = content.metadata.url.split('/').collect();
        if url_parts.len() < 2 || url_parts[0] != "books" {
            return Ok(String::new());
        }
        
        let book_dir_name = url_parts[1];
        let book_dir = Path::new("_content/books").join(book_dir_name);
        
        if !book_dir.exists() {
            return Ok(String::new());
        }
        
        // Generate book TOC
        let generator = BookTocGenerator::new();
        let chapters = generator.generate(&book_dir)?;
        
        // Render TOC as HTML
        let toc_html = generator.render_html(&chapters);
        
        Ok(toc_html)
    }
    
    /// Get book title from the book's index.rst file
    fn get_book_title(&self, content: &RstContent) -> Result<String> {
        use std::fs;
        
        // Extract book directory from URL
        let url_parts: Vec<&str> = content.metadata.url.split('/').collect();
        if url_parts.len() < 2 || url_parts[0] != "books" {
            return Ok(content.metadata.title.clone());
        }
        
        let book_dir_name = url_parts[1];
        let index_path = Path::new("_content/books").join(book_dir_name).join("index.rst");
        
        if !index_path.exists() {
            return Ok(content.metadata.title.clone());
        }
        
        let index_content = fs::read_to_string(&index_path)?;
        
        // Extract title from frontmatter
        if let Some(frontmatter) = index_content.split("---").nth(1) {
            for line in frontmatter.lines() {
                if line.starts_with("title:") {
                    let title = line.trim_start_matches("title:")
                        .trim()
                        .trim_start_matches('"')
                        .trim_end_matches('"')
                        .trim_start_matches('\'')
                        .trim_end_matches('\'');
                    return Ok(title.to_string());
                }
            }
        }
        
        // Fallback to content title
        Ok(content.metadata.title.clone())
    }
    
    /// Get book author from the book's index.rst file
    fn get_book_author(&self, content: &RstContent) -> Result<String> {
        use std::fs;
        
        // Extract book directory from URL
        let url_parts: Vec<&str> = content.metadata.url.split('/').collect();
        if url_parts.len() < 2 || url_parts[0] != "books" {
            return Ok(content.metadata.author.clone().unwrap_or_default());
        }
        
        let book_dir_name = url_parts[1];
        let index_path = Path::new("_content/books").join(book_dir_name).join("index.rst");
        
        if !index_path.exists() {
            return Ok(content.metadata.author.clone().unwrap_or_default());
        }
        
        let index_content = fs::read_to_string(&index_path)?;
        
        // Extract author from frontmatter
        if let Some(frontmatter) = index_content.split("---").nth(1) {
            for line in frontmatter.lines() {
                if line.starts_with("author:") {
                    let author = line.trim_start_matches("author:")
                        .trim()
                        .trim_start_matches('"')
                        .trim_end_matches('"')
                        .trim_start_matches('\'')
                        .trim_end_matches('\'');
                    return Ok(author.to_string());
                }
            }
        }
        
        // Fallback to content author
        Ok(content.metadata.author.clone().unwrap_or_default())
    }
    
    /// Generate contexts data files (site.json, search.json, tags.json)
    fn generate_contexts_data(&self, output_dir: &PathBuf) -> Result<()> {
        let contexts_dir = output_dir.join("contexts");
        std::fs::create_dir_all(&contexts_dir)?;
        
        // Generate site.json
        self.generate_site_context(&contexts_dir)?;
        
        // Generate search.json (enhanced with component data)
        self.generate_search_context(&contexts_dir)?;
        
        // Generate tags.json
        self.generate_tags_context(&contexts_dir)?;
        
        Ok(())
    }
    
    /// Generate site.json context data
    fn generate_site_context(&self, contexts_dir: &PathBuf) -> Result<()> {
        use chrono::Utc;
        
        // Build navigation structure
        let mut main_nav = Vec::new();
        let mut footer_nav = Vec::new();
        
        // Add main navigation items
        main_nav.push(serde_json::json!({
            "title": "Home",
            "url": "/",
            "icon": "home",
            "weight": 10
        }));
        
        main_nav.push(serde_json::json!({
            "title": "Documentation",
            "url": "/docs/",
            "icon": "book",
            "weight": 20,
            "children": [
                {"title": "Getting Started", "url": "/docs/getting-started/", "weight": 21},
                {"title": "Components", "url": "/docs/components/", "weight": 22},
                {"title": "Themes", "url": "/docs/themes/", "weight": 23},
                {"title": "Deployment", "url": "/docs/deployment/", "weight": 24}
            ]
        }));
        
        // Add content type navigation
        let content_types = vec!["articles", "books", "projects", "snippets"];
        for (i, content_type) in content_types.iter().enumerate() {
            let title = content_type.chars().next().unwrap().to_uppercase().collect::<String>() + &content_type[1..];
            main_nav.push(serde_json::json!({
                "title": title,
                "url": format!("/{}/", content_type),
                "icon": match *content_type {
                    "articles" => "file-text",
                    "books" => "book-open",
                    "projects" => "code",
                    "snippets" => "terminal",
                    _ => "file"
                },
                "weight": 30 + i as i32 * 10
            }));
        }
        
        // Add footer navigation
        footer_nav.push(serde_json::json!({
            "title": "GitHub",
            "url": "https://github.com/peta-rs/peta",
            "icon": "github",
            "external": true
        }));
        
        footer_nav.push(serde_json::json!({
            "title": "Discord",
            "url": "https://discord.gg/peta",
            "icon": "message-circle",
            "external": true
        }));
        
        // Generate site context
        let site_context = serde_json::json!({
            "site": {
                "title": self.config.site.title,
                "description": self.config.site.description,
                "url": self.config.site.url,
                "base_url": "",
                "author": self.config.site.author,
                "email": self.config.site.author,
                "logo": "/assets/images/logo.svg",
                "favicon": "/assets/images/favicon.ico",
                "language": "en",
                "timezone": "UTC",
                "build_date": Utc::now().to_rfc3339(),
                "version": "4.0.0",
                "generator": "Peta v4.0.0"
            },
            "navigation": {
                "main": main_nav,
                "footer": footer_nav
            },
            "social": {
                "twitter": "https://twitter.com/peta_dev",
                "github": "https://github.com/peta-rs/peta",
                "discord": "https://discord.gg/peta"
            },
            "metadata": {
                "keywords": ["static site generator", "rust", "web development", "components", "themes"],
                "robots": "index, follow"
            },
            "features": {
                "search": self.config.search.enabled,
                "dark_mode": true,
                "comments": false,
                "analytics": false,
                "rss": true,
                "sitemap": true
            },
            "theme": {
                "default": "default",
                "available": ["default", "minimal", "blog", "documentation"]
            },
            "build": {
                "minify": true,
                "optimize_images": true,
                "generate_sitemap": true,
                "generate_rss": true,
                "output_dir": "_out/dist"
            }
        });
        
        let site_json = serde_json::to_string_pretty(&site_context)?;
        std::fs::write(contexts_dir.join("site.json"), site_json)?;
        
        Ok(())
    }
    
    /// Generate search.json with enhanced data for V4 search
    fn generate_search_context(&self, contexts_dir: &PathBuf) -> Result<()> {
        use chrono::Utc;
        let mut search_pages = Vec::new();
        
        for content in &self.rst_content {
            let page_data = serde_json::json!({
                "id": content.metadata.url.replace("/", "_").trim_matches('_'),
                "title": content.metadata.title,
                "url": content.metadata.url,
                "content": self.strip_html_tags(&content.html),
                "description": content.metadata.excerpt.as_deref().unwrap_or(""),
                "type": match content.metadata.content_type {
                    ContentType::Article => "article",
                    ContentType::Book => "book",
                    ContentType::Snippet => "snippet",
                    ContentType::Project => "project",
                },
                "tags": content.metadata.tags,
                "category": match content.metadata.content_type {
                    ContentType::Article => "articles",
                    ContentType::Book => "books",
                    ContentType::Snippet => "snippets",
                    ContentType::Project => "projects",
                },
                "date": content.metadata.date,
                "author": content.metadata.author.as_deref().unwrap_or("Peta Team"),
                "views": 0, // Would be populated from analytics
                "reading_time": self.estimate_reading_time(&content.html)
            });
            
            search_pages.push(page_data);
        }
        
        let search_context = serde_json::json!({
            "index": {
                "version": "1.0.0",
                "generated": Utc::now().to_rfc3339(),
                "total_pages": search_pages.len()
            },
            "pages": search_pages
        });
        
        let search_json = serde_json::to_string_pretty(&search_context)?;
        std::fs::write(contexts_dir.join("search.json"), search_json)?;
        
        Ok(())
    }
    
    /// Generate tags.json with tag categories and popular tags
    fn generate_tags_context(&self, contexts_dir: &PathBuf) -> Result<()> {
        use std::collections::HashMap;
        
        let mut tag_counts: HashMap<String, usize> = HashMap::new();
        let mut tag_categories: HashMap<String, String> = HashMap::new();
        
        // Count tags and assign categories
        for content in &self.rst_content {
            for tag in &content.metadata.tags {
                *tag_counts.entry(tag.clone()).or_insert(0) += 1;
                
                // Assign category based on tag
                if !tag_categories.contains_key(tag) {
                    let category = self.categorize_tag(tag);
                    tag_categories.insert(tag.clone(), category);
                }
            }
        }
        
        // Build tags array
        let mut tags = Vec::new();
        for (name, count) in tag_counts {
            let category = tag_categories.get(&name).map(|s| s.as_str()).unwrap_or("general");
            let slug = name.to_lowercase().replace(' ', "-");
            
            tags.push(serde_json::json!({
                "name": name,
                "slug": slug,
                "count": count,
                "description": format!("{} content", name),
                "category": category
            }));
        }
        
        // Sort by count (descending)
        tags.sort_by(|a, b| {
            b.get("count").unwrap_or(&0.into()).as_u64().unwrap_or(0)
                .cmp(&a.get("count").unwrap_or(&0.into()).as_u64().unwrap_or(0))
        });
        
        // Build categories
        let mut category_counts: HashMap<String, usize> = HashMap::new();
        for tag in &tags {
            let category = tag.get("category").unwrap().as_str().unwrap_or("general");
            *category_counts.entry(category.to_string()).or_insert(0) += 1;
        }
        
        let mut categories = Vec::new();
        for (name, count) in category_counts {
            categories.push(serde_json::json!({
                "name": name,
                "description": match name.as_str() {
                    "general" => "General content and topics",
                    "documentation" => "Documentation and guides",
                    "academic" => "Academic and educational content",
                    "programming" => "Programming and development",
                    "mathematics" => "Mathematical content",
                    "physics" => "Physics content",
                    _ => "Other content"
                },
                "count": count
            }));
        }
        
        // Popular tags (top 10)
        let popular: Vec<_> = tags.iter().take(10).map(|tag| {
            serde_json::json!({
                "name": tag.get("name"),
                "count": tag.get("count"),
                "trend": "up" // Would be calculated from analytics
            })
        }).collect();
        
        let tags_context = serde_json::json!({
            "tags": tags,
            "categories": categories,
            "popular": popular
        });
        
        let tags_json = serde_json::to_string_pretty(&tags_context)?;
        std::fs::write(contexts_dir.join("tags.json"), tags_json)?;
        
        Ok(())
    }
    
    /// Generate hooks system JavaScript files
    fn generate_hooks_system(&self, output_dir: &PathBuf) -> Result<()> {
        let hooks_dir = output_dir.join("hooks");
        std::fs::create_dir_all(&hooks_dir)?;
        
        // Copy hooks from theme if they exist
        let theme_hooks_dir = Path::new("themes")
            .join(self.theme_system.current_theme())
            .join("hooks");
        
        if theme_hooks_dir.exists() {
            for entry in std::fs::read_dir(theme_hooks_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() && path.extension().map_or(false, |ext| ext == "js") {
                    let filename = path.file_name().unwrap();
                    let output_path = hooks_dir.join(filename);
                    std::fs::copy(&path, &output_path)?;
                }
            }
        } else {
            // Create default hooks if theme doesn't provide them
            self.create_default_hooks(&hooks_dir)?;
        }
        
        Ok(())
    }
    
    /// Create default hooks if theme doesn't provide them
    fn create_default_hooks(&self, hooks_dir: &PathBuf) -> Result<()> {
        // Create default useSearch.js
        let use_search_js = r#"
/**
 * Default Search Hook - Fallback implementation
 */
class useSearch {
  constructor() {
    this.isInitialized = false;
  }
  
  async init() {
    this.isInitialized = true;
  }
  
  search(query) {
    return { results: [], isLoading: false, error: null, query };
  }
}

window.PETA_HOOKS = window.PETA_HOOKS || {};
window.PETA_HOOKS.useSearch = useSearch;
"#;
        
        // Create default useNavigation.js
        let use_navigation_js = r#"
/**
 * Default Navigation Hook - Fallback implementation
 */
class useNavigation {
  constructor() {
    this.currentPath = window.location.pathname;
  }
  
  navigateTo(path) {
    window.location.href = path;
  }
  
  goBack() {
    window.history.back();
  }
}

window.PETA_HOOKS = window.PETA_HOOKS || {};
window.PETA_HOOKS.useNavigation = useNavigation;
"#;
        
        // Create default useTheme.js
        let use_theme_js = r#"
/**
 * Default Theme Hook - Fallback implementation
 */
class useTheme {
  constructor() {
    this.currentTheme = 'light';
  }
  
  setTheme(theme) {
    this.currentTheme = theme;
    document.documentElement.setAttribute('data-theme', theme);
  }
  
  toggleTheme() {
    const newTheme = this.currentTheme === 'light' ? 'dark' : 'light';
    this.setTheme(newTheme);
  }
}

window.PETA_HOOKS = window.PETA_HOOKS || {};
window.PETA_HOOKS.useTheme = useTheme;
"#;
        
        std::fs::write(hooks_dir.join("useSearch.js"), use_search_js)?;
        std::fs::write(hooks_dir.join("useNavigation.js"), use_navigation_js)?;
        std::fs::write(hooks_dir.join("useTheme.js"), use_theme_js)?;
        
        Ok(())
    }
    
    /// Load components from theme directory
    fn load_components(&mut self, _template_engine: &TemplateEngine) -> Result<()> {
        use crate::components::ComponentLoader;
        
        let theme_path = PathBuf::from(&self.config.build.theme_dir).join("default");
        let loader = ComponentLoader::new(&theme_path);        
        // Discover and load all components
        let components = loader.load_components_from_theme(&theme_path)?;
        for component in &components {
            println!("  - {}", component.name);
        }        
        // Register components
        for component in components {
            self.component_registry.register_component(component)?;
        }
        
        // Load site component configuration
        let components_config = &self.config.components;
        // Enable/disable components based on configuration
        for name in &components_config.enabled_components {
            self.component_registry.enable_component(name)?;
        }
        
        Ok(())
    }
    
    /// Helper method to strip HTML tags from content
    fn strip_html_tags(&self, html: &str) -> String {
        use regex::Regex;
        let re = Regex::new(r"<[^>]*>").unwrap();
        re.replace_all(html, "").to_string()
    }
    
    /// Estimate reading time based on content length
    fn estimate_reading_time(&self, content: &str) -> String {
        let word_count = self.strip_html_tags(content).split_whitespace().count();
        let words_per_minute = 200;
        let minutes = (word_count as f64 / words_per_minute as f64).ceil() as u32;
        
        if minutes == 1 {
            "1 min".to_string()
        } else {
            format!("{} min", minutes)
        }
    }
    
    /// Categorize a tag based on its content
    fn categorize_tag(&self, tag: &str) -> String {
        let tag_lower = tag.to_lowercase();
        
        if tag_lower.contains("tutorial") || tag_lower.contains("guide") || tag_lower.contains("documentation") {
            "documentation".to_string()
        } else if tag_lower.contains("math") || tag_lower.contains("calculus") || tag_lower.contains("integral") || tag_lower.contains("derivative") {
            "mathematics".to_string()
        } else if tag_lower.contains("physics") || tag_lower.contains("quantum") || tag_lower.contains("wave") {
            "physics".to_string()
        } else if tag_lower.contains("rust") || tag_lower.contains("python") || tag_lower.contains("javascript") || tag_lower.contains("code") {
            "programming".to_string()
        } else if tag_lower.contains("education") || tag_lower.contains("learning") || tag_lower.contains("academic") {
            "academic".to_string()
        } else {
            "general".to_string()
        }
    }
    
    /// Sort content items by date (newest first)
    fn sort_content_by_date(&self, items: &mut Vec<serde_json::Value>) {
        items.sort_by(|a, b| {
            let date_a = a.get("date").and_then(|d| d.as_str());
            let date_b = b.get("date").and_then(|d| d.as_str());
            
            match (date_a, date_b) {
                (Some(da), Some(db)) => {
                    // Parse dates and sort in descending order (newest first)
                    let parsed_a = NaiveDate::parse_from_str(da, "%Y-%m-%d");
                    let parsed_b = NaiveDate::parse_from_str(db, "%Y-%m-%d");
                    
                    match (parsed_a, parsed_b) {
                        (Ok(pa), Ok(pb)) => pb.cmp(&pa), // Reverse order for newest first
                        _ => da.cmp(&db).reverse(), // Fallback to string comparison
                    }
                }
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });
    }
}