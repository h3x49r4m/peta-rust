//! RST parser implementation with improved architecture

use crate::content::rst::{
    toc_generator::TocGenerator, CodeHighlighter, directives::DirectiveHandler, MathProcessor, MathRenderer,
};
use crate::content::{ContentMetadata, ContentType, RstContent, TocEntry};
use crate::core::Error;
use crate::core::Result;
use regex::Regex;
use std::collections::HashMap;

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
        directive_handlers.insert(
            "code-block".to_string(),
            Box::new(crate::content::rst::directives::CodeBlockHandler::new()),
        );
        directive_handlers.insert(
            "snippet-card".to_string(),
            Box::new(crate::content::rst::directives::SnippetCardHandler::new()),
        );
        directive_handlers.insert(
            "toctree".to_string(),
            Box::new(crate::content::rst::directives::TocTreeHandler),
        );
        directive_handlers.insert(
            "diagram".to_string(),
            Box::new(crate::content::rst::directives::DiagramHandler::new()
                .map_err(|e| Error::Content(format!("Failed to create DiagramHandler: {}", e)))?),
        );
        directive_handlers.insert(
            "musicscore".to_string(),
            Box::new(crate::content::rst::directives::MusicScoreHandler::new()
                .map_err(|e| Error::Content(format!("Failed to create MusicScoreHandler: {}", e)))?),
        );
        directive_handlers.insert(
            "math".to_string(),
            Box::new(crate::content::rst::directives::MathDirectiveHandler::new()),
        );

        Ok(Self {
            math_renderer: MathRenderer::new(),
            math_processor: MathProcessor::new()?,
            code_highlighter: CodeHighlighter::new()
                .map_err(|e| Error::Content(format!("Failed to create code highlighter: {}", e)))?,
            directive_handlers,
            toc_generator: TocGenerator::new(),
        })
    }

    /// Parse RST content to HTML
    pub fn parse(&mut self, content: &str) -> Result<RstContent> {
        self.parse_with_type(content, None)
    }

    /// Parse RST content with optional content type override
    pub fn parse_with_type(
        &mut self,
        content: &str,
        content_type_override: Option<ContentType>,
    ) -> Result<RstContent> {
        self.parse_with_type_and_path(content, content_type_override, None)
    }

    /// Parse RST content with optional content type override and file path
    pub fn parse_with_type_and_path(
        &mut self,
        content: &str,
        content_type_override: Option<ContentType>,
        file_path: Option<&std::path::Path>,
    ) -> Result<RstContent> {
        // 1. Extract frontmatter
        let (frontmatter, rst_content) = self.extract_frontmatter(content)?;

        // 2. Extract metadata from frontmatter
        let metadata = self.extract_metadata_with_type_and_path(
            &frontmatter,
            content_type_override,
            file_path,
        )?;

        // 3. Parse RST structure and process directives
        let processed_html = self.process_rst_content(&rst_content)?;

        // 4. Generate table of contents
        let (toc, toc_html) = if metadata.content_type == ContentType::Book {
            self.extract_toc_from_toctree(&processed_html)?
        } else if metadata.content_type == ContentType::Article || metadata.content_type == ContentType::Project {
            // Use enhanced TOC generator that includes embedded snippet cards
            let toc_entries = self.toc_generator.generate_with_snippets(&processed_html)?;
            let toc_html = self.toc_generator.render_html(&toc_entries);
            (toc_entries, toc_html)
        } else {
            let toc_entries = self.toc_generator.generate(&processed_html)?;
            let toc_html = self.toc_generator.render_html(&toc_entries);
            (toc_entries, toc_html)
        };

        // 5. Detect math formulas (keep for optimization, but don't generate script)
        let math_detection = self.math_processor.auto_detect_math_content(&processed_html)?;

        Ok(RstContent {
            metadata,
            html: processed_html,
            toc,
            toc_html,
            frontmatter,
            has_math_formulas: math_detection.has_formulas,
            math_formula_count: math_detection.formula_count,
        })
    }

    /// Extract YAML frontmatter from RST content
    fn extract_frontmatter(
        &self,
        content: &str,
    ) -> Result<(HashMap<String, serde_json::Value>, String)> {
        if content.starts_with("---\n") {
            let without_prefix = &content[4..];
            if let Some(pos) = without_prefix.find("\n---\n") {
                let frontmatter_str = &without_prefix[..pos];
                let rst_content = &without_prefix[pos + 5..];

                let frontmatter: HashMap<String, serde_json::Value> =
                    serde_yaml::from_str(frontmatter_str).map_err(|e| {
                        crate::core::Error::rst_parse(format!("Failed to parse frontmatter: {}", e))
                    })?;

                return Ok((frontmatter, rst_content.to_string()));
            }
        }

        Ok((HashMap::new(), content.to_string()))
    }

    /// Extract metadata from frontmatter
    #[allow(dead_code)]
    fn extract_metadata_with_type(
        &self,
        frontmatter: &HashMap<String, serde_json::Value>,
        content_type_override: Option<ContentType>,
    ) -> Result<ContentMetadata> {
        self.extract_metadata_with_type_and_path(frontmatter, content_type_override, None)
    }

    fn extract_metadata_with_type_and_path(
        &self,
        frontmatter: &HashMap<String, serde_json::Value>,
        content_type_override: Option<ContentType>,
        file_path: Option<&std::path::Path>,
    ) -> Result<ContentMetadata> {
        let mut frontmatter_clone = frontmatter.clone();
        if let Some(ct) = content_type_override {
            frontmatter_clone.insert(
                "type".to_string(),
                serde_json::Value::String(ct.to_string()),
            );
        }

        crate::content::metadata::MetadataExtractor::extract_with_path(
            &frontmatter_clone,
            file_path,
        )
    }

    /// Process RST content and convert to HTML
    fn process_rst_content(&mut self, content: &str) -> Result<String> {
        let mut processed = content.to_string();

        // Process directives first (handles .. math:: blocks)
        processed = self.process_directives(&processed)?;
        
        // Process inline roles (handles :math:`...`)
        processed = self.process_roles(&processed)?;

        // Convert RST markup to HTML (including tables)
        processed = self.convert_rst_to_html(&processed)?;

        // Process legacy math syntax (for backward compatibility with $ and $$)
        processed = self.math_renderer.render(&processed)?;

        Ok(processed)
    }

    /// Process RST directives
    fn process_directives(&mut self, content: &str) -> Result<String> {
        let directive_start_regex = Regex::new(r"\.\. ([a-zA-Z0-9_-]+)::").map_err(|e| {
            crate::core::Error::rst_parse(format!("Failed to compile directive regex: {}", e))
        })?;

        let mut result = String::new();
        let mut last_pos = 0;

        let mut directive_starts = Vec::new();

        for mat in directive_start_regex.find_iter(content) {
            directive_starts.push((mat.start(), mat.end(), mat.as_str()));
        }

        for (i, &(start, end, directive_str)) in directive_starts.iter().enumerate() {
            result.push_str(&content[last_pos..start]);

            // Extract directive name and optional language
            // The directive_str is like ".. code-block::"
            // After trimming ".. ", we get "code-block::"
            let directive_full = directive_str.trim_start_matches(".. ");
            
            // Split on "::" to get the directive name
            let directive_name = directive_full.split("::").next().unwrap_or("").trim();
            
            // Extract language from the original content (the part after "::")
            // The content starts at 'end', which is after the "::"
            // So we need to look at the characters between the end of the directive and the start of the indented content
            let content_after_directive = &content[end..];
            let first_newline = content_after_directive.find('\n').unwrap_or(content_after_directive.len());
            let language = content_after_directive[..first_newline].trim();

            // The actual code content starts after the language line
            let content_start = end + first_newline + 1; // +1 to skip the newline
            let lines_after_directive: Vec<&str> = content[content_start..].lines().collect();
            let mut content_end = content.len();
            let mut found_indented_content = false;

            if directive_name == "snippet-card" {
                // For snippet-card, the snippet ID is on the same line as the directive
                // Format: ".. snippet-card:: snippet-id"
                // The language variable already contains the snippet ID (extracted from content_after_directive)
                // Set content_end to content_start (empty content)
                content_end = content_start;
            } else if directive_name == "toctree" {
                // For toctree, skip all indented content and options
                // The toctree directive has options (:maxdepth:, :caption:) followed by indented chapter names
                // We want to remove all of this from the output
                for (line_idx, line) in lines_after_directive.iter().enumerate() {
                    let line_start_pos = content_start
                        + if line_idx > 0 {
                            lines_after_directive[0..line_idx].join("\n").len() + 1
                        } else {
                            0
                        };

                    let trimmed = line.trim();

                    // Skip options (lines starting with :)
                    if trimmed.starts_with(':') {
                        continue;
                    }

                    // Skip empty lines
                    if trimmed.is_empty() {
                        continue;
                    }

                    // Check if line is indented
                    let is_indented = line.starts_with(' ') || line.starts_with('\t');

                    if is_indented {
                        continue;
                    }

                    // If we hit a non-indented, non-option line, stop here
                    if !is_indented && !trimmed.starts_with(':') {
                        content_end = line_start_pos;
                        break;
                    }
                }

                // If we reached the end without finding a stopping point, use the end of content
                if content_end == content.len() {
                    // Keep content_end as is
                }

                // If we have more directives, don't go past them
                if i + 1 < directive_starts.len() {
                    let next_directive_start = directive_starts[i + 1].0;
                    if content_end > next_directive_start {
                        content_end = next_directive_start;
                    }
                }
            } else {
                for (line_idx, line) in lines_after_directive.iter().enumerate() {
                    let line_start_pos = content_start
                        + if line_idx > 0 {
                            lines_after_directive[0..line_idx].join("\n").len() + 1
                        } else {
                            0
                        };

                    // Check if line is indented (has leading whitespace)
                    let is_indented = line.starts_with(' ') || line.starts_with('\t');

                    if !found_indented_content && line.trim().is_empty() {
                        continue;
                    }

                    if is_indented {
                        found_indented_content = true;
                        continue;
                    }

                    // If we found indented content and now hit a non-indented line, stop
                    if found_indented_content && !is_indented {
                        content_end = line_start_pos;
                        break;
                    }
                }

                // If we found indented content but never hit a non-indented line,
                // set content_end to the end of the content
                if found_indented_content && content_end == content.len() {
                    // Keep content_end as is (end of file)
                }
                
                // If we have more directives, make sure we don't go past them
                if i + 1 < directive_starts.len() {
                    let next_directive_start = directive_starts[i + 1].0;
                    if content_end > next_directive_start {
                        content_end = next_directive_start;
                    }
                }
            }

            let directive_content = &content[content_start..content_end];

            // Extract field list options (lines starting with ":")
            let mut options = std::collections::HashMap::new();
            let mut content_lines: Vec<&str> = Vec::new();
            
            for line in directive_content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with(':') && trimmed.len() > 1 {
                    // This is a field list option
                    if let Some(colon_pos) = trimmed[1..].find(':') {
                        let actual_colon_pos = colon_pos + 1;
                        let key = trimmed[1..actual_colon_pos].trim().to_string();
                        let value = trimmed[actual_colon_pos + 1..].trim().to_string();
                        if !key.is_empty() {
                            options.insert(key, value);
                        }
                    }
                } else {
                    // This is content
                    content_lines.push(line);
                }
            }
            
            let actual_content = content_lines.join("\n");

            if let Some(handler) = self.directive_handlers.get_mut(directive_name) {
                let processed = handler.handle(language, &actual_content, &options)?;
                result.push_str(&processed);
            }

            last_pos = content_end;
        }

        result.push_str(&content[last_pos..]);

        Ok(result)
    }

    /// Process RST inline roles (like :math:`...`)
    fn process_roles(&self, content: &str) -> Result<String> {
        let role_regex = Regex::new(r":([a-zA-Z0-9_-]+):`([^`]*)`").map_err(|e| {
            crate::core::Error::rst_parse(format!("Failed to compile role regex: {}", e))
        })?;
        
        let result = role_regex.replace_all(content, |caps: &regex::Captures| {
            let role_name = caps.get(1).unwrap().as_str();
            let role_content = caps.get(2).unwrap().as_str();
            
            match role_name {
                "math" => {
                    // Render as inline math
                    format!(
                        r#"<span class="math-inline" data-latex="{}"></span>"#,
                        role_content
                    )
                }
                _ => caps.get(0).unwrap().as_str().to_string(), // Preserve other roles
            }
        }).to_string();
        
        Ok(result)
    }

    /// Convert RST markup to HTML
    fn convert_rst_to_html(&self, content: &str) -> Result<String> {
        let mut html = content.to_string();

        html = self.convert_tables(&html)?;
        html = self.convert_headers(&html)?;
        html = self.convert_emphasis(&html)?;
        html = self.convert_links(&html)?;
        html = self.convert_lists(&html)?;
        html = self.convert_paragraphs(&html)?;

        Ok(html)
    }

    /// Convert RST headers to HTML
    fn convert_headers(&self, content: &str) -> Result<String> {
        let mut result = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        let mut in_html_block = false;
        let mut html_block_depth = 0;

        while i < lines.len() {
            let line = lines[i];
            let trimmed = line.trim();

            // Track HTML block depth
            if trimmed.starts_with("<div") {
                html_block_depth += 1;
                in_html_block = true;
            } else if trimmed.starts_with("</div>") && in_html_block {
                html_block_depth -= 1;
                if html_block_depth == 0 {
                    in_html_block = false;
                }
            }

            // Skip processing if inside HTML block
            if in_html_block {
                result.push(line.to_string());
                i += 1;
                continue;
            }

            // Skip RST underline characters at the start of processing
            if self.is_rst_underline(trimmed) {
                i += 1;
                continue;
            }

            if i + 1 < lines.len() {
                let next_line = lines[i + 1];
                let current_line = line.trim();
                // Support all RST underline characters: = - ~ ^ " ' ` : # * + _ < > |
                let is_underline = next_line.chars().all(|c| matches!(c, '=' | '-' | '~' | '^' | '"' | '\'' | '`' | ':' | '#' | '*' | '+' | '_' | '<' | '>' | '|'));
                if is_underline
                    && next_line.len() >= current_line.len()
                    && !current_line.is_empty()
                {
                    let level = match next_line.chars().next() {
                        Some('=') => "2",
                        Some('-') => "3",
                        Some('~') => "4",
                        Some('^') => "5",
                        Some('"') => "6",
                        _ => "3", // Default to level 3 for other underline characters
                    };
                    let anchor = self.slugify(current_line);
                    result.push(format!(
                        "<h{} id=\"{}\">{}</h{}>",
                        level, anchor, current_line, level
                    ));
                    i += 2;
                    continue;
                }
            }

            let header_regex = Regex::new(r"^(#{1,6})\s+(.+)$").map_err(|e| {
                crate::core::Error::rst_parse(format!("Failed to compile header regex: {}", e))
            })?;
            if let Some(captures) = header_regex.captures(line) {
                let level = captures.get(1).unwrap().as_str().len();
                let title = captures.get(2).unwrap().as_str();
                let anchor = self.slugify(title);
                result.push(format!(
                    "<h{} id=\"{}\">{}</h{}>",
                    level, anchor, title, level
                ));
                i += 1;
            } else if line.trim().len() > 0
                && !line.starts_with(' ')
                && !line.contains('[')
                && !line.contains('`')
                && !line.contains('<')
            {
                let trimmed = line.trim();
                // Skip lines that look like list items
                if trimmed.starts_with("- ") || trimmed.starts_with("* ") ||
                   (trimmed.chars().next().map(|c| c.is_numeric()).unwrap_or(false) && trimmed.contains('.')) {
                    result.push(line.to_string());
                    i += 1;
                } else if trimmed.len() > 0 &&
                   !self.is_rst_underline(trimmed) &&
                   trimmed.len() <= 50 &&
                   !trimmed.contains('.') &&
                   !trimmed.contains(',') &&
                   trimmed.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_')
                {
                    let anchor = self.slugify(trimmed);
                    result.push(format!("<h3 id=\"{}\">{}</h3>", anchor, trimmed));
                    i += 1;
                } else {
                    result.push(line.to_string());
                    i += 1;
                }
            } else {
                result.push(line.to_string());
                i += 1;
            }
        }

        Ok(result.join("\n"))
    }

    /// Convert RST emphasis to HTML
    fn convert_emphasis(&self, content: &str) -> Result<String> {
        let mut result = String::new();
        let mut pos = 0;

        let html_tag_regex = Regex::new(r"<[^>]*>").map_err(|e| {
            crate::core::Error::rst_parse(format!("Failed to compile HTML tag regex: {}", e))
        })?;

        let mut html_tags = Vec::new();
        for mat in html_tag_regex.find_iter(content) {
            html_tags.push((mat.start(), mat.end()));
        }

        for &(start, end) in &html_tags {
            let before_tag = &content[pos..start];
            let processed_before = self.process_emphasis_text(before_tag)?;
            result.push_str(&processed_before);
            result.push_str(&content[start..end]);
            pos = end;
        }

        if pos < content.len() {
            let remaining = &content[pos..];
            let processed_remaining = self.process_emphasis_text(remaining)?;
            result.push_str(&processed_remaining);
        }

        Ok(result)
    }

    /// Process emphasis conversion for regular text
    fn process_emphasis_text(&self, content: &str) -> Result<String> {
        let mut html = content.to_string();

        html = regex::Regex::new(r"\*\*([^*]+)\*\*")
            .map_err(|e| {
                crate::core::Error::rst_parse(format!("Failed to compile bold regex: {}", e))
            })?
            .replace_all(&html, "<strong>$1</strong>")
            .to_string();

        html = regex::Regex::new(r"\*([^*]+)\*")
            .map_err(|e| {
                crate::core::Error::rst_parse(format!("Failed to compile italic regex: {}", e))
            })?
            .replace_all(&html, "<em>$1</em>")
            .to_string();

        html = regex::Regex::new(r"``([^`]+)``")
            .map_err(|e| {
                crate::core::Error::rst_parse(format!("Failed to compile code regex: {}", e))
            })?
            .replace_all(&html, "<code>$1</code>")
            .to_string();

        Ok(html)
    }

    /// Convert RST links to HTML
    fn convert_links(&self, content: &str) -> Result<String> {
        let link_regex = Regex::new(r"`([^`]+)<([^>]+)>`_").map_err(|e| {
            crate::core::Error::rst_parse(format!("Failed to compile link regex: {}", e))
        })?;

        let html = link_regex
            .replace_all(content, r#"<a href="$2">$1</a>"#)
            .to_string();

        Ok(html)
    }

    /// Convert RST lists to HTML
    fn convert_lists(&self, content: &str) -> Result<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i];
            
            // Skip non-list lines
            if !self.is_list_item(line) {
                result.push(line.to_string());
                i += 1;
                continue;
            }
            
            // Found a list - collect all list items
            let mut list_items = Vec::new();
            let list_start_indent = self.calculate_indent(line);
            
            while i < lines.len() && self.is_list_item(lines[i]) {
                let item_indent = self.calculate_indent(lines[i]);
                
                // Include this item
                list_items.push((item_indent, lines[i].to_string()));
                i += 1;
                
                // Look ahead for nested items
                while i < lines.len() {
                    let next_line = lines[i];
                    if next_line.trim().is_empty() {
                        i += 1;
                        continue;
                    }
                    
                    let next_indent = self.calculate_indent(next_line);
                    if next_indent > item_indent && self.is_list_item(next_line) {
                        // This is a nested item - include it
                        list_items.push((next_indent, next_line.to_string()));
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
            
            // Convert the collected list items to HTML
            if !list_items.is_empty() {
                let list_html = self.convert_list_group(&list_items, list_start_indent)?;
                result.push(list_html);
            }
        }
        
        Ok(result.join("\n"))
    }
    
    /// Convert a group of list items to HTML
    fn convert_list_group(&self, items: &[(usize, String)], _base_indent: usize) -> Result<String> {
        let mut result = Vec::new();
        let mut i = 0;
        let mut list_type = "ul".to_string();
        
        while i < items.len() {
            let (item_indent, item_line) = &items[i];
            let (_, detected_type, item_content) = self.parse_list_item_raw(item_line)?;
            
            // Track the list type from the first item
            if result.is_empty() {
                list_type = detected_type.clone();
            }
            
            // Open list tag if needed
            if result.is_empty() || !self.contains_open_list(&result) {
                result.push(format!("<{}>", list_type));
            }
            
            result.push(format!("<li>{}</li>", item_content));
            i += 1;
            
            // Check for nested items
            let mut nested_items = Vec::new();
            let mut nested_base_indent = None;
            
            while i < items.len() {
                let (next_indent, _) = &items[i];
                if *next_indent > *item_indent {
                    if nested_base_indent.is_none() {
                        nested_base_indent = Some(*next_indent);
                    }
                    nested_items.push((*next_indent, items[i].1.clone()));
                    i += 1;
                } else {
                    break;
                }
            }
            
            // Process nested items
            if !nested_items.is_empty() {
                let nested_html = self.convert_list_group(&nested_items, *item_indent)?;
                result.push(nested_html);
            }
        }
        
        // Close list tag
        result.push(format!("</{}>", list_type));
        
        Ok(result.join("\n"))
    }
    
    /// Check if there's an open list tag in the result
    fn contains_open_list(&self, result: &[String]) -> bool {
        let html = result.join("\n");
        let mut open_count = 0;
        
        for tag in ["<ul>", "<ol>"] {
            let start_tag: &str = tag;
            let end_tag = tag.replace("<", "</");
            
            let mut pos = 0;
            while let Some(found) = html[pos..].find(start_tag) {
                open_count += 1;
                pos += found + start_tag.len();
            }
            
            pos = 0;
            while let Some(found) = html[pos..].find(&end_tag) {
                open_count -= 1;
                pos += found + end_tag.len();
            }
        }
        
        open_count > 0
    }
    
    /// Parse a list item without modifying the line
    fn parse_list_item_raw(&self, line: &str) -> Result<(usize, String, String)> {
        let trimmed = line.trim();

        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            let content = trimmed.trim_start_matches("- ").trim_start_matches("* ");
            return Ok((self.calculate_indent(line), "ul".to_string(), content.to_string()));
        }

        if let Some(first_char) = trimmed.chars().next() {
            if first_char.is_numeric() {
                if let Some(dot_pos) = trimmed.find('.') {
                    let before_dot = &trimmed[..dot_pos];
                    if before_dot.chars().all(|c| c.is_numeric()) {
                        let content = trimmed[dot_pos + 1..].trim();
                        return Ok((self.calculate_indent(line), "ol".to_string(), content.to_string()));
                    }
                }
            }
        }

        Err(crate::core::Error::rst_parse(
            "Invalid list item format".to_string(),
        ))
    }

    /// Check if a line is a list item
    fn is_list_item(&self, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            return true;
        }

        if let Some(first_char) = trimmed.chars().next() {
            if first_char.is_numeric() {
                if let Some(dot_pos) = trimmed.find('.') {
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

    /// Parse a list item
    #[allow(dead_code)]
    fn parse_list_item(&self, line: &str) -> Result<(String, String)> {
        let trimmed = line.trim();

        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            let content = trimmed.trim_start_matches("- ").trim_start_matches("* ");
            return Ok(("ul".to_string(), content.to_string()));
        }

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

        Err(crate::core::Error::rst_parse(
            "Invalid list item format".to_string(),
        ))
    }

    /// Convert RST paragraphs to HTML
    fn convert_paragraphs(&self, content: &str) -> Result<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();
        let mut paragraph = Vec::new();
        let mut in_html_block = false;
        let mut html_block_depth = 0;

        for line in lines {
            let trimmed = line.trim();

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
                result.push(line.to_string());
            } else if self.is_html_tag(trimmed) {
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
            } else if self.is_rst_underline(trimmed) {
                // Skip RST underline characters (already processed as headers)
                if !paragraph.is_empty() {
                    result.push(format!("<p>{}</p>", paragraph.join(" ")));
                    paragraph.clear();
                }
            } else if !trimmed.starts_with("    ") && !trimmed.starts_with("\t") {
                paragraph.push(trimmed);
            } else {
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
        line.starts_with("<") && line.ends_with(">")
            || line.starts_with("<div")
            || line.starts_with("</div")
            || line.starts_with("<pre")
            || line.starts_with("</pre")
            || line.starts_with("<code")
            || line.starts_with("</code")
            || line.starts_with("<span")
            || line.starts_with("</span")
            || line.starts_with("<button")
            || line.starts_with("</button")
    }

    /// Check if a line consists only of RST underline characters
    fn is_rst_underline(&self, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return false;
        }
        trimmed.chars().all(|c| matches!(c, '=' | '-' | '~' | '^' | '"' | '\'' | '`' | ':' | '#' | '*' | '+' | '_' | '<' | '>' | '|'))
    }

    /// Convert RST simple tables to HTML
    fn convert_tables(&self, content: &str) -> Result<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            
            // Skip HTML tags (already processed content)
            if line.trim().starts_with('<') {
                result.push(line.to_string());
                i += 1;
                continue;
            }
            
            // Check if this line looks like a table separator (contains at least 3 dashes and plus signs)
            if self.is_table_separator(line) {
                // Look ahead to see if we have a proper table structure
                let table_start = self.find_table_start(&lines, i);
                if table_start.is_some() {
                    let (start_idx, table, line_count) = table_start.unwrap();
                    if !table.is_empty() && line_count >= 2 {
                        // Remove lines that were already added before we found the table
                        while result.len() > start_idx {
                            result.pop();
                        }
                        result.push(table);
                        i += line_count; // Skip all table lines
                        continue;
                    }
                }
            }
            
            result.push(line.to_string());
            i += 1;
        }

        Ok(result.join("\n"))
    }

    /// Check if a line is a table separator
    fn is_table_separator(&self, line: &str) -> bool {
        let trimmed = line.trim();
        let dash_count = trimmed.chars().filter(|&c| c == '-').count();
        let plus_count = trimmed.chars().filter(|&c| c == '+').count();
        let pipe_count = trimmed.chars().filter(|&c| c == '|').count();
        
        // Must have at least 3 dashes and use + or = or | as column separators
        (dash_count >= 3 || trimmed.chars().filter(|&c| c == '=').count() >= 3) 
            && (plus_count > 0 || trimmed.contains('=') || pipe_count > 1)
    }

    /// Check if a line looks like a table row (contains pipe separators)
    fn is_table_row(&self, line: &str) -> bool {
        let trimmed = line.trim();
        let pipe_count = trimmed.chars().filter(|&c| c == '|').count();
        pipe_count >= 2 && trimmed.starts_with('|') && trimmed.ends_with('|')
    }

    /// Find the start of a table and parse it
    fn find_table_start(&self, lines: &[&str], idx: usize) -> Option<(usize, String, usize)> {
        // Look back a few lines to find the start of the table
        let look_back = std::cmp::min(5, idx + 1);
        let mut table_start_idx = idx;
        
        for i in (idx.saturating_sub(look_back))..=idx {
            if i < lines.len() && self.is_table_row(lines[i]) {
                table_start_idx = i;
                break;
            }
        }
        
        let (table, line_count) = self.parse_table(lines, table_start_idx).ok()?;
        if !table.is_empty() {
            Some((table_start_idx, table, line_count))
        } else {
            None
        }
    }

    /// Parse a table starting from the current line
    fn parse_table(&self, lines: &[&str], start_idx: usize) -> Result<(String, usize)> {
        let mut table_lines = Vec::new();
        let mut i = start_idx;
        
        // Collect table lines
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Stop at empty line or when line doesn't look like table content
            if line.is_empty() || (!self.is_table_separator(line) && !line.contains('|') && !line.contains('+')) {
                break;
            }
            
            table_lines.push(line.to_string());
            i += 1;
        }
        
        if table_lines.is_empty() {
            return Ok((String::new(), 0));
        }
        
        let line_count = table_lines.len();
        
        // Parse table structure
        let html = self.render_table(&table_lines)?;
        
        Ok((html, line_count))
    }

    /// Render table lines to HTML
    fn render_table(&self, table_lines: &[String]) -> Result<String> {
        let mut html = String::new();
        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut header_sep_idx: Option<usize> = None;
        
        for (idx, line) in table_lines.iter().enumerate() {
            // Split by pipe and trim cells
            let cells: Vec<String> = line
                .split('|')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            if cells.is_empty() {
                continue;
            }
            
            // Check if this is a separator line (all cells are just dashes or equals)
            let is_simple_sep = cells.iter().all(|cell| {
                cell.chars().all(|c| c == '-' || c == '=')
            });
            
            // Track separator lines (first separator after first row)
            if is_simple_sep && idx > 0 && header_sep_idx.is_none() {
                header_sep_idx = Some(idx);
                continue; // Skip the separator line itself
            }
            
            // Skip other separator lines
            if is_simple_sep {
                continue;
            }
            
            rows.push(cells);
        }
        
        if rows.is_empty() {
            return Ok(String::new());
        }
        
        // Determine if we have a header (first row before separator)
        let has_header = header_sep_idx.is_some() && rows.len() > 1;
        
        html.push_str("<table>\n");
        
        // Render header and body
        if has_header {
            // First row is header, rest are body
            html.push_str("  <thead>\n    <tr>\n");
            for cell in &rows[0] {
                html.push_str(&format!("      <th>{}</th>\n", cell));
            }
            html.push_str("    </tr>\n  </thead>\n");
            
            // Render body (skip first row which is header)
            html.push_str("  <tbody>\n");
            for row in &rows[1..] {
                html.push_str("    <tr>\n");
                for cell in row {
                    html.push_str(&format!("      <td>{}</td>\n", cell));
                }
                html.push_str("    </tr>\n");
            }
            html.push_str("  </tbody>\n");
        } else {
            // No header, render all rows as body
            html.push_str("  <tbody>\n");
            for row in &rows {
                html.push_str("    <tr>\n");
                for cell in row {
                    html.push_str(&format!("      <td>{}</td>\n", cell));
                }
                html.push_str("    </tr>\n");
            }
            html.push_str("  </tbody>\n");
        }
        
        html.push_str("</table>");
        
        Ok(html)
    }

    /// Extract TOC from toctree directive output in HTML
    fn extract_toc_from_toctree(&self, html: &str) -> Result<(Vec<TocEntry>, String)> {
        let toc_tree_pattern = r#"<div class="toc-tree">(.+?)</div>"#;
        let toc_caption_pattern = r#"<div class="toc-caption">(.+?)</div>"#;
        let toc_tree_regex = Regex::new(toc_tree_pattern)?;
        let toc_caption_regex = Regex::new(toc_caption_pattern)?;

        let mut toc_html = String::new();
        let mut entries = Vec::new();

        if let Some(caps) = toc_caption_regex.captures(html) {
            if let Some(caption) = caps.get(1) {
                toc_html.push_str(&format!(
                    "<div class=\"toc-caption\">{}</div>",
                    caption.as_str()
                ));
            }
        }

        if let Some(caps) = toc_tree_regex.captures(html) {
            let toc_content = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let item_pattern = r#"<div class="toc-item"><a href="([^"]+)">([^<]+)</a></div>"#;
            let item_regex = Regex::new(item_pattern)?;

            for item_caps in item_regex.captures_iter(toc_content) {
                if let (Some(_url), Some(title)) = (item_caps.get(1), item_caps.get(2)) {
                    let title_text = title.as_str().trim().to_string();
                    let anchor = title_text.to_lowercase().replace(' ', "-");

                    entries.push(TocEntry {
                        level: 1,
                        title: title_text,
                        anchor,
                        children: Vec::new(),
                    });
                }
            }

            toc_html.push_str("<div class=\"toc-tree\">");
            toc_html.push_str(toc_content);
            toc_html.push_str("</div>");
        }

        Ok((entries, toc_html))
    }

    /// Convert title to URL-friendly slug
    fn slugify(&self, title: &str) -> String {
        let mut result = title.to_lowercase();

        result = result.replace("c++", "cpp");
        result = result.replace("c#", "csharp");
        result = result.replace("f#", "fsharp");
        result = result.replace("c++/cli", "cpp-cli");
        result = result.replace(".net", "dotnet");
        result = result.replace("node.js", "nodejs");
        result = result.replace("react.js", "reactjs");
        result = result.replace("vue.js", "vuejs");
        result = result.replace("angular.js", "angularjs");

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

        result = result.replace(&[' ', '-', '_', '.', ',', ';', ':', '!', '?', '@', '#', '$', '%', '^', '&', '*', '(', ')', '=', '[', ']', '{', '}', '\\', '|', '<', '>', '/', '"', '\''][..], "-");

        result = result.replace(['"', '\''], "");

        result = result.chars().filter(|c| c.is_alphanumeric() || *c == '-').collect::<String>();

        while result.contains("--") {
            result = result.replace("--", "-");
        }

        result.trim_matches('-').to_string()
    }
}

