use image::ImageFormat;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoverResult {
    pub url: String,
    pub source: String, // "MusicBrainz", "Discogs"...
    pub size: Option<(u32, u32)>,
    pub preview_data: Option<Vec<u8>>,
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
    thumbnails: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct CoverArtResponse {
    images: Vec<CoverArtImage>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ItunesAlbum {
    #[serde(rename = "artworkUrl100")]
    artwork_url_100: String,
    #[serde(rename = "collectionName")]
    collection_name: String,
    #[serde(rename = "artistName")]
    artist_name: String,
}

#[derive(Debug, Deserialize)]
struct ItunesResponse {
    results: Vec<ItunesAlbum>,
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

    async fn download_preview(&self, url: &str) -> Option<Vec<u8>> {
        // Use browser UA for images to avoid blocking (CDNs often block bots)
        match self
            .client
            .get(url)
            .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status().is_success() {
                    let bytes = resp.bytes().await.ok().map(|b| b.to_vec())?;
                    // Check dimensions
                    if let Ok(img) = image::load_from_memory(&bytes) {
                        if img.width() >= 300 && img.height() >= 300 {
                            return Some(bytes);
                        } else {
                            println!(
                                "SERVICE: Image too small ({}x{}), skipping: {}",
                                img.width(),
                                img.height(),
                                url
                            );
                        }
                    }
                    None
                } else {
                    None
                }
            }
            Err(e) => {
                println!("SERVICE: Preview download failed for {}: {}", url, e);
                None
            }
        }
    }

    pub async fn search_cover(
        &self,
        artist: &str,
        album: &str,
    ) -> Result<Vec<CoverResult>, String> {
        println!("SERVICE: Starting search for '{}' - '{}'", artist, album);
        let mut results = Vec::new();

        // 1. Recherche MusicBrainz
        let query = format!("artist:{} AND release:{}", artist, album);
        let url = format!(
            "https://musicbrainz.org/ws/2/release-group/?query={}&fmt=json",
            urlencoding::encode(&query)
        );
        println!("SERVICE: MusicBrainz URL: {}", url);

        match self.client.get(&url).send().await {
            Ok(resp) => {
                println!("SERVICE: MusicBrainz Status: {}", resp.status());
                if resp.status().is_success() {
                    if let Ok(mb_data) = resp.json::<MusicBrainzSearchResponse>().await {
                        println!(
                            "SERVICE: MusicBrainz found {} release groups",
                            mb_data.release_groups.len()
                        );

                        for release in mb_data.release_groups.iter().take(3) {
                            // Pour chaque release, chercher la cover sur CoverArtArchive
                            let cover_url =
                                format!("https://coverartarchive.org/release-group/{}", release.id);
                            if let Ok(cover_resp) = self.client.get(&cover_url).send().await {
                                if cover_resp.status().is_success() {
                                    if let Ok(cover_data) =
                                        cover_resp.json::<CoverArtResponse>().await
                                    {
                                        for img in cover_data.images {
                                            if img.front {
                                                println!("SERVICE: Found MB cover: {}", img.image);

                                                // Determine preview URL (prefer 500px or large thumbnail for quality check)
                                                let preview_url =
                                                    if let Some(thumbs) = &img.thumbnails {
                                                        thumbs
                                                            .get("500")
                                                            .or_else(|| thumbs.get("large"))
                                                            .unwrap_or(&img.image)
                                                            .clone()
                                                    } else {
                                                        img.image.clone()
                                                    };

                                                // Download preview (and validate size >= 300x300)
                                                if let Some(preview_data) =
                                                    self.download_preview(&preview_url).await
                                                {
                                                    // Force HTTPS for the main URL just in case
                                                    let secure_url =
                                                        img.image.replace("http://", "https://");

                                                    results.push(CoverResult {
                                                        url: secure_url,
                                                        source: "MusicBrainz".to_string(),
                                                        size: None,
                                                        preview_data: Some(preview_data),
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    println!("SERVICE: MusicBrainz failed with status: {}", resp.status());
                }
            }
            Err(e) => {
                println!("SERVICE: MusicBrainz request failed: {}", e);
            }
        }

        // 2. Recherche iTunes
        let term = format!("{} {}", artist, album);
        let itunes_url = format!(
            "https://itunes.apple.com/search?term={}&entity=album&limit=5",
            urlencoding::encode(&term)
        );
        println!("SERVICE: iTunes URL: {}", itunes_url);

        match self.client.get(&itunes_url).send().await {
            Ok(resp) => {
                println!("SERVICE: iTunes Status: {}", resp.status());
                if let Ok(itunes_data) = resp.json::<ItunesResponse>().await {
                    println!(
                        "SERVICE: iTunes found {} results",
                        itunes_data.results.len()
                    );
                    for result in itunes_data.results {
                        // Use 600x600 for preview to pass the 300x300 check
                        let preview_url = result.artwork_url_100.replace("100x100bb", "600x600bb");

                        // Download preview (and validate size >= 300x300)
                        if let Some(preview_data) = self.download_preview(&preview_url).await {
                            // iTunes returns 100x100 by default, but we can get higher res by replacing the URL
                            let high_res_url =
                                result.artwork_url_100.replace("100x100bb", "1000x1000bb");
                            results.push(CoverResult {
                                url: high_res_url,
                                source: "iTunes".to_string(),
                                size: None,
                                preview_data: Some(preview_data),
                            });
                        }
                    }
                } else {
                    println!("SERVICE: iTunes JSON parse error");
                }
            }
            Err(e) => println!("SERVICE: iTunes request error: {}", e),
        }

        println!("SERVICE: Total results found: {}", results.len());
        Ok(results)
    }

    pub async fn download_cover(&self, url: &str, target_path: &str) -> Result<(), String> {
        let resp = self
            .client
            .get(url)
            .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
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

        // Check minimum size for final download too
        if img.width() < 300 || img.height() < 300 {
            return Err(format!(
                "Image trop petite ({}x{}), minimum 300x300 requis.",
                img.width(),
                img.height()
            ));
        }

        let mut output = File::create(target_path).map_err(|e| e.to_string())?;
        img.write_to(&mut output, ImageFormat::Jpeg)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
