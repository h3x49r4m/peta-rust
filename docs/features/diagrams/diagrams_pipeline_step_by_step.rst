Diagram Rendering Pipeline: Step by Step
==========================================

This document provides a detailed, step-by-step walkthrough of how diagrams are rendered from RST files to final HTML output with embedded SVG.

Overview
--------

The diagram rendering pipeline consists of the following stages:

1. **RST File Parsing** - Reading and extracting diagram directives from RST files
2. **Directive Processing** - Extracting diagram type, content, and options
3. **Content Parsing** - Converting text-based diagram syntax into data models
4. **Layout Calculation** - Computing node positions and canvas dimensions
5. **SVG Generation** - Creating SVG markup from the layout
6. **HTML Embedding** - Wrapping SVG in container HTML with download button
7. **Asset Generation** - Creating CSS and JavaScript for diagram styling and interactions
8. **Final Output** - Generating the complete HTML page

Step 1: RST File Parsing
--------------------------

The pipeline begins when Peta reads an RST file (e.g., ``articles/diagram-feature-test.rst``):

**Input RST Example:**

.. code-block:: rst

   .. diagram:: flowchart
      :title: Order Processing Flow
      
      Start -> Process -> End

**File Reading Process:**

.. code-block:: rust

   // In RstParser::parse_with_type_and_path()
   let (frontmatter, rst_content) = self.extract_frontmatter(content)?;
   let processed_html = self.process_rst_content(&rst_content)?;

The RST parser:

1. Extracts YAML frontmatter (if present)
2. Identifies directive blocks (lines starting with ``..``)
3. Separates directives from regular content

Step 2: Directive Processing
------------------------------

The ``process_directives()`` method identifies and processes diagram directives:

**Regex Pattern Matching:**

.. code-block:: rust

   let directive_start_regex = Regex::new(r"\.\. ([a-zA-Z0-9_-]+)::")?;

**Process:**

1. **Extract directive name**: ``diagram`` in ``.. diagram:: flowchart``
2. **Extract diagram type**: ``flowchart`` from the line after ``::``
3. **Extract content**: The indented text following the directive
4. **Extract field list options**: Lines starting with ``:`` (e.g., ``:title: ...``)

**Field List Parsing:**

.. code-block:: rust

   // Extract field list options (lines starting with ":")
   let mut options = std::collections::HashMap::new();
   let mut content_lines: Vec<&str> = Vec::new();
   
   for line in directive_content.lines() {
       let trimmed = line.trim();
       if trimmed.starts_with(':') && trimmed.len() > 1 {
           // This is a field list option
           if let Some(colon_pos) = trimmed[1..].find(':') {
               let actual_colon_pos = colon_pos + 1;
               let key = trimmed[1..actual_colon_pos].trim().to_string();
               let value = trimmed[actual_colon_pos + 1..].trim().to_string();
               if !key.is_empty() {
                   options.insert(key, value);
               }
           }
       } else {
           // This is content
           content_lines.push(line);
       }
   }

**Extracted Values:**

- ``directive_name``: ``diagram``
- ``diagram_type``: ``flowchart``
- ``title``: ``Order Processing Flow`` (from options)
- ``content``: ``Start -> Process -> End``

Step 3: Handler Dispatch
-----------------------

The parsed data is passed to the appropriate handler:

.. code-block:: rust

   if let Some(handler) = self.directive_handlers.get_mut(directive_name) {
       let processed = handler.handle(diagram_type, &actual_content, &options)?;
       result.push_str(&processed);
   }

**DiagramHandler Implementation:**

.. code-block:: rust

   impl DirectiveHandler for DiagramHandler {
       fn handle(&mut self, diagram_type: &str, content: &str, options: &HashMap<String, String>) -> Result<String> {
           // Clean up the diagram content
           let content = content
               .replace("<p>", "")
               .replace("</p>", "\n");

           // Extract title from options
           let title = options.get("title").map(|t| t.as_str());

           // Render the diagram
           self.renderer.render(diagram_type, &content, title)
       }
   }

Step 4: Diagram Type Dispatch
-----------------------------

The ``DiagramRenderer`` dispatches to the appropriate renderer based on the diagram type:

