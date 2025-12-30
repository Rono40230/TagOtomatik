pub mod commands;
pub mod db;
pub mod models;
pub mod services;

use commands::{
    add_exception, auto_correct_album, delete_exception, get_exceptions, save_album_changes,
    scan_directory,
};
use db::Database;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = Database::init().expect("error initializing database");

    tauri::Builder::default()
        .manage(db)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            scan_directory,
            auto_correct_album,
            add_exception,
            get_exceptions,
            delete_exception,
            save_album_changes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
