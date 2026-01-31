Peta (Rust) - High-Performance Static Site Generator
====================================================

A modern static site generator written in Rust with RST-first architecture and component-based theming. Peta processes reStructuredText (RST) files directly to HTML, providing exceptional performance and flexibility.

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

* **Embedded Snippet Cards**: Inline snippet rendering with automatic heading hierarchy adjustment
* **Diagrams**: Flowchart, Gantt, sequence, class, and state diagrams with Rust-based SVG rendering
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
---------------------

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

Build From Source
-----------------

If you already have a peta project:

.. code-block:: bash

    # Build peta
    make build-peta

    # Build the site
    make build

    # Start development server
    make serve

    # Clean build artifacts
    make clean

    # Show available commands
    make help

Deployment Options
==================

GitHub Pages Deployment
------------------------

Automated deployment using GitHub Actions on every push to deploy branch:

**Setup:**

1. Create GitHub Actions workflow (already included in ``.github/workflows/deploy.yml``)
2. Enable GitHub Pages: Go to repository Settings → Pages → Build and deployment → Source: GitHub Actions
3. Push changes to the ``deploy`` branch - deployment happens automatically

**How base_url works:**

The ``--base-url`` CLI option allows you to specify the base URL for different environments without modifying ``peta.toml``:

* **Local Development**: Use default empty ``base_url``

  .. code-block:: bash

      peta build

  URLs resolve as ``/books.html``, ``/snippets.html``, etc.

* **GitHub Pages**: Specify the repository name as ``base_url``

  .. code-block:: bash

      peta build --base-url "/your-repo-name"

  URLs resolve as ``/your-repo-name/books.html``, ``/your-repo-name/snippets.html``, etc.

  The GitHub Actions workflow in ``.github/workflows/deploy.yml`` automatically uses this option during deployment:

  .. code-block:: yaml

      - name: Build site
        run: peta build --base-url "/your-repo-name"

**Workflow features:**
- Caches Cargo registry, index, and build artifacts for faster builds
- Builds peta from source
- Generates the site in ``_out/dist``
- Deploys to GitHub Pages automatically
- Uses ``--base-url`` option for proper URL generation

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

Build and Development Commands
-------------------------------

.. code-block:: bash

    # Build the site
    peta build

    # Build with custom content directory
    peta build --content-dir /path/to/content

    # Build with custom output directory
    peta build --output /path/to/output

    # Build with custom theme
    peta build --theme mytheme

    # Build with base URL (for GitHub Pages or subdirectory deployment)
    peta build --base-url "/your-repo-name"

    # Build including draft content
    peta build --draft

    # Start development server (default port 3566)
    peta serve

    # Start server on custom port
    peta serve --port 8080

    # Start server on specific host
    peta serve --host 0.0.0.0

    # Start server and open browser
    peta serve --open

    # Serve with draft content
    peta serve --draft

Theme Management Commands
---------------------------

.. code-block:: bash

    # List available themes
    peta theme list

    # Create a new theme
    peta theme create mytheme

    # Validate theme structure
    peta theme validate mytheme

    # Get theme information
    peta theme info mytheme

    # Install a theme
    peta theme install /path/to/theme

Cleanup Commands
-----------------

.. code-block:: bash

    # Clean build artifacts
    peta clean

    # Clean all artifacts including cache
    peta clean --all

Configuration
=============

Create a ``peta.toml`` file in your project root:

.. code-block:: toml

    [site]
    title = "Peta"
    description = "High-Performance Static Site Generator"
    url = "https://example.com"
    author = "Peta Team"
    base_url = ""  # Default base URL, can be overridden via --base-url CLI option

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

    [search]
    enabled = true
    client_side = true
    index_content = true
    index_metadata = true

    [server]
    port = 3566
    host = "127.0.0.1"
    open_browser = true
    livereload = true

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

Blog posts and articles with date-based organization.

**Frontmatter:**

.. code-block:: yaml

    ---
    title: "Getting Started with Peta"
    date: "2026-01-30T10:00:00"
    tags: ["tutorial", "getting-started"]
    author: "Author Name"
    ---

**Content:**

.. code-block:: rst

    Introduction
    ============

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

**Reference:**

.. _features: #features

See the Features section above.

Books
-----

Multi-section books with chapter navigation and book-specific TOC.

**Frontmatter:**

.. code-block:: yaml

    ---
    title: "My Book"
    date: "2026-01-30T00:00:00"
    tags: ["book", "guide"]
    author: "Author Name"
    description: "A comprehensive guide"
    ---

**Content:**

.. code-block:: rst

    My Book
    =======

    Chapter 1
    ---------

    Introduction content.

    Chapter 2
    ---------

    More content.

    .. toctree::
       :maxdepth: 2

       chapter1
       chapter2
       chapter3

Snippets
--------

Reusable code snippets with language categorization and embedding support.

**Frontmatter:**

.. code-block:: yaml

    ---
    title: "Rust Async/Await Pattern"
    date: "2026-01-30T00:00:00"
    tags: ["rust", "async", "pattern"]
    snippet_id: "rust-async-await"
    author: "Author Name"
    ---

