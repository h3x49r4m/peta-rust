//! Flowchart renderer

use crate::core::Result;
use crate::content::rst::diagrams::models::{FlowchartDiagram, FlowchartNode, FlowchartNodeType};

/// Flowchart renderer
pub struct FlowchartRenderer;

impl FlowchartRenderer {
    /// Create a new flowchart renderer
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Render a flowchart to HTML with embedded SVG
    pub fn render(&self, diagram: &FlowchartDiagram) -> Result<String> {
        // Calculate layout
        let layout = Self::calculate_layout(diagram);

        // Generate SVG
        let svg = self.generate_svg(diagram, &layout);

        // Generate HTML container
        let diagram_id = format!("flowchart-{}", uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("0"));

        let html = format!(
            r#"<div class="diagram-container" data-diagram-id="{}" data-diagram-type="flowchart">
  <svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg" class="diagram-svg">
    {}
  </svg>
  <details class="diagram-source">
    <summary>View source</summary>
    <pre>flowchart\n</pre>
  </details>
</div>"#,
            diagram_id, layout.width, layout.height, svg
        );

        Ok(html)
    }

    /// Calculate node positions (simple hierarchical layout)
    fn calculate_layout(diagram: &FlowchartDiagram) -> FlowchartLayout {
        let mut layout = FlowchartLayout {
            nodes: Vec::new(),
            edges: Vec::new(),
            width: 800.0,
            height: 400.0,
        };

        // Simple horizontal layout with levels
        let mut levels: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        // Calculate levels using BFS
        let mut queue: Vec<(String, usize)> = Vec::new();
        
        // Find nodes with no incoming edges as starting points
        let mut has_incoming: std::collections::HashSet<String> = std::collections::HashSet::new();
        for edge in &diagram.edges {
            has_incoming.insert(edge.to.clone());
        }
        
        for node in &diagram.nodes {
            if !has_incoming.contains(&node.id) {
                queue.push((node.id.clone(), 0));
                levels.insert(node.id.clone(), 0);
            }
        }

        // BFS to calculate levels
        while !queue.is_empty() {
            queue.sort_by_key(|(_, level)| *level);
            let (node_id, level) = queue.remove(0);
            
            if visited.contains(&node_id) {
                continue;
            }
            visited.insert(node_id.clone());

            // Find outgoing edges
            for edge in &diagram.edges {
                if edge.from == node_id {
                    let new_level = level + 1;
                    let current_level = levels.get(&edge.to).copied().unwrap_or(usize::MAX);
                    if new_level < current_level {
                        levels.insert(edge.to.clone(), new_level);
                        queue.push((edge.to.clone(), new_level));
                    }
                }
            }
        }

        // Ensure all nodes have a level
        for node in &diagram.nodes {
            if !levels.contains_key(&node.id) {
                levels.insert(node.id.clone(), 0);
            }
        }

        // Group nodes by level
        let mut level_nodes: std::collections::HashMap<usize, Vec<&FlowchartNode>> = std::collections::HashMap::new();
        for node in &diagram.nodes {
            let level = levels.get(&node.id).copied().unwrap_or(0);
            level_nodes.entry(level).or_insert_with(Vec::new).push(node);
        }

        // Calculate positions
        let _level_width = 150.0;
        let level_height = 100.0;
        let node_width = 120.0;
        let node_height = 50.0;

        for (level, nodes_at_level) in &level_nodes {
            let level_width_total = nodes_at_level.len() as f64 * node_width + (nodes_at_level.len() as f64 - 1.0) * 20.0;
            let start_x = (layout.width - level_width_total) / 2.0;

            for (idx, node) in nodes_at_level.iter().enumerate() {
                let x = start_x + idx as f64 * (node_width + 20.0);
                let y = 50.0 + *level as f64 * level_height;
                
                layout.nodes.push(NodePosition {
                    id: node.id.clone(),
                    x,
                    y,
                    width: node_width,
                    height: node_height,
                });
            }
        }

        // Update canvas size
        if !level_nodes.is_empty() {
            let max_level = level_nodes.keys().max().unwrap_or(&0);
            layout.height = 100.0 + (max_level + 1) as f64 * level_height + 50.0;
        }

        layout
    }

    /// Generate SVG content
    fn generate_svg(&self, diagram: &FlowchartDiagram, layout: &FlowchartLayout) -> String {
        let mut svg = String::new();

        // Add definitions for arrowhead
        svg.push_str(r##"
  <defs>
    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="#3b82f6"/>
    </marker>
  </defs>
"##);

        // Add edges first (so they appear behind nodes)
        for edge in &diagram.edges {
            if let (Some(from_pos), Some(to_pos)) = (
                layout.nodes.iter().find(|n| n.id == edge.from),
                layout.nodes.iter().find(|n| n.id == edge.to),
            ) {
                let from_x = from_pos.x + from_pos.width / 2.0;
                let from_y = from_pos.y + from_pos.height / 2.0;
                let to_x = to_pos.x + to_pos.width / 2.0;
                let to_y = to_pos.y + to_pos.height / 2.0;

                svg.push_str(&format!(
                    r##"    <path d="M {} {} L {} {}" stroke="#3b82f6" stroke-width="2" marker-end="url(#arrowhead)" />
"##,
                    from_x, from_y, to_x, to_y
                ));

                // Add label if present
                if let Some(label) = &edge.label {
                    let mid_x = (from_x + to_x) / 2.0;
                    let mid_y = (from_y + to_y) / 2.0 - 10.0;
                    svg.push_str(&format!(
                        r##"    <text x="{}" y="{}" text-anchor="middle" font-size="12" font-family="Inter" fill="#374151">{}</text>
"##,
                        mid_x, mid_y, label
                    ));
                }
            }
        }

        // Add nodes
        for node in &diagram.nodes {
            if let Some(pos) = layout.nodes.iter().find(|n| n.id == node.id) {
                let (fill, stroke, rx) = match node.node_type {
                    FlowchartNodeType::StartEnd => {
                        ("#d1fae5", "#059669", 25.0)
                    }
                    FlowchartNodeType::Decision => {
                        ("#fef3c7", "#d97706", 8.0)
                    }
                    _ => ("#dbeafe", "#2563eb", 8.0)
                };

                svg.push_str(&format!(
                    r##"    <rect x="{}" y="{}" width="{}" height="{}" rx="{}" fill="{}" stroke="{}" stroke-width="2"/>
"##,
                    pos.x, pos.y, pos.width, pos.height, rx, fill, stroke
                ));

                svg.push_str(&format!(
                    r##"    <text x="{}" y="{}" text-anchor="middle" font-size="14" font-family="Inter" fill="#1f2937">{}</text>
"##,
                    pos.x + pos.width / 2.0,
                    pos.y + pos.height / 2.0 + 5.0,
                    node.label
                ));
            }
        }

        svg
    }
}

impl Default for FlowchartRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create FlowchartRenderer")
    }
}

/// Layout information for a flowchart
#[allow(dead_code)]
struct FlowchartLayout {
    nodes: Vec<NodePosition>,
    edges: Vec<EdgePath>,
    width: f64,
    height: f64,
}

/// Node position in layout
#[allow(dead_code)]
struct NodePosition {
    id: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

/// Edge path in layout
#[allow(dead_code)]
struct EdgePath {
    from: String,
    to: String,
    path: String,
}