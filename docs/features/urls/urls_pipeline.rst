URL Generation Pipeline
========================

Overview
--------

The URL generation pipeline in peta-rust is a centralized system that ensures all URLs throughout the application respect the ``base_url`` configuration. This pipeline handles URL generation for both local development (``--base-url ""``) and production deployments like GitHub Pages (``--base-url "/peta-rust"``).

Pipeline Architecture
---------------------

The URL generation pipeline consists of three main layers:

1. **Core Utility Layer** (``peta/src/utils/url.rs``)
2. **Template Function Layer** (``peta/src/templates/engine.rs``)
3. **Component Rendering Layer** (``peta/src/components/``)

Flow Diagram
-------------

.. code-block:: text

    Command Line (--base-url)
            ↓
    SiteConfig (base_url field)
            ↓
    SiteBuilder (with config)
            ↓
    TemplateEngine (receives config)
            ↓
    ┌─────────────────────────────────────┐
    │  URL Generation Functions           │
    ├─────────────────────────────────────┤
    │  • build_url() [Rust code]          │
    │  • url() [Template function]        │
    │  • asset_url() [Template function]  │
    └─────────────────────────────────────┘
            ↓
    Templates (access via context)
            ↓
    Generated HTML (with correct URLs)

Phase 1: Configuration Input
-----------------------------

1.1 Command Line Arguments
~~~~~~~~~~~~~~~~~~~~~~~~~~

The pipeline starts with command line arguments in ``peta/src/cli/commands.rs``:

.. code-block:: rust

    pub async fn build_site(
        content_dir: Option<String>,
        output_dir: Option<String>,
        theme: Option<String>,
        base_url: String,  // <-- CLI argument
        draft: bool,
        output: &mut OutputFormatter
    ) -> Result<()>

1.2 Configuration Loading
~~~~~~~~~~~~~~~~~~~~~~~~~~

The config is loaded and updated in ``peta/src/cli/commands.rs`` (lines 192-212):

.. code-block:: rust

    let mut config = SiteConfig::load_from_file("peta.toml")?;
    
    // Override base_url if provided from CLI
    if !base_url.is_empty() {
        config.site.base_url = base_url;  // <-- Update config
    }
    
    let mut builder = crate::core::SiteBuilder::new(config);

**Key Point**: The CLI argument overrides the ``peta.toml`` config.

Phase 2: SiteBuilder Initialization
-----------------------------------

2.1 Builder Creation
~~~~~~~~~~~~~~~~~~~~

``SiteBuilder::new()`` in ``peta/src/core/builder.rs`` (line 24-50):

.. code-block:: rust

    pub fn new(config: SiteConfig) -> Self {
        Self {
            config,  // <-- Stores the updated config
            rst_content: Vec::new(),
            theme_system,
            search_index: SearchIndex::new(),
            component_registry,
        }
    }

2.2 Template Engine Creation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**Critical Fix**: TemplateEngine now receives the correct config.

In ``peta/src/core/builder.rs`` (line 240):

.. code-block:: rust

    let template_engine = TemplateEngine::new_with_components(
        &theme,
        self.component_registry.clone(),
        self.config.clone()  // <-- Pass the correct config with CLI overrides
    )?;

**Before Fix**: ``TemplateEngine::new_with_components()`` loaded config from ``peta.toml`` directly, ignoring CLI overrides.

**After Fix**: The config is passed from ``SiteBuilder``, which has the CLI overrides applied.

Phase 3: Template Engine Setup
-------------------------------

3.1 Engine Initialization
~~~~~~~~~~~~~~~~~~~~~~~~~~

``TemplateEngine::new_with_config()`` in ``peta/src/templates/engine.rs`` (line 210-245):

