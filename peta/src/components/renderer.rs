//! Component renderer for rendering components in templates

use crate::components::{Component, ComponentRegistry};
use crate::core::{Error, Result};
use crate::templates::TemplateEngine;
use serde_json::{json, Value};

/// Processed component result
#[derive(Debug, Clone)]
pub struct ProcessedComponent {
    /// Component name
    pub name: String,
    /// Rendered HTML content
    pub html: String,
    /// CSS content
    pub css: Option<String>,
    /// JavaScript content
    pub js: Option<String>,
    /// Component metadata
    pub metadata: serde_json::Value,
}

/// Template component renderer for V4 architecture
pub struct TemplateComponentRenderer {
    #[allow(dead_code)]
    theme: crate::core::Theme,
    component_registry: ComponentRegistry,
    template_cache: std::collections::HashMap<String, String>,
}

impl TemplateComponentRenderer {
    pub fn new(theme: crate::core::Theme, component_registry: ComponentRegistry) -> Self {
        Self {
            theme,
            component_registry,
            template_cache: std::collections::HashMap::new(),
        }
    }
    
    pub fn render_component(&mut self, component_name: &str, props: &serde_json::Value, slots: &std::collections::HashMap<String, String>) -> crate::core::Result<String> {
        let key = format!("{}:{}:{}", component_name, serde_json::to_string(props).unwrap_or_default(), serde_json::to_string(slots).unwrap_or_default());
        
        if let Some(cached) = self.template_cache.get(&key) {
            return Ok(cached.clone());
        }
        
        let component = self.component_registry.get_component(component_name)
            .ok_or_else(|| crate::core::Error::ComponentNotFound(component_name.to_string()))?;
        
        let mut context = tera::Context::new();
        context.insert("props", props);
        context.insert("slots", &slots);
        
        let mut tera = tera::Tera::default();
        tera.add_raw_template(&component.name, &component.templates[0])?;
        
        let rendered = tera.render(&component.name, &context)?;
        
        self.template_cache.insert(key, rendered.clone());
        Ok(rendered)
    }
    
    pub fn render_atomic_component(&mut self, component_name: &str, props: &serde_json::Value) -> crate::core::Result<String> {
        self.render_component(component_name, props, &std::collections::HashMap::new())
    }
    
    pub fn render_composite_component(&mut self, component_name: &str, props: &serde_json::Value, slots: &std::collections::HashMap<String, String>) -> crate::core::Result<String> {
        self.render_component(component_name, props, slots)
    }
    
    pub fn render_layout_component(&mut self, component_name: &str, content: &str, props: &serde_json::Value) -> crate::core::Result<String> {
        let mut slots = std::collections::HashMap::new();
        slots.insert("content".to_string(), content.to_string());
        self.render_component(component_name, props, &slots)
    }
}

/// Component renderer for rendering components in templates
#[derive(Clone)]
pub struct ComponentRendererWrapper {
    template_engine: Box<TemplateEngine>,
    registry: ComponentRegistry,
}

impl ComponentRendererWrapper {
    /// Create a new component renderer
    pub fn new(template_engine: TemplateEngine, registry: ComponentRegistry) -> Self {
        Self {
            template_engine: Box::new(template_engine),
            registry,
        }
    }
    
    /// Render a component with the given context
    pub fn render_component(&self, name: &str, context: &serde_json::Value) -> crate::core::Result<String> {
        let component = self.registry.get_component(name)
            .ok_or_else(|| Error::ComponentNotFound(name.to_string()))?;
        
        // Merge component default config with provided config
        let merged_config = self.merge_component_config(&component, context)?;
        
        // Create component context
        let mut component_context = context.clone();
        component_context["component"] = json!(merged_config);
        component_context["component_name"] = json!(name);
        
        // Render main template
        let template_name = &component.templates[0];
        let context = tera::Context::from_serialize(&component_context)?;
        let html = self.template_engine.render(template_name, &context)?;
        
        Ok(html)
    }
    
    /// Render a component with specific configuration
    pub fn render_component_with_config(&self, name: &str, config: &Value, context: &serde_json::Value) -> crate::core::Result<String> {
        let component = self.registry.get_component(name)
            .ok_or_else(|| Error::ComponentNotFound(name.to_string()))?;
        
        // Create component context with specific config
        let mut component_context = context.clone();
        component_context["component"] = config.clone();
        component_context["component_name"] = json!(name);
        component_context["component_config"] = config.clone();
        
        // Render main template
        let template_name = &component.templates[0];
        let context = tera::Context::from_serialize(&component_context)?;
        let html = self.template_engine.render(template_name, &context)?;
        
        Ok(html)
    }
    
    /// Render multiple components in order
    pub fn render_components(&self, component_names: &[String], context: &serde_json::Value) -> crate::core::Result<Vec<String>> {
        let mut rendered = Vec::new();
        
        for name in component_names {
            let html = self.render_component(name, context)?;
            rendered.push(html);
        }
        
        Ok(rendered)
    }
    
