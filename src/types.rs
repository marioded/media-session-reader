#[derive(Debug, Clone)]
pub struct Track {
    pub title: String,
    pub artist: String,

    pub duration_ms: u64,
    pub position_ms: u64,

    pub playing: bool,
}