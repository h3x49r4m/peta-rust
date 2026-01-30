//! Sequence diagram renderer

use crate::core::Result;
use crate::content::rst::diagrams::models::SequenceDiagram;

/// Sequence diagram renderer
pub struct SequenceRenderer;

impl SequenceRenderer {
    /// Create a new sequence renderer
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Render a sequence diagram to HTML with embedded SVG
    pub fn render(&self, diagram: &SequenceDiagram, title: Option<&str>) -> Result<String> {
        // Calculate layout
        let layout = Self::calculate_layout(diagram, title);

        // Generate SVG
        let svg = self.generate_svg(diagram, &layout, title);

        // Generate HTML container
        let diagram_id = format!("sequence-{}", uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("0"));

        let html = format!(
            r#"<div class="diagram-container" data-diagram-id="{}" data-diagram-type="sequence">
  <button class="diagram-download" data-diagram-id="{}" data-diagram-type="sequence" aria-label="Download diagram as SVG">
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
      <polyline points="7 10 12 15 17 10"/>
      <line x1="12" y1="15" x2="12" y2="3"/>
    </svg>
  </button>
  <svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg" class="diagram-svg">
    {}
  </svg>
</div>"#,
            diagram_id, diagram_id, layout.width, layout.height, svg
        );

        Ok(html)
    }

    /// Calculate layout
    fn calculate_layout(diagram: &SequenceDiagram, title: Option<&str>) -> SequenceLayout {
        let mut layout = SequenceLayout {
            actors: Vec::new(),
            messages: Vec::new(),
            width: 600.0,
            height: 400.0,
        };

        // Adjust height if title is present
        if title.is_some() {
            layout.height += 40.0;
        }

        let y_offset = title.map(|_| 40.0).unwrap_or(0.0);

        // Calculate actor positions
        let actor_width = 100.0;
        let spacing = 50.0;
        let left_margin = 50.0;

        for (idx, actor) in diagram.actors.iter().enumerate() {
            let x = left_margin + idx as f64 * (actor_width + spacing);
            layout.actors.push(ActorPosition {
                id: actor.id.clone(),
                label: actor.label.clone(),
                x,
                y: 30.0 + y_offset,
                width: actor_width,
                height: 40.0,
                line_end_y: 350.0 + y_offset,
            });
        }

        // Calculate message positions
        for (idx, message) in diagram.messages.iter().enumerate() {
            if let (Some(from_actor), Some(to_actor)) = (
                layout.actors.iter().find(|a| a.id == message.from),
                layout.actors.iter().find(|a| a.id == message.to),
            ) {
                let y = 100.0 + idx as f64 * 50.0 + y_offset;
                layout.messages.push(MessagePosition {
                    from: message.from.clone(),
                    to: message.to.clone(),
                    label: message.label.clone(),
                    x1: from_actor.x + from_actor.width / 2.0,
                    y1: y,
                    x2: to_actor.x + to_actor.width / 2.0,
                    y2: y,
                });
            }
        }

        layout.height = 380.0 + y_offset;
        layout.width = 50.0 + diagram.actors.len() as f64 * (actor_width + spacing) + 50.0;

        layout
    }

    /// Generate SVG content
    fn generate_svg(&self, _diagram: &SequenceDiagram, layout: &SequenceLayout, title: Option<&str>) -> String {
        let mut svg = String::new();

        // Add title if present
        if let Some(title_text) = title {
            svg.push_str(&format!(
                r##"    <text x="{}" y="25" text-anchor="middle" font-size="18" font-weight="bold" font-family="Inter" fill="#1f2937">{}</text>
"##,
                layout.width / 2.0, title_text
            ));
        }

        // Add definitions for arrowhead
        svg.push_str(r##"
  <defs>
    <marker id="arrowhead-seq" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="#3b82f6"/>
    </marker>
  </defs>
"##);

        // Draw actors and vertical lines
        svg.push_str("    <!-- Actors -->\n");
        for actor in &layout.actors {
            // Actor box
            svg.push_str(&format!(
                r##"    <rect x="{}" y="{}" width="{}" height="{}" rx="8" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
"##,
                actor.x, actor.y, actor.width, actor.height
            ));
            svg.push_str(&format!(
                r##"    <text x="{}" y="{}" text-anchor="middle" font-size="12" font-family="Inter" fill="#1f2937">{}</text>
"##,
                actor.x + actor.width / 2.0,
                actor.y + actor.height / 2.0 + 4.0,
                actor.label
            ));
            
            // Vertical line
            svg.push_str(&format!(
                r##"    <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#9ca3af" stroke-width="1" stroke-dasharray="5,5"/>
"##,
                actor.x + actor.width / 2.0,
                actor.y + actor.height,
                actor.x + actor.width / 2.0,
                actor.line_end_y
            ));
        }

        // Draw messages
        svg.push_str("    <!-- Messages -->\n");
        for message in &layout.messages {
            svg.push_str(&format!(
                r##"    <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#3b82f6" stroke-width="2" marker-end="url(#arrowhead-seq)"/>
"##,
                message.x1, message.y1, message.x2, message.y2
            ));
            
            // Message label
            let mid_x = (message.x1 + message.x2) / 2.0;
            svg.push_str(&format!(
                r##"    <text x="{}" y="{}" text-anchor="middle" font-size="11" font-family="Inter" fill="#374151">{}</text>
"##,
                mid_x, message.y1 - 5.0, message.label
            ));
        }

        svg
    }
}

impl Default for SequenceRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create SequenceRenderer")
    }
}

/// Layout information for a sequence diagram
struct SequenceLayout {
    actors: Vec<ActorPosition>,
    messages: Vec<MessagePosition>,
    width: f64,
    height: f64,
}

/// Actor position in layout
struct ActorPosition {
    id: String,
    label: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    line_end_y: f64,
}

/// Message position in layout
#[allow(dead_code)]
struct MessagePosition {
    from: String,
    to: String,
    label: String,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}