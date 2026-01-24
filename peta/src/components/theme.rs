//! Theme management system for component-based themes

use crate::core::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Theme configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Theme name
    pub name: String,
    /// Theme version
    pub version: String,
    /// Theme description
    pub description: Option<String>,
    /// Parent theme to inherit from
    pub extends: Option<String>,
    /// Theme variables (CSS custom properties)
    pub variables: HashMap<String, String>,
    /// Component configurations
    pub components: HashMap<String, ComponentConfig>,
    /// Asset configuration
    pub assets: AssetConfig,
}

/// Component configuration within a theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentConfig {
    /// Component name
    pub name: String,
    /// Component version
    pub version: Option<String>,
    /// Component template path
    pub template: Option<String>,
    /// Component styles
    pub styles: Vec<String>,
    /// Component scripts
    pub scripts: Vec<String>,
    /// Component-specific configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Component variants
    pub variants: HashMap<String, VariantConfig>,
}

/// Component variant configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantConfig {
    /// Variant name
    pub name: String,
    /// Variant template override
    pub template: Option<String>,
    /// Variant additional styles
    pub styles: Vec<String>,
    /// Variant configuration overrides
    pub config: HashMap<String, serde_json::Value>,
}

/// Asset configuration for themes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetConfig {
    /// CSS files to include
    pub css: Vec<String>,
    /// JavaScript files to include
    pub js: Vec<String>,
    /// Image directories
    pub images: Vec<String>,
    /// Font directories
    pub fonts: Vec<String>,
    /// Asset optimization settings
    pub optimization: OptimizationConfig,
}

/// Asset optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Minify CSS
    pub minify_css: bool,
    /// Minify JS
    pub minify_js: bool,
    /// Optimize images
    pub optimize_images: bool,
    /// Generate critical CSS
    pub critical_css: bool,
    /// Image quality (0-100)
    pub image_quality: u8,
}

/// Theme manager for loading and managing themes
#[derive(Clone)]
pub struct ThemeManager {
    themes: HashMap<String, ThemeConfig>,
    theme_paths: HashMap<String, PathBuf>,
    base_path: PathBuf,
}

impl ThemeManager {
    /// Create a new theme manager
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            themes: HashMap::new(),
            theme_paths: HashMap::new(),
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    /// Load a theme from the given path
    pub fn load_theme<P: AsRef<Path>>(&mut self, theme_name: &str, theme_path: P) -> Result<()> {
        let theme_path = theme_path.as_ref();
        let config_path = theme_path.join("theme.yaml");
        
        if !config_path.exists() {
            return Err(crate::core::Error::theme(format!(
                "Theme configuration not found: {}",
                config_path.display()
            )));
        }

        let config_content = std::fs::read_to_string(&config_path)?;
        let theme_config: ThemeConfig = serde_yaml::from_str(&config_content)
            .map_err(|e| crate::core::Error::theme(format!(
                "Failed to parse theme config: {}", e
            )))?;

        self.themes.insert(theme_name.to_string(), theme_config.clone());
        self.theme_paths.insert(theme_name.to_string(), theme_path.to_path_buf());

        // Load parent theme if specified
        if let Some(parent_theme) = &theme_config.extends {
            if !self.themes.contains_key(parent_theme) {
                let parent_path = self.base_path.join(parent_theme);
                self.load_theme(parent_theme, parent_path)?;
            }
        }

        Ok(())
    }

    /// Get theme configuration
    pub fn get_theme(&self, theme_name: &str) -> Option<&ThemeConfig> {
        self.themes.get(theme_name)
    }

    /// Get theme path
    pub fn get_theme_path(&self, theme_name: &str) -> Option<&PathBuf> {
        self.theme_paths.get(theme_name)
    }

    /// Resolve component configuration with inheritance
    pub fn resolve_component_config(&self, theme_name: &str, component_name: &str) -> Option<ComponentConfig> {
        let theme = self.get_theme(theme_name)?;
        
        // Start with current theme's component config
        let mut config = theme.components.get(component_name).cloned();
        
        // Check parent theme for inheritance
        if let Some(parent_theme) = &theme.extends {
            let parent_config = self.resolve_component_config(parent_theme, component_name);
            
            match (config, parent_config) {
                (Some(mut current), Some(parent)) => {
                    // Merge configurations
                    if current.template.is_none() {
                        current.template = parent.template;
                    }
                    current.styles.extend(parent.styles);
                    current.scripts.extend(parent.scripts);
                    
                    // Merge variants
                    for (variant_name, variant) in parent.variants {
                        if !current.variants.contains_key(&variant_name) {
                            current.variants.insert(variant_name, variant);
                        }
                    }
                    
                    // Merge config
                    for (key, value) in parent.config {
                        if !current.config.contains_key(&key) {
                            current.config.insert(key, value);
                        }
                    }
                    
                    config = Some(current);
                }
                (None, Some(parent)) => config = Some(parent),
                (Some(current), None) => config = Some(current),
                (None, None) => config = None,
            }
        }
        
        config
    }

    /// Get all theme variables with inheritance
    pub fn get_theme_variables(&self, theme_name: &str) -> HashMap<String, String> {
        let mut variables = HashMap::new();
        
        if let Some(theme) = self.get_theme(theme_name) {
            // Start with parent theme variables
            if let Some(parent_theme) = &theme.extends {
                variables = self.get_theme_variables(parent_theme);
            }
            
            // Override with current theme variables
            variables.extend(theme.variables.clone());
        }
        
        variables
    }

    /// List all loaded themes
    pub fn list_themes(&self) -> Vec<&String> {
        self.themes.keys().collect()
    }

    /// Validate theme configuration
    pub fn validate_theme(&self, theme_name: &str) -> Result<()> {
        let theme = self.get_theme(theme_name)
            .ok_or_else(|| crate::core::Error::theme(format!("Theme not found: {}", theme_name)))?;

        // Validate parent theme exists
        if let Some(parent_theme) = &theme.extends {
            if !self.themes.contains_key(parent_theme) {
                return Err(crate::core::Error::theme(format!(
                    "Parent theme not found: {}", parent_theme
                )));
            }
        }

        // Validate component paths
        let theme_path = self.get_theme_path(theme_name).unwrap();
        for component_name in theme.components.keys() {
            let component_path = theme_path.join("components").join(component_name);
            if !component_path.exists() {
                return Err(crate::core::Error::theme(format!(
                                    "Component path not found: {}",
                                    component_path.display()
                 )));            }
        }

        Ok(())
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "Default Theme".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            extends: None,
            variables: HashMap::new(),
            components: HashMap::new(),
            assets: AssetConfig::default(),
        }
    }
}

impl Default for AssetConfig {
    fn default() -> Self {
        Self {
            css: vec!["main.css".to_string()],
            js: vec!["main.js".to_string()],
            images: vec!["images".to_string()],
            fonts: vec!["fonts".to_string()],
            optimization: OptimizationConfig::default(),
        }
    }
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            minify_css: true,
            minify_js: true,
            optimize_images: true,
            critical_css: false,
            image_quality: 85,
        }
    }
}