//! File utilities

use crate::core::Result;
use std::path::Path;

/// File utilities
pub struct FileUtils;

impl FileUtils {
    /// Ensure directory exists
    pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<()> {
        std::fs::create_dir_all(path)
            .map_err(|e| crate::core::Error::io(e))
    }
    
    /// Copy file
    pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
        std::fs::copy(from, to)
            .map(|_| ())
            .map_err(|e| crate::core::Error::io(e))
    }
    
    /// Read file to string
    pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
        std::fs::read_to_string(path)
            .map_err(|e| crate::core::Error::io(e))
    }
    
    /// Write string to file
    pub fn write_string<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
        if let Some(parent) = path.as_ref().parent() {
            Self::ensure_dir(parent)?;
        }
        
        std::fs::write(path, content)
            .map_err(|e| crate::core::Error::io(e))
    }
}