Math Formulas Pipeline Implementation: Step by Step
=====================================================

This guide documents the complete step-by-step process of unifying the math formula rendering pipeline in the Peta Rust static site generator, moving from a split Rust/Theme architecture to a unified Rust-based system similar to the code block pipeline.

Prerequisites
------------

- Rust 1.70+ with cargo
- Understanding of RST (reStructuredText) parsing
- Knowledge of static site generation concepts
- Familiarity with KaTeX math rendering library

Problem Statement
-----------------

**Before:** Math formula rendering was split between:
- Rust: HTML generation with data-latex attributes
- Per-page script generation: Different scripts for each page
- Theme templates: Manual script injection in some templates
- Theme JS: Duplicate KaTeX loading in snippet modal
- Theme CSS: Manual math styling

**Issues:**
- **Inconsistent rendering**: Math works on article/snippet/project pages but fails on book pages
- Multiple points of failure
- Duplicate KaTeX loading code
- Manual script injection required in templates
- No unified styling system
- Difficult to maintain and test
- No single source of truth

**Example of the Problem:**

Book page (book.html):
.. code-block:: html

    {% extends "base.html" %}
    {% block content %}
    {{ component(name="book_modal", ...) | safe }}
    {% endblock %}
    <!-- NO {% block scripts %} for math! -->

Article page (article.html):
.. code-block:: html

    {% extends "base.html" %}
    {% block content %}
    {{ component(name="article_modal", ...) | safe }}
    {% endblock %}
    {% block scripts %}
    {% if has_math_formulas %}
    {{ math_render_script | safe }}
    {% endif %}
    {% endblock %}

**Result:** Math formulas render on article pages but NOT on book pages!

**Goal:** Unify all math rendering in Rust with pre-rendered HTML, CSS, and JS generated programmatically, similar to the code block pipeline.

Proposed Solution: Unified Architecture
--------------------------------------

The new architecture follows the same pattern as the successful code block unification:

**Key Principles:**
1. Single source of truth in Rust
2. Pre-generated CSS and JS at build time
3. No manual script injection in templates
4. Consistent rendering across ALL page types
5. Unified styling system
6. Easy to maintain and test

Architecture Overview
-------------------

NEW ARCHITECTURE:

.. code-block:: text

    ┌───────────────────────────────────────────────────────────────────────────┐
    │  UNIFIED RUST-CENTRIC PIPELINE (like code blocks)                         │
    │  ───────────────────────────────────────────────────────────────────────  │
    │  • Rust:          MathProcessor - Detect and extract formulas             │
    │  • Rust:          MathRenderer - Generate complete HTML                   │
    │  • Rust:          MathCssGenerator - Programmatic CSS generation          │
    │  • Rust:          MathJsGenerator - Minimal JavaScript generation         │
    │  • Rust:          AssetPipeline - Generate math.css and math.js           │
    │  • Template:      Simple reference to generated assets                    │
    └───────────────────────────────────────────────────────────────────────────┘

FILE STRUCTURE:

.. code-block:: text

    peta/src/content/rst/
    ├── math_processor.rs      [EXISTING - enhance]
    ├── math_renderer.rs       [EXISTING - refactor]
    ├── math_css_generator.rs  [NEW]
    └── math_js_generator.rs   [NEW]

    peta/src/assets/
    ├── css_generator.rs       [ADD math support]
    ├── js_generator.rs        [ADD math support]
    └── pipeline.rs            [ADD math asset generation]

    themes/default/templates/
    ├── base.html              [REPLACE with unified math assets]
    ├── article.html           [REMOVE {% block scripts %} for math]
    ├── book.html              [NO CHANGES needed]
    ├── snippet.html           [NO CHANGES needed]
    └── project.html           [NO CHANGES needed]

    OUTPUT:
    _out/dist/assets/
    ├── css/
    │   ├── code-blocks.css    [EXISTING]
    │   └── math-formulas.css  [NEW - generated]
    └── js/
        ├── code-blocks.js     [EXISTING]
        └── math-formulas.js   [NEW - generated]

Implementation Steps
--------------------

Step 1: Create MathCssGenerator
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/content/rst/math_css_generator.rs``

Create the CSS generator for math formula styling:

