//! URL generation tests

use peta::utils::url::build_url;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url_empty_base() {
        // Test with empty base_url (localhost)
        assert_eq!(build_url("", "css/main.css"), "/css/main.css");
        assert_eq!(build_url("", "/css/main.css"), "/css/main.css");
        assert_eq!(build_url("", "books.html"), "/books.html");
        assert_eq!(build_url("", ""), "/");
        assert_eq!(build_url("", "snippets/test.html"), "/snippets/test.html");
    }

    #[test]
    fn test_build_url_with_base() {
        // Test with non-empty base_url (GitHub Pages)
        assert_eq!(build_url("/peta-rust", "css/main.css"), "/peta-rust/css/main.css");
        assert_eq!(build_url("/peta-rust", "/css/main.css"), "/peta-rust/css/main.css");
        assert_eq!(build_url("/peta-rust", "books.html"), "/peta-rust/books.html");
        assert_eq!(build_url("/peta-rust", ""), "/peta-rust/");
        assert_eq!(build_url("/peta-rust", "snippets/test.html"), "/peta-rust/snippets/test.html");
    }

    #[test]
    fn test_build_url_trailing_slash_base() {
        // Test base_url with trailing slash
        assert_eq!(build_url("/peta-rust/", "css/main.css"), "/peta-rust/css/main.css");
        assert_eq!(build_url("/peta-rust/", "/css/main.css"), "/peta-rust/css/main.css");
        assert_eq!(build_url("/peta-rust/", "books.html"), "/peta-rust/books.html");
    }

    #[test]
    fn test_build_url_nested_paths() {
        // Test nested paths
        assert_eq!(
            build_url("/peta-rust", "books/deep-learning/intro.html"),
            "/peta-rust/books/deep-learning/intro.html"
        );
        assert_eq!(
            build_url("", "books/deep-learning/intro.html"),
            "/books/deep-learning/intro.html"
        );
        assert_eq!(
            build_url("/site", "tags/rust-programming.html"),
            "/site/tags/rust-programming.html"
        );
    }

    #[test]
    fn test_build_url_special_characters() {
        // Test paths with special characters
        assert_eq!(
            build_url("/peta-rust", "articles/c++-basics.html"),
            "/peta-rust/articles/c++-basics.html"
        );
        assert_eq!(
            build_url("", "articles/c++-basics.html"),
            "/articles/c++-basics.html"
        );
    }

    #[test]
    fn test_build_url_multiple_leading_slashes() {
        // Test paths with multiple leading slashes
        assert_eq!(build_url("", "//css/main.css"), "/css/main.css");
        assert_eq!(build_url("/base", "//css/main.css"), "/base/css/main.css");
    }

    #[test]
    fn test_build_url_consistency() {
        // Ensure consistent behavior regardless of leading slash in path
        let base = "/site";
        let path1 = "test.html";
        let path2 = "/test.html";
        
        assert_eq!(build_url(base, path1), build_url(base, path2));
    }
}