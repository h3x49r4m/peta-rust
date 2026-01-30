//! ABC notation renderer

use crate::core::Result;
use crate::content::rst::music_scores::models::{AbcScore, MusicScoreLayout};

/// ABC notation renderer
pub struct AbcRenderer;

impl AbcRenderer {
    /// Create a new ABC renderer
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Render an ABC score to HTML with embedded SVG
    pub fn render(&self, score: &AbcScore, title: Option<&str>) -> Result<String> {
        // Calculate layout
        let layout = Self::calculate_layout(score, title);

        // Generate SVG
        let svg = self.generate_svg(score, &layout, title);

        // Generate HTML container
        let score_id = format!("score-{}", uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("0"));

        let html = format!(
            r#"<div class="music-score-container" data-score-id="{}" data-score-type="abc">
  <button class="music-score-download" data-score-id="{}" data-score-type="abc" aria-label="Download music score as SVG">
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
      <polyline points="7 10 12 15 17 10"/>
      <line x1="12" y1="15" x2="12" y2="3"/>
    </svg>
  </button>
  <svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg" class="music-score-svg">
    {}
  </svg>
</div>"#,
            score_id, score_id, layout.width, layout.height, svg
        );

        Ok(html)
    }

    /// Calculate layout for the score
    fn calculate_layout(score: &AbcScore, title: Option<&str>) -> MusicScoreLayout {
        let mut layout = MusicScoreLayout {
            width: 800.0,
            height: 400.0,
            staff_y: 150.0,
            title_offset: 0.0,
        };

        // Adjust height for title
        if title.is_some() || score.title.is_some() {
            layout.title_offset = 40.0;
            layout.height += 40.0;
        }

        // Adjust height for multi-voice
        if score.voices.len() > 1 {
            layout.height += (score.voices.len() - 1) as f64 * 100.0;
        }

        // Adjust width based on note count (rough estimate)
        let note_count = score.notes.split_whitespace().count();
        let estimated_width = 800.0 + (note_count as f64 * 20.0);
        layout.width = estimated_width.min(1200.0);

        layout
    }

    /// Generate SVG for the score
    fn generate_svg(&self, score: &AbcScore, layout: &MusicScoreLayout, title: Option<&str>) -> String {
        let mut svg = String::new();

        // Background
        svg.push_str(&format!(
            r#"<rect x="0" y="0" width="{}" height="{}" fill="white"/>"#,
            layout.width, layout.height
        ));

        // Title
        let display_title = title.or(score.title.as_deref()).unwrap_or("");
        if !display_title.is_empty() {
            svg.push_str(&format!(
                r#"<text x="{}" y="30" text-anchor="middle" font-size="20" font-weight="bold" font-family="sans-serif">{}</text>"#,
                layout.width / 2.0,
                Self::escape_xml(display_title)
            ));
        }

        // Meter/Time signature
        if let Some(ref meter) = score.meter {
            svg.push_str(&format!(
                r#"<text x="80" y="{}" text-anchor="middle" font-size="14" font-family="serif">{}</text>"#,
                layout.staff_y + 20.0,
                Self::escape_xml(meter)
            ));
        }

        // Key signature
        if let Some(ref key) = score.key {
            svg.push_str(&format!(
                r#"<text x="120" y="{}" text-anchor="middle" font-size="14" font-family="serif">{}</text>"#,
                layout.staff_y + 20.0,
                Self::escape_xml(key)
            ));
        }

        // Draw staff lines
        let staff_width = layout.width - 100.0;
        let base_y = layout.staff_y + layout.title_offset;

        // Draw main staff
        svg.push_str(&Self::draw_staff(50.0, base_y, staff_width));

        // Draw treble clef (simplified as text)
        svg.push_str(&format!(
            r#"<text x="55" y="{}" font-size="36" font-family="serif">𝄞</text>"#,
            base_y + 28.0
        ));

        // Draw notes (simplified representation)
        if !score.notes.is_empty() {
            svg.push_str(&Self::draw_notes(&score.notes, 140.0, base_y, staff_width));
        }

        // Draw voices if present
        for (i, voice) in score.voices.iter().enumerate() {
            let voice_y = base_y + (i as f64 + 1.0) * 100.0;
            svg.push_str(&Self::draw_staff(50.0, voice_y, staff_width));
            
            // Voice label
            svg.push_str(&format!(
                r#"<text x="55" y="{}" font-size="12" font-weight="bold" font-family="sans-serif">{}</text>"#,
                base_y + 10.0,
                Self::escape_xml(&voice.name)
            ));

            if !voice.notes.is_empty() {
                svg.push_str(&Self::draw_notes(&voice.notes, 140.0, voice_y, staff_width));
            }
        }

        svg
    }

    /// Draw staff lines
    fn draw_staff(x: f64, y: f64, width: f64) -> String {
        let mut svg = String::new();
        let line_spacing = 10.0;

        for i in 0..5 {
            let line_y = y + (i as f64 * line_spacing);
            svg.push_str(&format!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="1"/>"#,
                x, line_y, x + width, line_y
            ));
        }

        svg
    }

    /// Draw notes (simplified representation)
    fn draw_notes(notes: &str, start_x: f64, staff_y: f64, max_width: f64) -> String {
        let mut svg = String::new();
        let mut x = start_x;
        let base_note_y = staff_y + 20.0; // Middle of staff

        for token in notes.split_whitespace() {
            // Skip bar lines
            if token == "|" || token == "||" || token == "|:" || token == ":|" {
                svg.push_str(&format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2"/>"#,
                    x, staff_y, x, staff_y + 40.0
                ));
                x += 15.0;
                continue;
            }

            // Draw simplified note
            let note_y = Self::calculate_note_y(token, base_note_y);
            
            svg.push_str(&format!(
                r#"<ellipse cx="{}" cy="{}" rx="6" ry="5" fill="black" transform="rotate(-20 {} {})"/>"#,
                x, note_y, x, note_y
            ));
            
            // Stem
            svg.push_str(&format!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="1.5"/>"#,
                x + 5.0, note_y, x + 5.0, note_y - 25.0
            ));

            x += 25.0;

            // Wrap to next line if needed
            if x > max_width {
                x = start_x;
                // Would need to implement multi-line rendering for full support
            }
        }

        svg
    }

    /// Calculate Y position for a note based on its pitch (simplified)
    fn calculate_note_y(note_token: &str, base_y: f64) -> f64 {
        // Extract note name (first character)
        let note_char = note_token.chars().next().unwrap_or('C');
        
        // Simple mapping: higher letters = higher pitch
        match note_char {
            'A' | 'a' => base_y - 15.0,
            'B' | 'b' => base_y - 10.0,
            'C' | 'c' => base_y,
            'D' | 'd' => base_y + 5.0,
            'E' | 'e' => base_y + 10.0,
            'F' | 'f' => base_y + 15.0,
            'G' | 'g' => base_y + 20.0,
            _ => base_y,
        }
    }

    /// Escape XML special characters
    fn escape_xml(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}

impl Default for AbcRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create AbcRenderer")
    }
}