.. code-block:: rust

    pub fn new_with_config(theme: &Theme, config: crate::core::config::SiteConfig) -> Result<Self> {
        let mut tera = Tera::default();
        filters::register(&mut tera);
        functions::register(&mut tera);
        
        let theme_dir = theme.path().to_path_buf();
        let component_manager = Arc::new(RwLock::new(ComponentManager::new(&theme_dir)));
        
        // Register component functions WITH config
        Self::register_component_functions(&mut tera, &component_manager, &config);
        Self::register_theme_functions(&mut tera);
        Self::load_templates(&mut tera, &theme.templates_dir)?;
        
        Ok(Self {
            tera,
            theme_dir,
            config,  // <-- Store config in engine
            component_registry: None,
            component_manager,
            component_renderer: None,
            theme_manager: None,
            current_theme: None,
        })
    }

3.2 Component Function Registration
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

``register_component_functions()`` in ``peta/src/templates/engine.rs`` (line 255-330):

.. code-block:: rust

    fn register_component_functions(
        tera: &mut Tera,
        component_manager: &Arc<RwLock<ComponentManager>>,
        config: &crate::core::config::SiteConfig
    ) {
        let config_clone = config.clone();  // <-- Clone config for closure
        
        tera.register_function(
            "component",
            Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
                // ... component rendering logic ...
                
                // Build context WITH base_url
                let context = Self::build_component_context(
                    component_name,
                    &props,
                    &tag_collector,
                    &config_clone  // <-- Use the correct config
                );
                
                // ... render component ...
            })
        );
    }

3.3 Context Building
~~~~~~~~~~~~~~~~~~~~~

``build_component_context()`` in ``peta/src/templates/engine.rs`` (line 578-630):

.. code-block:: rust

    fn build_component_context(
        component_name: &str,
        props: &Value,
        tag_collector: &Arc<RwLock<TagCollector>>,
        config: &crate::core::config::SiteConfig
    ) -> Context {
        let mut context = Context::new();
        context.insert("props", props);
        context.insert("config", &config);
        
        // ... other context variables ...
        
        // ADD base_url TO CONTEXT
        context.insert("base_url", &config.site.base_url);  // <-- Critical line
        
        context
    }

**Key Point**: ``base_url`` is inserted into the component context, making it available to all templates.

Phase 4: URL Generation Functions
----------------------------------

4.1 Core Utility: ``build_url()``
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Location: ``peta/src/utils/url.rs``

.. code-block:: rust

    pub fn build_url(base_url: &str, path: &str) -> String {
        let clean_path = path.trim_start_matches('/');
        
        if base_url.is_empty() {
            format!("/{}", clean_path)
        } else {
            format!("{}/{}", base_url.trim_end_matches('/'), clean_path)
        }
    }

**Behavior**:

- Empty ``base_url`` → ``/path/to/file``
- Non-empty ``base_url`` → ``/base-url/path/to/file``
- Handles leading slashes in path
- Handles trailing slashes in base_url

4.2 Template Function: ``url()``
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Location: ``peta/src/templates/engine.rs`` (line 1065-1083)

.. code-block:: rust

    tera.register_function(
        "url",
        Box::new(|args: &HashMap<String, Value>| -> tera::Result<Value> {
            let path = args.get("0")
                .or_else(|| args.get("path"))
                .and_then(|v| v.as_str())
                .ok_or_else(|| tera::Error::msg("URL path is required"))?;
            
            // Get base_url from function arguments or default to empty
            let base_url = args.get("base_url")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            let clean_path = path.trim_start_matches('/');
            
            // Support external URLs (http/https)
            let url = if clean_path.starts_with("http") {
                clean_path.to_string()
            } else {
                crate::utils::url::build_url(base_url, path)
            };
            
            Ok(Value::String(url))
        })
    );

**Usage in Templates**:

.. code-block:: html

    <a href="{{ url(path='books.html', base_url=base_url) }}">Books</a>

4.3 Template Function: ``asset_url()``
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Location: ``peta/src/templates/engine.rs`` (line 1052-1063)

