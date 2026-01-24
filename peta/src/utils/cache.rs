//! Caching system

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Cache entry
#[derive(Clone)]
struct CacheEntry<T> {
    value: T,
    expires_at: Instant,
}

/// Simple in-memory cache
pub struct Cache<T: Clone> {
    entries: HashMap<String, CacheEntry<T>>,
    default_ttl: Duration,
}

impl<T: Clone> Cache<T> {
    /// Create a new cache with default TTL
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            default_ttl: Duration::from_secs(3600), // 1 hour
        }
    }
    
    /// Create a new cache with custom TTL
    pub fn with_ttl(ttl: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            default_ttl: ttl,
        }
    }
    
    /// Get value from cache
    pub fn get(&self, key: &str) -> Option<T> {
        self.entries.get(key).and_then(|entry| {
            if entry.expires_at > Instant::now() {
                Some(entry.value.clone())
            } else {
                None
            }
        })
    }
    
    /// Set value in cache
    pub fn set(&mut self, key: String, value: T) {
        let expires_at = Instant::now() + self.default_ttl;
        self.entries.insert(key, CacheEntry { value, expires_at });
    }
    
    /// Set value with custom TTL
    pub fn set_with_ttl(&mut self, key: String, value: T, ttl: Duration) {
        let expires_at = Instant::now() + ttl;
        self.entries.insert(key, CacheEntry { value, expires_at });
    }
    
    /// Remove expired entries
    pub fn cleanup(&mut self) {
        let now = Instant::now();
        self.entries.retain(|_, entry| entry.expires_at > now);
    }
    
    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl<T: Clone> Default for Cache<T> {
    fn default() -> Self {
        Self::new()
    }
}