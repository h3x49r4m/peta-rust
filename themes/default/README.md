# Peta Rust V4 Default Theme

A modern, component-based theme for Peta Rust static site generator with React-inspired architecture.

## Features

- **Component-Based Architecture**: React-inspired component system with props, slots, and state
- **Responsive Design**: Mobile-first responsive layouts with flexible breakpoints
- **Dark Mode Support**: Built-in dark mode with automatic detection
- **Accessibility**: WCAG 2.1 AA compliant components
- **Performance Optimized**: CSS/JS minification, image optimization, and lazy loading
- **SEO Friendly**: Structured data, Open Graph, and Twitter Cards support

## Architecture

### Component Categories

#### Atomic Components
Basic building blocks that cannot be broken down further:
- **Button**: Interactive button with multiple variants and states
- **Input**: Form input with validation and styling
- **Badge**: Small status indicators and labels
- **Icon**: Icon component with multiple icon libraries

#### Composite Components
Complex UI components built from atomic components:
- **Navigation**: Site navigation with responsive design
- **Card**: Flexible card component for content display
- **Modal**: Dialog and modal components
- **Dropdown**: Dropdown menus and select components

#### Layout Components
Page structure templates:
- **Base Layout**: Main page layout with header, content, and footer
- **Articles Layout**: Specialized layout for article listings
- **Books Layout**: Layout for book content with navigation
- **Snippets Layout**: Layout for code snippets with filtering
- **Projects Layout**: Layout for project showcases

#### Content Components
Content-specific UI components:
- **Article Card**: Article preview card with metadata
- **Book Card**: Book preview with progress tracking
- **Snippet Card**: Code snippet preview with language detection
- **Project Card**: Project showcase with tech stack

## Usage

### Template Syntax

Use components in templates with the `{% component %}` tag:

```tera
{% component "button" with variant="primary" size="lg" href="/articles" %}
  {% slot children %}Explore Articles{% endslot %}
{% endcomponent %}

{% component "article_card" with show_excerpt=true show_tags=true %}
  {% slot title %}
    <h3><a href="{{ article.url }}">{{ article.title }}</a></h3>
  {% endslot %}
  {% slot excerpt %}
    <p>{{ article.excerpt }}</p>
  {% endslot %}
  {% slot tags %}
    {% for tag in article.tags %}
    <a href="/tags/{{ tag }}" class="tag">{{ tag }}</a>
    {% endfor %}
  {% endslot %}
{% endcomponent %}
```

### Component Props

Pass configuration to components using the `with` keyword:

```tera
{% component "navigation" with variant="horizontal" sticky=true %}
  {% slot brand %}...{% endslot %}
  {% slot menu %}...{% endslot %}
  {% slot actions %}...{% endslot %}
{% endcomponent %}
```

### Slots

Slots allow content injection into components:

```tera
{% component "card" with variant="elevated" %}
  {% slot header %}
    <h2>Card Title</h2>
  {% endslot %}
  {% slot body %}
    <p>Card content goes here.</p>
  {% endslot %}
  {% slot footer %}
    <button>Action</button>
  {% endslot %}
{% endcomponent %}
```

### Component Configuration

Each component has a `component.yaml` file defining its configuration:

```yaml
name: "button"
version: "1.0.0"
category: "Atomic"
description: "Interactive button component with multiple variants"
dependencies: []

props:
  variant:
    type: string
    enum: ["primary", "secondary", "outline", "text"]
    default: "primary"
  size:
    type: string
    enum: ["sm", "md", "lg"]
    default: "md"
  disabled:
    type: boolean
    default: false

slots:
  - name: children
    description: "Button content"
  - name: icon
    description: "Optional icon"

state:
  - name: loading
    type: boolean
    default: false

templates:
  - button.html
styles:
  - button.css
scripts:
  - button.js
```

## Customization

### Theme Variables

Customize the theme appearance using CSS variables:

```css
:root {
  --color-primary-500: #3b82f6;
  --color-secondary-500: #6c757d;
  --background-color: #ffffff;
  --text-color: #1e293b;
  --font-family: "Inter, system-ui, sans-serif";
}
```

### Component Override

Override component styles by creating custom CSS:

```css
/* Custom button variant */
.button-custom {
  background: linear-gradient(45deg, #667eea 0%, #764ba2 100%);
  border: none;
  color: white;
}

/* Custom card layout */
.card-custom {
  display: grid;
  grid-template-columns: 1fr 2fr;
  gap: 1rem;
}
```

### Component Extension

Extend existing components by creating new components that inherit from base components:

```yaml
name: "custom_button"
extends: "button"
props:
  # Additional props
  gradient:
    type: boolean
    default: false
```

## Performance

### Asset Optimization

The theme includes built-in asset optimization:
- CSS minification and compression
- JavaScript minification and bundling
- Image optimization and WebP conversion
- Critical CSS generation
- Cache busting for assets

### Lazy Loading

Images and components are lazy loaded by default:

```html
<!-- Lazy loaded image -->
<img src="placeholder.jpg" data-src="actual.jpg" loading="lazy" alt="Description">

<!-- Lazy loaded component -->
<div data-component="heavy_component" data-lazy="true"></div>
```

### Code Splitting

JavaScript is automatically split by component:

```javascript
// Components are loaded on demand
import('./components/heavy_component.js')
  .then(module => module.initialize())
  .catch(error => console.error(error));
```

## Development

### Local Development

1. Install dependencies:
```bash
npm install
```

2. Start development server:
```bash
peta serve --theme default
```

3. Open browser to `http://localhost:3000`

### Component Development

1. Create component directory:
```bash
mkdir themes/default/components/atomic/my_component
```

2. Create component files:
```bash
touch themes/default/components/atomic/my_component/{component.yaml,template.html,style.css,script.js}
```

3. Define component configuration in `component.yaml`

4. Implement component logic in template, CSS, and JavaScript

5. Test component in templates

### Testing

Run tests to ensure component compatibility:

```bash
peta test --theme default
```

## Browser Support

- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## License

MIT License - see LICENSE file for details.