//! ABC notation parser

use crate::core::{Error, Result};
use crate::content::rst::music_scores::models::*;
use regex::Regex;

/// ABC notation parser
pub struct MusicScoreParser;

impl MusicScoreParser {
    /// Create a new ABC parser
    pub fn new() -> Self {
        Self
    }

    /// Parse music score from text
    pub fn parse(&self, score_type: &str, content: &str) -> Result<MusicScore> {
        match score_type.to_lowercase().as_str() {
            "abc" => self.parse_abc(content),
            _ => Err(Error::content(format!("Unsupported music score type: {}", score_type))),
        }
    }

    /// Parse ABC notation
    fn parse_abc(&self, content: &str) -> Result<MusicScore> {
        let mut score = AbcScore {
            index: None,
            title: None,
            composer: None,
            meter: None,
            default_length: None,
            key: None,
            notes: String::new(),
            voices: Vec::new(),
        };

        let mut current_voice: Option<AbcVoice> = None;
        let voice_regex = Regex::new(r"^V:\s*(.+)$").unwrap();
        let field_regex = Regex::new(r"^([A-Z]):\s*(.+)$").unwrap();

        for line in content.lines() {
            let line = line.trim();
            
            // Skip empty lines
            if line.is_empty() {
                continue;
            }

            // Check for voice marker
            if let Some(caps) = voice_regex.captures(line) {
                // Save previous voice if exists
                if let Some(voice) = current_voice.take() {
                    score.voices.push(voice);
                }
                
                // Start new voice
                let voice_name = caps.get(1).unwrap().as_str().to_string();
                current_voice = Some(AbcVoice {
                    name: voice_name.clone(),
                    clef: None,
                    notes: String::new(),
                });
                continue;
            }

            // Check for field markers
            if let Some(caps) = field_regex.captures(line) {
                let field = caps.get(1).unwrap().as_str();
                let value = caps.get(2).unwrap().as_str();

                match field {
                    "X" => score.index = Some(value.to_string()),
                    "T" => score.title = Some(value.to_string()),
                    "C" => score.composer = Some(value.to_string()),
                    "M" => score.meter = Some(value.to_string()),
                    "L" => score.default_length = Some(value.to_string()),
                    "K" => score.key = Some(value.to_string()),
                    _ => {}
                }
                continue;
            }

            // If we're in a voice, add notes to voice
            if let Some(ref mut voice) = current_voice {
                if !voice.notes.is_empty() {
                    voice.notes.push(' ');
                }
                voice.notes.push_str(line);
            } else {
                // Add to main notes
                if !score.notes.is_empty() {
                    score.notes.push(' ');
                }
                score.notes.push_str(line);
            }
        }

        // Save last voice if exists
        if let Some(voice) = current_voice {
            score.voices.push(voice);
        }

        Ok(MusicScore::Abc(score))
    }
}

impl Default for MusicScoreParser {
    fn default() -> Self {
        Self::new()
    }
}