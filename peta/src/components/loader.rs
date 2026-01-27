//! Component loader for loading components from theme directories

use crate::components::{Component, ComponentConfig};
use crate::core::{Error, Result};
use std::path::{Path, PathBuf};

/// Component loader for loading components from the filesystem
pub struct ComponentLoader {
    theme_dir: PathBuf,
}

impl ComponentLoader {
    /// Create a new component loader
    pub fn new(theme_dir: &PathBuf) -> Self {
        Self {
            theme_dir: theme_dir.clone(),
        }
    }
    
    /// Load all components from a theme directory
    pub fn load_components_from_theme(&self, theme_dir: &Path) -> Result<Vec<Component>> {
        let mut components = Vec::new();
        let components_dir = theme_dir.join("components");
        
        if !components_dir.exists() {
            return Ok(components);
        }
        
        let component_categories = ["atomic", "composite"];
        
        for category in &component_categories {
            let category_dir = components_dir.join(category);
            if category_dir.exists() {
                for entry in std::fs::read_dir(&category_dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path.is_dir() {
                        let component_name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown");
                            
                        match self.load_component(component_name) {
                            Ok(component) => components.push(component),
                            Err(e) => {
                                eprintln!("Warning: Failed to load component '{}': {}", component_name, e);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(components)
    }
    
    /// Load a component from the theme directory
    pub fn load_component(&self, name: &str) -> Result<Component> {
        let component_dir = self.theme_dir.join("components")
            .join(self.get_category_dir(name))
            .join(name);
        
        if !component_dir.exists() {
            return Err(Error::Component(format!("Component directory not found: {}", component_dir.display())));
        }
        
        // Load component configuration
        let config_path = component_dir.join("component.yaml");
        let config_content = std::fs::read_to_string(&config_path)
            .map_err(|e| Error::Component(format!("Failed to read component config: {}", e)))?;
        
        let config: ComponentConfig = serde_yaml::from_str(&config_content)
            .map_err(|e| Error::Component(format!("Failed to parse component config: {}", e)))?;
        
        // Validate component structure
        self.validate_component_structure(&component_dir, &config)?;
        
        // Create component
        let component = Component {
            name: config.name.clone(),
            version: config.version,
            category: config.category,
            description: config.description,
            enabled: true, // Default to enabled
            dependencies: config.dependencies,
            props: config.props,
            slots: config.slots,
            state: config.state,
            templates: self.discover_files(&component_dir, "html")?,
            styles: self.discover_files(&component_dir, "css")?,
            scripts: self.discover_files(&component_dir, "js")?,
            static_data: config.static_data,
            config_schema: config.config_schema,
            default_config: config.default_config,
            seo: config.seo,
        };
        
        Ok(component)
    }
    
    /// Load all components from the theme directory
    pub fn load_all_components(&self) -> Result<Vec<Component>> {
        let mut components = Vec::new();
        let components_dir = self.theme_dir.join("components");
        
        if !components_dir.exists() {
            return Ok(components);
        }
        
        // Load components from each category
        let categories = ["atomic", "composite"];
        
        for category in &categories {
            let category_dir = components_dir.join(category);
            if !category_dir.exists() {
                continue;
            }
            
            for entry in std::fs::read_dir(&category_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    let component_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    
                    match self.load_component(component_name) {
                        Ok(component) => components.push(component),
                        Err(e) => {
                            eprintln!("Warning: Failed to load component '{}': {}", component_name, e);
                        }
                    }
                }
            }
        }
        
        Ok(components)
    }
    
    /// Discover files of a specific type in a directory
    fn discover_files(&self, dir: &PathBuf, _extension: &str) -> Result<Vec<String>> {
        let mut files = Vec::new();
        
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == extension {
                        if let Some(name) = path.file_name() {
                            if let Some(name_str) = name.to_str() {
                                files.push(name_str.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        Ok(files)
    }
    
    /// Get the category directory for a component name
    fn get_category_dir(&self, name: &str) -> &'static str {
        // This is a simple heuristic - in a real implementation,
        // we might use a mapping or component metadata
        match name {
            "navbar" => "atomic",
            "contacts" => "atomic",
            "tag_cloud" => "atomic",
            "grid_card" => "atomic",
            "content_div" => "atomic",
            "article_toc" => "atomic",
            "article_content" => "atomic",
            "book_toc" => "atomic",
                "book_content" => "atomic",
                "header" => "composite",            "footer" => "composite",
            "page_tags" => "composite",
            "snippet_card_modal" => "composite",
            "grid_cards" => "composite",
            "article_modal" => "composite",
            "book_modal" => "composite",
            _ => "content",
        }
    }
    
    /// Validate that the component directory has the required structure
    fn validate_component_structure(&self, dir: &PathBuf, config: &ComponentConfig) -> Result<()> {
        // Check if required files exist
        for template in &config.templates {
            let template_path = dir.join(template);
            if !template_path.exists() {
                return Err(Error::Component(format!("Template file not found: {}", template_path.display())));
            }
        }
        
        for style in &config.styles {
            let style_path = dir.join(style);
            if !style_path.exists() {
                eprintln!("Warning: Style file not found: {}", style_path.display());
            }
        }
        
        for script in &config.scripts {
            let script_path = dir.join(script);
            if !script_path.exists() {
                eprintln!("Warning: Script file not found: {}", script_path.display());
            }
        }
        
        Ok(())
    }
    
    /// Load component configuration from YAML file
    pub fn load_component_config(&self, path: &PathBuf) -> Result<ComponentConfig> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::Component(format!("Failed to read component config: {}", e)))?;
        
        serde_yaml::from_str(&content)
            .map_err(|e| Error::Component(format!("Failed to parse component config: {}", e)))
    }
    
    /// Save component configuration to YAML file
    pub fn save_component_config(&self, config: &ComponentConfig, path: &PathBuf) -> Result<()> {
        let content = serde_yaml::to_string(config)
            .map_err(|e| Error::Component(format!("Failed to serialize component config: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| Error::Component(format!("Failed to write component config: {}", e)))
    }
    
    /// Create a new component directory structure
    pub fn create_component_directory(&self, name: &str, category: &str) -> Result<PathBuf> {
        let component_dir = self.theme_dir.join("components")
            .join(category)
            .join(name);
        
        std::fs::create_dir_all(&component_dir)?;
        
        // Create subdirectories
        std::fs::create_dir_all(component_dir.join("templates"))?;
        std::fs::create_dir_all(component_dir.join("styles"))?;
        std::fs::create_dir_all(component_dir.join("scripts"))?;
        
        Ok(component_dir)
    }
    
    /// Install a component from a template
    pub fn install_component_template(&self, template: &str, name: &str) -> Result<()> {
        let template_dir = self.theme_dir.join("component_templates").join(template);
        if !template_dir.exists() {
            return Err(Error::Component(format!("Component template not found: {}", template)));
        }
        
        let component_dir = self.create_component_directory(name, "content")?;
        
        // Copy template files
        self.copy_directory(&template_dir, &component_dir)?;
        
        Ok(())
    }
    
    /// Copy directory contents
    fn copy_directory(&self, src: &PathBuf, dst: &PathBuf) -> Result<()> {
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            
            if src_path.is_dir() {
                std::fs::create_dir_all(&dst_path)?;
                self.copy_directory(&src_path, &dst_path)?;
            } else {
                std::fs::copy(&src_path, &dst_path)?;
            }
        }
        
        Ok(())
    }
}