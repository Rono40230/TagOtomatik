use crate::models::{Album, AppError};
use crate::services::ScannerService;

#[tauri::command]
pub async fn scan_directory(path: String) -> Result<Vec<Album>, AppError> {
    // Validation basique
    if path.is_empty() {
        return Err(AppError::Validation(
            "Le chemin ne peut pas être vide".to_string(),
        ));
    }

    // Exécution du scan (peut être long, donc async est bien, mais ScannerService est synchrone pour l'instant)
    // Idéalement on devrait le wrapper dans un spawn_blocking si c'est très lourd,
    // mais pour l'instant on l'appelle directement car tauri::command le gère dans un threadpool.

    let scanner = ScannerService::new();
    scanner.scanner_dossier(&path)
}
