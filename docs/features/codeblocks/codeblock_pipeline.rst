Unified Code Block Rendering Pipeline
======================================

Overview
--------

The code block rendering pipeline has been redesigned to provide a unified, Rust-based system that generates complete HTML with syntax highlighting, eliminating the split responsibilities between Rust and theme JavaScript/CSS files.

Architecture
------------

The new architecture consolidates all rendering logic in the Rust backend:

.. image:: code-block-pipeline.png
   :alt: Code Block Rendering Pipeline

Core Components
---------------

Rust Rendering Layer
~~~~~~~~~~~~~~~~~~~~

SyntaxHighlighter (NEW)
    Uses the ``syntect`` crate for actual syntax highlighting with support for:
    
    - Tokenization of code into semantic tokens (keywords, strings, comments, numbers, etc.)
    - Theme application (One Dark, Solarized, and custom themes)
    - HTML generation with proper CSS classes for each token type
    - Support for 100+ programming languages

CodeBlockRenderer (NEW)
    Unified code block rendering entry point that:
    
    - Generates complete, self-contained HTML structure
    - Includes all necessary CSS classes and data attributes
    - Handles language detection and resolution with aliases
    - Supports code block options (line numbers, highlight lines, copy button)
    - Produces pre-rendered HTML (static, no runtime parsing)

Directives (REFACTORED)
    Simplified directive handling that:
    
    - Parses ``.. code-block::`` directives from RST files
    - Delegates rendering to CodeBlockRenderer
    - Focuses on validation and configuration only

Asset Generation Layer
~~~~~~~~~~~~~~~~~~~~~~

CssGenerator (NEW)
    Generates CSS programmatically from Rust including:
    
    - Token color mappings based on selected theme
    - Code block container styling
    - Line number styling with hover effects
    - Copy button animations
    - Responsive design breakpoints
    - Theme-specific color schemes

JsGenerator (NEW)
    Generates minimal JavaScript for client-side interactions:
    
    - Copy to clipboard functionality using Clipboard API
    - Line number hover effects (CSS-driven where possible)
    - Keyboard shortcuts (Ctrl/Cmd+K to copy)
    - Error handling and user feedback
    - Single bundled file for all code blocks

Pipeline (ENHANCED)
    Enhanced asset pipeline that:
    
    - Integrates syntax highlighting into build process
    - Generates CSS from Rust instead of bundling theme CSS files
    - Pre-renders all code blocks during build (static HTML)
    - Supports hot-reloading in development mode
    - Handles minification and cache busting

Template Engine Integration
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Engine (UPDATED)
    Template engine enhancements:
    
    - Registers new code block renderer function
    - Injects generated CSS/JS into templates
    - Supports code block rendering without theme HTML files
    - Maintains backward compatibility with existing components

Rendering Flow
--------------

.. code-block:: text

    RST File
      ↓
    RstParser
      ↓
    Code Block Directive
      ↓
    CodeBlockRenderer
      ├─ SyntaxHighlighter (syntect)
      │  ├─ Tokenization
      │  ├─ Theme Application
      │  └─ HTML Generation
      ├─ HTML Assembly
      │  ├─ Header (language, copy button)
      │  └─ Content (line numbers, highlighted code)
      └─ Output HTML String
      ↓
    Template Engine
      ├─ Inject into content
      └─ Add CSS/JS references
      ↓
    Asset Generators
      ├─ CssGenerator → code-blocks.css
      └─ JsGenerator → code-blocks.js
      ↓
    Final HTML Output
      ↓
    Browser (static rendering)

Configuration
-------------

Peta Configuration
~~~~~~~~~~~~~~~~~~

All code block settings are configured in ``peta.toml``:

.. code-block:: toml

    [code_blocks]
    default_theme = "one-dark"
    enable_line_numbers = true
    enable_copy_button = true
    
    [code_blocks.languages]
    python = { aliases = ["py", "python3"] }
    javascript = { aliases = ["js", "node"] }
    typescript = { aliases = ["ts", "tsx"] }
    
    [code_blocks.themes]
    one-dark = { name = "One Dark", colors = {...} }
    solarized = { name = "Solarized", colors = {...} }

