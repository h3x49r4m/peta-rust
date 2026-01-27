Component Pipeline
==================

Overview
--------

The component pipeline is the system that discovers, loads, registers, and renders components in Peta. It provides automatic component discovery from the filesystem, eliminating the need for hardcoded component lists.

Pipeline Flow
-------------

.. code-block:: text

    ┌─────────────────────────────────────────────────────────────────┐
    │                      Component Discovery                        │
    │  Scans: themes/default/components/                              │
    │  Categories: atomic/, composite/                                │
    └──────────────────────────┬──────────────────────────────────────┘
                               │
                               ▼
    ┌─────────────────────────────────────────────────────────────────┐
    │                    Component YAML Files                         │
    │  Example: themes/default/components/atomic/navbar/               │
    │           └── component.yaml                                     │
    │               - name: navbar                                    │
    │               - category: atomic                                 │
    │               - version: 1.0.0                                  │
    │               - description: Site navigation bar component       │
    │               - dependencies: []                                 │
    └──────────────────────────┬──────────────────────────────────────┘
                               │
                               ▼
    ┌─────────────────────────────────────────────────────────────────┐
    │                  ComponentDiscovery::discover()                  │
    │  - Scans filesystem for component directories                   │
    │  - Reads component.yaml files                                   │
    │  - Parses metadata with backward compatibility                   │
    │  - Returns Vec<ComponentInfo>                                   │
    └──────────────────────────┬──────────────────────────────────────┘
                               │
                               ▼
    ┌─────────────────────────────────────────────────────────────────┐
    │                    ComponentManager::initialize()               │
    │  - Creates ComponentManager with theme_dir                       │
    │  - Calls ComponentDiscovery::discover(theme_dir)                │
    │  - Registers all discovered components in ComponentRegistry     │
    │  - Caches component metadata for fast lookup                    │
    └──────────────────────────┬──────────────────────────────────────┘
                               │
                               ▼
    ┌─────────────────────────────────────────────────────────────────┐
    │                  ComponentRegistry::register_component()        │
    │  - Validates component configuration                            │
    │  - Checks dependencies are satisfied                            │
    │  - Stores component in internal registry                        │
    │  - Provides lookup methods:                                    │
    │    - get_component(name)                                       │
    │    - get_components_by_category(category)                       │
    └──────────────────────────┬──────────────────────────────────────┘
                               │
                               ▼
    ┌─────────────────────────────────────────────────────────────────┐
    │                 TemplateEngine::new_with_components()           │
    │  - Creates TemplateEngine with ComponentManager                 │
    │  - Initializes TagCollector for tag caching                      │
    │  - Initializes TemplateCache for template caching                │
    │  - Registers component functions in Tera                        │
    └──────────────────────────┬──────────────────────────────────────┘
                               │
                               ▼
    ┌─────────────────────────────────────────────────────────────────┐
    │           TemplateEngine::register_component_functions()        │
    │  - Registers "component" function for rendering                 │
    │  - Registers "get_component_category" function                  │
    │  - Registers "component_styles" function                        │
    │  - Registers "component_scripts" function                       │
    │  - All functions use ComponentManager for lookups               │
    └──────────────────────────┬──────────────────────────────────────┘
                               │
                               ▼
    ┌─────────────────────────────────────────────────────────────────┐
    │                  Component Rendering Pipeline                    │
    │                                                                  │
    │  Template calls: component("navbar", {...})                     │
    │         │                                                        │
    │         ▼                                                        │
    │  ComponentManager::get_component_category("navbar")            │
    │         │                                                        │
    │         ▼                                                        │
    │  ComponentDiscovery::load_component_template("navbar")         │
    │         │                                                        │
    │         ▼                                                        │
    │  TemplateCache::load(".../navbar/navbar.html")                 │
    │         │                                                        │
    │         ▼                                                        │
    │  Tera::render("navbar", context)                                │
    │         │                                                        │
    │         ▼                                                        │
    │  Handle nested components (if any)                              │
    │         │                                                        │
    │         ▼                                                        │
    │  Return rendered HTML                                          │
    └─────────────────────────────────────────────────────────────────┘

Key Components
--------------

**ComponentDiscovery** (peta/src/components/discovery.rs)
    - Filesystem scanner for component directories
    - YAML metadata parser with backward compatibility
    - Returns Vec<ComponentInfo> with component metadata

**ComponentManager** (peta/src/components/manager.rs)
    - Registry and caching layer
    - Initializes with ComponentDiscovery
    - Provides component lookup methods
    - Caches discovered components

**ComponentRegistry** (peta/src/components/registry.rs)
    - Internal component storage
    - Validates component configurations
    - Checks dependencies
    - Provides component lookups by name and category

**TemplateEngine** (peta/src/templates/engine.rs)
    - Rendering orchestration
    - Registers component functions in Tera
    - Manages TagCollector and TemplateCache
    - Handles nested component rendering

**TagCollector** (peta/src/templates/engine.rs)
    - Extracts tags from content files
    - Caches tag collections
    - Supports per-directory tag collection

**TemplateCache** (peta/src/templates/engine.rs)
    - Caches loaded template files
    - Reduces filesystem I/O
    - Improves rendering performance

Component Metadata (component.yaml)
-----------------------------------

Each component directory contains a ``component.yaml`` file with metadata:

.. code-block:: yaml

    name: navbar
    category: atomic
    version: 1.0.0
    description: Site navigation bar component
    dependencies: []

Supported Categories
--------------------

- **atomic**: Basic building blocks (navbar, grid_card, code_block, etc.)
- **composite**: Complex UI components (header, footer, page_tags, etc.)

Backward Compatibility
---------------------

If a ``component.yaml`` file is missing, the system automatically infers metadata:

- **name**: Directory name
- **category**: Parent directory name (atomic/composite)
- **version**: "1.0.0"
- **description**: Empty string
- **enabled**: true
- **dependencies**: Empty vector

Flow Summary
------------

.. code-block:: text

    Filesystem → YAML Parser → Discovery → Manager → Registry → Engine → Render
        ↓           ↓            ↓          ↓         ↓        ↓        ↓
      Scan     Parse Metadata  Cache    Register  Store   Lookup  HTML

Usage Example
-------------

In a template file:

.. code-block:: html

    {{ component("navbar", brand="My Site", links=[...]) }}

This triggers the pipeline:

1. Lookup component category from ComponentManager
2. Load template from filesystem (cached in TemplateCache)
3. Build rendering context with props
4. Render using Tera template engine
5. Handle nested components if needed
6. Return rendered HTML

Benefits
--------

- **Automatic Discovery**: No need to manually register components
- **Theme-Aware**: Components are loaded from the active theme directory
- **Declarative Metadata**: Component info stored in YAML files
- **Performance**: Template and tag caching reduces I/O
- **Maintainability**: Adding new components only requires creating directory and YAML
- **Flexible**: Supports multiple themes with different component sets