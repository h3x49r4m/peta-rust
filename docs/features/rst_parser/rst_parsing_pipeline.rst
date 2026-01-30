RST Parsing Pipeline
==================

Overview
--------

The RST (reStructuredText) parsing pipeline is responsible for converting RST markup content into HTML for rendering in the Peta static site generator. The parser supports a wide range of RST features including frontmatter, directives, math formulas, tables, lists, code blocks, and semantic markup.

Architecture
------------

The RST parser follows a multi-stage pipeline architecture:

.. code-block:: text

    RST File (.rst)
      ↓
    RstParser::parse()
      ↓
    ┌─────────────────────────────────────┐
    │ Stage 1: Extract Frontmatter        │
    │   - Parse YAML frontmatter          │
    │   - Extract metadata               │
    │   - Return content without FM      │
    └─────────────────────────────────────┘
      ↓
    ┌─────────────────────────────────────┐
    │ Stage 2: Process Directives       │
    │   - .. code-block::                │
    │   - .. snippet-card::              │
    │   - .. toctree::                  │
    │   - Custom directives              │
    └─────────────────────────────────────┘
      ↓
    ┌─────────────────────────────────────┐
    │ Stage 3: Convert RST Markup       │
    │   - Tables                         │
    │   - Headers                        │
    │   - Emphasis                       │
    │   - Links                          │
    │   - Lists                          │
    │   - Paragraphs                     │
    └─────────────────────────────────────┘
      ↓
    ┌─────────────────────────────────────┐
    │ Stage 4: Render Math Formulas     │
    │   - Inline math $...$             │
    │   - Display math $$...$$          │
    │   - KaTeX integration              │
    └─────────────────────────────────────┘
      ↓
    ┌─────────────────────────────────────┐
    │ Stage 5: Generate TOC              │
    │   - Articles/Projects              │
    │   - Books (from toctree)          │
    │   - Embedded snippet cards        │
    └─────────────────────────────────────┘
      ↓
    ┌─────────────────────────────────────┐
    │ Stage 6: Final Assembly           │
    │   - RstContent struct              │
    │   - HTML + Metadata + TOC          │
    │   - Math detection stats           │
    └─────────────────────────────────────┘

Core Components
---------------

RstParser
~~~~~~~~~~

Main parser struct that orchestrates all parsing stages:

.. code-block:: rust

    pub struct RstParser {
        math_renderer: MathRenderer,
        math_processor: MathProcessor,
        code_highlighter: CodeHighlighter,
        directive_handlers: HashMap<String, Box<dyn DirectiveHandler>>,
        toc_generator: TocGenerator,
    }

Key Methods
^^^^^^^^^^^

- ``parse()`` - Parse RST content to HTML
- ``parse_with_type()`` - Parse with content type override
- ``parse_with_type_and_path()`` - Parse with type and file path
- ``extract_frontmatter()`` - Extract YAML frontmatter
- ``process_rst_content()`` - Process all RST markup
- ``process_directives()`` - Handle custom directives
- ``convert_rst_to_html()`` - Convert RST markup to HTML
- ``convert_tables()`` - Convert RST tables to HTML
- ``convert_headers()`` - Convert headers to HTML
- ``convert_emphasis()`` - Convert bold/italic/code
- ``convert_links()`` - Convert link syntax
- ``convert_lists()`` - Convert ordered/unordered lists
- ``convert_paragraphs()`` - Wrap text in paragraphs

Pipeline Stages
===============

Stage 1: Frontmatter Extraction
------------------------------

Purpose
````````

Extract YAML frontmatter for metadata configuration.

Format
^^^^^^

RST files can optionally begin with YAML frontmatter:

.. code-block:: rst

    ---
    title: My Article
    date: 2026-01-30
    tags:
      - tutorial
      - rust
    author: John Doe
    ---
    
    Article content here...

Extraction Process
^^^^^^^^^^^^^^^^^

1. Check for ``---\n`` at start of content
2. Find closing ``\n---\n`` delimiter
3. Parse YAML between delimiters
4. Extract metadata using serde_yaml
5. Return both frontmatter and RST content

Stage 2: Directive Processing
------------------------------

Purpose
````````

Process RST directives for special content blocks.

Supported Directives
^^^^^^^^^^^^^^^^^^^^^^

.. list-table:: Supported RST Directives
   :header-rows: 1
   :widths: 25 50 25

   * - Directive
     - Syntax
     - Description
   
   * - ``.. code-block::``
     - ``.. code-block:: python``
     - Syntax-highlighted code blocks
   
   * - ``.. snippet-card::``
     - ``.. snippet-card:: my-snippet``
     - Embedded snippet cards with metadata
   
   * - ``.. toctree::``
     - ``.. toctree:: :maxdepth: 2``
     - Table of contents for book chapters

