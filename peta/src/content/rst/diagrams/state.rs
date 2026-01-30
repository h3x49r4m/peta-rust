//! State diagram renderer

use crate::core::Result;
use crate::content::rst::diagrams::models::StateDiagram;

/// State diagram renderer
pub struct StateRenderer;

impl StateRenderer {
    /// Create a new state renderer
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Render a state diagram to HTML with embedded SVG
    pub fn render(&self, diagram: &StateDiagram, title: Option<&str>) -> Result<String> {
        // Calculate layout
        let layout = Self::calculate_layout(diagram, title);

        // Generate SVG
        let svg = self.generate_svg(diagram, &layout, title);

        // Generate HTML container
        let diagram_id = format!("state-{}", uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("0"));

        let html = format!(
            r#"<div class="diagram-container" data-diagram-id="{}" data-diagram-type="state">
  <button class="diagram-download" data-diagram-id="{}" data-diagram-type="state" aria-label="Download diagram as SVG">
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

    /// Calculate layout (circular layout for states)
    fn calculate_layout(diagram: &StateDiagram, title: Option<&str>) -> StateLayout {
        let mut layout = StateLayout {
            states: Vec::new(),
            transitions: Vec::new(),
            width: 600.0,
            height: 400.0,
        };

        // Adjust height if title is present
        if title.is_some() {
            layout.height += 40.0;
        }

        let y_offset = title.map(|_| 40.0).unwrap_or(0.0);

        // Circular layout
        let center_x = 300.0;
        let center_y = 200.0 + y_offset;
        let radius = 120.0;
        let state_count = diagram.states.len() as f64;

        for (idx, state) in diagram.states.iter().enumerate() {
            let angle = (idx as f64 / state_count) * 2.0 * std::f64::consts::PI;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();

            let (width, height) = match state.state_type {
                crate::content::rst::diagrams::models::StateType::Initial => (20.0, 20.0),
                crate::content::rst::diagrams::models::StateType::Final => (25.0, 25.0),
                crate::content::rst::diagrams::models::StateType::Normal => (100.0, 50.0),
            };

            layout.states.push(StatePosition {
                id: state.id.clone(),
                label: state.label.clone(),
                state_type: state.state_type.clone(),
                x,
                y,
                width,
                height,
            });
        }

        layout.height = 400.0 + y_offset;

        layout
    }

    /// Generate SVG content
    fn generate_svg(&self, diagram: &StateDiagram, layout: &StateLayout, title: Option<&str>) -> String {
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
    <marker id="arrowhead-state" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="#3b82f6"/>
    </marker>
  </defs>
"##);

        // Draw transitions first
        svg.push_str("    <!-- Transitions -->\n");
        for transition in &diagram.transitions {
            if let (Some(from_state), Some(to_state)) = (
                layout.states.iter().find(|s| s.id == transition.from),
                layout.states.iter().find(|s| s.id == transition.to),
            ) {
                let from_x = from_state.x + from_state.width / 2.0;
                let from_y = from_state.y + from_state.height / 2.0;
                let to_x = to_state.x + to_state.width / 2.0;
                let to_y = to_state.y + to_state.height / 2.0;

                svg.push_str(&format!(
                    r##"    <path d="M {} {} L {} {}" stroke="#3b82f6" stroke-width="2" marker-end="url(#arrowhead-state)"/>
"##,
                    from_x, from_y, to_x, to_y
                ));

                // Transition label
                let mid_x = (from_x + to_x) / 2.0;
                let mid_y = (from_y + to_y) / 2.0 - 10.0;
                svg.push_str(&format!(
                    r##"    <text x="{}" y="{}" text-anchor="middle" font-size="11" font-family="Inter" fill="#374151">{}</text>
"##,
                    mid_x, mid_y, transition.label
                ));
            }
        }

        // Draw states
        svg.push_str("    <!-- States -->\n");
        for state in &layout.states {
            match state.state_type {
                crate::content::rst::diagrams::models::StateType::Initial => {
                    // Initial state: small filled circle
                    svg.push_str(&format!(
                        r##"    <circle cx="{}" cy="{}" r="10" fill="#3b82f6"/>
"##,
                        state.x + state.width / 2.0,
                        state.y + state.height / 2.0
                    ));
                }
                crate::content::rst::diagrams::models::StateType::Final => {
                    // Final state: double circle
                    svg.push_str(&format!(
                        r##"    <circle cx="{}" cy="{}" r="12" fill="none" stroke="#3b82f6" stroke-width="2"/>
"##,
                        state.x + state.width / 2.0,
                        state.y + state.height / 2.0
                    ));
                    svg.push_str(&format!(
                        r##"    <circle cx="{}" cy="{}" r="8" fill="#3b82f6"/>
"##,
                        state.x + state.width / 2.0,
                        state.y + state.height / 2.0
                    ));
                }
                crate::content::rst::diagrams::models::StateType::Normal => {
                    // Normal state: rounded rectangle
                    svg.push_str(&format!(
                        r##"    <rect x="{}" y="{}" width="{}" height="{}" rx="25" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
"##,
                        state.x, state.y, state.width, state.height
                    ));
                    svg.push_str(&format!(
                        r##"    <text x="{}" y="{}" text-anchor="middle" font-size="12" font-family="Inter" fill="#1f2937">{}</text>
"##,
                        state.x + state.width / 2.0,
                        state.y + state.height / 2.0 + 4.0,
                        state.label
                    ));
                }
            }
        }

        svg
    }
}

impl Default for StateRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create StateRenderer")
    }
}

/// Layout information for a state diagram
#[allow(dead_code)]
struct StateLayout {
    states: Vec<StatePosition>,
    transitions: Vec<TransitionPath>,
    width: f64,
    height: f64,
}

/// State position in layout
#[allow(dead_code)]
struct StatePosition {
    id: String,
    label: String,
    state_type: crate::content::rst::diagrams::models::StateType,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

/// Transition path in layout
#[allow(dead_code)]
struct TransitionPath {
    from: String,
    to: String,
}
