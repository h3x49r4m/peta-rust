//! Development server

use crate::core::{Site, Result};
use crate::server::file_watcher::FileWatcher;
use crate::server::livereload::LiveReload;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Router, routing::get, response::Html};
use tokio::sync::mpsc as tokio_mpsc;
use tokio::sync::Mutex;

/// Development server
pub struct DevServer {
    port: u16,
    site: Site,
    livereload: Arc<Mutex<LiveReload>>,
}

impl DevServer {
    /// Create a new development server
    pub fn new(port: u16, site: Site) -> Result<Self> {
        let livereload = Arc::new(Mutex::new(LiveReload::new()));
        
        Ok(Self {
            port,
            site,
            livereload,
        })
    }
    
    /// Start the development server
    pub async fn start(&mut self) -> Result<()> {
        // Create channel for file change events
        let (event_sender, mut event_receiver) = tokio_mpsc::channel::<std::path::PathBuf>(100);
        
        // Create and start file watcher
        let file_watcher = FileWatcher::new(&self.site.config.build.content_dir)?;
        file_watcher.start(event_sender).await?;
        
        // Start live reload
        self.livereload.lock().await.start(self.port).await?;
        
        // Spawn task to handle file changes
        let livereload = self.livereload.clone();
        let config = self.site.config.clone();
        
        tokio::spawn(async move {
            let mut last_rebuild_time = std::time::Instant::now();
            let rebuild_delay = std::time::Duration::from_millis(300); // Debounce rebuilds
            
            while let Some(_changed_path) = event_receiver.recv().await {
                let now = std::time::Instant::now();
                
                // Debounce rebuilds to avoid rebuilding on every file change
                if now.duration_since(last_rebuild_time) < rebuild_delay {
                    tokio::time::sleep(rebuild_delay).await;
                }
                
                // Rebuild site
                println!("🔄 Rebuilding site...");
                let mut builder = crate::core::builder::SiteBuilder::new(config.clone());
                match builder.build().await {
                    Ok(_) => {
                        println!("✓ Site rebuilt successfully");
                        // Trigger live reload
                        if let Err(e) = livereload.lock().await.trigger().await {
                            eprintln!("Failed to trigger live reload: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to rebuild site: {}", e);
                    }
                }
                
                last_rebuild_time = std::time::Instant::now();
            }
        });
        
        // Create router
        let router = self.create_router();
        
        // Start HTTP server
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = tokio::net::TcpListener::bind(addr).await?;
        println!("🚀 Development server running at http://{}", addr);
        axum::serve(listener, router.into_make_service()).await?;
        
        Ok(())
    }
    
    /// Create router for serving files
    fn create_router(&self) -> Router {
        Router::new()
            .route("/", get(|| async { 
                match std::fs::read_to_string("_out/dist/index.html") {
                    Ok(content) => Html(content),
                    Err(_) => Html("<h1>Error: Site not built</h1>".to_string()),
                }
            }))
            .route("/livereload.js", get(|| async { Html(include_str!("static/livereload.js")) }))
            .route("/articles", get(|| async { 
                match std::fs::read_to_string("_out/dist/articles.html") {
                    Ok(content) => Html(content),
                    Err(_) => Html("<h1>Articles not found</h1>".to_string()),
                }
            }))
            .route("/snippets", get(|| async { 
                match std::fs::read_to_string("_out/dist/snippets.html") {
                    Ok(content) => Html(content),
                    Err(_) => Html("<h1>Snippets not found</h1>".to_string()),
                }
            }))
            .route("/books", get(|| async { 
                match std::fs::read_to_string("_out/dist/books.html") {
                    Ok(content) => Html(content),
                    Err(_) => Html("<h1>Books not found</h1>".to_string()),
                }
            }))
            .route("/projects", get(|| async { 
                match std::fs::read_to_string("_out/dist/projects.html") {
                    Ok(content) => Html(content),
                    Err(_) => Html("<h1>Projects not found</h1>".to_string()),
                }
            }))
            .route("/search", get(|| async { 
                match std::fs::read_to_string("_out/dist/search.html") {
                    Ok(content) => Html(content),
                    Err(_) => Html("<h1>Search not found</h1>".to_string()),
                }
            }))
            .route("/*path", get(serve_file))
    }
}

/// Serve static files
async fn serve_file(path: axum::extract::Path<String>) -> axum::response::Response {
    let path = path.0;
    let output_dir = std::path::Path::new("_out/dist");
    
    // Try the path as-is first
    let file_path = output_dir.join(&path);
    
    // If not found, try adding .html extension
    let final_path = if !file_path.exists() && !path.ends_with(".html") {
        output_dir.join(format!("{}.html", path))
    } else {
        file_path
    };
    
    match std::fs::read(&final_path) {
        Ok(contents) => {
            let mime_type = mime_guess::from_path(&final_path)
                .first_or_octet_stream()
                .as_ref()
                .to_string();
            
            axum::response::Response::builder()
                .status(axum::http::StatusCode::OK)
                .header("Content-Type", mime_type)
                .body(axum::body::Body::from(contents))
                .unwrap()
        }
        Err(_) => {
            // Return 404 page instead of NOT_FOUND status
            match std::fs::read_to_string("_out/dist/404.html") {
                Ok(content) => {
                    axum::response::Response::builder()
                        .status(axum::http::StatusCode::NOT_FOUND)
                        .header("Content-Type", "text/html")
                        .body(axum::body::Body::from(content))
                        .unwrap()
                }
                Err(_) => {
                    // Fallback if 404.html is not available
                    let fallback_html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 - Page Not Found</title>
    <style>
        body { font-family: system-ui, -apple-system, sans-serif; text-align: center; padding: 2rem; }
        h1 { font-size: 4rem; color: #3b82f6; margin-bottom: 1rem; }
        p { color: #64748b; margin-bottom: 2rem; }
        a { color: #3b82f6; text-decoration: none; font-weight: 600; }
        a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <h1>404</h1>
    <p>Page not found. <a href="/">Go home</a></p>
</body>
</html>
                    "#;
                    
                    axum::response::Response::builder()
                        .status(axum::http::StatusCode::NOT_FOUND)
                        .header("Content-Type", "text/html")
                        .body(axum::body::Body::from(fallback_html))
                        .unwrap()
                }
            }
        }
    }
}