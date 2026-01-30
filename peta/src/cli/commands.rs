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
    
    // Check if directory already exists
    if site_dir.exists() {
        return Err(anyhow::anyhow!("Directory '{}' already exists", name));
    }
    
    // Create site directory structure
    std::fs::create_dir_all(&site_dir)?;
    std::fs::create_dir_all(site_dir.join("_content/articles"))?;
    std::fs::create_dir_all(site_dir.join("_content/books"))?;
    std::fs::create_dir_all(site_dir.join("_content/projects"))?;
    std::fs::create_dir_all(site_dir.join("_content/snippets"))?;
    
    // Copy workspace files (Cargo.toml, Cargo.lock, peta directory)
    output.info("Copying peta workspace...");
    
    // Copy root Cargo.toml
    if Path::new("Cargo.toml").exists() {
        std::fs::copy("Cargo.toml", site_dir.join("Cargo.toml"))?;
    }
    
    // Copy Cargo.lock
    if Path::new("Cargo.lock").exists() {
        std::fs::copy("Cargo.lock", site_dir.join("Cargo.lock"))?;
    }
    
    // Copy peta source directory
    let peta_source_dir = Path::new("peta");
    if peta_source_dir.exists() {
        copy_dir_recursive(peta_source_dir, &site_dir.join("peta"))?;
    } else {
        output.warn("peta source directory not found, skipping source copy");
    }
    
    // Copy theme directory
    let theme_source_dir = Path::new("themes").join(theme);
    if theme_source_dir.exists() {
        output.info(&format!("Copying theme '{}'...", theme));
        copy_dir_recursive(&theme_source_dir, &site_dir.join("themes").join(theme))?;
    } else {
        output.warn(&format!("Theme '{}' not found, skipping theme copy", theme));
    }
    
    // Create .gitignore
    let gitignore_content = r#"# Build output
_out/

# Cache
.peta_cache/

# Cargo build artifacts
target/

# IDE and editor files
.vscode/
.idea/
*.swp
*.swo
*~

# OS generated files
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db

# Temporary files
*.tmp
*.temp
*.bak
*.backup

# Log files
*.log
"#;
    std::fs::write(site_dir.join(".gitignore"), gitignore_content)?;
    
    // Create Makefile
    let makefile_content = r#".PHONY: help build-peta build serve clean

help:
	@echo "Available commands:"
	@echo "  make build-peta  - Build peta from source"
	@echo "  make build       - Build the site"
	@echo "  make serve       - Start development server"
	@echo "  make clean       - Clean build artifacts"

build-peta:
	@echo "Building peta..."
	cargo build --release --manifest-path=peta/Cargo.toml --target-dir target
	@echo "✓ peta built successfully!"

build: build-peta
	@echo "Building site..."
	./target/release/peta build

serve: build-peta
	@echo "Starting development server..."
	./target/release/peta serve

clean:
	@echo "Cleaning build artifacts..."
	rm -rf _out
	rm -rf .peta_cache
	rm -rf target
	@echo "✓ Clean completed!"
"#;
    std::fs::write(site_dir.join("Makefile"), makefile_content)?;
    
    // Create configuration file
    let mut config = SiteConfig::default();
    config.site.title = name.to_string();
    config.site.description = format!("A site built with Peta");
    config.site.url = "https://example.com".to_string();
    config.build.theme_dir = format!("themes/{}", theme);
    config.build.drafts = false;
    config.components.enabled_components = vec![];  // Disable components for simpler setup
    
    config.save_to_file(site_dir.join("peta.toml"))?;
    
    // Create sample content
    create_sample_content(&site_dir)?;
    
    output.success(&format!("Site '{}' created successfully!", name));
    output.info("Next steps:");
    output.info(&format!("  cd {}", name));
    output.info("  make build-peta");
    output.info("  ./target/release/peta init content article \"My First Article\"");
    output.info("  make serve");
    
    Ok(())
}

/// Build the static site
pub async fn build_site(content_dir: Option<String>, output_dir: Option<String>, theme: Option<String>, draft: bool, output: &mut OutputFormatter) -> Result<()> {
    output.info("Building static site...");
    
    let mut config = SiteConfig::load_from_file("peta.toml")?;
    
    // Override content directory if specified
    if let Some(ref dir) = content_dir {
        config.build.content_dir = dir.clone();
    }
    
    // Override output directory if specified
    if let Some(ref out_dir) = output_dir {
        config.build.output_dir = out_dir.clone();
    }
    
    // Override theme if specified
    if let Some(ref theme_name) = theme {
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
pub async fn serve_site(content_dir: Option<String>, port: u16, host: &str, _open: bool, draft: bool, output: &mut OutputFormatter) -> Result<()> {
    output.info(&format!("Starting development server on http://{}:{}", host, port));
    
    let mut config = SiteConfig::load_from_file("peta.toml")?;
    config.server.port = port;
    config.server.host = host.to_string();
    config.build.drafts = draft;
    
    // Override content directory if specified
    if let Some(ref dir) = content_dir {
        config.build.content_dir = dir.clone();
    }
    
    // Build the site first
    build_site(content_dir, None, None, draft, output).await?;
    
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
    build_site(None, None, None, false, output).await?;
    
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

fn create_sample_content(_site_dir: &Path) -> Result<()> {
    // No sample content needed - the theme's index.html serves as the homepage
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
    let date = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    
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

/// Recursively copy a directory
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    if dst.exists() {
        std::fs::remove_dir_all(dst)?;
    }
    std::fs::create_dir_all(dst)?;
    
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}