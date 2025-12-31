use crate::models::AppError;
use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn init() -> Result<Self, AppError> {
        // Move DB out of src-tauri to avoid infinite rebuild loop in dev mode
        let path = "../tagotomatik.db";
        let conn = Connection::open(path).map_err(|e| AppError::DatabaseError(e.to_string()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS exceptions (
                id INTEGER PRIMARY KEY,
                original TEXT NOT NULL,
                corrected TEXT NOT NULL,
                category TEXT NOT NULL,
                UNIQUE(original, category)
            )",
            [],
        )
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS scan_history (
                id INTEGER PRIMARY KEY,
                path TEXT NOT NULL UNIQUE,
                last_accessed DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(Database {
            conn: Mutex::new(conn),
        })
    }

    pub fn add_history(&self, path: &str) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO scan_history (path, last_accessed) VALUES (?1, CURRENT_TIMESTAMP)
             ON CONFLICT(path) DO UPDATE SET last_accessed = CURRENT_TIMESTAMP",
            [path],
        )
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub fn get_history(&self) -> Result<Vec<String>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT path FROM scan_history ORDER BY last_accessed DESC LIMIT 10")
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut history = Vec::new();
        for row in rows {
            history.push(row.map_err(|e| AppError::DatabaseError(e.to_string()))?);
        }
        Ok(history)
    }
}
