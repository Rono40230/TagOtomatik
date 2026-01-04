use crate::commands::apply::apply_auto_correct_logic;
use crate::db::Database;
use crate::models::{Album, AppError};
use crate::services::processor::ReplacementRule;
use crate::services::{ExceptionService, MetadataProcessorService};
use regex::Regex;
use std::path::Path;
use tauri::State;

#[tauri::command]
pub async fn preview_auto_correct(
    db: State<'_, Database>,
    mut album: Album,
) -> Result<Album, AppError> {
    let processor = MetadataProcessorService::new();

    // Fetch exceptions
    let mut exceptions_list = ExceptionService::get_all(&db)?;
    // Sort by length descending to handle overlapping matches (e.g. "New York" before "New")
    exceptions_list.sort_by(|a, b| b.original.len().cmp(&a.original.len()));

    let mut rules = Vec::new();
    for ex in exceptions_list {
        let pattern = regex::escape(&ex.original);
        // Heuristic: if it starts/ends with alphanumeric, add word boundary
        let start_boundary = if ex
            .original
            .chars()
            .next()
            .is_some_and(char::is_alphanumeric)
        {
            "\\b"
        } else {
            ""
        };
        let end_boundary = if ex
            .original
            .chars()
            .last()
            .is_some_and(char::is_alphanumeric)
        {
            "\\b"
        } else {
            ""
        };

        let regex_str = format!(r"(?i){}{}{}", start_boundary, pattern, end_boundary);

        if let Ok(re) = Regex::new(&regex_str) {
            rules.push(ReplacementRule {
                category: ex.category.to_lowercase(),
                regex: re,
                replacement: ex.corrected,
            });
        }
    }

    // 1. Correct Filename Only (as requested by user workflow change)
    for track in &mut album.tracks {
        // processor.nettoyer_track(track, &exceptions_map); // Disabled: User wants to clean filename instead

        // Correct Filename
        let new_filename = processor.nettoyer_filename(&track.filename, track.track_number, &rules);
        if new_filename != track.filename {
            track.filename = new_filename;
            track.is_modified = true;
        }

        // Sync Title from Filename (Reflect corrected filename in Title tag)
        let path = Path::new(&track.filename);
        if let Some(stem) = path.file_stem() {
            let stem_str = stem.to_string_lossy().to_string();
            // Remove "NN - " prefix (e.g. "01 - ")
            let re_prefix = Regex::new(r"^\d{2,3}\s*-\s*").unwrap();
            let clean_title = re_prefix.replace(&stem_str, "").to_string();

            if track.title != clean_title {
                track.title = clean_title;
                track.is_modified = true;
            }
        }

        // Apply exceptions to all fields (Artist, Album, Title)
        processor.appliquer_exceptions(track, &rules);

        // Correct Album (as requested)
        let old_album = track.album.clone();
        processor.nettoyer_album_metadata(track, &rules);
        if track.album != old_album {
            track.is_modified = true;
        }
    }

    // Update Album-level metadata from the first track (to reflect changes in UI header)
    if let Some(first_track) = album.tracks.first() {
        // Calculate Year (Min) for display
        let years: Vec<u32> = album.tracks.iter().filter_map(|t| t.year).collect();
        let year_str = if let Some(min_year) = years.iter().min() {
            format!("({}) ", min_year)
        } else {
            String::new()
        };

        album.title = format!("{}{}", year_str, first_track.album);
        // album.artist = first_track.album_artist.clone(); // If we were correcting artists
    }

    Ok(album)
}

#[tauri::command]
pub async fn apply_auto_correct(album: Album) -> Result<Album, AppError> {
    apply_auto_correct_logic(album).await
}