Processing Flow
^^^^^^^^^^^^^^^

1. Find all directive patterns with regex: ``\.\. ([a-zA-Z0-9_-]+)::``
2. For each directive:
   
   - Extract directive name and optional arguments
   - Parse directive content (indented block)
   - Call appropriate handler
   - Replace directive with rendered output
   - Handle nested directives recursively

Stage 3: RST Markup Conversion
---------------------------------

Purpose
````````

Convert standard RST markup to semantic HTML.

Conversion Order
^^^^^^^^^^^^^^

The conversion must happen in a specific order to ensure correct rendering:

1. **Tables** - Must be converted before other markup to avoid interference
2. **Headers** - Convert underlines and markdown-style headers
3. **Emphasis** - Bold, italic, code
4. **Links** - Reference and inline links
5. **Lists** - Ordered and unordered lists
6. **Paragraphs** - Wrap remaining text

Table Conversion
^^^^^^^^^^^^^^^

Simple pipe tables with optional header rows:

.. code-block:: rst

    | Header 1 | Header 2 | Header 3 |
    |----------|----------|----------|
    | Cell 1   | Cell 2   | Cell 3   |
    | Cell 4   | Cell 5   | Cell 6   |

Features:

- Automatic header detection (row with separator below)
- Support for math formulas in cells
- Inline math support: ``$O(n)$`` and ``O(\sqrt{N})$``
- Table header rendered as ``<thead>`` with ``<th>`` cells
- Table body rendered as ``<tbody>`` with ``<td>`` cells

Header Conversion
^^^^^^^^^^^^^^^

Supports both underline and markdown-style headers:

.. code-block:: rst

    Level 1 Header (with equals signs)
    ==============================
    
    Level 2 Header (with dashes)
    ------------------------------
    
    ### Level 3 Header (markdown-style)

Rules:

- Underline headers require matching underline characters
- ``=`` → ``<h2>``, ``-`` → ``<h3>``, ``~`` → ``<h4>``
- Headers are slugified for anchor links
- ``id`` attribute generated for navigation

Emphasis Conversion
^^^^^^^^^^^^^^^^^^

Supports multiple emphasis styles:

.. code-block:: rst

    **Bold text**
    *Italic text*
    ``Inline code``
    
    :sub:`subscript`
    :sup:`superscript`

Output:

.. code-block:: html

    <strong>Bold text</strong>
    <em>Italic text</em>
    <code>Inline code</code>
    <sub>subscript</sub>
    <sup>superscript</sup>

Link Conversion
^^^^^^^^^^^^^^^

Supports reference-style links:

.. code-block:: rst

    `Link text <https://example.com>`_
    
    .. _reference-label:
    
    `Reference link`_

Output:

.. code-block:: html

    <a href="https://example.com">Link text</a>
    <a href="#reference-label">Reference link</a>

List Conversion
^^^^^^^^^^^^^^^

Supports ordered and unordered lists with nesting:

.. code-block:: rst

    1. First item
    2. Second item
       - Nested unordered item
       - Another nested item
    3. Third item
    
    - Unordered item
    - Another unordered item

Output:

.. code-block:: html

    <ol>
      <li>First item</li>
      <li>Second item
        <ul>
          <li>Nested unordered item</li>
          <li>Another nested item</li>
        </ul>
      </li>
      <li>Third item</li>
    </ol>
    <ul>
      <li>Unordered item</li>
      <li>Another unordered item</li>
    </ul>

Paragraph Conversion
^^^^^^^^^^^^^^^^^^^

Wraps non-markup text in paragraph tags:

- Skips HTML tags (already formatted)
- Skips empty lines
- Wraps consecutive text lines
- Handles indentation for code blocks

Stage 4: Math Formula Rendering
---------------------------------

Purpose
````````

Render mathematical formulas using KaTeX.

Supported Syntax
^^^^^^^^^^^^^^^

Inline Math
    Single dollar signs: ``$x^2 + y^2 = z^2$``

Display Math
    Double dollar signs: ``$$\int_0^1 x dx = \frac{1}{2}$$``

Math Blocks
    RST math directives (legacy support)

Rendering Process
^^^^^^^^^^^^^^^^

1. Detect math formulas using regex patterns
2. Convert to KaTeX-compatible format
3. Add ``data-latex`` attribute for rendering
4. Generate HTML structure with appropriate classes:

.. code-block:: html

    <span class="math-inline" data-latex="x^2 + y^2 = z^2">
    </span>
    
    <div class="math-display" data-latex="\int_0^1 x dx = \frac{1}{2}">
    </div>

