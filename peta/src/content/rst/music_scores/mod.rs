//! Music score rendering module

pub mod models;
pub mod parser;
pub mod renderer;

pub use models::*;
pub use parser::MusicScoreParser;

use crate::core::Result;

/// Main music score renderer that dispatches to specific renderers
pub struct MusicScoreRenderer {
    parser: MusicScoreParser,
}

impl MusicScoreRenderer {
    /// Create a new music score renderer
    pub fn new() -> Result<Self> {
        Ok(Self {
            parser: MusicScoreParser::new(),
        })
    }

    /// Render a music score from text
    pub fn render(&self, score_type: &str, content: &str, title: Option<&str>) -> Result<String> {
        let score = self.parser.parse(score_type, content)?;
        
        match score {
            MusicScore::Abc(score) => renderer::AbcRenderer::new()?.render(&score, title),
        }
    }
}

impl Default for MusicScoreRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create MusicScoreRenderer")
    }
}