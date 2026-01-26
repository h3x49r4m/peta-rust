//! Enhanced template engine with V4 component support

use crate::core::{Result, Error};
use crate::core::theme::Theme;
use crate::templates::{filters, functions};
use crate::components::ComponentRegistry;
use crate::content::ProcessedArticle;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use tera::{Tera, Context, Value};

/// Template engine with enhanced component support
#[derive(Clone)]
pub struct TemplateEngine {
    tera: Tera,
    theme_dir: std::path::PathBuf,
    component_registry: Option<ComponentRegistry>,
    component_renderer: Option<crate::components::renderer::ComponentRendererWrapper>,
    theme_manager: Option<crate::components::ThemeManager>,
    current_theme: Option<String>,
}

/// Collect all tags from content files
fn collect_all_tags() -> serde_json::Value {
    use std::collections::HashMap;
    use std::fs;
    
    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    
    // Define content directories to scan
    let content_dirs = [
        "_content/articles",
        "_content/books",
        "_content/snippets",
        "_content/projects"
    ];
    
    for dir in &content_dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // First check for index.rst in the directory
                    let index_path = path.join("index.rst");
                    if index_path.exists() {
                        // Read index.rst and extract tags
                        if let Ok(content) = fs::read_to_string(&index_path) {
                            extract_tags_from_content(&content, &mut tag_counts);
                        }
                    }
                    
                    // Then scan all .rst files in the directory
                    if let Ok(file_entries) = fs::read_dir(&path) {
                        for file_entry in file_entries.flatten() {
                            let file_path = file_entry.path();
                            if file_path.is_file() && 
                               file_path.extension().unwrap_or_default() == "rst" &&
                               file_path.file_name().unwrap() != "index.rst" {
                                // Read the file and extract tags
                                if let Ok(content) = fs::read_to_string(&file_path) {
                                    extract_tags_from_content(&content, &mut tag_counts);
                                }
                            }
                        }
                    }
                } else if path.is_file() && path.extension().unwrap_or_default() == "rst" {
                    // Direct .rst file in the content directory
                    if let Ok(content) = fs::read_to_string(&path) {
                        extract_tags_from_content(&content, &mut tag_counts);
                    }
                }
            }
        }
    }
    
    // Convert to JSON array sorted by count
    let mut tags: Vec<_> = tag_counts.into_iter().collect();
    tags.sort_by(|a, b| b.1.cmp(&a.1));
    
    serde_json::Value::Array(
        tags.into_iter()
            .map(|(name, count)| serde_json::json!({
                "name": name,
                "count": count
            }))
            .collect()
    )
}

// Helper function to collect tags from a specific directory
fn collect_tags_from_directory(dir_path: &str) -> serde_json::Value {
    use std::collections::HashMap;
    use std::fs;
    
    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    
    // Recursively scan the specified directory
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Recursively scan subdirectories
                if let Ok(sub_entries) = fs::read_dir(&path) {
                    for sub_entry in sub_entries.flatten() {
                        let sub_path = sub_entry.path();
                        if sub_path.is_file() && 
                           sub_path.extension().unwrap_or_default() == "rst" {
                            // Read the file and extract tags
                            if let Ok(content) = fs::read_to_string(&sub_path) {
                                extract_tags_from_content(&content, &mut tag_counts);
                            }
                        }
                    }
                }
            } else if path.is_file() && 
                      path.extension().unwrap_or_default() == "rst" {
                // Read the file and extract tags
                if let Ok(content) = fs::read_to_string(&path) {
                    extract_tags_from_content(&content, &mut tag_counts);
                }
            }
        }
    }
    
    // Convert to JSON array sorted by count
    let mut tags: Vec<_> = tag_counts.into_iter().collect();
    tags.sort_by(|a, b| b.1.cmp(&a.1));
    
    serde_json::Value::Array(
        tags.into_iter()
            .map(|(name, count)| serde_json::json!({
                "name": name,
                "count": count
            }))
            .collect()
    )
}

