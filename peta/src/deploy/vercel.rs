//! Vercel deployment

use crate::core::{Site, Result};

/// Vercel deployer
pub struct VercelDeployer {
    #[allow(dead_code)]
    config: VercelConfig,
}

/// Vercel deployment configuration
#[derive(Debug, Clone)]
pub struct VercelConfig {
    pub project_id: String,
    pub token: String,
    pub output_dir: String,
}

impl VercelDeployer {
    /// Create a new Vercel deployer with default config
    pub fn new() -> Self {
        Self { 
            config: VercelConfig {
                project_id: String::new(),
                token: String::new(),
                output_dir: "_dist".to_string(),
            }
        }
    }
    
    /// Create a new Vercel deployer
    pub fn with_config(config: VercelConfig) -> Self {
        Self { config }
    }
    
    /// Deploy to Vercel
    pub async fn deploy(&self, _site: &Site) -> Result<()> {
        // For now, this is a placeholder
        // In a real implementation, you would use Vercel CLI or API
        println!("Deploying to Vercel...");
        Ok(())
    }
}