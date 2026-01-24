//! Progress reporting

use std::time::{Duration, Instant};

/// Progress reporter
pub struct ProgressReporter {
    total: usize,
    current: usize,
    start_time: Instant,
    last_update: Instant,
    update_interval: Duration,
}

impl ProgressReporter {
    /// Create a new progress reporter
    pub fn new(total: usize) -> Self {
        Self {
            total,
            current: 0,
            start_time: Instant::now(),
            last_update: Instant::now(),
            update_interval: Duration::from_millis(100),
        }
    }
    
    /// Increment progress
    pub fn increment(&mut self) {
        self.current += 1;
        self.try_update();
    }
    
    /// Add multiple to progress
    pub fn add(&mut self, count: usize) {
        self.current += count;
        self.try_update();
    }
    
    /// Set current progress
    pub fn set(&mut self, current: usize) {
        self.current = current;
        self.try_update();
    }
    
    /// Try to update display
    fn try_update(&mut self) {
        let now = Instant::now();
        if now - self.last_update >= self.update_interval {
            self.display();
            self.last_update = now;
        }
    }
    
    /// Display progress
    fn display(&self) {
        let percentage = if self.total > 0 {
            (self.current * 100) / self.total
        } else {
            100
        };
        
        let elapsed = self.start_time.elapsed();
        let rate = if elapsed.as_secs() > 0 {
            self.current as f64 / elapsed.as_secs() as f64
        } else {
            0.0
        };
        
        let eta = if rate > 0.0 {
            Duration::from_secs_f64((self.total - self.current) as f64 / rate)
        } else {
            Duration::from_secs(0)
        };
        
        eprint!(
            "\rProgress: {}% ({}/{}) | Rate: {:.1}/s | ETA: {}",
            percentage,
            self.current,
            self.total,
            rate,
            format_duration(eta)
        );
    }
    
    /// Finish progress
    pub fn finish(&self) {
        eprintln!("\nCompleted {} items in {}", self.total, format_duration(self.start_time.elapsed()));
    }
}

/// Format duration for display
fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    
    if total_seconds < 60 {
        format!("{}s", total_seconds)
    } else if total_seconds < 3600 {
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{}m {}s", minutes, seconds)
    } else {
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    }
}