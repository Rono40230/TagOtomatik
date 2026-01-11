use crate::db::Database;
use crate::models::{AppError, ScanResult};
use crate::services::{ScannerService, InspectorService};
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
pub async fn scan_junk(
    path: String, 
    artist: Option<String>, 
    title: Option<String>,
    year: Option<u32>,
    year_min: Option<u32>,
    year_max: Option<u32>
) -> Result<Vec<String>, AppError> {
    // Calcul de la chaîne d'année (identique à check_files)
    let year_str = if let (Some(min), Some(max)) = (year_min, year_max) {
             if min > 0 && max > 0 && min != max {
                 let max_short = max % 100;
                 format!("{}-{:02}", min, max_short)
             } else {
                 year.unwrap_or(0).to_string()
             }
    } else {
             year.unwrap_or(0).to_string()
    };
    
    let context = if let (Some(a), Some(t)) = (artist.as_ref(), title.as_ref()) {
        if !a.is_empty() && !t.is_empty() {
            Some((a.as_str(), year_str.as_str(), t.as_str()))
        } else {
            None
        }
    } else {
        None
    };

    InspectorService::detecter_fichiers_inutiles(&path, context)
}
