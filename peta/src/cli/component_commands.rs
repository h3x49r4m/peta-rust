//! CLI commands for component management

use crate::cli::output::OutputFormatter;
use crate::components::{ComponentRegistry, ComponentConfig, ComponentCategory, ComponentVersion};
use crate::core::{Error, Result};
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// Component management commands
pub struct ComponentCommands;

impl ComponentCommands {
    /// List available components
    pub fn list_components(
        registry: &ComponentRegistry,
        category: Option<ComponentCategory>,
        output: &mut OutputFormatter,
    ) -> Result<()> {
        output.info("Available components:");
        
        let components = if let Some(cat) = category {
            registry.get_components_by_category(&cat)
        } else {
            registry.get_enabled_components()
        };
        
        if components.is_empty() {
            output.warn("No components found");
            return Ok(());
        }
        
        for component in components {
            output.info(&format!("  {} (v{}) - {}", component.name, component.version, component.description));
            
            if !component.dependencies.is_empty() {
                output.info(&format!("    Dependencies: {}", component.dependencies.join(", ")));
            }
            
            if !component.templates.is_empty() {
                output.info(&format!("    Templates: {}", component.templates.join(", ")));
            }
            
            if !component.styles.is_empty() {
                output.info(&format!("    Styles: {}", component.styles.join(", ")));
            }
            
            if !component.scripts.is_empty() {
                output.info(&format!("    Scripts: {}", component.scripts.join(", ")));
            }
        }
        
        Ok(())
    }
    
    /// Create a new component
    pub fn create_component(
        name: &str,
        category: ComponentCategory,
        base: Option<String>,
        output: &mut OutputFormatter,
    ) -> Result<()> {
        output.info(&format!("Creating new component: {}", name));
        
        let component_dir = Path::new("themes").join("default").join("components");
        
        // Determine component directory based on category
        let category_dir = match category {
            ComponentCategory::Atomic => component_dir.join("atomic"),
            ComponentCategory::Composite => component_dir.join("composite"),
            ComponentCategory::Layout => component_dir.join("layouts"),
        };
        
        let target_dir = category_dir.join(name);
        
        if target_dir.exists() {
            return Err(Error::Component(format!("Component '{}' already exists", name)));
        }
        
        // Create component directory structure
        fs::create_dir_all(&target_dir)
            .map_err(|e| Error::Component(format!("Failed to create component directory: {}", e)))?;
        
        // Create component configuration
        let component_config = Self::generate_component_config(name, category, base)?;
        let config_path = target_dir.join("component.yaml");
        fs::write(&config_path, component_config)
            .map_err(|e| Error::Component(format!("Failed to write component config: {}", e)))?;
        
        // Create component template
        let template_content = Self::generate_component_template(name, &category)?;
        let template_path = target_dir.join(format!("{}.html", name));
        fs::write(&template_path, template_content)
            .map_err(|e| Error::Component(format!("Failed to write component template: {}", e)))?;
        
        // Create component styles
        let styles_content = Self::generate_component_styles(name, &category)?;
        let styles_path = target_dir.join(format!("{}.css", name));
        fs::write(&styles_path, styles_content)
            .map_err(|e| Error::Component(format!("Failed to write component styles: {}", e)))?;
        
        // Create component script
        let script_content = Self::generate_component_script(name, &category)?;
        let script_path = target_dir.join(format!("{}.js", name));
        fs::write(&script_path, script_content)
            .map_err(|e| Error::Component(format!("Failed to write component script: {}", e)))?;
        
        output.success(&format!("Component '{}' created successfully!", name));
        output.info("Next steps:");
        output.info(&format!("  1. Edit themes/default/components/{}/{}/component.yaml to configure your component", 
            match category {
                ComponentCategory::Atomic => "atomic",
                ComponentCategory::Composite => "composite", 
                ComponentCategory::Layout => "layouts",
            }, name));
        output.info(&format!("  2. Modify themes/default/components/{}/{}/{}.html to customize appearance", 
            match category {
                ComponentCategory::Atomic => "atomic",
                ComponentCategory::Composite => "composite", 
                ComponentCategory::Layout => "layouts",
            }, name, name));
        output.info("  3. Use your component with: {% component \"your_component_name\" %}");
        
        Ok(())
    }
    
