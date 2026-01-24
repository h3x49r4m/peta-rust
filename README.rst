Peta (Rust) - High-Performance Static Site Generator
==========================================

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
    git clone https://github.com/h3x49r4m/peta-rust.git
    cd peta-rust

    # Build the project
    cargo build --bin peta

    # Build the site
    cargo run --bin peta -- build

    # Start development server
    cargo run --bin peta -- serve --port 3566

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

    [server]
    port = 3566
    host = "localhost"
    auto_open = false
    livereload = true

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
