use super::*;

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
fn test_format_folder_name() {
    let processor = MetadataProcessorService::new();

    assert_eq!(
        processor.format_folder_name("Artist", "Album", Some(2020)),
        "(2020) Album"
    );
    assert_eq!(
        processor.format_folder_name("Artist", "Album", None),
        "Album"
    );

    // Artist Exception
    // Expect Sentence Case: "Live at river plate"
    assert_eq!(
        processor.format_folder_name("AC/DC", "Live at River Plate", Some(2012)),
        "(2012) Live at river plate"
    );

    // If album contains artist name, preserve artist case
    // AC/DC -> AC-DC in folder name
    assert_eq!(
        processor.format_folder_name("AC/DC", "The AC/DC Collection", Some(2000)),
        "(2000) The AC-DC collection"
    );
}

#[test]
fn test_format_track_filename() {
    let processor = MetadataProcessorService::new();

    assert_eq!(
        processor.format_track_filename(Some(1), "Title", "mp3"),
        "01. Title.mp3"
    );
    assert_eq!(
        processor.format_track_filename(Some(10), "Title", ".FLAC"),
        "10. Title.FLAC"
    );
    assert_eq!(
        processor.format_track_filename(None, "Title", "mp3"),
        "00. Title.mp3"
    );

    // Sanitization
    assert_eq!(
        processor.format_track_filename(Some(1), "Title/With/Slash", "mp3"),
        "01. Title-With-Slash.mp3"
    );
}
