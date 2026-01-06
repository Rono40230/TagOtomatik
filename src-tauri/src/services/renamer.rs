use sanitize_filename;

pub struct RenamerService;

impl Default for RenamerService {
    fn default() -> Self {
        Self::new()
    }
}

impl RenamerService {
    pub fn new() -> Self {
        Self
    }

    pub fn format_folder_name(
        &self,
        artist: &str,
        album: &str,
        year_min: Option<u32>,
        year_max: Option<u32>,
    ) -> String {
        let year_str = match (year_min, year_max) {
            (Some(min), Some(max)) => {
                if min == max {
                    format!("({}) ", min)
                } else {
                    let max_short = max % 100;
                    format!("({}-{:02}) ", min, max_short)
                }
            }
            (Some(y), None) | (None, Some(y)) => format!("({}) ", y),
            _ => String::new(),
        };

        let formatted_title = self.apply_sentence_case_with_exception(album, artist);

        // Sanitize filename (remove illegal chars)
        let safe_title = self.sanitize_path_component(&formatted_title);
        let safe_year = self.sanitize_path_component(&year_str);

        format!("{}{}", safe_year, safe_title).trim().to_string()
    }

    pub fn format_track_filename(
        &self,
        track_number: Option<u32>,
        title: &str,
        extension: &str,
    ) -> String {
        let num_str = match track_number {
            Some(n) => format!("{:02}", n),
            None => "00".to_string(),
        };

        let safe_title = self.sanitize_path_component(title);
        let clean_title = safe_title.trim();

        // Remove leading dot if extension has it
        let ext = extension.trim_start_matches('.');

        // Format: "01 - Title.mp3" (Legacy Python style)
        format!("{} - {}.{}", num_str, clean_title, ext)
    }

    fn sanitize_path_component(&self, name: &str) -> String {
        // Custom replacements before sanitization
        let pre_sanitized = name
            .replace(['/', '\\', '|', ':'], "-")
            .replace('<', "(")
            .replace('>', ")")
            .replace('"', "'");

        sanitize_filename::sanitize(pre_sanitized)
    }

    fn apply_sentence_case_with_exception(&self, text: &str, exception: &str) -> String {
        if text.is_empty() {
            return String::new();
        }

        // 1. Sentence case: First letter uppercase, rest lowercase
        let mut chars = text.chars();
        let first_char = chars.next().unwrap_or_default().to_uppercase().to_string();
        let rest = chars.as_str().to_lowercase();
        let mut sentence_cased = format!("{}{}", first_char, rest);

        // 2. Exception: Restore artist name casing if present
        if !exception.is_empty() {
            // Case insensitive search
            let lower_text = sentence_cased.to_lowercase();
            let lower_exception = exception.to_lowercase();

            if let Some(start_idx) = lower_text.find(&lower_exception) {
                let end_idx = start_idx + lower_exception.len();

                // Reconstruct string with original exception casing
                let before = &sentence_cased[..start_idx];
                let after = &sentence_cased[end_idx..];

                sentence_cased = format!("{}{}{}", before, exception, after);
            }
        }

        sentence_cased
    }
}
