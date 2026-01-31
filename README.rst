Peta (Rust) - High-Performance Static Site Generator
====================================================

A modern static site generator written in Rust with RST-first architecture, component-based theming (V4), and advanced rendering capabilities. Peta processes reStructuredText (RST) files directly to HTML, providing exceptional performance and flexibility.

::

    ┌─────────────────────────────────────────────────────────────────────┐
    │                        PETA - CLI Interface                         │
    └─────────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
    ┌─────────────────────────────────────────────────────────────────────┐
    │                      SiteBuilder (Orchestrator)                     │
    │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐               │
    │  │ RST Parser   │  │ Template     │  │ Asset        │               │
    │  │ (Direct      │→ │ Engine       │→ │ Pipeline     │               │
    │  │  RST→HTML)   │  │ (Tera-based) │  │ (Minify/Opt) │               │
    │  └──────────────┘  └──────────────┘  └──────────────┘               │
    │         │                  │                  │                     │
    │         ▼                  ▼                  ▼                     │
    │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐               │
    │  │ Content      │  │ Component    │  │ Search       │               │
    │  │ Resolver     │  │ Manager      │  │ Indexer      │               │
    │  └──────────────┘  └──────────────┘  └──────────────┘               │
    └─────────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
                        ┌─────────────────┐
                        │  Static HTML    │
                        │  (_out/dist/)   │
                        └─────────────────┘

Features
========

Core Features
-------------

* **RST-First Architecture**: Direct RST→HTML conversion without intermediate JSON, pure build-time processing
* **Component-Based Themes (V4)**: React-inspired architecture with atomic, composite, and content components
* **Site Initialization**: Create new sites with complete peta source code and theme included
* **Content CLI**: Initialize articles, books, snippets, and projects with template generation
* **Math Rendering**: KaTeX integration for LaTeX equations with automatic detection and on-demand loading
* **Code Highlighting**: Syntect-based syntax highlighting with 30+ language support, line numbers, and copy button
* **Development Server**: Live reload with file watching and WebSocket support on port 3566
* **Asset Processing**: CSS/JS minification and image optimization pipeline
* **Search Functionality**: Client-side search with Tantivy indexing, metadata support, and relevance scoring
* **Performance Optimized**: Rust-based performance with efficient compilation and serving

Advanced Features
-----------------

* **Embedded Snippet Cards**: Inline snippet rendering using ``.. snippet-card::<id>`` with automatic heading hierarchy adjustment
* **Diagrams**: Flowchart, Gantt, sequence, class, and state diagrams with Rust-based SVG rendering and download support
* **Music Scores**: ABC notation support with SVG rendering, staff lines, clefs, and multi-voice support
* **Cross-References**: Automatic linking between documents using RST ``:ref:`` directives
* **Table of Contents**: Auto-generated TOC with customizable depth and navigation
* **Book Support**: Multi-section book structure with chapter navigation and book-specific TOC
* **Project Portfolio**: Showcase projects with metadata, GitHub URLs, and demo links
* **Tag System**: Organize content with tags and tag clouds
* **Social Links**: Configurable social media integration
* **Theme Variables**: CSS custom properties for light/dark mode and comprehensive theming
* **Component Scripts**: JavaScript hooks for interactive components
* **Theme Manager**: Dynamic theme switching and component loading

Quick Start
===========

Initialize a New Site
--------------------

.. code-block:: bash

    # Clone repository
    git clone https://github.com/h3x49r4m/peta-rust.git
    cd peta-rust

    # Build the project
    cargo build --release

    # Initialize a new site (includes peta source code and theme)
    ./_out/target/release/peta init site myblog

    # Navigate to the new site
    cd myblog

    # Build peta from source
    make build-peta

    # Initialize new content
    ./target/release/peta init content article "Getting Started"
    ./target/release/peta init content snippet "Code Example"
    ./target/release/peta init content project "My Portfolio"

    # Build the site
    make build

    # Start development server
    make serve

Build From Source (Existing Project)
------------------------------------

.. code-block:: bash

    # Clone repository
    git clone https://github.com/h3x49r4m/peta-rust.git
    cd peta-rust

    # Build the project (faster for development)
    cargo build --bin peta

    # Initialize new content
    cargo run --bin peta -- init content article "Getting Started"
    cargo run --bin peta -- init content snippet "Code Example"
    cargo run --bin peta -- init content project "My Portfolio"

    # Build the site
    cargo run --bin peta -- build

    # Start development server
    cargo run --bin peta -- serve --port 3566

Performance Tips
----------------

For faster development workflow:

