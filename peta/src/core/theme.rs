//! Core theme system integration

use crate::components::ThemeManager;
use crate::core::{Result, SiteConfig};
use std::path::{Path, PathBuf};

/// Theme representation
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub path: PathBuf,
    pub templates_dir: PathBuf,
    pub assets_dir: PathBuf,
}

impl Theme {
    /// Create a new theme
    pub fn new(name: &str, path: impl Into<PathBuf>) -> Self {
        let path_buf = path.into();
        Self {
            name: name.to_string(),
            templates_dir: path_buf.join("templates"),
            assets_dir: path_buf.join("assets"),
            path: path_buf,
        }
    }
    
    /// Load theme from directory
    pub fn load(name: &str, path: impl Into<PathBuf>) -> Result<Self> {
        let path_buf = path.into();
        if !path_buf.exists() {
            return Err(crate::core::Error::theme(format!("Theme directory not found: {:?}", path_buf)));
        }
        
        Ok(Self::new(name, path_buf))
    }
    
    /// Get theme path
    pub fn path(&self) -> &Path {
        &self.path
    }
    
    /// Check if template exists
    pub fn has_template(&self, template: &str) -> bool {
        self.templates_dir.join(template).exists()
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new("default", "themes/default")
    }
}

/// Theme system integration for site building
#[derive(Clone)]
pub struct ThemeSystem {
    manager: ThemeManager,
    current_theme: String,
}

impl ThemeSystem {
    /// Create a new theme system
    pub fn new<P: AsRef<Path>>(themes_dir: P) -> Self {
        Self {
            manager: ThemeManager::new(themes_dir),
            current_theme: "default".to_string(),
        }
    }

    /// Initialize theme system with site configuration
    pub fn initialize(&mut self, _config: &SiteConfig) -> Result<()> {
        let theme_name = "default"; // For now, use default theme
        
        self.load_theme(theme_name)?;
        self.current_theme = theme_name.to_string();
        
        Ok(())
    }

    /// Load a theme
    pub fn load_theme(&mut self, theme_name: &str) -> Result<()> {
        let theme_path = Path::new("themes").join(theme_name);
        self.manager.load_theme(theme_name, theme_path)
    }

    /// Get current theme name
    pub fn current_theme(&self) -> &str {
        &self.current_theme
    }

    /// Get theme manager
    pub fn manager(&self) -> &ThemeManager {
        &self.manager
    }

    /// Get mutable theme manager
    pub fn manager_mut(&mut self) -> &mut ThemeManager {
        &mut self.manager
    }

    /// Validate current theme
    pub fn validate_current_theme(&self) -> Result<()> {
        self.manager.validate_theme(&self.current_theme)
    }

    /// Get theme variables
    pub fn get_variables(&self) -> std::collections::HashMap<String, String> {
        self.manager.get_theme_variables(&self.current_theme)
    }
}