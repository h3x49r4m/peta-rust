//! Enhanced template engine with V4 component support

use crate::core::{Result, Error};
use crate::core::theme::Theme;
use crate::templates::{filters, functions};
use crate::components::{ComponentRegistry, ComponentManager};
use crate::content::ProcessedArticle;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tera::{Tera, Context, Value};

/// Tag collector with caching
#[derive(Clone)]
struct TagCollector {
    cache: Arc<RwLock<HashMap<String, Value>>>,
}

impl TagCollector {
    fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn collect_all(&self) -> Value {
        let key = "all".to_string();
        if let Ok(cache) = self.cache.read() {
            if let Some(cached) = cache.get(&key) {
                return cached.clone();
            }
        }
        let result = self.scan_directories(&["_content/articles", "_content/books", "_content/snippets", "_content/projects"]);
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(key, result.clone());
        }
        result
    }

    fn collect_from_directory(&self, dir_path: &str) -> Value {
        if let Ok(cache) = self.cache.read() {
            if let Some(cached) = cache.get(dir_path) {
                return cached.clone();
            }
        }
        let result = self.scan_directory(dir_path);
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(dir_path.to_string(), result.clone());
        }
        result
    }

    fn scan_directories(&self, dirs: &[&str]) -> Value {
        use std::collections::HashMap;
        let mut tag_counts: HashMap<String, usize> = HashMap::new();
        for dir in dirs {
            self.scan_directory_recursive(dir, &mut tag_counts);
        }
        self.to_json_array(tag_counts)
    }

    fn scan_directory(&self, dir_path: &str) -> Value {
        use std::collections::HashMap;
        let mut tag_counts: HashMap<String, usize> = HashMap::new();
        self.scan_directory_recursive(dir_path, &mut tag_counts);
        self.to_json_array(tag_counts)
    }

    fn scan_directory_recursive(&self, dir_path: &str, tag_counts: &mut HashMap<String, usize>) {
        use std::fs;
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let index_path = path.join("index.rst");
                    if index_path.exists() {
                        if let Ok(content) = fs::read_to_string(&index_path) {
                            self.extract_tags(&content, tag_counts);
                        }
                    }
                    if let Ok(file_entries) = fs::read_dir(&path) {
                        for file_entry in file_entries.flatten() {
                            let file_path = file_entry.path();
                            if file_path.is_file() && 
                               file_path.extension().unwrap_or_default() == "rst" &&
                               file_path.file_name().unwrap() != "index.rst" {
                                if let Ok(content) = fs::read_to_string(&file_path) {
                                    self.extract_tags(&content, tag_counts);
                                }
                            }
                        }
                    }
                } else if path.is_file() && path.extension().unwrap_or_default() == "rst" {
                    if let Ok(content) = fs::read_to_string(&path) {
                        self.extract_tags(&content, tag_counts);
                    }
                }
            }
        }
    }

    fn extract_tags(&self, content: &str, tag_counts: &mut HashMap<String, usize>) {
        if let Some(start) = content.find("---") {
            if let Some(end) = content[start + 3..].find("---") {
                let frontmatter = &content[start + 3..start + 3 + end];
                for line in frontmatter.lines() {
                    if line.trim().starts_with("tags:") {
                        let tags_line = line.trim()[6..].trim();
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

    fn to_json_array(&self, tag_counts: HashMap<String, usize>) -> Value {
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
}

impl Default for TagCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Template cache for component templates
#[derive(Clone)]
struct TemplateCache {
    cache: Arc<RwLock<HashMap<String, String>>>,
}

impl TemplateCache {
    fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn get(&self, path: &str) -> Option<String> {
        if let Ok(cache) = self.cache.read() {
            cache.get(path).cloned()
        } else {
            None
        }
    }

    fn load(&self, path: &str) -> Result<String> {
        if let Some(cached) = self.get(path) {
            return Ok(cached);
        }
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::template(format!("Failed to read template {}: {}", path, e)))?;
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(path.to_string(), content.clone());
        }
        Ok(content)
    }
}

impl Default for TemplateCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Template engine with enhanced component support
#[derive(Clone)]
pub struct TemplateEngine {
    tera: Tera,
    theme_dir: PathBuf,
    component_registry: Option<ComponentRegistry>,
    #[allow(dead_code)]
    component_manager: Arc<RwLock<ComponentManager>>,
    component_renderer: Option<crate::components::renderer::ComponentRendererWrapper>,
    theme_manager: Option<crate::components::ThemeManager>,
    current_theme: Option<String>,
    #[allow(dead_code)]
    tag_collector: TagCollector,
    #[allow(dead_code)]
    template_cache: TemplateCache,
}

impl TemplateEngine {
    /// Create a new template engine
    pub fn new(theme: &Theme) -> Result<Self> {
        let mut tera = Tera::default();
        filters::register(&mut tera);
        functions::register(&mut tera);
        
        let theme_dir = theme.path().to_path_buf();
        let component_manager = Arc::new(RwLock::new(ComponentManager::new(&theme_dir)));
        
        // Initialize component manager to discover components
        if let Ok(mut manager) = component_manager.write() {
            let _ = manager.initialize();
        }
        
        Self::register_component_functions(&mut tera, &component_manager);
        Self::register_theme_functions(&mut tera);
        Self::load_templates(&mut tera, &theme.templates_dir)?;
        
        Ok(Self { 
            tera,
            theme_dir,
            component_registry: None,
            component_manager,
            component_renderer: None,
            theme_manager: None,
            current_theme: None,
            tag_collector: TagCollector::default(),
            template_cache: TemplateCache::default(),
        })
    }

    /// Create a new template engine with component registry
    pub fn new_with_components(theme: &Theme, registry: ComponentRegistry) -> Result<Self> {
        let mut engine = Self::new(theme)?;
        engine.component_registry = Some(registry);
        if let (Some(registry), Some(_theme_name)) = (&engine.component_registry, &engine.current_theme) {
            engine.component_renderer = Some(crate::components::renderer::ComponentRendererWrapper::new(
                engine.clone(),
                registry.clone(),
            ));
        }
        Ok(engine)
    }

    /// Register component functions
    fn register_component_functions(tera: &mut Tera, component_manager: &Arc<RwLock<ComponentManager>>) {
        let tag_collector = Arc::new(RwLock::new(TagCollector::new()));
        let template_cache = Arc::new(RwLock::new(TemplateCache::new()));
        let component_manager_clone = Arc::clone(component_manager);
        let component_manager_clone2 = Arc::clone(component_manager);
        let component_manager_clone3 = Arc::clone(component_manager);
        let component_manager_clone4 = Arc::clone(component_manager);
        let component_manager_clone5 = Arc::clone(component_manager);
        let theme_dir_clone = Arc::new(RwLock::new(PathBuf::from("themes/default")));
        let theme_dir_clone2 = Arc::clone(&theme_dir_clone);
        let theme_dir_clone3 = Arc::clone(&theme_dir_clone);

        tera.register_function(
            "component",
            Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
                let component_name = args.get("0")
                    .or_else(|| args.get("name"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| tera::Error::msg("Component name is required"))?;

                let props = Self::extract_props(args);
                
                // Get category from component manager
                let category = if let Ok(manager) = component_manager_clone.read() {
                    manager.get_component_category(component_name).unwrap_or_else(|| "content".to_string())
                } else {
                    "content".to_string()
                };

                // Get theme_dir from closure
                let theme_dir = if let Ok(dir) = theme_dir_clone.read() {
                    dir.clone()
                } else {
                    PathBuf::from("themes/default")
                };

                let component_path = theme_dir.join("components").join(&category).join(component_name);
                let template_path = component_path.join(format!("{}.html", component_name));

                // Skip code_block as it's handled by Rust rendering
                if component_name == "code_block" {
                    return Ok(Value::String(String::new()));
                }

                if !template_path.exists() {
                    return Ok(Value::String(format!("Component not found: {}", component_name)));
                }

                let template_content = std::fs::read_to_string(&template_path)
                    .map_err(|e| tera::Error::msg(format!("Failed to read component: {}", e)))?;

                let mut nested_tera = tera::Tera::default();
                nested_tera.autoescape_on(vec![]);

                Self::load_component_templates_for_tera(&mut nested_tera, &theme_dir)?;
                Self::register_component_function_for_tera(
                    &mut nested_tera,
                    &component_manager_clone,
                    &tag_collector,
                    &template_cache,
                    &theme_dir,
                );
                nested_tera.add_raw_template(component_name, &template_content)?;

                let context = Self::build_component_context(component_name, &props, &tag_collector);

                match nested_tera.render(component_name, &context) {
                    Ok(mut rendered) => {
                        rendered = Self::handle_nested_components(component_name, &rendered, &props, &tag_collector, &template_cache, &theme_dir, &component_manager_clone2)
                            .map_err(|e| tera::Error::msg(e.to_string()))?;
                        Ok(Value::String(rendered))
                    }
                    Err(e) => {
                        eprintln!("Component render error for {}: {}", component_name, e);
                        eprintln!("Props: {:?}", props);
                        Ok(Value::String(format!("Component render error: {}", e)))
                    }
                }
            })
        );

        tera.register_function(
            "get_component_category",
            Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
                let component_name = args.get("0")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| tera::Error::msg("Component name is required"))?;
                let category = if let Ok(manager) = component_manager_clone4.read() {
                    manager.get_component_category(component_name).unwrap_or_else(|| "content".to_string())
                } else {
                    "content".to_string()
                };
                Ok(Value::String(category))
            })
        );

        tera.register_function(
            "component_styles",
            Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
                let component_names = args.get("component_names")
                    .or_else(|| args.get("0"))
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| tera::Error::msg("Component names array is required"))?;

                let mut styles = String::new();
                let theme_dir = if let Ok(dir) = theme_dir_clone3.read() {
                    dir.clone()
                } else {
                    PathBuf::from("themes/default")
                };

                for component in component_names {
                    if let Some(name) = component.as_str() {
                        // Skip code_block as it's handled by Rust generators
                        if name == "code_block" {
                            continue;
                        }

                        styles.push_str(&format!("/* Styles for component: {} */\n", name));

                        // Get component category from manager
                        let category = if let Ok(manager) = component_manager_clone5.read() {
                            manager.get_component_category(name).unwrap_or_else(|| "atomic".to_string())
                        } else {
                            "atomic".to_string()
                        };

                        let css_path = theme_dir.join("components").join(&category).join(name).join(format!("{}.css", name));

                        if css_path.exists() {
                            if let Ok(css_content) = std::fs::read_to_string(&css_path) {
                                styles.push_str(&css_content);
                                styles.push('\n');
                            }
                        }
                    }
                }
                Ok(Value::String(styles))
            })
        );

        tera.register_function(
            "component_scripts",
            Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
                let component_names = args.get("component_names")
                    .or_else(|| args.get("0"))
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| tera::Error::msg("Component names array is required"))?;

                let mut scripts = String::new();
                for component in component_names {
                    if let Some(name) = component.as_str() {
                        // Skip code_block as it's handled by Rust generators
                        if name == "code_block" {
                            continue;
                        }
                        let category = if let Ok(manager) = component_manager_clone3.read() {
                            manager.get_component_category(name).unwrap_or_else(|| "atomic".to_string())
                        } else {
                            "atomic".to_string()
                        };
                        let theme_dir = if let Ok(dir) = theme_dir_clone2.read() {
                            dir.clone()
                        } else {
                            PathBuf::from("themes/default")
                        };
                        let js_path = theme_dir.join("components").join(&category).join(name).join(format!("{}.js", name));

                        if let Ok(js_content) = std::fs::read_to_string(&js_path) {
                            scripts.push_str(&format!("// Loaded from: {}\n", js_path.display()));
                            scripts.push_str(&js_content);
                            scripts.push('\n');
                        } else {
                            scripts.push_str(&format!("// Script for component: {} (file not found: {})\n", name, js_path.display()));
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

    fn extract_props(args: &HashMap<String, Value>) -> Value {
        if let Some(positional_props) = args.get("1") {
            positional_props.clone()
        } else if let Some(named_props) = args.get("props") {
            named_props.clone()
        } else {
            let mut props_map = serde_json::Map::new();
            for (key, value) in args {
                if key != "0" && key != "name" {
                    props_map.insert(key.clone(), value.clone());
                }
            }
            Value::Object(props_map)
        }
    }

    #[allow(dead_code)]
    fn load_component_templates(tera: &mut Tera) -> Result<()> {
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
        Ok(())
    }

    fn load_component_templates_for_tera(tera: &mut tera::Tera, theme_dir: &Path) -> tera::Result<()> {
        let component_categories = ["atomic", "composite"];
        for category in &component_categories {
            let components_dir = theme_dir.join("components").join(category);
            if let Ok(entries) = std::fs::read_dir(&components_dir) {
                for entry in entries.flatten() {
                    if let Some(component_dir_name) = entry.file_name().to_str() {
                        let component_template_path = components_dir.join(component_dir_name).join(format!("{}.html", component_dir_name));
                        if component_template_path.exists() {
                            if let Ok(template_content) = std::fs::read_to_string(&component_template_path) {
                                let _ = tera.add_raw_template(component_dir_name, &template_content);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Register component function in a Tera instance for nested component rendering
    fn register_component_function_for_tera(
        tera: &mut tera::Tera,
        component_manager: &Arc<RwLock<ComponentManager>>,
        tag_collector: &Arc<RwLock<TagCollector>>,
        template_cache: &Arc<RwLock<TemplateCache>>,
        theme_dir: &Path,
    ) {
        let component_manager_clone = Arc::clone(component_manager);
        let component_manager_clone2 = Arc::clone(component_manager);
        let tag_collector_clone = Arc::clone(tag_collector);
        let template_cache_clone = Arc::clone(template_cache);
        let theme_dir_clone = theme_dir.to_path_buf();

        tera.register_function(
            "component",
            Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
                let component_name = args.get("0")
                    .or_else(|| args.get("name"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| tera::Error::msg("Component name is required"))?;

                let props = Self::extract_props(args);
                
                let category = if let Ok(manager) = component_manager_clone.read() {
                    manager.get_component_category(component_name).unwrap_or_else(|| "content".to_string())
                } else {
                    "content".to_string()
                };

                let component_path = theme_dir_clone.join("components").join(&category).join(component_name);
                let template_path = component_path.join(format!("{}.html", component_name));

                if component_name == "code_block" {
                    return Ok(Value::String(String::new()));
                }

                if !template_path.exists() {
                    return Ok(Value::String(format!("Component not found: {}", component_name)));
                }

                let template_content = std::fs::read_to_string(&template_path)
                    .map_err(|e| tera::Error::msg(format!("Failed to read component: {}", e)))?;

                let mut nested_tera = tera::Tera::default();
                nested_tera.autoescape_on(vec![]);

                Self::load_component_templates_for_tera(&mut nested_tera, &theme_dir_clone)?;
                Self::register_component_function_for_tera(
                    &mut nested_tera,
                    &component_manager_clone,
                    &tag_collector_clone,
                    &template_cache_clone,
                    &theme_dir_clone,
                );
                nested_tera.add_raw_template(component_name, &template_content)?;

                let context = Self::build_component_context(component_name, &props, &tag_collector_clone);

                match nested_tera.render(component_name, &context) {
                    Ok(mut rendered) => {
                        rendered = Self::handle_nested_components(component_name, &rendered, &props, &tag_collector_clone, &template_cache_clone, &theme_dir_clone, &component_manager_clone2)
                            .map_err(|e| tera::Error::msg(e.to_string()))?;
                        Ok(Value::String(rendered))
                    }
                    Err(e) => {
                        eprintln!("Component render error for {}: {}", component_name, e);
                        eprintln!("Props: {:?}", props);
                        Ok(Value::String(format!("Component render error: {}", e)))
                    }
                }
            })
        );
    }

    fn build_component_context(
        component_name: &str,
        props: &Value,
        tag_collector: &Arc<RwLock<TagCollector>>,
    ) -> Context {
        let mut context = Context::new();
        context.insert("props", props);

        if let Some(props_obj) = props.as_object() {
            for (key, value) in props_obj {
                context.insert(key, value);
            }
        }

        if let Some(page) = props.get("page") {
            context.insert("page", page);
        }

        // Pass book_toc and book_title to book_toc component
        if component_name == "book_toc" {
            if let Some(props_obj) = props.as_object() {
                if let Some(book_toc) = props_obj.get("book_toc") {
                    context.insert("book_toc", book_toc);
                }
                if let Some(book_title) = props_obj.get("book_title") {
                    context.insert("book_title", book_title);
                }
            }
        }

        let page_type = Self::detect_page_type(component_name, props);
        let all_tags = Self::get_tags_for_page_type(&page_type, tag_collector);

        context.insert("site", &serde_json::json!({
            "title": "Peta",
            "page_type": page_type,
            "all_tags": all_tags
        }));

        context
    }

    fn detect_page_type(component_name: &str, props: &Value) -> String {
        if component_name == "page_tags" {
            if let Some(props) = props.as_object() {
                if let Some(title) = props.get("title").and_then(|v| v.as_str()) {
                    return match title.to_lowercase().as_str() {
                        "books" => "books".to_string(),
                        "articles" => "articles".to_string(),
                        "snippets" => "snippets".to_string(),
                        "projects" => "projects".to_string(),
                        _ => "default".to_string()
                    };
                }
            }
        }
        "default".to_string()
    }

    fn get_tags_for_page_type(page_type: &str, tag_collector: &Arc<RwLock<TagCollector>>) -> Value {
        if let Ok(collector) = tag_collector.read() {
            match page_type {
                "books" => collector.collect_from_directory("_content/books"),
                "articles" => collector.collect_from_directory("_content/articles"),
                "snippets" => collector.collect_from_directory("_content/snippets"),
                "projects" => collector.collect_from_directory("_content/projects"),
                _ => collector.collect_all(),
            }
        } else {
            serde_json::Value::Array(vec![])
        }
    }

    fn handle_nested_components(
        component_name: &str,
        rendered: &str,
        props: &Value,
        tag_collector: &Arc<RwLock<TagCollector>>,
        template_cache: &Arc<RwLock<TemplateCache>>,
        theme_dir: &Path,
        component_manager: &Arc<RwLock<ComponentManager>>,
    ) -> Result<String> {
        let mut result = rendered.to_string();

        match component_name {
            "header" => {
                // Skip processing if header already uses component() function calls
                if !result.contains("component(name=") {
                    result = Self::render_nested_component(
                        "navbar",
                        "<!-- Navbar component will be injected here -->\n      <div id=\"navbar-placeholder\"></div>",
                        &result,
                        props,
                        template_cache,
                        theme_dir,
                        component_manager,
                    )?;
                }
            }
            "page_tags" => {
                result = Self::render_tag_cloud_nested(&result, props, tag_collector, template_cache, theme_dir, component_manager)?;
            }
            "article_modal" => {
                result = Self::render_article_modal_nested(&result, props, template_cache, theme_dir, component_manager)?;
            }
            "project_modal" => {
                result = Self::render_project_modal_nested(&result, props, template_cache, theme_dir, component_manager)?;
            }
            _ => {}
        }

        Ok(result)
    }

    fn render_nested_component(
        component_name: &str,
        placeholder: &str,
        rendered: &str,
        props: &Value,
        template_cache: &Arc<RwLock<TemplateCache>>,
        theme_dir: &Path,
        component_manager: &Arc<RwLock<ComponentManager>>,
    ) -> Result<String> {
        let category = if let Ok(manager) = component_manager.read() {
            manager.get_component_category(component_name).unwrap_or_else(|| "atomic".to_string())
        } else {
            "atomic".to_string()
        };
        
        let template_path = theme_dir.join("components").join(&category).join(component_name).join(format!("{}.html", component_name));
        if !template_path.exists() {
            return Ok(rendered.to_string());
        }

        let template_content = if let Ok(cache) = template_cache.read() {

                            cache.load(template_path.to_str().unwrap())?

                        } else {

                            std::fs::read_to_string(&template_path)?

                        };

        

                let mut nested_tera = tera::Tera::default();

                nested_tera.autoescape_on(vec![]);

                Self::load_component_templates_for_tera(&mut nested_tera, theme_dir)?;

                // Note: We don't register component function here since nested components use placeholders

                nested_tera.add_raw_template(component_name, &template_content)?;

        

                let context = Self::build_nested_context(component_name, props);

        

                let nested_rendered = nested_tera.render(component_name, &context)

                    .map_err(|e| Error::template(format!("Failed to render nested component {}: {}", component_name, e)))?;

        

                Ok(rendered.replace(placeholder, &nested_rendered))
    }

    fn render_tag_cloud_nested(
        rendered: &str,
        props: &Value,
        tag_collector: &Arc<RwLock<TagCollector>>,
        template_cache: &Arc<RwLock<TemplateCache>>,
        theme_dir: &Path,
        component_manager: &Arc<RwLock<ComponentManager>>,
    ) -> Result<String> {
        // Get component category from manager
        let category = if let Ok(manager) = component_manager.read() {
            manager.get_component_category("tag_cloud").unwrap_or_else(|| "atomic".to_string())
        } else {
            "atomic".to_string()
        };

        let template_path = theme_dir.join("components").join(&category).join("tag_cloud").join("tag_cloud.html");
        if !template_path.exists() {
            return Ok(rendered.to_string());
        }

        let template_content = if let Ok(cache) = template_cache.read() {
            cache.load(template_path.to_str().unwrap())?
        } else {
            std::fs::read_to_string(&template_path)?
        };

        let mut tag_cloud_props = serde_json::Map::new();
        if let Some(props_obj) = props.as_object() {
            if let Some(_title) = props_obj.get("title").and_then(|v| v.as_str()) {
                let tags = Self::get_tags_for_page_type(&Self::detect_page_type("page_tags", props), tag_collector);
                tag_cloud_props.insert("tags".to_string(), tags);
            }
            if let Some(tags) = props_obj.get("tags") {
                tag_cloud_props.insert("tags".to_string(), tags.clone());
            }
        }

        let mut tera = tera::Tera::default();
        tera.autoescape_on(vec![]);
        Self::load_component_templates_for_tera(&mut tera, theme_dir)?;
        tera.add_raw_template("tag_cloud", &template_content)?;

        let mut context = Context::new();
        context.insert("props", &Value::Object(tag_cloud_props.clone()));

        let nested_rendered = tera.render("tag_cloud", &context)
            .map_err(|e| Error::template(format!("Failed to render tag_cloud: {}", e)))?;

        Ok(rendered.replace("<!-- Tag cloud component will be injected here -->", &nested_rendered))
    }

    fn render_article_modal_nested(
            rendered: &str,
            props: &Value,
            template_cache: &Arc<RwLock<TemplateCache>>,
            theme_dir: &Path,
            component_manager: &Arc<RwLock<ComponentManager>>,
        ) -> Result<String> {
            let mut result = rendered.to_string();

            // Render article_toc
            let toc_category = if let Ok(manager) = component_manager.read() {
                manager.get_component_category("article_toc").unwrap_or_else(|| "atomic".to_string())
            } else {
                "atomic".to_string()
            };
            let toc_template_path = theme_dir.join("components").join(&toc_category).join("article_toc").join("article_toc.html");
            if toc_template_path.exists() {
                let template_content = if let Ok(cache) = template_cache.read() {
                    cache.load(toc_template_path.to_str().unwrap())?
                } else {
                    std::fs::read_to_string(&toc_template_path)?
                };
    
                let mut toc_props = serde_json::Map::new();
                if let Some(props) = props.as_object() {
                    if let Some(toc) = props.get("toc") {
                        toc_props.insert("toc".to_string(), toc.clone());
                    }
                    if let Some(title) = props.get("title") {
                        toc_props.insert("title".to_string(), title.clone());
                    }
                }
    
                let mut tera = tera::Tera::default();
                tera.autoescape_on(vec![]);
                Self::load_component_templates_for_tera(&mut tera, theme_dir)?;
                tera.add_raw_template("article_toc", &template_content)?;            let mut context = Context::new();
            context.insert("props", &Value::Object(toc_props.clone()));

            let nested_rendered = tera.render("article_toc", &context)
                .map_err(|e| Error::template(format!("Failed to render article_toc: {}", e)))?;

            result = result.replace("<div id=\"article-toc-placeholder\"></div>", &nested_rendered);
        }

        // Render article_content
            let content_category = if let Ok(manager) = component_manager.read() {
                manager.get_component_category("article_content").unwrap_or_else(|| "atomic".to_string())
            } else {
                "atomic".to_string()
            };
            let content_template_path = theme_dir.join("components").join(&content_category).join("article_content").join("article_content.html");
            if content_template_path.exists() {
                    let template_content = if let Ok(cache) = template_cache.read() {
                        cache.load(content_template_path.to_str().unwrap())?
                    } else {
                        std::fs::read_to_string(&content_template_path)?
                    };
        
                    let mut content_props = serde_json::Map::new();
                    if let Some(props) = props.as_object() {
                        if let Some(title) = props.get("title") {
                            content_props.insert("title".to_string(), title.clone());
                        }
                        if let Some(content) = props.get("content") {
                            content_props.insert("content".to_string(), content.clone());
                        }
                        if let Some(meta) = props.get("meta") {
                            content_props.insert("page".to_string(), meta.clone());
                        }
                    }
        
                    let mut tera = tera::Tera::default();
                    tera.autoescape_on(vec![]);
                    Self::load_component_templates_for_tera(&mut tera, theme_dir)?;
                    tera.add_raw_template("article_content", &template_content)?;            let mut context = Context::new();
            context.insert("props", &Value::Object(content_props.clone()));
            if let Some(meta) = props.get("meta") {
                context.insert("page", meta);
            }

            let nested_rendered = tera.render("article_content", &context)
                .map_err(|e| Error::template(format!("Failed to render article_content: {}", e)))?;

            result = result.replace("<div id=\"article-content-placeholder\"></div>", &nested_rendered);
        }

        Ok(result)
    }

    fn render_project_modal_nested(
            rendered: &str,
            props: &Value,
            template_cache: &Arc<RwLock<TemplateCache>>,
            theme_dir: &Path,
            component_manager: &Arc<RwLock<ComponentManager>>,
        ) -> Result<String> {
            let mut result = rendered.to_string();

            // Render project_toc
            let toc_category = if let Ok(manager) = component_manager.read() {
                manager.get_component_category("project_toc").unwrap_or_else(|| "atomic".to_string())
            } else {
                "atomic".to_string()
            };
            let toc_template_path = theme_dir.join("components").join(&toc_category).join("project_toc").join("project_toc.html");
            if toc_template_path.exists() {
                let template_content = if let Ok(cache) = template_cache.read() {
                    cache.load(toc_template_path.to_str().unwrap())?
                } else {
                    std::fs::read_to_string(&toc_template_path)?
                };
    
                let mut toc_props = serde_json::Map::new();
                if let Some(props_obj) = props.as_object() {
                    if let Some(toc) = props_obj.get("toc") {
                        toc_props.insert("toc".to_string(), toc.clone());
                    }
                }
    
                let mut tera = tera::Tera::default();
                tera.autoescape_on(vec![]);
                Self::load_component_templates_for_tera(&mut tera, theme_dir)?;
                tera.add_raw_template("project_toc", &template_content)?;            
                let mut context = Context::new();
                context.insert("props", &Value::Object(toc_props.clone()));

                let nested_rendered = tera.render("project_toc", &context)
                    .map_err(|e| Error::template(format!("Failed to render project_toc: {}", e)))?;

                result = result.replace("<div id=\"project-toc-placeholder\"></div>", &nested_rendered);
            }

            // Render project_content
            let content_category = if let Ok(manager) = component_manager.read() {
                manager.get_component_category("project_content").unwrap_or_else(|| "atomic".to_string())
            } else {
                "atomic".to_string()
            };
            let content_template_path = theme_dir.join("components").join(&content_category).join("project_content").join("project_content.html");
            if content_template_path.exists() {
                let template_content = if let Ok(cache) = template_cache.read() {
                    cache.load(content_template_path.to_str().unwrap())?
                } else {
                    std::fs::read_to_string(&content_template_path)?
                };
        
                let mut content_props = serde_json::Map::new();
                if let Some(props_obj) = props.as_object() {
                    if let Some(title) = props_obj.get("title") {
                        content_props.insert("title".to_string(), title.clone());
                    }
                    if let Some(content) = props_obj.get("content") {
                        content_props.insert("content".to_string(), content.clone());
                    }
                    if let Some(page) = props_obj.get("page") {
                        content_props.insert("page".to_string(), page.clone());
                    }
                }
        
                let mut tera = tera::Tera::default();
                tera.autoescape_on(vec![]);
                Self::load_component_templates_for_tera(&mut tera, theme_dir)?;
                tera.add_raw_template("project_content", &template_content)?;            
                let mut context = Context::new();
                context.insert("props", &Value::Object(content_props.clone()));
                if let Some(page) = props.as_object().and_then(|p| p.get("page")) {
                    context.insert("page", page);
                }

                let nested_rendered = tera.render("project_content", &context)
                    .map_err(|e| Error::template(format!("Failed to render project_content: {}", e)))?;

                result = result.replace("<div id=\"project-content-placeholder\"></div>", &nested_rendered);
            }

            Ok(result)
        }

    fn build_nested_context(component_name: &str, props: &Value) -> Context {
        let mut context = Context::new();
        context.insert("props", props);
        if let Some(props_obj) = props.as_object() {
            for (key, value) in props_obj {
                context.insert(key, value);
            }
        }
        if let Some(page) = props.get("page") {
            context.insert("page", page);
        }
        
        // Pass book_toc to book_toc component
        if component_name == "book_toc" {
            if let Some(book_toc) = props.get("book_toc") {
                context.insert("book_toc", book_toc);
            }
            if let Some(book_title) = props.get("book_title") {
                context.insert("book_title", book_title);
            }
        }
        
        context
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
                Ok(Value::String(format!("/assets/{}", clean_path)))
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

        templates.sort_by(|a, b| {
            if a.0 == "base.html" && b.0 != "base.html" {
                std::cmp::Ordering::Less
            } else if a.0 != "base.html" && b.0 == "base.html" {
                std::cmp::Ordering::Greater
            } else {
                a.0.cmp(&b.0)
            }
        });

        for (name, content) in templates {
            tera.add_raw_template(&name, &content)
                .map_err(|e| Error::template(e.to_string()))?;
        }

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
        let mut enhanced_context = context.clone();

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

        let theme_vars = self.get_theme_variables();
        enhanced_context.insert("theme_variables", &theme_vars);

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
        self.render("article.html", &context)
    }

    /// Helper function to render nested components
    #[allow(dead_code)]
    fn render_nested_component_helper(
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