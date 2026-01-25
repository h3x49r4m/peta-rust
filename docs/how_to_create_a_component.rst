How to Create a Component
==========================

This guide explains how to create components in the Peta static site generator. Components are reusable building blocks that can be used across different pages and templates.

Component Structure
-------------------

Each component lives in the ``themes/default/components/`` directory and is organized by category:

- ``atomic/`` - Small, single-purpose components (buttons, inputs, tags, etc.)
- ``composite/`` - Complex components that combine multiple atomic components

A typical component has the following structure::

    component_name/
    ├── component.yaml      # Component configuration
    ├── component_name.html # HTML template
    ├── component_name.css  # CSS styles
    └── component_name.js   # JavaScript (optional)

Component Configuration (component.yaml)
----------------------------------------

The ``component.yaml`` file defines the component's metadata, properties, and dependencies::

    name: my_component
    version: 1.0.0
    category: Atomic  # or Composite
    description: A brief description of what this component does
    enabled: true
    dependencies: []
    props:
      title:
        type: string
        required: true
        description: The title to display
      show_icon:
        type: boolean
        required: false
        default: true
        description: Whether to show an icon
    slots: []
    templates:
      - my_component.html
    styles:
      - my_component.css
    scripts:
      - my_component.js
    static_data: []
    config_schema: {}
    default_config: {}
    seo: null
    state: []

Key fields:
- ``name``: Component identifier (must match directory name)
- ``category``: Either "Atomic" or "Composite"
- ``props``: Component properties that can be passed when using the component
- ``templates``: List of HTML template files
- ``styles``: List of CSS files
- ``scripts``: List of JavaScript files (optional)

HTML Template
-------------

The HTML template defines the structure of your component. Use the ``props`` object to access component properties::

    <div class="my-component{% if props.show_icon %} has-icon{% endif %}" data-component="my_component">
        {% if props.show_icon %}
        <div class="component-icon">
            <i class="fas fa-star"></i>
        </div>
        {% endif %}
        <h2 class="component-title">{{ props.title }}</h2>
        <div class="component-content">
            {% if slots.content %}
                {{ slots.content }}
            {% endif %}
        </div>
    </div>

CSS Styles
----------

CSS files define the visual appearance of the component. Use the component name as a CSS class to avoid conflicts::

    /* My Component Styles */
    .my-component {
        background: var(--background-primary, #ffffff);
        border: 1px solid var(--border-color, #e2e8f0);
        border-radius: var(--radius-md, 0.375rem);
        padding: var(--spacing-4, 1rem);
        margin-bottom: var(--spacing-4, 1rem);
    }

    .my-component.has-icon {
        display: flex;
        align-items: center;
        gap: var(--spacing-3, 0.75rem);
    }

    .component-title {
        font-size: var(--font-size-lg, 1.125rem);
        font-weight: var(--font-weight-semibold, 600);
        color: var(--text-color, #1a202c);
        margin: 0 0 var(--spacing-2, 0.5rem) 0;
    }

JavaScript (Optional)
--------------------

JavaScript files add interactivity to components. Use the ``data-component`` attribute to find component instances::

    // My Component JavaScript
    document.addEventListener('DOMContentLoaded', function() {
        // Find all component instances
        const components = document.querySelectorAll('[data-component="my_component"]');
        
        components.forEach(function(component) {
            // Add event listeners or initialize functionality
            component.addEventListener('click', function() {
                // Handle click events
                console.log('Component clicked:', component);
            });
        });
    });

Registering Components
----------------------

After creating a component, you need to register it in the component loader. Edit ``peta/src/components/loader.rs`` and add your component to the ``get_category_dir`` function::

    fn get_category_dir(&self, name: &str) -> &'static str {
        match name {
            // ... existing components ...
            "my_component" => "atomic",
            // ... other components ...
            _ => "content",
        }
    }

Using Components in Templates
-----------------------------

Components can be used in templates using the ``component`` function::

    {{ component(name="my_component", title="Hello World", show_icon=true) | safe }}

For components with content slots::

    {% component name="my_component" %}
        <p>This is the content inside the component.</p>
    {% endcomponent %}

Including Component Styles
--------------------------

Component styles are automatically included when you add the component to the ``component_styles`` function in your templates. Edit ``themes/default/templates/base.html``::

    <style>
    {{ component_styles(component_names=["header", "navbar", "my_component", "footer"]) | safe }}
    </style>

Best Practices
---------------

1. **Naming**: Use descriptive, lowercase names with underscores for components
2. **CSS Classes**: Prefix all CSS classes with the component name to avoid conflicts
3. **Props**: Make components configurable through props rather than hardcoding values
4. **Responsive Design**: Use CSS variables and responsive breakpoints for consistent design
5. **Documentation**: Include clear descriptions in the component.yaml file

Example: Creating a Simple Button Component
-------------------------------------------

1. Create the directory::

    mkdir -p themes/default/components/atomic/my_button

2. Create ``component.yaml``::

    name: my_button
    version: 1.0.0
    category: Atomic
    description: A simple button component
    enabled: true
    dependencies: []
    props:
      text:
        type: string
        required: true
        description: Button text
      variant:
        type: string
        required: false
        default: "primary"
        description: Button style variant (primary, secondary, danger)
    slots: []
    templates:
      - my_button.html
    styles:
      - my_button.css
    scripts: []
    static_data: []
    config_schema: {}
    default_config: {}
    seo: null
    state: []

3. Create ``my_button.html``::

    <button class="my-button my-button--{{ props.variant | default(value='primary') }}" data-component="my_button">
        {{ props.text }}
    </button>

4. Create ``my_button.css``::

    .my-button {
        padding: var(--spacing-2, 0.5rem) var(--spacing-4, 1rem);
        border: none;
        border-radius: var(--radius-md, 0.375rem);
        font-weight: var(--font-weight-medium, 500);
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .my-button--primary {
        background: var(--color-primary, #3b82f6);
        color: white;
    }

    .my-button--secondary {
        background: var(--background-secondary, #f3f4f6);
        color: var(--text-color, #1a202c);
    }

    .my-button--danger {
        background: var(--color-danger, #ef4444);
        color: white;
    }

5. Register the component in ``peta/src/components/loader.rs``::

    fn get_category_dir(&self, name: &str) -> &'static str {
        match name {
            // ... existing components ...
            "my_button" => "atomic",
            // ... other components ...
            _ => "content",
        }
    }

6. Use the component in templates::

    {{ component(name="my_button", text="Click me", variant="primary") | safe }}

This completes the component creation process. You can now reuse your component across different pages and templates in your Peta site.