.. code-block:: bash

    # Compile once in release mode for optimal performance
    cargo build --release

    # Use the compiled binary directly (much faster than cargo run)
    ./target/release/peta init content article "My Article"
    ./target/release/peta build
    ./target/release/peta serve

    # Or install globally for system-wide access
    cargo install --path .

    # Then use directly
    peta init content article "My Article"
    peta build
    peta serve

Deployment Options
===================

Standalone Site (Self-Contained)
---------------------------------

Use ``peta init site`` to create a self-contained site with peta source code included:

.. code-block:: bash

    peta init site myblog
    cd myblog
    make build-peta
    ./target/release/peta init content article "My Article"
    make build
    make serve

**Pros:**
- Self-contained, no external dependencies
- Easy to version control the entire site
- No need to manage peta updates separately

**Cons:**
- Larger repository size (includes peta source)
- Need to rebuild peta when updating

Git Submodule (Shared peta)
-----------------------------

Use peta as a git submodule to share the peta binary across multiple sites:

.. code-block:: bash

    # Create a new site directory
    mkdir myblog
    cd myblog

    # Initialize git repository
    git init

    # Add peta as a git submodule
    git submodule add https://github.com/h3x49r4m/peta-rust.git peta-rust

    # Create content directory
    mkdir -p _content/{articles,books,projects,snippets}

    # Create peta.toml configuration
    # (see Configuration section below)

    # Clone themes (optional)
    git clone https://github.com/h3x49r4m/peta-rust-themes.git themes

    # Initialize content
    ./peta-rust/target/release/peta init content article "My Article"

    # Build the site
    ./peta-rust/target/release/peta build --content-dir _content

    # Serve the site
    ./peta-rust/target/release/peta serve --content-dir _content

**Updating peta submodule:**

.. code-block:: bash

    # Pull latest changes from peta submodule
    cd peta-rust
    git pull origin main

    # Rebuild peta
    cargo build --release

    # Go back to site root
    cd ..

    # Build and serve
    ./peta-rust/target/release/peta serve --content-dir _content

**Pros:**
- Smaller repository size
- Share peta updates across multiple sites
- Easier to update peta globally

**Cons:**
- Requires managing submodule updates
- Slightly more complex setup

GitHub Pages Deployment
------------------------

Automated deployment using GitHub Actions on every push to main branch:

**Setup:**

1. Set ``base_url`` in ``peta.toml`` for GitHub Pages (e.g., ``base_url = "/your-repo-name"``)
2. Create GitHub Actions workflow (already included in ``.github/workflows/deploy.yml``)
3. Enable GitHub Pages: Go to repository Settings → Pages → Build and deployment → Source: GitHub Actions
4. Push changes to the ``deploy`` branch - deployment happens automatically

**How base_url works:**

The ``base_url`` configuration in ``peta.toml`` automatically adjusts URLs for different environments:

* **Local Development**: Set ``base_url = ""`` in ``peta.toml``

  .. code-block:: toml

      [site]
      base_url = ""

  URLs resolve as ``/books.html``, ``/snippets.html``, etc.

* **GitHub Pages**: Set ``base_url = "/your-repo-name"`` in ``peta.toml``

  .. code-block:: toml

      [site]
      base_url = "/your-repo-name"

  URLs resolve as ``/your-repo-name/books.html``, ``/your-repo-name/snippets.html``, etc.

  The GitHub Actions workflow in ``.github/workflows/deploy.yml`` automatically updates this value during deployment using:

  .. code-block:: yaml

      - name: Set base_url for GitHub Pages
        run: sed -i 's/base_url = ""/base_url = "\/your-repo-name"/' peta.toml

**Workflow features:**
- Caches Cargo registry, index, and build artifacts for faster builds
- Builds peta from source
- Generates the site in ``_out/dist``
- Deploys to GitHub Pages automatically
- Automatically sets ``base_url`` during deployment

Build Commands
==============

Site Initialization Commands
----------------------------

.. code-block:: bash

    # Initialize a new site with default theme
    peta init site myblog

    # Initialize a new site with custom theme
    peta init site --theme custom myblog

Content Creation Commands
--------------------------

.. code-block:: bash

    # Initialize new content (article/book/snippet/project)
    peta init content article "My Article Title"
    peta init content book "My Book Title"
    peta init content snippet "My Snippet Title"
    peta init content project "My Project Title"

    # Initialize content with custom content directory
    peta init content article "My Article" --content-dir /path/to/content
    peta init content book "My Book" --content-dir /path/to/content
    peta init content snippet "My Snippet" -d /path/to/content
    peta init content project "My Project" -d /path/to/content

