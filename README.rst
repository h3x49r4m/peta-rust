Peta - High-Performance Static Site Generator
==========================================

A modern static site generator written in Rust with component-based themes and RST-first architecture. Peta processes reStructuredText (RST) files directly to HTML, providing exceptional performance and flexibility.

.. raw:: html

    <div style="text-align: center; margin: 20px 0;">
        <pre style="display: inline-block; text-align: left; font-family: monospace; background: #1e293b; color: #e2e8f0; padding: 20px; border-radius: 8px;">
┌─────────────────────────────────────────────────────────────────┐
│                    PETA - Static Site Generator                   │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   RST Files     │───▶│  Component       │───▶│  Static HTML    │
│   (_content/)   │    │  Rendering       │    │  (_out/dist/)    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
        </pre>
    </div>

Features
========

* **Component-Based Themes**: V4 architecture with atomic, composite, and content components
* **RST-First Architecture**: Direct RST→HTML conversion with advanced parsing capabilities
* **Math Rendering**: KaTeX integration for LaTeX equations with fallback support
* **Code Highlighting**: Syntect-based syntax highlighting with comprehensive language support
* **Development Server**: Live reload with file watching and WebSocket support on port 3566
* **Asset Processing**: CSS/JS minification and image optimization pipeline
* **Search Functionality**: Metadata-based search indexing for fast content discovery
* **Modern UI**: Clean, responsive design with component-based theming
* **Performance Optimized**: Rust-based performance with efficient compilation and serving

Quick Start
===========

Build From Source
-----------------

.. code-block:: bash

    # Clone repository
    git clone <repository-url>
    cd peta-rust

    # Build the project
    cargo build --bin peta

    # Build the site
    cargo run --bin peta -- build

    # Start development server
    cargo run --bin peta -- serve --port 3566

Project Structure
================

::

    peta-rust/
    ├── Cargo.toml                    # Workspace configuration
    ├── peta.toml                     # Site configuration
    ├── _content/                     # Content directory
    │   ├── articles/                 # Blog posts and articles
    │   ├── snippets/                 # Code snippets gallery
    │   ├── books/                    # Multi-section books
    │   └── projects/                 # Portfolio projects
    ├── themes/                       # Theme system (V4)
    │   └── default/
    │       ├── components/           # Component-based themes
    │       │   ├── atomic/          # Atomic components
    │       │   ├── composite/       # Composite components
    │       │   ├── content/         # Content components
    │       │   └── layouts/         # Layout components
    │       ├── templates/            # HTML templates
    │       ├── css/                  # Stylesheets
    │       └── js/                   # JavaScript
    ├── _out/                         # Generated static site
    ├── peta/                         # Main package
    │   ├── src/
    │   │   ├── main.rs               # CLI entry point
    │   │   ├── lib.rs                # Core library
    │   │   ├── cli/                  # Command-line interface
    │   │   ├── core/                 # Core engine
    │   │   ├── content/              # Content processing (RST)
    │   │   ├── components/           # Component system
    │   │   ├── templates/            # Template engine
    │   │   ├── server/               # Development server
    │   │   ├── assets/               # Asset processing
    │   │   ├── search/               # Search functionality
    │   │   ├── deploy/               # Deployment tools
    │   │   └── utils/                # Utilities
    │   └── Cargo.toml               # Package configuration
    └── tests/                        # Test suite

Build Commands
==============

.. code-block:: bash

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

    peta_rust/
    ├── Cargo.toml                    # Workspace configuration
    ├── peta_rust.toml               # Site configuration
    ├── _content/                     # Content directory
    │   ├── articles/                 # Blog posts and documentation
    │   ├── snippets/                 # Code snippets gallery
    │   ├── books/                    # Multi-section books
    │   └── projects/                 # Portfolio projects
    ├── _themes/                       # Theme system
    │   └── default/
    │       ├── templates/            # HTML templates
    │       ├── css/                  # Stylesheets
    │       └── js/                   # JavaScript
    ├── _dist/                         # Generated static site
    ├── peta/                         # Main package
    │   ├── src/
    │   │   ├── main.rs               # CLI entry point
    │   │   ├── lib.rs                # Core library
    │   │   ├── cli/                  # Command-line interface
    │   │   ├── core/                 # Core engine
    │   │   ├── content/              # Content processing
    │   │   ├── templates/            # Template system
    │   │   ├── server/               # Development server
    │   │   ├── assets/               # Asset processing
    │   │   ├── search/               # Search functionality
    │   │   ├── deploy/               # Deployment tools
    │   │   └── utils/                # Utilities
    │   └── Cargo.toml               # Package configuration
    └── tests/                        # Test suite

