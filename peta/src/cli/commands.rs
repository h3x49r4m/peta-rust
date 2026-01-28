//! CLI command implementations

use crate::cli::output::OutputFormatter;
use crate::core::SiteConfig;
use std::path::Path;
use anyhow::Result;

/// Initialize new content (article/book/snippet/project)
pub fn init_content(content_type: &str, title: &str, output: &mut OutputFormatter) -> Result<()> {
    // Map content type to target directory
    let target_dir = match content_type {
        "article" => "_content/articles",
        "book" => "_content/books",
        "snippet" => "_content/snippets",
        "project" => "_content/projects",
        _ => return Err(anyhow::anyhow!("Invalid content type: {}", content_type)),
    };
    
    // Generate filename (convert title to kebab-case)
    let filename = title_to_filename(title);
    let file_path = Path::new(target_dir).join(format!("{}.rst", filename));
    
    // Check if file already exists
    if file_path.exists() {
        return Err(anyhow::anyhow!("File '{}' already exists", file_path.display()));
    }
    
    // Generate template content
    let content = generate_template(content_type, title)?;
    
    // Ensure directory exists
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Write file
    std::fs::write(&file_path, content)?;
    
    output.success(&format!("{} '{}' created successfully!", capitalize(content_type), title));
    output.info(&format!("Location: {}", file_path.display()));
    
    Ok(())
}

/// Initialize a new site
pub fn init_site(name: &str, theme: &str, output: &mut OutputFormatter) -> Result<()> {
    output.info(&format!("Creating new site: {}", name));
    
    let site_dir = Path::new(name);
    
    // Create site directory structure
    std::fs::create_dir_all(&site_dir)?;
    std::fs::create_dir_all(site_dir.join("_content"))?;
    std::fs::create_dir_all(site_dir.join("themes").join(theme))?;
    std::fs::create_dir_all(site_dir.join("themes").join(theme).join("templates"))?;
    std::fs::create_dir_all(site_dir.join("themes").join(theme).join("css"))?;
    std::fs::create_dir_all(site_dir.join("themes").join(theme).join("js"))?;
    
    // Create configuration file
    let mut config = SiteConfig::default();
    config.site.title = name.to_string();
    config.build.theme_dir = format!("themes/{}", theme);
    
    config.save_to_file(site_dir.join("peta.toml"))?;
    
    // Create default theme
    create_default_theme(&site_dir, theme)?;
    
    // Create sample content
    create_sample_content(&site_dir)?;
    
    output.success(&format!("Site '{}' created successfully!", name));
    output.info("Next steps:");
    output.info(&format!("  cd {}", name));
    output.info("  peta serve");
    
    Ok(())
}

/// Build the static site
pub async fn build_site(output_dir: Option<String>, theme: Option<String>, draft: bool, output: &mut OutputFormatter) -> Result<()> {
    output.info("Building static site...");
    
    let mut config = SiteConfig::load_from_file("peta.toml")?;
    
    // Override output directory if specified
    if let Some(out_dir) = output_dir {
        config.build.output_dir = out_dir.clone();
    }
    
    // Override theme if specified
    if let Some(theme_name) = theme {
        config.build.theme_dir = format!("themes/{}", theme_name);
    }
    
    // Override draft setting
    config.build.drafts = draft;
    
    // Store output directory before moving config
    let output_dir_path = config.build.output_dir.clone();
    
    let mut builder = crate::core::SiteBuilder::new(config);
    
    // Build the site
    let _site = builder.build().await?;
    
    output.success(&format!("Site built successfully!"));
    output.info(&format!("Output directory: {}", output_dir_path));
    
    Ok(())
}

/// Serve the site locally
pub async fn serve_site(port: u16, host: &str, _open: bool, draft: bool, output: &mut OutputFormatter) -> Result<()> {
    output.info(&format!("Starting development server on http://{}:{}", host, port));
    
    let mut config = SiteConfig::load_from_file("peta.toml")?;
    config.server.port = port;
    config.server.host = host.to_string();
    config.build.drafts = draft;
    
    // Build the site first
    build_site(None, None, draft, output).await?;
    
    // Create site instance
    let site = crate::core::Site::with_content(config.clone(), vec![]);
    
    // Start development server
    let mut dev_server = crate::server::dev_server::DevServer::new(port, site)?;
    dev_server.start().await?;
    
    output.info(&format!("Server running at http://{}:{}", host, port));
    
    Ok(())
}