5. KaTeX renders on page load in the browser

Features
^^^^^^^^

- Auto-detection of math content
- Counting of formula statistics
- Support for all common LaTeX math commands
- Integration with other content (tables, lists, etc.)

Stage 5: Table of Contents Generation
-------------------------------------

Purpose
````````

Generate navigation structure from content headers.

Content Type Differences
^^^^^^^^^^^^^^^^^^^^^^^^^^^

Articles and Projects
    Uses enhanced TOC generator that includes:
    
    - Standard headers
    - Embedded snippet cards
    - Proper section hierarchy

Books
    Extracts TOC from toctree directives:
    
    - Chapter structure
    - Multi-level navigation
    - Book-specific organization

Snippets
    Simple TOC (optional snippet cards)

TOC Generation Process
^^^^^^^^^^^^^^^^^^^^^^

1. Parse HTML for heading elements (h2, h3, h4, etc.)
2. Extract heading text and anchor IDs
3. Build hierarchical structure
4. Render as navigation HTML
5. Include in template for display

Output Structure
^^^^^^^^^^^^^^^

.. code-block:: html

    <nav class="toc">
      <ul class="toc-list">
        <li class="toc-item">
          <a href="#section-1" class="toc-link">Section 1</a>
          <ul class="toc-sublist">
            <li class="toc-item">
              <a href="#section-1-1" class="toc-link">Section 1.1</a>
            </li>
          </ul>
        </li>
      </ul>
    </nav>

Stage 6: Final Assembly
----------------------

Output Structure
^^^^^^^^^^^^^^^

The parser returns a ``RstContent`` struct:

.. code-block:: rust

    pub struct RstContent {
        pub metadata: ContentMetadata,
        pub html: String,
        pub toc: Vec<TocEntry>,
        pub toc_html: String,
        pub frontmatter: HashMap<String, serde_json::Value>,
        pub has_math_formulas: bool,
        pub math_formula_count: usize,
    }

Fields:

- ``metadata`` - Content metadata (title, date, tags, etc.)
- ``html`` - Rendered HTML content
- ``toc`` - Structured TOC entries for programmatic use
- ``toc_html`` - Rendered TOC HTML for display
- ``frontmatter`` - Raw frontmatter data
- ``has_math_formulas`` - Whether page contains math
- ``math_formula_count`` - Number of math formulas

Usage Example
=============

Basic Usage
~~~~~~~~~~

.. code-block:: rust

    use peta::content::rst::RstParser;

    let mut parser = RstParser::new()?;
    
    let rst_content = r#"
    ---
    title: My Article
    ---
    
    Introduction
    =============
    
    This is **bold** and *italic* text.
    
    $$ E = mc^2 $$
    "#;
    
    let content = parser.parse(rst_content)?;
    
    println!("Title: {}", content.metadata.title);
    println!("HTML: {}", content.html);
    println!("TOC: {}", content.toc_html);

With Content Type Override
~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: rust

    let content = parser.parse_with_type(
        rst_content,
        Some(ContentType::Article)
    )?;

With File Path
~~~~~~~~~~~~~~~~

.. code-block:: rust

    use std::path::Path;
    
    let content = parser.parse_with_type_and_path(
        rst_content,
        Some(ContentType::Article),
        Some(Path::new("content/article.rst"))
    )?;

Supported Features
===================

RST Syntax
~~~~~~~~~~

Headers
    - Underline style: ``Title`` followed by ``====``
    - Markdown style: ``# Title``, ``## Subtitle``
    - Automatic anchor generation

Text Formatting
    - Bold: ``**text**``
    - Italic: ``*text*``
    - Monospace: ````text````
    - Line breaks: ``|  (escaped vertical bar)
    - Block quotes: ``.. note::`` (limited support)

Links
    - External: ```Link text <url>`_```
    - Internal: ```Link text <page.html>`_```
    - Anchors: ```.. _anchor-name:```

Lists
    - Ordered: ``1.``, ``2.``, ``3.``
    - Unordered: ``-``, ``*``
    - Nested lists (indented)
    - Definition lists (limited support)

Code Blocks
    - Via directive: ``.. code-block:: python``
    - Inline code: ````code````
    - Syntax highlighting via syntect