Features Preserved
------------------

Syntax Highlighting
    Full syntect-based highlighting with 100+ language support and customizable themes

Line Numbers
    Pre-rendered in Rust with support for highlighting specific lines

Copy Functionality
    Copy button HTML generated in Rust with clipboard API in minimal JS

Line Highlights
    Data attributes + CSS for highlighting specific lines

Keyboard Shortcuts
    Ctrl/Cmd+K to copy focused code block

Theme Support
    Multiple built-in themes (One Dark, Solarized) with custom theme support

Responsive Design
    Mobile-friendly with CSS media queries

Accessibility
    ARIA labels and keyboard navigation support

Performance
    Static HTML rendering with no runtime parsing or client-side processing

Customization
    YAML and TOML configuration for all options

Benefits
--------

Single Source of Truth
    All rendering logic in one place (Rust) with no theme file dependencies

Better Performance
    Pre-rendered HTML, minimal JavaScript, no runtime parsing

Easier Maintenance
    Type-safe Rust code with compile-time checks, no template maintenance

Theme Consistency
    Rust-controlled themes ensure consistency across the site

Reduced Duplication
    No split between Rust, theme CSS, and theme JS - everything in peta/

Better Testing
    Pure Rust functions are easier to test with deterministic output

Future-Proof
    Easy to add new features without touching theme files or templates

Simplified Deployment
    No theme-specific code block assets to manage or version
    Single point of configuration in ``peta.toml``

Migration Path
--------------

Backward Compatibility
    - Legacy theme files can remain but will be ignored
    - Configuration automatically migrates from YAML to TOML
    - Feature flags enable gradual migration

Migration Steps
    1. Create ``syntax_highlighter.rs`` with syntect integration
    2. Create ``code_block_renderer.rs`` for unified rendering
    3. Refactor ``directives.rs`` to use new renderer
    4. Create ``css_generator.rs`` for programmatic CSS
    5. Create ``js_generator.rs`` for minimal client-side JS
    6. Update ``pipeline.rs`` to integrate new generators
    7. Update ``engine.rs`` for template integration
    8. Remove ``themes/default/components/atomic/code_block/`` directory
    9. Update ``peta.toml`` with code block configuration
    10. Test all existing features
    11. Performance testing

Theme Structure Changes
------------------------

Complete Removal of Theme Files
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The entire ``themes/default/components/atomic/code_block/`` directory is **removed**. All code block rendering is now handled entirely within the ``peta/`` Rust codebase.

.. code-block:: text

    REMOVED:
    themes/default/components/atomic/code_block/
      ├── component.yaml
      ├── code_block.html
      ├── code_block.css
      └── code_block.js

    NEW (in peta/):
    peta/src/content/rst/
      ├── syntax_highlighter.rs    (syntect-based highlighting)
      ├── code_block_renderer.rs   (unified HTML generation)
      └── directives.rs            (updated to use renderer)
    
    peta/src/assets/
      ├── css_generator.rs         (programmatic CSS generation)
      ├── js_generator.rs          (minimal JS generation)
      └── pipeline.rs              (enhanced asset pipeline)
    
    peta/src/templates/
      └── engine.rs                (updated for integration)

No Theme Code Block Files
~~~~~~~~~~~~~~~~~~~~~~~~~~

All code block functionality is now:

- **HTML generation**: ``peta/src/content/rst/code_block_renderer.rs``
- **CSS generation**: ``peta/src/assets/css_generator.rs``
- **JS generation**: ``peta/src/assets/js_generator.rs``
- **Configuration**: ``peta.toml`` only

Configuration
-------------

The component YAML is **removed**. All configuration is now in ``peta.toml``:

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
    one-dark = { name = "One Dark", colors = {...} }
    solarized = { name = "Solarized", colors = {...} }
    
    [code_blocks.styles]
    font_family = "SF Mono"
    font_size = "0.9rem"
    line_height = "1.6"
    border_radius = "1rem"

Comparison: Before vs After
----------------------------

