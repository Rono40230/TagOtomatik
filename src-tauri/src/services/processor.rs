use crate::models::Track;
use crate::services::dictionaries::{ABBREVIATIONS, ROMAN_NUMERALS};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // Regex pour nettoyer les espaces multiples
    static ref RE_SPACES: Regex = Regex::new(r"\s+").unwrap();

    // Regex pour les patterns indésirables (ex: [320kbps], www.site.com)
    static ref RE_GARBAGE: Regex = Regex::new(r"(?i)(\[.*?\]|\(.*?\)|\bwww\..*?\.[a-z]{2,4}\b)").unwrap();

    // Regex pour supprimer le contenu entre parenthèses
    static ref RE_PARENTHESES: Regex = Regex::new(r"\(.*?\)").unwrap();

    // Regex pour normaliser les connecteurs
    // Modif utilisateur: on ne touche plus à "with"
    static ref RE_FEAT: Regex = Regex::new(r"(?i)\s+(feat\.?|ft\.?)\s+").unwrap();
    static ref RE_AND: Regex = Regex::new(r"(?i)\s+(and|et)\s+").unwrap();

    // Regex pour supprimer le préfixe de numéro de piste (ex: "01 - ", "1. ", "1 ")
    static ref RE_TRACK_PREFIX: Regex = Regex::new(r"^\d+[\.\-\s]+\s*").unwrap();
}

pub struct ReplacementRule {
    pub category: String,
    pub regex: Regex,
    pub replacement: String,
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

    pub fn apply_rules_to_string(
        &self,
        text: &str,
        category: &str,
        rules: &Vec<ReplacementRule>,
    ) -> String {
        let mut result = text.to_string();
        for rule in rules {
            if rule.category == "global" || rule.category == category {
                result = rule
                    .regex
                    .replace_all(&result, rule.replacement.as_str())
                    .to_string();
            }
        }
        result
    }

    pub fn nettoyer_filename(
        &self,
        filename: &str,
        track_number: Option<u32>,
        rules: &Vec<ReplacementRule>,
    ) -> String {
        let path = std::path::Path::new(filename);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(filename);
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        // 1. Supprimer le préfixe existant (numéro de piste)
        let stem_no_prefix = RE_TRACK_PREFIX.replace(stem, "");

        // 2. Nettoyage de base (Regex) sur le reste
        let mut cleaned_stem = self.nettoyer_chaine(&stem_no_prefix);

        // 3. Suppression des parenthèses
        cleaned_stem = RE_PARENTHESES.replace_all(&cleaned_stem, "").to_string();

        // 4. Re-nettoyage espaces
        cleaned_stem = RE_SPACES.replace_all(&cleaned_stem, " ").trim().to_string();

        // 5. Correction de la casse
        cleaned_stem = self.corriger_casse(&cleaned_stem);

        // 6. Appliquer les exceptions (Global + Title car le filename est souvent le titre)
        cleaned_stem = self.apply_rules_to_string(&cleaned_stem, "title", rules);

        // 7. Ajouter le préfixe standardisé si un numéro est fourni
        if let Some(n) = track_number {
            cleaned_stem = format!("{:02} - {}", n, cleaned_stem);
        }

        if !ext.is_empty() {
            format!("{}.{}", cleaned_stem, ext)
        } else {
            cleaned_stem
        }
    }

    pub fn nettoyer_track(&self, track: &mut Track, rules: &Vec<ReplacementRule>) {
        track.title = self.nettoyer_chaine(&track.title);
        track.artist = self.nettoyer_chaine(&track.artist);
        track.album = self.nettoyer_chaine(&track.album);

        // Suppression des parenthèses pour Titre et Album
        track.title = RE_PARENTHESES.replace_all(&track.title, "").to_string();
        track.album = RE_PARENTHESES.replace_all(&track.album, "").to_string();

        // Re-nettoyage des espaces après suppression
        track.title = RE_SPACES.replace_all(&track.title, " ").trim().to_string();
        track.album = RE_SPACES.replace_all(&track.album, " ").trim().to_string();

        // Genre cleaning
        if let Some(g) = &track.genre {
            let clean_g = self.nettoyer_chaine(g);
            // Handle ID3v1 numeric genres (simple check)
            let normalized = if clean_g.starts_with('(') && clean_g.ends_with(')') {
                // Try to parse number inside
                let inner = &clean_g[1..clean_g.len() - 1];
                if inner.parse::<u32>().is_ok() {
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

        self.appliquer_exceptions(track, rules);

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

    pub fn nettoyer_album_metadata(&self, track: &mut Track, rules: &Vec<ReplacementRule>) {
        // 1. Nettoyage de base
        track.album = self.nettoyer_chaine(&track.album);

        // 2. Suppression des parenthèses
        track.album = RE_PARENTHESES.replace_all(&track.album, "").to_string();

        // 3. Re-nettoyage espaces
        track.album = RE_SPACES.replace_all(&track.album, " ").trim().to_string();

        // 4. Correction de la casse
        track.album = self.corriger_casse(&track.album);

        // 5. Exceptions
        track.album = self.apply_rules_to_string(&track.album, "album", rules);
    }

    pub fn appliquer_exceptions(&self, track: &mut Track, rules: &Vec<ReplacementRule>) {
        track.artist = self.apply_rules_to_string(&track.artist, "artist", rules);
        track.album = self.apply_rules_to_string(&track.album, "album", rules);
        track.title = self.apply_rules_to_string(&track.title, "title", rules);
    }

    fn nettoyer_chaine(&self, input: &str) -> String {
        let mut cleaned = input.to_string();

        // 1. Supprimer les patterns indésirables (simplifié pour l'instant, à affiner)
        // cleaned = RE_GARBAGE.replace_all(&cleaned, "").to_string();

        // 2. Normaliser les connecteurs
        cleaned = RE_FEAT.replace_all(&cleaned, " feat. ").to_string();
        // cleaned = RE_AND.replace_all(&cleaned, " & ").to_string();

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