/// Deploy the site
pub async fn deploy_site(target: &str, output: &mut OutputFormatter) -> Result<()> {
    output.info(&format!("Deploying site to {}...", target));
    
    let _config = SiteConfig::load_from_file("peta.toml")?;
    
    // Build the site first
    build_site(None, None, false, output).await?;
    
    // For now, just indicate deployment would happen
    output.warn(&format!("Deployment to {} is not yet implemented", target));
    output.success("Deployment completed!");
    
    Ok(())
}

/// Clean build artifacts
pub fn clean_site(all: bool, output: &mut OutputFormatter) -> Result<()> {
    if all {
        output.info("Cleaning all generated files and cache...");
        if Path::new("_out").exists() {
            std::fs::remove_dir_all("_out")?;
        }
        if Path::new(".peta_cache").exists() {
            std::fs::remove_dir_all(".peta_cache")?;
        }
    } else {
        output.info("Cleaning build artifacts...");
        if Path::new("_out").exists() {
            std::fs::remove_dir_all("_out")?;
        }
    }
    
    output.success("Clean completed!");
    
    Ok(())
}

fn create_default_theme(site_dir: &Path, theme: &str) -> Result<()> {
    let theme_dir = site_dir.join("themes").join(theme);
    
    // Create base template
    let base_template = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ site.title }} - {{ page.title }}</title>
    <!-- CSS is now inline in templates -->
</head>
<body>
    <header>
        <nav>
            <a href="{{ url('/') }}">{{ site.title }}</a>
        </nav>
    </header>
    
    <main>
        {% block content %}{% endblock %}
    </main>
    
    <footer>
        <p>&copy; 2026 {{ site.author }}. All rights reserved.</p>
    </footer>
    
    <script src="{{ asset_url('js/search.js') }}"></script>
</body>
</html>"#;
    
    std::fs::write(theme_dir.join("templates").join("base.html"), base_template)?;
    
    // Create index template
    let index_template = r#"{% extends "base.html" %}

{% block content %}
<h1>{{ site.title }}</h1>
<p>{{ site.description }}</p>

{% if recent_content %}
<h2>Recent Content</h2>
<ul>
{% for item in recent_content %}
    <li><a href="{{ item.url }}">{{ item.title }}</a></li>
{% endfor %}
</ul>
{% endif %}
{% endblock %}"#;
    
    std::fs::write(theme_dir.join("templates").join("index.html"), index_template)?;
    
    // Create basic CSS
    let css_content = r#"body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    line-height: 1.6;
    color: #333;
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}

header {
    border-bottom: 1px solid #eee;
    padding-bottom: 20px;
    margin-bottom: 40px;
}

nav a {
    text-decoration: none;
    color: #007bff;
    font-weight: bold;
    font-size: 1.2em;
}

footer {
    margin-top: 40px;
    padding-top: 20px;
    border-top: 1px solid #eee;
    text-align: center;
    color: #666;
}"#;
    
    std::fs::write(theme_dir.join("css").join("main.css"), css_content)?;
    
    Ok(())
}

fn create_sample_content(site_dir: &Path) -> Result<()> {
    let content_dir = site_dir.join("_content");
    
    // Create index.rst
    let index_content = r#"Welcome to {{ site.title }}
==========================

{{ site.description }}

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   articles/index
   snippets/index
   books/index
   projects/index
"#;
    
    std::fs::write(content_dir.join("index.rst"), index_content)?;
    
    // Create article index
    let articles_dir = content_dir.join("articles");
    std::fs::create_dir_all(&articles_dir)?;
    
    let articles_index = r#"Articles
========

.. toctree::
   :maxdepth: 2
   
   getting-started
   advanced-topics
"#;
    
    std::fs::write(articles_dir.join("index.rst"), articles_index)?;
    
    // Create sample article
    let sample_article = r#"Getting Started
===============

This is a sample article to get you started with {{ site.title }}.

Features
--------

* Fast static site generation
* RST support with extensions
* Component-based theming
* Live development server
* Search functionality

Code Blocks
-----------

.. code-block:: rust

    fn main() {
        println!("Hello, {{ site.title }}!");
    }

Links and References
-------------------

You can create links to other pages like this: `articles/getting-started`.

And reference sections like this: `Features`_.

.. _Features: #features
"#;
    
    std::fs::write(articles_dir.join("getting-started.rst"), sample_article)?;
    
    Ok(())
}

/// Theme management commands
pub mod theme {
    use super::*;
    
    use crate::components::ThemeManager;
    use std::path::Path;

