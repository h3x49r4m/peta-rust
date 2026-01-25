//! Main RST parser implementation following RST-first architecture

use crate::content::{RstContent, ContentMetadata, ContentType};
use crate::content::rst::{MathRenderer, CodeHighlighter, DirectiveHandler, toc_generator::TocGenerator, MathProcessor, MathDetectionResult};
use crate::core::Result;
use crate::core::Error;
use std::collections::HashMap;
use regex::Regex;

/// Main RST parser that processes RST content directly to HTML
pub struct RstParser {
    math_renderer: MathRenderer,
    math_processor: MathProcessor,
    #[allow(dead_code)]
    code_highlighter: CodeHighlighter,
    directive_handlers: HashMap<String, Box<dyn DirectiveHandler>>,
    toc_generator: TocGenerator,
}

impl RstParser {
    /// Create a new RST parser with default handlers
    pub fn new() -> crate::core::Result<Self> {
        let mut directive_handlers: HashMap<String, Box<dyn DirectiveHandler>> = HashMap::new();
        
        // Register default directive handlers
        directive_handlers.insert("code-block".to_string(), Box::new(crate::content::rst::directives::CodeBlockHandler::new()));
        directive_handlers.insert("snippet-card".to_string(), Box::new(crate::content::rst::directives::SnippetCardHandler));
        directive_handlers.insert("toctree".to_string(), Box::new(crate::content::rst::directives::TocTreeHandler));
        
Ok(Self {
            math_renderer: MathRenderer::new(),
            math_processor: MathProcessor::new()?,
            code_highlighter: CodeHighlighter::new().map_err(|e| Error::Content(format!("Failed to create code highlighter: {}", e)))?,
            directive_handlers,
            toc_generator: TocGenerator::new(),
        })
    }
    
    /// Parse RST content to HTML following RST-first architecture
    pub fn parse(&mut self, content: &str) -> Result<RstContent> {
        self.parse_with_type(content, None)
    }
    
    /// Parse RST content with optional content type override
    pub fn parse_with_type(&mut self, content: &str, content_type_override: Option<ContentType>) -> Result<RstContent> {
        // 1. Extract frontmatter
        let (frontmatter, rst_content) = self.extract_frontmatter(content)?;
        
        // 2. Extract metadata from frontmatter with optional type override
        let metadata = self.extract_metadata_with_type(&frontmatter, content_type_override)?;
        
        // 3. Parse RST structure and process directives
        let processed_html = self.process_rst_content(&rst_content)?;
        
        // 4. Generate table of contents
        let toc = self.toc_generator.generate(&processed_html)?;
        
        // 5. Detect math formulas and generate rendering script
        let math_detection = self.math_processor.auto_detect_math_content(&processed_html)?;
        let math_script = if math_detection.has_formulas {
            Some(self.math_renderer.generate_on_demand_script(&math_detection))
        } else {
            None
        };
        
        Ok(RstContent {
            metadata,
            html: processed_html,
            toc,
            frontmatter,
            has_math_formulas: math_detection.has_formulas,
            math_formula_count: math_detection.formula_count,
            math_render_script: math_script,
        })
    }
    
    /// Extract YAML frontmatter from RST content
    fn extract_frontmatter(&self, content: &str) -> Result<(HashMap<String, serde_json::Value>, String)> {
        // Simple approach: split on the first occurrence of "---\n"
        if content.starts_with("---\n") {
            let without_prefix = &content[4..]; // Remove "---\n"
            if let Some(pos) = without_prefix.find("\n---\n") {
                let frontmatter_str = &without_prefix[..pos];
                let rst_content = &without_prefix[pos + 5..]; // Skip "\n---\n"
                
                let frontmatter: HashMap<String, serde_json::Value> = serde_yaml::from_str(frontmatter_str)
                    .map_err(|e| crate::core::Error::rst_parse(format!("Failed to parse frontmatter: {}", e)))?;
                
                return Ok((frontmatter, rst_content.to_string()));
            }
        }
        
        // No frontmatter found
        Ok((HashMap::new(), content.to_string()))
    }
    
    /// Extract metadata from frontmatter
    #[allow(dead_code)]
    fn extract_metadata(&self, frontmatter: &HashMap<String, serde_json::Value>) -> Result<ContentMetadata> {
        self.extract_metadata_with_type(frontmatter, None)
    }
    
