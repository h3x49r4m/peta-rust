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
    category: atomic  # or composite (lowercase)
    description: A brief description of what this component does
    dependencies: []

Key fields:
- ``name``: Component identifier (must match directory name)
- ``category``: Either "atomic" or "composite" (lowercase)
- ``version``: Component version (defaults to "1.0.0")
- ``description``: Brief description of the component (optional)
- ``dependencies``: List of component dependencies (optional)

**Note**: All other fields (props, slots, templates, styles, scripts, etc.) are optional and have default values. For most simple components, you only need to specify ``name``, ``category``, ``version``, and ``description``.

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

Automatic Component Discovery
------------------------------

**Important**: As of Peta V4, components are automatically discovered from the filesystem. You do NOT need to manually register components in the code.

When you create a component directory with a ``component.yaml`` file in ``themes/default/components/``, it will be automatically discovered and available for use. The system scans the following directories:

- ``themes/default/components/atomic/`` - For atomic components
- ``themes/default/components/composite/`` - For composite components

For more details on how component discovery works, see :doc:`component_pipeline`.

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
    category: atomic
    description: A simple button component
    dependencies: []

**Note**: Use lowercase "atomic" or "composite" for the category.

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

5. Use the component in templates::

    {{ component(name="my_button", text="Click me", variant="primary") | safe }}

**Note**: No manual registration required! The component will be automatically discovered and available for use.

This completes the component creation process. You can now reuse your component across different pages and templates in your Peta site.