Diagrams Design Specification
==============================

Overview
--------

The diagrams feature provides a comprehensive, extensible system for creating various types of diagrams in RST files with full Rust-based rendering. This system follows Peta's existing patterns for code blocks and embedded snippet cards, ensuring consistency and maintainability.

Key Design Principles
~~~~~~~~~~~~~~~~~~~~~

- **Full Rust Rendering**: All diagram rendering happens during build time in Rust, with complete SVG generation embedded in the final HTML
- **No Client-Side Rendering Dependencies**: No external JavaScript libraries required for diagram rendering
- **Extensible Architecture**: Easy to add new diagram types following established patterns
- **Theme-Aware**: Light/dark mode support with consistent styling
- **Performance-Focused**: Static SVG output with minimal runtime overhead
- **Consistent with Existing Patterns**: Follows code blocks and embedded snippet cards architecture

Supported Diagram Types
-----------------------

.. list-table:: Supported Diagram Types
   :widths: 25 50 25
   :header-rows: 1

   * - Type
     - Description
     - Status
   * - Flowchart
     - Node-based process diagrams with arrows
     - Planned
   * - Gantt Chart
     - Timeline-based project schedules
     - Planned
   * - Sequence Diagram
     - Interaction sequences between actors
     - Planned
   * - Class Diagram
     - UML class structures and relationships
     - Planned
   * - State Diagram
     - State transitions and events
     - Planned

Architecture
------------

Directory Structure
~~~~~~~~~~~~~~~~~~~

.. code-block:: text

   peta/src/content/rst/
   └── diagrams/
       ├── mod.rs              # Module exports
       ├── parser.rs           # RST diagram syntax parser
       ├── models.rs           # Data structures for diagrams
       ├── flowchart.rs        # Flowchart renderer
       ├── gantt.rs            # Gantt chart renderer
       ├── sequence.rs         # Sequence diagram renderer
       ├── class.rs            # Class diagram renderer
       └── state.rs            # State diagram renderer

Rendering Pipeline
~~~~~~~~~~~~~~~~~~

