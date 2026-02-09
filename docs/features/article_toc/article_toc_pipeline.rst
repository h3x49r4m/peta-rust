Article TOC Pipeline
==================

Overview
--------

The Article TOC (Table of Contents) pipeline automatically generates hierarchical navigation for article pages based on the document structure. It extracts headers and snippet cards from the rendered HTML and builds a properly nested TOC that reflects the actual document hierarchy.

Key Features
------------

* **Automatic Hierarchy Detection**: Automatically detects header levels (h1-h6) and builds hierarchical structure
* **Snippet Card Integration**: Includes snippet cards in the TOC with proper nesting
* **Position-Based Nesting**: Snippets are nested under headers based on their position in the document
* **Semantic Filtering**: Automatically excludes snippet internal headers and metadata headers

How It Works
------------

Pipeline Steps
~~~~~~~~~~~~~~~

1. **Parse RST Content**
   
   The RST parser processes the article content, including:
   
   * Extract frontmatter metadata
   * Merge article parts (for multi-file articles using `article-parts` directive)
   * Process all directives (snippet-card, math, diagrams, etc.)
   * Generate HTML output

2. **Generate HTML Output**
   
   The parser produces HTML with properly marked-up headers and snippet cards:
   
   * Headers have `id` attributes for anchor linking
   * Snippet cards have `embedded-snippet-title` elements for identification
   * Snippet internal headers are scoped with snippet IDs

3. **Extract TOC from HTML**
   
   The TOC generator analyzes the HTML to build the table of contents:

   **Header Extraction**:
   
   * Find all `<h1>` through `<h6>` elements with `id` attributes
   * Extract title text, level, and anchor ID
   * Filter out snippet internal headers (IDs starting with snippet_id-)
   * Skip metadata headers (Table of Contents, Articles, Tags, etc.)

   **Snippet Card Extraction**:
   
   * Find all `<h4 class="embedded-snippet-title">` elements
   * Extract snippet title
   * Generate snippet ID using the same algorithm as the renderer
   * Format snippet entry as "Snippet: <title>"

4. **Build Hierarchical Structure**
   
   **Header Hierarchy**:
   
   * Process headers in document order
   * Use a stack to track current nesting level
   * For each header:
     - Pop stack items with level >= current level
     - If stack has items, add as child of last item
     - Otherwise, add as top-level and push to stack

   **Snippet Nesting**:
   
   * Track the most recent header (last_header_idx)
   * For each snippet:
     - If last_header_idx exists, nest snippet under that header
     - Otherwise, add snippet as top-level
   * This ensures snippets belong to the section they appear in

5. **Render TOC HTML**
   
   The TOC generator converts the hierarchical structure to HTML:

   * Top-level items use `<ul class="toc-list">`
   * Nested items use `<ul class="toc-sublist">`
   * Each item has CSS class `toc-level-{level}` for styling
   * Anchors link to the corresponding `#{id}` in the content

Example
-------

Input RST (quantum-mechanics.rst):

.. code-block:: rst

    .. snippet-card:: the-wave-function

    The Wave Function
    -----------------

    .. snippet-card:: uncertainty-principle

    Uncertainty Principle
    --------------------

    Applications
    ------------

    .. snippet-card:: quantum-computing-applications

Rendered HTML Structure:

.. code-block:: html

    <h4 class="embedded-snippet-title">The Wave Function</h4>
    <h3 id="the-wave-function">The Wave Function</h3>
    <h4 class="embedded-snippet-title">Uncertainty Principle</h4>
    <h3 id="uncertainty-principle">Uncertainty Principle</h3>
    <h3 id="applications">Applications</h3>
    <h4 class="embedded-snippet-title">Quantum Computing Applications</h4>

Generated TOC Structure:

.. code-block:: html

    <ul class="toc-list">
      <li class="toc-item toc-level-1">
        <a href="#snippet-the-wave-function">Snippet: The Wave Function</a>
      </li>
      <li class="toc-item toc-level-3">
        <a href="#the-wave-function">The Wave Function</a>
        <ul class="toc-sublist">
          <li class="toc-item toc-level-1">
            <a href="#snippet-uncertainty-principle">Snippet: Uncertainty Principle</a>
          </li>
        </ul>
      </li>
      <li class="toc-item toc-level-3">
        <a href="#uncertainty-principle">Uncertainty Principle</a>
      </li>
      <li class="toc-item toc-level-3">
        <a href="#applications">Applications</a>
        <ul class="toc-sublist">
          <li class="toc-item toc-level-1">
            <a href="#snippet-quantum-computing-applications">Snippet: Quantum Computing Applications</a>
          </li>
        </ul>
      </li>
    </ul>

Key Behaviors
-------------

Snippet Card Nesting Rules
~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Snippets appearing **before** the first header are top-level
* Snippets appearing **between** two headers are nested under the first header
* Snippets appearing **after** the last header are nested under that header
* This is based purely on document position, not title matching

Header Hierarchy Rules
~~~~~~~~~~~~~~~~~~~~~~~

* Headers with lower levels (h1, h2) can contain higher-level headers (h3, h4)
* Headers with same or higher levels are siblings, not nested
* The stack-based algorithm ensures correct nesting

Multi-File Articles
~~~~~~~~~~~~~~~~~~~

For articles using the `article-parts` directive:

* Part files are merged before TOC generation
* Headers from all parts are included in the TOC
* The hierarchical structure reflects the merged document
* Snippet cards from all parts are correctly nested

Code Location
-------------

* **Parser**: `peta/src/content/rst/parser.rs`
* **TOC Generator**: `peta/src/content/rst/toc_generator.rs`
* **Article TOC Generator**: `peta/src/content/rst/article_toc_generator.rs` (for RST-based parsing)

The article TOC is generated in the `parse_with_type_and_path` method of `RstParser` when `content_type == ContentType::Article`.