impl Default for RstParser {
    fn default() -> Self {
        Self::new().expect("Failed to create RstParser")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    mod fixtures {
        pub fn frontmatter_valid() -> &'static str {
            r#"---
title: "Test Title"
date: 2023-01-01
tags: ["test", "example"]
author: "Test Author"
---

"#
        }

        pub fn frontmatter_all_fields() -> &'static str {
            r#"---
title: "Complete Test"
date: 2023-01-01
tags: ["tag1", "tag2"]
author: "Author Name"
type: "article"
---

"#
        }

        pub fn simple_header() -> &'static str {
            "Test Header\n===========\n"
        }

        pub fn markdown_header() -> &'static str {
            "## Test Header\n"
        }

        pub fn unordered_list() -> &'static str {
            "- Item 1\n- Item 2\n- Item 3\n"
        }

        pub fn ordered_list() -> &'static str {
            "1. First item\n2. Second item\n3. Third item\n"
        }

        pub fn code_block_python() -> &'static str {
            r#".. code-block:: python

    def hello():
        print("Hello, World!")
"#
        }

        pub fn snippet_card_directive() -> &'static str {
            r#".. snippet-card:: test_snippet
"#
        }

        pub fn toctree_directive() -> &'static str {
            r#".. toctree::
   :maxdepth: 2
   :caption: Contents:

   chapter1
   chapter2
   chapter3
