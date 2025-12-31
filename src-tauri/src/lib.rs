pub mod commands;
pub mod db;
pub mod models;
pub mod services;

use commands::cover::CoverServiceState;
use commands::playlist::PlaylistServiceState;
use commands::{
    add_exception, auto_correct_album,
    converter::convert_file,
    cover::{download_cover, search_cover},
    delete_exception, get_exceptions,
    history::get_scan_history,
    player::{pause_track, play_track, resume_track, set_volume, stop_track},
    playlist::{add_to_playlist, create_playlist, get_playlist_tracks, list_playlists},
    save_album_changes, scan_directory, scan_junk,
};
use db::Database;
use services::cover::CoverService;
use services::player::AudioPlayerState;
use services::playlist::PlaylistService;
use std::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = Database::init().expect("error initializing database");
    let player_state = AudioPlayerState::new();
    let cover_service = CoverServiceState(Mutex::new(CoverService::new()));
    let playlist_service = PlaylistServiceState(Mutex::new(PlaylistService::new()));

    tauri::Builder::default()
        .manage(db)
        .manage(player_state)
        .manage(cover_service)
        .manage(playlist_service)
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            scan_directory,
            scan_junk,
            auto_correct_album,
            add_exception,
            get_exceptions,
            delete_exception,
            save_album_changes,
            play_track,
            pause_track,
            resume_track,
            stop_track,
            set_volume,
            search_cover,
            download_cover,
            list_playlists,
            get_playlist_tracks,
            create_playlist,
            add_to_playlist,
            convert_file,
            get_scan_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