    fn extract_metadata_with_type(&self, frontmatter: &HashMap<String, serde_json::Value>, content_type_override: Option<ContentType>) -> Result<ContentMetadata> {
        let title = frontmatter.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Untitled")
            .to_string();
        
        let content_type = content_type_override.or_else(|| {
            frontmatter.get("type")
                .and_then(|v| v.as_str())
                .map(|s| ContentType::from_string(s))
        }).unwrap_or(ContentType::Article);
        
        let date = frontmatter.get("date")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let tags = frontmatter.get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();
        
        let author = frontmatter.get("author")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let excerpt = frontmatter.get("excerpt")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        // Generate URL from title
        let url = self.generate_url(&title, &content_type);
        
        // Generate ID from title
        let id = self.generate_id(&title);
        
        Ok(ContentMetadata {
            id,
            title,
            content_type,
            date,
            tags,
            author,
            excerpt,
            url,
            extra: HashMap::new(),
        })
    }
    
    /// Generate URL from title and content type
    fn generate_url(&self, title: &str, content_type: &ContentType) -> String {
        let slug = self.slugify(title);
        match content_type {
            ContentType::Article => format!("articles/{}.html", slug),
            ContentType::Book => format!("books/{}/index.html", slug),
            ContentType::Snippet => format!("snippets/{}.html", slug),
            ContentType::Project => format!("projects/{}.html", slug),
        }
    }
    
    /// Generate ID from title
    fn generate_id(&self, title: &str) -> String {
        self.slugify(title)
    }
    
    /// Convert title to URL-friendly slug
    fn slugify(&self, title: &str) -> String {
        let mut result = title.to_lowercase();
        
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
        result.trim_matches('-').to_string()
    }
    
    /// Process RST content and convert to HTML
    fn process_rst_content(&mut self, content: &str) -> Result<String> {
        let mut processed = content.to_string();
        
        // Process directives first
        processed = self.process_directives(&processed)?;
        
        // Process math equations
        processed = self.math_renderer.render(&processed)?;
        
        // Process code blocks (only actual code blocks, not entire content)
        // This is handled in the directive processing, so we don't apply it to the entire document
        
        // Convert RST markup to HTML (skip directive content)
        processed = self.convert_rst_to_html(&processed)?;
        
        Ok(processed)
    }
    
    
    
    /// Process RST directives
    
        fn process_directives(&mut self, content: &str) -> Result<String> {
    
                // Use a simpler approach - split by directive patterns
    
                let directive_start_regex = Regex::new(r"\.\. ([a-zA-Z0-9_-]+)::")
                    .map_err(|e| crate::core::Error::rst_parse(format!("Failed to compile directive regex: {}", e)))?;
    
                
    
                let mut result = content.to_string();
    
                
    
                // Find all directive starts
    
                let mut directive_starts = Vec::new();
    
                for mat in directive_start_regex.find_iter(content) {
    
                    directive_starts.push((mat.start(), mat.end(), mat.as_str()));
    
                }
    
                
    
                // Process each directive
    
                for (i, &(start, end, directive_str)) in directive_starts.iter().enumerate() {
    
                    // Extract directive type from the directive string
    
                    let directive_type = directive_str
    
                        .trim_start_matches(".. ")
    
                        .trim_end_matches("::")
    
                        .trim();
    
                    
    
                    // Find end of directive content (next directive or end of file)
    
                    let content_start = end;
    
                    let content_end = if i + 1 < directive_starts.len() {
    
                        directive_starts[i + 1].0
    
                    } else {
    
                        content.len()
    
                    };
    
                    
    
                    let directive_content = &content[content_start..content_end];
    
                    
    
                    // Process directive if we have a handler
    
                    if let Some(handler) = self.directive_handlers.get_mut(directive_type) {
    
                        let processed = handler.handle(directive_content)?;
    
                        let original = &content[start..content_end];
    
                        result = result.replace(original, &processed);
    
                    }
    
                }
    
                
    
                Ok(result)
    
                
    
                    }
    
    /// Convert RST markup to HTML
    fn convert_rst_to_html(&self, content: &str) -> Result<String> {
        let mut html = content.to_string();
        
        // Convert headers
        html = self.convert_headers(&html)?;
        
        // Convert emphasis
        html = self.convert_emphasis(&html)?;
        
        // Convert links
        html = self.convert_links(&html)?;
        
        // Convert lists
        html = self.convert_lists(&html)?;
        
        // Convert paragraphs
        html = self.convert_paragraphs(&html)?;
        
        Ok(html)
    }
    
    /// Convert RST headers to HTML
    fn convert_headers(&self, content: &str) -> Result<String> {
        let mut result = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i];
            
            // Check for RST-style underlined headers
            if i + 1 < lines.len() {
                let next_line = lines[i + 1];
                if (next_line.chars().all(|c| c == '=') || next_line.chars().all(|c| c == '-')) && 
                   next_line.len() >= line.len() / 2 {
                    let level = if next_line.chars().all(|c| c == '=') { "2" } else { "3" };
                    result.push(format!("<h{}>{}</h{}>", level, line.trim(), level));
                    i += 2; // Skip both the title and underline
                    continue;
                }
            }
            
