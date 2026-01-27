Code Block Rendering Pipeline Diagram
======================================

This document illustrates the complete code block rendering pipeline using ASCII flow diagrams.

High-Level Architecture
-----------------------

.. code-block:: text

    +-------------------+
    |   RST Source File |
    +---------+---------+
              |
              v
    +-------------------+
    |   RstParser       |
    |   (Parse RST)     |
    +---------+---------+
              |
              v
    +-------------------+
    |   Directive       |
    |   (Extract code   |
    |    block)         |
    +---------+---------+
              |
              v
    +-------------------+
    | CodeBlockRenderer |
    |  (Rust Backend)   |
    +---------+---------+
              |
              +---------------------+
              |                     |
              v                     v
    +-------------------+  +-------------------+
    | SyntaxHighlighter|  |  Asset Generators|
    |   (syntect)      |  |  (CSS & JS)       |
    +---------+---------+  +---------+---------+
              |                     |
              v                     v
    +-------------------+  +-------------------+
    |  HTML Generation |  |  CSS/JS Output   |
    +---------+---------+  +---------+---------+
              |                     |
              +---------------------+
                              |
                              v
                     +-------------------+
                     |   Template Engine |
                     |  (Inject HTML)     |
                     +---------+---------+
                               |
                               v
                      +-------------------+
                      |   Final HTML Page |
                      +-------------------+

Detailed Rendering Flow
-------------------------

Step 1: RST Parsing
~~~~~~~~~~~~~~~~~

.. code-block:: text

    RST File: example.rst
    +--------------------------------------------------+
    | .. code-block:: python                         |
    |                                                  |
    |     def hello():                                |
    |         print("Hello, World!")                  |
    +--------------------------------------------------+
                        |
                        v
    +--------------------------------------------------+
    | RstParser                                       |
    |                                                  |
    | 1. Read RST content                             |
    | 2. Identify directives with regex               |
    | 3. Extract directive type (language)            |
    | 4. Extract directive content (code)             |
    +--------------------------------------------------+
                        |
                        v
    Parsed Output:
    +--------------------------------------------------+
    | directive_type: "python"                        |
    | content: "def hello():\n    print('...')"      |
    +--------------------------------------------------+

Step 2: Directive Handling
~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: text

    Parsed Input:
    +--------------------------------------------------+
    | directive_type: "python"                        |
    | content: "def hello():\n    print('...')"      |
    +--------------------------------------------------+
                        |
                        v
    +--------------------------------------------------+
    | CodeBlockHandler                                |
    |                                                  |
    | 1. Receive parsed directive                      |
    | 2. Validate language (default to "text")       |
    | 3. Clean content (remove <p> tags)              |
    | 4. Delegate to CodeBlockRenderer                |
    +--------------------------------------------------+
                        |
                        v
    Clean Input:
    +--------------------------------------------------+
    | language: "python"                              |
    | code: "def hello():\n    print('Hello...')"    |
    +--------------------------------------------------+

