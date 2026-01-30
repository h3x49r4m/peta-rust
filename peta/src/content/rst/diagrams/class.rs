//! Class diagram renderer

use crate::core::Result;
use crate::content::rst::diagrams::models::ClassDiagram;

/// Class diagram renderer
pub struct ClassRenderer;

impl ClassRenderer {
    /// Create a new class renderer
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Render a class diagram to HTML with embedded SVG
    pub fn render(&self, diagram: &ClassDiagram, title: Option<&str>) -> Result<String> {
        // Calculate layout
        let layout = Self::calculate_layout(diagram, title);

        // Generate SVG
        let svg = self.generate_svg(diagram, &layout, title);

        // Generate HTML container
        let diagram_id = format!("class-{}", uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("0"));

        let html = format!(
            r#"<div class="diagram-container" data-diagram-id="{}" data-diagram-type="class">
  <button class="diagram-download" data-diagram-id="{}" data-diagram-type="class" aria-label="Download diagram as SVG">
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

    /// Calculate layout (simple tree-like layout)
    fn calculate_layout(diagram: &ClassDiagram, title: Option<&str>) -> ClassLayout {
        let mut layout = ClassLayout {
            classes: Vec::new(),
            relationships: Vec::new(),
            width: 600.0,
            height: 400.0,
        };

        // Adjust height if title is present
        if title.is_some() {
            layout.height += 40.0;
        }

        let y_offset = title.map(|_| 40.0).unwrap_or(0.0);

        // Simple horizontal layout
        let class_width = 140.0;
        let spacing = 60.0;
        let left_margin = 50.0;

        for (idx, class) in diagram.classes.iter().enumerate() {
            let x = left_margin + idx as f64 * (class_width + spacing);
            layout.classes.push(ClassPosition {
                id: class.id.clone(),
                label: class.label.clone(),
                x,
                y: 100.0 + y_offset,
                width: class_width,
                height: 120.0,
            });
        }

        layout.width = 50.0 + diagram.classes.len() as f64 * (class_width + spacing) + 50.0;
        layout.height = 400.0 + y_offset;

        layout
    }

    /// Generate SVG content
    fn generate_svg(&self, diagram: &ClassDiagram, layout: &ClassLayout, title: Option<&str>) -> String {
        let mut svg = String::new();

        // Add title if present
        if let Some(title_text) = title {
            svg.push_str(&format!(
                r##"    <text x="{}" y="25" text-anchor="middle" font-size="18" font-weight="bold" font-family="Inter" fill="#1f2937">{}</text>
"##,
                layout.width / 2.0, title_text
            ));
        }

        // Add definitions for relationship markers
        svg.push_str(r##"
  <defs>
    <marker id="arrowhead-inheritance" markerWidth="12" markerHeight="12" refX="10" refY="6" orient="auto">
      <polygon points="0 0, 12 6, 0 12" fill="none" stroke="#3b82f6" stroke-width="2"/>
    </marker>
    <marker id="diamond-composition" markerWidth="14" markerHeight="14" refX="14" refY="7" orient="auto">
      <polygon points="0 7, 7 0, 14 7, 7 14" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
    </marker>
    <marker id="diamond-aggregation" markerWidth="14" markerHeight="14" refX="14" refY="7" orient="auto">
      <polygon points="0 7, 7 0, 14 7, 7 14" fill="none" stroke="#2563eb" stroke-width="2"/>
    </marker>
  </defs>
"##);

        // Draw relationships first
        svg.push_str("    <!-- Relationships -->\n");
        for rel in &diagram.relationships {
            if let (Some(from_pos), Some(to_pos)) = (
                layout.classes.iter().find(|c| c.id == rel.from),
                layout.classes.iter().find(|c| c.id == rel.to),
            ) {
                let from_x = from_pos.x + from_pos.width / 2.0;
                let from_y = from_pos.y + from_pos.height;
                let to_x = to_pos.x + to_pos.width / 2.0;
                let to_y = to_pos.y;

                let (marker, stroke) = match rel.relationship_type {
                    crate::content::rst::diagrams::models::ClassRelationshipType::Inheritance => {
                        ("url(#arrowhead-inheritance)", "#3b82f6")
                    }
                    crate::content::rst::diagrams::models::ClassRelationshipType::Composition => {
                        ("url(#diamond-composition)", "#2563eb")
                    }
                    crate::content::rst::diagrams::models::ClassRelationshipType::Aggregation => {
                        ("url(#diamond-aggregation)", "#2563eb")
                    }
                    _ => ("", "#3b82f6")
                };

                if marker.is_empty() {
                    svg.push_str(&format!(
                        r#"    <path d="M {} {} L {} {}" stroke="{}" stroke-width="2" />
"#,
                        from_x, from_y, to_x, to_y, stroke
                    ));
                } else {
                    svg.push_str(&format!(
                        r#"    <path d="M {} {} L {} {}" stroke="{}" stroke-width="2" marker-end="{}" />
"#,
                        from_x, from_y, to_x, to_y, stroke, marker
                    ));
                }
            }
        }

        // Draw classes
        svg.push_str("    <!-- Classes -->\n");
        for class in &layout.classes {
            // Class box
            svg.push_str(&format!(
                r##"    <rect x="{}" y="{}" width="{}" height="{}" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
"##,
                class.x, class.y, class.width, class.height
            ));
            
            // Class name
            svg.push_str(&format!(
                r##"    <text x="{}" y="{}" text-anchor="middle" font-size="13" font-weight="bold" font-family="Inter" fill="#1f2937">{}</text>
"##,
                class.x + class.width / 2.0, class.y + 20.0, class.label
            ));
            
            // Separator line
            svg.push_str(&format!(
                r##"    <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#2563eb" stroke-width="1"/>
"##,
                class.x + 5.0, class.y + 35.0, class.x + class.width - 5.0, class.y + 35.0
            ));
            
            // Attributes section
            svg.push_str(&format!(
                r##"    <text x="{}" y="{}" text-anchor="start" font-size="10" font-family="Inter" fill="#374151">Attributes</text>
"##,
                class.x + 10.0, class.y + 50.0
            ));
            
            // Methods section
            svg.push_str(&format!(
                r##"    <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#2563eb" stroke-width="1"/>
"##,
                class.x + 5.0, class.y + 65.0, class.x + class.width - 5.0, class.y + 65.0
            ));
            svg.push_str(&format!(
                r##"    <text x="{}" y="{}" text-anchor="start" font-size="10" font-family="Inter" fill="#374151">Methods</text>
"##,
                class.x + 10.0, class.y + 80.0
            ));
        }

        svg
    }
}

impl Default for ClassRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create ClassRenderer")
    }
}

/// Layout information for a class diagram
#[allow(dead_code)]
struct ClassLayout {
    classes: Vec<ClassPosition>,
    relationships: Vec<RelationshipPath>,
    width: f64,
    height: f64,
}

/// Class position in layout
#[allow(dead_code)]
struct ClassPosition {
    id: String,
    label: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

/// Relationship path in layout
#[allow(dead_code)]
struct RelationshipPath {
    from: String,
    to: String,
}