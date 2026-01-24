//! Component registry for managing component registration and dependencies

use crate::components::Component;
use crate::components::config::ComponentCategory;
use crate::core::{Error, Result};
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

/// Dependency graph for managing component dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    /// Adjacency list representing dependencies
    dependencies: HashMap<String, Vec<String>>,
    /// Reverse adjacency list for dependents
    dependents: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    /// Create a new dependency graph
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }
    
    /// Add a dependency relationship
    pub fn add_dependency(&mut self, component: &str, dependency: &str) {
        self.dependencies
            .entry(component.to_string())
            .or_insert_with(Vec::new)
            .push(dependency.to_string());
        
        self.dependents
            .entry(dependency.to_string())
            .or_insert_with(Vec::new)
            .push(component.to_string());
    }
    
    /// Get dependencies for a component
    pub fn get_dependencies(&self, component: &str) -> Vec<String> {
        self.dependencies
            .get(component)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get dependents for a component
    pub fn get_dependents(&self, component: &str) -> Vec<String> {
        self.dependents
            .get(component)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Resolve dependencies in correct order
    pub fn resolve_dependencies(&self, component: &str) -> Result<Vec<String>> {
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();
        let mut result = Vec::new();
        
        self.resolve_dependencies_recursive(
            component,
            &mut visited,
            &mut visiting,
            &mut result,
        )?;
        
        Ok(result)
    }
    
    /// Recursive dependency resolution with cycle detection
    fn resolve_dependencies_recursive(
        &self,
        component: &str,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<()> {
        if visited.contains(component) {
            return Ok(());
        }
        
        if visiting.contains(component) {
            return Err(Error::Component(format!(
                "Circular dependency detected involving component: {}",
                component
            )));
        }
        
        visiting.insert(component.to_string());
        
        for dependency in self.get_dependencies(component) {
            self.resolve_dependencies_recursive(&dependency, visited, visiting, result)?;
        }
        
        visiting.remove(component);
        visited.insert(component.to_string());
        result.push(component.to_string());
        
        Ok(())
    }
}

/// Component registry for managing registered components
#[derive(Debug, Clone)]
pub struct ComponentRegistry {
    /// Registered components by name
    components: HashMap<String, Component>,
    /// Dependency graph
    dependency_graph: DependencyGraph,
    /// Enabled components
    enabled_components: HashSet<String>,
    /// Component categories for quick lookup
    categories: HashMap<ComponentCategory, Vec<String>>,
}

impl ComponentRegistry {
    /// Create a new component registry
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            dependency_graph: DependencyGraph::new(),
            enabled_components: HashSet::new(),
            categories: HashMap::new(),
        }
    }
    
    /// Register a component
    pub fn register_component(&mut self, component: Component) -> Result<()> {
        // Validate component
        component.validate()?;
        
        let name = component.name.clone();
        
        // Check for conflicts
        if self.components.contains_key(&name) {
            return Err(Error::Component(format!("Component '{}' already registered", name)));
        }
        
        // Add dependencies to graph
        for dependency in &component.dependencies {
            self.dependency_graph.add_dependency(&name, dependency);
        }
        
        // Register component
        self.components.insert(name.clone(), component.clone());
        
        // Add to category
        use std::collections::hash_map::Entry;
        match self.categories.entry(component.category.clone()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(name.clone());
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![name.clone()]);
            }
        }
        
        // Enable by default
        self.enabled_components.insert(name);
        
        Ok(())
    }
    
    /// Get a component by name
    pub fn get_component(&self, name: &str) -> Option<&Component> {
        self.components.get(name)
    }
    
    /// Get all components
    pub fn get_all_components(&self) -> &HashMap<String, Component> {
        &self.components
    }
    
    /// Get enabled components
    pub fn get_enabled_components(&self) -> Vec<&Component> {
        self.enabled_components
            .iter()
            .filter_map(|name| self.components.get(name))
            .collect()
    }
    
    /// Get components by category
    pub fn get_components_by_category(&self, category: &ComponentCategory) -> Vec<&Component> {
        self.categories
            .get(category)
            .map(|names| {
                names.iter()
                    .filter_map(|name| self.components.get(name))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Enable a component
    pub fn enable_component(&mut self, name: &str) -> Result<()> {
        if !self.components.contains_key(name) {
            return Err(Error::ComponentNotFound(name.to_string()));
        }
        
        self.enabled_components.insert(name.to_string());
        
        // Enable dependencies
        let dependencies = self.dependency_graph.get_dependencies(name);
        for dependency in dependencies {
            self.enable_component(&dependency)?;
        }
        
        Ok(())
    }
    
    /// Disable a component
    pub fn disable_component(&mut self, name: &str) -> Result<()> {
        if !self.components.contains_key(name) {
            return Err(Error::ComponentNotFound(name.to_string()));
        }
        
        // Check if any enabled components depend on this one
        let dependents = self.dependency_graph.get_dependents(name);
        for dependent in dependents {
            if self.enabled_components.contains(&dependent) {
                return Err(Error::Component(format!(
                    "Cannot disable component '{}' because '{}' depends on it",
                    name, dependent
                )));
            }
        }
        
        self.enabled_components.remove(name);
        Ok(())
    }
    
    /// Check if a component is enabled
    pub fn is_component_enabled(&self, name: &str) -> bool {
        self.enabled_components.contains(name)
    }
    
    /// Resolve dependencies for a component
    pub fn resolve_dependencies(&self, name: &str) -> Result<Vec<String>> {
        self.dependency_graph.resolve_dependencies(name)
    }
    
    /// Get all dependencies for enabled components
    pub fn get_all_enabled_dependencies(&self) -> Result<Vec<String>> {
        let mut all_dependencies = HashSet::new();
        
        for component_name in &self.enabled_components {
            let dependencies = self.resolve_dependencies(component_name)?;
            all_dependencies.extend(dependencies);
        }
        
        Ok(all_dependencies.into_iter().collect())
    }
    
    /// Validate component dependencies
    pub fn validate_dependencies(&self) -> Result<()> {
        for (name, component) in &self.components {
            for dependency in &component.dependencies {
                if !self.components.contains_key(dependency) {
                    return Err(Error::Component(format!(
                        "Component '{}' depends on '{}' which is not registered",
                        name, dependency
                    )));
                }
            }
        }
        
        // Check for circular dependencies
        for name in self.components.keys() {
            self.dependency_graph.resolve_dependencies(name)?;
        }
        
        Ok(())
    }
    
    /// Get component statistics
    pub fn get_statistics(&self) -> ComponentStats {
        let mut stats = ComponentStats::default();
        
        stats.total_components = self.components.len();
        stats.enabled_components = self.enabled_components.len();
        
        for component in self.components.values() {
            match component.category {
                ComponentCategory::Atomic => stats.atomic_components += 1,
                ComponentCategory::Composite => stats.composite_components += 1,
                ComponentCategory::Layout => stats.layout_components += 1,
            }
        }
        
        stats
    }
    
    /// Unregister a component
    pub fn unregister_component(&mut self, name: &str) -> Result<()> {
        if !self.components.contains_key(name) {
            return Err(Error::ComponentNotFound(name.to_string()));
        }
        
        // Check if any enabled components depend on this one
        let dependents = self.dependency_graph.get_dependents(name);
        for dependent in dependents {
            if self.enabled_components.contains(&dependent) {
                return Err(Error::Component(format!(
                    "Cannot unregister component '{}' because '{}' depends on it",
                    name, dependent
                )));
            }
        }
        
        // Remove from enabled components
        self.enabled_components.remove(name);
        
        // Remove from categories
        if let Some(component) = self.components.get(name) {
            if let Some(category_components) = self.categories.get_mut(&component.category) {
                category_components.retain(|n| n != name);
            }
        }
        
        // Remove from components
        self.components.remove(name);
        
        Ok(())
    }
}

/// Component statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComponentStats {
    /// Total number of registered components
    pub total_components: usize,
    /// Number of enabled components
    pub enabled_components: usize,
    /// Number of atomic components
    pub atomic_components: usize,
    /// Number of composite components
    pub composite_components: usize,
    /// Number of layout components
    pub layout_components: usize,
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_component_registration() {
        let mut registry = ComponentRegistry::new();
        
        let mut component = Component::new(
            "TestComponent".to_string(),
            "1.0.0".to_string(),
            ComponentCategory::Composite,
        );
        component.templates.push("test.html".to_string());
        
        assert!(registry.register_component(component).is_ok());
        assert!(registry.get_component("TestComponent").is_some());
        assert!(registry.is_component_enabled("TestComponent"));
    }
    
    #[test]
    fn test_dependency_resolution() {
        let mut registry = ComponentRegistry::new();
        
        let mut component_a = Component::new(
            "ComponentA".to_string(),
            "1.0.0".to_string(),
            ComponentCategory::Composite,
        );
        component_a.dependencies = vec!["ComponentB".to_string()];
        component_a.templates.push("component_a.html".to_string());
        
        let mut component_b = Component::new(
            "ComponentB".to_string(),
            "1.0.0".to_string(),
            ComponentCategory::Atomic,
        );
        component_b.templates.push("component_b.html".to_string());
        
        assert!(registry.register_component(component_b).is_ok());
        assert!(registry.register_component(component_a).is_ok());
        
        let dependencies = registry.resolve_dependencies("ComponentA").unwrap();
        assert_eq!(dependencies, vec!["ComponentB", "ComponentA"]);
    }
    
    #[test]
    fn test_circular_dependency_detection() {
        let mut registry = ComponentRegistry::new();
        
        let mut component_a = Component::new(
            "ComponentA".to_string(),
            "1.0.0".to_string(),
            ComponentCategory::Composite,
        );
        component_a.dependencies = vec!["ComponentB".to_string()];
        component_a.templates.push("component_a.html".to_string());
        
        let mut component_b = Component::new(
            "ComponentB".to_string(),
            "1.0.0".to_string(),
            ComponentCategory::Composite,
        );
        component_b.dependencies = vec!["ComponentA".to_string()];
        component_b.templates.push("component_b.html".to_string());
        
        assert!(registry.register_component(component_a).is_ok());
        assert!(registry.register_component(component_b).is_err());
    }
}