Configuration
=============

Create a ``peta.toml`` file in your project root:

.. code-block:: toml

    [site]
    title = "Peta"
    description = "High-Performance Static Site Generator"
    url = "https://example.com"
    author = "Peta Team"

    [build]
    content_dir = "_content"
    output_dir = "_out/dist"
    theme_dir = "themes"
    drafts = false

    [content]
    default_directives = ["code-block", "note", "warning"]
    math_renderer = "katex"
    code_highlighter = "syntect"
    toc_depth = 3
    cross_references = true

    [content.math]
    katex_delimiters = ["$$", "$", "\\[", "\\]"]
    fallback_mathjax = true
    cache_rendered = true

    [content.code]
    line_numbers = true
    copy_button = true
    theme = "one-dark"

    [search]
    enabled = true
    index_content = true
    index_metadata = true

    [server]
    port = 3566
    host = "127.0.0.1"
    livereload = true
    open_browser = false

    [assets]
    minify_css = true
    minify_js = true
    optimize_images = true
    image_quality = 85

Content Types
=============

Articles
--------

Single RST files with automatic table of contents:

.. code-block:: rst

    ---
    title: "Getting Started with PETA_RUST"
    date: "2023-01-01"
    tags: ["tutorial", "rust"]
    author: "Peta Team"
    ---

    # Getting Started with PETA_RUST

    This is a comprehensive guide to getting started with PETA_RUST.

    ## Installation

    First, install PETA_RUST using cargo:

    .. code-block:: bash

        cargo install peta-rust

    ## Usage

    Create a new site with the following command:

    .. code-block:: bash

        peta new my-site

Books
-----

Multi-file RST books with toctree navigation:

.. code-block:: rst

    # Advanced Rust Programming

    .. toctree::
       :maxdepth: 2
       :caption: Contents:

       introduction
       memory-management
       concurrency
       advanced-patterns

Snippets
--------

Code gallery with enhanced display:

.. code-block:: rst

    ---
    title: "Rust Iterator Chain"
    language: "rust"
    tags: ["rust", "functional", "iterator"]
    ---

    .. code-block:: rust

        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter()
            .filter(|&n| n % 2 == 0)
            .map(|&n| n * 2)
            .sum();

Projects
--------

Portfolio showcases with metadata:

.. code-block:: rst

    ---
    title: "PETA_RUST Static Site Generator"
    type: "project"
    date: "2023-01-01"
    tags: ["rust", "static-site-generator", "rst"]
    url: "https://github.com/peta-rust/peta"
    ---

    A high-performance static site generator written in Rust that processes
    reStructuredText files directly to HTML.

    Features include:
    - RST-first architecture
    - Math rendering with KaTeX
    - Code highlighting with Syntect
    - Client-side search
    - Multiple deployment targets

CLI Commands
============

.. code-block:: bash

    # Build the site
    peta build

    # Start development server
    peta serve

    # Start server on custom port
    peta serve --port 8080

    # Start server and open browser
    peta serve --open

    # Clean build artifacts
    peta clean

    # Build with verbose output
    peta build --verbose

    # Include drafts in build
    peta build --drafts

Component-Based Themes
========================

Peta uses a component-based theme system with three types of components:

Atomic Components
-----------------

Basic UI elements that can be composed into larger structures:

.. code-block:: html

    <!-- Button component -->
    <div class="btn btn-primary">Click me</div>

Composite Components
------------------

Complex components that combine atomic components:

.. code-block:: html

    <!-- Navigation bar component -->
    <nav class="navbar">
        <div class="navbar-brand">Site Name</div>
        <ul class="navbar-menu">
            <li><a href="/">Home</a></li>
            <li><a href="/articles">Articles</a></li>
        </ul>
    </nav>