.. code-block:: rust

    tera.register_function(
        "asset_url",
        Box::new(|args: &HashMap<String, Value>| -> tera::Result<Value> {
            let path = args.get("0")
                .or_else(|| args.get("path"))
                .and_then(|v| v.as_str())
                .ok_or_else(|| tera::Error::msg("Asset path is required"))?;
            
            let base_url = args.get("base_url")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            let url = crate::utils::url::build_url(base_url, path);
            Ok(Value::String(url))
        })
    );

**Usage in Templates**:

.. code-block:: html

    <link rel="stylesheet" href="{{ asset_url(path='css/main.css', base_url=base_url) }}">

Phase 5: Template Rendering
----------------------------

5.1 Base Context Creation
~~~~~~~~~~~~~~~~~~~~~~~~~~

Location: ``peta/src/core/builder.rs`` (line 655-666)

.. code-block:: rust

    fn create_base_context(&self) -> Context {
        let mut context = Context::new();
        
        context.insert("config", &self.config);
        context.insert("site", &self.config.site);
        context.insert("build", &self.config.build);
        context.insert("theme", &self.config.theme);
        
        // ADD base_url TO GLOBAL CONTEXT
        context.insert("base_url", &self.config.site.base_url);  // <-- Critical
        
        context
    }

5.2 Template Usage
~~~~~~~~~~~~~~~~~~~

Templates access ``base_url`` from context and pass it to URL functions.

Example: ``themes/default/components/atomic/navbar/navbar.html``

.. code-block:: html

    <nav class="navbar" data-component="navbar">
      <ul class="navbar-list">
        <li class="navbar-item">
          <a href="{{ url(path='books.html', base_url=base_url) }}" class="navbar-link">
            Books
          </a>
        </li>
        <li class="navbar-item">
          <a href="{{ url(path='articles.html', base_url=base_url) }}" class="navbar-link">
            Articles
          </a>
        </li>
        <!-- ... more links ... -->
      </ul>
    </nav>

5.3 Nested Component Rendering
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

When a component calls another component (e.g., header → navbar), the ``base_url`` must propagate through the call chain.

In ``peta/src/templates/engine.rs`` (line 321-326):

.. code-block:: rust

    let context = Self::build_component_context(component_name, &props, &tag_collector, &config_clone);

    match nested_tera.render(component_name, &context) {
        Ok(mut rendered) => {
            // Pass base_url to nested components
            rendered = Self::handle_nested_components(
                component_name,
                &rendered,
                &props,
                &tag_collector,
                &template_cache,
                &theme_dir,
                &component_manager_clone2,
                &config_clone.site.base_url.as_str()  // <-- Pass base_url
            )?;
            Ok(Value::String(rendered))
        }
        Err(e) => {
            // ... error handling ...
        }
    }

Phase 6: Rust Code URL Generation
----------------------------------

6.1 Book TOC Generator
~~~~~~~~~~~~~~~~~~~~~~

Location: ``peta/src/content/rst/book_toc_generator.rs`` (line 51-55)

.. code-block:: rust

    // Before:
    // let full_url = if base_url.is_empty() {
    //     format!("/{}", path)
    // } else {
    //     format!("{}/{}", base_url.trim_end_matches('/'), path)
    // };

    // After:
    let full_url = crate::utils::url::build_url(base_url, path);

6.2 Snippet Card Renderer
~~~~~~~~~~~~~~~~~~~~~~~~~~

Location: ``peta/src/content/rst/embedded_snippet_cards/embedded_snippet_card_renderer.rs`` (line 86-89)

.. code-block:: rust

    // Before:
    // let snippet_url = if base_url.is_empty() {
    //     format!("/{}", path)
    // } else {
    //     format!("{}/{}", base_url.trim_end_matches('/'), path)
    // };

    // After:
    let snippet_url = crate::utils::url::build_url(base_url, path);

