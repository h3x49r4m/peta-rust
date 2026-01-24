//! Error types for the Peta static site generator

use thiserror::Error;

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for Peta
#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Content processing error: {0}")]
    Content(String),
    
    #[error("RST parsing error: {0}")]
    RstParse(String),
    
    #[error("Template error: {0}")]
    Template(String),
    
    #[error("Component error: {0}")]
    Component(String),
    
    #[error("Component not found: {0}")]
    ComponentNotFound(String),
    
    #[error("Theme error: {0}")]
    Theme(String),
    
    #[error("Search error: {0}")]
    Search(String),
    
    #[error("Asset processing error: {0}")]
    Asset(String),
    
    #[error("Server error: {0}")]
    Server(String),
    
    #[error("Deployment error: {0}")]
    Deploy(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] toml::de::Error),
    
    #[error("Serialization error: {0}")]
    SerializationJson(#[from] serde_json::Error),
    
    #[error("Template error: {0}")]
    TemplateTera(#[from] tera::Error),
    
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    
    #[error("Image processing error: {0}")]
    Image(String),
    
    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),
    
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    
    #[error("Other error: {0}")]
    Other(String),
}

impl Error {
    /// Create a configuration error
    pub fn config<S: Into<String>>(msg: S) -> Self {
        Self::Config(msg.into())
    }
    
    /// Create a content processing error
    pub fn content<S: Into<String>>(msg: S) -> Self {
        Self::Content(msg.into())
    }
    
    /// Create an RST parsing error
    pub fn rst_parse<S: Into<String>>(msg: S) -> Self {
        Self::RstParse(msg.into())
    }
    
    /// Create a template error
    pub fn template<S: Into<String>>(msg: S) -> Self {
        Self::Template(msg.into())
    }
    
    /// Create a component error
    pub fn component<S: Into<String>>(msg: S) -> Self {
        Self::Component(msg.into())
    }
    
    /// Create a component not found error
    pub fn component_not_found<S: Into<String>>(msg: S) -> Self {
        Self::ComponentNotFound(msg.into())
    }
    
    /// Create a theme error
    pub fn theme<S: Into<String>>(msg: S) -> Self {
        Self::Theme(msg.into())
    }
    
    /// Create a search error
    pub fn search<S: Into<String>>(msg: S) -> Self {
        Self::Search(msg.into())
    }
    
    /// Create an asset processing error
    pub fn asset<S: Into<String>>(msg: S) -> Self {
        Self::Asset(msg.into())
    }
    
    /// Create a server error
    pub fn server<S: Into<String>>(msg: S) -> Self {
        Self::Server(msg.into())
    }
    
    /// Create a deployment error
    pub fn deploy<S: Into<String>>(msg: S) -> Self {
        Self::Deploy(msg.into())
    }
    
    /// Create an IO error
    pub fn io(e: std::io::Error) -> Self {
        Self::Io(e)
    }
    
    /// Create a generic other error
    pub fn other<S: Into<String>>(msg: S) -> Self {
        Self::Other(msg.into())
    }
}