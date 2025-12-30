use crate::db::Database;
use crate::models::{AppError, CaseException};
use crate::services::ExceptionService;
use tauri::State;

#[tauri::command]
pub async fn add_exception(
    db: State<'_, Database>,
    original: String,
    corrected: String,
    category: String,
) -> Result<CaseException, AppError> {
    ExceptionService::create(&db, original, corrected, category)
}

#[tauri::command]
pub async fn get_exceptions(db: State<'_, Database>) -> Result<Vec<CaseException>, AppError> {
    ExceptionService::get_all(&db)
}

#[tauri::command]
pub async fn delete_exception(db: State<'_, Database>, id: i64) -> Result<(), AppError> {
    ExceptionService::delete(&db, id)
}