.. code-block:: rust

   pub fn render(&self, diagram_type: &str, content: &str, title: Option<&str>) -> Result<String> {
       let diagram = self.parser.parse(diagram_type, content)?;
       
       match diagram {
           Diagram::Flowchart(d) => FlowchartRenderer::new()?.render(&d, title),
           Diagram::Gantt(d) => GanttRenderer::new()?.render(&d, title),
           Diagram::Sequence(d) => SequenceRenderer::new()?.render(&d, title),
           Diagram::Class(d) => ClassRenderer::new()?.render(&d, title),
           Diagram::State(d) => StateRenderer::new()?.render(&d, title),
       }
   }

Step 5: Content Parsing
-----------------------

The ``DiagramParser`` converts text-based diagram syntax into structured data models.

**Flowchart Example:**

**Input Text:**

.. code-block:: text

   Start -> Process -> Decision -> End
   Decision -> No -> Process
   Decision -> Yes -> End

**Parsing Process:**

.. code-block:: rust

   fn parse_flowchart(&self, content: &str) -> Result<FlowchartDiagram> {
       let mut nodes: Vec<FlowchartNode> = Vec::new();
       let mut edges: Vec<FlowchartEdge> = Vec::new();
       let mut node_map: HashMap<String, usize> = HashMap::new();
       
       for line in content.lines() {
           if line.contains("->") {
               // Split on "->" to handle multi-arity edges
               let parts: Vec<&str> = line.split("->").collect();
               
               // Create nodes for all parts
               for (idx, part) in parts.iter().enumerate() {
                   let node_id = part.trim();
                   if !node_id.is_empty() && !node_map.contains_key(node_id) {
                       let node = FlowchartNode {
                           id: node_id.to_string(),
                           label: node_id.to_string(),
                           node_type: FlowchartNodeType::Process,
                       };
                       node_map.insert(node_id.clone(), nodes.len());
                       nodes.push(node);
                   }
               }
               
               // Create edges between consecutive parts
               for i in 0..parts.len() - 1 {
                   let from = parts[i].trim();
                   let to = parts[i + 1].trim();
                   if !from.is_empty() && !to.is_empty() {
                       edges.push(FlowchartEdge {
                           from: from.to_string(),
                           to: to.to_string(),
                           label: None,
                       });
                   }
               }
           }
       }
       
       Ok(FlowchartDiagram { nodes, edges })
   }

**Output Data Model:**

.. code-block:: rust

   FlowchartDiagram {
       nodes: [
           FlowchartNode { id: "Start", label: "Start", node_type: Process },
           FlowchartNode { id: "Process", label: "Process", node_type: Process },
           FlowchartNode { id: "Decision", label: "Decision", node_type: Decision },
           FlowchartNode { id: "End", label: "End", node_type: StartEnd },
           FlowchartNode { id: "No", label: "No", node_type: Process },
           FlowchartNode { id: "Yes", label: "Yes", node_type: Process },
       ],
       edges: [
           FlowchartEdge { from: "Start", to: "Process", label: None },
           FlowchartEdge { from: "Process", to: "Decision", label: None },
           FlowchartEdge { from: "Decision", to: "End", label: None },
           FlowchartEdge { from: "Decision", to: "No", label: None },
           FlowchartEdge { from: "Decision", to: "Yes", label: None },
       ],
   }

Step 6: Layout Calculation
--------------------------

Each renderer calculates the layout for its diagram type.

**Flowchart Layout Algorithm (Hierarchical BFS):**

