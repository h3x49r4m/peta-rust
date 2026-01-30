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
* **Site Initialization**: Create new sites with complete peta source code and theme included
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
    mkdir _content
    mkdir _content/articles
    mkdir _content/books
    mkdir _content/projects
    mkdir _content/snippets

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

Build and Serve Commands
-------------------------

.. code-block:: bash

    # Build the site
    peta build

    # Build with custom content directory (for git submodule setup)
    peta build --content-dir _content

    # Start development server (default port 3566)
    peta serve

    # Serve with custom content directory (for git submodule setup)
    peta serve --content-dir _content

    # Start server on custom port
    peta serve --port 8080

    # Start server and open browser
    peta serve --port 3566 --open

    # Clean build artifacts
    peta clean

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