            // Check for markdown-style headers as fallback
            let header_regex = Regex::new(r"^(#{1,6})\s+(.+)$")
                    .map_err(|e| crate::core::Error::rst_parse(format!("Failed to compile header regex: {}", e)))?;
            if let Some(captures) = header_regex.captures(line) {
                let level = captures.get(1).unwrap().as_str().len();
                let title = captures.get(2).unwrap().as_str();
                let anchor = self.slugify(title);
                result.push(format!("<h{} id=\"{}\">{}</h{}>", level, anchor, title, level));
                i += 1;
            } else {
                result.push(line.to_string());
                i += 1;
            }
        }
        
        Ok(result.join("\n"))
    }
    
    /// Convert RST emphasis to HTML
    fn convert_emphasis(&self, content: &str) -> Result<String> {
        let mut html = content.to_string();
        
        // Bold text
        html = regex::Regex::new(r"\*\*([^*]+)\*\*")
            .map_err(|e| crate::core::Error::rst_parse(format!("Failed to compile bold regex: {}", e)))?
            .replace_all(&html, "<strong>$1</strong>")
            .to_string();
        
        // Italic text
        html = regex::Regex::new(r"\*([^*]+)\*")
            .map_err(|e| crate::core::Error::rst_parse(format!("Failed to compile italic regex: {}", e)))?
            .replace_all(&html, "<em>$1</em>")
            .to_string();
        
        // Inline code
        html = regex::Regex::new(r"``([^`]+)``")
            .map_err(|e| crate::core::Error::rst_parse(format!("Failed to compile code regex: {}", e)))?
            .replace_all(&html, "<code>$1</code>")
            .to_string();
        
        Ok(html)
    }
    
    /// Convert RST links to HTML
    fn convert_links(&self, content: &str) -> Result<String> {
        let link_regex = Regex::new(r"`([^`]+)<([^>]+)>`_")
                    .map_err(|e| crate::core::Error::rst_parse(format!("Failed to compile link regex: {}", e)))?;
        
        let html = link_regex
            .replace_all(content, r#"<a href="$2">$1</a>"#)
            .to_string();
        
        Ok(html)
    }
    
    /// Convert RST lists to HTML with proper nesting support
    fn convert_lists(&self, content: &str) -> Result<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();
        let mut list_stack = Vec::new(); // Stack to track nested lists
        let mut last_indent = 0;
        
        for line in lines {
            let trimmed = line.trim();
            
            // Skip empty lines and lines that don't look like list items
            if trimmed.is_empty() || !self.is_list_item(line) {
                // Close all open lists when we encounter a non-list line
                while !list_stack.is_empty() {
                    let (list_type, _) = list_stack.pop().unwrap();
                    result.push(format!("</{}>", list_type));
                }
                last_indent = 0;
                result.push(line.to_string());
                continue;
            }
            
            // Calculate current indentation (in spaces)
            let current_indent = self.calculate_indent(line);
            let (list_type, item_content) = self.parse_list_item(line)?;
            
            // If we're at a different indentation level, adjust the list stack
            if current_indent > last_indent {
                // We're going deeper - open a new list
                result.push(format!("<{}>", list_type));
                list_stack.push((list_type, current_indent));
            } else if current_indent < last_indent {
                // We're going up - close lists until we're at the right level
                while let Some((stack_type, stack_indent)) = list_stack.last() {
                    if *stack_indent > current_indent {
                        result.push(format!("</{}>", stack_type));
                        list_stack.pop();
                    } else {
                        break;
                    }
                }
                
                // If the top list type is different, close it and open the new one
                if let Some((stack_type, _)) = list_stack.last() {
                    if *stack_type != list_type {
                        result.push(format!("</{}>", stack_type));
                        list_stack.pop();
                        result.push(format!("<{}>", list_type));
                        list_stack.push((list_type, current_indent));
                    }
                }
            } else {
                // Same indentation level
                if let Some((stack_type, _)) = list_stack.last() {
                    if *stack_type != list_type {
                        result.push(format!("</{}>", stack_type));
                        list_stack.pop();
                        result.push(format!("<{}>", list_type));
                        list_stack.push((list_type, current_indent));
                    }
                } else {
                    // No open list, start a new one
                    result.push(format!("<{}>", list_type));
                    list_stack.push((list_type, current_indent));
                }
            }
            
            result.push(format!("<li>{}</li>", item_content));
            last_indent = current_indent;
        }
        
        // Close any remaining open lists
        while !list_stack.is_empty() {
            let (list_type, _) = list_stack.pop().unwrap();
            result.push(format!("</{}>", list_type));
        }
        
        Ok(result.join("\n"))
    }
    
    /// Check if a line is a list item
    fn is_list_item(&self, line: &str) -> bool {
        let trimmed = line.trim();
        // Check for unordered list items
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            return true;
        }
        
        // Check for ordered list items
        if let Some(first_char) = trimmed.chars().next() {
            if first_char.is_numeric() {
                if let Some(dot_pos) = trimmed.find('.') {
                    // Check if everything before the dot is numeric
                    let before_dot = &trimmed[..dot_pos];
                    if before_dot.chars().all(|c| c.is_numeric()) {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    /// Calculate the indentation level of a line
    fn calculate_indent(&self, line: &str) -> usize {
        line.len() - line.trim_start().len()
    }
    
    /// Parse a list item and return (list_type, content)
    fn parse_list_item(&self, line: &str) -> Result<(String, String)> {
        let trimmed = line.trim();
        
        // Check for unordered list items
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            let content = trimmed.trim_start_matches("- ").trim_start_matches("* ");
            return Ok(("ul".to_string(), content.to_string()));
        }
        
        // Check for ordered list items
        if let Some(first_char) = trimmed.chars().next() {
            if first_char.is_numeric() {
                if let Some(dot_pos) = trimmed.find('.') {
                    let before_dot = &trimmed[..dot_pos];
                    if before_dot.chars().all(|c| c.is_numeric()) {
                        let content = trimmed[dot_pos + 1..].trim();
                        return Ok(("ol".to_string(), content.to_string()));
                    }
                }
            }
        }
        
        Err(crate::core::Error::rst_parse("Invalid list item format".to_string()))
    }
    
    /// Convert RST paragraphs to HTML
    fn convert_paragraphs(&self, content: &str) -> Result<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();
        let mut paragraph = Vec::new();
        let mut in_html_block = false;
        let mut html_block_depth = 0; // Track nested HTML blocks
        
        for line in lines {
            let trimmed = line.trim();
            
            // Track HTML block depth
            if trimmed.starts_with("<div") {
                if html_block_depth == 0 && !paragraph.is_empty() {
                    result.push(format!("<p>{}</p>", paragraph.join(" ")));
                    paragraph.clear();
                }
                html_block_depth += 1;
                in_html_block = true;
                result.push(line.to_string());
            } else if trimmed.starts_with("</div>") && in_html_block {
                html_block_depth -= 1;
                result.push(line.to_string());
                if html_block_depth == 0 {
                    in_html_block = false;
                }
            } else if in_html_block {
                // Inside an HTML block, preserve the line as-is
                result.push(line.to_string());
            } else if self.is_html_tag(trimmed) {
                // Single-line HTML tag
                if !paragraph.is_empty() {
                    result.push(format!("<p>{}</p>", paragraph.join(" ")));
                    paragraph.clear();
                }
                result.push(line.to_string());
            } else if trimmed.is_empty() {
                if !paragraph.is_empty() {
                    result.push(format!("<p>{}</p>", paragraph.join(" ")));
                    paragraph.clear();
                }
            } else if !trimmed.starts_with("    ") && !trimmed.starts_with("\t") {
                // Regular text line
                paragraph.push(trimmed);
            } else {
                // Indented content (should be preserved as-is)
                if !paragraph.is_empty() {
                    result.push(format!("<p>{}</p>", paragraph.join(" ")));
                    paragraph.clear();
                }
                result.push(line.to_string());
            }
        }
        
        if !paragraph.is_empty() {
            result.push(format!("<p>{}</p>", paragraph.join(" ")));
        }
        
        Ok(result.join("\n"))
    }
    
/// Check if a line is already an HTML tag
    fn is_html_tag(&self, line: &str) -> bool {
        line.starts_with("<") && line.ends_with(">") || 
        line.starts_with("<div") || 
        line.starts_with("</div") ||
        line.starts_with("<pre") ||
        line.starts_with("</pre") ||
        line.starts_with("<code") ||
        line.starts_with("</code") ||
        line.starts_with("<span") ||
        line.starts_with("</span") ||
        line.starts_with("<button") ||
        line.starts_with("</button")
    }
    
    /// Parse RST content with math detection
    pub fn parse_with_math_detection(&mut self, content: &str) -> Result<(RstContent, MathDetectionResult)> {
        // Process content with math detection
        let (processed_content, math_detection) = self.math_processor.process_with_detection(content)?;
        
        // Parse the processed content
        let mut rst_content = self.parse(&processed_content)?;
        
        // Add math detection metadata
        rst_content.metadata.extra.insert("has_math_formulas".to_string(), math_detection.has_formulas.to_string());
        rst_content.metadata.extra.insert("math_formula_count".to_string(), math_detection.formula_count.to_string());
        
        Ok((rst_content, math_detection))
    }
}

impl Default for RstParser {
    fn default() -> Self {
        Self::new().expect("Failed to create RstParser")
    }
}
