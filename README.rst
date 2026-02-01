Peta (Rust) - High-Performance RST-First Static Site Generator
=========================================================

Peta is a blazing-fast static site generator built with Rust that uses reStructuredText (RST) as its primary content format. Designed for technical documentation, blogs, and knowledge bases, Peta combines the power of Rust's performance with the expressiveness of RST markup.

.. image:: https://img.shields.io/badge/version-0.1.0-orange
   :target: https://github.com/h3x49r4m/peta-rust
   :alt: Version

.. image:: https://img.shields.io/badge/license-Apache--2.0-blue
   :target: LICENSE
   :alt: License

.. image:: https://img.shields.io/badge/rust-1.70+-orange
   :target: https://www.rust-lang.org
   :alt: Rust Version

.. image:: https://img.shields.io/badge/RST-reStructuredText-blue
   :alt: RST Support

Features
--------

**RST-First Architecture**
- Native support for reStructuredText with powerful directives
- Math formulas with KaTeX rendering (inline and display mode)
- Code blocks with syntax highlighting (20+ languages)
- Diagram generation (class, sequence, flowchart, state, gantt)
- Music score rendering (LilyPond notation)

**Component System**
- Modular, reusable components (atomic and composite)
- Dynamic component loading and discovery
- Theme-specific component registries
- Component versioning and configuration

**Modern Web Features**
- Responsive, mobile-first design
- Dark mode support (One Dark theme)
- Real-time development server with hot reload
- Client-side search with full-text indexing
- Asset minification and optimization

**Developer Experience**
- Fast build times (incremental compilation)
- Zero-config default setup
- Flexible configuration via ``peta.toml``
- Command-line interface with subcommands
- Comprehensive error reporting

**Deployment Ready**
- GitHub Pages integration
- Configurable base URL for different environments
- CDN-friendly static output
- CI/CD pipeline support

Quick Start
-----------

Installation
~~~~~~~~~~~~

**From Source**

.. code-block:: bash

   git clone https://github.com/h3x49r4m/peta-rust.git
   cd peta-rust
   cargo install --path .

**Using Cargo**

.. code-block:: bash

   cargo install peta

Basic Usage
~~~~~~~~~~~

1. **Create a new site** (optional - use existing structure):

.. code-block:: bash

   mkdir my-site && cd my-site
   peta init

2. **Add content** in ``_content/`` directory:

.. code-block:: bash

   # Create articles
   echo "
   My First Article
   ================

   This is my first article using Peta.
   
   .. code-block:: python
   
       print('Hello, World!')
   
   Math example: :math:`E = mc^2`

   " > _content/articles/hello.rst

3. **Build your site**:

.. code-block:: bash

   peta build

4. **Preview locally**:

.. code-block:: bash

   peta serve

   # Open browser
   open http://localhost:3566

5. **Build for GitHub Pages**:

.. code-block:: bash

   peta build --base-url "/your-repo-name"

Configuration
-------------

Peta uses a single ``peta.toml`` configuration file:

.. code-block:: toml

   [site]
   title = "My Site"
   description = "My awesome site"
   url = "https://username.github.io"
   author = "Your Name"
   base_url = ""  # Use "" for localhost, "/repo-name" for GitHub Pages

   [build]
   content_dir = "_content"
   output_dir = "_out/dist"
   theme_dir = "themes"
   drafts = false

   [server]
   port = 3566
   host = "127.0.0.1"
   livereload = true

   [search]
   enabled = true
   client_side = true

Key Configuration Options
--------------------------

**base_url** (Important)
   - ``""`` - Local development (default)
   - ``"/repo-name"`` - GitHub Pages deployment
   - ``"/custom/path"`` - Custom deployment path

**rst.code.theme**
   - ``one-dark`` - Dark theme (default)
   - ``solarized`` - Solarized theme

**math_rendering**
   - ``engine`` - ``katex`` (only supported)
   - ``auto_detect`` - Automatically detect and render math
   - ``on_demand_loading`` - Load KaTeX only when math is detected

