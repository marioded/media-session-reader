use super::Cover;
use serde::Serialize;

pub use crate::RepeatMode;

#[derive(Debug, Clone, Serialize)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub cover: Option<Cover>,

    pub duration_ms: u64,
    pub position_ms: u64,

    pub playing: bool,
    pub playback_rate: Option<f64>,

    pub shuffle: Option<bool>,
    pub repeat: Option<RepeatMode>,
    pub can_next: bool,
    pub can_previous: bool,

    pub track_number: Option<u32>,
    pub genre: Option<String>,
}
