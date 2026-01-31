//! Site configuration management

use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::{Result, Context};


/// Site configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub site: SiteInfo,
    pub social: SocialConfig,
    pub build: BuildConfig,
    pub rst: RstConfig,
    pub server: ServerConfig,
    pub search: SearchConfig,
    pub assets: AssetsConfig,
    pub deploy: DeployConfig,
    pub components: crate::components::SiteComponentConfig,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            site: SiteInfo::default(),
            social: SocialConfig::default(),
            build: BuildConfig::default(),
            rst: RstConfig::default(),
            server: ServerConfig::default(),
            search: SearchConfig::default(),
            assets: AssetsConfig::default(),
            deploy: DeployConfig::default(),
            components: crate::components::SiteComponentConfig::default(),
        }
    }
}

/// Social media configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialConfig {
    pub github: Option<String>,
    pub x: Option<String>,
    pub email: Option<String>,
}

impl Default for SocialConfig {
    fn default() -> Self {
        Self {
            github: None,
            x: None,
            email: None,
        }
    }
}

/// Site information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInfo {
    pub title: String,
    pub description: String,
    pub url: String,
    pub author: String,
    pub base_url: String,
}

impl Default for SiteInfo {
    fn default() -> Self {
        Self {
            title: "Peta".to_string(),
            description: "High-Performance Static Site Generator".to_string(),
            url: "https://example.com".to_string(),
            author: "Peta Team".to_string(),
            base_url: String::new(),
        }
    }
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub content_dir: String,
    pub output_dir: String,
    pub theme_dir: String,
    pub drafts: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            content_dir: "_content".to_string(),
            output_dir: "_out/dist".to_string(),
            theme_dir: "themes".to_string(),
            drafts: false,
        }
    }
}

/// RST processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RstConfig {
    pub default_directives: Vec<String>,
    pub math_renderer: String,
    pub code_highlighter: String,
    pub toc_depth: usize,
    pub cross_references: bool,
    pub math: MathConfig,
    pub code: CodeConfig,
}

impl Default for RstConfig {
    fn default() -> Self {
        Self {
            default_directives: vec![
                "code-block".to_string(),
                "snippet-card".to_string(),
                "toctree".to_string(),
            ],
            math_renderer: "katex".to_string(),
            code_highlighter: "syntect".to_string(),
            toc_depth: 3,
            cross_references: true,
            math: MathConfig::default(),
            code: CodeConfig::default(),
        }
    }
}

/// Math rendering configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathConfig {
    pub katex_delimiters: Vec<String>,
    pub fallback_mathjax: bool,
    pub cache_rendered: bool,
}

impl Default for MathConfig {
    fn default() -> Self {
        Self {
            katex_delimiters: vec![
                "$$".to_string(),
                "$".to_string(),
                "\\[".to_string(),
                "\\]".to_string(),
            ],
            fallback_mathjax: true,
            cache_rendered: true,
        }
    }
}

/// Code highlighting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeConfig {
    pub line_numbers: bool,
    pub copy_button: bool,
    pub theme: String,
}

impl Default for CodeConfig {
    fn default() -> Self {
        Self {
            line_numbers: true,
            copy_button: true,
            theme: "one-dark".to_string(),
        }
    }
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub open_browser: bool,
    pub livereload: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 3566,
            host: "127.0.0.1".to_string(),
            open_browser: true,
            livereload: true,
        }
    }
}

/// Search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub enabled: bool,
    pub client_side: bool,
    pub index_content: bool,
    pub index_metadata: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            client_side: true,
            index_content: true,
            index_metadata: true,
        }
    }
}

/// Asset processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetsConfig {
    pub minify_css: bool,
    pub minify_js: bool,
    pub optimize_images: bool,
    pub image_quality: u8,
}

impl Default for AssetsConfig {
    fn default() -> Self {
        Self {
            minify_css: true,
            minify_js: true,
            optimize_images: true,
            image_quality: 85,
        }
    }
}

/// Deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployConfig {
    pub target: String,
    pub branch: String,
    pub domain: String,
}

impl Default for DeployConfig {
    fn default() -> Self {
        Self {
            target: "github".to_string(),
            branch: "gh-pages".to_string(),
            domain: "username.github.io".to_string(),
        }
    }
}

impl SiteConfig {
    /// Load configuration from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;
        
        let config: SiteConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.as_ref().display()))?;
        
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .with_context(|| "Failed to serialize configuration")?;
        
        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write config file: {}", path.as_ref().display()))?;
        
        Ok(())
    }
    
    /// Get output directory as PathBuf
    pub fn output_dir(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(&self.build.output_dir)
    }
    
    /// Get content directory as PathBuf
    pub fn content_dir(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(&self.build.content_dir)
    }
    
    /// Get theme directory as PathBuf
    pub fn theme_dir(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(&self.build.theme_dir)
    }
}