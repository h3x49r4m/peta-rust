//! Diagram rendering module

pub mod models;
pub mod parser;
pub mod flowchart;
pub mod gantt;
pub mod sequence;
pub mod class;
pub mod state;

pub use models::*;
pub use parser::DiagramParser;
pub use flowchart::FlowchartRenderer;
pub use gantt::GanttRenderer;
pub use sequence::SequenceRenderer;
pub use class::ClassRenderer;
pub use state::StateRenderer;

use crate::core::Result;

/// Main diagram renderer that dispatches to specific renderers
pub struct DiagramRenderer {
    parser: DiagramParser,
}

impl DiagramRenderer {
    /// Create a new diagram renderer
    pub fn new() -> Result<Self> {
        Ok(Self {
            parser: DiagramParser::new(),
        })
    }

    /// Render a diagram from text
    pub fn render(&self, diagram_type: &str, content: &str, title: Option<&str>) -> Result<String> {
        let diagram = self.parser.parse(diagram_type, content)?;
        
        match diagram {
            Diagram::Flowchart(d) => FlowchartRenderer::new()?.render(&d, title),
            Diagram::Gantt(d) => GanttRenderer::new()?.render(&d, title),
            Diagram::Sequence(d) => SequenceRenderer::new()?.render(&d, title),
            Diagram::Class(d) => ClassRenderer::new()?.render(&d, title),
            Diagram::State(d) => StateRenderer::new()?.render(&d, title),
        }
    }
}

impl Default for DiagramRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create DiagramRenderer")
    }
}