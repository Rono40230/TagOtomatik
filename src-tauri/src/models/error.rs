use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum AppError {
    #[error("IO Error: {0}")]
    Io(String),

    #[error("Database Error: {0}")]
    DatabaseError(String),

    #[error("Audio Error: {0}")]
    Audio(String),

    #[error("Validation Error: {0}")]
    Validation(String),

    #[error("Unknown Error: {0}")]
    Unknown(String),
}

// Implement conversion from std::io::Error to AppError
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

// Implement conversion from rusqlite::Error to AppError
impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

// Implement conversion from lofty::error::LoftyError to AppError
impl From<lofty::error::LoftyError> for AppError {
    fn from(err: lofty::error::LoftyError) -> Self {
        AppError::Audio(err.to_string())
    }
}
