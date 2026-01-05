use crate::models::{Album, AlbumStatus, AppError};
use std::path::Path;

pub struct ValidatorService;

impl ValidatorService {
    pub fn evaluate_album_status(album: &mut Album) {
        album.issues.clear();

        Self::check_cover(album);
        Self::check_playlist(album);

        if album.tracks.is_empty() {
            album.status = AlbumStatus::Incomplete;
            album.issues.push("Aucune piste audio trouvée".to_string());
            return;
        }

        Self::check_tags(album);
        Self::check_files(album);

        if !album.issues.is_empty() {
            album.status = AlbumStatus::Dirty;
        } else {
            album.status = AlbumStatus::Clean;
        }
    }

    fn check_cover(album: &mut Album) {
        let mut cover_valid = false;
        if let Some(path) = &album.cover_path {
            if Path::new(path).exists() {
                cover_valid = true;
            }
        }

        if !cover_valid {
            album.cover_path = Self::find_cover_image(Path::new(&album.path));
        }

        if album.cover_path.is_none() {
            album.issues.push("Cover manquante".to_string());
        }
    }

    fn check_playlist(album: &mut Album) {
        album.has_playlist = Self::has_playlist(Path::new(&album.path));
        if !album.has_playlist {
            album.issues.push("Playlist manquante".to_string());
        }
    }

    fn check_tags(album: &mut Album) {
        // Année
        if album.year.is_none() || album.year == Some(0) {
            let has_missing_year = album
                .tracks
                .iter()
                .any(|t| t.year.is_none() || t.year == Some(0));
            if has_missing_year {
                album.issues.push("Année manquante".to_string());
            }
        }

        // Tags essentiels
        let mut missing_titles = 0;
        let mut missing_artists = 0;
        let mut missing_albums = 0;
        let mut missing_genres = 0;

        for t in &album.tracks {
            if t.title.trim().is_empty() {
                missing_titles += 1;
            }
            if t.artist.trim().is_empty() {
                missing_artists += 1;
            }
            if t.album.trim().is_empty() {
                missing_albums += 1;
            }
            if t.genre.as_deref().unwrap_or("").trim().is_empty() {
                missing_genres += 1;
            }
        }

        if missing_titles > 0 {
            album
                .issues
                .push(format!("Titre manquant ({} pistes)", missing_titles));
        }
        if missing_artists > 0 {
            album
                .issues
                .push(format!("Artiste manquant ({} pistes)", missing_artists));
        }
        if missing_albums > 0 {
            album
                .issues
                .push(format!("Album manquant ({} pistes)", missing_albums));
        }
        if missing_genres > 0 {
            album
                .issues
                .push(format!("Genre manquant ({} pistes)", missing_genres));
        }

        // Numéros de piste
        let has_missing_track_number = album
            .tracks
            .iter()
            .any(|t| t.track_number.is_none() || t.track_number == Some(0));
        if has_missing_track_number {
            album.issues.push("Numéros de piste manquants".to_string());
        }
    }

    fn check_files(album: &mut Album) {
        // Fichiers inutiles
        if let Ok(junk) = Self::detecter_fichiers_inutiles(&album.path) {
            if !junk.is_empty() {
                album.issues.push(format!(
                    "Fichiers inutiles détectés ({} fichiers)",
                    junk.len()
                ));
            }
        }

        // Conformité noms et majuscules
        let mut bad_filenames = 0;
        let mut bad_casing = 0;

        for t in &album.tracks {
            let path = Path::new(&t.path);
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if !name_str.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                    bad_filenames += 1;
                }
            }

            if t.title.len() > 3
                && (t.title == t.title.to_uppercase() || t.title == t.title.to_lowercase())
            {
                bad_casing += 1;
            }
        }

        if bad_filenames > 0 {
            album.issues.push(format!(
                "Noms de fichiers non conformes ({} pistes)",
                bad_filenames
            ));
        }
        if bad_casing > 0 {
            album.issues.push(format!(
                "Problèmes de majuscules détectés ({} pistes)",
                bad_casing
            ));
        }
    }

    pub fn detecter_fichiers_inutiles(chemin_dossier: &str) -> Result<Vec<String>, AppError> {
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