    /// Validate component configuration
    pub fn validate_component(
        name: &str,
        output: &mut OutputFormatter,
    ) -> Result<()> {
        output.info(&format!("Validating component: {}", name));
        
        let component_path = Path::new("themes").join("default").join("components");
        
        // Try to find the component in different categories
        let categories = ["atomic", "composite", "layouts"];
        let mut component_config_path = None;
        
        for category in &categories {
            let path = component_path.join(category).join(name).join("component.yaml");
            if path.exists() {
                component_config_path = Some(path);
                break;
            }
        }
        
        let config_path = component_config_path
            .ok_or_else(|| Error::ComponentNotFound(name.to_string()))?;
        
        // Load and validate component configuration
        let config_content = fs::read_to_string(&config_path)
            .map_err(|e| Error::Component(format!("Failed to read component config: {}", e)))?;
        
        let component_config: ComponentConfig = serde_yaml::from_str(&config_content)
            .map_err(|e| Error::Component(format!("Failed to parse component config: {}", e)))?;
        
        // Validate component
        component_config.validate()
            .map_err(|e| Error::Component(format!("Component validation failed: {}", e)))?;
        
        output.success(&format!("Component '{}' is valid!", name));
        
        // Show component details
        output.info(&format!("Name: {}", component_config.name));
        output.info(&format!("Version: {}", component_config.version));
        output.info(&format!("Category: {:?}", component_config.category));
        
        if let Some(description) = component_config.description {
            output.info(&format!("Description: {}", description));
        }
        
        if let Some(base) = component_config.extends {
            output.info(&format!("Extends: {}", base));
        }
        
        if !component_config.props.is_empty() {
            output.info("Props:");
            for (prop_name, prop_config) in &component_config.props {
                output.info(&format!("  {}: {} (required: {})", prop_name, prop_config.r#type, prop_config.required));
            }
        }
        
        if !component_config.slots.is_empty() {
            output.info("Slots:");
            for slot in &component_config.slots {
                output.info(&format!("  {} (required: {})", slot.name, slot.required));
            }
        }
        
        if !component_config.state.is_empty() {
            output.info("State:");
            for state in &component_config.state {
                output.info(&format!("  {}: {} (persistent: {})", state.name, state.r#type, state.persistent));
            }
        }
        
        Ok(())
    }
    
    /// Show component information
    pub fn component_info(
        name: &str,
        output: &mut OutputFormatter,
    ) -> Result<()> {
        let component_path = Path::new("themes").join("default").join("components");
        
        // Try to find the component in different categories
        let categories = ["atomic", "composite", "layouts"];
        let mut component_dir = None;
        
        for category in &categories {
            let path = component_path.join(category).join(name);
            if path.exists() {
                component_dir = Some(path);
                break;
            }
        }
        
        let component_dir = component_dir
            .ok_or_else(|| Error::ComponentNotFound(name.to_string()))?;
        
        // Load component configuration
        let config_path = component_dir.join("component.yaml");
        let config_content = fs::read_to_string(&config_path)
            .map_err(|e| Error::Component(format!("Failed to read component config: {}", e)))?;
        
        let component_config: ComponentConfig = serde_yaml::from_str(&config_content)
            .map_err(|e| Error::Component(format!("Failed to parse component config: {}", e)))?;
        
        output.info(&format!("Component: {}", component_config.name));
        output.info(&format!("Version: {}", component_config.version));
        
        if let Some(description) = &component_config.description {
            output.info(&format!("Description: {}", description));
        }
        
        if let Some(parent) = &component_config.extends {
            output.info(&format!("Extends: {}", parent));
        }
        
        // List files
        output.info("Files:");
        for template in &component_config.templates {
            output.info(&format!("  - templates/{}", template));
        }
        
        for style in &component_config.styles {
            output.info(&format!("  - styles/{}", style));
        }
        
        for script in &component_config.scripts {
            output.info(&format!("  - scripts/{}", script));
        }
        
        // Show props
        if !component_config.props.is_empty() {
            output.info(&format!("Props: {}", component_config.props.len()));
        }
        
        // Show slots
        if !component_config.slots.is_empty() {
            output.info(&format!("Slots: {}", component_config.slots.len()));
        }
        
        // Show state
        if !component_config.state.is_empty() {
            output.info(&format!("State: {}", component_config.state.len()));
        }
        
        Ok(())
    }
    
    /// Install component from repository
    pub fn install_component(
        source: &str,
        name: Option<String>,
        output: &mut OutputFormatter,
    ) -> Result<()> {
        let component_name = name.unwrap_or_else(|| {
            // Extract name from source path
            source.split('/').last().unwrap_or(source).to_string()
        });
        
        output.info(&format!("Installing component '{}' from '{}'", component_name, source));
        
        // For now, this is a placeholder implementation
        // In a real implementation, this would handle Git repository cloning, etc.
        output.warn("Component installation from repositories is not yet implemented");
        output.info(&format!("Please manually clone the component to themes/default/components/{}", component_name));
        
        Ok(())
    }
    
    /// Generate component configuration
    fn generate_component_config(
        name: &str,
        category: ComponentCategory,
        base: Option<String>,
    ) -> Result<String> {
        let config = ComponentConfig::new(name.to_string(), "1.0.0".to_string(), category);
        
        let config_yaml = serde_yaml::to_string(&config)
            .map_err(|e| Error::Component(format!("Failed to serialize component config: {}", e)))?;
        
        Ok(config_yaml)
    }
    
    /// Generate component template
    fn generate_component_template(name: &str, category: &ComponentCategory) -> Result<String> {
        let template = match category {
            ComponentCategory::Atomic => {
                format!(r#"<div class="{}" data-component="{}">
  {{#if props}}
    {{#each props}}
    <span class="{}-{{@key}}">{{this}}</span>
    {{/each}}
  {{/if}}
</div>"#, name, name, name)
            }
            ComponentCategory::Composite => {
                format!(r#"<div class="{}" data-component="{}">
  <div class="{}-header">
    {{#if props.title}}
    <h3 class="{}-title">{{props.title}}</h3>
    {{/if}}
  </div>
  
  <div class="{}-content">
    {{#if slot.children}}
      {{slot.children}}
    {{else}}
      <p>Default content for {} component</p>
    {{/if}}
  </div>
  
  {{#if props.showFooter}}
  <div class="{}-footer">
    <small>{{name}} component</small>
  </div>
  {{/if}}
</div>"#, name, name, name, name, name, name, name, name, name)
            }
            ComponentCategory::Layout => {
                format!(r#"<div class="{}-layout" data-component="{}">
  <header class="{}-header">
    {{#if slot.header}}
      {{slot.header}}
    {{/if}}
  </header>
  
  <main class="{}-main">
    {{#if slot.content}}
      {{slot.content}}
    {{/if}}
  </main>
  
  <footer class="{}-footer">
    {{#if slot.footer}}
      {{slot.footer}}
    {{/if}}
  </footer>
</div>"#, name, name, name, name, name, name)
            }
        };
        
        Ok(template)
    }
    
    /// Generate component styles
    fn generate_component_styles(name: &str, _category: &ComponentCategory) -> Result<String> {
        let styles = format!(r#".{} {{
  display: block;
  margin: 0;
  padding: 0;
}}

.{}-header {{
  margin-bottom: 1rem;
}}

.{}-title {{
  font-size: 1.2rem;
  font-weight: 600;
  margin: 0;
}}

.{}-content {{
  margin-bottom: 1rem;
}}

.{}-footer {{
  font-size: 0.875rem;
  color: #666;
}}

/* Component-specific styles */
.{} {{
  /* Add your custom styles here */
}}"#, name, name, name, name, name, name, name);
        
        Ok(styles)
    }
    
    /// Generate component script
    fn generate_component_script(name: &str, _category: &ComponentCategory) -> Result<String> {
        let script = format!(r#"class {}Component {{
  constructor(element) {{
    this.element = element;
    this.init();
  }}
  
  init() {{
    // Initialize component
    console.log('{} component initialized');
    
    // Add event listeners
    this.bindEvents();
  }}
  
  bindEvents() {{
    // Add event listeners here
  }}
  
  destroy() {{
    // Clean up component
    console.log('{} component destroyed');
  }}
}}

// Initialize components
document.addEventListener('DOMContentLoaded', () => {{
  document.querySelectorAll('[data-component="{}"]').forEach(element => {{
    new {}Component(element);
  }});
}});
"#, name, name, name, name);
        
        Ok(script)
    }
}

/// Component creation options
#[derive(Debug, Clone)]
pub struct CreateOptions {
    pub name: String,
    pub category: ComponentCategory,
    pub base: Option<String>,
    pub props: Vec<String>,
    pub slots: Vec<String>,
    pub state: Vec<String>,
}

impl Default for CreateOptions {
    fn default() -> Self {
        Self {
            name: String::new(),
            category: ComponentCategory::Composite,
            base: None,
            props: Vec::new(),
            slots: Vec::new(),
            state: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_component_config_generation() {
        let config = ComponentCommands::generate_component_config(
            "test-component",
            ComponentCategory::Composite,
            None,
        ).unwrap();
        
        assert!(config.contains("name: test-component"));
        assert!(config.contains("version: 1.0.0"));
        assert!(config.contains("category: Composite"));
    }
    
    #[test]
    fn test_component_template_generation() {
        let template = ComponentCommands::generate_component_template(
            "test-component",
            &ComponentCategory::Composite,
        ).unwrap();
        
        assert!(template.contains("data-component=\"test-component\""));
        assert!(template.contains("test-component-header"));
        assert!(template.contains("test-component-content"));
    }
    
    #[test]
    fn test_component_styles_generation() {
        let styles = ComponentCommands::generate_component_styles(
            "test-component",
            &ComponentCategory::Composite,
        ).unwrap();
        
        assert!(styles.contains(".test-component {"));
        assert!(styles.contains(".test-component-header {"));
        assert!(styles.contains(".test-component-content {"));
    }
    
    #[test]
    fn test_component_script_generation() {
        let script = ComponentCommands::generate_component_script(
            "test-component",
            &ComponentCategory::Composite,
        ).unwrap();
        
        assert!(script.contains("class testComponentComponent"));
        assert!(script.contains("test-component initialized"));
        assert!(script.contains("[data-component=\"test-component\"]"));
    }
}