use image::ImageFormat;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoverResult {
    pub url: String,
    pub source: String, // "MusicBrainz", "Discogs"...
    pub size: Option<(u32, u32)>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MusicBrainzReleaseGroup {
    id: String,
    title: String,
    #[serde(rename = "primary-type")]
    primary_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MusicBrainzSearchResponse {
    #[serde(rename = "release-groups")]
    release_groups: Vec<MusicBrainzReleaseGroup>,
}

#[derive(Debug, Deserialize)]
struct CoverArtImage {
    image: String,
    front: bool,
}

#[derive(Debug, Deserialize)]
struct CoverArtResponse {
    images: Vec<CoverArtImage>,
}

pub struct CoverService {
    client: Client,
}

impl Default for CoverService {
    fn default() -> Self {
        Self::new()
    }
}

impl CoverService {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("TagOtomatik/0.1.0 ( contact@rono.com )")
                .build()
                .unwrap(),
        }
    }

    pub async fn search_cover(
        &self,
        artist: &str,
        album: &str,
    ) -> Result<Vec<CoverResult>, String> {
        let mut results = Vec::new();

        // 1. Recherche MusicBrainz
        let query = format!("artist:{} AND release:{}", artist, album);
        let url = format!(
            "https://musicbrainz.org/ws/2/release-group/?query={}&fmt=json",
            urlencoding::encode(&query)
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        let mb_data: MusicBrainzSearchResponse = resp.json().await.map_err(|e| e.to_string())?;

        for release in mb_data.release_groups.iter().take(3) {
            // Pour chaque release, chercher la cover sur CoverArtArchive
            let cover_url = format!("https://coverartarchive.org/release-group/{}", release.id);
            if let Ok(cover_resp) = self.client.get(&cover_url).send().await {
                if cover_resp.status().is_success() {
                    if let Ok(cover_data) = cover_resp.json::<CoverArtResponse>().await {
                        for img in cover_data.images {
                            if img.front {
                                results.push(CoverResult {
                                    url: img.image,
                                    source: "MusicBrainz".to_string(),
                                    size: None, // On ne connaît pas la taille sans télécharger
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    pub async fn download_cover(&self, url: &str, target_path: &str) -> Result<(), String> {
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let bytes = resp.bytes().await.map_err(|e| e.to_string())?;

        // Redimensionner et convertir en JPEG standard
        let img = image::load_from_memory(&bytes).map_err(|e| e.to_string())?;

        // Resize si trop grand (ex: max 1000x1000)
        let img = if img.width() > 1000 || img.height() > 1000 {
            img.resize(1000, 1000, image::imageops::FilterType::Lanczos3)
        } else {
            img
        };

        let mut output = File::create(target_path).map_err(|e| e.to_string())?;
        img.write_to(&mut output, ImageFormat::Jpeg)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
