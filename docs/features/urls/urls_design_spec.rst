URL Unification Design Specification
=====================================

Objective
---------

Eliminate ALL hardcoded URLs from the codebase and establish a single, centralized URL generation system that respects ``base_url`` configuration for both localhost (``--base-url ""``) and GitHub Pages (``--base-url "/peta-rust"``).

Core Principles
---------------

1. **Zero Hardcoded URLs**: No URL strings starting with ``/`` are allowed anywhere in the codebase
2. **Single Source of Truth**: All URL generation goes through one utility function
3. **base_url Transparency**: base_url is automatically applied at generation time
4. **Template Function Integration**: Templates use consistent functions, not manual path construction

Architecture
------------

Phase 1: Create Centralized URL Utility Module
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**New File**: ``peta/src/utils/url.rs``

.. code-block:: rust

    pub fn build_url(base_url: &str, path: &str) -> String {
        let clean_path = path.trim_start_matches('/');
        if base_url.is_empty() {
            format!("/{}", clean_path)
        } else {
            format!("{}/{}", base_url.trim_end_matches('/'), clean_path)
        }
    }

**Update**: ``peta/src/utils/mod.rs``

- Add ``pub mod url;`` export

Phase 2: Refactor Rust Code (5 files)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

2.1 ``peta/src/content/rst/book_toc_generator.rs``
- **Line 51-55**: Replace ``get_full_url()`` implementation with ``utils::url::build_url()``

2.2 ``peta/src/content/rst/embedded_snippet_cards/embedded_snippet_card_renderer.rs``
- **Line 86-89**: Replace inline URL construction with ``utils::url::build_url()``

2.3 ``peta/src/templates/engine.rs``
- **Lines 1025-1054**: Update ``url()`` and ``asset_url()`` functions to use ``utils::url::build_url()``
- These functions should accept ``path`` parameter and automatically get ``base_url`` from context

2.4 ``peta/src/templates/functions.rs``
- **Line 37-40**: Update ``AssetFunction::call()`` to use ``utils::url::build_url()``

2.5 ``peta/src/core/builder.rs``
- **Lines 808-811**: Replace ``add_base`` closure with ``utils::url::build_url()``

Phase 3: Ensure base_url in Template Context
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**File**: ``peta/src/core/builder.rs``

**Function**: ``create_base_context()`` (line 655-666)

**Change**: Add ``base_url`` to global context

.. code-block:: rust

    context.insert("base_url", &self.config.site.base_url);

**Impact**: Template functions can now access ``base_url`` without manual passing

Phase 4: Update Template Functions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**File**: ``peta/src/templates/engine.rs``

**Function Registration** (``url()`` and ``asset_url()``):

**New Signature**:

.. code-block:: rust

    tera.register_function("url", Box::new(|args: &HashMap<String, Value>| -> tera::Result<Value> {
        let path = args.get("path")
            .or_else(|| args.get("0"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| tera::Error::msg("Missing 'path' argument"))?;
        
        // Get base_url from context - always available now
        let base_url = args.get("base_url")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let url = crate::utils::url::build_url(base_url, path);
        Ok(Value::String(url))
    }));

**Same pattern for ``asset_url()``**

Phase 5: Refactor HTML Templates (Replace Hardcoded URLs)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

5.1 ``themes/default/templates/base.html``

**Current Issues** (Lines 13-31, 55-70):

.. code-block:: html

    <link rel="stylesheet" href="/css/main.css">
    <link rel="stylesheet" href="/assets/css/code-blocks.css">
    <!-- ... 6 more CSS files ... -->
    <script src="/js/main.js"></script>
    <script src="/assets/js/code-blocks.js">
    <!-- ... 5 more JS files ... -->

**Fix**: Replace with template functions

.. code-block:: html

    <link rel="stylesheet" href="{{ asset_url(path='css/main.css') }}">
    <link rel="stylesheet" href="{{ asset_url(path='assets/css/code-blocks.css') }}">
    <!-- ... etc ... -->
    <script src="{{ asset_url(path='js/main.js') }}"></script>
    <script src="{{ asset_url(path='assets/js/code-blocks.js') }}"></script>
    <!-- ... etc ... -->

**Inline JavaScript URLs** (Line 261-262):

.. code-block:: javascript

    // Before:
    if (window.location.pathname !== '/snippets.html') {
        history.replaceState({}, '', '/snippets.html');

    // After:
    const snippetsUrl = "{{ url(path='snippets.html') }}";
    if (window.location.pathname !== snippetsUrl) {
        history.replaceState({}, '', snippetsUrl);

5.2 ``themes/default/components/atomic/navbar/navbar.html``

**Current Issues** (Lines 4, 7, 10, 13):

.. code-block:: html

    <a href="/books.html" class="navbar-link">Books</a>
    <a href="/articles.html" class="navbar-link">Articles</a>
    <a href="/snippets.html" class="navbar-link">Snippets</a>
    <a href="/projects.html" class="navbar-link">Projects</a>

**Fix**:

.. code-block:: html

    <a href="{{ url(path='books.html') }}" class="navbar-link">Books</a>
    <a href="{{ url(path='articles.html') }}" class="navbar-link">Articles</a>
    <a href="{{ url(path='snippets.html') }}" class="navbar-link">Snippets</a>
    <a href="{{ url(path='projects.html') }}" class="navbar-link">Projects</a>

5.3 ``themes/default/templates/snippets.html``

**Current Issues** (Line 27):

.. code-block:: javascript

    history.replaceState({}, '', '/snippets.html');

**Fix**: Use variable passed from template

.. code-block:: javascript

    const snippetsPageUrl = "{{ url(path='snippets.html') }}";
    history.replaceState({}, '', snippetsPageUrl);

5.4 ``themes/default/templates/snippet.html``

**Current Issues** (Lines 227, 251, 339):

.. code-block:: html

    <a href="/snippets.html#tag-{{ tag }}" class="snippet-tag">{{ tag }}</a>
    <a href="/snippets.html">

**Fix**:

.. code-block:: html

    <a href="{{ url(path='snippets.html') }}#tag-{{ tag }}" class="snippet-tag">{{ tag }}</a>
    <a href="{{ url(path='snippets.html') }}">

**JavaScript URL** (Line 339):

.. code-block:: javascript

    // Before:
    window.location.href = '/snippets.html?modal=' + encodeURIComponent(currentUrl);

    // After:
    const snippetsUrl = "{{ url(path='snippets.html') }}";
    window.location.href = snippetsUrl + '?modal=' + encodeURIComponent(currentUrl);

5.5 ``themes/default/templates/404.html``

**Current Issues** (Lines 206, 213):

.. code-block:: html

    <a href="/" class="error-404-btn error-404-btn-primary">
    <a href="/articles.html" class="error-404-btn error-404-btn-secondary">

**Fix**:

.. code-block:: html

    <a href="{{ url(path='') }}" class="error-404-btn error-404-btn-primary">
    <a href="{{ url(path='articles.html') }}" class="error-404-btn error-404-btn-secondary">

5.6 ``themes/default/components/composite/header/header.html``

**Current Issues** (Line 4):

.. code-block:: html

    <a href="/" class="site-name">

**Fix**:

.. code-block:: html

    <a href="{{ url(path='') }}" class="site-name">

5.7 ``themes/default/components/atomic/site_stats.html``

**Current Issues** (Lines 4, 17, 33, 46):

.. code-block:: html

    <a href="/books.html" class="stat-card">
    <a href="/articles.html" class="stat-card">
    <a href="/snippets.html" class="stat-card">
    <a href="/projects.html" class="stat-card">

**Fix**:

.. code-block:: html

    <a href="{{ url(path='books.html') }}" class="stat-card">
    <a href="{{ url(path='articles.html') }}" class="stat-card">
    <a href="{{ url(path='snippets.html') }}" class="stat-card">
    <a href="{{ url(path='projects.html') }}" class="stat-card">

5.8 Tag URLs (Partially Dynamic)

**Files**: 
- ``themes/default/components/atomic/article_content/article_content.html`` (Lines 210, 302)
- ``themes/default/components/atomic/project_content/project_content.html`` (Lines 98, 119)

**Current**:

.. code-block:: html

    <a href="/tags/{{ tag | slugify }}.html" class="tag">

**Fix**: Create a ``tag_url()`` function or use nested ``url()``:

.. code-block:: html

    <a href="{{ url(path=('tags/' ~ (tag | slugify) ~ '.html')) }}" class="tag">

Phase 6: Fix JavaScript Generation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**File**: ``peta/src/assets/js_generator.rs``

**Current Issue** (Line 498):

.. code-block:: javascript

    fetch(`/snippets/${snippetId}.json`)

**Fix**: Pass base_url to generated JavaScript

.. code-block:: javascript

    // In the generation code, inject a constant
    const baseUrl = "${base_url}";
    const snippetsJsonUrl = baseUrl 
      ? `${baseUrl}/snippets/${snippetId}.json`
      : `/snippets/${snippetId}.json`;

    fetch(snippetsJsonUrl)

Phase 7: Remove <base> Tag (Optional but Recommended)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**File**: ``themes/default/templates/base.html``

**Current** (Line 7):

.. code-block:: html

    <base href="{{ config.site.base_url | safe }}/">

**Decision**: Keep for now, but all URLs should work without it

**Rationale**: The ``<base>`` tag creates confusion and inconsistency. With proper URL functions, it's not needed.

Phase 8: Add URL Tests
~~~~~~~~~~~~~~~~~~~~~~

**New File**: ``peta/tests/url_generation_test.rs**

**Test Cases**:

.. code-block:: rust

    #[cfg(test)]
    mod tests {
        use peta::utils::url::build_url;
        
        #[test]
        fn test_build_url_empty_base() {
            assert_eq!(build_url("", "css/main.css"), "/css/main.css");
            assert_eq!(build_url("", "/css/main.css"), "/css/main.css");
            assert_eq!(build_url("", "books.html"), "/books.html");
        }
        
        #[test]
        fn test_build_url_with_base() {
            assert_eq!(build_url("/peta-rust", "css/main.css"), "/peta-rust/css/main.css");
            assert_eq!(build_url("/peta-rust", "/css/main.css"), "/peta-rust/css/main.css");
            assert_eq!(build_url("/peta-rust", "books.html"), "/peta-rust/books.html");
        }
        
        #[test]
        fn test_build_url_trailing_slash() {
            assert_eq!(build_url("/peta-rust/", "css/main.css"), "/peta-rust/css/main.css");
            assert_eq!(build_url("/peta-rust", "/css/main.css"), "/peta-rust/css/main.css");
        }
    }

**Integration Tests**: Build site with both ``--base-url ""`` and ``--base_url "/peta-rust"`` and verify all generated URLs

Implementation Order
--------------------

1. **Phase 1**: Create ``peta/src/utils/url.rs``
2. **Phase 2**: Refactor Rust code (5 files)
3. **Phase 3**: Update ``create_base_context()`` to include ``base_url``
4. **Phase 4**: Update template functions to use new utility
5. **Phase 5**: Refactor HTML templates (11 files)
6. **Phase 6**: Fix JavaScript generation
7. **Phase 7**: (Optional) Remove ``<base>`` tag
8. **Phase 8**: Add comprehensive tests

Verification Steps
------------------

After implementation:

1. **Localhost Test**:

   .. code-block:: bash

       peta build --base-url ""
       peta serve

   - Navigate all pages
   - Check all links work
   - Verify asset loading
   - Test snippet modal

2. **GitHub Pages Test**:

   .. code-block:: bash

       peta build --base-url "/peta-rust"

   - Inspect generated HTML
   - Verify all URLs start with ``/peta-rust/``
   - Check no hardcoded ``/`` URLs remain

3. **Code Review**:

   - Search for ``href="/`` in templates (should be 0 results)
   - Search for ``src="/`` in templates (should be 0 results)
   - Search for ``fetch("/`` in Rust code (should be 0 results)

Summary of Changes
------------------

+----------------+-------+---------------+
| Category       | Files | Lines Changed |
+================+=======+===============+
| New Files      | 2     | ~150          |
+----------------+-------+---------------+
| Rust Code      | 5     | ~50           |
+----------------+-------+---------------+
| Templates      | 11    | ~80           |
+----------------+-------+---------------+
| Tests          | 1     | ~50           |
+----------------+-------+---------------+
| **Total**      | **19** | **~330**      |
+----------------+-------+---------------+

Key Benefits
------------

1. **Consistency**: All URLs use the same generation logic
2. **Maintainability**: Single function to modify for URL rules
3. **Flexibility**: Easy to add URL transformations (e.g., CDN, versioning)
4. **Testability**: Centralized logic is easy to test
5. **Zero Hardcoding**: No more broken URLs when base_url changes