.. code-block:: rust

    use crate::core::Result;
    use serde::{Deserialize, Serialize};

    /// CSS generator for math formulas
    pub struct MathCssGenerator {
        config: MathCssConfig,
    }

    /// Configuration for math CSS generation
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MathCssConfig {
        /// Theme name
        pub theme: String,
        /// Font scaling factor
        pub font_scale: f32,
        /// Line height
        pub line_height: f32,
        /// Display math margin
        pub display_margin: String,
        /// Inline math padding
        pub inline_padding: String,
        /// Color scheme
        pub color_scheme: String,
    }

    impl Default for MathCssConfig {
        fn default() -> Self {
            Self {
                theme: "default".to_string(),
                font_scale: 1.0,
                line_height: 1.5,
                display_margin: "1.5em 0".to_string(),
                inline_padding: "0.2em 0.3em".to_string(),
                color_scheme: "auto".to_string(),
            }
        }
    }

    impl MathCssGenerator {
        /// Create a new CSS generator with default configuration
        pub fn new() -> Result<Self> {
            Self::with_config(MathCssConfig::default())
        }

        /// Create a CSS generator with custom configuration
        pub fn with_config(config: MathCssConfig) -> Result<Self> {
            Ok(Self { config })
        }

        /// Generate complete CSS for math formulas
        pub fn generate(&self) -> Result<String> {
            let mut css = String::new();

            // Header
            css.push_str("/*\n");
            css.push_str(" * Math Formula Styles\n");
            css.push_str(" * Generated by Peta Static Site Generator\n");
            css.push_str(" */\n\n");

            // Display styles
            css.push_str(&self.generate_display_styles());
            css.push('\n');

            // Inline styles
            css.push_str(&self.generate_inline_styles());
            css.push('\n');

            // KaTeX base styles
            css.push_str(&self.generate_katex_base_styles());
            css.push('\n');

            // Theme styles
            css.push_str(&self.generate_theme_styles());
            css.push('\n');

            // Responsive styles
            css.push_str(&self.generate_responsive_styles());
            css.push('\n');

            // Print styles
            css.push_str(&self.generate_print_styles());

            Ok(css)
        }

        /// Generate display math styles
        fn generate_display_styles(&self) -> String {
            format!(
                r#"/* Display Math Styles */
.math-display {{
    display: block;
    text-align: center;
    margin: {};
    padding: 0.5em 0;
    overflow-x: auto;
    overflow-y: hidden;
    line-height: {};
}}

.math-display .katex {{
    font-size: calc(1.1em * {});
    display: inline-block;
    max-width: 100%;
}}
"#,
                self.config.display_margin,
                self.config.line_height,
                self.config.font_scale
            )
        }

        /// Generate inline math styles
        fn generate_inline_styles(&self) -> String {
            format!(
                r#"/* Inline Math Styles */
.math-inline {{
    display: inline;
    white-space: nowrap;
    padding: {};
    line-height: {};
}}

.math-inline .katex {{
    font-size: calc(1em * {});
    vertical-align: middle;
}}
"#,
                self.config.inline_padding,
                self.config.line_height,
                self.config.font_scale
            )
        }

        /// Generate KaTeX base styles
        fn generate_katex_base_styles(&self) -> String {
            r#"/* KaTeX Base Styles */
.katex {
    font-family: 'KaTeX_Main', 'Times New Roman', serif;
    line-height: 1.1;
    text-indent: 0;
}

.katex-display {
    margin: 1em 0;
    text-align: center;
}

.katex-display > .katex {
    text-align: center;
}
"#.to_string()
        }

        /// Generate theme styles
        fn generate_theme_styles(&self) -> String {
            match self.config.theme.as_str() {
                "dark" => self.generate_dark_theme(),
                _ => self.generate_light_theme(),
            }
        }

        /// Generate light theme styles
        fn generate_light_theme(&self) -> String {
            r#"/* Light Theme */
@media (prefers-color-scheme: light) {
    .math-display {
        background: #f8fafc;
        border-radius: 4px;
    }

    .math-inline {
        color: #0f172a;
    }
}
"#.to_string()
        }

        /// Generate dark theme styles
        fn generate_dark_theme(&self) -> String {
            r#"/* Dark Theme */
@media (prefers-color-scheme: dark) {
    .math-display {
        background: #1e293b;
        border-radius: 4px;
    }

    .math-inline {
        color: #e2e8f0;
    }

    .katex {
        color: #e2e8f0;
    }
}
"#.to_string()
        }

        /// Generate responsive styles
        fn generate_responsive_styles(&self) -> String {
            r#"/* Responsive Styles */
@media (max-width: 768px) {
    .math-display {
        font-size: 0.9em;
        padding: 0.3em 0;
    }

    .math-inline {
        font-size: 0.95em;
    }
}

@media (max-width: 480px) {
    .math-display {
        font-size: 0.85em;
        overflow-x: scroll;
    }
}
"#.to_string()
        }

        /// Generate print styles
        fn generate_print_styles(&self) -> String {
            r#"/* Print Styles */
@media print {
    .math-display {
        page-break-inside: avoid;
        margin: 1em 0;
    }

    .math-inline {
        white-space: normal;
    }
}
"#.to_string()
        }

        /// Set theme
        pub fn set_theme(&mut self, theme: &str) {
            self.config.theme = theme.to_string();
        }

        /// Set font scale
        pub fn set_font_scale(&mut self, scale: f32) {
            self.config.font_scale = scale;
        }

        /// Get configuration
        pub fn config(&self) -> &MathCssConfig {
            &self.config
        }
    }

    impl Default for MathCssGenerator {
        fn default() -> Self {
            Self::new().expect("Failed to create MathCssGenerator")
        }
    }

**Key Points:**
- Generates complete CSS for math elements
- Supports themes (light/dark)
- Responsive design for mobile
- Print-friendly styles
- Configurable via MathCssConfig

Step 2: Create MathJsGenerator
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/content/rst/math_js_generator.rs``

Create the JavaScript generator for KaTeX rendering:

