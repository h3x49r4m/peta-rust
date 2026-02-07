UNIFIED MATH FORMULA RENDERING PIPELINE DESIGN
=============================================

This document presents the design for unifying the math formula rendering pipeline in the Peta Rust static site generator, moving from a split Rust/Theme architecture to a unified Rust-based system similar to the successful code block pipeline.

**Note:** This pipeline now supports both official RST math syntax (``.. math::`` directive and ``:math:`` role) and legacy LaTeX-style syntax (``$$...$$`` and ``$...$``) for backward compatibility.

Problem Analysis
----------------

CURRENT ISSUE: Math formulas render on article/snippet/project pages but NOT on book pages

ROOT CAUSE: Inconsistent math script injection across templates

• article.html:  ✅ Injects math_render_script via {% block scripts %}
• snippet.html:  ✅ Injects via base.html snippet modal JavaScript
• project.html:  ✅ Injects via base.html snippet modal JavaScript
• book.html:     ❌ NO {% block scripts %} - relies on base.html only
                  ❌ No math_render_script injection

CURRENT ARCHITECTURE:

┌───────────────────────────────────────────────────────────────────────────┐
│  SPLIT RESPONSIBILITY (PROBLEMATIC)                                        │
│  ───────────────────────────────────────────────────────────────────────  │
│  • Rust:          Generate HTML with data-latex attributes                │
│  • Rust:          Generate on-demand JavaScript (per page)                │
│  • Template:      Manually inject math_render_script in some templates     │
│  • Theme JS:      Manual KaTeX loading in snippet modal                    │
│  • Theme CSS:     Manual math styling                                      │
└───────────────────────────────────────────────────────────────────────────┘

ISSUES:
  - ❌ Inconsistent rendering across page types
  - ❌ Multiple points of failure
  - ❌ Duplicate KaTeX loading code
  - ❌ No unified styling system
  - ❌ Manual script injection required
  - ❌ Hard to maintain and test


Proposed Solution: Unified Architecture
----------------------------------------

NEW ARCHITECTURE:

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

BENEFITS:
  - ✅ Single source of truth in Rust
  - ✅ Consistent rendering across ALL page types
  - ✅ Pre-rendered HTML at build time
  - ✅ No manual script injection
  - ✅ Unified styling system
  - ✅ Better testability


Component Architecture
----------------------

FILE STRUCTURE:

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


Pipeline Stages
---------------

STAGE 1: DETECTION & EXTRACTION
────────────────────────────────────────────────────────────────────────────
MathProcessor::extract_math_blocks()
  • Scan content for official RST syntax:
    - ``.. math::`` directive (display math with optional :label:)
    - ``:math:`...``` role (inline math)
  • Scan content for legacy LaTeX syntax:
    - ``$$...$$`` (display math)
    - ``$...$`` (inline math)
    - ``\[...\]`` (display math)
    - ``\(...\)`` (inline math)
  • Return MathDetectionResult {has_formulas, formula_count, math_blocks}
  • Cache detection results for performance

STAGE 2: HTML GENERATION
────────────────────────────────────────────────────────────────────────────
MathRenderer::render()
  • Transform LaTeX to HTML with data-latex attributes
  • Generate self-contained math elements
  • Support both inline and display modes
  • Support labeled equations (via :label: option)
  • Return complete HTML structure

OFFICIAL RST SYNTAX EXAMPLES:

Display math with directive:
  .. math::
     :label: eq1
     
     E = mc^2

  Output:
  <div class="math-display" data-label="eq1" data-latex="E = mc^2"></div>

Inline math with role:
  The energy is :math:`E = mc^2`.

  Output:
  The energy is <span class="math-inline" data-latex="E = mc^2"></span>.

LEGACY SYNTAX EXAMPLES:

Display math with $$:
  $$E = mc^2$$

  Output:
  <div class="math-display" data-latex="E = mc^2"></div>

Inline math with $:
  The energy is $E = mc^2$.

  Output:
  The energy is <span class="math-inline" data-latex="E = mc^2"></span>.

RST MATH SYNTAX SUPPORT
────────────────────────────────────────────────────────────────────────────

The math rendering pipeline supports both official RST math syntax and legacy LaTeX-style syntax for maximum compatibility and standards compliance.

OFFICIAL RST SYNTAX (RECOMMENDED):

Display Math Directive:
  .. math::
     :label: optional_label
     
     \int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}

Features:
  - Official reStructuredText/Docutils syntax
  - Optional :label: for equation references
  - Automatic dedentation of multi-line equations
  - Consistent with Sphinx and other RST tools

Inline Math Role:
  The Gaussian integral is :math:`\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}`.

Features:
  - Official reStructuredText inline role
  - Seamless integration with paragraph text
  - No extra delimiters needed

LEGACY LATEX SYNTAX (BACKWARD COMPATIBILITY):

Display Math:
  $$\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}$$
  or
  \[\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}\]