Step 3: Syntax Highlighting
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: text

    Code Input:
    +--------------------------------------------------+
    | language: "python"                              |
    | code: "def hello():\n    print('Hello...')"    |
    +--------------------------------------------------+
                        |
                        v
    +--------------------------------------------------+
    | SyntaxHighlighter                               |
    |                                                  |
    | 1. Resolve language alias (py → python)         |
    | 2. Find syntax definition in syntect           |
    | 3. Tokenize code into semantic tokens           |
    | 4. Generate HTML with class attributes          |
    | 5. Convert syntect classes to token classes     |
    +--------------------------------------------------+
                        |
                        +-------------------------+
                        |                         |
                        v                         v
    +----------------------------------+  +----------------------------------+
    | Language Resolution       |  |  Tokenization            |
    |                            |  |                          |
    | Input:  "python"           |  | Input:  "def hello():..."  |
    |                            |  |                          |
    | Process:                   |  | Process:                   |
    | - Check aliases            |  | - Break into tokens        |
    | - Validate language        |  | - Identify token types     |
    | - Return resolved name     |  | - Apply syntax rules      |
    |                            |  |                          |
    | Output: "python"           |  | Output: [keyword, fn, str] |
    +----------------------------------+  +----------------------------------+
                        |                         |
                        +-------------------------+
                                          |
                                          v
    +--------------------------------------------------+
    | HTML Generation                                  |
    |                                                  |
    | Input Tokens:                                     |
    | - keyword: "def"                                 |
    | - function: "hello"                              |
    | - punctuation: "()"                              |
    | - string: "'Hello...'"                           |
    |                                                  |
    | Process:                                          |
    | - Wrap each token in <span>                       |
    | - Add class attributes                           |
    | - Generate line structure                        |
    |                                                  |
    | Output HTML:                                      |
    | <span class="token-keyword">def</span>            |
    | <span class="token-function">hello</span>         |
    | <span class="token-punctuation">(</span>           |
    | <span class="token-punctuation">)</span>           |
    +--------------------------------------------------+
                        |
                        v
    Highlighted HTML:
    +--------------------------------------------------+
    | <span class="token-keyword">def</span>            |
    | <span class="token-function">hello</span>         |
    | <span class="token-punctuation">(</span>           |
    | <span class="token-punctuation">)</span>           |
    | <span class="token-operator"> </span>             |
    | <span class="token-keyword">print</span>          |
    | <span class="token-punctuation">(</span>           |
    | <span class="token-string">"Hello, World!"</span>  |
    | <span class="token-punctuation">)</span>           |
    +--------------------------------------------------+

Step 4: Code Block Assembly
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: text

    Highlighted HTML Input:
    +--------------------------------------------------+
    | <span class="token-keyword">def</span>...       |
    +--------------------------------------------------+
                        |
                        v
    +--------------------------------------------------+
    | CodeBlockRenderer                                |
    |                                                  |
    | 1. Generate HTML structure                      |
    | 2. Add header with language & copy button        |
    | 3. Add line numbers (if enabled)                 |
    | 4. Add highlighted code content                  |
    | 5. Add data attributes for styling               |
    +--------------------------------------------------+
                        |
                        v
    +--------------------------------------------------+
    | HTML Structure Assembly                         |
    |                                                  |
    | <div class="code-block"                          |
    |   data-language="python"                         |
    |   data-theme="one-dark"                         |
    |   data-line-count="2">                          |
    |                                                  |
    |   <div class="code-header">                      |
    |     <span class="code-language">PYTHON</span>    |
    |     <button class="code-copy-button">...         |
    |   </div>                                         |
    |                                                  |
    |   <div class="code-content">                     |
    |     <pre><code class="language-python">         |
    |       <span class="line-number">1</span>        |
    |       <span class="token-keyword">def</span>    |
    |       <span class="token-function">hello</span>  |
    |       ...                                        |
    |     </code></pre>                               |
    |   </div>                                         |
    | </div>                                           |
    +--------------------------------------------------+
                        |
                        v
    Complete HTML Output:
    +--------------------------------------------------+
    | <div class="code-block" ...>                     |
    |   <div class="code-header">...</div>             |
    |   <div class="code-content">                     |
    |     <pre><code class="language-python">         |
    |       ...highlighted tokens...                   |
    |     </code></pre>                               |
    |   </div>                                         |
    | </div>                                           |
    +--------------------------------------------------+

