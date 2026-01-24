//! Component configuration structures for V4 architecture

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Component category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComponentCategory {
    /// Basic building blocks
    Atomic,
    /// Complex UI components
    Composite,
    /// Content components
    Content,    /// Page structure templates
    Layout,
}

/// Prop configuration for component properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropConfig {
    /// Prop type
    pub r#type: String,
    /// Whether prop is required
    pub required: bool,
    /// Default value
    pub default: Option<serde_json::Value>,
    /// Validation rules
    pub validation: Option<ValidationRules>,
    /// Description
    pub description: Option<String>,
}

/// Validation rules for props
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    /// Minimum value (for numbers)
    pub min: Option<f64>,
    /// Maximum value (for numbers)
    pub max: Option<f64>,
    /// Minimum length (for strings/arrays)
    pub min_length: Option<usize>,
    /// Maximum length (for strings/arrays)
    pub max_length: Option<usize>,
    /// Pattern for string validation (regex)
    pub pattern: Option<String>,
    /// Enum values
    pub enum_values: Option<Vec<serde_json::Value>>,
}

/// Slot configuration for component content slots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotConfig {
    /// Slot name
    pub name: String,
    /// Slot description
    pub description: Option<String>,
    /// Whether slot is required
    pub required: bool,
    /// Default content
    pub default: Option<String>,
}

/// State configuration for component state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateConfig {
    /// State name
    pub name: String,
    /// State type
    pub r#type: String,
    /// Default value
    pub default: serde_json::Value,
    /// Whether state is persistent
    pub persistent: bool,
}

/// Component configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentConfig {
    /// Component name
    pub name: String,
    /// Component version
    pub version: String,
    /// Component category
    pub category: ComponentCategory,
    /// Component description
    pub description: String,
    /// Whether component is enabled
    pub enabled: bool,
    /// Required dependencies
    pub dependencies: Vec<String>,
    /// Component props
    pub props: HashMap<String, PropConfig>,
    /// Component slots
    pub slots: Vec<SlotConfig>,
    /// Component state
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

/// SEO configuration for components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoConfig {
    /// Structured data type
    pub structured_data: Option<String>,
    /// Properties for structured data
    pub properties: HashMap<String, serde_json::Value>,
}

/// Site component configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteComponentConfig {
    /// Enabled components
    pub enabled_components: Vec<String>,
    /// Component-specific configurations
    pub component_configs: HashMap<String, serde_json::Value>,
    /// Default layout
    pub layout: String,
    /// Active theme
    pub theme: String,
}

impl ComponentConfig {
    /// Create a new component configuration
    pub fn new(name: String, version: String, category: ComponentCategory) -> Self {
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
    
    /// Add a prop to the component
    pub fn add_prop(&mut self, name: String, prop_config: PropConfig) {
        self.props.insert(name, prop_config);
    }
    
    /// Add a slot to the component
    pub fn add_slot(&mut self, slot_config: SlotConfig) {
        self.slots.push(slot_config);
    }
    
    /// Add state to the component
    pub fn add_state(&mut self, state_config: StateConfig) {
        self.state.push(state_config);
    }
    
    /// Validate the component configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Component name cannot be empty".to_string());
        }
        
        if self.templates.is_empty() {
            return Err("Component must have at least one template".to_string());
        }
        
        // Validate version format (basic semantic version)
        if !self.version.matches(r"^\d+\.\d+\.\d+").next().is_some() {
            return Err("Version must follow semantic versioning (x.y.z)".to_string());
        }
        
        // Validate required props
        for (prop_name, prop_config) in &self.props {
            if prop_config.required && !self.default_config.get(prop_name).is_some() {
                return Err(format!("Required prop '{}' is missing from default config", prop_name));
            }
        }
        
        Ok(())
    }
}

impl PropConfig {
    /// Create a new prop configuration
    pub fn new(r#type: String) -> Self {
        Self {
            r#type,
            required: false,
            default: None,
            validation: None,
            description: None,
        }
    }
    
    /// Mark prop as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    /// Set default value
    pub fn default(mut self, value: serde_json::Value) -> Self {
        self.default = Some(value);
        self
    }
    
    /// Set description
    pub fn description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }
}

impl SlotConfig {
    /// Create a new slot configuration
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            required: false,
            default: None,
        }
    }
    
    /// Mark slot as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    /// Set default content
    pub fn default(mut self, content: String) -> Self {
        self.default = Some(content);
        self
    }
    
    /// Set description
    pub fn description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }
}

impl StateConfig {
    /// Create a new state configuration
    pub fn new(name: String, r#type: String, default: serde_json::Value) -> Self {
        Self {
            name,
            r#type,
            default,
            persistent: false,
        }
    }
    
    /// Mark state as persistent
    pub fn persistent(mut self) -> Self {
        self.persistent = true;
        self
    }
}

impl Default for SiteComponentConfig {
    fn default() -> Self {
        Self {
            enabled_components: vec![
                "navigation".to_string(),
                "footer".to_string(),
            ],
            component_configs: HashMap::new(),
            layout: "default".to_string(),
            theme: "default".to_string(),
        }
    }
}

impl SiteComponentConfig {
    /// Load component configuration from file
    pub fn load_from_file(path: &std::path::Path) -> Result<Self, crate::core::Error> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| crate::core::Error::Config(format!("Failed to read component config: {}", e)))?;
        
        serde_yaml::from_str(&content)
            .map_err(|e| crate::core::Error::Config(format!("Failed to parse component config: {}", e)))
    }
    
    /// Save component configuration to file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), crate::core::Error> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| crate::core::Error::Config(format!("Failed to serialize component config: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| crate::core::Error::Config(format!("Failed to write component config: {}", e)))
    }
    
    /// Get configuration for a specific component
    pub fn get_component_config(&self, name: &str) -> Option<&serde_json::Value> {
        self.component_configs.get(name)
    }
    
    /// Set configuration for a specific component
    pub fn set_component_config(&mut self, name: String, config: serde_json::Value) {
        self.component_configs.insert(name, config);
    }
    
    /// Check if a component is enabled
    pub fn is_component_enabled(&self, name: &str) -> bool {
        self.enabled_components.contains(&name.to_string())
    }
    
    /// Enable a component
    pub fn enable_component(&mut self, name: &str) {
        if !self.enabled_components.contains(&name.to_string()) {
            self.enabled_components.push(name.to_string());
        }
    }
    
    /// Disable a component
    pub fn disable_component(&mut self, name: &str) {
        self.enabled_components.retain(|c| c != name);
    }
}