**components.enabled**
   - ``true`` - Enable component system
   - ``enabled_components`` - List of active components

Content Structure
-----------------

Peta organizes content in the ``_content/`` directory:

.. code-block:: text

   _content/
   ├── articles/          # Blog posts and articles
   │   ├── calculus-fundamentals.rst
   │   └── quantum-mechanics.rst
   ├── books/             # Multi-page documentation
   │   ├── machine-learning-basics/
   │   │   ├── index.rst
   │   │   ├── introduction.rst
   │   │   └── supervised-learning.rst
   │   └── deep-learning-with-python/
   ├── projects/          # Project showcases
   │   ├── math-visualizer.rst
   │   └── quantum-simulator.rst
   └── snippets/          # Code snippets (with modal viewer)
       ├── python-data-processing.rst
       └── rust-concurrent-programming.rst

RST Directives
--------------

Peta supports powerful RST directives:

**Code Blocks**

.. code-block:: rst

   .. code-block:: python
      :line_numbers: true
      :theme: one-dark

      def hello():
          print("Hello, World!")

**Snippet Cards** (with modal viewer)

.. code-block:: rst

   .. snippet-card:: python-data-processing
      :title: Data Processing with Pandas
      :description: Clean and analyze data using pandas
      :tags: python, data-science

      .. code-block:: python

          import pandas as pd
          df = pd.read_csv('data.csv')
          print(df.head())

**Math Formulas**

.. code-block:: rst

   Inline math: :math:`E = mc^2`

   Display math:

   .. math::

       \int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}

**Diagrams**

.. code-block:: rst

   .. diagram:: class-diagram
      :type: class

      class User {
          +name: str
          +email: str
          +login()
          +logout()
      }

**Music Scores**

.. code-block:: rst

   .. music-score:: simple-melody
      :title: Simple Melody

      c'4 d' e' f' | g'2. |

**TOC (Table of Contents)**

.. code-block:: rst

   .. toctree::
      :maxdepth: 2
      :caption: Contents

      introduction
      getting-started
      advanced-features

Theme System
------------

Peta uses a modular theme system with components:

.. code-block:: text

   themes/
   └── default/
       ├── theme.yaml              # Theme configuration
       ├── assets/
       │   ├── css/                # Theme stylesheets
       │   └── js/                 # Theme JavaScript
       ├── components/             # Reusable components
       │   ├── atomic/             # Single-purpose components
       │   │   ├── navbar/
       │   │   ├── footer/
       │   │   └── site_stats/
       │   └── composite/          # Compound components
       │       ├── header/
       │       └── contacts/
       └── templates/              # Page templates
           ├── base.html
           ├── index.html
           ├── article.html
           └── snippet.html

Creating Components
-------------------

Components are reusable HTML snippets that can be used across templates.

**1. Create component directory**:

.. code-block:: bash

   mkdir -p themes/default/components/atomic/my_component

**2. Create component HTML**:

.. code-block:: html

   <!-- themes/default/components/atomic/my_component/my_component.html -->
   <div class="my-component" data-component="my_component">
       {% if props.title %}
       <h2>{{ props.title }}</h2>
       {% endif %}
       <div class="my-component-content">
           {{ props.content | default(value="Default content") }}
       </div>
   </div>

**3. Create component config** (optional):

.. code-block:: yaml

   # themes/default/components/atomic/my_component/my_component.yaml
   name: My Component
   description: A simple custom component
   version: 1.0.0
   props:
     title:
       type: string
       required: false
       default: ""
     content:
       type: string
       required: false
       default: ""

**4. Use component in template**:

.. code-block:: html

   {{ component(name="my_component", props={"title": "Hello", "content": "World"}) }}

For more details, see :doc:`docs/features/components/how_to_create_a_component`.

Commands
--------

``peta new``
~~~~~~~~~~~~

Create a new site.

