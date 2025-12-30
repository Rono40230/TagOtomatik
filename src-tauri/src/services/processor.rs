use crate::models::Track;
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

    static ref PARTICULES: std::collections::HashSet<&'static str> = {
        let mut s = std::collections::HashSet::new();
        let list = vec![
            "a", "an", "the", "and", "but", "or", "nor", "at", "by", "for", "from", "in", "into", "of", "off", "on", "onto", "out", "over", "up", "with", "to", "as",
            "le", "la", "les", "un", "une", "des", "du", "de", "et", "ou", "mais", "ni", "car", "dans", "par", "pour", "en", "vers", "avec", "sans", "sous", "sur"
        ];
        for p in list { s.insert(p); }
        s
    };
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

        track.title = self.corriger_casse(&track.title);
        track.artist = self.corriger_casse(&track.artist);
        track.album = self.corriger_casse(&track.album);

        self.appliquer_exceptions(track, exceptions);

        // Marquer comme modifié si différent de l'original
        if let Some(original) = &track.original_metadata {
            if track.title != original.title
                || track.artist != original.artist
                || track.album != original.album
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

    fn corriger_casse(&self, input: &str) -> String {
        let words: Vec<&str> = input.split_whitespace().collect();
        let mut result = String::new();

        for (i, word) in words.iter().enumerate() {
            let lower_word = word.to_lowercase();
            let is_first_last = i == 0 || i == words.len() - 1;

            let corrected = if !is_first_last && PARTICULES.contains(lower_word.as_str()) {
                lower_word
            } else {
                // Capitalize first letter
                let mut chars = lower_word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                }
            };

            if i > 0 {
                result.push(' ');
            }
            result.push_str(&corrected);
        }

        if result.is_empty() {
            input.to_string()
        } else {
            result
        }
    }
}
