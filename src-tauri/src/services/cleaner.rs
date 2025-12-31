use crate::models::Track;
use crate::services::MetadataProcessorService;
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
        processor: &MetadataProcessorService,
    ) {
        for track in tracks {
            let current_path = Path::new(&track.path);
            if !current_path.exists() {
                continue;
            }

            let extension = current_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("mp3");

            let new_filename =
                processor.format_track_filename(track.track_number, &track.title, extension);

            let target_path = album_path.join(&new_filename);

            // Avoid overwriting if source == target
            if target_path != current_path {
                // Create parent dir if missing (should be album_path)
                if let Some(parent) = target_path.parent() {
                    let _ = fs::create_dir_all(parent);
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
                let _ = fs::rename(src, &target_cover);
            }
        }
    }

    /// Nettoie le dossier (fichiers indésirables et dossiers vides)
    pub fn clean_directory(&self, album_path: &Path) {
        let junk_extensions = [
            "nfo",
            "m3u",
            "txt",
            "url",
            "sfv",
            "ini",
            "db",
            "ds_store",
            "thumbs.db",
            "png",
            "tmp",
            "temp",
            "bak",
            "log",
            "md5",
            "pdf",
            "doc",
            "docx",
            "gif",
            "bmp",
        ];
        let junk_files = [
            "desktop.ini",
            ".fuse_hidden",
            "._metadata",
            "#recycle",
            "recycle.bin",
        ];

        // Pass 1: Delete junk files recursively
        let mut dirs_to_check = vec![album_path.to_path_buf()];
        while let Some(dir) = dirs_to_check.pop() {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        dirs_to_check.push(path);
                    } else {
                        let name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                            .to_lowercase();
                        let ext = path
                            .extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_lowercase();

                        if junk_files.contains(&name.as_str())
                            || junk_extensions.contains(&ext.as_str())
                        {
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
