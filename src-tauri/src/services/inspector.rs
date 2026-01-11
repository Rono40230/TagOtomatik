use crate::models::AppError;
use std::path::Path;

pub struct InspectorService;

impl InspectorService {
    pub fn detecter_fichiers_inutiles(chemin_dossier: &str, album_info: Option<(&str, &str, &str)>) -> Result<Vec<String>, AppError> {
        let mut junk_files = Vec::new();
        let audio_ext = ["mp3", "flac", "ogg", "m4a", "wav"];
        let playlist_ext = ["m3u", "m3u8"];

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


                // Autoriser la playlist si elle respecte le format "Artist - (Year) Album"
                let is_valid_playlist = if let Some((artist, year, title)) = album_info {
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        if playlist_ext.contains(&ext_str.as_str()) {
                             let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                             let expected = format!("{} - ({}) {}", artist.trim(), year, title.trim());
                             
                             // Normalisation robuste : espaces + quotes
                             let normalize = |s: &str| {
                                 s.replace('’', "'")
                                  .replace('`', "'")
                                  .split_whitespace()
                                  .collect::<Vec<_>>()
                                  .join(" ")
                             };
                             
                             let stem_norm = normalize(&stem);
                             let expected_norm = normalize(&expected);
                             let title_norm = normalize(title); 
                             let expected_simple = normalize(&format!("{} - {}", artist.trim(), title.trim()));

                             // On accepte :
                             // 1. Format strict : Artist - (Year) Title
                             // 2. Format simple : Artist - Title
                             // 3. Titre seul : Title
                             stem_norm.eq_ignore_ascii_case(&expected_norm) || 
                             stem_norm.eq_ignore_ascii_case(&expected_simple) ||
                             stem_norm.eq_ignore_ascii_case(&title_norm)
                        } else {
                             false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                if !is_audio && !is_cover && !is_valid_playlist {
                     if let Some(ext) = path.extension() {
                          let ext_str = ext.to_string_lossy().to_lowercase();
                          if playlist_ext.contains(&ext_str.as_str()) {
                               // Si on est ici, c'est que is_valid_playlist a retourné FALSE
                               // On n'ajoute RIEN aux junk_files si c'est une playlist qui échoue "juste" la vérification de nom
                               // C'est un choix UX : une playlist mal nommée n'est pas "inutile" (junk), elle est juste mal nommée.
                               // Le système de "Junk detection" vise les fichiers .txt, .nfo, images webp, etc.
                               // Pour éviter de frustrer l'utilisateur avec des faux positifs, on ignore silencieusement les playlists.
                               continue; 
                          }
                     }

                    if file_name.to_lowercase() == "cover.jpg" {
                        junk_files.push(format!("{} (sera renommé en cover.jpg)", file_name));
                    } else {
                        junk_files.push(file_name.to_string());
                    }
                }
            }
        }
        Ok(junk_files)
    }

    pub fn find_cover_image(path: &Path) -> Option<String> {
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

    pub fn has_playlist(path: &Path) -> bool {
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