.. code-block:: rust

    use crate::core::Result;
    use serde::{Deserialize, Serialize};

    /// JavaScript generator for math formulas
    pub struct MathJsGenerator {
        config: MathJsConfig,
    }

    /// Configuration for math JS generation
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MathJsConfig {
        /// KaTeX version
        pub katex_version: String,
        /// CDN base URL
        pub cdn_base: String,
        /// Load on demand
        pub load_on_demand: bool,
        /// Auto-render on page load
        pub auto_render: bool,
        /// Debug mode
        pub debug_mode: bool,
        /// Support for modals
        pub modal_support: bool,
    }

    impl Default for MathJsConfig {
        fn default() -> Self {
            Self {
                katex_version: "0.16.9".to_string(),
                cdn_base: "https://cdn.jsdelivr.net/npm/katex".to_string(),
                load_on_demand: true,
                auto_render: true,
                debug_mode: false,
                modal_support: true,
            }
        }
    }

    impl MathJsGenerator {
        /// Create a new JS generator with default configuration
        pub fn new() -> Result<Self> {
            Self::with_config(MathJsConfig::default())
        }

        /// Create a JS generator with custom configuration
        pub fn with_config(config: MathJsConfig) -> Result<Self> {
            Ok(Self { config })
        }

        /// Generate complete JavaScript for math formulas
        pub fn generate(&self) -> Result<String> {
            let mut js = String::new();

            // Header
            js.push_str("/**\n");
            js.push_str(" * Math Formula Rendering\n");
            js.push_str(" * Generated by Peta Static Site Generator\n");
            js.push_str(" */\n\n");

            // IIFE to avoid global pollution
            js.push_str("(function() {\n");
            js.push_str("'use strict';\n\n");

            // State management
            js.push_str(&self.generate_state_management());
            js.push('\n');

            // Loader
            js.push_str(&self.generate_loader());
            js.push('\n');

            // Renderer
            js.push_str(&self.generate_renderer());
            js.push('\n');

            // Modal support
            if self.config.modal_support {
                js.push_str(&self.generate_modal_support());
                js.push('\n');
            }

            // Auto-render
            if self.config.auto_render {
                js.push_str(&self.generate_auto_render());
                js.push('\n');
            }

            // Error handler
            js.push_str(&self.generate_error_handler());
            js.push('\n');

            // Close IIFE
            js.push_str("})();\n");

            Ok(js)
        }

        /// Generate state management
        fn generate_state_management(&self) -> String {
            format!(
                r#"// State management
if (typeof window.petaMathLoaded === 'undefined') {{
    window.petaMathLoaded = false;
    window.petaMathPending = false;
}}

{}"#,
                if self.config.debug_mode {
                    "console.log('[Peta Math] Initialized');".to_string()
                } else {
                    String::new()
                }
            )
        }

        /// Generate loader
        fn generate_loader(&self) -> String {
            let css_url = format!(
                "{}@{}/dist/katex.min.css",
                self.config.cdn_base, self.config.katex_version
            );
            let js_url = format!(
                "{}@{}/dist/katex.min.js",
                self.config.cdn_base, self.config.katex_version
            );
            let auto_render_url = format!(
                "{}@{}/dist/contrib/auto-render.min.js",
                self.config.cdn_base, self.config.katex_version
            );

            format!(
                r#"// Load KaTeX on demand
function loadKaTeX() {{
    if (window.petaMathLoaded) {{
        return Promise.resolve();
    }}

    return new Promise(function(resolve, reject) {{
        // Load CSS
        if (!document.querySelector('link[href*="katex.min.css"]')) {{
            const css = document.createElement('link');
            css.rel = 'stylesheet';
            css.href = '{}';
            document.head.appendChild(css);
        }}

        // Load KaTeX JS
        const katex = document.createElement('script');
        katex.src = '{}';
        katex.async = true;

        katex.onload = function() {{
            // Load auto-render extension
            const autoRender = document.createElement('script');
            autoRender.src = '{}';
            autoRender.async = true;

            autoRender.onload = function() {{
                window.petaMathLoaded = true;
                {}
                resolve();
            }};

            autoRender.onerror = reject;
            document.body.appendChild(autoRender);
        }};

        katex.onerror = reject;
        document.body.appendChild(katex);
    }});
}}
"#,
                css_url,
                js_url,
                auto_render_url,
                if self.config.debug_mode {
                    "console.log('[Peta Math] KaTeX loaded');"
                } else {
                    ""
                }
            )
        }

        /// Generate renderer
        fn generate_renderer(&self) -> String {
            r#"// Render math elements
function renderMath() {
    if (!window.petaMathLoaded) {
        window.petaMathPending = true;
        loadKaTeX().then(renderMath).catch(handleError);
        return;
    }

    const elements = document.querySelectorAll('[data-latex]');
    elements.forEach(function(el) {
        const latex = el.getAttribute('data-latex');
        if (latex && window.katex) {
            try {
                el.innerHTML = '';
                window.katex.render(latex, el, {
                    displayMode: el.classList.contains('math-display'),
                    throwOnError: false,
                    trust: true
                });
            } catch (e) {
                handleError(e, el);
            }
        }
    });
}
"#.to_string()
        }

        /// Generate modal support
        fn generate_modal_support(&self) -> String {
            r#"// Modal support - render math when modal opens
function setupModalSupport() {
    // Observer for DOM changes (for dynamic content)
    const observer = new MutationObserver(function(mutations) {
        mutations.forEach(function(mutation) {
            mutation.addedNodes.forEach(function(node) {
                if (node.nodeType === 1) { // Element node
                    const mathElements = node.querySelectorAll ? 
                        node.querySelectorAll('[data-latex]') : [];
                    if (mathElements.length > 0) {
                        renderMath();
                    }
                }
            });
        });
    });

    observer.observe(document.body, {
        childList: true,
        subtree: true
    });

    return observer;
}

setupModalSupport();
"#.to_string()
        }

        /// Generate auto-render
        fn generate_auto_render(&self) -> String {
            r#"// Auto-render on page load
function autoRender() {
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', renderMath);
    } else {
        renderMath();
    }
}

autoRender();
"#.to_string()
        }

        /// Generate error handler
        fn generate_error_handler(&self) -> String {
            format!(
                r#"// Error handling
function handleError(error, element) {{
    if (element) {{
        element.innerHTML = element.getAttribute('data-latex');
        element.classList.add('math-error');
    }}
    {}
}}

// Global error handler for unhandled promise rejections
window.addEventListener('unhandledrejection', function(event) {{
    if (event.reason) {{
        handleError(event.reason);
    }}
}});
"#,
                if self.config.debug_mode {
                    "console.error('[Peta Math] Error:', error);"
                } else {
                    ""
                }
            )
        }

        /// Set debug mode
        pub fn set_debug_mode(&mut self, debug: bool) {
            self.config.debug_mode = debug;
        }

        /// Get configuration
        pub fn config(&self) -> &MathJsConfig {
            &self.config
        }
    }

    impl Default for MathJsGenerator {
        fn default() -> Self {
            Self::new().expect("Failed to create MathJsGenerator")
        }
    }

