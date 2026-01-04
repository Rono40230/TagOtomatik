use super::*;
use crate::models::Track;
use crate::services::processor::ReplacementRule;
use regex::Regex;

#[test]
fn test_corriger_casse() {
    let processor = MetadataProcessorService::new();

    // Basic Sentence Case
    assert_eq!(processor.corriger_casse("HELLO WORLD"), "Hello world");
    assert_eq!(processor.corriger_casse("hello world"), "Hello world");

    // Roman Numerals
    assert_eq!(processor.corriger_casse("part ii"), "Part II");
    assert_eq!(processor.corriger_casse("chapter iv"), "Chapter IV");
    assert_eq!(processor.corriger_casse("louis xiv"), "Louis XIV");

    // Abbreviations
    assert_eq!(processor.corriger_casse("live in usa"), "Live in USA");
    assert_eq!(processor.corriger_casse("bbc news"), "BBC news");

    // "I" isolated
    assert_eq!(processor.corriger_casse("am i wrong"), "Am I wrong");
    assert_eq!(processor.corriger_casse("i love you"), "I love you");

    // Punctuation
    assert_eq!(processor.corriger_casse("hello. world"), "Hello. World");
    assert_eq!(
        processor.corriger_casse("title: subtitle"),
        "Title: Subtitle"
    );
    assert_eq!(processor.corriger_casse("(remix)"), "(Remix)");
}

#[test]
fn test_nettoyer_track_parentheses() {
    let processor = MetadataProcessorService::new();
    let mut track = Track {
        path: "".to_string(),
        filename: "".to_string(),
        title: "Song Title (Remix)".to_string(),
        artist: "Artist (feat. Someone)".to_string(),
        album_artist: "".to_string(),
        album: "Album Title (Deluxe Edition)".to_string(),
        year: None,
        track_number: None,
        genre: None,
        duration_sec: 0,
        format: "".to_string(),
        bit_rate: None,
        size: 0,
        has_cover: false,
        original_metadata: None,
        is_modified: false,
    };

    let rules = Vec::new();
    processor.nettoyer_track(&mut track, &rules);

    assert_eq!(track.title, "Song title");
    assert_eq!(track.album, "Album title");
}

#[test]
fn test_format_folder_name() {
    let renamer = RenamerService::new();

    assert_eq!(
        renamer.format_folder_name("Artist", "Album", Some(2020)),
        "(2020) Album"
    );
    assert_eq!(renamer.format_folder_name("Artist", "Album", None), "Album");

    // Artist Exception
    // Expect Sentence Case: "Live at river plate"
    assert_eq!(
        renamer.format_folder_name("AC/DC", "Live at River Plate", Some(2012)),
        "(2012) Live at river plate"
    );

    // If album contains artist name, preserve artist case
    // AC/DC -> AC-DC in folder name
    assert_eq!(
        renamer.format_folder_name("AC/DC", "The AC/DC Collection", Some(2000)),
        "(2000) The AC-DC collection"
    );
}

#[test]
fn test_format_track_filename() {
    let renamer = RenamerService::new();

    assert_eq!(
        renamer.format_track_filename(Some(1), "Title", "mp3"),
        "01 - Title.mp3"
    );
    assert_eq!(
        renamer.format_track_filename(Some(10), "Title", ".FLAC"),
        "10 - Title.FLAC"
    );
    assert_eq!(
        renamer.format_track_filename(None, "Title", "mp3"),
        "00 - Title.mp3"
    );

    // Sanitization
    assert_eq!(
        renamer.format_track_filename(Some(1), "Title/With/Slash", "mp3"),
        "01 - Title-With-Slash.mp3"
    );
}

#[test]
fn test_appliquer_exceptions_regex() {
    let processor = MetadataProcessorService::new();
    let mut track = Track {
        path: "".to_string(),
        filename: "".to_string(),
        title: "Hung et and I".to_string(),
        artist: "Artist".to_string(),
        album_artist: "".to_string(),
        album: "Album".to_string(),
        year: None,
        track_number: None,
        genre: None,
        duration_sec: 0,
        format: "".to_string(),
        bit_rate: None,
        size: 0,
        has_cover: false,
        original_metadata: None,
        is_modified: false,
    };

    let mut rules = Vec::new();
    // Rule 1: "et" -> "&"
    rules.push(ReplacementRule {
        category: "global".to_string(),
        regex: Regex::new(r"(?i)\bet\b").unwrap(),
        replacement: "&".to_string(),
    });
    // Rule 2: "and" -> "&"
    rules.push(ReplacementRule {
        category: "global".to_string(),
        regex: Regex::new(r"(?i)\band\b").unwrap(),
        replacement: "&".to_string(),
    });

    processor.appliquer_exceptions(&mut track, &rules);

    assert_eq!(track.title, "Hung & & I");
}
