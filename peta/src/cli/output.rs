//! Output formatting utilities

use std::io::{self, Write};
use termcolor::{Color, ColorChoice, StandardStream, WriteColor};

/// Output formatter for CLI commands
pub struct OutputFormatter {
    stream: StandardStream,
}

impl OutputFormatter {
    /// Create a new output formatter
    pub fn new() -> Self {
        Self {
            stream: StandardStream::stdout(ColorChoice::Auto),
        }
    }
    
    /// Print an info message
    pub fn info(&self, message: &str) {
        let _ = self.print_with_color(Color::Cyan, "ℹ", message);
    }
    
    /// Print a success message
    pub fn success(&self, message: &str) {
        let _ = self.print_with_color(Color::Green, "✓", message);
    }
    
    /// Print a warning message
    pub fn warn(&self, message: &str) {
        let _ = self.print_with_color(Color::Yellow, "⚠", message);
    }
    
    /// Print an error message
    pub fn error(&self, message: &str) {
        let _ = self.print_with_color(Color::Red, "✗", message);
    }
    
    /// Print a message with color and icon
    fn print_with_color(&self, color: Color, icon: &str, message: &str) -> io::Result<()> {
        let mut stream = self.stream.lock();
        stream.set_color(ColorSpec::new().set_fg(Some(color)))?;
        write!(stream, "{} ", icon)?;
        stream.reset()?;
        writeln!(stream, "{}", message)?;
        stream.flush()
    }
}

impl Default for OutputFormatter {
    fn default() -> Self {
        Self::new()
    }
}

use termcolor::ColorSpec;