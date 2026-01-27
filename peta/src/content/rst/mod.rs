//! RST processing module

pub mod parser;
pub mod directives;
pub mod math_processor;
pub mod math_renderer;
pub mod math_css_generator;
pub mod math_js_generator;
pub mod code_highlighter;
pub mod syntax_highlighter;
pub mod code_block_renderer;
pub mod cross_ref;
pub mod toc_generator;
pub mod book_toc_generator;

pub use parser::RstParser;
pub use directives::*;
pub use math_processor::{MathProcessor, MathDetectionResult};
pub use math_renderer::MathRenderer;
pub use math_css_generator::MathCssGenerator;
pub use math_js_generator::MathJsGenerator;
pub use code_highlighter::CodeHighlighter;
pub use syntax_highlighter::{SyntaxHighlighter, HighlighterConfig};
pub use code_block_renderer::CodeBlockRenderer;
pub use book_toc_generator::BookTocGenerator;