Tables
    - Simple pipe tables: ``| Header |```
    - Row separators: ``|--------|``
    - Header detection
    - Math formulas in cells

Math
    - Inline: ``$formula$``
    - Display: ``$$formula$$``
    - KaTeX rendering
    - LaTeX command support

Directives
    - ``.. code-block::`` - Syntax-highlighted code
    - ``.. snippet-card::`` - Embedded snippets
    - ``.. toctree::`` - Book TOC
    - Custom directives extensible

Frontmatter
    - YAML format
    - Metadata extraction
    - Type and author fields
    - Tags and categories

Extensions
~~~~~~~~~~

The parser includes several extensions beyond standard RST:

Embedded Snippet Cards
    - Special directive for cross-referencing snippets
    - Automatic metadata inclusion
    - Responsive card rendering

Math Formula Auto-Detection
    - Automatic LaTeX detection
    - Performance optimization
    - Skip rendering if no math present

TOC with Embedded Cards
    - Enhanced table of contents
    - Includes snippet cards in navigation
    - Better content discovery

Limitations
============

Not Currently Supported
~~~~~~~~~~~~~~~~~~~~~~~~~

- Grid tables (CSV-style)
- Complex list types (definition lists, option lists)
- Substitutions and roles (``|replace|``)
- Admonitions (``.. note::``, ``.. warning::``)
- Citations and footnotes
- Transitions and separators
- Raw HTML in RST (partially supported)
- Field lists and option lists
- Sidebar and margin notes

Known Issues
~~~~~~~~~~~~

- Complex nested lists may not render perfectly
- Mixed list types in single block
- Table row spanning/colspan not supported
- Very large tables may impact build performance
- Unicode characters in URLs may need encoding

Troubleshooting
===============

Common Issues
~~~~~~~~~~~~~~

Headers Not Rendering
    - Check underline length matches title length
    - Verify no extra spaces in underline
    - Ensure proper RST syntax

Tables Not Rendering
    - Verify pipe syntax: ``| Header |``
    - Check separator line: ``|--------|``
    - Ensure proper cell alignment
    - Check for empty cells or missing pipes

Math Not Rendering
    - Verify KaTeX is loaded in template
    - Check ``data-latex`` attributes are present
    - Ensure proper LaTeX syntax
    - Check browser console for errors

Links Not Working
    - Verify URL format: ```text <url>`_```
    - Check for missing backticks
    - Ensure proper reference definition
    - Check for broken links in output

TOC Not Generating
    - Verify content has headers
    - Check content type is correct
    - Ensure proper heading hierarchy
    - Check TOC generator logic

Performance Tips
~~~~~~~~~~~~~~~~

- Enable build caching for faster rebuilds
- Use math formula detection to skip rendering
- Optimize large tables with fewer rows
- Reduce nested list depth where possible
- Use simple markup over complex directives

Best Practices
==============

Writing RST Content
~~~~~~~~~~~~~~~~~~

1. **Start with frontmatter** - Always include metadata
2. **Use consistent headers** - Choose one style and stick to it
3. **Test tables** - Verify pipe alignment and separators
4. **Check math syntax** - Validate LaTeX formulas
5. **Preview output** - Build and check rendered HTML
6. **Keep it simple** - Avoid overly complex structures

Content Organization
~~~~~~~~~~~~~~~~~~~

1. **Logical structure** - Use clear heading hierarchy
2. **Short paragraphs** - One idea per paragraph
3. **Meaningful links** - Descriptive link text
4. **Accurate code blocks** - Test code before including
5. **Consistent style** - Follow project conventions

File Organization
~~~~~~~~~~~~~~~~~

1. **Use clear filenames** - Descriptive and lowercase
2. **Include index files** - For directory contents
3. **Add frontmatter** - For all content files
4. **Test locally** - Build before committing
5. **Use paths** - Correct relative paths for links

Future Enhancements
===================

Planned Features
~~~~~~~~~~~~~~~~

- Grid table support (CSV-style)
- Complex list types (definition lists, option lists)
- Enhanced directive system
- Custom directives plugin API
- Better error messages
- Performance profiling
- Caching layer
- Parallel processing
- Streaming parser for large files

Integration
-----------

The RST parser integrates with:

- **Builder**: ``SiteBuilder`` uses parser for content generation
- **Template Engine**: Parser output injected into templates
- **Asset Pipeline**: CSS/JS generation for enhanced features
- **Server**: Dev server uses parser for live reloading
- **CLI**: Build commands use parser for site generation

Additional Resources
====================

- `peta/src/content/rst/parser.rs` - Main parser implementation
- `peta/src/content/rst/directives.rs` - Directive handlers
- `peta/src/content/rst/toc_generator.rs` - TOC generation
- `peta/src/content/metadata.rs` - Metadata extraction
- `docs/features/codeblocks/codeblock_pipeline.rst` - Code block rendering
- `docs/features/math_formulas/math_formulas_pipeline.rst` - Math rendering
- `docs/features/embedded_snippet_cards/embedded_snippet_cards_pipeline.rst` - Snippet cards