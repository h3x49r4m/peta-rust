//! Component version management for V4 architecture

use serde::{Deserialize, Serialize};
use std::fmt;

/// Component version information following semantic versioning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentVersion {
    /// Major version
    pub major: u32,
    /// Minor version
    pub minor: u32,
    /// Patch version
    pub patch: u32,
    /// Pre-release identifier
    pub pre_release: Option<String>,
    /// Build metadata
    pub build: Option<String>,
}

impl ComponentVersion {
    /// Create a new version
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: None,
            build: None,
        }
    }
    
    /// Create version with pre-release
    pub fn with_pre_release(major: u32, minor: u32, patch: u32, pre_release: String) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: Some(pre_release),
            build: None,
        }
    }
    
    /// Parse version from string
    pub fn from_string(version: &str) -> Result<Self, crate::core::Error> {
        let mut parts = version.split('+');
        let version_part = parts.next().unwrap_or("");
        let build = parts.next().map(|s| s.to_string());
        
        let mut parts = version_part.split('-');
        let base_version = parts.next().unwrap_or("");
        let pre_release = parts.next().map(|s| s.to_string());
        
        let version_numbers: Vec<&str> = base_version.split('.').collect();
        if version_numbers.len() != 3 {
            return Err(crate::core::Error::Component(format!("Invalid version format: {}", version)));
        }
        
        let major = version_numbers[0].parse()
            .map_err(|_| crate::core::Error::Component(format!("Invalid major version: {}", version_numbers[0])))?;
        let minor = version_numbers[1].parse()
            .map_err(|_| crate::core::Error::Component(format!("Invalid minor version: {}", version_numbers[1])))?;
        let patch = version_numbers[2].parse()
            .map_err(|_| crate::core::Error::Component(format!("Invalid patch version: {}", version_numbers[2])))?;
        
        Ok(Self {
            major,
            minor,
            patch,
            pre_release,
            build,
        })
    }
    
    /// Check if this version is compatible with another version
    /// Following semantic versioning compatibility rules
    pub fn compatible_with(&self, other: &ComponentVersion) -> bool {
        // Major versions must match for compatibility
        if self.major != other.major {
            return false;
        }
        
        // Pre-release versions are only compatible with exact matches
        match (&self.pre_release, &other.pre_release) {
            (Some(_), Some(_)) => return self == other,
            (Some(_), None) => return false,
            (None, Some(_)) => return false,
            (None, None) => {} // Continue
        }
        
        // Minor version can be greater or equal
        if self.minor < other.minor {
            return false;
        }
        
        // If minor versions are equal, patch must be greater or equal
        if self.minor == other.minor && self.patch < other.patch {
            return false;
        }
        
        true
    }
    
    /// Get the next patch version
    pub fn next_patch(&self) -> Self {
        Self {
            major: self.major,
            minor: self.minor,
            patch: self.patch + 1,
            pre_release: None,
            build: None,
        }
    }
    
    /// Get the next minor version
    pub fn next_minor(&self) -> Self {
        Self {
            major: self.major,
            minor: self.minor + 1,
            patch: 0,
            pre_release: None,
            build: None,
        }
    }
    
    /// Get the next major version
    pub fn next_major(&self) -> Self {
        Self {
            major: self.major + 1,
            minor: 0,
            patch: 0,
            pre_release: None,
            build: None,
        }
    }
    
    /// Check if this is a pre-release version
    pub fn is_pre_release(&self) -> bool {
        self.pre_release.is_some()
    }
    
    /// Check if this is a stable version
    pub fn is_stable(&self) -> bool {
        self.pre_release.is_none()
    }
}

impl fmt::Display for ComponentVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        
        if let Some(pre) = &self.pre_release {
            write!(f, "-{}", pre)?;
        }
        
        if let Some(build) = &self.build {
            write!(f, "+{}", build)?;
        }
        
        Ok(())
    }
}

impl Default for ComponentVersion {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_parsing() {
        let v = ComponentVersion::from_string("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert!(v.pre_release.is_none());
        assert!(v.build.is_none());
    }
    
    #[test]
    fn test_pre_release_version() {
        let v = ComponentVersion::from_string("1.2.3-alpha.1").unwrap();
        assert_eq!(v.pre_release, Some("alpha.1".to_string()));
        assert!(v.is_pre_release());
        assert!(!v.is_stable());
    }
    
    #[test]
    fn test_compatibility() {
        let v1 = ComponentVersion::from_string("1.2.3").unwrap();
        let v2 = ComponentVersion::from_string("1.2.4").unwrap();
        let v3 = ComponentVersion::from_string("1.3.0").unwrap();
        let v4 = ComponentVersion::from_string("2.0.0").unwrap();
        
        assert!(v2.compatible_with(&v1)); // Same major, higher patch
        assert!(v3.compatible_with(&v1)); // Same major, higher minor
        assert!(!v4.compatible_with(&v1)); // Different major
    }
    
    #[test]
    fn test_version_increment() {
        let v = ComponentVersion::from_string("1.2.3").unwrap();
        
        let next_patch = v.next_patch();
        assert_eq!(next_patch.to_string(), "1.2.4");
        
        let next_minor = v.next_minor();
        assert_eq!(next_minor.to_string(), "1.3.0");
        
        let next_major = v.next_major();
        assert_eq!(next_major.to_string(), "2.0.0");
    }
}