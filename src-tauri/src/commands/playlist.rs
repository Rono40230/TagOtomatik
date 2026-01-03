use crate::models::playlist::Playlist;
use crate::models::playlist::PlaylistTrack;
use crate::services::playlist::PlaylistService;
use std::sync::Mutex;
use tauri::State;

pub struct PlaylistServiceState(pub Mutex<PlaylistService>);

#[tauri::command]
pub async fn list_playlists(
    state: State<'_, PlaylistServiceState>,
    root_path: String,
) -> Result<Vec<Playlist>, String> {
    let service = state.0.lock().unwrap();
    service.list_playlists(&root_path)
}

#[tauri::command]
pub async fn get_playlist_tracks(
    state: State<'_, PlaylistServiceState>,
    playlist_path: String,
) -> Result<Vec<PlaylistTrack>, String> {
    let service = state.0.lock().unwrap();
    service.get_playlist_tracks(&playlist_path)
}

#[tauri::command]
pub async fn create_playlist(
    state: State<'_, PlaylistServiceState>,
    root_path: String,
    name: String,
) -> Result<String, String> {
    let service = state.0.lock().unwrap();
    service.create_playlist(&root_path, &name)
}

#[tauri::command]
pub async fn add_to_playlist(
    state: State<'_, PlaylistServiceState>,
    playlist_path: String,
    track_path: String,
) -> Result<(), String> {
    let service = state.0.lock().unwrap();
    service.add_track_to_playlist(&playlist_path, &track_path)
}

#[tauri::command]
pub async fn write_playlist(
    state: State<'_, PlaylistServiceState>,
    options: crate::models::playlist::PlaylistOptions,
) -> Result<String, String> {
    let service = state.0.lock().unwrap();
    service.write_playlist(&options)
}
