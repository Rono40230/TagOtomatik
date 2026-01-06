use crate::models::{Album, AppError};
use crate::services::{AudioService, IOService, RenamerService, ValidatorService};
use regex::Regex;
use std::path::PathBuf;

fn sanitize_filename(name: &str) -> String {
    name.replace("/", "_")
        .replace("\\", "_")
        .replace(":", "_")
        .replace("*", "_")
        .replace("?", "_")
        .replace("\"", "_")
        .replace("<", "_")
        .replace(">", "_")
        .replace("|", "_")
}

#[tauri::command]
pub async fn save_album_changes(mut album: Album) -> Result<Album, AppError> {
    let audio_service = AudioService::new();

    // 1. Save tags FIRST (before renaming, so path is still valid)
    for track in &mut album.tracks {
        if track.is_modified {
            // Verify file exists before writing
            if !std::path::Path::new(&track.path).exists() {
                return Err(AppError::Io(format!(
                    "Fichier introuvable avant écriture tags: {}",
                    track.path
                )));
            }
            audio_service.ecrire_metadonnees(track)?;
            track.is_modified = false;
            track.original_metadata = Some(Box::new(track.clone()));
        }
    }

    // 2. Rename files
    for track in &mut album.tracks {
        let old_path = PathBuf::from(&track.path);
        if !old_path.exists() {
            // Skip if file doesn't exist (maybe already renamed or deleted externally)
            continue;
        }

        let parent = old_path
            .parent()
            .ok_or_else(|| AppError::Io("Invalid path".into()))?;

        // Use the filename provided by the frontend (user edits)
        // We assume the user knows what they are doing regarding extensions
        let new_filename = sanitize_filename(&track.filename);

        let new_path = parent.join(&new_filename);

        // Check if path has changed (ignoring case if needed, but here strict equality)
        if new_path != old_path {
            IOService::rename_file(old_path.to_str().unwrap(), new_path.to_str().unwrap())?;
            track.path = new_path.to_string_lossy().to_string();
        }
    }

    // 3. Rename folder (Album level)
    if let Some(first_track) = album.tracks.first() {
        let track_path = PathBuf::from(&first_track.path);
        let current_folder = track_path.parent().unwrap();

        // Check if we have a parent folder to rename within
        if let Some(parent_of_folder) = current_folder.parent() {
            let renamer = RenamerService::new();

            // Calculate Year Range if not present in album (it should be, but let's be safe)
            // or trust the frontend/scanner values if they are populated.
            // If the user modified 'year' to a single value in the UI, we might respect that.
            // BUT: The UI logic (AlbiumSidebar) sends back 'year' if modified singly.
            // We should check if year_min/max are consistent with 'year' or if valid.

            // Logic:
            // 1. If 'year' matches 'year_min' or 'year_max' OR if min/max are None, try to recompute from tracks?
            // Actually, we should probably recompute from the tracks present in the album being saved,
            // because the user might have changed dates on individual tracks.

            let years: Vec<u32> = album
                .tracks
                .iter()
                .filter_map(|t| t.year)
                .filter(|&y| y > 0)
                .collect();

            let year_min = years.iter().min().copied().or(album.year_min);
            let year_max = years.iter().max().copied().or(album.year_max);

            // Use album.title as the source of truth for the folder name
            // But strip existing year prefix if present
            let safe_title = sanitize_filename(&album.title);
            let re = Regex::new(r"^\(\d{4}(?:-\d{2})?\)\s*").unwrap();
            let clean_title = re.replace(&safe_title, "").to_string();

            let new_folder_name =
                renamer.format_folder_name(&album.artist, &clean_title, year_min, year_max);

            let current_folder_name = current_folder.file_name().unwrap().to_string_lossy();

            if new_folder_name != current_folder_name {
                let new_folder_path = parent_of_folder.join(&new_folder_name);
                IOService::rename_folder(
                    current_folder.to_str().unwrap(),
                    new_folder_path.to_str().unwrap(),
                )?;

                // Update Album Path
                album.path = new_folder_path.to_string_lossy().to_string();

                // Update all track paths
                for track in &mut album.tracks {
                    let p = PathBuf::from(&track.path);
                    let filename = p.file_name().unwrap();
                    let new_p = new_folder_path.join(filename);
                    track.path = new_p.to_string_lossy().to_string();
                }

                // Update cover path if it was inside the folder
                if let Some(cover) = &album.cover_path {
                    let cover_path = PathBuf::from(cover);
                    let mut updated = false;

                    // Strategy 1: Path prefix replacement
                    if cover_path.starts_with(current_folder) {
                        // Reconstruct path
                        if let Ok(relative) = cover_path.strip_prefix(current_folder) {
                            album.cover_path =
                                Some(new_folder_path.join(relative).to_string_lossy().to_string());
                            updated = true;
                        }
                    }

                    // Strategy 2: If prefix failed (e.g. symlinks or path mismatch), try filename in new folder
                    if !updated {
                        if let Some(file_name) = cover_path.file_name() {
                            let candidate = new_folder_path.join(file_name);
                            // We assume that since we renamed the folder, the file is there
                            // We can check existence to be sure, but if we just renamed, it should be there
                            if candidate.exists() {
                                album.cover_path = Some(candidate.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // 4. Re-evaluate album status (Clean/Dirty)
    ValidatorService::evaluate_album_status(&mut album);

    Ok(album)
}
