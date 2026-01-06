use super::Album;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub albums: Vec<Album>,
    pub errors: Vec<String>,
}
