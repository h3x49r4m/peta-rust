//! Asset processing module

pub mod css;
pub mod js;
pub mod images;
pub mod minifier;
pub mod pipeline;
pub mod css_generator;
pub mod js_generator;

pub use css::CssProcessor;
pub use js::JsProcessor;
pub use images::ImageProcessor;
pub use minifier::Minifier;
pub use pipeline::AssetPipeline;
pub use css_generator::{CssGenerator, CssConfig, DiagramCssGenerator, MusicScoreCssGenerator};
pub use js_generator::{JsGenerator, JsConfig, DiagramJsGenerator, MusicScoreJsGenerator};