use crate::services::player::AudioPlayerState;
use tauri::State;

#[tauri::command]
pub async fn play_track(state: State<'_, AudioPlayerState>, path: String) -> Result<(), String> {
    state.play(&path)
}

#[tauri::command]
pub async fn pause_track(state: State<'_, AudioPlayerState>) -> Result<(), String> {
    state.pause();
    Ok(())
}

#[tauri::command]
pub async fn resume_track(state: State<'_, AudioPlayerState>) -> Result<(), String> {
    state.resume();
    Ok(())
}

#[tauri::command]
pub async fn stop_track(state: State<'_, AudioPlayerState>) -> Result<(), String> {
    state.stop();
    Ok(())
}

#[tauri::command]
pub async fn set_volume(state: State<'_, AudioPlayerState>, volume: f32) -> Result<(), String> {
    state.set_volume(volume);
    Ok(())
}
