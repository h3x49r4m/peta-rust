RST Tables Pipeline
=====================

This document describes the pipeline for processing reStructuredText (RST) tables in the Peta static site generator.

Overview
--------

The RST tables pipeline processes four types of RST tables and converts them to semantic HTML with interactive features:

1. **Grid Tables** - Tables with explicit borders using ``+`` characters
2. **Simple Tables** - Tables with underlines (``=`` or ``-``) for structure
3. **CSV Table Directive** - Tables using the ``.. csv-table::`` directive
4. **List Table Directive** - Tables using the ``.. list-table::`` directive

Architecture
-----------

The pipeline consists of several key components:

**Parser Layer** (``peta/src/content/rst/tables/``)

- ``mod.rs`` - Module definition and table detection utilities
- ``parser.rs`` - Parses grid and simple tables
- ``directive_parser.rs`` - Parses CSV and list table directives
- ``html_generator.rs`` - Generates semantic HTML from parsed tables

**Integration Layer** (``peta/src/content/rst/``)

- ``directives.rs`` - ``TableDirectiveHandler`` for processing table directives
- ``parser.rs`` - Main RST parser with table detection logic

**Asset Generation** (``peta/src/assets/``)

- ``table_css_generator.rs`` - Generates CSS for table styling
- ``table_js_generator.rs`` - Generates JavaScript for table interactivity

Processing Flow
----------------

1. **Content Parsing**

   The RST parser reads the source file and identifies table markers:

   - Grid tables: Lines starting with ``+`` and containing ``+``
   - Simple tables: Lines with ``=`` or ``-`` separators with multiple groups
   - Directives: Lines starting with ``.. csv-table::`` or ``.. list-table::``

2. **Table Detection**

   The ``TableDetector`` analyzes each line to determine table type:

   .. code-block:: rust

       pub fn detect(line: &str) -> Option<TableType> {
           // Check for directives
           if line.contains(".. csv-table::") {
               return Some(TableType::CsvDirective);
           }
           // Check for grid table
           if line.starts_with('+') && line.contains('+') {
               return Some(TableType::GridTable);
           }
           // Check for simple table (requires multiple groups)
           if is_simple_separator_with_multiple_groups(line) {
               return Some(TableType::SimpleTable);
           }
           None
       }

3. **Table Parsing**

   Each table type has a dedicated parser:

   *Grid Tables* (``parser.rs``)

   - Extracts cells separated by ``|``
   - Identifies header rows using ``===`` separators
   - Handles merged cells (basic support)

   *Simple Tables* (``parser.rs``)

   - Parses columns separated by whitespace
   - Uses ``=`` for header underlines and ``-`` for body separators
   - Detects headers by finding separator lines

   *CSV Tables* (``directive_parser.rs``)

   - Parses directive options (``:header:``, ``:widths:``)
   - Handles quoted values with commas
   - Supports multi-line CSV data

   *List Tables* (``directive_parser.rs``)

   - Parses hierarchical list structure
   - Each row starts with ``*`` or ``-``
   - Each cell within a row starts with ``-`` on a separate line

4. **HTML Generation**

   The ``TableHtmlGenerator`` creates semantic HTML:

   .. code-block:: html

       <div class="rst-table" data-type="grid">
         <div class="table-controls">
           <input type="search" class="table-search" placeholder="Search table...">
           <button class="table-copy" title="Copy table">📋</button>
         </div>
         <div class="table-wrapper">
           <table>
             <thead>
               <tr>
                 <th>Header 1</th>
                 <th>Header 2</th>
               </tr>
             </thead>
             <tbody>
               <tr>
                 <td>Cell 1</td>
                 <td>Cell 2</td>
               </tr>
             </tbody>
           </table>
         </div>
         <div class="table-footer">1 rows × 2 columns</div>
       </div>

