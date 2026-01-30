Peta (Rust) - High-Performance Static Site Generator
====================================================

A modern static site generator written in Rust with component-based themes and RST-first architecture. Peta processes reStructuredText (RST) files directly to HTML, providing exceptional performance and flexibility.

.. code-block::

    ┌─────────────────────────────────────────────────────────────────┐
    │                    PETA - Static Site Generator                 │
    └─────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
    ┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
    │   RST Files     │───▶│  Component       │───▶│  Static HTML    │
    │   (_content/)   │    │  Rendering       │    │  (_out/dist/)   │
    └─────────────────┘    └──────────────────┘    └─────────────────┘

Features
========

Core Features
-------------

* **Component-Based Themes (V4)**: Atomic, composite, and content components with flexible theming system
* **RST-First Architecture**: Direct RST→HTML conversion with advanced parsing capabilities
* **Content CLI**: Initialize articles, books, snippets, and projects with template generation
* **Math Rendering**: KaTeX integration for LaTeX equations with automatic detection and fallback support
* **Code Highlighting**: Syntect-based syntax highlighting with 30+ language support, line numbers, and copy button
* **Development Server**: Live reload with file watching and WebSocket support on port 3566
* **Asset Processing**: CSS/JS minification and image optimization pipeline
* **Search Functionality**: Client-side search with metadata indexing, relevance scoring, and real-time filtering
* **Modern UI**: Clean, responsive design with component-based theming
* **Performance Optimized**: Rust-based performance with efficient compilation and serving

Advanced Features
-----------------

* **Embedded Snippet Cards**: Automatically embed code snippets from files with syntax highlighting
* **Cross-References**: Automatic linking between documents using RST ``:ref:`` directives
* **Table of Contents**: Auto-generated TOC with customizable depth and navigation
* **Book Support**: Multi-section book structure with chapter navigation
* **Project Portfolio**: Showcase projects with metadata and descriptions
* **Tag System**: Organize content with tags and tag clouds
* **Social Links**: Configurable social media integration
* **Deployment Tools**: GitHub Pages, Netlify, Vercel, and S3 deployment support
* **Component Scripts**: JavaScript hooks for interactive components
* **Theme Manager**: Dynamic theme switching and component loading

Quick Start
===========

Build From Source
-----------------

.. code-block:: bash

    # Clone repository
    git clone https://github.com/h3x49r4m/peta-rust.git
    cd peta-rust

    # Build the project (faster for development)
    cargo build --bin peta

    # Initialize new content
    cargo run --bin peta -- init article "Getting Started"
    cargo run --bin peta -- init snippet "Code Example"
    cargo run --bin peta -- init project "My Portfolio"

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
    ./target/release/peta init article "My Article"
    ./target/release/peta build
    ./target/release/peta serve

    # Or install globally for system-wide access
    cargo install --path .

    # Then use directly
    peta init article "My Article"
    peta build
    peta serve

Build Commands
==============

.. code-block:: bash

    # Initialize new content (article/book/snippet/project)
    cargo run --bin peta -- init article "My Article Title"
    cargo run --bin peta -- init book "My Book Title"
    cargo run --bin peta -- init snippet "My Snippet Title"
    cargo run --bin peta -- init project "My Project Title"

    # Build the site
    cargo run --bin peta -- build

    # Start development server (default port 3566)
    cargo run --bin peta -- serve

    # Start server on custom port
    cargo run --bin peta -- serve --port 8080

    # Start server and open browser
    cargo run --bin peta -- serve --port 3566 --open

    # Clean build artifacts
    cargo run --bin peta -- clean

Project Structure
=================