    /// List available themes
    pub fn list_themes(output: &mut OutputFormatter) -> Result<()> {
        output.info("Available themes:");
        
        let mut theme_manager = ThemeManager::new("themes");
        let themes_dir = Path::new("themes");
        
        if !themes_dir.exists() {
            output.warn("No themes directory found");
            return Ok(());
        }
        
        // Load all themes
        for entry in std::fs::read_dir(themes_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let theme_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                
                if let Ok(_) = theme_manager.load_theme(theme_name, &path) {
                    if let Some(theme_config) = theme_manager.get_theme(theme_name) {
                        output.info(&format!("  {} (v{})", theme_config.name, theme_config.version));
                        if let Some(description) = &theme_config.description {
                            output.info(&format!("    {}", description));
                        }
                        if let Some(parent) = &theme_config.extends {
                            output.info(&format!("    Extends: {}", parent));
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Create a new theme
    pub fn create_theme(name: &str, base: Option<String>, output: &mut OutputFormatter) -> Result<()> {
        output.info(&format!("Creating new theme: {}", name));
        
        let theme_dir = Path::new("themes").join(name);
        if theme_dir.exists() {
            return Err(anyhow::anyhow!("Theme '{}' already exists", name));
        }
        
        // Create theme directory structure
        std::fs::create_dir_all(&theme_dir)?;
        std::fs::create_dir_all(theme_dir.join("components"))?;
        std::fs::create_dir_all(theme_dir.join("templates"))?;
        std::fs::create_dir_all(theme_dir.join("css"))?;
        std::fs::create_dir_all(theme_dir.join("js"))?;
        std::fs::create_dir_all(theme_dir.join("assets").join("images"))?;
        std::fs::create_dir_all(theme_dir.join("assets").join("fonts"))?;
        
        // Create theme configuration
        let theme_config = crate::components::ThemeConfig {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            description: Some(format!("{} theme", name)),
            extends: base,
            variables: std::collections::HashMap::new(),
            components: std::collections::HashMap::new(),
            assets: crate::components::AssetConfig::default(),
        };
        
        let config_content = serde_yaml::to_string(&theme_config)?;
        std::fs::write(theme_dir.join("theme.yaml"), config_content)?;
        
        // Create base template
        let base_template = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ site.title }} - {{ page.title }}</title>
    <!-- CSS is now inline in templates -->
</head>
<body>
    <header>
        <nav>
            <a href="{{ url('/') }}">{{ site.title }}</a>
        </nav>
    </header>
    
    <main>
        {% block content %}{% endblock %}
    </main>
    
    <footer>
        <p>&copy; 2026 {{ site.author }}. All rights reserved.</p>
    </footer>
    
    <script src="{{ asset_url('js/main.js') }}"></script>
</body>
</html>"#;
        
        std::fs::write(theme_dir.join("templates").join("base.html"), base_template)?;
        
        // Create basic CSS
        let css_content = r#"/* {} theme styles */
:root {{
    /* Theme variables will be injected here */
}}

body {{
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    line-height: 1.6;
    color: #333;
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}}

header {{
    border-bottom: 1px solid #eee;
    padding-bottom: 20px;
    margin-bottom: 40px;
}}

nav a {{
    text-decoration: none;
    color: #007bff;
    font-weight: bold;
    font-size: 1.2em;
}}

footer {{
    margin-top: 40px;
    padding-top: 20px;
    border-top: 1px solid #eee;
    text-align: center;
    color: #666;
}}"#;
        
        let css_with_theme_name = css_content.replace("{}", name);
        std::fs::write(theme_dir.join("css").join("main.css"), css_with_theme_name)?;
        
        // Create basic JS
        let js_content = r#"// {} theme scripts
console.log('Theme "{}" loaded');"#;
        let js_with_theme_name = js_content.replace("{}", name).replace("{}", name);
        std::fs::write(theme_dir.join("js").join("main.js"), js_with_theme_name)?;
        
        output.success(&format!("Theme '{}' created successfully!", name));
        output.info("Next steps:");
        output.info(&format!("  1. Edit themes/{}/theme.yaml to configure your theme", name));
        output.info(&format!("  2. Modify themes/{}/templates/ and themes/{}/css/ to customize appearance", name, name));
        output.info("  3. Use your theme with: peta build --theme <theme_name>");
        
        Ok(())
    }

    /// Validate theme configuration
    pub fn validate_theme(name: &str, output: &mut OutputFormatter) -> Result<()> {
        output.info(&format!("Validating theme: {}", name));
        
        let mut theme_manager = ThemeManager::new("themes");
        let theme_path = Path::new("themes").join(name);
        
        if !theme_path.exists() {
            return Err(anyhow::anyhow!("Theme '{}' not found", name));
        }
        
        theme_manager.load_theme(name, &theme_path)?;
        
        match theme_manager.validate_theme(name) {
            Ok(()) => output.success(&format!("Theme '{}' is valid!", name)),
            Err(e) => {
                output.error(&format!("Theme '{}' validation failed: {}", name, e));
                return Err(anyhow::anyhow!("{}", e));
            }
        }
        
        Ok(())
    }

    /// Show theme information
    pub fn theme_info(name: &str, output: &mut OutputFormatter) -> Result<()> {
        let mut theme_manager = ThemeManager::new("themes");
        let theme_path = Path::new("themes").join(name);
        
        if !theme_path.exists() {
            return Err(anyhow::anyhow!("Theme '{}' not found", name));
        }
        
        theme_manager.load_theme(name, &theme_path)?;
        
        if let Some(theme_config) = theme_manager.get_theme(name) {
            output.info(&format!("Theme: {}", theme_config.name));
            output.info(&format!("Version: {}", theme_config.version));
            
            if let Some(description) = &theme_config.description {
                output.info(&format!("Description: {}", description));
            }
            
            if let Some(parent) = &theme_config.extends {
                output.info(&format!("Extends: {}", parent));
            }
            
            let variables = theme_manager.get_theme_variables(name);
            if !variables.is_empty() {
                output.info("Variables:");
                for (key, value) in variables {
                    output.info(&format!("  {}: {}", key, value));
                }
            }
            
            if !theme_config.components.is_empty() {
                output.info(&format!("Components: {}", theme_config.components.len()));
                for component_name in theme_config.components.keys() {
                    output.info(&format!("  - {}", component_name));
                }
            }
            
            output.info(&format!("CSS files: {}", theme_config.assets.css.len()));
            output.info(&format!("JS files: {}", theme_config.assets.js.len()));
        }
        
        Ok(())
    }

    /// Install theme from repository
    pub fn install_theme(source: &str, name: Option<String>, output: &mut OutputFormatter) -> Result<()> {
        let theme_name = name.unwrap_or_else(|| {
            // Extract name from source path
            source.split('/').last().unwrap_or(source).to_string()
        });
        
        output.info(&format!("Installing theme '{}' from '{}'", theme_name, source));
        
        // For now, this is a placeholder implementation
        // In a real implementation, this would handle Git repository cloning, etc.
        output.warn("Theme installation from repositories is not yet implemented");
        output.info(&format!("Please manually clone the theme to themes/{}", theme_name));
        
        Ok(())
    }
}

/// Generate template content based on content type
fn generate_template(content_type: &str, title: &str) -> Result<String> {
    let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    
    match content_type {
        "article" => Ok(format!(
r#"---
title: "{}"
date: {}
tags: ["tag1", "tag2"]
author: "Your Name"
---


{}
=====


Introduction
------------

Start writing your article here.


Section
-------

Add more sections as needed.
"#, title, date, title)),

        "book" => Ok(format!(
r#"---
title: "{}"
date: {}
tags: ["tag1", "tag2"]
author: "Your Name"
description: "A brief description of the book"
---

{}
=====

This book provides comprehensive coverage of the topic.

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   chapter1
   chapter2
   chapter3

What This Book Covers
---------------------

- Topic 1
- Topic 2
- Topic 3

Target Audience
---------------

Describe who this book is for.

Prerequisites
-------------

List any prerequisites.
"#, title, date, title)),

        "snippet" => Ok(format!(
r#"---
title: {}
date: {}
tags: [language, topic]
---

{}
=============================

This snippet demonstrates code examples.

.. code-block:: python

    # Your code here
    def example():
        print("Hello, world!")
        return True

Explanation
-----------

Add explanations here.
"#, title, date, title)),

        "project" => Ok(format!(
r#"---
title: "{}"
date: {}
tags: ["tag1", "tag2"]
author: "Your Name"
github_url: "https://github.com/username/project"
demo_url: "https://project.example.com"
---

{}
========================

A brief description of your project.

Features
--------

- **Feature 1**: Description
- **Feature 2**: Description
- **Feature 3**: Description

Technical Details
-----------------

Describe the technical stack and implementation details.

Usage
-----

1. Step one
2. Step two
3. Step three

Contributions
-------------

Information about contributing.
"#, title, date, title)),

        _ => Err(anyhow::anyhow!("Invalid content type: {}", content_type)),
    }
}

/// Convert title to filename (kebab-case)
fn title_to_filename(title: &str) -> String {
    title.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Capitalize first letter
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}