Build and Serve Commands
-------------------------

.. code-block:: bash

    # Build the site
    peta build

    # Build with custom content directory (for git submodule setup)
    peta build --content-dir _content

    # Build with custom output directory
    peta build --output custom_output

    # Build with custom theme
    peta build --theme custom_theme

    # Include draft content
    peta build --draft

    # Start development server (default port 3566)
    peta serve

    # Serve with custom content directory (for git submodule setup)
    peta serve --content-dir _content

    # Start server on custom port
    peta serve --port 8080

    # Start server on custom host
    peta serve --host 0.0.0.0

    # Start server and open browser
    peta serve --open

    # Serve draft content
    peta serve --draft

    # Clean build artifacts
    peta clean

    # Clean all artifacts including cache
    peta clean --all

Theme Management Commands
---------------------------

.. code-block:: bash

    # List available themes
    peta theme list

    # Create a new theme
    peta theme create mytheme --base default

    # Validate theme configuration
    peta theme validate mytheme

    # Show theme information
    peta theme info mytheme

    # Install theme from repository
    peta theme install https://github.com/user/theme.git

Makefile Commands (New Sites)
-----------------------------

When you create a new site with ``peta init site``, a Makefile is included:

.. code-block:: bash

    # Build peta from source
    make build-peta

    # Build the site
    make build

    # Start development server
    make serve

    # Clean build artifacts
    make clean

    # Show available commands
    make help

Configuration
=============

Create a ``peta.toml`` file in your project root:

.. code-block:: toml

    [site]
    title = "Peta"
    description = "High-Performance Static Site Generator"
    url = "https://example.com"
    author = "Peta Team"
    base_url = ""  # Set to "" for local dev, "/your-repo-name" for GitHub Pages

    [social]
    github = "https://github.com/username/repo"
    x = "https://x.com/username"
    email = "user@example.com"

    [components]
    enabled = true
    enabled_components = ["footer", "contacts", "navbar", "header"]
    component_configs = {}
    layout = "default"
    theme = "default"

    [build]
    content_dir = "_content"
    output_dir = "_out/dist"
    theme_dir = "themes"
    drafts = false

    [rst]
    default_directives = ["code-block", "snippet-card", "toctree"]
    math_renderer = "katex"
    code_highlighter = "syntect"
    toc_depth = 3
    cross_references = true

    [rst.math]
    katex_delimiters = ["$$", "$", "\\[", "\\]"]
    fallback_mathjax = true
    cache_rendered = true

    [math_rendering]
    engine = "katex"
    auto_detect = true
    on_demand_loading = true

    [math_rendering.katex]
    version = "0.16.9"
    cdn_base = "https://cdn.jsdelivr.net/npm/katex"

    [math_rendering.css]
    theme = "default"
    font_scale = 1.0
    line_height = 1.5

    [math_rendering.js]
    auto_render = true
    debug_mode = false
    modal_support = true

    [code_blocks]
    default_theme = "one-dark"
    enable_line_numbers = true
    enable_copy_button = true
    enable_keyboard_shortcuts = true
    enable_line_hover = true

    [code_blocks.languages]
    python = { aliases = ["py", "python3"] }
    javascript = { aliases = ["js", "node"] }
    typescript = { aliases = ["ts", "tsx"] }
    rust = { aliases = ["rs"] }
    go = { aliases = [] }
    bash = { aliases = ["sh", "shell", "zsh"] }
    ruby = { aliases = ["rb"] }
    php = { aliases = [] }
    java = { aliases = [] }
    kotlin = { aliases = ["kt"] }
    scala = { aliases = [] }
    csharp = { aliases = ["cs"] }
    cpp = { aliases = ["c++", "cxx", "cc", "hpp"] }
    c = { aliases = ["h"] }
    sql = { aliases = [] }
    html = { aliases = ["htm"] }
    xml = { aliases = [] }
    css = { aliases = ["scss", "sass", "less"] }
    json = { aliases = [] }
    yaml = { aliases = ["yml"] }
    toml = { aliases = [] }
    dockerfile = { aliases = [] }
    makefile = { aliases = [] }
    cmake = { aliases = [] }
    diff = { aliases = ["patch"] }
    log = { aliases = [] }

    [code_blocks.themes]
    one-dark = { name = "One Dark", description: "Dark theme based on Atom's One Dark" }
    solarized = { name = "Solarized", description: "Precision colors for solarized theme" }

    [code_blocks.styles]
    font_family = "SF Mono, Monaco, 'Inconsolata', 'Roboto Mono', 'Source Code Pro', monospace"
    font_size = "0.9rem"
    line_height = "1.6"
    border_radius = "1rem"
    background_gradient_start = "#1e293b"
    background_gradient_end = "#0f172a"
    header_background = "rgba(15, 23, 42, 0.6)"
    copy_success_duration = 2000

    [server]
    port = 3566
    host = "127.0.0.1"
    open_browser = true
    livereload = true

    [search]
    enabled = true
    client_side = true
    index_content = true
    index_metadata = true

    [assets]
    minify_css = true
    minify_js = true
    optimize_images = true
    image_quality = 85

    [deploy]
    target = "github"
    branch = "gh-pages"
    domain = "username.github.io"

