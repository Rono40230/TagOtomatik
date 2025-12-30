use crate::models::AppError;
use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn init() -> Result<Self, AppError> {
        let path = "tagotomatik.db";
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

        Ok(Database {
            conn: Mutex::new(conn),
        })
    }
}
