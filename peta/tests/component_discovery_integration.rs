//! Integration tests for component discovery system
//!
//! These tests verify that the component discovery system works correctly
//! with the actual project structure.

use std::path::PathBuf;

#[test]
fn test_discover_actual_theme_components() {
    let theme_dir = PathBuf::from("themes/default");
    let mut discovery = peta::components::ComponentDiscovery::new(&theme_dir);
    
    let components = discovery.discover_all();
    assert!(components.is_ok(), "Failed to discover components");
    
    let components = components.unwrap();
    
    // Verify we discovered the expected number of components
    assert_eq!(components.len(), 17, "Expected 17 components");
    
    // Verify specific components exist
    let component_names: Vec<&str> = components.iter().map(|c| c.name.as_str()).collect();
    
    // Atomic components
    assert!(component_names.contains(&"navbar"), "navbar component not found");
    assert!(component_names.contains(&"footer"), "footer component not found");
    assert!(component_names.contains(&"tag_cloud"), "tag_cloud component not found");
    assert!(component_names.contains(&"grid_card"), "grid_card component not found");
    assert!(component_names.contains(&"code_block"), "code_block component not found");
    
    // Composite components
    assert!(component_names.contains(&"header"), "header component not found");
    assert!(component_names.contains(&"page_tags"), "page_tags component not found");
    assert!(component_names.contains(&"grid_cards"), "grid_cards component not found");
    assert!(component_names.contains(&"article_modal"), "article_modal component not found");
}

#[test]
fn test_discover_component_categories() {
    let theme_dir = PathBuf::from("themes/default");
    let mut discovery = peta::components::ComponentDiscovery::new(&theme_dir);
    
    // Test atomic category
    let atomic_components = discovery.get_components_by_category("atomic");
    assert!(atomic_components.is_ok(), "Failed to get atomic components");
    let atomic_components = atomic_components.unwrap();
    assert!(atomic_components.len() >= 10, "Expected at least 10 atomic components");
    
    // Test composite category
    let composite_components = discovery.get_components_by_category("composite");
    assert!(composite_components.is_ok(), "Failed to get composite components");
    let composite_components = composite_components.unwrap();
    assert!(composite_components.len() >= 7, "Expected at least 7 composite components");
}

#[test]
fn test_component_yaml_files_exist() {
    let theme_dir = PathBuf::from("themes/default");
    let mut discovery = peta::components::ComponentDiscovery::new(&theme_dir);
    
    let components = discovery.discover_all().unwrap();
    
    // Verify all components have YAML files
    for component in &components {
        let yaml_path = component.path.join("component.yaml");
        assert!(yaml_path.exists(), "component.yaml missing for {}", component.name);
        assert!(component.has_yaml, "has_yaml flag not set for {}", component.name);
    }
}

#[test]
fn test_component_metadata_from_yaml() {
    let theme_dir = PathBuf::from("themes/default");
    let mut discovery = peta::components::ComponentDiscovery::new(&theme_dir);
    
    // Test navbar component
    let navbar = discovery.get_component("navbar");
    assert!(navbar.is_ok(), "Failed to get navbar component");
    let navbar = navbar.unwrap();
    assert!(navbar.is_some(), "navbar component not found");
    
    let navbar = navbar.unwrap();
    assert_eq!(navbar.name, "navbar");
    assert_eq!(navbar.category, "atomic");
    assert_eq!(navbar.version, "1.0.0");
    assert!(!navbar.description.is_empty());
    assert!(navbar.has_yaml);
    
    // Test header component with dependencies
    let header = discovery.get_component("header");
    assert!(header.is_ok(), "Failed to get header component");
    let header = header.unwrap();
    assert!(header.is_some(), "header component not found");
    
    let header = header.unwrap();
    assert_eq!(header.name, "header");
    assert_eq!(header.category, "composite");
    assert_eq!(header.version, "1.0.0");
    assert!(!header.dependencies.is_empty(), "header should have dependencies");
    assert!(header.dependencies.contains(&"navbar".to_string()), "header should depend on navbar");
}

#[test]
fn test_component_manager_integration() {
    let theme_dir = PathBuf::from("themes/default");
    let mut manager = peta::components::ComponentManager::new(&theme_dir);
    
    // Initialize manager (should discover components)
    let result = manager.initialize();
    assert!(result.is_ok(), "Failed to initialize component manager");
    
    // Verify components are registered
    let all_components = manager.get_all();
    assert!(!all_components.is_empty(), "No components registered");
    assert_eq!(all_components.len(), 17, "Expected 17 registered components");
    
    // Test get_component_info
    let navbar_info = manager.get_component_info("navbar");
    assert!(navbar_info.is_some(), "navbar not found in manager");
    
    let navbar_info = navbar_info.unwrap();
    assert_eq!(navbar_info.name, "navbar");
    assert_eq!(navbar_info.version, "1.0.0");
    
    // Test get_component_category
    let navbar_category = manager.get_component_category("navbar");
    assert!(navbar_category.is_some(), "navbar category not found");
    assert_eq!(navbar_category.unwrap(), "atomic");
    
    let header_category = manager.get_component_category("header");
    assert!(header_category.is_some(), "header category not found");
    assert_eq!(header_category.unwrap(), "composite");
}

#[test]
fn test_component_to_component_conversion() {
    let theme_dir = PathBuf::from("themes/default");
    let mut discovery = peta::components::ComponentDiscovery::new(&theme_dir);
    
    let navbar = discovery.get_component("navbar").unwrap().unwrap();
    let component = navbar.to_component();
    
    assert!(component.is_ok(), "Failed to convert to Component");
    let component = component.unwrap();
    
    assert_eq!(component.name, "navbar");
    assert_eq!(component.version, "1.0.0");
    assert!(!component.templates.is_empty(), "Component should have templates");
    assert!(component.templates.contains(&"navbar.html".to_string()));
}

#[test]
fn test_discovery_cache() {
    let theme_dir = PathBuf::from("themes/default");
    let mut discovery = peta::components::ComponentDiscovery::new(&theme_dir);
    
    // First discovery
    let components1 = discovery.discover_all().unwrap();
    
    // Second discovery (should use cache)
    let components2 = discovery.discover_all().unwrap();
    
    assert_eq!(components1.len(), components2.len());
    
    // Clear cache and rediscover
    discovery.clear_cache();
    let components3 = discovery.discover_all().unwrap();
    
    assert_eq!(components1.len(), components3.len());
}