Inline Math:
  The value is $x^2$ or \(x^2\).

Features:
  - Standard LaTeX syntax
  - Automatic detection and rendering
  - Useful for migrating from LaTeX documents

SYNTAX PREFERENCE:
  • New content: Use official RST syntax (``.. math::`` and ``:math:``)
  • Legacy content: Legacy LaTeX syntax still supported
  • Mixed usage: Both syntaxes can be used in the same document

PROCESSING ORDER:
  1. Process ``.. math::`` directives (RST block syntax)
  2. Process ``:math:`...``` roles (RST inline syntax)
  3. Convert RST markup to HTML
  4. Process legacy LaTeX syntax (``$$``, ``$``, ``\[``, ``\(``)

This ensures that official RST syntax is prioritized while maintaining full backward compatibility.

STAGE 3: CSS GENERATION
────────────────────────────────────────────────────────────────────────────
MathCssGenerator::generate()
  • Generate complete styling for math elements
  • Support multiple themes (light/dark)
  • Responsive design
  • Print-friendly styles

OUTPUT: math-formulas.css (generated at build time)
  ┌──────────────────────────────────────────────────────────────────────┐
  │  /* Math Formula Styles - Generated by Peta */                        │
  │  .math-display { ... }                                                │
  │  .math-inline { ... }                                                 │
  │  .katex { ... }                                                       │
  │  /* Theme variables */                                                 │
  │  /* Responsive rules */                                                │
  │  /* Print styles */                                                    │
  └──────────────────────────────────────────────────────────────────────┘

STAGE 4: JS GENERATION
────────────────────────────────────────────────────────────────────────────
MathJsGenerator::generate()
  • Generate minimal JavaScript for KaTeX rendering
  • On-demand loading
  • Auto-render on page load
  • Modal support

OUTPUT: math-formulas.js (generated at build time)
  ┌──────────────────────────────────────────────────────────────────────┐
  │  /** Math Rendering - Generated by Peta */                            │
  │  (function() {                                                        │
  │      if (typeof window.petaMathLoaded === 'undefined') {              │
  │          window.petaMathLoaded = false;                               │
  │          function loadKaTeX() { ... }                                 │
  │          function renderMath() { ... }                                │
  │          // Auto-render on DOM ready                                  │
  │      }                                                                 │
  │  })();                                                                 │
  └──────────────────────────────────────────────────────────────────────┘

STAGE 5: ASSET PIPELINE
────────────────────────────────────────────────────────────────────────────
AssetPipeline::build()
  • Generate math-formulas.css
  • Generate math-formulas.js
  • Write to _out/dist/assets/
  • Cache for performance

STAGE 6: TEMPLATE INTEGRATION
────────────────────────────────────────────────────────────────────────────
base.html (single point of reference):
  ┌──────────────────────────────────────────────────────────────────────┐
  │  <head>                                                               │
  │      <!-- Math Formula Styles (generated from Rust) -->               │
  │      <link rel="stylesheet" href="/assets/css/math-formulas.css">     │
  │  </head>                                                              │
  │  <body>                                                               │
  │      <!-- ... content ... -->                                         │
  │      <!-- Math Formula Scripts (generated from Rust) -->              │
  │      <script src="/assets/js/math-formulas.js"></script>             │
  │  </body>                                                              │
  └──────────────────────────────────────────────────────────────────────┘

ALL OTHER TEMPLATES:
  • article.html - REMOVE {% block scripts %} for math
  • book.html - NO changes needed
  • snippet.html - NO changes needed
  • project.html - NO changes needed
  • ALL templates get math rendering automatically


Configuration (peta.toml)
--------------------------

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


Implementation Steps
--------------------

STEP 1: Create MathCssGenerator
  • File: peta/src/content/rst/math_css_generator.rs
  • Generate complete CSS for math elements
  • Support themes and responsive design

STEP 2: Create MathJsGenerator
  • File: peta/src/content/rst/math_js_generator.rs
  • Generate minimal JavaScript for KaTeX
  • Remove per-page script generation

STEP 3: Refactor MathRenderer
  • Remove generate_on_demand_script()
  • Simplify to HTML generation only
  • Keep caching for performance
  • Make render_equation() public for directive handlers

STEP 3.5: Add RST Math Directive Handler
  • Create MathDirectiveHandler for ``.. math::`` directive
  • Support :label: option for equation references
  • Handle multi-line equations with proper dedentation
  • Generate HTML with data-label attribute when label provided

STEP 4: Enhance MathProcessor
  • Improve detection logic
  • Add caching
  • Better error handling

STEP 5: Update AssetPipeline
  • Add generate_math_assets()
  • Generate math-formulas.css
  • Generate math-formulas.js

STEP 6: Update Parser
  • Remove math_render_script from RstContent
  • Keep has_math_formulas flag (for optimization)
  • Add process_roles() method for inline ``:math:`...`` syntax
  • Update processing pipeline order:
    1. process_directives() - handles ``.. math::`` blocks
    2. process_roles() - handles ``:math:`...`` inline
    3. convert_rst_to_html() - converts RST markup
    4. math_renderer.render() - handles legacy LaTeX syntax
  • Add HTML block tracking to prevent header detection inside math content

STEP 7: Update Templates
  • base.html: Add math asset references
  • article.html: Remove {% block scripts %} for math
  • snippet.html: Remove manual math rendering code
  • project.html: Remove manual math rendering code
  • book.html: NO changes needed

STEP 8: Update Configuration
  • Add [math_rendering] section to peta.toml
  • Support KaTeX configuration
  • CSS and JS configuration options

STEP 9: Remove Old Code
  • Remove duplicate KaTeX loading from snippet modal
  • Remove math_render_script injection from article.html
  • Remove any theme-specific math files

STEP 10: Test
  • Test math rendering on ALL page types
  • Verify book pages now render math correctly
  • Test modal support
  • Performance testing


Key Benefits
------------

1. CONSISTENCY
   ✅ Single rendering pipeline for ALL page types
   ✅ No more manual script injection
   ✅ Unified styling across the entire site
   ✅ Official RST syntax support for standards compliance

2. PERFORMANCE
   ✅ Pre-generated CSS and JS at build time
   ✅ On-demand KaTeX loading (only when needed)
   ✅ Caching of math detection and rendering

3. MAINTAINABILITY
   ✅ Single source of truth in Rust
   ✅ Type-safe configuration
   ✅ Easy to test and debug

4. FLEXIBILITY
   ✅ Theme support
   ✅ Configurable KaTeX version
   ✅ Easy to add new features
   ✅ Support for both RST and LaTeX syntax

5. FUTURE-PROOF
   ✅ Easy to switch to MathJax if needed
   ✅ Extensible architecture
   ✅ Clean separation of concerns
   ✅ Full backward compatibility with legacy math syntax


Migration Path
--------------

PHASE 1: Create Generators (Build new system)
  • Create MathCssGenerator
  • Create MathJsGenerator
  • Add unit tests

PHASE 2: Integrate Pipeline (Wire it together)
  • Update AssetPipeline
  • Generate assets during build
  • Verify output files

PHASE 3: Update Templates (Switch to new system)
  • Update base.html
  • Clean up article.html
  • Clean up snippet.html
  • Clean up project.html

PHASE 4: Test & Verify (Ensure it works)
  • Test all page types
  • Verify book pages render math
  • Performance testing
  • Browser testing

PHASE 5: Remove Old Code (Clean up)
  • Remove old math rendering code
  • Remove duplicate KaTeX loading
  • Update documentation


Comparison: OLD vs NEW
----------------------

OLD SYSTEM (CURRENT):
┌───────────────────────────────────────────────────────────────────────────┐
│  ❌ Inconsistent script injection                                         │
│  ❌ Multiple KaTeX loading paths                                          │
│  ❌ Math works on some pages, fails on others                             │
│  ❌ Manual template maintenance                                           │
│  ❌ Duplicate code in snippet modal                                        │
│  ❌ No unified styling system                                             │
└───────────────────────────────────────────────────────────────────────────┘

NEW SYSTEM (PROPOSED):
┌───────────────────────────────────────────────────────────────────────────┐
│  ✅ Single asset reference in base.html                                   │
│  ✅ Unified KaTeX loading                                                 │
│  ✅ Math works on ALL pages consistently                                  │
│  ✅ No manual template maintenance for math                               │
│  ✅ Clean, generated CSS and JS                                           │
│  ✅ Unified, themeable styling system                                     │
│  ✅ Same architecture as code blocks (proven)                             │
└───────────────────────────────────────────────────────────────────────────┘


Conclusion
----------

This unified math formula rendering pipeline design follows the proven architecture of the code block system. It will:

- Fix the book page math rendering issue permanently
- Provide consistent rendering across all page types
- Improve performance with pre-generated assets
- Simplify maintenance with a single source of truth
- Enable easy customization through configuration
- Support official RST math syntax for standards compliance
- Maintain full backward compatibility with legacy LaTeX syntax

The implementation provides users with the flexibility to choose between official RST syntax (recommended for new content) and legacy LaTeX syntax (for backward compatibility), both rendered through a unified, efficient pipeline.

For detailed implementation steps with code examples, see math_formulas_pipeline_step_by_step.rst.

References
----------

- Code Block Pipeline: docs/features/codeblocks/codeblock_pipeline_step_by_step.rst
- Math Implementation Guide: docs/features/math_formulas/math_formulas_pipeline_step_by_step.rst
- KaTeX Documentation: https://katex.org/docs/
- reStructuredText Directives: https://docutils.sourceforge.io/docs/ref/rst/directives.html#math
- Peta Source Code: https://github.com/h3x49r4m/peta-rust