**Key Points:**
- Generates minimal JavaScript for KaTeX
- On-demand loading (only when math is detected)
- Auto-render on page load
- Modal support via MutationObserver
- Error handling and debugging

Step 3: Refactor MathRenderer
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/content/rst/math_renderer.rs``

Refactor to remove script generation:

.. code-block:: rust

    use crate::core::Result;
    use regex::Regex;
    std::collections::HashMap;

    /// Math renderer for LaTeX equations
    pub struct MathRenderer {
        cache: HashMap<String, String>,
    }

    impl MathRenderer {
        /// Create a new math renderer
        pub fn new() -> Self {
            Self {
                cache: HashMap::new(),
            }
        }

        /// Check if content needs math rendering
        pub fn should_render(&self, content: &str) -> bool {
            content.contains("$") ||
            content.contains("\\[") ||
            content.contains("\\(") ||
            content.contains("data-latex")
        }

        /// Render math equations in HTML content
        pub fn render(&mut self, content: &str) -> Result<String> {
            let mut result = content.to_string();

            // Render display math ($$...$$ or \[...\])
            result = self.render_display_math(&result)?;

            // Render inline math ($...$ or \(...\))
            result = self.render_inline_math(&result)?;

            Ok(result)
        }

        /// Render display math equations
        fn render_display_math(&mut self, content: &str) -> Result<String> {
            static DISPLAY_REGEX: once_cell::sync::Lazy<Regex> =
                once_cell::sync::Lazy::new(|| Regex::new(r"\$\$(.*?)\$\$").unwrap());
            static LATEX_REGEX: once_cell::sync::Lazy<Regex> =
                once_cell::sync::Lazy::new(|| Regex::new(r"\\\\\[(.*?)\\\\\]").unwrap());

            let mut result = content.to_string();

            // Handle $$...$$ delimiters
            result = DISPLAY_REGEX.replace_all(&result, |caps: &regex::Captures| {
                let equation = caps.get(1).unwrap().as_str().trim();
                self.render_equation(equation, true).unwrap_or_else(|_| {
                    format!("<span class=\"math-error\">{}</span>", equation)
                })
            }).to_string();

            // Handle \[...\] delimiters
            result = LATEX_REGEX.replace_all(&result, |caps: &regex::Captures| {
                let equation = caps.get(1).unwrap().as_str().trim();
                self.render_equation(equation, true).unwrap_or_else(|_| {
                    format!("<span class=\"math-error\">{}</span>", equation)
                })
            }).to_string();

            Ok(result)
        }

        /// Render inline math equations
        fn render_inline_math(&mut self, content: &str) -> Result<String> {
            static LATEX_INLINE_REGEX: once_cell::sync::Lazy<Regex> =
                once_cell::sync::Lazy::new(|| Regex::new(r"\\\\\((.*?)\\\\\)").unwrap());

            let mut result = content.to_string();

            // Handle \(...\) delimiters first
            result = LATEX_INLINE_REGEX.replace_all(&result, |caps: &regex::Captures| {
                let equation = caps.get(1).unwrap().as_str().trim();
                self.render_equation(equation, false).unwrap_or_else(|_| {
                    format!("<span class=\"math-error\">{}</span>", equation)
                })
            }).to_string();

            // Handle $...$ delimiters with a custom approach to avoid $$...$$
            let display_marker = uuid::Uuid::new_v4().to_string();
            static DISPLAY_TEMP_REGEX: once_cell::sync::Lazy<Regex> =
                once_cell::sync::Lazy::new(|| Regex::new(r"\$\$(.*?)\$\$").unwrap());

            result = DISPLAY_TEMP_REGEX.replace_all(&result, |caps: &regex::Captures| {
                format!("__DISPLAY_MATH_{}__{}", display_marker, caps.get(0).unwrap().as_str())
            }).to_string();

            static INLINE_REGEX: once_cell::sync::Lazy<Regex> =
                once_cell::sync::Lazy::new(|| Regex::new(r"\$([^$\n]+?)\$").unwrap());

            result = INLINE_REGEX.replace_all(&result, |caps: &regex::Captures| {
                let equation = caps.get(1).unwrap().as_str().trim();
                self.render_equation(equation, false).unwrap_or_else(|_| {
                    format!("<span class=\"math-error\">{}</span>", equation)
                })
            }).to_string();

            static RESTORE_REGEX: once_cell::sync::Lazy<Regex> =
                once_cell::sync::Lazy::new(|| Regex::new(r"__DISPLAY_MATH_[^_]+__\$\$(.*?)\$\$").unwrap());

            result = RESTORE_REGEX.replace_all(&result, "$$$1$$").to_string();

            Ok(result)
        }

        /// Render a single equation
        fn render_equation(&mut self, equation: &str, display: bool) -> Result<String> {
            let cache_key = format!("{}:{}", equation, display);
            if let Some(cached) = self.cache.get(&cache_key) {
                return Ok(cached.clone());
            }

            let rendered = if display {
                format!(
                    r#"<div class="math-display" data-latex="{}"></div>"#,
                    equation
                )
            } else {
                format!(
                    r#"<span class="math-inline" data-latex="{}"></span>"#,
                    equation
                )
            };

            self.cache.insert(cache_key, rendered.clone());
            Ok(rendered)
        }
    }

    impl Default for MathRenderer {
        fn default() -> Self {
            Self::new()
        }
    }