.. code-block:: text

   RST File (.. diagram:: type)
          ↓
   RstParser::process_directives()
          ↓
   DiagramHandler (directives.rs)
          ↓
   DiagramParser (diagrams/parser.rs)
          ↓
   Diagram Models (diagrams/models.rs)
          ↓
   Type-Specific Renderer (diagrams/*.rs)
          ↓
   Complete HTML with Embedded SVG
          ↓
   Template Engine
          ↓
   Final HTML Output

Data Flow
~~~~~~~~~

1. **Parsing**: Text-based diagram syntax is parsed into structured data models
2. **Layout Calculation**: Node positions and edge paths are calculated using layout algorithms
3. **SVG Generation**: Complete SVG markup is generated with theme-aware styling
4. **HTML Wrapping**: SVG is wrapped in container div with metadata attributes
5. **Output Integration**: Final HTML is integrated into the page template

Syntax Examples
---------------

Flowchart
~~~~~~~~~

.. code-block:: rst

   .. diagram:: flowchart
      
      Start -> Process -> Decision -> End
      Decision -> No -> Process
      Decision -> Yes -> End

Gantt Chart
~~~~~~~~~~~

.. code-block:: rst

   .. diagram:: gantt
      
      Task1 [2024-01-01] : 5d
      Task2 [2024-01-06] : 3d
      Task3 [2024-01-09] : 4d

Sequence Diagram
~~~~~~~~~~~~~~~~

.. code-block:: rst

   .. diagram:: sequence
      
      Alice -> Bob: Hello
      Bob -> Alice: Hi there
      Alice -> Bob: How are you?

Class Diagram
~~~~~~~~~~~~~

.. code-block:: rst

   .. diagram:: class-diagram
      
      User |+| Database
      User |+| API
      API |o| Cache

State Diagram
~~~~~~~~~~~~~

.. code-block:: rst

   .. diagram:: state
      
      Idle -> Running : start
      Running -> Idle : stop
      Running -> Paused : pause
      Paused -> Running : resume

Implementation Details
----------------------

Core Components
~~~~~~~~~~~~~~~

**models.rs**

Defines data structures for all diagram types:

.. code-block:: rust

   pub enum DiagramType {
       Flowchart,
       Gantt,
       Sequence,
       ClassDiagram,
       State,
   }

   pub struct FlowchartNode {
       pub id: String,
       pub label: String,
       pub x: f64,
       pub y: f64,
       pub width: f64,
       pub height: f64,
   }

   pub struct FlowchartEdge {
       pub from: String,
       pub to: String,
       pub label: Option<String>,
   }

**parser.rs**

Parses text-based diagram syntax into data models:

.. code-block:: rust

   pub struct DiagramParser;

   impl DiagramParser {
       pub fn parse(input: &str, diagram_type: DiagramType) -> Result<Diagram> {
           match diagram_type {
               DiagramType::Flowchart => self.parse_flowchart(input),
               DiagramType::Gantt => self.parse_gantt(input),
               // ... other types
           }
       }
   }

**flowchart.rs**

Renders flowcharts to SVG:

.. code-block:: rust

   pub struct FlowchartRenderer;

   impl FlowchartRenderer {
       pub fn render(diagram: &FlowchartDiagram) -> Result<String> {
           // 1. Calculate node positions
           // 2. Generate SVG elements for nodes
           // 3. Generate SVG paths for edges
           // 4. Add arrow markers
           // 5. Apply theme colors
       }
   }

HTML Output Structure
~~~~~~~~~~~~~~~~~~~~~

Complete HTML with embedded SVG:

.. code-block:: html

   <div class="diagram-container" data-diagram-id="diag-123" data-diagram-type="flowchart">
     <svg viewBox="0 0 800 400" xmlns="http://www.w3.org/2000/svg" class="diagram-svg">
       <!-- Background -->
       <rect width="800" height="400" fill="white"/>
       
       <!-- Arrow marker -->
       <defs>
         <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
           <polygon points="0 0, 10 3.5, 0 0 7" fill="#3b82f6"/>
         </marker>
       </defs>
       
       <!-- Edges -->
       <path d="M 100 200 L 300 200" stroke="#3b82f6" stroke-width="2" marker-end="url(#arrowhead)"/>
       
       <!-- Nodes -->
       <rect x="50" y="180" width="100" height="40" rx="8" fill="#e0f2fe" stroke="#3b82f6" stroke-width="2"/>
       <text x="100" y="205" text-anchor="middle" font-size="14" font-family="Inter">Start</text>
       
       <!-- More nodes... -->
     </svg>
     
     <!-- Optional: source code display -->
     <details class="diagram-source">
       <summary>View source</summary>
       <pre>Start -> Process -> Decision -> End</pre>
     </details>
   </div>

Layout Algorithms
~~~~~~~~~~~~~~~~~

**Flowchart Layout**

- Hierarchical layout algorithm
- Automatic node positioning based on graph topology
- Edge routing with bezier curves
- Support for cycles and subgraphs

**Gantt Chart Layout**

- Timeline-based layout
- Date-to-pixel conversion
- Task bar positioning
- Grid line generation

**Sequence Diagram Layout**

- Lane-based layout
- Actor positioning
- Message arrow routing
- Activation box calculation

**Class Diagram Layout**

- Tree-based layout
- Relationship positioning
- Inheritance arrow routing
- Composition marker placement

Integration with RST Parser
~~~~~~~~~~~~~~~~~~~~~~~~~~~

Integration point in ``directives.rs``:

.. code-block:: rust

   directive_handlers.insert(
       "diagram".to_string(),
       Box::new(DiagramHandler::new()),
   );

DiagramHandler implementation:

.. code-block:: rust

   pub struct DiagramHandler {
       diagram_parser: DiagramParser,
   }

   impl DirectiveHandler for DiagramHandler {
       fn process(&self, args: &DirectiveArgs, content: &str) -> Result<String> {
           let diagram_type = parse_diagram_type(args)?;
           let diagram = self.diagram_parser.parse(content, diagram_type)?;
           let renderer = get_renderer(diagram_type)?;
           let html = renderer.render(&diagram)?;
           Ok(html)
       }
   }

Configuration
-------------

peta.toml Configuration
~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: toml

   [diagrams]
   default_theme = "peta"
   default_font_family = "Inter"
   default_font_size = 14
   enable_source_toggle = true
   enable_zoom = false

Theme Configuration
~~~~~~~~~~~~~~~~~~~

Color schemes for light/dark modes:

.. code-block:: toml

   [diagrams.theme.light]
   background = "#ffffff"
   text = "#1f2937"
   stroke = "#3b82f6"
   fill = "#e0f2fe"
   
   [diagrams.theme.dark]
   background = "#1f2937"
   text = "#f3f4f6"
   stroke = "#60a5fa"
   fill = "#1e3a8a"

CSS Styling
~~~~~~~~~~~

Container and SVG styling:

.. code-block:: css

   .diagram-container {
       margin: 2rem 0;
       padding: 1rem;
       border: 1px solid #e5e7eb;
       border-radius: 0.5rem;
   }
   
   .diagram-svg {
       width: 100%;
       height: auto;
   }
   
   .diagram-source {
       margin-top: 1rem;
   }

Theming
-------

Light/Dark Mode Support
~~~~~~~~~~~~~~~~~~~~~~~

- CSS variables for dynamic theme switching
- Automatic theme detection from page context
- Theme-aware color application during SVG generation
- High contrast ratios for accessibility

Custom Themes
~~~~~~~~~~~~~

Users can define custom themes in their site configuration:

.. code-block:: toml

   [diagrams.themes.custom]
   name = "Corporate"
   colors = {
       primary = "#2563eb",
       secondary = "#10b981",
       background = "#f8fafc",
       text = "#0f172a"
   }

Accessibility
-------------

ARIA Labels
~~~~~~~~~~~

Diagrams include ARIA attributes for screen readers:

.. code-block:: html

   <div class="diagram-container" 
        role="img" 
        aria-labelledby="diagram-title-123"
        aria-describedby="diagram-desc-123">
     <h2 id="diagram-title-123">Process Flowchart</h2>
     <div id="diagram-desc-123">
       A flowchart showing the process from Start through Decision to End
     </div>
     <!-- SVG content -->
   </div>

Keyboard Navigation
~~~~~~~~~~~~~~~~~~~

Optional keyboard support for interactive features:
- Tab navigation through nodes
- Arrow key navigation
- Enter to view node details

Performance Considerations
---------------------------

Build-Time Rendering
~~~~~~~~~~~~~~~~~~~~

- All SVG generation happens during build
- No client-side computation required
- Static HTML output is highly cacheable
- Fast page load times

Optimization Strategies
~~~~~~~~~~~~~~~~~~~~~~~

- SVG minification
- Shared marker definitions
- Reusable SVG patterns
- Efficient layout algorithms
- Memory-efficient string building

Future Extensibility
--------------------

Adding New Diagram Types
~~~~~~~~~~~~~~~~~~~~~~~~

To add a new diagram type:

1. Add variant to ``DiagramType`` enum in ``models.rs``
2. Add parser implementation in ``parser.rs``
3. Create renderer module (e.g., ``network.rs``)
4. Register renderer in module exports
5. Add syntax documentation

Potential Future Diagram Types
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- Network topology diagrams
- Entity-relationship diagrams
- Mind maps
- Tree diagrams
- Block diagrams
- Timeline diagrams

Known Limitations
-----------------

Current Scope
~~~~~~~~~~~~~

- Limited to 2D diagram rendering
- No real-time animation support
- Fixed layout algorithms
- Limited interactivity

Planned Enhancements
~~~~~~~~~~~~~~~~~~~~

- Advanced layout algorithms (force-directed, hierarchical)
- Interactive hover effects
- Pan and zoom capabilities
- Export to PNG/SVG
- Customizable node shapes
- Subdiagram support

Troubleshooting
---------------

Common Issues
~~~~~~~~~~~~~

**Diagrams not rendering**

- Check that diagram syntax is valid
- Ensure diagram type is supported
- Verify Rust compilation succeeded

**Layout issues**

- Review diagram structure for cycles
- Check node label lengths
- Verify syntax correctness

**Color/Theme issues**

- Check theme configuration in peta.toml
- Ensure CSS is properly included
- Verify color values are valid

Testing Strategy
----------------

Unit Tests
~~~~~~~~~~

- Parser tests for each diagram type
- Layout algorithm validation
- SVG generation verification

Integration Tests
~~~~~~~~~~~~~~~~~

- End-to-end RST processing
- Template integration
- Theme switching

Visual Regression Tests
~~~~~~~~~~~~~~~~~~~~~~~

- Snapshot testing for SVG output
- Theme validation
- Cross-browser compatibility

References
----------

Similar Systems
~~~~~~~~~~~~~~~

- Mermaid.js (syntax inspiration)
- Graphviz (layout algorithms)
- PlantUML (diagram types)

Related Peta Features
~~~~~~~~~~~~~~~~~~~~~

- Code Blocks (rendering pattern)
- Embedded Snippet Cards (component architecture)
- Math Formulas (RST integration)
- RST Parser (directive system)

Version History
---------------

- v1.0.0 (Planned): Initial implementation with flowchart, gantt, sequence, and class diagrams