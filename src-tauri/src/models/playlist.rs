use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Playlist {
    pub name: String,
    pub path: String,
    pub track_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlaylistTrack {
    pub path: String,
    pub title: String,
    pub artist: String,
    pub duration: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlaylistOptions {
    pub album_path: String,
    pub tracks: Vec<super::Track>,
    pub filename: String,
    pub format: String,
    pub path_type: String,
    pub use_extended_info: bool,
    pub path_separator: String,
}
