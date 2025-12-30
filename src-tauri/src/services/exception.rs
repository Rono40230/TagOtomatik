use crate::db::Database;
use crate::models::{AppError, CaseException};
use rusqlite::params;

pub struct ExceptionService;

impl ExceptionService {
    pub fn create(
        db: &Database,
        original: String,
        corrected: String,
        category: String,
    ) -> Result<CaseException, AppError> {
        let conn = db
            .conn
            .lock()
            .map_err(|_| AppError::DatabaseError("Lock error".into()))?;

        conn.execute(
            "INSERT OR REPLACE INTO exceptions (original, corrected, category) VALUES (?1, ?2, ?3)",
            params![original, corrected, category],
        )
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let id = conn.last_insert_rowid();

        Ok(CaseException {
            id: Some(id),
            original,
            corrected,
            category,
        })
    }

    pub fn get_all(db: &Database) -> Result<Vec<CaseException>, AppError> {
        let conn = db
            .conn
            .lock()
            .map_err(|_| AppError::DatabaseError("Lock error".into()))?;

        let mut stmt = conn
            .prepare("SELECT id, original, corrected, category FROM exceptions")
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let exception_iter = stmt
            .query_map([], |row| {
                Ok(CaseException {
                    id: Some(row.get(0)?),
                    original: row.get(1)?,
                    corrected: row.get(2)?,
                    category: row.get(3)?,
                })
            })
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut exceptions = Vec::new();
        for exception in exception_iter {
            exceptions.push(exception.map_err(|e| AppError::DatabaseError(e.to_string()))?);
        }

        Ok(exceptions)
    }

    pub fn delete(db: &Database, id: i64) -> Result<(), AppError> {
        let conn = db
            .conn
            .lock()
            .map_err(|_| AppError::DatabaseError("Lock error".into()))?;

        conn.execute("DELETE FROM exceptions WHERE id = ?1", params![id])
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