Step 5: Asset Generation
~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: text

    +--------------------------------------------------+
    |  Asset Pipeline                                  |
    |                                                  |
    |  Input: Configuration from peta.toml             |
    +--------------------------------------------------+
                        |
                        +---------------------+
                        |                     |
                        v                     v
    +----------------------------------+  +----------------------------------+
    | CssGenerator            |  | JsGenerator             |
    |                          |  |                          |
    | Process:                 |  | Process:                 |
    | 1. Load theme config     |  | 1. Define functions      |
    | 2. Generate token colors |  | 2. Add event listeners    |
    | 3. Generate container    |  | 3. Add keyboard shortcuts |
    | 4. Generate line numbers |  | 4. Add error handling     |
    | 5. Generate copy button  |  |                          |
    |                          |  |                          |
    | Output:                  |  | Output:                  |
    | code-blocks.css          |  | code-blocks.js           |
    +----------------------------------+  +----------------------------------+
                        |                     |
                        v                     v
    +--------------------------------------------------+
    | Generated Files                                  |
    |                                                  |
    | _out/dist/assets/css/code-blocks.css            |
    | - Token colors (keyword, string, etc.)           |
    | - Container styling (background, border)         |
    | - Line number styling                             |
    | - Copy button animations                         |
    | - Responsive breakpoints                         |
    |                                                  |
    | _out/dist/assets/js/code-blocks.js             |
    | - copyCode() function                             |
    | - Keyboard shortcuts                              |
    | - Event listeners                                 |
    | - Error handling                                 |
    +--------------------------------------------------+

Step 6: Template Integration
~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: text

    HTML from CodeBlockRenderer:
    +--------------------------------------------------+
    | <div class="code-block" ...>                     |
    |   <div class="code-header">...</div>             |
    |   <div class="code-content">...</div>            |
    | </div>                                           |
    +--------------------------------------------------+
                        |
                        v
    +--------------------------------------------------+
    | Template Engine                                  |
    |                                                  |
    | 1. Load base template                            |
    | 2. Inject code block HTML into content           |
    | 3. Add CSS reference to <head>                    |
    | 4. Add JS reference to <body>                     |
    | 5. Render final HTML                             |
    +--------------------------------------------------+
                        |
                        v
    +--------------------------------------------------+
    | Base Template Structure                         |
    |                                                  |
    | <html>                                           |
    |   <head>                                         |
    |     <link rel="stylesheet"                        |
    |           href="/assets/css/code-blocks.css">    |
    |   </head>                                        |
    |   <body>                                         |
    |     <div class="article-body">                    |
    |       <div class="code-block">...                |
    |     </div>                                         |
    |     <script src="/assets/js/code-blocks.js">    |
    |   </body>                                        |
    | </html>                                          |
    +--------------------------------------------------+
                        |
                        v
    Final HTML Output:
    +--------------------------------------------------+
    | Complete HTML page with:                        |
    | - Code block HTML (pre-rendered)                |
    | - CSS reference (generated)                      |
    | - JS reference (generated)                       |
    +--------------------------------------------------+

Step 7: Browser Rendering
~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: text

    Final HTML Page:
    +--------------------------------------------------+
    | Complete HTML with code block, CSS, JS refs     |
    +--------------------------------------------------+
                        |
                        v
    +--------------------------------------------------+
    | Browser                                         |
    |                                                  |
    | 1. Parse HTML                                    |
    | 2. Load CSS (code-blocks.css)                   |
    | 3. Load JS (code-blocks.js)                      |
    | 4. Render code block with syntax highlighting   |
    | 5. Apply CSS styles to tokens                    |
    | 6. Initialize JS interactions                    |
    +--------------------------------------------------+
                        |
                        v
    +--------------------------------------------------+
    | Rendered Page                                   |
    |                                                  |
    | - Code block with syntax highlighting           |
    | - Colored tokens (keywords, strings, etc.)       |
    | - Line numbers displayed                         |
    | - Copy button functional                         |
    | - Hover effects working                          |
    +--------------------------------------------------+

Component Interaction Diagram
-----------------------------

