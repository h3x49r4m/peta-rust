CLI System
===========

Peta provides a comprehensive command-line interface (CLI) for managing static sites. The CLI is built using `clap` for robust argument parsing and user-friendly help messages.

Overview
--------

The basic syntax for using Peta CLI is::

    peta <COMMAND> [OPTIONS]

To see all available commands::

    peta --help

To get help for a specific command::

    peta <COMMAND> --help

Available Commands
------------------

Site Management
^^^^^^^^^^^^^^

**new** - Create a new site
~~~~~~~~~~~~~~~~~~~~~~~~~~~

Initialize a new Peta site with the specified name and theme::

    peta new --name <SITE_NAME> [--theme <THEME>]

Options:

- ``-n, --name <SITE_NAME>``: Site name (required)
- ``-t, --theme <THEME>``: Theme to use (default: "default")

Example::

    peta new --name myblog --theme default

This creates a new site directory with the following structure::

    myblog/
    ├── _content/
    ├── themes/
    ├── peta.toml
    └── ...

**build** - Build the static site
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Compile the site and generate static HTML files::

    peta build [OPTIONS]

Options:

- ``-o, --output <OUTPUT>``: Output directory (default: "_out/dist")
- ``-t, --theme <THEME>``: Theme to use (default: "default")
- ``--draft``: Include draft content

Examples::

    peta build
    peta build --output my_output
    peta build --theme custom --draft

**serve** - Serve the site locally
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Start a development server with live-reload support::

    peta serve [OPTIONS]

Options:

- ``-p, --port <PORT>``: Port to serve on (default: 3566)
- ``--host <HOST>``: Host to serve on (default: "127.0.0.1")
- ``-o, --open``: Open browser automatically
- ``--draft``: Include draft content

Examples::

    peta serve
    peta serve --port 8080 --open
    peta serve --host 0.0.0.0 --draft

The development server automatically rebuilds the site when files change and reloads the browser.

**deploy** - Deploy the site
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Deploy the built site to various platforms::

    peta deploy [OPTIONS]

Options:

- ``-t, --target <TARGET>``: Deployment target (default: "github")

Supported deployment targets:

- ``github``: GitHub Pages
- ``netlify``: Netlify
- ``vercel``: Vercel
- ``s3``: Amazon S3

Example::

    peta deploy --target github

Note: Deployment features may require additional configuration in ``peta.toml``.

**clean** - Clean build artifacts
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Remove generated files and caches::

    peta clean [OPTIONS]

Options:

- ``-a, --all``: Clean all artifacts including output directory and cache

Examples::

    peta clean          # Remove output directory only
    peta clean --all    # Remove output and cache directories

Content Management
^^^^^^^^^^^^^^^^^^^

**init** - Initialize new content
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Create a new content file (article, book, snippet, or project) with a template::

    peta init <TYPE> <TITLE>

Arguments:

- ``<TYPE>``: Content type - ``article``, ``book``, ``snippet``, or ``project``
- ``<TITLE>``: Title of the content

Examples::

    peta init article "Getting Started with Peta"
    peta init book "Rust Programming Guide"
    peta init snippet "Data Processing Example"
    peta init project "My Awesome Project"

This command creates a new ``.rst`` file in the appropriate directory:

- ``_content/articles/`` for articles
- ``_content/books/`` for books
- ``_content/snippets/`` for snippets
- ``_content/projects/`` for projects

The generated files include YAML front matter and basic RST structure with placeholders.

Theme Management
^^^^^^^^^^^^^^^^

Peta provides a powerful theme system with its own subcommands.

**theme list** - List available themes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Display all installed themes with their versions and descriptions::

    peta theme list

**theme create** - Create a new theme
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Create a new theme with optional base theme::

    peta theme create --name <THEME_NAME> [--base <BASE_THEME>]

Options:

- ``--name <THEME_NAME>``: Theme name (required)
- ``--base <BASE_THEME>``: Base theme to extend from

Example::

    peta theme create --name mytheme --base default

This creates a new theme directory with::

    themes/mytheme/
    ├── components/
    ├── templates/
    ├── css/
    ├── js/
    └── theme.yaml

**theme validate** - Validate theme configuration
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Check if a theme's configuration is valid::

    peta theme validate --name <THEME_NAME>

Example::

    peta theme validate --name mytheme

**theme info** - Show theme information
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Display detailed information about a theme::

    peta theme info --name <THEME_NAME>

Example::

    peta theme info --name default

This shows:

- Theme name and version
- Description
- Parent theme (if any)
- Theme variables
- Available components
- Asset files

**theme install** - Install theme from repository
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Install a theme from a Git repository or other source::

    peta theme install --source <SOURCE> [--name <THEME_NAME>]

Options:

- ``--source <SOURCE>``: Theme repository URL or name
- ``--name <THEME_NAME>``: Custom theme name (optional)

Example::

    peta theme install --source https://github.com/user/theme.git

Configuration
-------------

Peta uses a ``peta.toml`` configuration file in the project root. The file is automatically created when you run ``peta new``.

Basic configuration example::

    [site]
    title = "My Site"
    description = "A static site built with Peta"
    author = "Your Name"
    url = "https://example.com"

    [build]
    theme_dir = "themes/default"
    output_dir = "_out/dist"
    drafts = false

    [server]
    port = 3566
    host = "127.0.0.1"

Configuration options can be overridden using command-line options.

Exit Codes
----------

The CLI uses the following exit codes:

- ``0``: Success
- ``1``: General error
- ``2``: Invalid command or arguments

Development
-----------

The CLI system is located in the ``peta/src/cli/`` module:

- ``args.rs``: Command-line argument parsing using ``clap``
- ``commands.rs``: Command implementations
- ``output.rs``: Output formatting and styling

To add a new command:

1. Define the command in ``args.rs`` using ``clap`` derive macros
2. Implement the command logic in ``commands.rs``
3. Add the command handler in ``main.rs``

Best Practices
--------------

1. **Always use ``--help``**: When in doubt, check the help text for any command
2. **Use absolute paths**: When specifying file paths, use absolute paths for reliability
3. **Check configuration**: Ensure ``peta.toml`` exists and is valid before running build/serve
4. **Clean before deployment**: Run ``peta clean`` before deploying to ensure clean output
5. **Test locally**: Use ``peta serve`` to preview changes before deploying
6. **Version control**: Keep your ``peta.toml`` and theme files in version control

Common Workflows
----------------

**Create a new site and add content**::

    peta new --name myblog
    cd myblog
    peta init article "My First Post"
    peta serve --open

**Build and deploy**::

    peta build
    peta deploy --target github

**Work with drafts**::

    peta build --draft
    peta serve --draft

**Custom theme development**::

    peta theme create --name mytheme
    # Edit theme files
    peta theme validate --name mytheme
    peta serve --theme mytheme

Troubleshooting
---------------

**"Site not found" error**: Ensure you're in a directory with ``peta.toml``

**"Theme not found" error**: Check that the theme exists in the ``themes/`` directory

**Build fails**: Run ``peta clean --all`` and try building again

**Port already in use**: Specify a different port with ``--port`` option

**File already exists**: The ``init`` command won't overwrite existing files. Use a different title or remove the existing file first.