**Content:**

.. code-block:: rst

    Async/Await Pattern
    ~~~~~~~~~~~~~~~~~~~~

    .. code-block:: rust

        async fn fetch_data() -> Result<String, Error> {
            Ok("data".to_string())
        }

Projects
--------

Project portfolio entries with metadata and links.

**Frontmatter:**

.. code-block:: yaml

    ---
    title: "My Awesome Project"
    date: "2026-01-30T00:00:00"
    tags: ["project", "web", "rust"]
    author: "Author Name"
    github_url: "https://github.com/username/project"
    demo_url: "https://project.example.com"
    excerpt: "Brief description"
    ---

**Content:**

.. code-block:: text

    Project Description
    ~~~~~~~~~~~~~~~~~~~~

    Describe your project here.

    Features
    --------

    List key features.

RST Directives
================

Peta supports a comprehensive set of RST directives for advanced content creation.

Code Blocks
~~~~~~~~~~~

Syntax-highlighted code blocks with copy button and line numbers:

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

.. code-block:: text

    .. snippet-card:: my-snippet-id

Use this to embed a snippet by its ``snippet_id`` field.

Math Formulas
~~~~~~~~~~~~~

LaTeX math rendering with KaTeX:

**Inline math:** ``$E = mc^2$``

**Display math:**

.. code-block:: text

    $$\int_0^1 f(x) dx$$

Diagrams
~~~~~~~~

Create various types of diagrams using Rust-based SVG rendering:

**Flowchart:**

.. code-block:: text

    .. diagram:: flowchart
        :title: Deployment Process

**Gantt Chart:**

.. code-block:: text

    .. diagram:: gantt
        :title: Project Timeline

**Sequence Diagram:**

.. code-block:: text

    .. diagram:: sequence
        :title: Request Flow

**Class Diagram:**

.. code-block:: text

    .. diagram:: class
        :title: User System

**State Diagram:**

.. code-block:: text

    .. diagram:: state
        :title: Page Lifecycle

Music Scores
~~~~~~~~~~~~

ABC notation support for music score rendering:

.. code-block:: text

    .. musicscore:: melody
        :title: Simple Melody

Cross-References
~~~~~~~~~~~~~~~~

Automatic linking between documents:

.. code-block:: text

    See :ref:`features` for more information.

Architecture
============

Project Structure
-----------------

::

    peta-rust/
    ├── peta/                    # Core library
    │   ├── src/
    │   │   ├── cli/            # Command-line interface
    │   │   ├── components/     # Component system
    │   │   ├── content/        # Content processing
    │   │   │   └── rst/        # RST parser and directives
    │   │   ├── core/           # Core functionality
    │   │   ├── templates/      # Template engine (Tera)
    │   │   ├── assets/         # Asset processing
    │   │   ├── search/         # Search indexing
    │   │   └── server/         # Development server
    │   └── Cargo.toml
    ├── _content/                # Content directory
    │   ├── articles/
    │   ├── books/
    │   ├── snippets/
    │   └── projects/
    ├── themes/                  # Theme directory
    │   └── default/
    ├── _out/                    # Output directory
    │   └── dist/               # Generated HTML
    ├── docs/                    # Documentation
    ├── tests/                   # Tests
    ├── peta.toml                # Configuration
    └── README.rst

Component System
----------------

Peta V4 uses a component-based architecture inspired by React:

* **Atomic Components**: Single-purpose components (buttons, tags, icons)
* **Composite Components**: Complex components built from atomic components (cards, modals, tables)
* **Content Components**: Content-aware components (article TOC, book navigation, search)

Documentation
=============

For detailed documentation, see the `docs/` directory:

* CLI System (`docs/features/cli/`)
* Code Blocks (`docs/features/codeblocks/`)
* Components (`docs/features/components/`)
* Diagrams (`docs/features/diagrams/`)
* Math Formulas (`docs/features/math_formulas/`)
* Music Scores (`docs/features/music_scores/`)
* RST Parsing (`docs/features/rst_parser/`)
* Search (`docs/features/search/`)
* Embedded Snippet Cards (`docs/features/embedded_snippet_cards/`)

Testing
=======

Run tests:

.. code-block:: bash

    # Run all tests
    cargo test

    # Run tests with output
    cargo test -- --nocapture

    # Run specific test
    cargo test test_name

Performance
===========

* **Fast Compilation**: Rust's zero-cost abstractions and efficient compiler
* **Incremental Builds**: Only rebuild what changed
* **Asset Caching**: Intelligent caching for CSS, JS, and images
* **Lazy Loading**: On-demand loading for heavy resources (math, search)

License
=======

This project is licensed under the MIT License - see the LICENSE file for details.

Contributing
============

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (``git checkout -b feature/AmazingFeature``)
3. Commit your changes (``git commit -m 'Add some AmazingFeature'``)
4. Push to the branch (``git push origin feature/AmazingFeature``)
5. Open a Pull Request

Acknowledgments
===============

* Rust community for excellent tooling and libraries
* reStructuredText specification and community
* Tera template engine
* Syntect syntax highlighting
* KaTeX math rendering
* Tantivy search engine