.. code-block:: bash

   # Create a new site with default theme
   peta new --name my-site

   # Create a new site with specific theme
   peta new --name my-site --theme custom

``peta init``
~~~~~~~~~~~~

Initialize new content or site.

.. code-block:: bash

   # Initialize a new site
   peta init site --name my-site --theme default

   # Initialize new content
   peta init content --type article --title "My Article"
   peta init content --type book --title "My Book"
   peta init content --type snippet --title "My Snippet"
   peta init content --type project --title "My Project"

   # Initialize content with custom directory
   peta init content --type article --title "My Article" --content-dir custom-content

``peta build``
~~~~~~~~~~~~~

Build the static site.

.. code-block:: bash

   # Local development (default)
   peta build

   # GitHub Pages deployment
   peta build --base-url "/repo-name"

   # Custom content directory
   peta build --content-dir custom-content

   # Custom output directory
   peta build --output-dir ./dist

   # Use specific theme
   peta build --theme custom

   # Include draft content
   peta build --draft

   # Combine options
   peta build --base-url "/repo-name" --content-dir content --output-dir build --theme default --draft

``peta serve``
~~~~~~~~~~~~~~

Start development server with hot reload.

.. code-block:: bash

   # Default settings (localhost:3566)
   peta serve

   # Custom port
   peta serve --port 8080

   # Custom host
   peta serve --host 0.0.0.0

   # Open browser automatically
   peta serve --open

   # Use custom content directory
   peta serve --content-dir custom-content

   # Include draft content
   peta serve --draft

   # Combine options
   peta serve --port 8080 --host 0.0.0.0 --open --draft

``peta deploy``
~~~~~~~~~~~~~~

Deploy the site to a target platform.

.. code-block:: bash

   # Deploy to GitHub Pages (default)
   peta deploy

   # Deploy to specific target
   peta deploy --target github
   peta deploy --target netlify
   peta deploy --target vercel

``peta clean``
~~~~~~~~~~~~~

Clean build artifacts.

.. code-block:: bash

   # Clean only build artifacts
   peta clean

   # Clean all artifacts including output directory
   peta clean --all

``peta theme``
~~~~~~~~~~~~~

Theme management commands.

.. code-block:: bash

   # List available themes
   peta theme list

   # Create a new theme
   peta theme create --name my-theme
   peta theme create --name my-theme --base default

   # Validate theme configuration
   peta theme validate --name default

   # Show theme information
   peta theme info --name default

   # Install theme from repository
   peta theme install --source https://github.com/user/theme
   peta theme install --source https://github.com/user/theme --name my-theme

URL System
----------

Peta uses a centralized URL generation system that respects the ``base_url`` configuration.

**Template Functions**

- ``{{ url(path='page.html', base_url=base_url) }}`` - Generate page URLs
- ``{{ asset_url(path='css/style.css', base_url=base_url) }}`` - Generate asset URLs

**Usage in Templates**

.. code-block:: html

   <a href="{{ url(path='books.html', base_url=base_url) }}">Books</a>
   <link rel="stylesheet" href="{{ asset_url(path='css/main.css', base_url=base_url) }}">

**URL Behavior**

- Empty ``base_url`` → ``/books.html``
- ``base_url="/repo"`` → ``/repo/books.html``

For detailed documentation, see :doc:`docs/features/urls/urls_pipeline`.

Deployment
----------

GitHub Pages
~~~~~~~~~~~~

1. **Configure repository**:

.. code-block:: bash

   # Update peta.toml
   [site]
   base_url = "/your-repo-name"

2. **Build site**:

.. code-block:: bash

   peta build --base-url "/your-repo-name"

3. **Deploy using GitHub Actions**:

.. code-block:: yaml

   # .github/workflows/deploy.yml
   name: Deploy to GitHub Pages
   on:
     push:
       branches: [ main ]
   jobs:
     deploy:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v3
         - uses: actions-rs/toolchain@v1
           with:
             toolchain: stable
         - run: cargo install --path .
         - run: peta build --base-url "/your-repo-name"
         - uses: peaceiris/actions-gh-pages@v3
           with:
             github_token: ${{ secrets.GITHUB_TOKEN }}
             publish_dir: ./_out/dist

