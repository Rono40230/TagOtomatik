use crate::models::{Album, AppError, ScanResult};
use crate::services::{AudioService, ValidatorService};
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

    pub fn scanner_dossier(&self, chemin_racine: &str) -> Result<ScanResult, AppError> {
        let audio_service = AudioService::new();
        let mut albums_map: HashMap<String, Album> = HashMap::new();
        let mut errors: Vec<String> = Vec::new();
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
                                let err_msg =
                                    format!("Erreur lecture fichier {}: {:?}", path.display(), e);
                                eprintln!("{}", err_msg);
                                errors.push(err_msg);
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
            // Calculer les années min et max
            let years: Vec<u32> = album
                .tracks
                .iter()
                .filter_map(|t| t.year)
                .filter(|&y| y > 0)
                .collect();

            if !years.is_empty() {
                album.year_min = years.iter().min().copied();
                album.year_max = years.iter().max().copied();
            }

            ValidatorService::evaluate_album_status(album);

            // Trier les pistes par numéro
            album.tracks.sort_by(|a, b| {
                a.track_number
                    .unwrap_or(0)
                    .cmp(&b.track_number.unwrap_or(0))
            });
        }

        Ok(ScanResult { albums, errors })
    }
}
