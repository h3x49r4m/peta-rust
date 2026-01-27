//! Component discovery system for automatic component detection
//!
//! This module provides functionality to automatically discover components
//! from the theme directory by scanning the filesystem and reading component
//! metadata from YAML files.

use crate::components::{Component, ComponentCategory};
use crate::core::{Error, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Component metadata discovered from filesystem
#[derive(Debug, Clone)]
pub struct DiscoveredComponent {
    /// Component name
    pub name: String,
    /// Component category
    pub category: String,
    /// Component version
    pub version: String,
    /// Component description
    pub description: String,
    /// Component dependencies
    pub dependencies: Vec<String>,
    /// Component directory path
    pub path: PathBuf,
    /// Whether YAML file was found
    pub has_yaml: bool,
}

impl DiscoveredComponent {
    /// Convert to Component struct
    pub fn to_component(&self) -> Result<Component> {
        let category = match self.category.as_str() {
            "atomic" => ComponentCategory::Atomic,
            "composite" => ComponentCategory::Composite,
            _ => ComponentCategory::Composite,
        };

        let templates = if self.path.join(format!("{}.html", self.name)).exists() {
            vec![format!("{}.html", self.name)]
        } else {
            vec![]
        };

        let styles = if self.path.join(format!("{}.css", self.name)).exists() {
            vec![format!("{}.css", self.name)]
        } else {
            vec![]
        };

        let scripts = if self.path.join(format!("{}.js", self.name)).exists() {
            vec![format!("{}.js", self.name)]
        } else {
            vec![]
        };

        Ok(Component {
            name: self.name.clone(),
            version: self.version.clone(),
            category,
            description: self.description.clone(),
            enabled: true,
            dependencies: self.dependencies.clone(),
            props: HashMap::new(),
            slots: vec![],
            state: vec![],
            templates,
            styles,
            scripts,
            static_data: vec![],
            config_schema: serde_json::Value::Object(serde_json::Map::new()),
            default_config: serde_json::Value::Object(serde_json::Map::new()),
            seo: None,
        })
    }
}

/// Component discovery system
pub struct ComponentDiscovery {
    theme_dir: PathBuf,
    cache: HashMap<String, Vec<DiscoveredComponent>>,
}

impl ComponentDiscovery {
    /// Create a new component discovery system
    pub fn new(theme_dir: &PathBuf) -> Self {
        Self {
            theme_dir: theme_dir.clone(),
            cache: HashMap::new(),
        }
    }

    /// Discover all components in the theme
    pub fn discover_all(&mut self) -> Result<Vec<DiscoveredComponent>> {
        let components_dir = self.theme_dir.join("components");

        if !components_dir.exists() {
            return Ok(vec![]);
        }

        let mut all_components = Vec::new();

        // Scan all category directories
        for entry in fs::read_dir(&components_dir)
            .map_err(|e| Error::Component(format!("Failed to read components directory: {}", e)))?
        {
            let entry = entry.map_err(|e| Error::Component(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();

            if path.is_dir() {
                let category = path.file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| Error::Component("Invalid category directory name".to_string()))?;

                let category_components = self.discover_category(&path, category)?;
                all_components.extend(category_components);
            }
        }

        // Cache the results
        self.cache.insert("all".to_string(), all_components.clone());

        Ok(all_components)
    }

    /// Discover components in a specific category
    pub fn discover_category(&mut self, category_dir: &Path, category: &str) -> Result<Vec<DiscoveredComponent>> {
        let mut components = Vec::new();

        for entry in fs::read_dir(category_dir)
            .map_err(|e| Error::Component(format!("Failed to read category directory {}: {}", category, e)))?
        {
            let entry = entry.map_err(|e| Error::Component(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(component) = self.discover_component(&path, category)? {
                    components.push(component);
                }
            }
        }

        Ok(components)
    }

    /// Discover a single component
    pub fn discover_component(&self, component_dir: &Path, category: &str) -> Result<Option<DiscoveredComponent>> {
        let component_name = component_dir.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| Error::Component("Invalid component directory name".to_string()))?;

        // Try to read component.yaml
        let yaml_path = component_dir.join("component.yaml");
        let component = if yaml_path.exists() {
            self.read_component_yaml(&yaml_path, component_name, category, component_dir)?
        } else {
            // Infer metadata from directory structure
            self.infer_component_metadata(component_name, category, component_dir)?
        };

        Ok(Some(component))
    }

    /// Read component metadata from YAML file
    fn read_component_yaml(
        &self,
        yaml_path: &Path,
        component_name: &str,
        category: &str,
        component_dir: &Path,
    ) -> Result<DiscoveredComponent> {
        let yaml_content = fs::read_to_string(yaml_path)
            .map_err(|e| Error::Component(format!("Failed to read component.yaml: {}", e)))?;

        // Parse YAML (simple parsing for now)
        let name = self.extract_yaml_value(&yaml_content, "name")
            .unwrap_or_else(|| component_name.to_string());

        let version = self.extract_yaml_value(&yaml_content, "version")
            .unwrap_or_else(|| "1.0.0".to_string());

        let description = self.extract_yaml_value(&yaml_content, "description")
            .unwrap_or_else(|| String::new());

        let dependencies = self.extract_yaml_array(&yaml_content, "dependencies")
            .unwrap_or_else(|| vec![]);

        Ok(DiscoveredComponent {
            name,
            category: category.to_string(),
            version,
            description,
            dependencies,
            path: component_dir.to_path_buf(),
            has_yaml: true,
        })
    }

    /// Infer component metadata from directory structure
    fn infer_component_metadata(
        &self,
        component_name: &str,
        category: &str,
        component_dir: &Path,
    ) -> Result<DiscoveredComponent> {
        Ok(DiscoveredComponent {
            name: component_name.to_string(),
            category: category.to_string(),
            version: "1.0.0".to_string(),
            description: String::new(),
            dependencies: vec![],
            path: component_dir.to_path_buf(),
            has_yaml: false,
        })
    }

    /// Extract a value from YAML content (simple parser)
    fn extract_yaml_value(&self, content: &str, key: &str) -> Option<String> {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with(&format!("{}:", key)) {
                let value = trimmed[key.len() + 1..].trim();
                return Some(value.to_string());
            }
        }
        None
    }

    /// Extract an array from YAML content (simple parser)
    fn extract_yaml_array(&self, content: &str, key: &str) -> Option<Vec<String>> {
        let mut in_array = false;
        let mut values = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with(&format!("{}:", key)) {
                in_array = true;
                continue;
            }

            if in_array {
                if trimmed.starts_with('-') {
                    let value = trimmed[1..].trim().trim_matches('"').to_string();
                    if !value.is_empty() {
                        values.push(value);
                    }
                } else if trimmed.is_empty() || trimmed.starts_with(&format!("{}:", key)) {
                    break;
                }
            }
        }

        if values.is_empty() {
            None
        } else {
            Some(values)
        }
    }

    /// Get component by name
    pub fn get_component(&mut self, name: &str) -> Result<Option<DiscoveredComponent>> {
        let components = self.discover_all()?;
        Ok(components.into_iter().find(|c| c.name == name))
    }

    /// Get components by category
    pub fn get_components_by_category(&mut self, category: &str) -> Result<Vec<DiscoveredComponent>> {
        let components = self.discover_all()?;
        Ok(components.into_iter().filter(|c| c.category == category).collect())
    }

    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_component(
        temp_dir: &Path,
        category: &str,
        name: &str,
        has_yaml: bool,
    ) -> PathBuf {
        let component_dir = temp_dir.join("components").join(category).join(name);
        fs::create_dir_all(&component_dir).unwrap();

        // Create HTML file
        fs::write(
            component_dir.join(format!("{}.html", name)),
            format!("<div>{{{{ props.title }}}}</div>"),
        ).unwrap();

        if has_yaml {
            fs::write(
                component_dir.join("component.yaml"),
                format!(
                    r#"name: {}
category: {}
version: 1.0.0
description: Test component
dependencies: []
"#,
                    name, category
                ),
            ).unwrap();
        }

        component_dir
    }

    #[test]
    fn test_discover_component_with_yaml() {
        let temp_dir = TempDir::new().unwrap();
        let theme_dir = temp_dir.path().to_path_buf();

        create_test_component(&theme_dir, "atomic", "test_component", true);

        let mut discovery = ComponentDiscovery::new(&theme_dir);
        let component = discovery.discover_component(
            &theme_dir.join("components/atomic/test_component"),
            "atomic",
        );

        assert!(component.is_ok());
        let component = component.unwrap().unwrap();
        assert_eq!(component.name, "test_component");
        assert_eq!(component.category, "atomic");
        assert_eq!(component.version, "1.0.0");
        assert!(component.has_yaml);
    }

    #[test]
    fn test_discover_component_without_yaml() {
        let temp_dir = TempDir::new().unwrap();
        let theme_dir = temp_dir.path().to_path_buf();

        create_test_component(&theme_dir, "composite", "test_component", false);

        let mut discovery = ComponentDiscovery::new(&theme_dir);
        let component = discovery.discover_component(
            &theme_dir.join("components/composite/test_component"),
            "composite",
        );

        assert!(component.is_ok());
        let component = component.unwrap().unwrap();
        assert_eq!(component.name, "test_component");
        assert_eq!(component.category, "composite");
        assert_eq!(component.version, "1.0.0");
        assert!(!component.has_yaml);
    }

    #[test]
    fn test_discover_all_components() {
        let temp_dir = TempDir::new().unwrap();
        let theme_dir = temp_dir.path().to_path_buf();

        create_test_component(&theme_dir, "atomic", "navbar", true);
        create_test_component(&theme_dir, "atomic", "footer", false);
        create_test_component(&theme_dir, "composite", "header", true);

        let mut discovery = ComponentDiscovery::new(&theme_dir);
        let components = discovery.discover_all();

        assert!(components.is_ok());
        let components = components.unwrap();
        assert_eq!(components.len(), 3);

        let component_names: Vec<&str> = components.iter().map(|c| c.name.as_str()).collect();
        assert!(component_names.contains(&"navbar"));
        assert!(component_names.contains(&"footer"));
        assert!(component_names.contains(&"header"));
    }

    #[test]
    fn test_discover_category() {
        let temp_dir = TempDir::new().unwrap();
        let theme_dir = temp_dir.path().to_path_buf();

        create_test_component(&theme_dir, "atomic", "test1", true);
        create_test_component(&theme_dir, "atomic", "test2", false);
        create_test_component(&theme_dir, "composite", "test3", true);

        let mut discovery = ComponentDiscovery::new(&theme_dir);
        let components = discovery.discover_category(
            &theme_dir.join("components/atomic"),
            "atomic",
        );

        assert!(components.is_ok());
        let components = components.unwrap();
        assert_eq!(components.len(), 2);
    }

    #[test]
    fn test_get_component_by_name() {
        let temp_dir = TempDir::new().unwrap();
        let theme_dir = temp_dir.path().to_path_buf();

        create_test_component(&theme_dir, "atomic", "navbar", true);

        let mut discovery = ComponentDiscovery::new(&theme_dir);
        let component = discovery.get_component("navbar");

        assert!(component.is_ok());
        let component = component.unwrap();
        assert!(component.is_some());
        assert_eq!(component.unwrap().name, "navbar");

        let component = discovery.get_component("nonexistent");
        assert!(component.is_ok());
        assert!(component.unwrap().is_none());
    }

    #[test]
    fn test_to_component() {
        let temp_dir = TempDir::new().unwrap();
        let theme_dir = temp_dir.path().to_path_buf();

        let component_dir = create_test_component(&theme_dir, "atomic", "test", true);

        let discovered = DiscoveredComponent {
            name: "test".to_string(),
            category: "atomic".to_string(),
            version: "1.0.0".to_string(),
            description: "Test component".to_string(),
            dependencies: vec![],
            path: component_dir.clone(),
            has_yaml: true,
        };

        let component = discovered.to_component();
        assert!(component.is_ok());
        let component = component.unwrap();
        assert_eq!(component.name, "test");
        assert_eq!(component.version, "1.0.0");
        assert_eq!(component.category, ComponentCategory::Atomic);
    }

    #[test]
    fn test_cache() {
        let temp_dir = TempDir::new().unwrap();
        let theme_dir = temp_dir.path().to_path_buf();

        create_test_component(&theme_dir, "atomic", "test", true);

        let mut discovery = ComponentDiscovery::new(&theme_dir);
        let components1 = discovery.discover_all().unwrap();
        let components2 = discovery.discover_all().unwrap();

        assert_eq!(components1.len(), components2.len());

        discovery.clear_cache();
        let components3 = discovery.discover_all().unwrap();

        assert_eq!(components1.len(), components3.len());
    }
}