Before (Split Responsibilities)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. code-block:: text

    Rust Parser
      - HTML generation (basic structure)
    
    Theme CSS
      - Token color classes
      - Container styling
      - Line number styling
    
    Theme JS
      - Copy functionality
      - Line hover effects
      - Keyboard shortcuts
    
    Browser
      - Runtime rendering
      - Multiple asset loads

After (Unified)
^^^^^^^^^^^^^^

.. code-block:: text

    Rust Rendering Layer
      - SyntaxHighlighter (syntect)
      - CodeBlockRenderer
      - Asset Generators
    
    Browser
      - Static HTML rendering
      - Single CSS file
      - Minimal JS (interactions only)

Implementation Details
----------------------

HTML Structure
~~~~~~~~~~~~~~

Generated HTML structure:

.. code-block:: html

    <div class="code-block" data-language="python" data-theme="one-dark">
      <div class="code-header">
        <span class="code-language">PYTHON</span>
        <button class="code-copy-button" onclick="copyCode(this)">
          <svg>...</svg>
          <span>Copy</span>
        </button>
      </div>
      <div class="code-content with-line-numbers">
        <pre><code class="language-python">
          <span class="line-number" data-line="1">1</span>
          <span class="token-keyword">def</span>
          <span class="token-function">hello</span>
          <span class="token-punctuation">(</span>
          <span class="token-punctuation">)</span>
          <span class="token-punctuation">:</span>
          <span class="token-operator"> </span>
          <span class="token-string">"world"</span>
        </code></pre>
      </div>
    </div>

CSS Classes
~~~~~~~~~~~

Token classes generated by SyntaxHighlighter:

- ``token-keyword`` - Language keywords
- ``token-string`` - String literals
- ``token-comment`` - Comments
- ``token-number`` - Numeric literals
- ``token-function`` - Function names
- ``token-variable`` - Variable names
- ``token-type`` - Type names
- ``token-operator`` - Operators
- ``token-punctuation`` - Punctuation
- ``token-property`` - Object properties
- ``token-tag`` - HTML/XML tags
- ``token-attribute`` - HTML/XML attributes
- ``token-selector`` - CSS selectors

JavaScript Functions
~~~~~~~~~~~~~~~~~~~

Minimal JavaScript functions:

- ``copyCode(button)`` - Copy code to clipboard
- ``initializeLineHover()`` - Line hover effects (CSS-driven)
- ``initializeKeyboardShortcuts()`` - Keyboard navigation

Testing
-------

Unit Tests
~~~~~~~~~~

- SyntaxHighlighter tokenization tests
- CodeBlockRenderer HTML generation tests
- CssGenerator output tests
- JsGenerator output tests

Integration Tests
~~~~~~~~~~~~~~~~

- End-to-end RST to HTML rendering tests
- Theme application tests
- Asset pipeline tests
- Browser compatibility tests

Performance Tests
~~~~~~~~~~~~~~~~~

- Build time benchmarks
- Bundle size comparisons
- Runtime performance measurements
- Memory usage profiling

Troubleshooting
---------------

Common Issues
~~~~~~~~~~~~~

Syntax Highlighting Not Working
    - Check if language is supported by syntect
    - Verify language aliases in configuration
    - Ensure theme is properly configured

Line Numbers Missing
    - Verify ``enable_line_numbers`` in configuration
    - Check component YAML settings
    - Ensure CSS is properly loaded

Copy Button Not Working
    - Check if clipboard API is available
    - Verify JavaScript is loaded
    - Check browser console for errors

Theme Not Applied
    - Verify theme name in configuration
    - Check CSS generator output
    - Ensure theme colors are defined

Performance Issues
    - Check if build cache is enabled
    - Verify minification settings
    - Profile build process

Future Enhancements
-------------------

Planned Features
~~~~~~~~~~~~~~~~

- Custom syntax highlighting themes
- Code block annotations
- Inline code highlighting
- Multi-file code blocks
- Diff highlighting
- Code folding
- Line wrapping options
- Font customization
- Copy as HTML/Markdown
- Code block sharing