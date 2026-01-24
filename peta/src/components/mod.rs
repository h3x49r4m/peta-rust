//! Component system for PETA_RUST V4
//! 
//! This module provides a flexible component-based architecture for building
//! static sites with reusable UI components following the V4 architecture.

pub mod config;
pub mod loader;
pub mod manager;
pub mod registry;
pub mod renderer;
pub mod theme;
pub mod version;

pub use config::{ComponentConfig, PropConfig, SlotConfig, StateConfig, ComponentCategory, SeoConfig};
pub use loader::ComponentLoader;
pub use manager::ComponentManager;
pub use registry::{ComponentRegistry, DependencyGraph};
pub use renderer::{ComponentRendererWrapper, TemplateComponentRenderer};
pub use theme::{ThemeManager, ThemeConfig, ComponentConfig as ThemeComponentConfig, VariantConfig, AssetConfig, OptimizationConfig};
pub use version::ComponentVersion;

// Re-export for backward compatibility
pub use config::SiteComponentConfig;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A component that can be used in site templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// Component name
    pub name: String,
    /// Component version
    pub version: String,
    /// Component category
    pub category: crate::components::config::ComponentCategory,
    /// Component description
    pub description: String,
    /// Whether the component is enabled
    pub enabled: bool,
    /// Required dependencies
    pub dependencies: Vec<String>,
    /// Component props configuration
    pub props: HashMap<String, PropConfig>,
    /// Component slots configuration
    pub slots: Vec<SlotConfig>,
    /// Component state configuration
    pub state: Vec<StateConfig>,
    /// Template files
    pub templates: Vec<String>,
    /// CSS files
    pub styles: Vec<String>,
    /// JavaScript files
    pub scripts: Vec<String>,
    /// Static data files
    pub static_data: Vec<String>,
    /// Configuration schema
    pub config_schema: serde_json::Value,
    /// Default configuration
    pub default_config: serde_json::Value,
    /// SEO configuration
    pub seo: Option<SeoConfig>,
}





/// Processed component ready for rendering
#[derive(Debug, Clone)]
pub struct ProcessedComponent {
    /// Component name
    pub name: String,
    /// Processed HTML template
    pub template: String,
    /// Processed CSS with theme variables
    pub styles: String,
    /// Processed JavaScript
    pub scripts: String,
    /// Component dependencies
    pub dependencies: Vec<String>,
    /// Static data
    pub static_data: HashMap<String, serde_json::Value>,
}

impl Component {
    /// Create a new component
    pub fn new(name: String, version: String, category: crate::components::config::ComponentCategory) -> Self {
        Self {
            name,
            version,
            category,
            description: String::new(),
            enabled: true,
            dependencies: Vec::new(),
            props: HashMap::new(),
            slots: Vec::new(),
            state: Vec::new(),
            templates: Vec::new(),
            styles: Vec::new(),
            scripts: Vec::new(),
            static_data: Vec::new(),
            config_schema: serde_json::Value::Object(serde_json::Map::new()),
            default_config: serde_json::Value::Object(serde_json::Map::new()),
            seo: None,
        }
    }
    
    /// Get component directory path
    pub fn get_directory_path(&self, theme_dir: &std::path::Path) -> std::path::PathBuf {
        let category_dir = match self.category {
            ComponentCategory::Atomic => "atomic",
            ComponentCategory::Composite => "composite",
            ComponentCategory::Layout => "layouts",
            ComponentCategory::Content => "content",        };
        
        theme_dir.join("components").join(category_dir).join(&self.name)
    }
    
    /// Validate component configuration
    pub fn validate(&self) -> Result<(), crate::core::Error> {
        // Check required fields
        if self.name.is_empty() {
            return Err(crate::core::Error::Component("Component name cannot be empty".to_string()));
        }
        
        if self.templates.is_empty() {
            return Err(crate::core::Error::Component("Component must have at least one template".to_string()));
        }
        
        // Validate version format
        ComponentVersion::from_string(&self.version)?;
        
        Ok(())
    }
    
    /// Check if component has a specific prop
    pub fn has_prop(&self, prop_name: &str) -> bool {
        self.props.contains_key(prop_name)
    }
    
    /// Get prop configuration
    pub fn get_prop(&self, prop_name: &str) -> Option<&PropConfig> {
        self.props.get(prop_name)
    }
    
    /// Check if component has a specific slot
    pub fn has_slot(&self, slot_name: &str) -> bool {
        self.slots.iter().any(|slot| slot.name == slot_name)
    }
    
    /// Get slot configuration
    pub fn get_slot(&self, slot_name: &str) -> Option<&SlotConfig> {
        self.slots.iter().find(|slot| slot.name == slot_name)
    }
}