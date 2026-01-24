//! HTTP utilities

/// HTTP utilities
pub struct HttpUtils;

impl HttpUtils {
    /// Check if URL is valid
    pub fn is_valid_url(url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }
    
    /// Get URL scheme
    pub fn get_scheme(url: &str) -> Option<String> {
        url::Url::parse(url)
            .ok()
            .map(|u| u.scheme().to_string())
    }
}