//! Peta - High-Performance RST-First Static Site Generator
//! 
//! This library provides the core functionality for building static sites
//! from RST (reStructuredText) content with advanced features including
//! math rendering, syntax highlighting, and client-side search.

pub mod cli;
pub mod components;
pub mod content;
pub mod core;
pub mod templates;
pub mod server;
pub mod assets;
pub mod search;
pub mod deploy;
pub mod utils;

// Re-export key types for convenience
pub use core::{Site, SiteBuilder, SiteConfig};
pub use core::theme::Theme;
pub use content::{RstContent, ContentType, ContentMetadata};
pub use templates::TemplateEngine;
pub use search::{SearchIndex, query::SearchResult};
pub use cli::{Cli, Commands};
pub use components::{Component, ComponentRegistry, ComponentManager};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration
pub fn default_config() -> SiteConfig {
    SiteConfig::default()
}