6.3 Builder URL Generation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Location: ``peta/src/core/builder.rs`` (line 808-811)

.. code-block:: rust

    // Before:
    // let add_base = |url: &str| {
    //     if base_url.is_empty() {
    //         format!("/{}", url.trim_start_matches('/'))
    //     } else {
    //         format!("{}/{}", base_url.trim_end_matches('/'), url.trim_start_matches('/'))
    //     }
    // };

    // After:
    let add_base = |url: &str| crate::utils::url::build_url(base_url, url);

Phase 7: JavaScript Generation
-------------------------------

7.1 Asset Pipeline Setup
~~~~~~~~~~~~~~~~~~~~~~~~~

Location: ``peta/src/assets/pipeline.rs``

The ``AssetConfig`` struct now includes ``base_url``:

.. code-block:: rust

    pub struct AssetConfig {
        pub base_url: String,
        pub theme_dir: PathBuf,
        pub output_dir: PathBuf,
        // ... other fields ...
    }

7.2 JavaScript Generator
~~~~~~~~~~~~~~~~~~~~~~~~

Location: ``peta/src/assets/js_generator.rs`` (line 498)

The generated JavaScript now respects ``base_url``:

.. code-block:: rust

    // In EmbeddedSnippetCardConfig
    pub struct EmbeddedSnippetCardConfig {
        pub base_url: String,  // <-- Add base_url
        // ... other fields ...
    }

    // When generating JavaScript
    let config = EmbeddedSnippetCardConfig {
        base_url: self.config.base_url.clone(),  // <-- Pass base_url
        // ... other fields ...
    };

    // In generated JavaScript
    // Before: fetch(`/snippets/${snippetId}.json`)
    // After:  fetch(`${baseUrl}/snippets/${snippetId}.json`)

Example Generated JavaScript:

.. code-block:: javascript

    const baseUrl = "/peta-rust";  // Injected from Rust
    const snippetsJsonUrl = `${baseUrl}/snippets/${snippetId}.json`;
    fetch(snippetsJsonUrl)
        .then(response => response.json())
        .then(data => { /* ... */ });

Execution Flow Example
----------------------

Example: Building with ``--base-url "/peta-rust"``

1. **CLI Command**:
   
   .. code-block:: bash
   
       peta build --base-url "/peta-rust"

2. **Config Update**:
   
   .. code-block:: text
   
   config.site.base_url = "/peta-rust"

3. **Builder Creation**:
   
   .. code-block:: text
   
   SiteBuilder { config: { site: { base_url: "/peta-rust" } } }

4. **Template Engine Creation**:
   
   .. code-block:: text
   
   TemplateEngine { config: { site: { base_url: "/peta-rust" } } }

5. **Component Rendering**:
   
   .. code-block:: text
   
   build_component_context() -> context["base_url"] = "/peta-rust"

6. **Template Execution**:
   
   .. code-block:: html
   
   {{ url(path='books.html', base_url='/peta-rust') }}

7. **URL Generation**:
   
   .. code-block:: text
   
   build_url("/peta-rust", "books.html") -> "/peta-rust/books.html"

8. **Generated HTML**:
   
   .. code-block:: html
   
   <a href="/peta-rust/books.html" class="navbar-link">Books</a>

Debugging URL Issues
--------------------

When URL generation doesn't work as expected, follow these steps:

1. **Check CLI Argument**:
   
   .. code-block:: bash
   
       peta build --base-url "/peta-rust" --verbose

2. **Verify Config Loading**:
   
   Check ``peta/src/cli/commands.rs`` line 212 confirms ``config.site.base_url`` is updated.

3. **Check Builder Config**:
   
   Verify ``SiteBuilder`` receives the correct config.

4. **Check Template Engine Config**:
   
   Verify ``TemplateEngine::new_with_config()`` receives the correct config.

5. **Check Context Building**:
   
   Verify ``build_component_context()`` inserts ``base_url`` into context.

