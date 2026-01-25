How to Render Math Formulas: Step by Step
==========================================

This guide explains in detail how math formulas are rendered in the Peta static site generator, from RST source to final HTML rendering.

Step 1: Writing Math in RST Content
-----------------------------------

When you write mathematical content in RST files, use these delimiters:

Inline Math
~~~~~~~~~~~

Use single dollar signs for inline math:

.. code-block:: rst

    The derivative of $f(x)$ is $f'(x)$.

Display Math
~~~~~~~~~~~~

Use double dollar signs for display math:

.. code-block:: rst

    $$f'(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$$

LaTeX delimiters are also supported:

.. code-block:: rst

    \[f'(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}\]

    \(f(x)\) for inline math

Step 2: Peta's Math Processing Pipeline
---------------------------------------

When you run ``peta build``, here's what happens:

2.1 RST Parsing
~~~~~~~~~~~~~~~

The ``RstParser`` in ``peta/src/content/rst/parser.rs`` processes the content:

.. code-block:: rust

    // Process math equations
    processed = self.math_renderer.render(&processed)?;

2.2 Math Detection
~~~~~~~~~~~~~~~~~~

The ``MathProcessor`` in ``peta/src/content/rst/math_processor.rs`` scans for formulas:

.. code-block:: rust

    // Extract display math blocks ($$...$$)
    static DISPLAY_REGEX: Lazy<Regex> = 
        Lazy::new(|| Regex::new(r"\$\$(.*?)\$\$").unwrap());
    
    // Extract inline math blocks ($...$)
    // Uses a custom approach to avoid conflicts with display math

2.3 HTML Generation
~~~~~~~~~~~~~~~~~~~

The ``MathRenderer`` creates HTML elements with ``data-latex`` attributes:

.. code-block:: rust

    // Display math
    format!(r#"<div class="math-display" data-latex="{}"></div>"#, equation)
    
    // Inline math
    format!(r#"<span class="math-inline" data-latex="{}"></span>"#, equation)

Step 3: Template Integration
----------------------------

3.1 Math Detection Metadata
~~~~~~~~~~~~~~~~~~~~~~~~~~~

During parsing, Peta detects if content has math and sets flags:

.. code-block:: rust

    let math_detection = self.math_processor.auto_detect_math_content(&processed_html)?;
    let math_script = if math_detection.has_formulas {
        Some(self.math_renderer.generate_on_demand_script(&math_detection))
    } else {
        None
    };

3.2 Template Variables
~~~~~~~~~~~~~~~~~~~~~~

These variables are passed to templates:

.. code-block:: rust

    context.insert("has_math_formulas", &article.has_math_formulas);
    context.insert("math_formula_count", &article.math_formula_count);
    if let Some(math_script) = &article.math_render_script {
        context.insert("math_render_script", math_script);
    }

3.3 Template Rendering
~~~~~~~~~~~~~~~~~~~~~~~

In templates like ``snippet.html``:

.. code-block:: html

    {% if has_math_formulas %}
    {{ math_render_script | safe }}
    {% endif %}

Step 4: Client-Side Rendering
------------------------------

4.1 Auto-Generated Script
~~~~~~~~~~~~~~~~~~~~~~~~~~

Peta generates a JavaScript script that:

1. Checks if math rendering is needed
2. Loads KaTeX CSS and JS on-demand
3. Renders formulas using ``data-latex`` attributes

.. code-block:: javascript

    // Auto-generated math renderer for X formulas
    (function() {
        if (typeof window.mathRendererLoaded === 'undefined') {
            window.mathRendererLoaded = false;
            
            function loadKaTeX() {
                // Load CSS
                const css = document.createElement('link');
                css.rel = 'stylesheet';
                css.href = 'https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css';
                document.head.appendChild(css);
                
                // Load JS
                const katex = document.createElement('script');
                katex.src = 'https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js';
                katex.onload = function() {
                    window.mathRendererLoaded = true;
                    renderMathFormulas();
                };
                document.body.appendChild(katex);
            }
            
            function renderMathFormulas() {
                const elements = document.querySelectorAll('[data-latex]');
                elements.forEach(el => {
                    const latex = el.getAttribute('data-latex');
                    if (latex && window.katex) {
                        window.katex.render(latex, el, {
                            displayMode: el.classList.contains('math-display'),
                            throwOnError: false
                        });
                    }
                });
            }
        }
    })();

4.2 Modal Support
~~~~~~~~~~~~~~~~~

The snippet modal in ``themes/default/templates/base.html`` also supports math:

.. code-block:: javascript

    // Check if content has math formulas
    const hasMathFormulas = snippet.content && (
        snippet.content.includes('data-latex') || 
        snippet.content.includes('$$')
    );
    
    if (hasMathFormulas) {
        // Load KaTeX if needed and render
        const elements = body.querySelectorAll('[data-latex]');
        elements.forEach(el => {
            const latex = el.getAttribute('data-latex');
            if (latex && window.katex) {
                window.katex.render(latex, el, {
                    displayMode: el.classList.contains('math-display'),
                    throwOnError: false
                });
            }
        });
    }

Step 5: Final Rendering
-----------------------

5.1 HTML Output
~~~~~~~~~~~~~~~

The final HTML contains:

.. code-block:: html

    <!-- Math elements with data-latex -->
    <span class="math-inline" data-latex="f(x)"></span>
    <div class="math-display" data-latex="f'(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}"></div>
    
    <!-- Auto-generated rendering script -->
    <script>
    // Auto-generated math renderer for 9 formulas
    (function() { ... })();
    </script>

5.2 KaTeX Rendering
~~~~~~~~~~~~~~~~~~

When the page loads:

1. Script detects math elements
2. Loads KaTeX from CDN (if not already loaded)
3. Renders each formula using ``window.katex.render()``
4. Applies KaTeX CSS styling

5.3 Final Appearance
~~~~~~~~~~~~~~~~~~~~~

The formulas are rendered with proper mathematical typography:

- Inline formulas: :math:`f(x)`
- Display formulas: centered with proper spacing
- All formulas styled with KaTeX's beautiful typography

Step 6: Configuration and Customization
-----------------------------------------

6.1 Peta Configuration
~~~~~~~~~~~~~~~~~~~~~~

Math rendering is configured in ``peta.toml``:

.. code-block:: toml

    math_renderer = "katex"
    
    [rst.math]
    fallback_mathjax = true

6.2 Theme Configuration
~~~~~~~~~~~~~~~~~~~~~~~

Themes can customize math rendering in ``theme.yaml``:

.. code-block:: yaml

    # Math rendering
    math_rendering: true
    math_engine: "katex"

6.3 CSS Styling
~~~~~~~~~~~~~~~

Themes provide CSS for math elements:

.. code-block:: css

    /* Math Formula Styles */
    .math-display {
        display: block;
        text-align: center;
        margin: 1em 0;
        padding: 0.5em 0;
        overflow-x: auto;
        overflow-y: hidden;
    }

    .math-inline {
        display: inline;
        white-space: nowrap;
    }

    .math-error {
        color: #cc0000;
        font-style: italic;
    }

Common Examples
---------------

Here's how different formulas are processed:

Fractions
~~~~~~~~~

RST input:

.. code-block:: rst

    $$\frac{a}{b}$$
    
    Inline: $\frac{1}{2}$

Processing steps:

1. MathProcessor detects ``$$...$$`` and ``$...$`` delimiters
2. MathRenderer creates HTML elements:

.. code-block:: html

    <div class="math-display" data-latex="\frac{a}{b}"></div>
    <span class="math-inline" data-latex="\frac{1}{2}"></span>

3. Client-side KaTeX renders them as beautiful fractions

Integrals
~~~~~~~~~

RST input:

.. code-block:: rst

    $$\int_0^1 x^2 dx = \frac{1}{3}$$

Processing:

1. Detected as display math
2. Wrapped in ``<div class="math-display">`` with ``data-latex`` attribute
3. Rendered with proper integral symbols and limits

Summations
~~~~~~~~~~

RST input:

.. code-block:: rst

    $$\sum_{i=1}^n i = \frac{n(n+1)}{2}$$

Processing:

1. Complex formula detected
2. Preserved in ``data-latex`` attribute
3. KaTeX renders with proper summation symbol and formatting

Matrices
~~~~~~~~

RST input:

.. code-block:: rst

    $$\begin{pmatrix}
    a & b \\
    c & d
    \end{pmatrix}$$

Processing:

1. Multi-line formula detected as display math
2. LaTeX environment preserved in ``data-latex``
3. KaTeX renders as a proper matrix with brackets

Step 7: Troubleshooting Step by Step
------------------------------------

7.1 Formulas Not Rendering
~~~~~~~~~~~~~~~~~~~~~~~~~~~

Follow these steps to debug:

1. **Check HTML output**:
   - View page source
   - Look for ``data-latex`` attributes
   - Verify math rendering script is present

2. **Check browser console**:
   - Open developer tools
   - Look for JavaScript errors
   - Check for KaTeX loading errors

3. **Verify network requests**:
   - Check Network tab in dev tools
   - Ensure KaTeX CSS/JS load from CDN
   - Verify no 404 errors

4. **Check math detection**:
   - Verify delimiters are correctly paired
   - Check for escaped characters
   - Ensure no nested delimiters

7.2 Common Issues and Solutions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Empty formulas
~~~~~~~~~~~~~~

Problem: ``$$`` or ``$`` with no content

Solution: Ensure content between delimiters:

.. code-block:: rst

    # Wrong
    $$
    
    # Right
    $$x = 5$$

Mismatched delimiters
~~~~~~~~~~~~~~~~~~~~

Problem: ``$$...$`` or ``$...$$``

Solution: Use matching delimiters:

.. code-block:: rst

    # Wrong
    $$x = 5$
    
    # Right
    $$x = 5$$

Invalid LaTeX
~~~~~~~~~~~~

Problem: Unsupported LaTeX commands

Solution: Check KaTeX supported functions or use fallback text

7.3 Debug Mode
~~~~~~~~~~~~~~

Enable debug logging in browser console:

.. code-block:: javascript

    // In browser console
    localStorage.debug = 'peta:math';
    
    // Check for debug messages
    console.log('Math formulas detected:', document.querySelectorAll('[data-latex]').length);

Step 8: Performance Optimization
--------------------------------

8.1 On-Demand Loading
~~~~~~~~~~~~~~~~~~~~~

The system optimizes performance by:

1. **Detection Phase**:
   - Scans content for math delimiters
   - Only loads KaTeX if math is detected
   - Pages without math don't load KaTeX

2. **Lazy Loading**:
   - KaTeX loaded only when needed
   - Script injected dynamically
   - No impact on pages without math

3. **Caching**:
   - KaTeX cached by browser
   - Math renderer state preserved
   - Reused across pages

8.2 Best Practices for Performance
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1. **Minimize formulas per page**:
   - Consider splitting complex content
   - Use collapsible sections for heavy math

2. **Optimize complex formulas**:
   - Test rendering time
   - Simplify where possible
   - Use supported LaTeX commands

3. **Consider preloading**:
   - For math-heavy sites
   - Add KaTeX to main layout
   - Trade bandwidth for UX

Step 9: Advanced Customization
------------------------------

9.1 Custom CSS Styling
~~~~~~~~~~~~~~~~~~~~~~~

Add custom styles in your theme:

.. code-block:: css

    /* Enhanced math display */
    .math-display {
        margin: 1.5em 0;
        padding: 1em;
        background: #f8f9fa;
        border-radius: 4px;
        overflow-x: auto;
    }
    
    /* Inline math emphasis */
    .math-inline {
        color: #0066cc;
        font-weight: 500;
    }
    
    /* Dark mode support */
    @media (prefers-color-scheme: dark) {
        .math-display {
            background: #2d3748;
            color: #e2e8f0;
        }
    }

9.2 Custom Rendering Logic
~~~~~~~~~~~~~~~~~~~~~~~~~~

Extend the math renderer in your theme:

.. code-block:: javascript

    // Custom math rendering hook
    window.addEventListener('mathRendered', function(e) {
        // Add custom post-processing
        e.element.classList.add('custom-math');
    });
    
    // Custom error handling
    window.addEventListener('mathError', function(e) {
        console.error('Math rendering failed:', e.error);
        e.element.textContent = e.latex;
    });

9.3 Integration with Other Systems
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Math formulas integrate with:

- **Search**: LaTeX content indexed for search
- **RSS**: Formulas preserved in feeds
- **Print**: Optimized for print stylesheets
- **Accessibility**: ARIA labels added automatically

Step 10: Snippet Modal Integration
----------------------------------

10.1 Modal Math Detection
~~~~~~~~~~~~~~~~~~~~~~~~~

When a snippet modal opens:

1. Content is checked for math formulas
2. KaTeX loaded if not already present
3. Formulas rendered in modal context

10.2 Modal-Specific Considerations
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- **Z-index**: Math elements respect modal stacking
- **Responsive**: Formulas adapt to modal size
- **Focus**: Math elements don't trap focus
- **Animation**: Smooth rendering with modal transitions

References
----------

- KaTeX Documentation: https://katex.org/docs/
- Supported LaTeX Functions: https://katex.org/docs/supported.html
- RST Primer: https://docutils.sourceforge.io/rst.html
- Peta Source Code: https://github.com/h3x49r4m/peta-rust