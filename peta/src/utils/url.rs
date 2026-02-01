//! URL utility functions for building consistent URLs with base_url support

/// Build a URL by combining base_url with a path
///
/// # Arguments
/// * `base_url` - The base URL (can be empty for localhost or "/path" for subdirectory deployment)
/// * `path` - The path to append (with or without leading slash)
///
/// # Returns
/// A properly formatted URL that respects the base_url
///
/// # Examples
/// ```
/// use peta::utils::url::build_url;
///
/// // Empty base_url (localhost)
/// assert_eq!(build_url("", "css/main.css"), "/css/main.css");
/// assert_eq!(build_url("", "/css/main.css"), "/css/main.css");
/// assert_eq!(build_url("", "books.html"), "/books.html");
///
/// // With base_url (GitHub Pages)
/// assert_eq!(build_url("/peta-rust", "css/main.css"), "/peta-rust/css/main.css");
/// assert_eq!(build_url("/peta-rust", "/css/main.css"), "/peta-rust/css/main.css");
/// assert_eq!(build_url("/peta-rust", "books.html"), "/peta-rust/books.html");
///
/// // Base URL with trailing slash
/// assert_eq!(build_url("/peta-rust/", "css/main.css"), "/peta-rust/css/main.css");
/// ```
pub fn build_url(base_url: &str, path: &str) -> String {
    let clean_path = path.trim_start_matches('/');
    if base_url.is_empty() {
        format!("/{}", clean_path)
    } else {
        format!("{}/{}", base_url.trim_end_matches('/'), clean_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url_empty_base() {
        assert_eq!(build_url("", "css/main.css"), "/css/main.css");
        assert_eq!(build_url("", "/css/main.css"), "/css/main.css");
        assert_eq!(build_url("", "books.html"), "/books.html");
        assert_eq!(build_url("", ""), "/");
    }

    #[test]
    fn test_build_url_with_base() {
        assert_eq!(build_url("/peta-rust", "css/main.css"), "/peta-rust/css/main.css");
        assert_eq!(build_url("/peta-rust", "/css/main.css"), "/peta-rust/css/main.css");
        assert_eq!(build_url("/peta-rust", "books.html"), "/peta-rust/books.html");
        assert_eq!(build_url("/peta-rust", ""), "/peta-rust/");
    }

    #[test]
    fn test_build_url_trailing_slash_base() {
        assert_eq!(build_url("/peta-rust/", "css/main.css"), "/peta-rust/css/main.css");
        assert_eq!(build_url("/peta-rust/", "/css/main.css"), "/peta-rust/css/main.css");
        assert_eq!(build_url("/peta-rust/", "books.html"), "/peta-rust/books.html");
    }

    #[test]
    fn test_build_url_nested_paths() {
        assert_eq!(build_url("/peta-rust", "books/deep-learning/intro.html"), "/peta-rust/books/deep-learning/intro.html");
        assert_eq!(build_url("", "books/deep-learning/intro.html"), "/books/deep-learning/intro.html");
    }
}