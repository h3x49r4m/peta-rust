---
title: RST Tables Guide
description: A comprehensive guide to all four types of reStructuredText tables
date: 2026-02-10
tags: [rst, tables, documentation]
---

RST Tables Guide
===============

This article demonstrates all four types of reStructuredText tables supported by the system.

Grid Tables
-----------

Grid tables use `+` characters for borders and provide precise control over layout.

+----------+----------+----------+
| Header 1 | Header 2 | Header 3 |
+==========+==========+==========+
| Cell 1   | Cell 2   | Cell 3   |
+----------+----------+----------+
| Cell 4   | Cell 5   | Cell 6   |
+----------+----------+----------+
| Cell 7   | Cell 8   | Cell 9   |
+----------+----------+----------+

Grid tables support:
- Explicit borders with `+` characters
- Header separator with `===`
- Body separators with `---`
- Multi-line cells
- Empty cells

Simple Tables
-------------

Simple tables use underlines (`=` or `-`) to define structure. They're simpler to write but less flexible.

Name      Age    Occupation
========= ====== ============
Alice     28     Engineer
Bob       32     Designer
Charlie   45     Manager

Simple tables:
- Use `=` for header underline
- Use `-` for body separators
- Columns are separated by whitespace
- No explicit borders

CSV Table Directive
-------------------

The `csv-table` directive is useful for tabular data from external sources.

.. csv-table:: Employee Data
   :header: "Name", "Department", "Salary"
   :widths: 25, 25, 15

   Alice Smith, Engineering, "$85,000"
   Bob Johnson, Design, "$72,000"
   Charlie Brown, Management, "$95,000"
   Diana Prince, Marketing, "$68,000"

CSV table features:
- Comma-separated values
- Optional `:header:` directive
- Custom column widths with `:widths:`
- Handles quoted values: `"Smith, John", Sales, $50,000`

List Table Directive
--------------------

The `list-table` directive provides more flexibility with list-based syntax.

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
   * - Support
     - Email
     - Email + Chat
     - 24/7 Phone
   * - Storage
     - 1 GB
     - 10 GB
     - Unlimited

List table benefits:
- Hierarchical list structure
- Support for multi-line content
- Precise column width control
- Complex cell formatting

Advanced Grid Table Example
---------------------------

Grid tables can handle complex layouts with merged cells and multi-line content.

+-------------------+-------------------+-------------------+
|                  |     Q1 Results    |     Q2 Results    |
+                  +===================+===================+
|                  | Sales             | Sales             |
+===================+===================+===================+
| Product A        | $10,000           | $12,500           |
+-------------------+-------------------+-------------------+
| Product B        | $15,000           | $18,000           |
+-------------------+-------------------+-------------------+
| Total            | $25,000           | $30,500           |
+-------------------+-------------------+-------------------+

Best Practices
--------------

When choosing a table type:

1. **Grid Tables**: Use for precise layout needs or when you need explicit borders
2. **Simple Tables**: Use for quick, simple tables without complex formatting
3. **CSV Tables**: Use when importing data from CSV files or databases
4. **List Tables**: Use for complex content or when you need multi-line cells

Tips for writing tables:

- Keep tables concise and readable
- Use clear, descriptive headers
- Align numeric columns to the right
- Consider responsive design for mobile devices
- Test tables in different screen sizes

Accessibility Considerations
-----------------------------

All RST table types are rendered with:

- Semantic HTML (&lt;table&gt;, &lt;thead&gt;, &lt;tbody&gt;)
- Proper header association (&lt;th&gt; tags)
- Keyboard navigation support
- Screen reader compatible markup
- Responsive scroll indicators for mobile

The generated tables also include:

- Search functionality to filter rows
- Clickable headers for sorting
- Copy button to export table data
- Row/column count in footer
- Dark mode support
- Print-friendly styles