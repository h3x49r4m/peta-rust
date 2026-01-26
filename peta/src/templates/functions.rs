//! Template functions

use std::collections::HashMap;
use tera::{Result as TeraResult, Value, Function, Tera};
use chrono::Utc;

/// URL function
pub struct UrlFunction;

impl Function for UrlFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let path = args.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| tera::Error::msg("Missing 'path' argument"))?;
        
        // For now, just return the path as-is
        // In a real implementation, you would handle URL generation
        Ok(Value::String(path.to_string()))
    }
}

/// Asset function
pub struct AssetFunction;

impl Function for AssetFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let asset_path = args.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| tera::Error::msg("Missing 'path' argument"))?;
        
        // Generate asset URL
        let url = format!("/assets/{}", asset_path.trim_start_matches('/'));
        Ok(Value::String(url))
    }
}

/// Now function
pub struct NowFunction;

impl Function for NowFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let format = args.get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("%Y-%m-%d %H:%M:%S");
        
        let now = Utc::now();
        let formatted = now.format(format).to_string();
        Ok(Value::String(formatted))
    }
}

/// Component scripts function
pub struct ComponentScriptsFunction;

impl Function for ComponentScriptsFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let component_names = args.get("component_names")
            .and_then(|v| v.as_array())
            .ok_or_else(|| tera::Error::msg("Missing 'component_names' argument"))?;
        
        let mut all_scripts = String::new();
        
        for component_name_value in component_names {
            if let Some(component_name) = component_name_value.as_str() {
                let category = match component_name {
                    "code_block" => "atomic",
                    "navbar" => "atomic",
                    "contacts" => "atomic",
                    "tag_cloud" => "atomic",
                    "grid_card" => "atomic",
                    "content_div" => "atomic",
                    "article_toc" => "atomic",
                    "article_content" => "atomic",
                    "book_toc" => "atomic",
                    "book_content" => "atomic",
                    "header" => "composite",
                    "footer" => "composite",
                    "page_tags" => "composite",
                    "snippet_card_modal" => "composite",
                    "grid_cards" => "composite",
                    "article_modal" => "composite",
                    "book_modal" => "composite",
                    _ => continue,
                };
                
                let script_path = format!("themes/default/components/{}/{}/{}.js", category, component_name, component_name);
                
                if std::path::Path::new(&script_path).exists() {
                    if let Ok(script_content) = std::fs::read_to_string(&script_path) {
                        all_scripts.push_str(&script_content);
                        all_scripts.push('\n');
                    }
                }
            }
        }
        
        Ok(Value::String(all_scripts))
    }
}

/// Register all functions with Tera
pub fn register(tera: &mut Tera) {
    tera.register_function("url", UrlFunction);
    tera.register_function("now", NowFunction);
    tera.register_function("component_scripts", ComponentScriptsFunction);
}