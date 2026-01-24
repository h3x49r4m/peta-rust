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

/// Register all functions with Tera
pub fn register(tera: &mut Tera) {
    tera.register_function("url", UrlFunction);
    tera.register_function("now", NowFunction);
}