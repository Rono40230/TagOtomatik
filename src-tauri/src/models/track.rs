use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Track {
    pub path: String,     // Chemin complet
    pub filename: String, // Nom de fichier
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: Option<u32>,
    pub track_number: Option<u32>,
    pub genre: Option<String>,
    pub duration_sec: u64,
    pub format: String, // "mp3", "flac"...
    pub bit_rate: Option<u32>,
    pub has_cover: bool,

    // État de modification
    pub original_metadata: Option<Box<Track>>, // Pour le diff/undo
    pub is_modified: bool,
}

impl Track {
    pub fn new(path: String, filename: String) -> Self {
        Self {
            path,
            filename,
            title: String::new(),
            artist: String::new(),
            album: String::new(),
            year: None,
            track_number: None,
            genre: None,
            duration_sec: 0,
            format: String::new(),
            bit_rate: None,
            has_cover: false,
            original_metadata: None,
            is_modified: false,
        }
    }
}