Content Types
=============

Articles
--------

Blog posts and articles with automatic TOC generation and cross-reference support.

**Frontmatter:**

.. code-block:: yaml

    ---
    title: "Article Title"
    date: "2026-01-30T00:00:00"
    tags: ["tag1", "tag2"]
    author: "Author Name"
    excerpt: "Brief description"
    ---

**Example:**

.. code-block:: rst

    Getting Started
    ===============

    Introduction
    ------------

    Write your article content here.

    Features
    --------

    * Feature one
    * Feature two

    Code Example
    ~~~~~~~~~~~

    .. code-block:: rust

        fn main() {
            println!("Hello, world!");
        }

    .. snippet-card:: example-snippet

    .. _features: #features

    See the Features section above.

Books
-----

Multi-section books with chapter navigation and book-specific TOC.

**Frontmatter:**

.. code-block:: yaml

    ---
    title: "Book Title"
    date: "2026-01-30T00:00:00"
    tags: ["tag1", "tag2"]
    author: "Author Name"
    description: "A brief description of the book"
    ---

**Example:**

.. code-block:: rst

    My Book
    =======

    This book provides comprehensive coverage of the topic.

    .. toctree::
       :maxdepth: 2
       :caption: Contents:

       chapter1
       chapter2
       chapter3

    What This Book Covers
    ---------------------

    - Topic 1
    - Topic 2
    - Topic 3

Snippets
--------

Code snippets that can be embedded in other content using ``.. snippet-card::<id>``.

**Frontmatter:**

.. code-block:: yaml

    ---
    title: "Snippet Title"
    date: "2026-01-30T00:00:00"
    tags: [language, topic]
    ---

**Example:**

.. code-block:: rst

    Python Example
    ==============

    .. code-block:: python

        def example():
            print("Hello, world!")
            return True

Projects
--------

Project portfolio entries with metadata and links.

**Frontmatter:**

.. code-block:: yaml

    ---
    title: "Project Title"
    date: "2026-01-30T00:00:00"
    tags: ["tag1", "tag2"]
    author: "Author Name"
    github_url: "https://github.com/username/project"
    demo_url: "https://project.example.com"
    ---

RST Directives
================

Peta supports a comprehensive set of RST directives for advanced content creation.

Code Blocks
~~~~~~~~~~~

Syntax-highlighted code blocks with copy button and line numbers:

.. code-block:: rst

    .. code-block:: rust
        :line-numbers:
        :copy-button:

        fn main() {
            println!("Hello, world!");
        }

**Supported Languages:** Python, JavaScript, TypeScript, Rust, Go, Bash, Ruby, PHP, Java, Kotlin, Scala, C#, C++, C, SQL, HTML, XML, CSS, JSON, YAML, TOML, Dockerfile, Makefile, CMake, Diff, and more.

Embedded Snippet Cards
~~~~~~~~~~~~~~~~~~~~~~

Inline snippet rendering with automatic heading hierarchy adjustment:

.. code-block:: rst

    .. snippet-card:: my-snippet-id

This will:
- Resolve the snippet by ID across the site
- Display the snippet with its metadata
- Adjust heading levels (h1→h3, h2→h4, etc.) to fit the context

Math Formulas
~~~~~~~~~~~~

LaTeX equation rendering with KaTeX:

.. code-block:: rst

    Inline math: $E = mc^2$

    Display math:

    .. math::

        \int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}

**Features:**
- Automatic math detection
- On-demand loading (only loads KaTeX when math is detected)
- Custom delimiters: ``$$`` for display, ``$`` for inline
- Fallback to MathJax support

Diagrams
~~~~~~~~

Create various types of diagrams with Rust-based SVG rendering:

**Flowchart:**

.. code-block:: rst

    .. diagram:: flowchart
        :title: Deployment Process

        A[Start] --> B[Build]
        B --> C[Test]
        C --> D[Deploy]
        C -->|Fail| B

**Gantt Chart:**

