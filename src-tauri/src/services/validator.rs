use crate::models::{Album, AlbumStatus};
use super::InspectorService;
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
            album.cover_path = InspectorService::find_cover_image(Path::new(&album.path));
        }

        if album.cover_path.is_none() {
            album.issues.push("Cover manquante".to_string());
        }
    }

    fn check_playlist(album: &mut Album) {
        album.has_playlist = InspectorService::has_playlist(Path::new(&album.path));
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
        // Fix for compilation year range (e.g. 1971-15) & Lifetime safety
        let year_str = if let (Some(min), Some(max)) = (album.year_min, album.year_max) {
             if min > 0 && max > 0 && min != max {
                 let max_short = max % 100;
                 format!("{}-{:02}", min, max_short)
             } else {
                 album.year.unwrap_or(0).to_string()
             }
        } else {
             album.year.unwrap_or(0).to_string()
        };

        // Préparation du contexte pour la playlist
        // NOTE: On passe toujours les infos même si vides pour permettre le debug dans detecter_fichiers_inutiles
        let context_args = Some((album.artist.as_str(), year_str.as_str(), album.title.as_str()));

        if let Ok(junk) = InspectorService::detecter_fichiers_inutiles(&album.path, context_args) {
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




}
