use crate::models::playlist::Playlist;
use crate::models::playlist::PlaylistTrack;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub struct PlaylistService;

impl Default for PlaylistService {
    fn default() -> Self {
        Self::new()
    }
}

impl PlaylistService {
    pub fn new() -> Self {
        Self
    }

    pub fn list_playlists(&self, root_path: &str) -> Result<Vec<Playlist>, String> {
        let mut playlists = Vec::new();
        let path = Path::new(root_path);

        if !path.exists() {
            return Ok(vec![]);
        }

        // Recherche récursive simple ou juste à la racine ?
        // Pour l'instant, cherchons à la racine du dossier scanné
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if ext == "m3u" || ext == "m3u8" {
                    let name = path.file_stem().unwrap().to_string_lossy().to_string();
                    // Compter les lignes non vides et non commentaires
                    let count = if let Ok(file) = File::open(&path) {
                        BufReader::new(file)
                            .lines()
                            .filter_map(Result::ok)
                            .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
                            .count()
                    } else {
                        0
                    };

                    playlists.push(Playlist {
                        name,
                        path: path.to_string_lossy().to_string(),
                        track_count: count,
                    });
                }
            }
        }

        Ok(playlists)
    }

    pub fn get_playlist_tracks(&self, playlist_path: &str) -> Result<Vec<PlaylistTrack>, String> {
        let file = File::open(playlist_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut tracks = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Le chemin peut être relatif ou absolu
            // Pour l'instant on retourne juste le chemin brut, le frontend ou un autre service devra résoudre les métadonnées
            // Idéalement on croiserait avec la DB ou on scannerait le fichier
            tracks.push(PlaylistTrack {
                path: line.to_string(),
                title: Path::new(line)
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                artist: "Inconnu".to_string(), // TODO: Lire les tags réels
                duration: 0,
            });
        }

        Ok(tracks)
    }

    pub fn create_playlist(&self, path: &str, name: &str) -> Result<String, String> {
        let file_path = Path::new(path).join(format!("{}.m3u8", name));
        let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
        writeln!(file, "#EXTM3U").map_err(|e| e.to_string())?;
        Ok(file_path.to_string_lossy().to_string())
    }

    pub fn add_track_to_playlist(
        &self,
        playlist_path: &str,
        track_path: &str,
    ) -> Result<(), String> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(playlist_path)
            .map_err(|e| e.to_string())?;

        writeln!(file, "{}", track_path).map_err(|e| e.to_string())?;
        Ok(())
    }
}