::

    peta-rust/
    ├── Cargo.toml                    # Workspace configuration
    ├── peta.toml                     # Site configuration
    ├── README.rst                    # This file
    ├── LICENSE                       # Apache 2.0 License
    ├── Makefile                      # Build automation
    ├── _content/                     # Content directory
    │   ├── articles/                 # Blog posts and articles
    │   ├── books/                    # Multi-section books
    │   ├── projects/                 # Portfolio projects
    │   └── snippets/                 # Code snippets gallery
    ├── themes/                       # Theme system (V4)
    │   └── default/
    │       ├── README.md             # Theme documentation
    │       ├── theme.yaml            # Theme configuration
    │       ├── components/           # Component-based themes
    │       │   ├── atomic/           # Atomic components (buttons, inputs, etc.)
    │       │   │   ├── search_bar/
    │       │   │   ├── search_results/
    │       │   │   ├── tag_cloud/
    │       │   │   └── ...
    │       │   ├── composite/        # Composite components (header, footer, etc.)
    │       │   │   ├── header/
    │       │   │   ├── footer/
    │       │   │   └── ...
    │       │   └── content/          # Content components
    │       ├── templates/            # HTML templates
    │       │   ├── base.html         # Base template
    │       │   ├── index.html        # Homepage
    │       │   ├── search.html       # Search page
    │       │   ├── articles.html     # Articles listing
    │       │   ├── article.html      # Article detail
    │       │   └── ...
    │       └── assets/               # Theme assets
    │           ├── css/              # Stylesheets
    │           └── js/               # JavaScript
    ├── _out/                         # Generated static site
    │   └── dist/                     # Production build output
    ├── docs/                         # Documentation
    │   ├── architecture/             # Architecture documentation
    │   └── features/                 # Feature documentation
    │       ├── cli/
    │       ├── codeblocks/
    │       ├── components/
    │       ├── css/
    │       ├── embedded_snippet_cards/
    │       ├── math_formulas/
    │       └── search/
    ├── examples/                     # Example content
    ├── peta/                         # Main package
    │   ├── Cargo.toml                # Package configuration
    │   └── src/
    │       ├── main.rs               # CLI entry point
    │       ├── lib.rs                # Core library
    │       ├── cli/                  # Command-line interface
    │       │   ├── args.rs           # CLI argument parsing
    │       │   ├── commands.rs       # CLI commands
    │       │   ├── component_commands.rs  # Component commands
    │       │   ├── mod.rs            # CLI module
    │       │   └── output.rs         # Output handling
    │       ├── components/           # Component system
    │       │   ├── config.rs         # Component configuration
    │       │   ├── discovery.rs      # Component discovery
    │       │   ├── loader.rs         # Component loader
    │       │   ├── manager.rs        # Component manager
    │       │   ├── mod.rs            # Components module
    │       │   ├── registry.rs       # Component registry
    │       │   ├── renderer.rs       # Component renderer
    │       │   ├── theme.rs          # Theme management
    │       │   └── version.rs        # Version management
    │       ├── content/              # Content processing
    │       │   ├── metadata.rs       # Content metadata
    │       │   ├── mod.rs            # Content module
    │       │   ├── resolver.rs       # Content resolver
    │       │   ├── taxonomy.rs       # Taxonomy management
    │       │   └── rst/              # RST processing
    │       │       ├── book_toc_generator.rs
    │       │       ├── cross_ref.rs
    │       │       ├── directives.rs
    │       │       ├── mod.rs
    │       │       ├── parser.rs
    │       │       ├── toc_generator.rs
    │       │       ├── code_blocks/
    │       │       ├── embedded_snippet_cards/
    │       │       └── math_formulas/
    │       ├── core/                 # Core engine
    │       │   ├── builder.rs        # Site builder
    │       │   ├── config.rs         # Configuration
    │       │   ├── error.rs          # Error types
    │       │   ├── mod.rs            # Core module
    │       │   ├── site.rs           # Site structure
    │       │   └── theme.rs          # Theme handling
    │       ├── deploy/               # Deployment tools
    │       │   ├── github.rs         # GitHub Pages deployment
    │       │   ├── mod.rs            # Deploy module
    │       │   ├── netlify.rs        # Netlify deployment
    │       │   ├── s3.rs             # S3 deployment
    │       │   └── vercel.rs         # Vercel deployment
    │       ├── search/               # Search functionality
    │       │   ├── indexer.rs        # Search indexer
    │       │   ├── mod.rs            # Search module
    │       │   ├── query.rs          # Query processing
    │       │   └── ranking.rs        # Ranking algorithm
    │       ├── server/               # Development server
    │       │   ├── dev_server.rs     # Dev server implementation
    │       │   ├── file_watcher.rs   # File watching
    │       │   ├── livereload.rs     # Live reload
    │       │   ├── mod.rs            # Server module
    │       │   ├── websocket.rs      # WebSocket support
    │       │   └── static/           # Static files
    │       ├── templates/            # Template engine
    │       │   ├── engine.rs         # Template engine
    │       │   ├── filters.rs        # Template filters
    │       │   ├── functions.rs      # Template functions
    │       │   ├── mod.rs            # Templates module
    │       │   └── renderer.rs       # Template renderer
    │       ├── assets/               # Asset processing
    │       │   ├── css_generator.rs  # CSS generation
    │       │   ├── css.rs            # CSS utilities
    │       │   ├── images.rs         # Image processing
    │       │   ├── js_generator.rs   # JS generation
    │       │   ├── js.rs             # JS utilities
    │       │   ├── minifier.rs       # Asset minification
    │       │   ├── mod.rs            # Assets module
    │       │   └── pipeline.rs       # Asset pipeline
    │       └── utils/                # Utilities
    │           ├── cache.rs          # Caching utilities
    │           ├── file.rs           # File utilities
    │           ├── http.rs           # HTTP utilities
    │           ├── mod.rs            # Utils module
    │           └── progress.rs       # Progress reporting
    └── tests/                        # Test suite
        └── component_discovery_integration.rs

Configuration
=============

Create a ``peta.toml`` file in your project root:

.. code-block:: toml

    [site]
    title = "Peta"
    description = "High-Performance Static Site Generator"
    url = "https://example.com"
    author = "Peta Team"

    [social]
    github = "https://github.com/username/repo"
    x = "https://x.com/username"
    email = "user@example.com"

    [components]
    enabled = true
    enabled_components = ["footer", "contacts", "navbar", "header"]
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

    [math_rendering]
    engine = "katex"
    auto_detect = true
    on_demand_loading = true

    [math_rendering.katex]
    version = "0.16.9"
    cdn_base = "https://cdn.jsdelivr.net/npm/katex"

    [code_blocks]
    default_theme = "one-dark"
    enable_line_numbers = true
    enable_copy_button = true
    enable_keyboard_shortcuts = true
    enable_line_hover = true

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

Architecture
============

Peta follows an RST-first architecture with component-based theming:

* **V4 Component System**: Atomic, composite, and content components with flexible composition
* **Direct RST Processing**: No intermediate JSON conversion, pure RST→HTML pipeline
* **Template Engine**: Tera-based with component support and custom filters
* **Asset Pipeline**: Integrated CSS/JS processing, minification, and optimization
* **Search System**: Client-side search with metadata indexing and relevance scoring
* **Math Rendering**: KaTeX integration with automatic detection and fallback to MathJax
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

For detailed information about the search pipeline, see `docs/features/search/search_pipeline.rst`.

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
