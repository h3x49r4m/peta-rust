//! Command-line interface module

pub mod commands;
pub mod args;
pub mod output;

pub use args::{Cli, Commands, ThemeAction};
pub use output::OutputFormatter;