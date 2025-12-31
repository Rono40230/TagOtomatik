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
