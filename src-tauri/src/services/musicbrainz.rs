use crate::models::AppError;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const USER_AGENT: &str = "TagOtomatik/0.1.0 ( contact@example.com )"; // TODO: Configurable contact
const BASE_URL: &str = "https://musicbrainz.org/ws/2";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicBrainzRelease {
    pub id: String,
    pub title: String,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub date: Option<String>,
    pub country: Option<String>,
    // Add more fields as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistCredit {
    pub name: String,
    // artist: Option<Artist>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchResponse {
    releases: Vec<MusicBrainzRelease>,
}

pub struct MusicBrainzService {
    client: Client,
}

impl Default for MusicBrainzService {
    fn default() -> Self {
        Self::new()
    }
}

impl MusicBrainzService {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent(USER_AGENT)
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn search_album(
        &self,
        artist: &str,
        album: &str,
    ) -> Result<Vec<MusicBrainzRelease>, AppError> {
        // Clean album title: remove (YYYY) or [YYYY] prefix
        let re = Regex::new(r"^[\(\[]\d{4}[\)\]]\s*").unwrap();
        let cleaned_album = re.replace(album, "");

        let query = format!("artist:\"{}\" AND release:\"{}\"", artist, cleaned_album);
        let url = format!("{}/release", BASE_URL);

        let response = self
            .client
            .get(&url)
            .query(&[("query", query), ("fmt", "json".to_string())])
            .send()
            .await
            .map_err(|e| AppError::Io(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::Io(format!(
                "MusicBrainz API Error: {}",
                response.status()
            )));
        }

        let search_result: SearchResponse = response
            .json()
            .await
            .map_err(|e| AppError::Io(e.to_string()))?;

        Ok(search_result.releases)
    }
}
