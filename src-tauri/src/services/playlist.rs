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

    pub fn write_playlist(
        &self,
        options: &crate::models::playlist::PlaylistOptions,
    ) -> Result<String, String> {
        let extension = options.format.to_lowercase();
        let file_name = if options
            .filename
            .to_lowercase()
            .ends_with(&format!(".{}", extension))
        {
            options.filename.clone()
        } else {
            format!("{}.{}", options.filename, extension)
        };

        let output_path = Path::new(&options.album_path).join(&file_name);
        let mut file = File::create(&output_path).map_err(|e| e.to_string())?;

        // Header
        match extension.as_str() {
            "m3u" | "m3u8" => {
                if options.use_extended_info {
                    writeln!(file, "#EXTM3U").map_err(|e| e.to_string())?;
                }
            }
            "pls" => {
                writeln!(file, "[playlist]").map_err(|e| e.to_string())?;
                writeln!(file, "NumberOfEntries={}", options.tracks.len())
                    .map_err(|e| e.to_string())?;
            }
            _ => {}
        }

        for (i, track) in options.tracks.iter().enumerate() {
            // 1. Resolve Path
            let track_path_str = if options.path_type == "Relative" {
                pathdiff::diff_paths(&track.path, &options.album_path)
                    .ok_or("Could not create relative path")?
                    .to_string_lossy()
                    .to_string()
            } else {
                track.path.clone()
            };

            // 2. Apply Separator
            let final_path = match options.path_separator.as_str() {
                "Slash" => track_path_str.replace('\\', "/"),
                "Backslash" => track_path_str.replace('/', "\\"),
                _ => track_path_str,
            };

            // 3. Write Entry
            match extension.as_str() {
                "m3u" | "m3u8" => {
                    if options.use_extended_info {
                        writeln!(
                            file,
                            "#EXTINF:{},{} - {}",
                            track.duration_sec, track.artist, track.title
                        )
                        .map_err(|e| e.to_string())?;
                    }
                    writeln!(file, "{}", final_path).map_err(|e| e.to_string())?;
                }
                "pls" => {
                    let idx = i + 1;
                    writeln!(file, "File{}={}", idx, final_path).map_err(|e| e.to_string())?;
                    if options.use_extended_info {
                        writeln!(file, "Title{}={} - {}", idx, track.artist, track.title)
                            .map_err(|e| e.to_string())?;
                        writeln!(file, "Length{}={}", idx, track.duration_sec)
                            .map_err(|e| e.to_string())?;
                    }
                }
                _ => {
                    writeln!(file, "{}", final_path).map_err(|e| e.to_string())?;
                }
            }
        }

        if extension == "pls" {
            writeln!(file, "Version=2").map_err(|e| e.to_string())?;
        }

        Ok(output_path.to_string_lossy().to_string())
    }
}
