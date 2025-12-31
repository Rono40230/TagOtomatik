use crate::db::Database;
use tauri::State;

#[tauri::command]
pub fn get_scan_history(db: State<Database>) -> Result<Vec<String>, String> {
    db.get_history().map_err(|e| e.to_string())
}
