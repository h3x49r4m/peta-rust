//! Component manager for managing components lifecycle

use crate::components::{Component, ComponentLoader, ComponentRegistry, ComponentVersion, ComponentDiscovery};
use crate::components::config::ComponentCategory;
use crate::core::{Error, Result};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Component manager for managing components lifecycle
pub struct ComponentManager {
    registry: ComponentRegistry,
    loader: ComponentLoader,
    discovery: Arc<RwLock<ComponentDiscovery>>,
    theme_dir: PathBuf,
}

impl ComponentManager {
    /// Create a new component manager
    pub fn new(theme_dir: &PathBuf) -> Self {
        let registry = ComponentRegistry::new();
        let loader = ComponentLoader::new(theme_dir);
        let discovery = Arc::new(RwLock::new(ComponentDiscovery::new(theme_dir)));
        
        Self {
            registry,
            loader,
            discovery,
            theme_dir: theme_dir.clone(),
        }
    }
    
    /// Initialize the component manager with default components
    pub fn initialize(&mut self) -> Result<()> {
        // Discover all components from theme using discovery system
        let discovered = if let Ok(mut discovery) = self.discovery.write() {
            discovery.discover_all()?
        } else {
            vec![]
        };
        
        // Convert discovered components to Component structs
        for discovered_component in discovered {
            if let Ok(component) = discovered_component.to_component() {
                let _ = self.registry.register_component(component);
            }
        }
        
        Ok(())
    }
    
    /// Install a component
    pub async fn install_component(&mut self, name: &str, version: Option<String>) -> Result<Component> {
        // Check if component already exists
        if self.registry.get_component(name).is_some() {
            return Err(Error::Component(format!("Component '{}' already exists", name)));
        }
        
        // Load component from registry or template
        let component = self.load_component_from_registry(name, version).await?;
        
        // Validate component dependencies
        self.validate_component_dependencies(&component)?;
        
        // Install component files
        self.install_component_files(&component)?;
        
        // Register component
        self.registry.register_component(component.clone())?;
        
        Ok(component)
    }
    
    /// Remove a component
    pub fn remove_component(&mut self, name: &str) -> Result<()> {
        // Check if component exists
        if self.registry.get_component(name).is_none() {
            return Err(Error::Component(format!("Component '{}' not found", name)));
        }
        
        // Check if other components depend on this one
        // TODO: Implement dependency checking when get_dependents is available
        // For now, we'll allow removal
        
        // Remove component files
        self.remove_component_files(name)?;
        
        // Unregister component
        let _ = self.registry.unregister_component(name);
        
        Ok(())
    }
    
    /// Update a component
    pub async fn update_component(&mut self, name: &str, version: Option<String>) -> Result<Component> {
        // Check if component exists
        if self.registry.get_component(name).is_none() {
            return Err(Error::Component(format!("Component '{}' not found", name)));
        }
        
        // Get current component
        let current_component = self.registry.get_component(name).unwrap().clone();
        
        // Load new version
        let new_component = self.load_component_from_registry(name, version).await?;
        
        // Check compatibility
        let current_version = ComponentVersion::from_string(&current_component.version)?;
        let new_version = ComponentVersion::from_string(&new_component.version)?;
        
        if !new_version.compatible_with(&current_version) {
            return Err(Error::Component(format!("Version incompatibility: current={}, new={}", current_version, new_version)));
        }
        
        // Remove old component files
        self.remove_component_files(name)?;
        
        // Install new component files
        self.install_component_files(&new_component)?;
        
        // Unregister old component
        let _ = self.registry.unregister_component(name);
        
        // Register new component
        self.registry.register_component(new_component.clone())?;
        
        Ok(new_component)
    }
    
    /// Get component information
    pub fn get_component_info(&self, name: &str) -> Option<&Component> {
        self.registry.get_component(name)
    }