.. code-block:: text

    +-------------------+     +-------------------+
    |   RST Files       |     |  peta.toml        |
    |  (Content)        |     |  (Config)         |
    +---------+---------+     +---------+---------+
              |                         |
              |                         |
              v                         v
    +--------------------------------------------------+
    |              Rust Backend (peta)                   |
    |                                                      |
    |  +-------------------+  +-------------------+    |
    |  |    RstParser      |  |  Config Loader    |    |
    |  +---------+---------+  +---------+---------+    |
    |           |                         |            |
    |           v                         v            |
    |  +-------------------+  +-------------------+    |
    |  |   Directives      |  |  Theme Config     |    |
    |  +---------+---------+  +---------+---------+    |
    |           |                         |            |
    |           +-------------------------+            |
    |                     |                              |
    |                     v                              |
    |  +--------------------------------------------------+
    |  |         CodeBlockRenderer                        |
    |  |                                                    |
    |  |  +-------------------+  +-------------------+      |
    |  |  | SyntaxHighlighter  |  |  Asset Generators |      |
    |  |  |   (syntect)       |  |  (CSS/JS)         |      |
    |  |  +---------+---------+  +---------+---------+      |
    |  |           |                         |            |
    |  |           +-------------------------+            |
    |  |                     |                              |
    |  |                     v                              |
    |  |            HTML Output                           |
    |  +--------------------------------------------------+    |
    +--------------------------------------------------+
                          |
                          v
    +--------------------------------------------------+
    |            Template Engine (Tera)                   |
    +--------------------------------------------------+
                          |
                          v
    +--------------------------------------------------+
    |              Output Directory (_out/dist)           |
    |                                                      |
    |  +-------------------+  +-------------------+        |
    |  |   HTML Files      |  |   Assets          |        |
    |  |   (pages/*.html)  |  |                   |        |
    |  +-------------------+  |  +-----------------+  |       |
    |                           |  | code-blocks.css |  |       |
    |                           |  +-----------------+  |       |
    |                           |  | code-blocks.js  |  |       |
    |                           |  +-----------------+  |       |
    |                           +-------------------+       |
    +--------------------------------------------------+
                          |
                          v
    +--------------------------------------------------+
    |                   Browser                            |
    |                                                      |
    |  - Parses HTML                                       |
    |  - Loads CSS (code-blocks.css)                      |
    |  - Loads JS (code-blocks.js)                         |
    |  - Renders code blocks with syntax highlighting       |
    |  - Applies interactive features (copy, hover)         |
    +--------------------------------------------------+

Data Flow Diagram
-----------------

.. code-block:: text

    User Input (RST)
        |
        |  ".. code-block:: python\n    def hello():"
        v
    +-----------------------+
    |  RST File Content     |
    +-----------------------+
        |
        | Parse
        v
    +-----------------------+
    |  Parsed Directive     |
    |  type: "python"       |
    |  content: "def..."    |
    +-----------------------+
        |
        | Extract
        v
    +-----------------------+
    |  Language: "python"   |
    |  Code: "def..."       |
    +-----------------------+
        |
        | Highlight
        v
    +-----------------------+
    |  Tokenized Code       |
    |  - keyword: "def"     |
    |  - function: "hello"  |
    |  - string: "..."      |
    +-----------------------+
        |
        | Generate HTML
        v
    +-----------------------+
    |  HTML with Classes    |
    |  <span class="token- |
    |   keyword">def</span> |
    +-----------------------+
        |
        | Assemble
        v
    +-----------------------+
    |  Complete Code Block |
    |  <div class="code-   |
    |   block">...</div>    |
    +-----------------------+
        |
        | Inject
        v
    +-----------------------+
    |  Final HTML Page      |
    |  with CSS/JS refs    |
    +-----------------------+
        |
        | Output
        v
    +-----------------------+
    |  Browser Display      |
    |  Colored syntax       |
    |  Copy button          |
    |  Line numbers         |
    +-----------------------+

Token Conversion Flow
---------------------

.. code-block:: text

    Syntect Token Output
        |
        |  class="keyword storage type function"
        |  class="string quoted double"
        |  class="entity name function"
        |  class="comment line number-sign"
        v
    +-----------------------+
    |  convert_classes()    |
    |  Function             |
    +-----------------------+
        |
        | Apply Regex Patterns
        v
    +-----------------------+
    |  Pattern Matching     |
    |                      |
    |  1. "keyword.*" → "token-keyword"   |
    |  2. "string.*" → "token-string"     |
    |  3. "entity name function" → "token-function" |
    |  4. "comment.*" → "token-comment"   |
    |  5. "constant numeric" → "token-number" |
    |  6. "entity name type" → "token-type" |
    |  7. "variable.*" → "token-variable" |
    |  8. "punctuation.*" → "token-punctuation" |
    +-----------------------+
        |
        | Replace
        v
    +-----------------------+
    |  Token Classes        |
    |                      |
    |  token-keyword        |
    |  token-string         |
    |  token-function       |
    |  token-comment        |
    |  token-number         |
    |  token-type           |
    |  token-variable       |
    |  token-punctuation    |
    +-----------------------+
        |
        | Apply CSS
        v
    +-----------------------+
    |  Colored Output        |
    |  - Keywords: purple   |
    |  - Strings: green     |
    |  - Functions: blue    |
    |  - Comments: gray     |
    |  - Numbers: orange    |
    |  - Types: yellow      |
    +-----------------------+

Error Handling Flow
------------------

.. code-block:: text

    RST File
        |
        | Contains error?
        v
    +-----------------------+
    |  RstParser            |
    +-----------------------+
        |
        | Parse error?
        +------------------+
        | Yes               | No
        v                   |
    +-------------------+   |
    | Error: Invalid   |   |
    | directive format |   |
    +-------------------+   |
        |                   |
        v                   |
    +-----------------------+
    |  Return Error        |
    |  to user             |
    +-----------------------+
                            |
                            | Valid directive
                            v
                +-----------------------+
                |  Directive Handler    |
                +-----------------------+
                            |
                            | Language not supported?
                            +------------------+
                            | Yes               | No
                            v                   |
                    +-------------------+   |
                    | Error: Language |   |
                    | not supported   |   |
                    +-------------------+   |
                            |                   |
                            v                   |
                    +-----------------------+
                    |  Return Error        |
                    |  to user             |
                    +-----------------------+
                                        |
                                        | Language valid
                                        v
                            +-----------------------+
                            |  SyntaxHighlighter    |
                            +-----------------------+
                                        |
                                        | Highlighting error?
                                        +------------------+
                                        | Yes               | No
                                        v                   |
                                +-------------------+   |
                                | Error: Failed to |   |
                                | tokenize code    |   |
                                +-------------------+   |
                                        |                   |
                                        v                   |
                                +-----------------------+
                                |  Return Error        |
                                |  to user             |
                                +-----------------------+
                                                    |
                                                    | Success
                                                    v
                                    +-------------------------------+
                                    |  Return Highlighted HTML   |
                                    +-------------------------------+
                                                    |
                                                    v
                                    +-------------------------------+
                                    |  Template Engine           |
                                    +-------------------------------+
                                                    |
                                                    | Render error?
                                                    +------------------+
                                                    | Yes               | No
                                                    v                   |
                                            +-------------------+   |
                                            | Error: Failed to |   |
                                            | render template |   |
                                            +-------------------+   |
                                                    |                   |
                                                    v                   |
                                            +-----------------------+
                                            |  Return Error            |
                                            |  to user                 |
                                            +-----------------------+
                                                                |
                                                                | Success
                                                                v
                                                +-------------------------------+
                                                |  Final HTML Page           |
                                                +-------------------------------+

Performance Flow
-----------------

.. code-block:: text

    +-----------------------+
    |  Build Process         |
    |  (cargo build)         |
    +-----------------------+
            |
            | Compile Rust
            v
    +-----------------------+
    |  Binary Executable    |
    |  (peta)                |
    +-----------------------+
            |
            | Run build
            v
    +-----------------------+
    |  Asset Pipeline        |
    +-----------------------+
            |
            | Process RST files
            v
    +-----------------------+
    |  Parse All RST        |
    +-----------------------+
            |
            | Parse directives
            v
    +-----------------------+
    |  Highlight All Code   |
    |  (syntect)            |
    +-----------------------+
            |
            | Generate HTML
            v
    +-----------------------+
    |  Generate CSS         |
    |  (programmatically)   |
    +-----------------------+
            |
            | Generate JS
            v
    +-----------------------+
    |  Write All Files      |
    |  (static)             |
    +-----------------------+
            |
            v
    +-----------------------+
    |  Output Directory     |
    |  _out/dist/           |
    +-----------------------+
            |
            | Browser requests
            v
    +-----------------------+
    |  Serve Static Files   |
    |  (no processing)      |
    +-----------------------+
            |
            v
    +-----------------------+
    |  Instant Display      |
    |  (pre-rendered)       |
    +-----------------------+

Timing Diagram
-------------

.. code-block:: text

    RST File      Parser        Directive     Highlighter    Renderer     HTML Output
       |            |              |               |            |             |
       |-- write -->|              |               |            |             |
       |            |-- parse --->|               |            |             |
       |            |              |-- handle ---->|            |             |
       |            |              |               |-- tokenize>|            |
       |            |              |               |            |-- render -->|
       |            |              |               |            |             |
       |            |              |               |            |<-- output --|
       |            |              |               |            |             |

    Config       CSS Gen       JS Gen        Assets       Template    Final Page
       |            |              |               |            |             |
       |-- load -->|              |               |            |             |
       |            |-- generate -->|               |            |             |
       |            |              |-- generate -->|            |             |
       |            |              |               |-- write -->|             |
       |            |              |               |            |-- inject -->|
       |            |              |               |            |             |
       |            |              |               |            |<-- output --|
       |            |              |               |            |             |

Complete Pipeline Summary
-------------------------

.. code-block:: text

    1. CONTENT INPUT
       ├─ RST file with code-block directive
       └─ Configuration (peta.toml)
       
    2. PARSING
       ├─ RstParser reads RST file
       ├─ Identifies directives
       ├─ Extracts language and code
       └─ Validates input
       
    3. SYNTAX HIGHLIGHTING
       ├─ SyntaxHighlighter resolves language
       ├─ Syntect tokenizes code
       ├─ Generates HTML with syntect classes
       ├─ Converts to token classes
       └─ Returns highlighted HTML
       
    4. CODE BLOCK ASSEMBLY
       ├─ CodeBlockRenderer creates structure
       ├─ Adds header (language, copy button)
       ├─ Adds line numbers
       ├─ Inserts highlighted code
       └─ Adds data attributes
       
    5. ASSET GENERATION
       ├─ CssGenerator generates styles
       │  ├─ Token colors
       │  ├─ Container styling
       │  ├─ Line numbers
       │  └─ Copy button
       └─ JsGenerator generates interactions
          ├─ Copy function
          ├─ Keyboard shortcuts
          └─ Event listeners
       
    6. TEMPLATE INTEGRATION
       ├─ Template Engine loads base template
       ├─ Injects code block HTML
       ├─ Adds CSS reference
       └─ Adds JS reference
       
    7. OUTPUT
       ├─ HTML files (pre-rendered)
       ├─ CSS file (generated)
       └─ JS file (generated)
       
    8. BROWSER RENDERING
       ├─ Parses HTML
       ├─ Loads CSS
       ├─ Loads JS
       ├─ Applies styles to tokens
       ├─ Initializes interactions
       └─ Displays to user

Key Features
------------

1. **Pre-rendered HTML**: All syntax highlighting done at build time
2. **No Runtime Processing**: Browser just displays pre-rendered content
3. **Single CSS File**: All code block styles in one file
4. **Minimal JavaScript**: Only for interactions (copy, shortcuts)
5. **100+ Languages**: Supported by syntect
6. **Theme Support**: Multiple themes with custom colors
7. **Line Numbers**: Pre-rendered with hover effects
8. **Copy Functionality**: Built-in with clipboard API
9. **Keyboard Shortcuts**: Ctrl/Cmd+K to copy
10. **Responsive Design**: Mobile-friendly

Benefits
--------

.. code-block:: text

    Performance
    ├─ Build-time highlighting (no runtime cost)
    ├─ Smaller bundle sizes (minimal JS)
    ├─ Faster page loads (static HTML)
    └─ Better caching (immutable assets)
    
    Maintenance
    ├─ Single source of truth (Rust)
    ├─ Type-safe code (compile-time checks)
    ├─ Easier testing (deterministic output)
    └─ Better documentation (diagrams + code)
    
    User Experience
    ├─ Instant page loads
    ├─ Consistent styling
    ├─ Smooth interactions
    └─ Accessibility features