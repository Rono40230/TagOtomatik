use crate::db::Database;
use crate::models::{Album, AppError};
use crate::services::{CleanerService, ExceptionService, MetadataProcessorService, RenamerService};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tauri::State;

#[tauri::command]
pub async fn preview_auto_correct(
    db: State<'_, Database>,
    mut album: Album,
) -> Result<Album, AppError> {
    let processor = MetadataProcessorService::new();

    // Fetch exceptions
    let exceptions_list = ExceptionService::get_all(&db)?;
    let mut exceptions_map = HashMap::new();
    for ex in exceptions_list {
        exceptions_map.insert((ex.category, ex.original.to_lowercase()), ex.corrected);
    }

    // 1. Correct Metadata Only
    for track in &mut album.tracks {
        processor.nettoyer_track(track, &exceptions_map);
    }

    Ok(album)
}

#[tauri::command]
pub async fn apply_auto_correct(mut album: Album) -> Result<Album, AppError> {
    let processor = MetadataProcessorService::new();
    let renamer = RenamerService::new();
    let cleaner = CleanerService::new();

    // 2. Rename Folder Logic
    // Calculate Year Range (Min-Max) or Single Year
    let years: Vec<u32> = album.tracks.iter().filter_map(|t| t.year).collect();
    let folder_year = if years.is_empty() {
        None
    } else {
        let min = years.iter().min().unwrap();
        let max = years.iter().max().unwrap();
        if min == max {
            Some(*min)
        } else {
            Some(*min)
        }
    };

    // Use the first track to determine Album Artist (heuristic)
    if let Some(first_track) = album.tracks.first() {
        let artist = &first_track.artist;
        let title = &first_track.album;

        // If range, we might want to format manually, but let's use processor for consistency
        let new_folder_name = renamer.format_folder_name(artist, title, folder_year);

        let current_path_buf = Path::new(&album.path).to_path_buf();
        let current_path = current_path_buf.as_path();

        if let Some(parent) = current_path.parent() {
            let new_path = parent.join(&new_folder_name);

            // Only rename if different and target doesn't exist
            if new_path != current_path && !new_path.exists() {
                match fs::rename(current_path, &new_path) {
                    Ok(_) => {
                        // Update Album path
                        let new_path_str = new_path.to_string_lossy().to_string();
                        let old_path_str = album.path.clone();

                        album.path = new_path_str.clone();

                        // Update all tracks paths (assuming they are inside the folder)
                        for track in &mut album.tracks {
                            if track.path.starts_with(&old_path_str) {
                                track.path = track.path.replace(&old_path_str, &new_path_str);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to rename folder: {}", e);
                    }
                }
            }
        }
    }

    // 3. Flatten & Rename Files
    let album_path = Path::new(&album.path);
    cleaner.rename_track_files(&mut album.tracks, album_path, &renamer);

    // 4. Handle Cover Image (Recursive Search)
    cleaner.handle_cover_image(album_path);

    // 5. Clean Directory (Delete junk, empty folders)
    cleaner.clean_directory(album_path);

    Ok(album)
}