    /// Get component category
    pub fn get_component_category(&self, name: &str) -> Option<String> {
        // Handle code_block specially as it's rendered by Rust
        if name == "code_block" {
            return Some("atomic".to_string());
        }
        
        if let Some(mut discovery) = self.discovery.write().ok() {
            if let Ok(Some(component)) = discovery.get_component(name) {
                return Some(component.category);
            }
        }
        None
    }

    /// Get all components
    pub fn get_all(&self) -> Vec<Component> {
        self.registry.get_all_components().values().cloned().collect()
    }
    
    /// List all components
    pub fn list_components(&self) -> Vec<String> {
        self.registry.get_all_components().keys().cloned().collect()
    }
    
    /// List components by category
    pub fn list_components_by_category(&self, category: &ComponentCategory) -> Vec<&Component> {
        self.registry.get_components_by_category(category)
    }
    
    /// Validate component dependencies
    fn validate_component_dependencies(&self, component: &Component) -> Result<()> {
        for dep in &component.dependencies {
            if self.registry.get_component(dep).is_none() {
                return Err(Error::Component(format!("Dependency '{}' not found for component '{}'", dep, component.name)));
            }
        }
        
        Ok(())
    }
    
    /// Remove component files
    fn remove_component_files(&self, name: &str) -> Result<()> {
        // Get component category from discovery system
        let category = self.get_component_category(name)
            .unwrap_or_else(|| "atomic".to_string());

        let component_dir = self.theme_dir.join("components")
            .join(&category)
            .join(name);

        if component_dir.exists() {
            std::fs::remove_dir_all(&component_dir)?;
        }

        Ok(())
    }
    
    /// Install component files
    fn install_component_files(&self, component: &Component) -> Result<()> {
        // Get category from component struct
        let category = match component.category {
            ComponentCategory::Atomic => "atomic",
            ComponentCategory::Composite => "composite",
        };

        let component_dir = self.theme_dir.join("components")
            .join(category)
            .join(&component.name);

        // Create component directory if it doesn't exist
        if !component_dir.exists() {
            std::fs::create_dir_all(&component_dir)?;
        }

        // Install templates
        for template in &component.templates {
            let template_path = self.theme_dir.join("components")
                .join(category)
                .join(&component.name)
                .join("templates")
                .join(template);

            let template_dir = template_path.parent().unwrap();
            if !template_dir.exists() {
                std::fs::create_dir_all(template_dir)?;
            }

            // In a real implementation, we'd copy from a template
            // For now, we just ensure the directory exists
        }

        Ok(())
    }
    
    /// Load component from registry (placeholder implementation)
    async fn load_component_from_registry(&self, name: &str, _version: Option<String>) -> Result<Component> {
        // In a real implementation, this would download from a registry
        // For now, we'll try to load from the theme directory
        match self.loader.load_component(name, None) {
            Ok(component) => Ok(component),
            Err(_) => {
                // Create a placeholder component
Ok(Component {
                    name: name.to_string(),
                    version: "1.0.0".to_string(),
                    category: ComponentCategory::Composite,
                    description: format!("Remote component: {}", name),
                    enabled: true,
                    dependencies: vec![],
                    props: std::collections::HashMap::new(),
                    slots: vec![],
                    state: vec![],
                    templates: vec![format!("{}.html", name)],
                    styles: vec![],
                    scripts: vec![],
                    static_data: vec![],
                    config_schema: serde_json::Value::Object(serde_json::Map::new()),
                    default_config: serde_json::Value::Object(serde_json::Map::new()),
                    seo: None,
                })
            }
        }
    }
    
    }

/// Get the global component manager instance
pub fn get_component_manager() -> &'static mut ComponentManager {
    use std::sync::Once;
    
    static INIT: Once = Once::new();
    static mut MANAGER: Option<ComponentManager> = None;
    
    INIT.call_once(|| {
        let theme_dir = PathBuf::from("themes/default");
        unsafe {
            MANAGER = Some(ComponentManager::new(&theme_dir));
        }
    });
    
    #[allow(static_mut_refs)]
    unsafe { MANAGER.as_mut() }.expect("Component manager not initialized")
}