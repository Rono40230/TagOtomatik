use crate::models::AppError;
use crate::services::musicbrainz::MusicBrainzRelease;
use crate::services::MusicBrainzService;

#[tauri::command]
pub async fn search_musicbrainz(
    artist: String,
    album: String,
) -> Result<Vec<MusicBrainzRelease>, AppError> {
    let service = MusicBrainzService::new();
    let results = service.search_album(&artist, &album).await?;
    Ok(results)
}