    /// Render a layout component
    pub fn render_layout(&self, layout_name: &str, context: &serde_json::Value) -> Result<String> {
        let layout_dir = self.template_engine.theme_dir_buf().join("layouts");
        let layout_file = layout_dir.join(format!("{}.html", layout_name));
        
        if !layout_file.exists() {
            return Err(Error::Template(format!("Layout template not found: {}", layout_file.display())));
        }
        
        let mut layout_context = context.clone();
        layout_context["layout_name"] = json!(layout_name);
        
        let context = tera::Context::from_serialize(&layout_context)?;
        self.template_engine.render(&format!("layouts/{}", layout_name), &context)
    }
    
    /// Include component styles in HTML
    pub fn include_component_styles(&self, component: &Component) -> Result<String> {
        let mut style_html = String::new();
        
        for style_file in &component.styles {
            let style_path = self.template_engine.theme_dir_buf().join("components")
                .join(self.get_category_dir(&component.name))
                .join(&component.name)
                .join("styles")
                .join(style_file);
            
            if style_path.exists() {
                let css_content = std::fs::read_to_string(&style_path)
                    .map_err(|e| Error::Template(format!("Failed to read style file: {}", e)))?;
                style_html.push_str(&css_content);
                style_html.push('\n');
            }
        }
        
        Ok(style_html)
    }
    
    /// Include component scripts in HTML
    pub fn include_component_scripts(&self, component: &Component) -> Result<String> {
        let mut script_html = String::new();
        
        for script_file in &component.scripts {
            let script_path = self.template_engine.theme_dir_buf().join("components")
                .join(self.get_category_dir(&component.name))
                .join(&component.name)
                .join("scripts")
                .join(script_file);
            
            if script_path.exists() {
                let js_content = std::fs::read_to_string(&script_path)
                    .map_err(|e| Error::Template(format!("Failed to read script file: {}", e)))?;
                script_html.push_str(&js_content);
                script_html.push('\n');
            }
        }
        
        Ok(script_html)
    }
    
    /// Get all component styles for a site
    pub fn get_all_component_styles(&self, component_names: &[String]) -> Result<String> {
        let mut all_styles = String::new();
        
        for name in component_names {
            if let Some(component) = self.registry.get_component(name) {
                let styles = self.include_component_styles(component)?;
                all_styles.push_str(&styles);
            }
        }
        
        Ok(all_styles)
    }
    
    /// Get all component scripts for a site
    pub fn get_all_component_scripts(&self, component_names: &[String]) -> Result<String> {
        let mut all_scripts = String::new();
        
        for name in component_names {
            if let Some(component) = self.registry.get_component(name) {
                let scripts = self.include_component_scripts(component)?;
                all_scripts.push_str(&scripts);
            }
        }
        
        Ok(all_scripts)
    }
    
    /// Validate component configuration against schema
    pub fn validate_component_config(&self, component: &Component, config: &Value) -> crate::core::Result<()> {
        // In a real implementation, we would validate against the schema
        // For now, we just check if the config is valid JSON
        if config.is_null() || config.is_string() || config.is_array() {
            return Ok(());
        }
        
        // Basic validation - in a real implementation, we'd use JSON Schema
        self.validate_config_against_schema(&component.config_schema, config)
    }
    
    /// Validate configuration against schema (simplified)
    fn validate_config_against_schema(&self, schema: &Value, config: &Value) -> Result<()> {
        // This is a simplified validation - in a real implementation,
        // we'd use a proper JSON Schema validator
        match (schema, config) {
            (Value::Object(_), Value::Object(_)) => Ok(()),
            (Value::Array(_), Value::Array(_)) => Ok(()),
            (Value::String(_), Value::String(_)) => Ok(()),
            (Value::Bool(_), Value::Bool(_)) => Ok(()),
            (Value::Number(_), Value::Number(_)) => Ok(()),
            (Value::Null, Value::Null) => Ok(()),
            _ => Err(Error::Component("Configuration type mismatch".to_string())),
        }
    }
    
    /// Merge component default config with provided config
    fn merge_component_config(&self, component: &Component, context: &serde_json::Value) -> crate::core::Result<serde_json::Value> {
        let mut merged = component.default_config.clone();
        
        // Override with user config if provided
        if let Some(site_config) = context.get("component_config") {
            if let Some(component_config) = site_config.get(&component.name) {
                self.merge_json_objects(&mut merged, component_config)?;
            }
        }
        
        Ok(merged)
    }
    
    /// Merge two JSON objects recursively
    fn merge_json_objects(&self, target: &mut serde_json::Value, source: &serde_json::Value) -> crate::core::Result<()> {
        match target {
            Value::Object(ref mut target_map) => {
                if let Value::Object(source_map) = source {
                    for (key, value) in source_map {
                        if let Some(target_value) = target_map.get_mut(key) {
                            // Both have the key, merge recursively
                            self.merge_json_objects(target_value, value)?;
                        } else {
                            // Key only in source, add it
                            target_map.insert(key.clone(), value.clone());
                        }
                    }
                } else {
                    // Override target with source
                    *target = source.clone();
                }
            }
            _ => {
                // Override target with source
                *target = source.clone();
            }
        }
        
        Ok(())
    }
    
    /// Get the category directory for a component name
    fn get_category_dir(&self, name: &str) -> &'static str {
        match name {
            "footer" => "core",
            _ => "content",
        }
    }
}