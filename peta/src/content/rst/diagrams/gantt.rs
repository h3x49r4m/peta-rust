//! Gantt chart renderer

use crate::core::Result;
use crate::content::rst::diagrams::models::GanttDiagram;
use chrono::{NaiveDate, Duration};

/// Gantt chart renderer
pub struct GanttRenderer;

impl GanttRenderer {
    /// Create a new gantt renderer
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Render a gantt chart to HTML with embedded SVG
    pub fn render(&self, diagram: &GanttDiagram) -> Result<String> {
        // Calculate layout
        let layout = Self::calculate_layout(diagram);

        // Generate SVG
        let svg = self.generate_svg(diagram, &layout);

        // Generate HTML container
        let diagram_id = format!("gantt-{}", uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("0"));

        let html = format!(
            r#"<div class="diagram-container" data-diagram-id="{}" data-diagram-type="gantt">
  <button class="diagram-download" data-diagram-id="{}" data-diagram-type="gantt" aria-label="Download diagram as SVG">
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
    fn calculate_layout(diagram: &GanttDiagram) -> GanttLayout {
        let mut layout = GanttLayout {
            tasks: Vec::new(),
            width: 800.0,
            height: 400.0,
        };

        // Calculate date range
        let start_date = NaiveDate::parse_from_str(&diagram.start_date, "%Y-%m-%d")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        let end_date = NaiveDate::parse_from_str(&diagram.end_date, "%Y-%m-%d")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        
        let total_days = (end_date - start_date).num_days() as f64;
        let days_per_pixel = total_days / 600.0;

        // Calculate task positions
        let row_height = 40.0;
        let header_height = 50.0;
        let left_margin = 20.0;
        let top_margin = header_height;

        for (idx, task) in diagram.tasks.iter().enumerate() {
            let task_start = NaiveDate::parse_from_str(&task.start_date, "%Y-%m-%d")
                .unwrap_or(start_date);
            let days_from_start = (task_start - start_date).num_days() as f64;
            
            let x = left_margin + days_from_start / days_per_pixel;
            let y = top_margin + idx as f64 * row_height;
            let width = task.duration_days as f64 / days_per_pixel;
            let height = 30.0;

            layout.tasks.push(TaskPosition {
                id: task.id.clone(),
                label: task.label.clone(),
                x,
                y,
                width,
                height,
            });
        }

        layout.height = header_height + diagram.tasks.len() as f64 * row_height + 20.0;

        layout
    }

    /// Generate SVG content
    fn generate_svg(&self, diagram: &GanttDiagram, layout: &GanttLayout) -> String {
        let mut svg = String::new();

        // Calculate date range
        let start_date = NaiveDate::parse_from_str(&diagram.start_date, "%Y-%m-%d")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        let end_date = NaiveDate::parse_from_str(&diagram.end_date, "%Y-%m-%d")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let total_days = (end_date - start_date).num_days();
        let days_per_pixel = total_days as f64 / 600.0;

        // Draw grid lines for weeks
        svg.push_str("    <!-- Grid lines -->\n");
        for week in 0..=(total_days / 7) {
            let day = week * 7;
            let x = 20.0 + day as f64 / days_per_pixel;
            svg.push_str(&format!(
                r##"    <line x1="{}" y1="50" x2="{}" y2="{}" stroke="#e5e7eb" stroke-width="1" stroke-dasharray="4"/>
"##,
                x, x, layout.height - 20.0
            ));
        }

        // Draw header with date labels
        svg.push_str("    <!-- Date labels -->\n");
        for day in (0..=total_days).step_by(7) {
            let date = start_date + Duration::days(day);
            let x = 20.0 + day as f64 / days_per_pixel;
            svg.push_str(&format!(
                r##"    <text x="{}" y="30" text-anchor="middle" font-size="10" font-family="Inter" fill="#6b7280">{}</text>
"##,
                x, date.format("%m/%d")
            ));
        }

        // Draw tasks
        svg.push_str("    <!-- Tasks -->\n");
        for task in &layout.tasks {
            svg.push_str(&format!(
                r##"    <rect x="{}" y="{}" width="{}" height="{}" rx="4" fill="#3b82f6" stroke="#2563eb" stroke-width="1"/>
"##,
                task.x, task.y, task.width, task.height
            ));
            svg.push_str(&format!(
                r##"    <text x="{}" y="{}" text-anchor="start" font-size="11" font-family="Inter" fill="#ffffff">{}</text>
"##,
                task.x + 5.0, task.y + 20.0, task.label
            ));
        }

        svg
    }
}

impl Default for GanttRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create GanttRenderer")
    }
}

/// Layout information for a gantt chart
struct GanttLayout {
    tasks: Vec<TaskPosition>,
    width: f64,
    height: f64,
}

/// Task position in layout
#[allow(dead_code)]
struct TaskPosition {
    id: String,
    label: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}