Layout Components
-----------------

Page-level layouts that structure the entire page:

.. code-block:: html

    <!-- Homepage layout -->
    <div class="homepage-layout">
        <header>
            <!-- Hero section -->
        </header>
        <main>
            <!-- Main content -->
        </main>
        <footer>
            <!-- Footer section -->
        </footer>
    </div>

Theme Structure
---------------

.. code-block:: bash

    themes/default/
    ├── components/
    │   ├── atomic/              # Atomic components
    │   │   ├── button/
    │   │   ├── input/
    │   │   └── card/
    │   ├── composite/           # Composite components
    │   │   ├── navbar/
    │   │   ├── footer/
    │   │   └── hero/
    │   └── layouts/             # Layout components
    │       ├── homepage/
    │       ├── articles/
    │       └── books/
    ├── templates/              # Page templates
    ├── css/                    # Component styles
    └── theme.yaml              # Theme configuration

Component Configuration
---------------------

Each component has a ``component.yaml`` configuration:

.. code-block:: yaml

    name: "navbar"
    version: "1.0.0"
    category: "Composite"
    description: "Navigation bar component"
    props:
      variant:
        type: string
        enum: ["horizontal", "vertical"]
        default: "horizontal"
    slots:
      - name: brand
      - name: menu
    templates:
      - navbar.html
    styles:
      - navbar.css

Development
===========

Building and Running
-------------------

.. code-block:: bash

    # Build for development
    cargo build

    # Run with cargo
    cargo run --bin peta -- build
    cargo run --bin peta -- serve

    # Run tests
    cargo test

    # Run with release optimizations
    cargo build --release
    cargo run --release --bin peta -- serve

Development Server Features
--------------------------

* **Live Reload**: Automatic browser refresh on file changes
* **File Watching**: Monitors _content directory for changes
* **WebSocket Support**: Real-time communication with browser
* **Default Port**: Runs on port 3566 (configurable)
* **Auto-open**: Optional browser opening on server start

Content Organization
===================

Articles
--------

Single RST files with frontmatter metadata:

.. code-block:: rst

    ---
    title: "Getting Started with Peta"
    date: "2024-01-01"
    tags: ["tutorial", "rust"]
    author: "Peta Team"
    ---

    # Getting Started with Peta

    This is a comprehensive guide to getting started with Peta.

    ## Installation

    First, build the project from source:

    .. code-block:: bash

        cargo build --bin peta

Books
-----

Multi-file RST books with navigation:

.. code-block:: rst

    # Advanced Rust Programming

    .. toctree::
       :maxdepth: 2
       :caption: Contents:

       introduction
       memory-management
       concurrency
       advanced-patterns

Snippets
--------

Code gallery with enhanced display:

.. code-block:: rst

    ---
    title: "Rust Iterator Chain"
    language: "rust"
    tags: ["rust", "functional", "iterator"]
    ---

    .. code-block:: rust

        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter()
            .filter(|&n| n % 2 == 0)
            .map(|&n| n * 2)
            .sum();

Projects
--------

Portfolio showcases with metadata:

.. code-block:: rst

    ---
    title: "Peta Static Site Generator"
    type: "project"
    date: "2024-01-01"
    tags: ["rust", "static-site-generator", "rst"]
    url: "https://github.com/peta-rust/peta"
    ---

    A high-performance static site generator written in Rust that processes
    reStructuredText files directly to HTML.

    Features include:
    - Component-based theme system
    - RST-first architecture
    - Math rendering with KaTeX
    - Code highlighting with Syntect
    - Development server with live reload
    - Modern responsive design

Architecture
============

Peta follows an RST-first architecture with component-based theming:

* **V4 Component System**: Atomic, composite, and layout components
* **Direct RST Processing**: No intermediate JSON conversion
* **Template Engine**: Tera-based with component support
* **Asset Pipeline**: Integrated CSS/JS processing and optimization
* **Search System**: Metadata-based indexing for fast queries

For detailed information about the architecture and design decisions, see the documentation in the ``docs/`` directory.

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

License
=======

This project is licensed under the Apache License 2.0 - see the ``LICENSE`` file for details.
