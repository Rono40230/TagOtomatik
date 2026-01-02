use crate::models::{Album, AlbumStatus, AppError};
use crate::services::AudioService;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

pub struct ScannerService;

impl Default for ScannerService {
    fn default() -> Self {
        Self::new()
    }
}

impl ScannerService {
    pub fn new() -> Self {
        Self
    }

    pub fn scanner_dossier(&self, chemin_racine: &str) -> Result<Vec<Album>, AppError> {
        let audio_service = AudioService::new();
        let mut albums_map: HashMap<String, Album> = HashMap::new();
        let extensions_supportees = ["mp3", "flac", "ogg", "m4a", "wav"];

        for entry in WalkDir::new(chemin_racine)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if extensions_supportees.contains(&ext_str.as_str()) {
                        // C'est un fichier audio supporté
                        match audio_service.lire_metadonnees(path.to_str().unwrap_or_default()) {
                            Ok(track) => {
                                let parent_dir = path.parent().unwrap_or(Path::new(""));
                                let parent_path = parent_dir.to_string_lossy().to_string();

                                let album =
                                    albums_map.entry(parent_path.clone()).or_insert_with(|| {
                                        // Créer un nouvel album si n'existe pas encore
                                        // On utilise le nom du dossier comme titre par défaut si métadonnées manquantes
                                        let folder_name = parent_dir
                                            .file_name()
                                            .unwrap_or_default()
                                            .to_string_lossy()
                                            .to_string();

                                        Album::new(
                                            parent_path,
                                            folder_name,
                                            "Artiste Inconnu".to_string(),
                                        )
                                    });

                                // Mettre à jour les infos de l'album avec les infos de la première piste (ou la plus fréquente - simplifions pour l'instant)
                                if album.tracks.is_empty() {
                                    // NOTE: On garde le nom du dossier comme titre de l'album (album.title)
                                    // au lieu d'écraser avec le tag album de la piste.
                                    // if !track.album.is_empty() {
                                    //    album.title = track.album.clone();
                                    // }
                                    if !track.artist.is_empty() {
                                        album.artist = track.artist.clone();
                                    }
                                    album.year = track.year;
                                }

                                album.tracks.push(track);
                            }
                            Err(e) => {
                                eprintln!("Erreur lecture fichier {}: {:?}", path.display(), e);
                                // On continue même si un fichier échoue
                            }
                        }
                    }
                }
            }
        }

        // Convertir la map en vecteur
        let mut albums: Vec<Album> = albums_map.into_values().collect();

        // Mettre à jour le statut des albums
        for album in &mut albums {
            Self::evaluate_album_status(album);

            // Trier les pistes par numéro
            album.tracks.sort_by(|a, b| {
                a.track_number
                    .unwrap_or(0)
                    .cmp(&b.track_number.unwrap_or(0))
            });
        }

        Ok(albums)
    }

    pub fn evaluate_album_status(album: &mut Album) {
        // 1. Check if we already have a valid cover path
        let mut cover_valid = false;
        if let Some(path) = &album.cover_path {
            if Path::new(path).exists() {
                cover_valid = true;
            }
        }

        if !cover_valid {
            // Chercher la cover
            album.cover_path = Self::find_cover_image(Path::new(&album.path));
        }

        // 2. Check for playlist
        album.has_playlist = Self::has_playlist(Path::new(&album.path));

        if album.tracks.is_empty() {
            album.status = AlbumStatus::Incomplete;
            return;
        }

        // Vérification de la qualité des tags (Dirty detection)
        let mut is_dirty = false;

        // 1. Cover manquante
        if album.cover_path.is_none() {
            is_dirty = true;
        }

        // 2. Playlist manquante
        if !album.has_playlist {
            is_dirty = true;
        }

        // 3. Année manquante ou incohérente (0)
        if album.year.is_none() || album.year == Some(0) {
            // Vérifier si les pistes ont une année
            let has_missing_year = album
                .tracks
                .iter()
                .any(|t| t.year.is_none() || t.year == Some(0));
            if has_missing_year {
                is_dirty = true;
            }
        }

        // 3. Tags essentiels manquants sur les pistes
        let has_missing_tags = album.tracks.iter().any(|t| {
            t.title.trim().is_empty()
                || t.artist.trim().is_empty()
                || t.album.trim().is_empty()
                || t.genre.as_deref().unwrap_or("").trim().is_empty()
        });
        if has_missing_tags {
            is_dirty = true;
        }

        if is_dirty {
            album.status = AlbumStatus::Dirty;
        } else {
            album.status = AlbumStatus::Clean;
        }
    }

    pub fn detecter_fichiers_inutiles(
        &self,
        chemin_dossier: &str,
    ) -> Result<Vec<String>, AppError> {
        let mut junk_files = Vec::new();
        let audio_ext = ["mp3", "flac", "ogg", "m4a", "wav"];

        let path = Path::new(chemin_dossier);
        if !path.exists() || !path.is_dir() {
            return Err(AppError::Validation("Dossier invalide".to_string()));
        }

        for entry in std::fs::read_dir(path).map_err(|e| AppError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| AppError::Io(e.to_string()))?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                let lower_name = file_name.to_lowercase();

                // Règle stricte : On garde uniquement les fichiers audio et "cover.jpg" (strictement minuscule)
                let is_audio = if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    audio_ext.contains(&ext_str.as_str())
                } else {
                    false
                };

                // Seul "cover.jpg" est autorisé. "Cover.jpg", "COVER.jpg" etc. sont considérés comme inutiles.
                let is_cover = file_name == "cover.jpg";

                if !is_audio && !is_cover {
                    junk_files.push(file_name.to_string());
                }
            }
        }
        Ok(junk_files)
    }

    fn find_cover_image(path: &Path) -> Option<String> {
        let extensions = ["jpg", "jpeg", "png", "bmp", "webp"];
        let common_names = ["cover", "folder", "front", "album"];

        // 1. Check for common names first
        for name in common_names {
            for ext in extensions {
                let p = path.join(format!("{}.{}", name, ext));
                if p.exists() {
                    return Some(p.to_string_lossy().to_string());
                }
            }
        }

        // 2. If not found, scan directory for any image
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() {
                    if let Some(ext) = p.extension() {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        if extensions.contains(&ext_str.as_str()) {
                            return Some(p.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        None
    }

    fn has_playlist(path: &Path) -> bool {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() {
                    if let Some(ext) = p.extension() {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        if ["m3u", "m3u8", "pls"].contains(&ext_str.as_str()) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}
