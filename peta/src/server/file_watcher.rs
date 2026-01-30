//! File system watcher

use crate::core::Result;
use std::path::Path;
use notify::{Watcher, RecursiveMode, RecommendedWatcher, Event};
use std::sync::mpsc::Receiver;
use tokio::sync::mpsc as tokio_mpsc;

/// File watcher
pub struct FileWatcher {
    #[allow(dead_code)]
    watcher: RecommendedWatcher,
    receiver: Receiver<std::result::Result<Event, notify::Error>>,
    watch_path: std::path::PathBuf,
}

impl FileWatcher {
    /// Create a new file watcher
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let (tx, rx) = std::sync::mpsc::channel();
        
        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        }).map_err(|e| crate::core::Error::server(e.to_string()))?;
        
        let watch_path = path.as_ref().to_path_buf();
        
        // Watch the directory
        watcher.watch(&watch_path, RecursiveMode::Recursive)
            .map_err(|e| crate::core::Error::server(e.to_string()))?;
        
        Ok(Self { watcher, receiver: rx, watch_path })
    }
    
    /// Start watching and send events to a channel
    pub async fn start(self, event_sender: tokio_mpsc::Sender<std::path::PathBuf>) -> Result<()> {
        let watch_path: std::path::PathBuf = self.watch_path;
        
        // Spawn a task to process file change events
        tokio::spawn(async move {
            while let Ok(event) = self.receiver.recv() {
                if let Ok(event) = event {
                    // Filter for relevant file changes
                    for path in event.paths {
                        // Only process files in watched directory
                        if path.starts_with(&watch_path) {
                            // Filter out temporary files and hidden files
                            let file_name = path.file_name()
                                .and_then(|n: &std::ffi::OsStr| n.to_str())
                                .unwrap_or("");
                            
                            // Skip temporary files, hidden files, and build artifacts
                            if !file_name.starts_with('.') 
                                && !file_name.ends_with('~')
                                && !file_name.ends_with(".swp")
                                && !file_name.ends_with(".tmp")
                                && !path.to_string_lossy().contains("/_out/")
                                && !path.to_string_lossy().contains("/target/") {
                                
                                // Send the changed file path
                                let _ = event_sender.send(path).await;
                            }
                        }
                    }
                }
            }
        });
        
        Ok(())
    }
}