.. code-block:: rust

   fn calculate_layout(diagram: &FlowchartDiagram, title: Option<&str>) -> FlowchartLayout {
       let mut layout = FlowchartLayout {
           nodes: Vec::new(),
           edges: Vec::new(),
           width: 800.0,
           height: 400.0,
       };

       // Adjust height if title is present
       if title.is_some() {
           layout.height += 40.0;
       }

       let y_offset = title.map(|_| 40.0).unwrap_or(0.0);

       // Calculate levels using BFS
       let mut levels: HashMap<String, usize> = HashMap::new();
       let mut queue: Vec<(String, usize)> = Vec::new();
       
       // Find starting nodes (no incoming edges)
       let mut has_incoming: HashSet<String> = HashSet::new();
       for edge in &diagram.edges {
           has_incoming.insert(edge.to.clone());
       }
       
       for node in &diagram.nodes {
           if !has_incoming.contains(&node.id) {
               queue.push((node.id.clone(), 0));
               levels.insert(node.id.clone(), 0);
           }
       }

       // BFS to assign levels
       while !queue.is_empty() {
           queue.sort_by_key(|(_, level)| *level);
           let (node_id, level) = queue.remove(0);
           
           for edge in &diagram.edges {
               if edge.from == node_id {
                   let new_level = level + 1;
                   let current_level = levels.get(&edge.to).copied().unwrap_or(usize::MAX);
                   if new_level < current_level {
                       levels.insert(edge.to.clone(), new_level);
                       queue.push((edge.to.clone(), new_level));
                   }
               }
           }
       }

       // Group nodes by level
       let mut level_nodes: HashMap<usize, Vec<&FlowchartNode>> = HashMap::new();
       for node in &diagram.nodes {
           let level = levels.get(&node.id).copied().unwrap_or(0);
           level_nodes.entry(level).or_insert_with(Vec::new).push(node);
       }

       // Calculate positions (centered horizontally)
       let node_width = 120.0;
       let level_height = 100.0;
       
       for (level, nodes_at_level) in &level_nodes {
           let level_width_total = nodes_at_level.len() as f64 * node_width;
           let start_x = (layout.width - level_width_total) / 2.0;
           
           for (idx, node) in nodes_at_level.iter().enumerate() {
               let x = start_x + idx as f64 * node_width;
               let y = 50.0 + *level as f64 * level_height + y_offset;
               
               layout.nodes.push(NodePosition {
                   id: node.id.clone(),
                   x, y,
                   width: node_width,
                   height: 50.0,
               });
           }
       }

       layout
   }

**Layout Output:**

- Each node has: ``x``, ``y`` coordinates, ``width``, ``height``
- Edges reference nodes by ID
- Canvas size adjusted for title and content

Step 7: SVG Generation
-----------------------

The renderer generates SVG markup from the calculated layout.

**SVG Structure:**

