use crate::models::{Album, AppError};
use crate::services::{AudioService, IOService};
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

    // 1. Save tags
    for track in &mut album.tracks {
        if track.is_modified {
            audio_service.ecrire_metadonnees(track)?;
            track.is_modified = false;
            track.original_metadata = Some(Box::new(track.clone()));
        }
    }

    // 2. Rename files
    for track in &mut album.tracks {
        let old_path = PathBuf::from(&track.path);
        let parent = old_path
            .parent()
            .ok_or_else(|| AppError::Io("Invalid path".into()))?;
        let ext = old_path.extension().unwrap_or_default().to_string_lossy();

        let track_num = track.track_number.unwrap_or(0);
        let title = sanitize_filename(&track.title);
        let new_filename = format!("{:02} - {}.{}", track_num, title, ext);

        if new_filename != track.filename {
            let new_path = parent.join(&new_filename);
            IOService::rename_file(old_path.to_str().unwrap(), new_path.to_str().unwrap())?;

            track.filename = new_filename;
            track.path = new_path.to_string_lossy().to_string();
        }
    }

    // 3. Rename folder (Album level)
    if let Some(first_track) = album.tracks.first() {
        let track_path = PathBuf::from(&first_track.path);
        let current_folder = track_path.parent().unwrap();

        // Check if we have a parent folder to rename within
        if let Some(parent_of_folder) = current_folder.parent() {
            let artist = sanitize_filename(&album.artist);
            let album_title = sanitize_filename(&album.title);
            let new_folder_name = format!("{} - {}", artist, album_title);

            let current_folder_name = current_folder.file_name().unwrap().to_string_lossy();

            if new_folder_name != current_folder_name {
                let new_folder_path = parent_of_folder.join(&new_folder_name);
                IOService::rename_folder(
                    current_folder.to_str().unwrap(),
                    new_folder_path.to_str().unwrap(),
                )?;

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
                    if cover_path.starts_with(current_folder) {
                        // Reconstruct path
                        if let Ok(relative) = cover_path.strip_prefix(current_folder) {
                            album.cover_path =
                                Some(new_folder_path.join(relative).to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(album)
}
