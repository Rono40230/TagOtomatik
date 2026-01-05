use crate::models::Track;
use crate::services::RenamerService;
use std::fs;
use std::path::Path;

pub struct CleanerService;

impl Default for CleanerService {
    fn default() -> Self {
        Self::new()
    }
}

impl CleanerService {
    pub fn new() -> Self {
        Self
    }

    /// Renomme les fichiers physiques des pistes
    pub fn rename_track_files(
        &self,
        tracks: &mut [Track],
        album_path: &Path,
        _renamer: &RenamerService, // Unused now
    ) {
        for track in tracks {
            let current_path = Path::new(&track.path);
            if !current_path.exists() {
                continue;
            }

            // Use the filename from the track struct (which might have been cleaned/edited)
            // instead of regenerating it from tags.
            let new_filename = track.filename.clone();

            let target_path = album_path.join(&new_filename);

            // Avoid overwriting if source == target
            if target_path != current_path {
                // Create parent dir if missing (should be album_path)
                if let Some(parent) = target_path.parent() {
                    if !parent.exists() {
                        let _ = fs::create_dir_all(parent);
                    }
                }

                match fs::rename(current_path, &target_path) {
                    Ok(_) => {
                        track.path = target_path.to_string_lossy().to_string();
                        track.filename = new_filename;
                    }
                    Err(e) => eprintln!("Error renaming file: {}", e),
                }
            }
        }
    }

    /// Gère l'image de couverture (recherche récursive et renommage)
    pub fn handle_cover_image(&self, album_path: &Path) {
        let mut images = Vec::new();
        // Simple BFS/DFS to find all images
        let mut stack = vec![album_path.to_path_buf()];
        while let Some(dir) = stack.pop() {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        stack.push(path);
                    } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        let ext_lower = ext.to_lowercase();
                        if ["jpg", "jpeg", "bmp", "gif"].contains(&ext_lower.as_str()) {
                            images.push(path);
                        }
                    }
                }
            }
        }

        // Priority: cover.jpg > front.jpg > largest file
        let target_cover = album_path.join("cover.jpg");

        // Rename known patterns to cover.jpg
        let rename_patterns = ["front", "folder", "album", "albumart", "artwork"];

        for image_path in &images {
            if let Some(stem) = image_path.file_stem().and_then(|s| s.to_str()) {
                let stem_lower = stem.to_lowercase();
                if rename_patterns.contains(&stem_lower.as_str()) {
                    // If cover.jpg doesn't exist, rename this one to cover.jpg
                    if !target_cover.exists() {
                        let _ = fs::rename(image_path, &target_cover);
                        return; // Done
                    } else {
                        // If cover.jpg exists, delete this duplicate/variant
                        let _ = fs::remove_file(image_path);
                    }
                }
                // Also handle albumart_... patterns if needed, but let's stick to exact matches first
            }
        }

        if !target_cover.exists() && !images.is_empty() {
            let best_image = images
                .iter()
                .find(|p| {
                    let name = p
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_lowercase();
                    name == "cover.jpg"
                })
                .or_else(|| {
                    images.iter().find(|p| {
                        let name = p
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                            .to_lowercase();
                        name == "front.jpg"
                    })
                })
                .or_else(|| {
                    images
                        .iter()
                        .max_by_key(|p| p.metadata().map(|m| m.len()).unwrap_or(0))
                });

            if let Some(src) = best_image {
                if src.exists() {
                    // Check existence as it might have been renamed/deleted above
                    let _ = fs::rename(src, &target_cover);
                }
            }
        }
    }

    /// Nettoie le dossier (fichiers indésirables et dossiers vides)
    pub fn clean_directory(&self, album_path: &Path) {
        let audio_ext = ["mp3", "flac", "ogg", "m4a", "wav"];

        // Pass 1: Delete junk files recursively (Strict Whitelist Logic)
        let mut dirs_to_check = vec![album_path.to_path_buf()];
        while let Some(dir) = dirs_to_check.pop() {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        dirs_to_check.push(path);
                    } else {
                        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                        let ext = path
                            .extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_lowercase();

                        // Règle stricte : On garde uniquement les fichiers audio et "cover.jpg"
                        let is_audio = audio_ext.contains(&ext.as_str());
                        let is_cover = name == "cover.jpg"; // Strict case check

                        if !is_audio && !is_cover {
                            // C'est un fichier inutile, on supprime
                            let _ = fs::remove_file(&path);
                        }
                    }
                }
            }
        }

        // Pass 2: Delete empty directories
        for _ in 0..3 {
            let mut subdirs = Vec::new();
            let mut stack = vec![album_path.to_path_buf()];
            while let Some(dir) = stack.pop() {
                if let Ok(entries) = fs::read_dir(&dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            subdirs.push(path.clone());
                            stack.push(path);
                        }
                    }
                }
            }
            subdirs.sort_by_key(|p| std::cmp::Reverse(p.as_os_str().len()));
            for dir in subdirs {
                if dir != album_path {
                    let _ = fs::remove_dir(dir);
                }
            }
        }
    }
}
