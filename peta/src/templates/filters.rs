//! Template filters

use tera::{Result as TeraResult, Value, Filter, Tera};
use chrono::NaiveDateTime;
use std::collections::HashMap;

/// Date filter
pub struct DateFilter;

impl Filter for DateFilter {
    fn filter(&self, value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let date_str = value.as_str().ok_or_else(|| tera::Error::msg("Expected string"))?;
        
        // Try to parse the date
        let date = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d")
            .or_else(|_| NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S"))
            .map_err(|_| tera::Error::msg("Invalid date format"))?;
        
        // Get format from args or use default
        let format = args.get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("%B %d, %Y");
        
        let formatted = date.format(format).to_string();
        Ok(Value::String(formatted))
    }
}

/// Markdown filter
pub struct MarkdownFilter;

impl Filter for MarkdownFilter {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> TeraResult<Value> {
        let text = value.as_str().ok_or_else(|| tera::Error::msg("Expected string"))?;
        
        // For now, just escape HTML and convert line breaks
        let html = text
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('\n', "<br>");
        
        Ok(Value::String(html))
    }
}

/// Slugify filter
pub struct SlugifyFilter;

impl Filter for SlugifyFilter {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> TeraResult<Value> {
        let text = value.as_str().ok_or_else(|| tera::Error::msg("Expected string"))?;
        
        let slug = text.to_lowercase()
            .replace(&[' ', '-', '_', '.', ',', ';', ':', '!', '?', '@', '#', '$', '%', '^', '&', '*', '(', ')', '+', '=', '[', ']', '{', '}', '\\', '|', '<', '>', '/', '"', '\''][..], "-")
            .replace(&['"', '\''][..], "")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>()
            .trim_matches('-')
            .to_string();
        
        Ok(Value::String(slug))
    }
}

/// Register all filters with Tera
pub fn register(tera: &mut Tera) {
    tera.register_filter("date", DateFilter);
    tera.register_filter("slugify", SlugifyFilter);
}