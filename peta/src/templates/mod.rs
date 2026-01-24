//! Template system module

pub mod engine;
pub mod renderer;
pub mod filters;
pub mod functions;

pub use engine::TemplateEngine;
pub use renderer::HtmlRenderer;