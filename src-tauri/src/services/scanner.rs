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
                                    if !track.album.is_empty() {
                                        album.title = track.album.clone();
                                    }
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
            // Chercher la cover
            album.cover_path = Self::find_cover_image(Path::new(&album.path));

            if album.tracks.is_empty() {
                album.status = AlbumStatus::Incomplete;
            } else {
                album.status = AlbumStatus::Clean; // Par défaut, on dira Clean, le processeur dira Dirty plus tard
            }
            // Trier les pistes par numéro
            album.tracks.sort_by(|a, b| {
                a.track_number
                    .unwrap_or(0)
                    .cmp(&b.track_number.unwrap_or(0))
            });
        }

        Ok(albums)
    }

    pub fn detecter_fichiers_inutiles(
        &self,
        chemin_dossier: &str,
    ) -> Result<Vec<String>, AppError> {
        let mut junk_files = Vec::new();
        let whitelist_ext = ["mp3", "flac", "ogg", "m4a", "wav", "jpg", "jpeg", "png"];

        let path = Path::new(chemin_dossier);
        if !path.exists() || !path.is_dir() {
            return Err(AppError::Validation("Dossier invalide".to_string()));
        }

        for entry in std::fs::read_dir(path).map_err(|e| AppError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| AppError::Io(e.to_string()))?;
            let path = entry.path();
            if path.is_file() {
                let is_whitelisted = if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    whitelist_ext.contains(&ext_str.as_str())
                } else {
                    false
                };

                if !is_whitelisted {
                    junk_files.push(
                        path.file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                    );
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
}
