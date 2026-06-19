use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum RepeatMode {
    None,
    Track,
    Playlist,
}
