use crate::services::cover::{CoverResult, CoverService};
use crate::services::AudioService;
use std::path::Path;
use std::sync::Mutex;
use tauri::State;
use walkdir::WalkDir;

pub struct CoverServiceState(pub Mutex<CoverService>);

#[tauri::command]
pub async fn search_cover(
    _state: State<'_, CoverServiceState>,
    artist: String,
    album: String,
) -> Result<Vec<CoverResult>, String> {
    println!(
        "CMD: search_cover called with artist='{}', album='{}'",
        artist, album
    );
    let service = CoverService::new();
    let results = service.search_cover(&artist, &album).await;
    match &results {
        Ok(res) => println!("CMD: search_cover found {} results", res.len()),
        Err(e) => println!("CMD: search_cover error: {}", e),
    }
    results
}

#[tauri::command]
pub async fn download_cover(
    _state: State<'_, CoverServiceState>,
    url: String,
    target_path: String,
) -> Result<(), String> {
    let service = CoverService::new();
    service.download_cover(&url, &target_path).await
}

#[tauri::command]
pub async fn apply_cover(
    _state: State<'_, CoverServiceState>,
    album_path: String,
    cover_url: String,
) -> Result<String, String> {
    let cover_service = CoverService::new();
    let audio_service = AudioService::new();
    let path = Path::new(&album_path);
    let target_cover_path = path.join("cover.jpg");
    let target_cover_str = target_cover_path.to_string_lossy().to_string();

    // 1. Download Cover
    cover_service
        .download_cover(&cover_url, &target_cover_str)
        .await?;

    // 2. Apply to all tracks
    let supported_extensions = ["mp3", "flac", "m4a", "ogg", "wav", "aiff"];

    for entry in WalkDir::new(&album_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if supported_extensions.contains(&ext.to_lowercase().as_str()) {
                    // Apply cover
                    if let Err(e) =
                        audio_service.definir_cover(&path.to_string_lossy(), &target_cover_str)
                    {
                        eprintln!("Failed to set cover for {:?}: {}", path, e);
                    }
                }
            }
        }
    }

    Ok(target_cover_str)
}

#[tauri::command]
pub async fn read_cover(path: String) -> Result<Vec<u8>, String> {
    std::fs::read(&path).map_err(|e| format!("Erreur lecture cover: {}", e))
}

#[tauri::command]
pub async fn read_track_cover(path: String) -> Result<Vec<u8>, String> {
    use lofty::{Probe, TaggedFileExt};

    let tagged_file = Probe::open(&path)
        .map_err(|e| format!("Erreur ouverture: {}", e))?
        .read()
        .map_err(|e| format!("Erreur lecture: {}", e))?;

    let tag = tagged_file
        .primary_tag()
        .ok_or_else(|| "Pas de tag trouvé".to_string())?;

    let pictures = tag.pictures();
    if let Some(pic) = pictures.first() {
        Ok(pic.data().to_vec())
    } else {
        Err("Pas de cover trouvée".to_string())
    }
}