**Key Changes:**
- Removed ``generate_on_demand_script()`` method
- Simplified to HTML generation only
- Kept caching for performance
- No per-page script generation

Step 4: Enhance MathProcessor
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/content/rst/math_processor.rs``

Enhance with caching and better detection:

.. code-block:: rust

    // ... existing code ...

    impl MathProcessor {
        // ... existing methods ...

        /// Auto-detect math content with caching
        pub fn auto_detect_math_content(&self, content: &str) -> Result<MathDetectionResult> {
            // Check cache first
            let content_hash = self.hash_content(content);
            if let Some(cached) = self.detection_cache.get(&content_hash) {
                return Ok(cached.clone());
            }

            // Perform detection
            let result = self.perform_detection(content)?;

            // Cache the result
            self.detection_cache.insert(content_hash, result.clone());

            Ok(result)
        }

        /// Perform actual detection
        fn perform_detection(&self, content: &str) -> Result<MathDetectionResult> {
            // First try to extract from original LaTeX syntax
            let math_blocks = self.extract_math_blocks(content)?;
            if !math_blocks.is_empty() {
                return Ok(MathDetectionResult {
                    has_formulas: true,
                    formula_count: math_blocks.len(),
                    math_blocks,
                });
            }

            // If no LaTeX found, check for data-latex attributes (already processed content)
            let data_latex_regex = Regex::new(r#"data-latex="([^"]*)""#)
                .map_err(|e| Error::content(format!("Invalid data-latex regex: {}", e)))?;

            let math_count = data_latex_regex.find_iter(content).count();
            let has_formulas = math_count > 0;

            Ok(MathDetectionResult {
                has_formulas,
                formula_count: math_count,
                math_blocks: Vec::new(),
            })
        }

        /// Hash content for caching
        fn hash_content(&self, content: &str) -> String {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            let mut hasher = DefaultHasher::new();
            content.hash(&mut hasher);
            format!("{:x}", hasher.finish())
        }

        /// Clear detection cache
        pub fn clear_cache(&mut self) {
            self.detection_cache.clear();
        }
    }

**Key Enhancements:**
- Added detection caching
- Better performance
- Hash-based cache keys
- Clear cache method for testing

Step 5: Update AssetPipeline
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/assets/pipeline.rs``

Add math asset generation:

.. code-block:: rust

    use crate::content::rst::{MathCssGenerator, MathJsGenerator};
    use crate::core::Result;

    impl AssetPipeline {
        pub fn build(&mut self) -> Result<()> {
            // Generate code block assets
            self.generate_code_block_assets()?;

            // Generate math formula assets
            self.generate_math_assets()?;

            Ok(())
        }

        /// Generate math formula assets
        fn generate_math_assets(&mut self) -> Result<()> {
            // Create generators
            let css_generator = MathCssGenerator::new()?;
            let js_generator = MathJsGenerator::new()?;

            // Generate CSS
            let css_output = self.output_dir.join("css").join("math-formulas.css");
            let css_content = css_generator.generate()?;
            std::fs::write(&css_output, css_content)?;

            // Generate JS
            let js_output = self.output_dir.join("js").join("math-formulas.js");
            let js_content = js_generator.generate()?;
            std::fs::write(&js_output, js_content)?;

            Ok(())
        }
    }

**Key Points:**
- Generates math-formulas.css
- Generates math-formulas.js
- Integrated into existing build pipeline
- Writes to _out/dist/assets/

Step 6: Update Parser
~~~~~~~~~~~~~~~~~~~~~

File: ``peta/src/content/rst/parser.rs``

Remove math_render_script from output:

.. code-block:: rust

    impl RstParser {
        pub fn parse_with_type_and_path(
            &mut self,
            content: &str,
            content_type_override: Option<ContentType>,
            file_path: Option<&std::path::Path>,
        ) -> Result<RstContent> {
            // ... existing code ...

            // 5. Detect math formulas (keep for optimization)
            let math_detection = self.math_processor.auto_detect_math_content(&processed_html)?;

            // Note: math_render_script is NO LONGER generated here
            // It's now generated once by MathJsGenerator and loaded globally

            Ok(RstContent {
                metadata,
                html: processed_html,
                toc,
                toc_html,
                frontmatter,
                has_math_formulas: math_detection.has_formulas,
                math_formula_count: math_detection.formula_count,
                math_render_script: None, // REMOVED - now in global assets
            })
        }
    }

**Key Changes:**
- Removed math_render_script generation
- Keep has_math_formulas flag for optimization
- Math rendering now handled globally

Step 7: Update Templates
~~~~~~~~~~~~~~~~~~~~~~~~

File: ``themes/default/templates/base.html``

Add unified math asset references:

.. code-block:: html

    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>{% block title %}{{ site.title }}{% endblock title %}</title>

        <!-- External CSS -->
        <link rel="stylesheet" href="/css/main.css">

        <!-- Code Block Styles (generated from Rust) -->
        <link rel="stylesheet" href="/assets/css/code-blocks.css">

        <!-- Math Formula Styles (generated from Rust) -->
        <link rel="stylesheet" href="/assets/css/math-formulas.css">

        <!-- Component Styles -->
        <style>
        {{ component_styles(...) | safe }}
        </style>

        {% block extra_styles %}{% endblock extra_styles %}
    </head>
    <body>
        <!-- ... content ... -->

        <!-- External JavaScript -->
        <script src="/js/main.js"></script>

        <!-- Code Block Scripts (generated from Rust) -->
        <script src="/assets/js/code-blocks.js"></script>

        <!-- Math Formula Scripts (generated from Rust) -->
        <script src="/assets/js/math-formulas.js"></script>

        <!-- Component Scripts -->
        <script>
        {{ component_scripts(...) | safe }}
        </script>

        <!-- Snippet Modal Script (remove duplicate math rendering code) -->
        <script>
        (function() {
            'use strict';

            function initSnippetModal() {
                // ... modal initialization ...
                // REMOVE all KaTeX loading code - now in math-formulas.js
            }

            // ... rest of modal code ...
        })();
        </script>

        {% block scripts %}{% endblock scripts %}
    </body>
    </html>

File: ``themes/default/templates/article.html``

Remove math script injection:

.. code-block:: html

    {% extends "base.html" %}

    {% block title %}{{ page.title }} - {{ site.title }}{% endblock %}

    {% block content %}
    {{ component(name="page_tags", ...) | safe }}
    {{ component(name="article_modal", ...) | safe }}
    {% endblock %}

    {% block scripts %}
    <!-- REMOVED: No longer needed - math renders globally via base.html -->
    <!-- {% if has_math_formulas %} -->
    <!-- {{ math_render_script | safe }} -->
    <!-- {% endif %} -->
    {% endblock %}

File: ``themes/default/templates/book.html``

No changes needed:

.. code-block:: html

    {% extends "base.html" %}

    {% block title %}{{ page.title }} - {{ site.title }}{% endblock %}

    {% block content %}
    {{ component(name="page_tags", ...) | safe }}
    {{ component(name="book_modal", ...) | safe }}
    {% endblock %}

    <!-- NO {% block scripts %} needed - math renders globally via base.html -->

**Key Points:**
- Single point of reference in base.html
- All pages get math rendering automatically
- No manual script injection needed
- Book pages now work without any changes

Step 8: Update Configuration
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

File: ``peta.toml``

Add math rendering configuration:

.. code-block:: toml

    [math_rendering]
    # Math rendering engine (katex only supported for now)
    engine = "katex"

    # Enable automatic math detection
    auto_detect = true

    # Enable on-demand loading (only load KaTeX when math is detected)
    on_demand_loading = true

    [math_rendering.katex]
    # KaTeX version to use
    version = "0.16.9"

    # CDN base URL
    cdn_base = "https://cdn.jsdelivr.net/npm/katex"

    [math_rendering.css]
    # Theme for math styling
    theme = "default"

    # Font scaling factor
    font_scale = 1.0

    # Line height
    line_height = 1.5

    # Display math margin
    display_margin = "1.5em 0"

    # Inline math padding
    inline_padding = "0.2em 0.3em"

    [math_rendering.js]
    # Auto-render on page load
    auto_render = true

    # Enable debug mode
    debug_mode = false

    # Render in modals
    modal_support = true

**Key Points:**
- Centralized configuration
- KaTeX version control
- CSS customization
- JS behavior control
- Debug mode for development

Step 9: Remove Old Code
~~~~~~~~~~~~~~~~~~~~~~~

Remove duplicate KaTeX loading from snippet modal in base.html:

.. code-block:: html

    <!-- BEFORE (in snippet modal script): -->
    <script>
    function initSnippetModal() {
        // ... modal code ...

        if (hasMathFormulas) {
            // LOAD KATEX - DUPLICATE CODE
            function loadKaTeX() {
                const css = document.createElement('link');
                css.rel = 'stylesheet';
                css.href = 'https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css';
                document.head.appendChild(css);
                // ... more KaTeX loading code ...
            }
            loadKaTeX();
        }
    }
    </script>

    <!-- AFTER (in snippet modal script): -->
    <script>
    function initSnippetModal() {
        // ... modal code ...

        if (hasMathFormulas) {
            // KATEX ALREADY LOADED by math-formulas.js
            // Just trigger render if needed
            if (window.petaMathLoaded) {
                renderMath();
            }
        }
    }
    </script>

Remove math_render_script references from RstContent:

.. code-block:: rust

    // peta/src/content/mod.rs

    pub struct RstContent {
        pub metadata: ContentMetadata,
        pub html: String,
        pub toc: Vec<TocEntry>,
        pub toc_html: String,
        pub frontmatter: HashMap<String, serde_json::Value>,
        pub has_math_formulas: bool,        // KEEP for optimization
        pub math_formula_count: usize,      // KEEP for statistics
        // pub math_render_script: Option<String>,  // REMOVED
    }

**Key Points:**
- Remove duplicate KaTeX loading code
- Remove math_render_script from RstContent
- Keep has_math_formulas for optimization
- Clean up old rendering logic

Step 10: Test Implementation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Create comprehensive tests:

.. code-block:: rust

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_math_css_generator() {
            let generator = MathCssGenerator::new().unwrap();
            let css = generator.generate().unwrap();

            assert!(css.contains(".math-display"));
            assert!(css.contains(".math-inline"));
            assert!(css.contains("@media print"));
            assert!(css.contains("@media (max-width: 768px)"));
        }

        #[test]
        fn test_math_js_generator() {
            let generator = MathJsGenerator::new().unwrap();
            let js = generator.generate().unwrap();

            assert!(js.contains("function loadKaTeX()"));
            assert!(js.contains("function renderMath()"));
            assert!(js.contains("window.petaMathLoaded"));
            assert!(js.contains("MutationObserver"));
        }

        #[test]
        fn test_math_renderer_without_script() {
            let mut renderer = MathRenderer::new();
            let content = "The formula $E = mc^2$ is famous.";
            let html = renderer.render(&content).unwrap();

            assert!(html.contains("data-latex"));
            assert!(!html.contains("<script>")); // No script generation
        }

        #[test]
        fn test_math_processor_caching() {
            let processor = MathProcessor::new().unwrap();
            let content = "Formula: $x^2$";

            // First call
            let result1 = processor.auto_detect_math_content(content).unwrap();
            assert_eq!(result1.formula_count, 1);

            // Second call - should use cache
            let result2 = processor.auto_detect_math_content(content).unwrap();
            assert_eq!(result2.formula_count, 1);
        }

        #[test]
        fn test_math_assets_generated() {
            let pipeline = AssetPipeline::new(/* ... */).unwrap();
            pipeline.build().unwrap();

            let css_path = pipeline.output_dir.join("css").join("math-formulas.css");
            let js_path = pipeline.output_dir.join("js").join("math-formulas.js");

            assert!(css_path.exists());
            assert!(js_path.exists());

            let css_content = std::fs::read_to_string(&css_path).unwrap();
            let js_content = std::fs::read_to_string(&js_path).unwrap();

            assert!(css_content.contains(".math-display"));
            assert!(js_content.contains("function loadKaTeX()"));
        }
    }