6. **Check Template Usage**:
   
   Verify templates pass ``base_url`` to URL functions.

7. **Check URL Function**:
   
   Verify ``url()`` and ``asset_url()`` receive ``base_url`` from arguments.

8. **Check Generated HTML**:
   
   Inspect ``_out/dist/*.html`` files for correct URLs.

Common Issues and Solutions
---------------------------

Issue 1: Navbar URLs show ``/books.html`` instead of ``/peta-rust/books.html``
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**Root Cause**: TemplateEngine loaded config from ``peta.toml`` instead of using CLI overrides.

**Solution**: Pass config from SiteBuilder to TemplateEngine:

.. code-block:: rust

    // Before:
    let template_engine = TemplateEngine::new_with_components(&theme, self.component_registry.clone())?;

    // After:
    let template_engine = TemplateEngine::new_with_components(
        &theme,
        self.component_registry.clone(),
        self.config.clone()  // <-- Pass the correct config
    )?;

Issue 2: URL function receives ``base_url=''`` in nested components
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**Root Cause**: ``base_url`` not passed through nested component call chain.

**Solution**: Ensure ``handle_nested_components()`` receives ``base_url`` parameter:

.. code-block:: rust

    rendered = Self::handle_nested_components(
        component_name,
        &rendered,
        &props,
        &tag_collector,
        &template_cache,
        &theme_dir,
        &component_manager_clone2,
        &config_clone.site.base_url.as_str()  // <-- Pass base_url
    )?;

Issue 3: Template syntax error with dynamic URL construction
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**Root Cause**: Trying to use complex expressions in ``url()`` function.

**Solution**: Use separate variables:

.. code-block:: html

    {% set tag_slug = tag | slugify %}
    {% set tag_path = "tags/" ~ tag_slug ~ ".html" %}
    <a href="{{ url(path=tag_path, base_url=base_url) }}">{{ tag }}</a>

**Note**: This approach may have issues with Tera syntax. Consider using hardcoded URLs for dynamic paths or creating a custom ``tag_url()`` function.

Issue 4: JavaScript fetch URLs don't respect base_url
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**Root Cause**: Generated JavaScript doesn't have access to base_url.

**Solution**: Pass base_url to JavaScript generator config:

.. code-block:: rust

    let config = EmbeddedSnippetCardConfig {
        base_url: self.config.base_url.clone(),
        // ... other fields ...
    };

Testing the Pipeline
---------------------

1. **Localhost Test**:
   
   .. code-block:: bash
   
       peta build --base-url ""
       grep -o 'href="[^"]*"' _out/dist/index.html | grep -E '(books|articles|snippets|projects)'
       # Expected: /books.html, /articles.html, /snippets.html, /projects.html

2. **GitHub Pages Test**:
   
   .. code-block:: bash
   
       peta build --base-url "/peta-rust"
       grep -o 'href="[^"]*"' _out/dist/index.html | grep -E '(books|articles|snippets|projects)'
       # Expected: /peta-rust/books.html, /peta-rust/articles.html, etc.

3. **Code Verification**:
   
   .. code-block:: bash
   
       # Search for hardcoded URLs (should return 0)
       grep -r 'href="/' themes/default/templates/
       grep -r 'src="/' themes/default/templates/
       grep -r 'fetch("/' peta/src/

Summary
-------

The URL generation pipeline ensures that:

1. **CLI arguments override config**: ``--base-url`` updates ``SiteConfig``
2. **Config propagates correctly**: From CLI → Builder → TemplateEngine → Components
3. **base_url is always available**: In template context and component functions
4. **URL generation is centralized**: Through ``build_url()`` utility function
5. **Templates use consistent functions**: ``url()`` and ``asset_url()`` for all URLs
6. **Nested components work**: base_url passes through component call chain

This pipeline eliminates hardcoded URLs and ensures consistent URL generation across the entire application.