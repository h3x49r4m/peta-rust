//! Enhanced template engine with V4 component support

use crate::core::{Result, Error};
use crate::core::theme::Theme;
use crate::templates::{filters, functions};
use crate::components::ComponentRegistry;
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
            Box::new(|args: &HashMap<String, Value>| -> tera::Result<Value> {
                let component_name = args.get("0")
                    .or_else(|| args.get("name"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| tera::Error::msg("Component name is required"))?;
                
                let props = args.get("1")
                    .or_else(|| args.get("props"))
                    .cloned()
                    .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
                
                let slots: std::collections::HashMap<String, String> = args.get("2")
                    .or_else(|| args.get("slots"))
                    .and_then(|v| v.as_object())
                    .map(|obj| {
                        obj.iter()
                            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                
                // In a real implementation, this would render the component
                // For now, return a structured placeholder
                let component_html = format!(
                    r#"<div data-component="{}" data-props="{}" data-slots="{}">"</div>"#,
                    component_name,
                    serde_json::to_string(&props).unwrap_or_default(),
                    serde_json::to_string(&slots).unwrap_or_default()
                );
                
                Ok(Value::String(component_html))
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
                        scripts.push_str(&format!("// Script for component: {}\n", name));
                        scripts.push_str(&format!("document.addEventListener('DOMContentLoaded', () => {{\n"));
                        scripts.push_str(&format!("  // Initialize {} component\n", name));
                        scripts.push_str(&format!("}});\n"));
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
            .map_err(|e| Error::template(e.to_string()))
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
}





/// Template filter trait
pub trait TemplateFilter: Send + Sync {
    fn filter(&self, value: &tera::Value, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value>;
}

/// Template function trait
pub trait TemplateFunction: Send + Sync {
    fn call(&self, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value>;
}