**Key Points:**
- Test CSS generation
- Test JS generation
- Verify no script generation in MathRenderer
- Test caching in MathProcessor
- Verify asset generation

Step 11: Build and Verify
~~~~~~~~~~~~~~~~~~~~~~~~~~~

Build the project:

.. code-block:: bash

    cargo build --release
    ./target/release/peta build

Verify output:

.. code-block:: bash

    # Check generated files
    ls -la _out/dist/assets/css/math-formulas.css
    ls -la _out/dist/assets/js/math-formulas.js

    # Check math elements in HTML
    grep "data-latex" _out/dist/articles/calculus-fundamentals.html
    grep "data-latex" _out/dist/books/deep-learning-with-python/computer-vision.html
    grep "data-latex" _out/dist/snippets/integrals.html

    # Verify base.html includes math assets
    grep "math-formulas.css" _out/dist/**/*.html
    grep "math-formulas.js" _out/dist/**/*.html

**Key Points:**
- Verify CSS/JS generation
- Check HTML structure
- Test math rendering on all page types
- Verify book pages now render math

Step 12: Browser Testing
~~~~~~~~~~~~~~~~~~~~~~~~

Test in browser:

.. code-block:: bash

    # Start dev server
    ./target/release/peta serve

    # Test URLs in browser
    # http://localhost:3566/articles/calculus-fundamentals.html
    # http://localhost:3566/books/deep-learning-with-python/computer-vision.html
    # http://localhost:3566/snippets/integrals.html
    # http://localhost:3566/projects/interactive-mathematical-visualizer.html

    # Check browser console for errors
    # Verify KaTeX loads only once
    # Verify math renders correctly on all pages
    # Verify math renders in modals

