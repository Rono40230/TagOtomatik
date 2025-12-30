use crate::models::AppError;
use std::fs;

pub struct IOService;

impl IOService {
    pub fn rename_file(old_path: &str, new_path: &str) -> Result<(), AppError> {
        fs::rename(old_path, new_path)
            .map_err(|e| AppError::Io(format!("Erreur renommage fichier: {}", e)))
    }

    pub fn rename_folder(old_path: &str, new_path: &str) -> Result<(), AppError> {
        fs::rename(old_path, new_path)
            .map_err(|e| AppError::Io(format!("Erreur renommage dossier: {}", e)))
    }
}
