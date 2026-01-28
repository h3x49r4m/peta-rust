Embedded Snippet Cards Pipeline
================================

Overview
--------

The embedded snippet cards feature allows you to render full snippet content inline within RST files using a simple directive syntax. This enables you to reference and display snippets directly in articles, projects, or books without requiring users to navigate away from the current page.

.. image:: ../../images/embedded-snippet-card.png
   :alt: Embedded snippet card example
   :width: 800

Key Features
------------

- **Inline Rendering**: Display full snippet content directly within RST files
- **Automatic Resolution**: Snippets are automatically resolved by ID
- **Metadata Display**: Shows title, tags, and date of the referenced snippet
- **Heading Hierarchy Adjustment**: Automatically adjusts heading levels to avoid conflicts
- **Site-wide Support**: Works across articles, projects, and books
- **Responsive Design**: Styled to match the site's design system

Pipeline Architecture
---------------------

The embedded snippet cards pipeline consists of several components:

1. **Directive Parsing** (``peta/src/content/rst/parser.rs``)
   - Detects ``.. snippet-card::<snippet-id>`` directives
   - Extracts snippet ID from the directive line
   - Generates placeholder HTML with ``data-snippet`` attribute

2. **Content Resolution** (``peta/src/content/resolver.rs``)
   - Builds a snippet index from all available snippets
   - Resolves snippet references by ID
   - Provides snippet data for rendering

3. **Card Rendering** (``peta/src/content/rst/embedded_snippet_cards/``)
   - ``embedded_snippet_card_renderer.rs``: Generates HTML for snippet cards
   - ``config.rs``: Configuration for card appearance and behavior
   - Adjusts heading hierarchy to avoid conflicts with parent content

4. **Asset Generation** (``peta/src/assets/``)
   - ``css_generator.rs``: Generates CSS styles for snippet cards
   - ``js_generator.rs``: Generates JavaScript for interactive features
   - ``pipeline.rs``: Integrates asset generation into build process

5. **Template Integration** (``themes/default/templates/base.html``)
   - Includes CSS and JavaScript assets
   - Applies styles globally across the site

Step-by-Step Process
---------------------

Step 1: Parse RST Directives
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

When parsing RST files, the directive parser identifies ``snippet-card`` directives:

.. code-block:: rst

   .. snippet-card:: derivatives

The parser generates a placeholder HTML element:

.. code-block:: html

   <div class="embedded-snippet-card" data-snippet="derivatives"></div>

**Location**: ``peta/src/content/rst/parser.rs:200-210``

Step 2: Build Snippet Index
~~~~~~~~~~~~~~~~~~~~~~~~~~~

The content resolver scans all snippet files and builds an index:

.. code-block:: rust

   pub fn build_index(&mut self, snippets: &[RstContent]) -> Result<()> {
       for snippet in snippets {
           let snippet_id = Self::generate_snippet_id(&snippet.metadata.title);
           self.snippets.insert(snippet_id, snippet.clone());
       }
       Ok(())
   }

**Location**: ``peta/src/content/resolver.rs:45-55``

Step 3: Resolve Snippet References
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

After all RST files are parsed, the builder resolves snippet references:

.. code-block:: rust

   async fn resolve_references(&mut self) -> Result<()> {
       let mut resolver = ContentResolver::new();
       resolver.build_index(&self.snippet_content)?;
       
       for content in self.rst_content.iter_mut() {
           // Replace placeholders with actual snippet cards
       }
       Ok(())
   }

**Location**: ``peta/src/core/builder.rs:250-280``

Step 4: Render Snippet Cards
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The embedded snippet card renderer generates HTML for each snippet:

.. code-block:: rust

   pub fn render(&self, snippet: &RstContent) -> Result<String> {
       let mut html = String::new();
       html.push_str(r#"<div class="embedded-snippet-card">"#);
       
       // Add header with metadata
       html.push_str(&self.render_header(snippet));
       
       // Add content with adjusted headings
       let adjusted_content = self.adjust_heading_hierarchy(&snippet.html);
       html.push_str(&adjusted_content);
       
       // Add footer with link
       html.push_str(&self.render_footer(snippet));
       
       html.push_str("</div>");
       Ok(html)
   }

**Location**: ``peta/src/content/rst/embedded_snippet_cards/embedded_snippet_card_renderer.rs:30-80``

Step 5: Adjust Heading Hierarchy
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

To avoid conflicts with parent page headings, the renderer adjusts heading levels:

- ``<h1>`` → ``<h3>``
- ``<h2>`` → ``<h4>``
- ``<h3>`` → ``<h5>``
- ``<h4>`` → ``<h6>``
- ``<h5>`` → ``<h6>``
- ``<h6>`` → ``<h6>``

**Location**: ``peta/src/content/rst/embedded_snippet_cards/embedded_snippet_card_renderer.rs:95-115``

Step 6: Generate CSS Assets
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The CSS generator creates styles for snippet cards:

.. code-block:: rust

   pub fn generate_css(&self) -> Result<String> {
       let mut css = String::new();
       css.push_str("/* Embedded Snippet Card Styles */\n\n");
       css.push_str(&self.generate_base_styles());
       css.push_str(&self.generate_header_styles());
       css.push_str(&self.generate_content_styles());
       css.push_str(&self.generate_footer_styles());
       css.push_str(&self.generate_tag_styles());
       Ok(css)
   }

**Location**: ``peta/src/assets/css_generator.rs:730-750``

Step 7: Generate JavaScript Assets
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The JavaScript generator adds interactive features:

.. code-block:: rust

   pub fn generate_js(&self) -> Result<String> {
       let mut js = String::new();
       js.push_str("// Embedded Snippet Card JavaScript\n\n");
       js.push_str("function initEmbeddedSnippetCards() {\n");
       js.push_str("  initCollapse();\n");
       js.push_str("  addModalButtons();\n");
       js.push_str("}\n");
       Ok(js)
   }

**Location**: ``peta/src/assets/js_generator.rs:520-540``

Step 8: Integrate into Build Pipeline
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The asset generation is integrated into the build pipeline:

.. code-block:: rust

   pub async fn generate_all_assets(&mut self) -> Result<()> {
       // ... other assets ...
       
       self.generate_embedded_snippet_card_assets().await?;
       
       Ok(())
   }

**Location**: ``peta/src/assets/pipeline.rs:120-140``

Usage
-----

To embed a snippet card in an RST file:

.. code-block:: rst

   .. snippet-card:: <snippet-id>

Example:

.. code-block:: rst

   Introduction to Derivatives
   ============================
   
   The derivative is a fundamental concept in calculus.
   
   .. snippet-card:: derivatives
   
   Understanding derivatives is essential for many applications.

Configuration
-------------

The embedded snippet card configuration can be customized:

.. code-block:: rust

   pub struct EmbeddedSnippetCardConfig {
       pub border_radius: String,
       pub border_color: String,
       pub background_color: String,
       pub shadow: String,
       pub header_background: String,
       pub header_border: String,
       pub title_color: String,
       pub tag_color: String,
       pub content_padding: String,
       pub content_background: String,
       pub footer_background: String,
       pub footer_border: String,
       pub link_color: String,
       pub show_metadata: bool,
       pub show_footer: bool,
   }

**Location**: ``peta/src/content/rst/embedded_snippet_cards/config.rs``

Styling
-------

The embedded snippet cards are styled to match the site's design system:

- **Color Scheme**: Blue primary color (#3b82f6) with light backgrounds
- **Borders**: Left accent border (4px) with thin borders on other sides
- **Spacing**: 1.5rem padding around content
- **Hover Effects**: Subtle translation and shadow changes
- **Responsive**: Adapts to mobile and tablet screens

**CSS Location**: ``_out/dist/assets/css/embedded-snippet-cards.css``

Troubleshooting
---------------

Snippet Not Found
~~~~~~~~~~~~~~~~~~

If you see "Snippet Not Found" error:

1. Check the snippet ID in the directive matches the snippet's title-based ID
2. Snippet IDs are generated from titles: "My Title" → "my-title"
3. Ensure the snippet file exists in the ``_content/snippets/`` directory

Styles Not Applying
~~~~~~~~~~~~~~~~~~~

If styles don't appear:

1. Verify CSS is linked in ``base.html``: ``/assets/css/embedded-snippet-cards.css``
2. Check the CSS file exists: ``_out/dist/assets/css/embedded-snippet-cards.css``
3. Ensure class names match between HTML and CSS

Headings Conflict
~~~~~~~~~~~~~~~~~~

If headings appear too large or too small:

1. The renderer automatically adjusts heading hierarchy
2. Headings are shifted down by 2 levels (h1→h3, h2→h4, etc.)
3. This prevents conflicts with parent page headings

Related Components
------------------

- **Snippet Card Modal**: For displaying snippets in a modal popup
- **Grid Card**: For displaying content cards in a grid layout
- **Content Resolver**: For resolving snippet references

Files
-----

- ``peta/src/content/rst/embedded_snippet_cards/mod.rs``: Module exports
- ``peta/src/content/rst/embedded_snippet_cards/config.rs``: Configuration
- ``peta/src/content/rst/embedded_snippet_cards/embedded_snippet_card_renderer.rs``: Renderer
- ``peta/src/content/rst/parser.rs``: Directive parsing
- ``peta/src/content/resolver.rs``: Content resolution
- ``peta/src/core/builder.rs``: Build orchestration
- ``peta/src/assets/css_generator.rs``: CSS generation
- ``peta/src/assets/js_generator.rs``: JavaScript generation
- ``peta/src/assets/pipeline.rs``: Asset pipeline
- ``themes/default/templates/base.html``: Template integration