**Expected Results:**
- ✅ Math formulas render on article pages
- ✅ Math formulas render on book pages (FIXED!)
- ✅ Math formulas render on snippet pages
- ✅ Math formulas render on project pages
- ✅ KaTeX loads only once per session
- ✅ Math renders in snippet modals
- ✅ No console errors

Common Issues and Solutions
---------------------------

Issue: Math Not Rendering on Book Pages
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**Problem:** Book pages still show raw LaTeX instead of rendered math.

**Solution:** Verify base.html includes math assets:

.. code-block:: bash

    # Check base.html template
    grep "math-formulas.css" themes/default/templates/base.html
    grep "math-formulas.js" themes/default/templates/base.html

    # Should output:
    # <link rel="stylesheet" href="/assets/css/math-formulas.css">
    # <script src="/assets/js/math-formulas.js"></script>

Issue: KaTeX Loading Multiple Times
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**Problem:** KaTeX CSS and JS load multiple times on page navigation.

**Solution:** Verify the load check in math-formulas.js:

.. code-block:: javascript

    // Should have this check:
    if (typeof window.petaMathLoaded === 'undefined') {
        window.petaMathLoaded = false;
    }

    function loadKaTeX() {
        if (window.petaMathLoaded) {
            return Promise.resolve(); // Don't load again
        }
        // ... loading code ...
    }

