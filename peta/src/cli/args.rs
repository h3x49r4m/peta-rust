//! Command-line argument parsing

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "peta")]
#[command(about = "A fast static site generator in Rust")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new site
    New {
        /// Site name
        #[arg(short, long)]
        name: String,
        
        /// Theme to use (default: "default")
        #[arg(short, long, default_value = "default")]
        theme: String,
    },
    
    /// Build the static site
    Build {
        /// Content directory (default: "_content")
        #[arg(short, long)]
        content_dir: Option<String>,
        
        /// Output directory (default: "_out/dist")
        #[arg(short, long)]
        output: Option<String>,
        
        /// Theme to use (default: "default")
        #[arg(short, long)]
        theme: Option<String>,

        /// Include draft content
        #[arg(long)]
        draft: bool,
    },
    
    /// Serve the site locally
    Serve {
        /// Content directory (default: "_content")
        #[arg(short, long)]
        content_dir: Option<String>,
        
        /// Port to serve on (default: 3566)
        #[arg(short, long, default_value = "3566")]
        port: u16,
        
        /// Host to serve on (default: 127.0.0.1)
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        
        /// Open browser automatically
        #[arg(short, long)]
        open: bool,
        
        /// Include draft content
        #[arg(long)]
        draft: bool,
    },
    
    /// Deploy the site
    Deploy {
        /// Deployment target (default: "github")
        #[arg(short, long, default_value = "github")]
        target: String,
    },
    
    /// Clean build artifacts
    Clean {
        /// Clean all artifacts including output directory
        #[arg(short, long)]
        all: bool,
    },
    
    /// Initialize new content or site
    Init {
        #[command(subcommand)]
        action: InitAction,
    },
    
    /// Theme management
    Theme {
        #[command(subcommand)]
        action: ThemeAction,
    },
}

#[derive(Subcommand)]
pub enum InitAction {
    /// Initialize a new site
    Site {
        /// Site name
        name: String,
        
        /// Theme to use (default: "default")
        #[arg(short, long, default_value = "default")]
        theme: String,
    },
    
    /// Initialize new content (article/book/snippet/project)
    Content {
        /// Content type: article, book, snippet, or project
        #[arg(value_parser = ["article", "book", "snippet", "project"])]
        r#type: String,
        
        /// Title of the content
        title: String,

        /// Content directory (default: "_content")
        #[arg(short = 'd', long)]
        content_dir: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ThemeAction {
    /// List available themes
    List,
    
    /// Create a new theme
    Create {
        /// Theme name
        name: String,
        
        /// Base theme to extend from
        #[arg(short, long)]
        base: Option<String>,
    },
    
    /// Validate theme configuration
    Validate {
        /// Theme name
        name: String,
    },
    
    /// Show theme information
    Info {
        /// Theme name
        name: String,
    },
    
    /// Install theme from repository
    Install {
        /// Theme repository URL or name
        source: String,
        
        /// Theme name (different from repository name)
        #[arg(short, long)]
        name: Option<String>,
    },
}