"#
        }

        pub fn inline_math() -> &'static str {
            "The formula is $E = mc^2$.\n"
        }

        pub fn display_math() -> &'static str {
            "Here is the formula:\n$$\\int_0^1 f(x) dx$$\n"
        }

        pub fn complete_document() -> &'static str {
            r#"---
title: "Complete Test Document"
date: 2023-01-01
tags: ["test"]
author: "Test Author"
---

Introduction
============

This is a test document.

First Section
-------------

Here is some **bold** text and *italic* text.

List Example
------------

- First item
- Second item
- Third item

Code Example
------------

.. code-block:: python

    def example():
        return 42

Math Example
------------

Inline math: $x^2 + y^2 = z^2$

Display math:
$$\\int_0\\infty e^{-x} dx = 1$$

Conclusion
----------

This concludes the test.
"#
        }
    }

    mod assertions {
        pub fn assert_html_contains(html: &str, expected: &str) {
            assert!(
                html.contains(expected),
                "Expected HTML to contain: {}\nActual: {}",
                expected,
                html
            );
        }
    }

    // Frontmatter Tests
    #[test]
    fn test_extract_frontmatter_valid() {
        let content = fixtures::frontmatter_valid();
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(content).unwrap();

        assert_eq!(result.metadata.title, "Test Title");
        assert_eq!(result.metadata.author.unwrap(), "Test Author");
        assert_eq!(result.metadata.tags, vec!["test", "example"]);
    }

    #[test]
    fn test_extract_frontmatter_with_all_fields() {
        let content = fixtures::frontmatter_all_fields();
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(content).unwrap();

        assert_eq!(result.metadata.title, "Complete Test");
        assert_eq!(result.metadata.tags, vec!["tag1", "tag2"]);
    }

    // Header Tests
    #[test]
    fn test_convert_underlined_headers() {
        let content = format!("{}{}", fixtures::frontmatter_valid(), fixtures::simple_header());
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(&content).unwrap();

        assertions::assert_html_contains(&result.html, "<h2");
        assertions::assert_html_contains(&result.html, "Test Header");
    }

    #[test]
    fn test_convert_markdown_headers() {
        let content = format!("{}{}", fixtures::frontmatter_valid(), fixtures::markdown_header());
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(&content).unwrap();

        assertions::assert_html_contains(&result.html, "<h2");
        assertions::assert_html_contains(&result.html, "Test Header");
    }

    // List Tests
    #[test]
    fn test_convert_unordered_lists() {
        let content = format!("{}{}", fixtures::frontmatter_valid(), fixtures::unordered_list());
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(&content).unwrap();

        assertions::assert_html_contains(&result.html, "<ul>");
        assertions::assert_html_contains(&result.html, "<li>Item 1</li>");
    }

    #[test]
    fn test_convert_ordered_lists() {
        let content = format!("{}{}", fixtures::frontmatter_valid(), fixtures::ordered_list());
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(&content).unwrap();

        assertions::assert_html_contains(&result.html, "<ol>");
        assertions::assert_html_contains(&result.html, "<li>First item</li>");
    }

    // Directive Tests
    #[test]
    fn test_process_code_block_directive() {
        let content = format!("{}{}", fixtures::frontmatter_valid(), fixtures::code_block_python());
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(&content).unwrap();

        assertions::assert_html_contains(&result.html, "code-block");
        assertions::assert_html_contains(&result.html, "python");
    }

    #[test]
    fn test_process_snippet_card_directive() {
        let content = format!("{}{}", fixtures::frontmatter_valid(), fixtures::snippet_card_directive());
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(&content).unwrap();

        assertions::assert_html_contains(&result.html, "snippet-card");
        assertions::assert_html_contains(&result.html, "test_snippet");
    }

    #[test]
    fn test_process_toctree_directive() {
        let content = format!("{}{}", fixtures::frontmatter_valid(), fixtures::toctree_directive());
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(&content).unwrap();

        assertions::assert_html_contains(&result.html, "toc-tree");
        assertions::assert_html_contains(&result.html, "toc-caption");
    }

    // Math Tests
    #[test]
    fn test_render_inline_math() {
        let content = format!("{}{}", fixtures::frontmatter_valid(), fixtures::inline_math());
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(&content).unwrap();

        assertions::assert_html_contains(&result.html, "math-inline");
        assert!(result.has_math_formulas);
    }

    #[test]
    fn test_render_display_math() {
        let content = format!("{}{}", fixtures::frontmatter_valid(), fixtures::display_math());
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(&content).unwrap();

        assertions::assert_html_contains(&result.html, "math-display");
        assert!(result.has_math_formulas);
    }

    // Integration Tests
    #[test]
    fn test_parse_complete_document() {
        let content = fixtures::complete_document();
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(content).unwrap();

        assert_eq!(result.metadata.title, "Complete Test Document");
        assertions::assert_html_contains(&result.html, "<h2");
        assertions::assert_html_contains(&result.html, "<ul>");
        assertions::assert_html_contains(&result.html, "code-block");
        assertions::assert_html_contains(&result.html, "math-display");
        assert!(result.has_math_formulas);
    }

    #[test]
    fn test_parser_creates_valid_html() {
        let content = fixtures::complete_document();
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(content).unwrap();

        assert!(result.html.contains("<h2") || result.html.contains("<h3"));
        assert!(result.html.contains("<p>") || result.html.contains("<ul"));
        assert!(!result.html.is_empty());
    }

    #[test]
    fn test_metadata_extraction() {
        let content = fixtures::complete_document();
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(content).unwrap();

        assert_eq!(result.metadata.title, "Complete Test Document");
        assert_eq!(result.metadata.author.unwrap(), "Test Author");
        assert_eq!(result.metadata.tags, vec!["test"]);
    }

    #[test]
    fn test_toc_generation() {
        let content = fixtures::complete_document();
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(content).unwrap();

        assert!(!result.toc.is_empty());
    }

    // Edge Case Tests
    #[test]
    fn test_parse_empty_document() {
        let content = "---\ntitle: \"Empty\"\n---\n";
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(content).unwrap();

        assert_eq!(result.metadata.title, "Empty");
        assert!(!result.has_math_formulas);
    }

    #[test]
    fn test_parse_document_with_only_frontmatter() {
        let content = "---\ntitle: \"Frontmatter Only\"\n---\n";
        let mut parser = RstParser::new().unwrap();
        let result = parser.parse(content).unwrap();

        assert_eq!(result.metadata.title, "Frontmatter Only");
    }
}
