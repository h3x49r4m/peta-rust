//! Music score data models

use serde::{Deserialize, Serialize};

/// Music score type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MusicScoreType {
    /// ABC notation
    Abc,
}

/// Music score representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MusicScore {
    /// ABC notation score
    Abc(AbcScore),
}

/// ABC notation score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbcScore {
    /// Score index
    pub index: Option<String>,
    /// Title
    pub title: Option<String>,
    /// Composer
    pub composer: Option<String>,
    /// Meter (time signature)
    pub meter: Option<String>,
    /// Default note length
    pub default_length: Option<String>,
    /// Key signature
    pub key: Option<String>,
    /// Notes and music content
    pub notes: String,
    /// Voices (multi-voice support)
    pub voices: Vec<AbcVoice>,
}

/// ABC voice (for multi-voice scores)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbcVoice {
    /// Voice name or identifier
    pub name: String,
    /// Clef (treble, bass, etc.)
    pub clef: Option<String>,
    /// Voice notes
    pub notes: String,
}

/// Layout for music score rendering
#[derive(Debug, Clone)]
pub struct MusicScoreLayout {
    /// Canvas width
    pub width: f64,
    /// Canvas height
    pub height: f64,
    /// Staff positions
    pub staff_y: f64,
    /// Title offset (if title present)
    pub title_offset: f64,
}