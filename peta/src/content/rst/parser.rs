//! RST parser implementation with improved architecture

use crate::content::rst::{
    toc_generator::TocGenerator, CodeHighlighter, DirectiveHandler, MathProcessor, MathRenderer,
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
            Box::new(crate::content::rst::directives::SnippetCardHandler),
        );
        directive_handlers.insert(
            "toctree".to_string(),
            Box::new(crate::content::rst::directives::TocTreeHandler),
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
        } else {
            let toc_entries = self.toc_generator.generate(&processed_html)?;
            let toc_html = self.toc_generator.render_html(&toc_entries);
            (toc_entries, toc_html)
        };

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
            toc_html,
            frontmatter,
            has_math_formulas: math_detection.has_formulas,
            math_formula_count: math_detection.formula_count,
            math_render_script: math_script,
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

        // Process directives first
        processed = self.process_directives(&processed)?;

        // Process math equations
        processed = self.math_renderer.render(&processed)?;

        // Convert RST markup to HTML
        processed = self.convert_rst_to_html(&processed)?;

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
                for line in lines_after_directive.iter() {
                    if line.trim().is_empty() {
                        continue;
                    }
                    let line_end = content_start + line.len();
                    content_end = line_end;
                    break;
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

            if let Some(handler) = self.directive_handlers.get_mut(directive_name) {
                let processed = handler.handle(language, directive_content)?;
                result.push_str(&processed);
            }

            last_pos = content_end;
        }

        result.push_str(&content[last_pos..]);

        Ok(result)
    }

    /// Convert RST markup to HTML
    fn convert_rst_to_html(&self, content: &str) -> Result<String> {
        let mut html = content.to_string();

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

        while i < lines.len() {
            let line = lines[i];
            let trimmed = line.trim();

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
        let mut list_stack = Vec::new();
        let mut last_indent = 0;

        for line in lines {
            let trimmed = line.trim();

            if trimmed.is_empty() || !self.is_list_item(line) {
                while !list_stack.is_empty() {
                    let (list_type, _) = list_stack.pop().unwrap();
                    result.push(format!("</{}>", list_type));
                }
                last_indent = 0;
                result.push(line.to_string());
                continue;
            }

            let current_indent = self.calculate_indent(line);
            let (list_type, item_content) = self.parse_list_item(line)?;

            if current_indent > last_indent {
                result.push(format!("<{}>", list_type));
                list_stack.push((list_type, current_indent));
            } else if current_indent < last_indent {
                while let Some((stack_type, stack_indent)) = list_stack.last() {
                    if *stack_indent > current_indent {
                        result.push(format!("</{}>", stack_type));
                        list_stack.pop();
                    } else {
                        break;
                    }
                }

                if let Some((stack_type, _)) = list_stack.last() {
                    if *stack_type != list_type {
                        result.push(format!("</{}>", stack_type));
                        list_stack.pop();
                        result.push(format!("<{}>", list_type));
                        list_stack.push((list_type, current_indent));
                    }
                }
            } else {
                if let Some((stack_type, _)) = list_stack.last() {
                    if *stack_type != list_type {
                        result.push(format!("</{}>", stack_type));
                        list_stack.pop();
                        result.push(format!("<{}>", list_type));
                        list_stack.push((list_type, current_indent));
                    }
                } else {
                    result.push(format!("<{}>", list_type));
                    list_stack.push((list_type, current_indent));
                }
            }

            result.push(format!("<li>{}</li>", item_content));
            last_indent = current_indent;
        }

        while !list_stack.is_empty() {
            let (list_type, _) = list_stack.pop().unwrap();
            result.push(format!("</{}>", list_type));
        }

        Ok(result.join("\n"))
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
        // Check if line consists only of RST underline characters
        trimmed.chars().all(|c| matches!(c, '=' | '-' | '~' | '^' | '"' | '\'' | '`' | ':' | '#' | '*' | '+' | '_' | '<' | '>' | '|'))
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
