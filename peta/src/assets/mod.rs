//! Asset processing module

pub mod css;
pub mod js;
pub mod images;
pub mod minifier;
pub mod pipeline;

pub use css::CssProcessor;
pub use js::JsProcessor;
pub use images::ImageProcessor;
pub use minifier::Minifier;
pub use pipeline::AssetPipeline;