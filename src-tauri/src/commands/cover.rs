use crate::services::cover::{CoverResult, CoverService};
use std::sync::Mutex;
use tauri::State;

pub struct CoverServiceState(pub Mutex<CoverService>);

#[tauri::command]
pub async fn search_cover(
    _state: State<'_, CoverServiceState>,
    artist: String,
    album: String,
) -> Result<Vec<CoverResult>, String> {
    // Clone the service to avoid holding the lock across await
    // But CoverService is stateless, so we can just create a new one or clone it if it was cheap.
    // However, here it's inside a Mutex.
    // The issue is that MutexGuard is not Send.
    // Since CoverService is stateless (empty struct), we can just instantiate it.
    // OR we can make CoverService Clone.

    let service = CoverService::new();
    service.search_cover(&artist, &album).await
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