4. **Enable GitHub Pages** in repository settings

Custom Deployment
~~~~~~~~~~~~~~~~~

Peta generates pure static files in ``_out/dist/`` that can be deployed anywhere:

- Netlify
- Vercel
- AWS S3 + CloudFront
- Any static hosting service

Architecture
------------

**Core Modules**

- ``peta/src/core/`` - Site building logic
- ``peta/src/content/`` - RST parsing and processing
- ``peta/src/templates/`` - Template engine (Tera)
- ``peta/src/components/`` - Component system
- ``peta/src/assets/`` - Asset processing
- ``peta/src/search/`` - Full-text search (Tantivy)
- ``peta/src/server/`` - Development server (Axum)
- ``peta/src/deploy/`` - Deployment integrations

**Pipeline Flow**

.. code-block:: text

   Content (_content/)
        ↓
   RST Parser
        ↓
   Directive Processors
        ↓
   HTML Generator
        ↓
   Template Engine (Tera)
        ↓
   Component Renderer
        ↓
   Static Files (_out/dist/)

**Key Dependencies**

- ``tokio`` - Async runtime
- ``tera`` - Template engine
- ``pulldown-cmark`` - Markdown parser (for mixed content)
- ``katex`` - Math rendering
- ``syntect`` - Syntax highlighting
- ``tantivy`` - Full-text search
- ``axum`` - Web server

Documentation
-------------

**Feature Documentation**

- :doc:`docs/features/urls/urls_pipeline` - URL generation system
- :doc:`docs/features/components/component_pipeline` - Component architecture
- :doc:`docs/features/codeblocks/codeblock_pipeline` - Code block rendering
- :doc:`docs/features/diagrams/diagrams_pipeline` - Diagram generation
- :doc:`docs/features/math_formulas/math_formulas_pipeline` - Math rendering
- :doc:`docs/features/search/search_pipeline` - Search system

**Development Guides**

- :doc:`docs/features/components/how_to_create_a_component` - Component creation
- :doc:`docs/features/cli/cli_system` - CLI architecture
- :doc:`docs/features/rst_parser/rst_parsing_pipeline` - RST parsing

Performance
-----------

**Build Performance**

- Incremental compilation with Rust
- Parallel content processing
- Asset caching and optimization
- Template compilation caching

**Runtime Performance**

- Zero runtime dependencies (pure static HTML)
- Lazy-loaded JavaScript (KaTeX, search)
- Optimized assets (minified CSS/JS)
- CDN-friendly output

**Benchmarks**

- Build time for 100 pages: ~2 seconds
- Development server response: <10ms
- Search indexing: <100ms for 1000 documents

Contributing
------------

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

**Development Setup**

.. code-block:: bash

   # Clone repository
   git clone https://github.com/h3x49r4m/peta-rust.git
   cd peta-rust

   # Install development dependencies
   cargo install --all-features

   # Run tests
   cargo test

   # Run with debug logging
   RUST_LOG=debug cargo run -- build

**Code Style**

- Use ``rustfmt`` for formatting
- Use ``clippy`` for linting
- Write documentation for public APIs
- Add unit tests for new features

License
-------

Peta is licensed under the Apache License 2.0. See `LICENSE`_ for details.

.. _LICENSE: LICENSE

Acknowledgments
---------------

- **Rust community** - Excellent tooling and libraries
- **Tera** - Powerful template engine
- **KaTeX** - Fast math rendering
- **Syntect** - Syntax highlighting
- **Tantivy** - Full-text search engine

Support
-------

- **GitHub Issues**: https://github.com/h3x49r4m/peta-rust/issues
- **Documentation**: https://h3x49r4m.github.io/peta-rust