.. code-block:: rust

   fn generate_svg(&self, diagram: &FlowchartDiagram, layout: &FlowchartLayout, title: Option<&str>) -> String {
       let mut svg = String::new();

       // 1. Add title if present
       if let Some(title_text) = title {
           svg.push_str(&format!(
               r##"    <text x="{}" y="25" text-anchor="middle" font-size="18" font-weight="bold" font-family="Inter" fill="#1f2937">{}</text>
"##,
               layout.width / 2.0, title_text
           ));
       }

       // 2. Add definitions for arrowhead
       svg.push_str(r##"
     <defs>
       <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
         <polygon points="0 0, 10 3.5, 0 7" fill="#3b82f6"/>
       </marker>
     </defs>
   "##);

       // 3. Add edges (behind nodes)
       for edge in &diagram.edges {
           if let (Some(from_pos), Some(to_pos)) = (
               layout.nodes.iter().find(|n| n.id == edge.from),
               layout.nodes.iter().find(|n| n.id == edge.to),
           ) {
               let from_x = from_pos.x + from_pos.width / 2.0;
               let from_y = from_pos.y + from_pos.height / 2.0;
               let to_x = to_pos.x + to_pos.width / 2.0;
               let to_y = to_pos.y + to_pos.height / 2.0;

               svg.push_str(&format!(
                   r##"    <path d="M {} {} L {} {}" stroke="#3b82f6" stroke-width="2" marker-end="url(#arrowhead)" />
"##,
                   from_x, from_y, to_x, to_y
               ));
           }
       }

       // 4. Add nodes
       for node in &diagram.nodes {
           if let Some(pos) = layout.nodes.iter().find(|n| n.id == node.id) {
               let (fill, stroke, rx) = match node.node_type {
                   FlowchartNodeType::StartEnd => ("#d1fae5", "#059669", 25.0),
                   FlowchartNodeType::Decision => ("#fef3c7", "#d97706", 8.0),
                   _ => ("#dbeafe", "#2563eb", 8.0)
               };

               svg.push_str(&format!(
                   r##"    <rect x="{}" y="{}" width="{}" height="{}" rx="{}" fill="{}" stroke="{}" stroke-width="2"/>
"##,
                   pos.x, pos.y, pos.width, pos.height, rx, fill, stroke
               ));

               svg.push_str(&format!(
                   r##"    <text x="{}" y="{}" text-anchor="middle" font-size="14" font-family="Inter" fill="#1f2937">{}</text>
"##,
                   pos.x + pos.width / 2.0,
                   pos.y + pos.height / 2.0 + 5.0,
                   node.label
               ));
           }
       }

       svg
   }

**Generated SVG:**

.. code-block:: html

   <svg viewBox="0 0 800 550" xmlns="http://www.w3.org/2000/svg" class="diagram-svg">
       <text x="400" y="25" text-anchor="middle" font-size="18" font-weight="bold" font-family="Inter" fill="#1f2937">Order Processing Flow</text>
     
     <defs>
       <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
         <polygon points="0 0, 10 3.5, 0 7" fill="#3b82f6"/>
       </marker>
     </defs>
     
     <path d="M 400 75 L 400 175" stroke="#3b82f6" stroke-width="2" marker-end="url(#arrowhead)" />
     <rect x="340" y="50" width="120" height="50" rx="25" fill="#d1fae5" stroke="#059669" stroke-width="2"/>
     <text x="400" y="80" text-anchor="middle" font-size="14" font-family="Inter" fill="#1f2937">Start</text>
     ...
   </svg>

Step 8: HTML Embedding
----------------------

The SVG is wrapped in a container HTML with a download button.

.. code-block:: rust

   let html = format!(
       r#"<div class="diagram-container" data-diagram-id="{}" data-diagram-type="flowchart">
     <button class="diagram-download" data-diagram-id="{}" data-diagram-type="flowchart" aria-label="Download diagram as SVG">
       <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
         <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
         <polyline points="7 10 12 15 17 10"/>
         <line x1="12" y1="15" x2="12" y2="3"/>
       </svg>
     </button>
     <svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg" class="diagram-svg">
       {}
     </svg>
   </div>"#,
       diagram_id, diagram_id, layout.width, layout.height, svg
   );

**Generated HTML:**

.. code-block:: html

   <div class="diagram-container" data-diagram-id="flowchart-abc123" data-diagram-type="flowchart">
     <button class="diagram-download" data-diagram-id="flowchart-abc123" data-diagram-type="flowchart" aria-label="Download diagram as SVG">
       <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
         <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
         <polyline points="7 10 12 15 17 10"/>
         <line x1="12" y1="15" x2="12" y2="3"/>
       </svg>
     </button>
     <svg viewBox="0 0 800 550" xmlns="http://www.w3.org/2000/svg" class="diagram-svg">
       ...
     </svg>
   </div>

Step 9: Asset Generation
-----------------------

The asset pipeline generates CSS and JavaScript for diagram styling and interactions.

**CSS Generation (DiagramCssGenerator):**

.. code-block:: rust

   pub fn generate(&self) -> Result<String> {
       let mut css = String::new();
       css.push_str("/* Diagram Styles */\n\n");
       css.push_str(&self.generate_base_styles());
       css.push_str(&self.generate_svg_styles());
       css.push_str(&self.generate_download_button_styles());
       css.push_str(&self.generate_dark_mode_styles());
       css.push_str(&self.generate_responsive_styles());
       Ok(css)
   }

**Generated CSS:**

.. code-block:: css

   .diagram-container {
     margin: 2rem 0;
     padding: 1.5rem;
     padding-top: 3rem;
     border: 1px solid #e5e7eb;
     border-radius: 8px;
     background: #ffffff;
     box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
     overflow-x: auto;
     position: relative;
   }

   .diagram-download {
     position: absolute;
     top: 0.75rem;
     right: 0.75rem;
     display: inline-flex;
     align-items: center;
     gap: 0.5rem;
     background: #ffffff;
     border: 1px solid #e5e7eb;
     padding: 0.5rem 0.875rem;
     border-radius: 0.375rem;
     font-size: 0.8125rem;
     font-weight: 500;
     color: #374151;
     cursor: pointer;
     transition: all 0.2s;
   }

   .diagram-svg {
     width: 100%;
     height: auto;
     display: block;
     margin: 0 auto;
   }

**JavaScript Generation (DiagramJsGenerator):**

.. code-block:: rust

   pub fn generate(&self) -> Result<String> {
       let mut js = String::new();
       
       js.push_str("/**\n");
       js.push_str(" * Diagram Component JavaScript\n");
       js.push_str(" */\n\n");
       
       js.push_str("(function() {\n");
       js.push_str("  'use strict';\n\n");
       
       js.push_str("  /**\n");
       js.push_str("   * Download diagram as SVG with copyright text\n");
       js.push_str("   */\n");
       js.push_str("  function downloadDiagram(button) {\n");
       // ... download logic
       js.push_str("  }\n\n");
       
       js.push_str("  /**\n");
       js.push_str("   * Add copyright text to SVG\n");
       js.push_str("   */\n");
       js.push_str("  function addCopyrightToSVG(svg) {\n");
       // ... copyright logic
       js.push_str("  }\n\n");
       
       js.push_str("  function initDownloadButtons() {\n");
       js.push_str("    const downloadButtons = document.querySelectorAll('.diagram-download');\n");
       js.push_str("    downloadButtons.forEach(button => {\n");
       js.push_str("      button.addEventListener('click', function(e) {\n");
       js.push_str("        e.preventDefault();\n");
       js.push_str("        downloadDiagram(this);\n");
       js.push_str("      });\n");
       js.push_str("    });\n");
       js.push_str("  }\n\n");
       
       js.push_str("  if (document.readyState === 'loading') {\n");
       js.push_str("    document.addEventListener('DOMContentLoaded', initDownloadButtons);\n");
       js.push_str("  } else {\n");
       js.push_str("    initDownloadButtons();\n");
       js.push_str("  }\n");
       
       js.push_str("})();\n");
       
       Ok(js)
   }

**Generated JavaScript Features:**

- Download button click handlers
- Copyright text generation and embedding
- SVG cloning and manipulation
- Text wrapping for long URLs
- ViewBox adjustment for copyright
- Blob creation and download

Step 10: Download Functionality
-------------------------------

When a user clicks the download button, the JavaScript:

1. **Clones the SVG** - Creates a copy to avoid modifying the displayed version
2. **Adds copyright** - Generates ``© <site_name> (<page_url>)`` text
3. **Wraps long URLs** - Splits URL into multiple lines if needed
4. **Adjusts viewBox** - Expands SVG height to accommodate copyright
5. **Creates Blob** - Generates a Blob with MIME type ``image/svg+xml``
6. **Triggers download** - Creates an anchor element and clicks it

**Copyright Generation Example:**

.. code-block:: javascript

   function addCopyrightToSVG(svg) {
       const currentPageUrl = window.location.href;
       const siteName = document.querySelector('meta[property="og:site_name"]')?.content || 'Peta';
       const copyrightText = `© ${siteName} (${currentPageUrl})`;
       
       // Calculate space needed for copyright
       const fontSize = 10;
       const lineHeight = 12;
       const textLines = wrapText(copyrightText, maxWidth, fontSize);
       const extraHeight = textLines.length * lineHeight + padding + bottomPadding;
       
       // Create copyright text element
       const copyrightGroup = document.createElementNS('http://www.w3.org/2000/svg', 'g');
       
       let currentY = height + fontSize + padding;
       
       textLines.forEach((line, index) => {
           const textElement = document.createElementNS('http://www.w3.org/2000/svg', 'text');
           textElement.setAttribute('x', padding.toString());
           textElement.setAttribute('y', currentY.toString());
           textElement.setAttribute('font-size', fontSize.toString());
           textElement.setAttribute('font-family', 'Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif');
           textagramElement.setAttribute('fill', '#9ca3af');
           textElement.textContent = line;
           copyrightGroup.appendChild(textElement);
           currentY += lineHeight;
       });
       
       // Update viewBox
       const newHeight = height + extraHeight;
       svg.setAttribute('viewBox', `${x} ${y} ${width} ${newHeight}`);
       
       svg.appendChild(copyrightGroup);
   }

Step 11: Final HTML Assembly
--------------------------

The template engine assembles all components into the final HTML page:

.. code-block:: html

   <!DOCTYPE html>
   <html lang="en">
   <head>
       <meta charset="UTF-8">
       <meta name="viewport" content="width=device-width, initial-scale=1.0">
       <title>Diagram Feature Test - Peta</title>
       
       <!-- External CSS -->
       <link rel="stylesheet" href="/css/main.css">
       
       <!-- Diagram Styles (generated from Rust) -->
       <link rel="stylesheet" href="/assets/css/diagrams.css">
   </head>
   <body>
       <div class="article-body">
           <h2 id="flowchart-example">Flowchart Example</h2>
           
           <div class="diagram-container" data-diagram-id="flowchart-abc123" data-diagram-type="flowchart">
               <button class="diagram-download" ...>
                   <svg>...</svg>
               </button>
               <svg viewBox="0 0 800 550" ...>
                   ...
               </svg>
           </div>
           
           <p>This is a simple flowchart...</p>
       </div>
       
       <!-- Diagram Scripts (generated from Rust) -->
       <script src="/assets/js/diagrams.js"></script>
   </body>
   </html>

Architecture Summary
------------------

.. code-block:: text

   RST File
     ↓
   RstParser (extract frontmatter, content)
     ↓
   process_directives (identify diagram directives)
     ↓
   DiagramHandler (extract type, content, options)
     ↓
   DiagramRenderer (dispatch to specific renderer)
     ↓
   DiagramParser (parse text → data model)
     ↓
   [FlowchartRenderer|GanttRenderer|...]
     ├─ calculate_layout (compute positions)
     └─ generate_svg (create SVG markup)
     ↓
   HTML Embedding (wrap in container + download button)
     ↓
   AssetPipeline (generate CSS/JS)
     ↓
   TemplateEngine (assemble final HTML)
     ↓
   _out/dist/articles/*.html

Supported Diagram Types
------------------------

Each diagram type has its own parser and renderer:

1. **Flowchart** - Hierarchical BFS layout, supports multi-arity edges
2. **Gantt Chart** - Timeline-based layout, date calculations
3. **Sequence Diagram** - Actor-based layout, message interactions
4. **Class Diagram** - Grid-based layout, UML relationships
5. **State Diagram** - Circular layout, state transitions

Key Features
------------

- **Pure Rust rendering** - No client-side JavaScript rendering
- **Build-time SVG generation** - Fast page loads, cacheable output
- **Customizable titles** - Via field list options
- **Download functionality** - SVG export with copyright
- **Responsive design** - CSS adapts to different screen sizes
- **Dark mode support** - Automatic theme adaptation
- **Extensible architecture** - Easy to add new diagram types

Performance Considerations
-------------------------

- Diagrams are rendered once during build time
- SVG is embedded directly in HTML - no runtime processing
- CSS/JS assets are minified and cached
- No external dependencies for diagram rendering
- Static output enables CDN deployment

Troubleshooting
---------------

**Issue: Diagram not rendering**

1. Check RST syntax: Ensure ``.. diagram:: type`` format is correct
2. Verify indentation: Content must be indented relative to directive
3. Check build logs: Look for parsing errors in cargo build output

**Issue: Title not showing**

1. Verify field list syntax: ``:title: Your Title`` must be on its own line
2. Ensure proper indentation: Options should be indented with content

**Issue: Download not working**

1. Check JavaScript: Ensure ``diagrams.js`` is linked in HTML
2. Verify browser console: Look for JavaScript errors
3. Check SVG ID: Download button must reference correct diagram ID

**Issue: Copyright text cut off**

1. Check viewbox: Verify SVG height is adjusted for copyright
2. Check URL length: Long URLs are wrapped, but very long URLs may still overflow
3. Adjust layout: Modify copyright spacing in ``DiagramJsGenerator``

Further Reading
---------------

- :doc:`diagrams_design_spec` - Design specification for diagram system
- :doc:`diagrams_pipeline` - High-level pipeline overview
- :doc:`/features/rst_parser/rst_parsing_pipeline` - RST parsing details
