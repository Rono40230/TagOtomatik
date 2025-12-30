use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CaseException {
    pub id: Option<i64>,
    pub original: String,
    pub corrected: String,
    pub category: String, // "artist", "album", "global"
}

impl CaseException {
    pub fn new(original: String, corrected: String, category: String) -> Self {
        Self {
            id: None,
            original,
            corrected,
            category,
        }
    }
}