// Helper function to extract tags from content
fn extract_tags_from_content(content: &str, tag_counts: &mut HashMap<String, usize>) {
    // Look for tags in the frontmatter
    if let Some(start) = content.find("---") {
        if let Some(end) = content[start + 3..].find("---") {
            let frontmatter = &content[start + 3..start + 3 + end];
            // Extract tags from frontmatter
            for line in frontmatter.lines() {
                if line.trim().starts_with("tags:") {
                    let tags_line = line.trim()[6..].trim();
                    // Parse tags array
                    let tags_str = tags_line.trim_start_matches('[').trim_end_matches(']');
                    for tag in tags_str.split(',') {
                        let tag = tag.trim().trim_matches('"').trim();
                        if !tag.is_empty() {
                            *tag_counts.entry(tag.to_string()).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }
}

impl TemplateEngine {
    /// Create a new template engine
    pub fn new(theme: &Theme) -> Result<Self> {
        let mut tera = Tera::default();
        
        // Add custom filters
        filters::register(&mut tera);
        functions::register(&mut tera);
        
        // Add enhanced component functions
        Self::register_component_functions(&mut tera);
        
        // Add theme-related functions
        Self::register_theme_functions(&mut tera);
        
        // Add templates from theme directory
        Self::load_templates(&mut tera, &theme.templates_dir)?;
        
        Ok(Self { 
            tera,
            theme_dir: theme.path().to_path_buf(),
            component_registry: None,
            component_renderer: None,
            theme_manager: None,
            current_theme: None,
        })
    }
    
    /// Create a new template engine with component registry
    pub fn new_with_components(theme: &Theme, registry: ComponentRegistry) -> Result<Self> {
        let mut engine = Self::new(theme)?;
        engine.component_registry = Some(registry);
        
        // Create component renderer
        if let (Some(registry), Some(_theme_name)) = (&engine.component_registry, &engine.current_theme) {
            engine.component_renderer = Some(crate::components::renderer::ComponentRendererWrapper::new(
                engine.clone(),
                registry.clone(),
            ));
        }
        
        Ok(engine)
    }
    
    /// Register component functions
    fn register_component_functions(tera: &mut Tera) {
        // Enhanced component function with props and slots
        tera.register_function(
            "component",
            Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
                let component_name = args.get("0")
                    .or_else(|| args.get("name"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| tera::Error::msg("Component name is required"))?;
                
                // Handle both positional and named arguments
                let props = if let Some(positional_props) = args.get("1") {
                    // Positional arguments: component("name", {"prop": "value"})
                    positional_props.clone()
                } else if let Some(named_props) = args.get("props") {
                    // Named props argument: component(name="...", props={...})
                    named_props.clone()
                } else {
                    // Individual named arguments: component(name="...", title="...", description="...")
                    let mut props_map = serde_json::Map::new();
                    for (key, value) in args {
                        if key != "0" && key != "name" {
                            props_map.insert(key.clone(), value.clone());
                        }
                    }
                    Value::Object(props_map)
                };
                
                // Try to load and render the component directly
                let category = match component_name {
                    "code_block" => "atomic",
                    "navbar" => "atomic",
                    "contacts" => "atomic",
                    "tag_cloud" => "atomic",
                    "grid_card" => "atomic",
                    "content_div" => "atomic",
                    "article_toc" => "atomic",
                    "article_content" => "atomic",
                    "book_toc" => "atomic",
                    "header" => "composite",
                    "footer" => "composite",
                    "page_tags" => "composite",
                    "snippet_card_modal" => "composite",
                    "grid_cards" => "composite",
                    "article_modal" => "composite",
                    "book_modal" => "composite",
                    _ => "content",
                };
                let component_path = format!("themes/default/components/{}/{}", category, component_name);
                
                let template_path = format!("{}/{}.html", component_path, component_name);
                
                if std::path::Path::new(&template_path).exists() {
                    match std::fs::read_to_string(&template_path) {
                        Ok(template_content) => {
                            // Create a simple template engine for this component
                            let mut tera = tera::Tera::default();
                            tera.autoescape_on(vec![]); // Disable autoescape for HTML components
                            
                            // Add the main component template
                            if let Err(e) = tera.add_raw_template(component_name, &template_content) {
                                eprintln!("Failed to add template {}: {}", component_name, e);
                                return Ok(Value::String(format!("Failed to add template {}: {}", component_name, e)));
                            }
                            
                                                        
                            
                                                        // Add all component templates for nested rendering
                            let component_categories = ["atomic", "composite"];
                            for category in &component_categories {
                                let components_dir = format!("themes/default/components/{}", category);
                                if let Ok(entries) = std::fs::read_dir(&components_dir) {
                                    for entry in entries.flatten() {
                                        if let Some(component_dir_name) = entry.file_name().to_str() {
                                            let component_template_path = format!("{}/{}/{}.html", components_dir, component_dir_name, component_dir_name);
                            
                                                                        if std::path::Path::new(&component_template_path).exists() {
                            
                                                                            if let Ok(template_content) = std::fs::read_to_string(&component_template_path) {
                            
                                                                                let _ = tera.add_raw_template(component_dir_name, &template_content);
                            
                                                                            }
                            
                                                                        }
                            
                                                                    }
                            
                                                                }
                            
                                                            }
                            
                                                        }
                            
                                                        
                            
                                                        let mut context = tera::Context::new();
                            
                                                        
                            
                                                                                                        context.insert("props", &props);
                            
                                                        
                            
                                                                                                        
                            
                                                        
                            
                                                                                                        // Pass all props as top-level variables for easy access
                            
                                                        
                            
                                                                                                        if let Some(props_obj) = props.as_object() {
                            
                                                        
                            
                                                                                                            for (key, value) in props_obj {
                            
                                                        
                            
                                                                                                                context.insert(key, value);
                            
                                                        
                            
                                                                                                            }
                            
                                                        
                            
                                                                                                        }
                            
                                                        
                            
                                                                                                        
                            
                                                        
                            
                                                                                                        // If page is passed as a prop, also insert it as page
                            
                                                        
                            
                                                                                                        if let Some(page) = props.get("page") {
                            
                                                        
                            
                                                                                                            context.insert("page", page);
                            
                                                        
                            
                                                                                                        }                                                        
                                                        // Add site context with page type detection
                                                        let page_type = if component_name == "page_tags" {
                                                            if let Some(props) = props.as_object() {
                                                                if let Some(title) = props.get("title").and_then(|v| v.as_str()) {
                                                                    match title.to_lowercase().as_str() {
                                                                        "books" => "books",
                                                                        "articles" => "articles",
                                                                        "snippets" => "snippets",
                                                                        "projects" => "projects",
                                                                        _ => "default"
                                                                    }
                                                                } else {
                                                                    "default"
                                                                }
                                                            } else {
                                                                "default"
                                                            }
                                                        } else {
                                                            "default"
                                                        };
                                                        
                                                        // Collect tags based on page type
                                                        let all_tags = match page_type {
                                                            "books" => collect_tags_from_directory("_content/books"),
                                                            "articles" => collect_tags_from_directory("_content/articles"),
                                                            "snippets" => collect_tags_from_directory("_content/snippets"),
                                                            "projects" => collect_tags_from_directory("_content/projects"),
                                                            _ => collect_all_tags() // For index page and default, show all tags
                                                        };
                                                        
                                                        context.insert("site", &serde_json::json!({
                                                            "title": "Peta",
                                                            "page_type": page_type,
                                                            "all_tags": all_tags
                                                        }));
                            
                                                        
                            
                                                                                    match tera.render(component_name, &context) {
                            
                                                        
                            
                                                                                        Ok(mut rendered) => {
                            
                                                        
                            
                                                                                            // Handle nested component substitution
                            
                                                        
                            
                                                                                            if component_name == "header" {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                            // Render navbar component
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                            if let Ok(navbar_template) = std::fs::read_to_string("themes/default/components/atomic/navbar/navbar.html") {
                            
                                                        
                            
                                                                                                                                
                            
                                                        
                            
                                                                                                                                if let Ok(navbar_rendered) = Self::render_nested_component("navbar", &navbar_template, &mut tera, &context) {
                            
                                                        
                            
                                                                                                                                    
                            
                                                        
                            
                                                                                                                                    rendered = rendered.replace("<!-- Navbar component will be injected here -->\n      <div id=\"navbar-placeholder\"></div>", &navbar_rendered);
                            
                                                        
                            
                                                                                                                                    
                            
                                                        
                            
                                                                                                                                }
                            
                                                        
                            
                                                                                                                                
                            
                                                        
                            
                                                                                                                            }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                        } else if component_name == "page_tags" {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                // Render tag_cloud component with tags from page_tags props
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                if let Ok(tag_cloud_template) = std::fs::read_to_string("themes/default/components/atomic/tag_cloud/tag_cloud.html") {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            // Create a new context for tag_cloud that includes the tags
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            let mut tag_cloud_context = context.clone();
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            // Create a props object for tag_cloud containing the tags
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            let mut tag_cloud_props = serde_json::Map::new();
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            // Extract tags from page_tags props and pass them to tag_cloud
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                        if let Some(props) = props.as_object() {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                            if let Some(title) = props.get("title").and_then(|v| v.as_str()) {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                // Collect tags based on page type
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                let tags = match title.to_lowercase().as_str() {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                    "books" => collect_tags_from_directory("_content/books"),
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                    "articles" => collect_tags_from_directory("_content/articles"),
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                    "snippets" => collect_tags_from_directory("_content/snippets"),
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                    "projects" => collect_tags_from_directory("_content/projects"),
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                    _ => collect_all_tags()
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                };
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                tag_cloud_props.insert("tags".to_string(), tags);
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                            }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                            if let Some(tags) = props.get("tags") {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                tag_cloud_props.insert("tags".to_string(), tags.clone());
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                            }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            tag_cloud_context.insert("props", &tag_cloud_props);
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            if let Ok(tag_cloud_rendered) = Self::render_nested_component("tag_cloud", &tag_cloud_template, &mut tera, &tag_cloud_context) {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                rendered = rendered.replace("<!-- Tag cloud component will be injected here -->", &tag_cloud_rendered);
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                        }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                            }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                    // Handle nested component substitution for article_modal
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                    if component_name == "article_modal" {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                        // Render article_toc component
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                        if let Ok(article_toc_template) = std::fs::read_to_string("themes/default/components/atomic/article_toc/article_toc.html") {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            let mut article_toc_context = context.clone();
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            // Create props object for article_toc containing the toc
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            let mut article_toc_props = serde_json::Map::new();
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            // Extract toc from article_modal props and pass it to article_toc
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            if let Some(props) = props.as_object() {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                if let Some(toc) = props.get("toc") {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                    article_toc_props.insert("toc".to_string(), toc.clone());
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            article_toc_context.insert("props", &article_toc_props);
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            if let Ok(article_toc_rendered) = Self::render_nested_component("article_toc", &article_toc_template, &mut tera, &article_toc_context) {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                rendered = rendered.replace("<div id=\"article-toc-placeholder\"></div>", &article_toc_rendered);
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                        }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                        // Render article_content component
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                        if let Ok(article_content_template) = std::fs::read_to_string("themes/default/components/atomic/article_content/article_content.html") {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            let mut article_content_context = context.clone();
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            // Create props object for article_content containing title, content, and page (meta)
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            let mut article_content_props = serde_json::Map::new();
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            // Extract title, content, and meta from article_modal props and pass them to article_content
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            if let Some(props) = props.as_object() {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                if let Some(title) = props.get("title") {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                    article_content_props.insert("title".to_string(), title.clone());
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                if let Some(content) = props.get("content") {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                    article_content_props.insert("content".to_string(), content.clone());
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                if let Some(meta) = props.get("meta") {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                    article_content_props.insert("page".to_string(), meta.clone());
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            article_content_context.insert("props", &article_content_props);
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                        // Also insert page separately for template access
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                        if let Some(meta) = props.get("meta") {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                            article_content_context.insert("page", meta);
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                    
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        if let Ok(article_content_rendered) = Self::render_nested_component("article_content", &article_content_template, &mut tera, &article_content_context) {
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                rendered = rendered.replace("<div id=\"article-content-placeholder\"></div>", &article_content_rendered);
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                                
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                            }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                        
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                        }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                                                    }
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                            
                            
                                                        
                            
                                                                                                                            
                            
                                                        
                            
                                                                                                                                                                                                                                                    Ok(Value::String(rendered))
                            
                                                        
                            
                                                                                        },
                            
                                                        
                            
                                                                                        Err(e) => {
                            
                                                        
                            
                                                                                                                    
                            
                                                        
                            
                                                                                                                    
                            
                                                        
                            
                                                                                                                                                                                    eprintln!("Component render error for {}: {}", component_name, e);
                            
                                                        
                            
                                                                                                                                                                                    eprintln!("Kind: {:?}", e.kind);
                            
                                                        
                            
                                                                                                                                                                                    eprintln!("Props: {:?}", props);
                            
                                                        
                            
                                                                                                                                                                                    eprintln!("Context: {:?}", context);
                            
                                                        
                            
                                                                                                                    
                            
                                                        
                            
                                                                                                                    
                            
                                                        
                            
                                                                                                                                                                                    Ok(Value::String(format!("Component render error: {}", e)))
                            
                                                        
                            
                                                                                                                    
                            
                                                        
                            
                                                                                                                    
                            
                                                        
                            
                                                                                                                                                                                },
                            
                                                        
                            
                                                                                    }
                        }
                        Err(e) => Ok(Value::String(format!("Component file error: {}", e))),
                    }
                } else {
                    Ok(Value::String(format!("Component not found: {}", component_name)))
                }
            })
        );
        
        // Helper function to get component category
        tera.register_function(
            "get_component_category",
            Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
                let component_name = args.get("0")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| tera::Error::msg("Component name is required"))?;
                
                let category = match component_name {
                    "code_block" => "atomic",
                    "navbar" => "atomic",
                    "contacts" => "atomic",
                    "tag_cloud" => "atomic",
                    "tag_cloud_all" => "atomic",
                    "grid_card" => "atomic",
                    "header" => "composite",
                    "footer" => "composite",
                    "page_tags" => "composite",
                    "snippet_card_modal" => "composite",
                    "grid_cards" => "composite",
                    _ => "content",
                };
                
                Ok(Value::String(category.to_string()))
            })
        );
        
        // Component styles function
        tera.register_function(
            "component_styles",
            Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
                let component_names = args.get("component_names")
                    .or_else(|| args.get("0"))
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| tera::Error::msg("Component names array is required"))?;
                
                let mut styles = String::new();
                
                // Load global theme CSS first (now using inline styles in templates)
                // Note: CSS is now inline in templates, so this section is commented out
                // if let Ok(global_css) = std::fs::read_to_string("themes/default/css/main.css") {
                //     styles.push_str("/* Global Theme Styles */\n");
                //     styles.push_str(&global_css);
                //     styles.push('\n');
                // }
                
                // Load component-specific CSS
                for component in component_names {
                    if let Some(name) = component.as_str() {
                        styles.push_str(&format!("/* Styles for component: {} */\n", name));
                        
                        // Try to load CSS from component directory
                        let css_paths = [
                            format!("themes/default/components/composite/{}/{}.css", name, name),
                            format!("themes/default/components/content/{}/{}.css", name, name),
                            format!("themes/default/components/layout/{}/{}.css", name, name),
                            format!("themes/default/components/atomic/{}/{}.css", name, name),
                        ];
                        
                        for css_path in &css_paths {
                            if std::path::Path::new(css_path).exists() {
                                if let Ok(css_content) = std::fs::read_to_string(css_path) {
                                    styles.push_str(&css_content);
                                    styles.push('\n');
                                    break;
                                }
                            }
                        }
                    }
                }
                
                Ok(Value::String(styles))
            })
        );
        
        // Component scripts function
        tera.register_function(
            "component_scripts",
            Box::new(|args: &HashMap<String, Value>| -> tera::Result<Value> {
                let component_names = args.get("component_names")
                    .or_else(|| args.get("0"))
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| tera::Error::msg("Component names array is required"))?;
                
                let mut scripts = String::new();
                for component in component_names {
                    if let Some(name) = component.as_str() {
                        // Load the actual JS file if it exists
                        let js_path = format!("{}/themes/default/components/{}/{}/{}.js", 
                            std::env::current_dir().unwrap().to_string_lossy(),
                            match name {
                                "code_block" => "atomic",
                                "navbar" => "atomic",
                                "contacts" => "atomic",
                                "tag_cloud" => "atomic",
                                "grid_card" => "atomic",
                                "content_div" => "atomic",
                                "header" => "composite",
                                "footer" => "composite",
                                "page_tags" => "composite",
                                "snippet_card_modal" => "composite",
                                "grid_cards" => "composite",
                                _ => "atomic",
                            }, name, name);
                        
                        if let Ok(js_content) = std::fs::read_to_string(&js_path) {
                            scripts.push_str(&format!("// Loaded from: {}\n", js_path));
                            scripts.push_str(&js_content);
                            scripts.push('\n');
                        } else {
                            // Fallback to placeholder script
                            scripts.push_str(&format!("// Script for component: {} (file not found: {})\n", name, js_path));
                            scripts.push_str(&format!("document.addEventListener('DOMContentLoaded', () => {{\n"));
                            scripts.push_str(&format!("  // Initialize {} component\n", name));
                            scripts.push_str(&format!("}});\n"));
                        }
                    }
                }
                
                Ok(Value::String(scripts))
            })
        );
    }
    
    /// Register theme functions
    fn register_theme_functions(tera: &mut Tera) {
        tera.register_function(
            "theme_var",
            Box::new(|args: &HashMap<String, Value>| -> tera::Result<Value> {
                let key = args.get("0")
                    .or_else(|| args.get("key"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| tera::Error::msg("Theme variable key is required"))?;
                
                // Return CSS custom property
                Ok(Value::String(format!("var(--theme-{})", key)))
            })
        );
        
        tera.register_function(
            "asset_url",
            Box::new(|args: &HashMap<String, Value>| -> tera::Result<Value> {
                let path = args.get("0")
                    .or_else(|| args.get("path"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| tera::Error::msg("Asset path is required"))?;
                
                let clean_path = path.trim_start_matches('/');
                let url = format!("/assets/{}", clean_path);
                Ok(Value::String(url))
            })
        );
        
        tera.register_function(
            "url",
            Box::new(|args: &HashMap<String, Value>| -> tera::Result<Value> {
                let path = args.get("0")
                    .or_else(|| args.get("path"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| tera::Error::msg("URL path is required"))?;
                
                let clean_path = path.trim_start_matches('/');
                let url = if clean_path.starts_with("http") {
                    clean_path.to_string()
                } else {
                    format!("/{}", clean_path)
                };
                
                Ok(Value::String(url))
            })
        );
    }
    
    /// Load templates from theme directory
    fn load_templates(tera: &mut Tera, templates_dir: &PathBuf) -> Result<()> {
        if !templates_dir.exists() {
            return Ok(());
        }
        
        // First, collect all template paths and contents
        let mut templates = Vec::new();
        
        for entry in walkdir::WalkDir::new(templates_dir) {
            let entry = entry.map_err(|e| Error::template(e.to_string()))?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("html") {
                let template_name = path.strip_prefix(templates_dir)
                    .map_err(|_| Error::template("Invalid template path".to_string()))?
                    .to_str()
                    .ok_or_else(|| Error::template("Invalid template name".to_string()))?;
                    
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| Error::template(e.to_string()))?;
                    
                templates.push((template_name.to_string(), content));
            }
        }
        
        // Sort templates to ensure base.html is loaded first
        templates.sort_by(|a, b| {
            // base.html should come first
            if a.0 == "base.html" && b.0 != "base.html" {
                std::cmp::Ordering::Less
            } else if a.0 != "base.html" && b.0 == "base.html" {
                std::cmp::Ordering::Greater
            } else {
                a.0.cmp(&b.0)
            }
        });
        
        // Add all templates to Tera
        for (name, content) in templates {
            tera.add_raw_template(&name, &content)
                .map_err(|e| Error::template(e.to_string()))?;
        }
        
        // Build inheritance chains after all templates are loaded
        tera.build_inheritance_chains()
            .map_err(|e| Error::template(e.to_string()))?;
        
        Ok(())
    }
    
    /// Set component registry
    pub fn set_component_registry(&mut self, registry: ComponentRegistry) {
        self.component_registry = Some(registry);
    }
    
    /// Set theme manager and current theme
    pub fn set_theme_manager(&mut self, theme_manager: crate::components::ThemeManager, theme_name: String) {
        self.theme_manager = Some(theme_manager);
        self.current_theme = Some(theme_name);
    }
    
    /// Get current theme name
    pub fn current_theme(&self) -> Option<&str> {
        self.current_theme.as_deref()
    }
    
    /// Get theme variable
    pub fn get_theme_variable(&self, key: &str) -> Option<String> {
        if let (Some(theme_manager), Some(current_theme)) = (&self.theme_manager, &self.current_theme) {
            theme_manager.get_theme_variables(current_theme).get(key).cloned()
        } else {
            None
        }
    }
    
    /// Get all theme variables
    pub fn get_theme_variables(&self) -> HashMap<String, String> {
        if let (Some(theme_manager), Some(current_theme)) = (&self.theme_manager, &self.current_theme) {
            theme_manager.get_theme_variables(current_theme)
        } else {
            HashMap::new()
        }
    }
    
    /// Get theme directory
    pub fn theme_dir(&self) -> &Path {
        &self.theme_dir
    }
    
    /// Get theme directory as PathBuf
    pub fn theme_dir_buf(&self) -> PathBuf {
        self.theme_dir.clone()
    }
    
    /// Render a template with context and component processing
    pub fn render(&self, template: &str, context: &Context) -> Result<String> {
        // Create enhanced context with component information
        let mut enhanced_context = context.clone();
        
        // Add component information to context
        if let Some(registry) = &self.component_registry {
            let enabled_components: Vec<Value> = registry.get_enabled_components()
                .iter()
                .map(|c| {
                    serde_json::json!({
                        "name": c.name,
                        "version": c.version,
                        "category": c.category,
                        "props": c.default_config,
                    })
                })
                .collect();
            
            enhanced_context.insert("components", &serde_json::json!({
                "enabled": enabled_components,
                "registry": registry.get_all_components(),
            }));
        }
        
        // Add theme variables
        let theme_vars = self.get_theme_variables();
        enhanced_context.insert("theme_variables", &theme_vars);
        
        // Render template
        self.tera.render(template, &enhanced_context)
            .map_err(|e| {
                eprintln!("Template rendering error for '{}': {}", template, e);
                Error::template(e.to_string())
            })
    }
    
    /// Render a component directly
    pub fn render_component(&self, component_name: &str, props: &Value, _slots: &HashMap<String, String>) -> Result<String> {
        if let Some(renderer) = &self.component_renderer {
            renderer.render_component(component_name, props)
        } else {
            Err(Error::Component("Component renderer not initialized".to_string()))
        }
    }
    
    /// Get component renderer
    pub fn component_renderer(&self) -> Option<&crate::components::renderer::ComponentRendererWrapper> {
        self.component_renderer.as_ref()
    }
    
    /// Add a template string
    pub fn add_template(&mut self, name: &str, content: &str) -> Result<()> {
        self.tera.add_raw_template(name, content)
            .map_err(|e| Error::template(e.to_string()))
    }
    
    /// Get component registry
    pub fn component_registry(&self) -> Option<&ComponentRegistry> {
        self.component_registry.as_ref()
    }
    
    /// Render article with math detection
    pub fn render_article_with_math(&self, article: &ProcessedArticle) -> Result<String> {
        let mut context = Context::new();
        
        context.insert("content", &article.content);
        context.insert("metadata", &article.metadata);
        context.insert("has_math_formulas", &article.has_math_formulas);
        context.insert("math_formula_count", &article.math_formula_count);
        
        if let Some(toc) = &article.toc {
            context.insert("toc", toc);
        }
        
        // If has math formulas, inject render script
        if let Some(math_script) = &article.math_render_script {
            context.insert("math_render_script", math_script);
        }
        
        self.render("article.html", &context)
    }
    
    /// Helper function to render nested components
    fn render_nested_component(
        component_name: &str,
        template_content: &str,
        tera: &mut tera::Tera,
        context: &tera::Context,
    ) -> Result<String> {
        if let Err(e) = tera.add_raw_template(component_name, template_content) {
            return Err(Error::template(format!("Failed to add nested template {}: {}", component_name, e)));
        }
        
        match tera.render(component_name, context) {
            Ok(rendered) => Ok(rendered),
            Err(e) => Err(Error::template(format!("Failed to render nested component {}: {}", component_name, e))),
        }
    }
}





/// Template filter trait
pub trait TemplateFilter: Send + Sync {
    fn filter(&self, value: &tera::Value, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value>;
}

/// Template function trait
pub trait TemplateFunction: Send + Sync {
    fn call(&self, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value>;
}