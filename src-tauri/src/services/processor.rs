use crate::models::Track;
use crate::services::dictionaries::{ABBREVIATIONS, ROMAN_NUMERALS};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    // Regex pour nettoyer les espaces multiples
    static ref RE_SPACES: Regex = Regex::new(r"\s+").unwrap();

    // Regex pour les patterns indésirables (ex: [320kbps], www.site.com)
    static ref RE_GARBAGE: Regex = Regex::new(r"(?i)(\[.*?\]|\(.*?\)|\bwww\..*?\.[a-z]{2,4}\b)").unwrap();

    // Regex pour normaliser les connecteurs
    static ref RE_FEAT: Regex = Regex::new(r"(?i)\s+(feat\.?|ft\.?|with)\s+").unwrap();
    static ref RE_AND: Regex = Regex::new(r"(?i)\s+and\s+").unwrap();
}

pub struct MetadataProcessorService;

impl Default for MetadataProcessorService {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataProcessorService {
    pub fn new() -> Self {
        Self
    }

    pub fn nettoyer_track(
        &self,
        track: &mut Track,
        exceptions: &HashMap<(String, String), String>,
    ) {
        track.title = self.nettoyer_chaine(&track.title);
        track.artist = self.nettoyer_chaine(&track.artist);
        track.album = self.nettoyer_chaine(&track.album);

        // Rule: Artist -> Album Artist if empty
        // Note: Track struct might not have album_artist field visible here?
        // Let's check Track definition. If it's missing, we can't do it.
        // Assuming Track has it or we skip it.
        // Checking models/track.rs content previously read...
        // Track struct has: title, artist, album, year, track_number, genre...
        // It does NOT have album_artist in the struct shown earlier!
        // We must skip this rule or add the field.
        // For now, let's stick to what we have.

        // Genre cleaning
        if let Some(g) = &track.genre {
            let clean_g = self.nettoyer_chaine(g);
            // Handle ID3v1 numeric genres (simple check)
            let normalized = if clean_g.starts_with('(') && clean_g.ends_with(')') {
                // Try to parse number inside
                let inner = &clean_g[1..clean_g.len() - 1];
                if inner.parse::<u32>().is_ok() {
                    // TODO: Map ID to string. For now, keep as is or clear?
                    // Python script converted it. We don't have the map yet.
                    // Let's just normalize casing for now.
                    self.normalize_genre(&clean_g)
                } else {
                    self.normalize_genre(&clean_g)
                }
            } else {
                self.normalize_genre(&clean_g)
            };
            track.genre = Some(normalized);
        }

        track.title = self.corriger_casse(&track.title);
        // Artiste : On préserve la casse originale (sauf nettoyage basique fait au-dessus)
        // track.artist = self.corriger_casse(&track.artist);
        track.album = self.corriger_casse(&track.album);

        self.appliquer_exceptions(track, exceptions);

        // Marquer comme modifié si différent de l'original
        if let Some(original) = &track.original_metadata {
            if track.title != original.title
                || track.artist != original.artist
                || track.album != original.album
                || track.genre != original.genre
            {
                track.is_modified = true;
            }
        }
    }

    fn appliquer_exceptions(
        &self,
        track: &mut Track,
        exceptions: &HashMap<(String, String), String>,
    ) {
        // Artist
        if let Some(corr) = exceptions.get(&("global".to_string(), track.artist.to_lowercase())) {
            track.artist = corr.clone();
        }
        if let Some(corr) = exceptions.get(&("artist".to_string(), track.artist.to_lowercase())) {
            track.artist = corr.clone();
        }

        // Album
        if let Some(corr) = exceptions.get(&("global".to_string(), track.album.to_lowercase())) {
            track.album = corr.clone();
        }
        if let Some(corr) = exceptions.get(&("album".to_string(), track.album.to_lowercase())) {
            track.album = corr.clone();
        }

        // Title
        if let Some(corr) = exceptions.get(&("global".to_string(), track.title.to_lowercase())) {
            track.title = corr.clone();
        }
    }

    fn nettoyer_chaine(&self, input: &str) -> String {
        let mut cleaned = input.to_string();

        // 1. Supprimer les patterns indésirables (simplifié pour l'instant, à affiner)
        // cleaned = RE_GARBAGE.replace_all(&cleaned, "").to_string();

        // 2. Normaliser les connecteurs
        cleaned = RE_FEAT.replace_all(&cleaned, " feat. ").to_string();
        cleaned = RE_AND.replace_all(&cleaned, " & ").to_string();

        // 3. Nettoyer les espaces (trim + collapse)
        cleaned = RE_SPACES.replace_all(&cleaned, " ").to_string();
        cleaned = cleaned.trim().to_string();

        cleaned
    }

    /// Applique le "Sentence Case" avec exceptions (Chiffres romains, I, Abréviations)
    pub fn corriger_casse(&self, texte: &str) -> String {
        if texte.trim().is_empty() {
            return String::new();
        }

        let words: Vec<&str> = texte.split_whitespace().collect();
        let mut processed_words = Vec::new();

        for (i, word) in words.iter().enumerate() {
            // Nettoyage pour vérification (retirer ponctuation)
            let clean_word: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
            let upper_clean = clean_word.to_uppercase();

            let mut new_word = word.to_lowercase();

            // 1. Exceptions Globales (Toujours Majuscules)
            if ROMAN_NUMERALS.contains(upper_clean.as_str()) {
                // On remplace la partie alphanumérique par sa version majuscule
                if !clean_word.is_empty() {
                    new_word = word.replace(&clean_word.to_lowercase(), &upper_clean);
                }
            } else if ABBREVIATIONS.contains(upper_clean.as_str()) {
                if !clean_word.is_empty() {
                    new_word = word.replace(&clean_word.to_lowercase(), &upper_clean);
                }
            } else if clean_word.to_lowercase() == "i" {
                // "i" isolé -> "I" (ex: "I love", "am I")
                new_word = word.replace("i", "I");
            } else {
                // 2. Logique Sentence Case
                let mut capitalize = false;

                if i == 0 {
                    capitalize = true;
                } else {
                    // Vérifier le mot précédent pour la ponctuation forte
                    let prev_word = words[i - 1];
                    if let Some(last_char) = prev_word.chars().last() {
                        if ".:?!-".contains(last_char) {
                            capitalize = true;
                        }
                    }
                    // Cas spécifique: "Title: Subtitle" -> le ":" est souvent collé au mot précédent
                }

                // Si le mot lui-même commence par une parenthèse, on doit capitaliser la lettre qui suit
                if capitalize {
                    new_word = self.capitalize_smart(&new_word);
                }
            }
            processed_words.push(new_word);
        }

        processed_words.join(" ")
    }

    /// Capitalise la première lettre alphanumérique d'un mot
    fn capitalize_smart(&self, word: &str) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        for c in chars.iter_mut() {
            if c.is_alphabetic() {
                *c = c.to_uppercase().next().unwrap();
                break; // On ne capitalise que la première lettre trouvée
            }
        }
        chars.into_iter().collect()
    }

    pub fn format_track_number(&self, track: &str) -> String {
        // Padding sur 2 chiffres (1 -> 01)
        if let Ok(num) = track.parse::<u32>() {
            format!("{:02}", num)
        } else {
            track.to_string()
        }
    }

    pub fn normalize_genre(&self, genre: &str) -> String {
        // Genre toujours en Title Case (Chaque Mot Majuscule)
        genre
            .split_whitespace()
            .map(|w| self.capitalize_smart(w))
            .collect::<Vec<String>>()
            .join(" ")
    }
}
