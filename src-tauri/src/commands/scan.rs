use crate::db::Database;
use crate::models::{AppError, ScanResult};
use crate::services::{ScannerService, ValidatorService};
use tauri::State;

#[tauri::command]
pub async fn scan_directory(path: String, db: State<'_, Database>) -> Result<ScanResult, AppError> {
    // Validation basique
    if path.is_empty() {
        return Err(AppError::Validation(
            "Le chemin ne peut pas être vide".to_string(),
        ));
    }

    // Add to history
    let _ = db.add_history(&path);

    // Exécution du scan (peut être long, donc async est bien, mais ScannerService est synchrone pour l'instant)
    // Idéalement on devrait le wrapper dans un spawn_blocking si c'est très lourd,
    // mais pour l'instant on l'appelle directement car tauri::command le gère dans un threadpool.

    let scanner = ScannerService::new();
    scanner.scanner_dossier(&path)
}

#[tauri::command]
pub async fn scan_junk(path: String) -> Result<Vec<String>, AppError> {
    ValidatorService::detecter_fichiers_inutiles(&path)
}