Issue: Math Not Rendering in Modals
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**Problem:** Math formulas in snippet modals don't render.

**Solution:** Verify MutationObserver is set up:

.. code-block:: javascript

    // math-formulas.js should include:
    function setupModalSupport() {
        const observer = new MutationObserver(function(mutations) {
            mutations.forEach(function(mutation) {
                mutation.addedNodes.forEach(function(node) {
                    if (node.nodeType === 1) {
                        const mathElements = node.querySelectorAll('[data-latex]');
                        if (mathElements.length > 0) {
                            renderMath();
                        }
                    }
                });
            });
        });

        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    }

Issue: Assets Not Generated
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**Problem:** math-formulas.css and math-formulas.js not in output.

**Solution:** Verify AssetPipeline generates math assets:

.. code-block:: rust

    // peta/src/assets/pipeline.rs

    impl AssetPipeline {
        pub fn build(&mut self) -> Result<()> {
            // ... existing code ...

            // Make sure this is called:
            self.generate_math_assets()?;  // <-- ADD THIS

            Ok(())
        }
    }

Issue: Styles Not Applied
~~~~~~~~~~~~~~~~~~~~~~~~~~

**Problem:** Math formulas render but have no styling.

**Solution:** Verify CSS file path and theme configuration:

.. code-block:: bash

    # Check CSS file exists
    ls -la _out/dist/assets/css/math-formulas.css

    # Check CSS content
    cat _out/dist/assets/css/math-formulas.css

    # Should contain:
    # .math-display { ... }
    # .math-inline { ... }
    # .katex { ... }

Performance Considerations
---------------------------

The new unified pipeline provides several performance benefits:

1. **Pre-generated Assets:** CSS and JS generated once at build time
2. **On-demand Loading:** KaTeX loads only when math is detected
3. **Caching:** Math detection and rendering cached
4. **No Duplicate Code:** Single KaTeX loading path
5. **Browser Caching:** Assets cached by browser after first load

Performance Comparison:

.. code-block:: text

    OLD SYSTEM:
    - Per-page script generation (CPU intensive)
    - Multiple KaTeX loading paths
    - Duplicate code execution
    - Browser downloads multiple scripts

    NEW SYSTEM:
    - Assets generated once at build time
    - Single KaTeX loading path
    - No duplicate code execution
    - Browser caches assets

    Expected Improvement:
    - Build time: Similar (small increase for asset generation)
    - Page load time: Faster (no per-page script generation)
    - Runtime performance: Better (cached rendering)
    - Memory usage: Lower (no duplicate code)

Migration Checklist
------------------

- [ ] Create MathCssGenerator
- [ ] Create MathJsGenerator
- [ ] Refactor MathRenderer (remove generate_on_demand_script)
- [ ] Enhance MathProcessor (add caching)
- [ ] Update AssetPipeline (add generate_math_assets)
- [ ] Update Parser (remove math_render_script)
- [ ] Update base.html (add math asset references)
- [ ] Update article.html (remove {% block scripts %} for math)
- [ ] Clean up snippet modal (remove duplicate KaTeX loading)
- [ ] Update peta.toml (add math_rendering section)
- [ ] Write tests
- [ ] Build and verify
- [ ] Test in browser (all page types)
- [ ] Test modals
- [ ] Performance testing
- [ ] Update documentation

Conclusion
----------

The unified math formula rendering pipeline provides:

- **Consistency:** Single rendering pipeline for ALL page types
- **Reliability:** Math works on book pages, article pages, snippet pages, and project pages
- **Performance:** Pre-generated assets, on-demand loading, caching
- **Maintainability:** Single source of truth in Rust
- **Flexibility:** Theme support, configurable KaTeX version
- **Simplicity:** No manual script injection in templates

All math formula functionality is now handled entirely within the Rust backend, with pre-generated CSS and JS assets loaded globally via base.html. This ensures consistent rendering across the entire site and eliminates the book page math rendering issue.

References
----------

- Code Block Pipeline: docs/features/codeblocks/codeblock_pipeline_step_by_step.rst
- KaTeX Documentation: https://katex.org/docs/
- Supported LaTeX Functions: https://katex.org/docs/supported.html
- Peta Source Code: https://github.com/h3x49r4m/peta-rust