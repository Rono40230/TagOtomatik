use crate::db::Database;
use crate::models::{Album, AppError};
use crate::services::{ExceptionService, MetadataProcessorService};
use std::collections::HashMap;
use tauri::State;

#[tauri::command]
pub async fn auto_correct_album(
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

    for track in &mut album.tracks {
        processor.nettoyer_track(track, &exceptions_map);
    }

    Ok(album)
}
