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
        
        let mut result = text.to_lowercase();
        
        // Handle common programming language notations first
        result = result.replace("c++", "cpp");
        result = result.replace("c#", "csharp");
        result = result.replace("f#", "fsharp");
        result = result.replace("c++/cli", "cpp-cli");
        result = result.replace(".net", "dotnet");
        result = result.replace("node.js", "nodejs");
        result = result.replace("react.js", "reactjs");
        result = result.replace("vue.js", "vuejs");
        result = result.replace("angular.js", "angularjs");
        
        // Replace common symbols with words
        result = result.replace("++", "plus");
        result = result.replace("--", "minus");
        result = result.replace("==", "equals");
        result = result.replace("!=", "not-equals");
        result = result.replace("<=", "less-equal");
        result = result.replace(">=", "greater-equal");
        result = result.replace("->", "arrow");
        result = result.replace("=>", "fat-arrow");
        result = result.replace("&&", "and");
        result = result.replace("||", "or");
        
        // Replace spaces and punctuation with dashes
        result = result.replace(&[' ', '-', '_', '.', ',', ';', ':', '!', '?', '@', '#', '$', '%', '^', '&', '*', '(', ')', '=', '[', ']', '{', '}', '\\', '|', '<', '>', '/', '"', '\''][..], "-");
        
        // Remove quotes completely
        result = result.replace(['"', '\''], "");
        
        // Filter to only keep alphanumeric characters and dashes
        result = result.chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();
        
        // Collapse multiple dashes into single dashes
        while result.contains("--") {
            result = result.replace("--", "-");
        }
        
        // Trim leading/trailing dashes
        let slug = result.trim_matches('-').to_string();
        
        Ok(Value::String(slug))
    }
}

/// Register all filters with Tera
pub fn register(tera: &mut Tera) {
    tera.register_filter("date", DateFilter);
    tera.register_filter("slugify", SlugifyFilter);
}