5. **Asset Generation**

   CSS and JavaScript assets are generated for table functionality:

   *CSS* (``table_css_generator.rs``)

   - Table styling with borders and spacing
   - Responsive design with scroll indicators
   - Dark mode support
   - Print-friendly styles

   *JavaScript* (``table_js_generator.rs``)

   - Row search and filtering
   - Column sorting (clickable headers)
   - Copy to clipboard functionality
   - Row/column count display

Key Features
------------

**Semantic HTML**

- Proper use of ``<table>``, ``<thead>``, ``<tbody>``, ``<th>``, ``<td>``
- Header association with ``scope`` attributes
- ``data-type`` attribute for table identification

**Accessibility**

- Keyboard navigation support
- Screen reader compatible markup
- ARIA labels for interactive elements
- Semantic table structure

**Interactivity**

- Search functionality to filter rows
- Clickable headers for sorting columns
- Copy button to export table data as CSV
- Row/column count display in footer

**Responsive Design**

- Horizontal scroll for wide tables on mobile
- Touch-friendly controls
- Adaptive column widths

**Theming**

- Dark mode support
- Consistent styling across table types
- Customizable via CSS variables

Table Type Examples
------------------

Grid Table
~~~~~~~~~~

.. code-block:: rst

    +----------+----------+----------+
    | Header 1 | Header 2 | Header 3 |
    +==========+==========+==========+
    | Cell 1   | Cell 2   | Cell 3   |
    +----------+----------+----------+
    | Cell 4   | Cell 5   | Cell 6   |
    +----------+----------+----------+

Simple Table
~~~~~~~~~~~~

.. code-block:: rst

    Name      Age    Occupation
    ========= ====== ============
    Alice     28     Engineer
    Bob       32     Designer
    Charlie   45     Manager

CSV Table Directive
~~~~~~~~~~~~~~~~~~~

.. code-block:: rst

    .. csv-table:: Employee Data
       :header: "Name", "Department", "Salary"
       :widths: 25, 25, 15

       Alice Smith, Engineering, $85,000
       Bob Johnson, Design, $72,000

List Table Directive
~~~~~~~~~~~~~~~~~~~~

.. code-block:: rst

    .. list-table:: Product Comparison
       :header-rows: 1
       :widths: 20, 20, 30, 30

       * - Feature
         - Basic
         - Professional
         - Enterprise
       * - Price
         - Free
         - $29/month
         - $99/month

Detection Rules
---------------

**Simple Table Detection**

To distinguish simple tables from section headings:

1. Header row must have at least 3 columns (words separated by whitespace)
2. Separator line must have at least 2 groups of ``=`` or ``-`` characters
3. This prevents headings like "Title" with "======" from being detected as tables

Example:

.. code-block:: rst

    # This is a heading (1 word, single group)
    Title
    =======

    # This is a table (3 words, 3 groups)
    Column 1  Column 2  Column 3
    ========= ========= =========

Known Limitations
-----------------

**Merged Cells**

Grid tables with merged cells (colspan/rowspan) have basic support. Complex merged cell layouts may not render correctly.

**Empty Cells**

Empty cells in simple tables may be skipped during parsing.

**Nested Tables**

Nested tables (tables within tables) are not supported.

Best Practices
--------------

When choosing a table type:

1. **Grid Tables** - Use for precise layout needs or when you need explicit borders
2. **Simple Tables** - Use for quick, simple tables without complex formatting
3. **CSV Tables** - Use when importing data from CSV files or databases
4. **List Tables** - Use for complex content or when you need multi-line cells

Tips for writing tables:

- Keep tables concise and readable
- Use clear, descriptive headers
- Align numeric columns to the right
- Consider responsive design for mobile devices
- Test tables in different screen sizes

Testing
-------

To test the tables pipeline:

1. Create an RST file with various table types
2. Run ``cargo run --release -- build``
3. Check the generated HTML in ``_out/dist/``
4. Verify table structure, styling, and interactivity
5. Test search, sorting, and copy functionality

See ``_content/articles/rst-tables-guide/index.rst`` for a comprehensive example of all table types.