.. code-block:: rst

    .. diagram:: gantt
        :title: Project Timeline

        task1 : 2024-01-01, 3d
        task2 : after task1, 5d
        task3 : after task2, 2d

**Sequence Diagram:**

.. code-block:: rst

    .. diagram:: sequence
        :title: Request Flow

        User->>Server: Request
        Server->>Database: Query
        Database-->>Server: Result
        Server-->>User: Response

**Class Diagram:**

.. code-block:: rst

    .. diagram:: class
        :title: User System

        class User {
            +name: str
            +email: str
            +login()
            +logout()
        }

        class Session {
            +token: str
            +expires: datetime
            +validate()
        }

        User "1" -- "*" Session : has

**State Diagram:**

.. code-block:: rst

    .. diagram:: state
        :title: Page Lifecycle

        [*] --> Draft
        Draft --> Published : publish
        Published --> Archived : archive
        Published --> Draft : unpublish

Music Scores
~~~~~~~~~~~~

Create music scores using ABC notation:

.. code-block:: rst

    .. musicscore:: melody
        :title: Simple Melody

        X:1
        T:Andante
        K:C
        C D E F | G A B c | c B A G | F E D C

**Features:**
- ABC notation parsing
- SVG rendering
- Staff lines, clefs, key signatures
- Multi-voice support
- Download as SVG

Table of Contents
~~~~~~~~~~~~~~~~

Auto-generated TOC with customizable depth:

.. code-block:: rst

    .. toctree::
       :maxdepth: 2
       :caption: Contents:

       chapter1
       chapter2
       chapter3

Cross-References
~~~~~~~~~~~~~~~~

Link between documents using RST ``:ref:`` directives:

.. code-block:: rst

    .. _features:

    Features Section
    ===============

    See the :ref:`introduction` section for more details.

    .. _introduction:

    Introduction
    ============

    This is the introduction. See :ref:`features` for more.

Architecture
============

Peta follows an RST-first architecture with component-based theming:

* **RST-First Processing**: Direct RST→HTML conversion without intermediate JSON structures
* **V4 Component System**: Atomic, composite, and content components with props, slots, and state management
* **Template Engine**: Tera-based with component support, custom filters, and functions
* **Asset Pipeline**: Integrated CSS/JS processing, minification, and optimization
* **Search System**: Client-side search with Tantivy indexing, metadata support, and relevance scoring
* **Math Rendering**: KaTeX integration with automatic detection and on-demand loading
* **Code Highlighting**: Syntect-based with 30+ language support and customizable themes
* **Development Server**: Live reload with file watching and WebSocket support

Documentation
=============

Detailed documentation is available in the ``docs/`` directory:

* ``docs/features/cli/`` - CLI commands and usage
* ``docs/features/codeblocks/`` - Code block rendering pipeline
* ``docs/features/components/`` - Component system architecture
* ``docs/features/css/`` - CSS processing and styling
* ``docs/features/embedded_snippet_cards/`` - Embedded snippet cards feature
* ``docs/features/math_formulas/`` - Math rendering pipeline
* ``docs/features/search/`` - Search pipeline and implementation
* ``docs/architecture/`` - Overall system architecture

For detailed information about specific features, see the corresponding documentation files.

Testing
=======

.. code-block:: bash

    # Run all tests
    cargo test

    # Run with output
    cargo test -- --nocapture

    # Run specific test modules
    cargo test --lib peta::core
    cargo test --lib peta::content

Development Workflow
===================

1. Make changes to source code
2. Run tests to ensure functionality
3. Build the project: ``cargo build``
4. Test the site: ``cargo run --bin peta -- serve``
5. Verify changes work as expected

Performance
==========

Peta is optimized for performance:

* **Rust Performance**: Native compilation and zero-cost abstractions
* **Efficient Parsing**: Direct RST→HTML conversion
* **Asset Optimization**: CSS/JS minification and image processing
* **Fast Development Server**: Live reload with minimal rebuild times
* **Component Caching**: Reusable component rendering

Optimization Tips
-----------------

For optimal performance during development:

1. **Use Release Build**: Compile once with ``cargo build --release`` and use the binary directly
2. **Avoid Repeated Compilation**: Use ``./target/release/peta serve`` instead of ``cargo run -- serve``
3. **Incremental Builds**: Cargo caches compiled modules, only recompiles changed files
4. **Parallel Processing**: Multi-core compilation for faster builds
5. **Asset Caching**: Static assets are cached between builds

The development server is fast but initial compilation may take time. For the best experience, compile once and use the binary directly.

License
=======

This project is licensed under the Apache License 2.0 - see the ``LICENSE`` file for details.
