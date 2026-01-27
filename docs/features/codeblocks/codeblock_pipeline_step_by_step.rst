Step-by-Step Code Block Pipeline Implementation
================================================

This guide documents the complete step-by-step process of unifying the code block rendering pipeline in the Peta Rust static site generator, moving from a split Rust/Theme architecture to a unified Rust-based system.

Prerequisites
------------

- Rust 1.70+ with cargo
- Understanding of RST (reStructuredText) parsing
- Knowledge of static site generation concepts
- Familiarity with syntect syntax highlighting library

Problem Statement
-----------------

**Before:** Code block rendering was split between:
- Rust: Basic HTML structure generation
- Theme CSS: Token colors, container styling, line numbers
- Theme JS: Copy functionality, hover effects, keyboard shortcuts

**Issues:**
- Multiple points of failure
- Inconsistent rendering across pages
- Difficult to maintain and test
- No single source of truth
- Performance overhead from runtime rendering

**Goal:** Unify all rendering in Rust with pre-rendered HTML, CSS, and JS generated programmatically.

Implementation Steps
--------------------

Step 1: Create Syntax Highlighter
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/content/rst/syntax_highlighter.rs``

Create the core syntax highlighting module using syntect:

.. code-block:: rust

    use syntect::html::{ClassedHTMLGenerator, ClassStyle};
    use syntect::parsing::SyntaxSet;
    use syntect::util::LinesWithEndings;
    use crate::core::Result;

    pub struct SyntaxHighlighter {
        syntax_set: SyntaxSet,
        config: HighlighterConfig,
    }

    #[derive(Debug, Clone)]
    pub struct HighlighterConfig {
        pub theme: String,
        pub line_numbers: bool,
        pub highlight_lines: Vec<usize>,
    }

    impl SyntaxHighlighter {
        pub fn new() -> Result<Self> {
            let syntax_set = SyntaxSet::load_defaults_newlines();
            Ok(Self {
                syntax_set,
                config: HighlighterConfig::default(),
            })
        }

        pub fn highlight(&self, code: &str, language: &str) -> Result<String> {
            let resolved_language = self.resolve_language(language);
            
            let syntax = self.syntax_set
                .find_syntax_by_token(&resolved_language)
                .or_else(|| self.syntax_set.find_syntax_by_extension(&resolved_language))
                .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

            let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                syntax,
                &self.syntax_set,
                ClassStyle::Spaced,
            );

            for line in LinesWithEndings::from(code) {
                html_generator.parse_html_for_line_which_includes_newline(line)?;
            }

            let html = html_generator.finalize();
            Ok(self.convert_classes(&html))
        }

        fn resolve_language(&self, language: &str) -> String {
            match language.to_lowercase().as_str() {
                "ts" | "typescript" => "javascript".to_string(),
                "js" => "javascript".to_string(),
                "py" | "python3" => "python".to_string(),
                _ => language.to_string(),
            }
        }

        fn convert_classes(&self, html: &str) -> String {
            use regex::Regex;
            
            let mut result = html.to_string();
            
            // Convert syntect classes to our token classes
            result = Regex::new(r#"class="keyword(\s+[^"]*)?""#)
                .unwrap()
                .replace_all(&result, r#"class="token-keyword""#)
                .to_string();
            
            result = Regex::new(r#"class="string(\s+[^"]*)?""#)
                .unwrap()
                .replace_all(&result, r#"class="token-string""#)
                .to_string();
            
            // ... more conversions for all token types
            
            result
        }
    }

**Key Points:**
- Uses syntect for actual syntax highlighting
- Supports 100+ programming languages
- Converts syntect classes to custom token classes
- Handles language aliases (typescript → javascript)

Step 2: Create Code Block Renderer
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/content/rst/code_block_renderer.rs``

Create the unified renderer that generates complete HTML:

.. code-block:: rust

    use crate::content::rst::SyntaxHighlighter;
    use crate::core::Result;

    pub struct CodeBlockRenderer {
        highlighter: SyntaxHighlighter,
        config: CodeBlockConfig,
    }

    #[derive(Debug, Clone)]
    pub struct CodeBlockConfig {
        pub line_numbers: bool,
        pub copy_button: bool,
        pub highlight_lines: Vec<usize>,
        pub theme: String,
    }

    impl CodeBlockRenderer {
        pub fn render(&self, code: &str, language: &str, title: Option<&str>) -> Result<String> {
            let (highlighted_code, line_count) = if self.config.line_numbers {
                self.highlighter.highlight_with_line_numbers(
                    code, 
                    language, 
                    &self.config.highlight_lines
                )?
            } else {
                let highlighted = self.highlighter.highlight(code, language)?;
                let line_count = code.lines().count();
                (highlighted, line_count)
            };

            // Generate HTML structure
            let mut html = String::new();
            html.push_str(r#"<div class="code-block""#);
            html.push_str(&format!(r#" data-language="{}""#, language));
            html.push_str(&format!(r#" data-theme="{}""#, self.config.theme));
            html.push_str(&format!(r#" data-line-count="{}""#, line_count));
            
            if self.config.line_numbers {
                html.push_str(r#" data-line-numbers="true""#);
            }
            
            html.push('>');
            html.push('\n');

            // Add header
            html.push_str("  <div class=\"code-header\">\n");
            html.push_str("    <div class=\"code-info\">\n");
            
            if let Some(t) = title {
                html.push_str(&format!(r#"      <span class="code-title">{}</span>"#, t));
                html.push('\n');
            }
            
            html.push_str(&format!(
                r#"      <span class="code-language">{}</span>"#,
                language.to_uppercase()
            ));
            html.push('\n');
            html.push_str("    </div>\n");

            // Add copy button
            if self.config.copy_button {
                html.push_str(r#"    <button class="code-copy-button" onclick="copyCode(this)">"#);
                // ... button HTML
            }

            html.push_str("  </div>\n");

            // Add content
            html.push_str("  <div class=\"code-content\">\n");
            html.push_str("    <pre><code class=\"language-");
            html.push_str(language);
            html.push_str("\">\n");
            html.push_str(&highlighted_code);
            html.push_str("\n    </code></pre>\n");
            html.push_str("  </div>\n");
            html.push_str("</div>");

            Ok(html)
        }
    }

**Key Points:**
- Generates complete, self-contained HTML
- Includes header, content, line numbers
- Adds data attributes for styling
- Pre-renders everything at build time

Step 3: Update Directive Handler
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/content/rst/directives.rs``

Update the directive trait to pass directive type:

.. code-block:: rust

    pub trait DirectiveHandler {
        fn handle(&mut self, directive_type: &str, content: &str) -> Result<String>;
    }

    pub struct CodeBlockHandler {
        renderer: CodeBlockRenderer,
    }

    impl DirectiveHandler for CodeBlockHandler {
        fn handle(&mut self, language: &str, content: &str) -> Result<String> {
            let language = if language.is_empty() { "text" } else { language };
            
            // Clean up content (remove paragraph tags)
            let mut code = content.to_string();
            code = code.replace("<p>", "").replace("</p>", "\n");
            
            self.renderer.render(&code, language, None)
        }
    }

**Key Points:**
- Simplified to delegate rendering to CodeBlockRenderer
- Focuses on validation and configuration
- Passes language and content to unified renderer

Step 4: Update Parser
~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/content/rst/parser.rs``

Update parser to extract language from directive:

.. code-block:: rust

    fn parse_directive(&self, content: &str) -> Result<ParsedDirective> {
        let directive_regex = Regex::new(
            r#"^\.\. code-block::\s*(.*?)\n(.*)$"#s
        )?;

        let captures = directive_regex.captures(content).ok_or_else(|| {
            crate::core::Error::content("Invalid code block directive format".to_string())
        })?;

        let directive_type = captures.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        let directive_content = captures.get(2).map(|m| m.as_str()).unwrap_or("");

        Ok(ParsedDirective {
            directive_type: directive_type.to_string(),
            content: directive_content.to_string(),
        })
    }

**Key Points:**
- Extracts language from directive type
- Captures content after directive
- Returns structured data for handler

Step 5: Create CSS Generator
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/assets/css_generator.rs``

Create programmatic CSS generation:

.. code-block:: rust

    pub struct CssGenerator {
        config: CssConfig,
        token_colors: HashMap<String, String>,
    }

    impl CssGenerator {
        pub fn generate(&self) -> Result<String> {
            let mut css = String::new();
            
            // Container styles
            css.push_str(&self.generate_container_styles());
            
            // Token colors
            css.push_str(&self.generate_token_colors());
            
            // Line numbers
            css.push_str(&self.generate_line_number_styles());
            
            // Copy button
            css.push_str(&self.generate_copy_button_styles());
            
            Ok(css)
        }

        fn generate_token_colors(&self) -> String {
            let mut css = String::new();
            
            css.push_str("/* Token Colors */\n");
            
            for (token, color) in &self.token_colors {
                css.push_str(&format!(
                    ".token-{} {{ color: {}; }}\n",
                    token, color
                ));
            }
            
            css
        }
    }

**Key Points:**
- Generates CSS from Rust configuration
- Theme-based color mappings
- No external CSS files needed
- Supports multiple themes

Step 6: Create JS Generator
~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/assets/js_generator.rs``

Create minimal JavaScript for interactions:

.. code-block:: rust

    pub struct JsGenerator {
        config: JsConfig,
    }

    impl JsGenerator {
        pub fn generate(&self) -> Result<String> {
            let mut js = String::new();
            
            js.push_str("/**\n * Code Block Component JavaScript\n */\n\n");
            
            // Copy function
            js.push_str(&self.generate_copy_function());
            
            // Keyboard shortcuts
            js.push_str(&self.generate_keyboard_shortcuts());
            
            Ok(js)
        }

        fn generate_copy_function(&self) -> String {
            r#"
function copyCode(button) {
    const codeBlock = button.closest('.code-block');
    const codeElement = codeBlock.querySelector('code');
    const text = codeElement.textContent;

    navigator.clipboard.writeText(text).then(() => {
        button.classList.add('copied');
        button.innerHTML = `<span>Copied!</span>`;
        
        setTimeout(() => {
            button.classList.remove('copied');
            button.innerHTML = `<span>Copy</span>`;
        }, 2000);
    }).catch(err => {
        console.error('Failed to copy:', err);
    });
}
"#.to_string()
        }
    }

**Key Points:**
- Minimal JavaScript for interactions only
- Copy functionality using Clipboard API
- Keyboard shortcuts
- No runtime parsing or highlighting

Step 7: Update Asset Pipeline
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/assets/pipeline.rs``

Integrate generators into build process:

.. code-block:: rust

    impl AssetPipeline {
        pub fn build(&mut self) -> Result<()> {
            // Generate code block assets
            self.generate_code_block_assets()?;
            
            // ... other asset generation
            
            Ok(())
        }

        fn generate_code_block_assets(&mut self) -> Result<()> {
            let css_generator = CssGenerator::new()?;
            let js_generator = JsGenerator::new()?;
            
            let css_output = self.output_dir.join("css").join("code-blocks.css");
            let js_output = self.output_dir.join("js").join("code-blocks.js");
            
            fs::write(&css_output, css_generator.generate()?)?;
            fs::write(&js_output, js_generator.generate()?)?;
            
            Ok(())
        }
    }

**Key Points:**
- Generates CSS and JS during build
- Writes to output directory
- Integrated into existing pipeline

Step 8: Update Template Engine
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/templates/engine.rs``

Register code block renderer function:

.. code-block:: rust

    impl TemplateEngine {
        pub fn register_functions(&mut self, tera: &mut Tera) -> Result<()> {
            // Register code block renderer
            tera.register_function("code_block", |args| -> Result<Value> {
                let code = args.get("code")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        Error::msg("code_block: missing 'code' argument")
                    })?;
                
                let language = args.get("language")
                    .and_then(|v| v.as_str())
                    .unwrap_or("text");
                
                let renderer = CodeBlockRenderer::new()?;
                let html = renderer.render(code, language, None)?;
                
                Ok(Value::String(html))
            });
            
            Ok(())
        }
    }

**Key Points:**
- Registers code block function
- Accessible in templates
- Returns rendered HTML

Step 9: Update Base Template
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``themes/default/templates/base.html``

Add references to generated assets:

.. code-block:: html

    <head>
        <!-- Code Block Styles (generated from Rust) -->
        <link rel="stylesheet" href="/assets/css/code-blocks.css">
    </head>
    
    <body>
        <!-- ... content ... -->
        
        <!-- Code Block Scripts (generated from Rust) -->
        <script src="/assets/js/code-blocks.js"></script>
    </body>

**Key Points:**
- References generated assets
- No theme-specific code block files
- Single CSS and JS file

Step 10: Remove Theme Files
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Delete theme code block directory:

.. code-block:: bash

    rm -rf themes/default/components/atomic/code_block/

Remove from component registry:

.. code-block:: rust

    // peta/src/components/manager.rs
    
    impl ComponentManager {
        pub fn get_component_category(&self, name: &str) -> Option<ComponentCategory> {
            match name {
                "navbar" => Some(ComponentCategory::Atomic),
                "contacts" => Some(ComponentCategory::Atomic),
                // ... other components
                // code_block removed
                _ => None,
            }
        }
    }

**Key Points:**
- Complete removal of theme files
- No references in component registry
- Clean separation of concerns

Step 11: Update Configuration
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta.toml``

Add code block configuration:

.. code-block:: toml

    [code_blocks]
    default_theme = "one-dark"
    enable_line_numbers = true
    enable_copy_button = true
    
    [code_blocks.languages]
    python = { aliases = ["py", "python3"] }
    javascript = { aliases = ["js", "node"] }
    typescript = { aliases = ["ts", "tsx"] }
    rust = { aliases = ["rs"] }
    go = { aliases = [] }
    sql = { aliases = [] }
    
    [code_blocks.themes]
    one-dark = { name = "One Dark" }

**Key Points:**
- Centralized configuration
- Language aliases support
- Theme definitions
- Feature flags

Step 12: Test Implementation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Create comprehensive tests:

.. code-block:: rust

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_syntax_highlighter() {
            let highlighter = SyntaxHighlighter::new().unwrap();
            let code = "def hello():\n    print('world')";
            let result = highlighter.highlight(code, "python").unwrap();
            
            assert!(result.contains("token-keyword"));
            assert!(result.contains("token-function"));
            assert!(result.contains("token-string"));
        }

        #[test]
        fn test_code_block_renderer() {
            let renderer = CodeBlockRenderer::new().unwrap();
            let code = "const x = 42;";
            let html = renderer.render(code, "javascript", None).unwrap();
            
            assert!(html.contains("code-block"));
            assert!(html.contains("data-language=\"javascript\""));
            assert!(html.contains("code-header"));
        }

        #[test]
        fn test_css_generator() {
            let generator = CssGenerator::new().unwrap();
            let css = generator.generate().unwrap();
            
            assert!(css.contains(".token-keyword"));
            assert!(css.contains(".token-string"));
            assert!(css.contains(".code-block"));
        }
    }

**Key Points:**
- Unit tests for each component
- Integration tests for pipeline
- Verify syntax highlighting
- Check HTML structure

Step 13: Build and Verify
~~~~~~~~~~~~~~~~~~~~~~~~~

Build the project:

.. code-block:: bash

    cargo build --release
    ./target/release/peta build

Verify output:

.. code-block:: bash

    # Check generated files
    ls -la _out/dist/assets/css/code-blocks.css
    ls -la _out/dist/assets/js/code-blocks.js
    
    # Check syntax highlighting in HTML
    grep "token-keyword" _out/dist/articles/example.html
    grep "token-string" _out/dist/articles/example.html

**Key Points:**
- Verify CSS/JS generation
- Check HTML structure
- Test syntax highlighting
- Validate token classes

Step 14: Performance Testing
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Measure performance improvements:

.. code-block:: bash

    # Build time
    time ./target/release/peta build
    
    # Bundle size
    du -sh _out/dist/assets/css/code-blocks.css
    du -sh _out/dist/assets/js/code-blocks.js
    
    # Runtime performance
    # Measure page load time with browser dev tools

**Key Points:**
- Build time comparison
- Bundle size analysis
- Runtime performance
- Memory usage

Step 15: Documentation
~~~~~~~~~~~~~~~~~~~~~~~~~~

Document the new pipeline:

.. code-block:: rst

    Code Block Rendering Pipeline
    =============================
    
    Overview
    --------
    
    The code block rendering pipeline is now unified in Rust...
    
    Architecture
    ------------
    
    * SyntaxHighlighter - syntect-based highlighting
    * CodeBlockRenderer - HTML generation
    * CssGenerator - Programmatic CSS
    * JsGenerator - Minimal JavaScript
    
    Configuration
    ------------
    
    See peta.toml for all settings.

**Key Points:**
- Document architecture
- Explain configuration
- Provide examples
- Include troubleshooting

Common Issues and Solutions
---------------------------

Issue: TypeScript Not Highlighted
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**Problem:** TypeScript code blocks show plain text.

**Solution:** TypeScript is not in syntect's default syntax set. Map it to JavaScript:

.. code-block:: rust

    fn resolve_language(&self, language: &str) -> String {
        match language.to_lowercase().as_str() {
            "ts" | "typescript" => "javascript".to_string(),
            // ... other mappings
            _ => language.to_string(),
        }
    }

Issue: Meta Classes Not Styled
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**Problem:** Classes like `meta struct go` appear but have no styles.

**Solution:** Add CSS rule for syntect wrapper classes:

.. code-block:: css

    /* Syntect wrapper classes - inherit styling from child elements */
    .meta,
    .source,
    .text {
        color: inherit;
    }

Issue: Prism.js Conflict
~~~~~~~~~~~~~~~~~~~~~~~~

**Problem:** Syntax highlighting not working on some pages.

**Solution:** Remove Prism.js from templates:

.. code-block:: bash

    # Check for Prism.js in templates
    grep -r "prism" themes/default/templates/
    
    # Remove from template files
    # Delete Prism.js script tags

Issue: Build Time Increase
~~~~~~~~~~~~~~~~~~~~~~~~~

**Problem:** Build takes longer than expected.

**Solutions:**
- Enable build caching
- Optimize syntax highlighting
- Parallelize asset generation
- Use release build

Best Practices
-------------

1. **Always use release builds for production:**
   
   .. code-block:: bash
   
       cargo build --release

2. **Enable build caching during development:**

   .. code-block:: bash
   
       ./target/release/peta build --cache

3. **Test with multiple languages:**

   .. code-block:: bash
   
       # Test Python, JavaScript, Rust, Go, etc.
       ./target/release/peta build
       # Check syntax highlighting in output

4. **Monitor bundle sizes:**

   .. code-block:: bash
   
       du -sh _out/dist/assets/css/code-blocks.css
       du -sh _out/dist/assets/js/code-blocks.js

5. **Validate HTML output:**

   .. code-block:: bash
   
       # Check for proper structure
       grep "code-block" _out/dist/**/*.html
       grep "token-" _out/dist/**/*.html

Migration Checklist
------------------

- [ ] Create syntax_highlighter.rs
- [ ] Create code_block_renderer.rs
- [ ] Update directives.rs
- [ ] Update parser.rs
- [ ] Create css_generator.rs
- [ ] Create js_generator.rs
- [ ] Update pipeline.rs
- [ ] Update engine.rs
- [ ] Update base.html
- [ ] Remove theme code_block directory
- [ ] Update peta.toml
- [ ] Write tests
- [ ] Build and verify
- [ ] Performance testing
- [ ] Documentation
- [ ] Remove Prism.js from templates
- [ ] Remove code-related CSS from modal files
- [ ] Final testing across all pages

Conclusion
----------

The unified code block rendering pipeline provides:

- **Single source of truth** in Rust
- **Better performance** with pre-rendered HTML
- **Easier maintenance** with type-safe code
- **Consistent rendering** across all pages
- **Reduced duplication** and complexity
- **Better testing** with deterministic output

All code block functionality is now handled entirely within the Rust backend, with no dependencies on theme JavaScript or CSS files.