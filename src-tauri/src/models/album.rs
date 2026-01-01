use super::track::Track;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AlbumStatus {
    Clean,
    Dirty,
    Processing,
    Incomplete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Album {
    pub id: String,   // Hash unique ou chemin dossier
    pub path: String, // Chemin dossier
    pub title: String,
    pub artist: String, // Artiste principal (Album Artist)
    pub year: Option<u32>,
    pub cover_path: Option<String>, // Chemin vers cover.jpg locale ou cache
    pub has_playlist: bool,
    pub tracks: Vec<Track>,
    pub status: AlbumStatus,
}

impl Album {
    pub fn new(path: String, title: String, artist: String) -> Self {
        Self {
            id: format!("{:x}", md5::compute(&path)), // Simple hash for ID based on path
            path,
            title,
            artist,
            year: None,
            cover_path: None,
            has_playlist: false,
            tracks: Vec::new(),
            status: AlbumStatus::Processing,
        }
    }
}
