use std::process::Command;

pub struct ConverterService;

impl Default for ConverterService {
    fn default() -> Self {
        Self::new()
    }
}

impl ConverterService {
    pub fn new() -> Self {
        Self
    }

    pub fn convert_to_mp3(
        &self,
        input_path: &str,
        output_path: &str,
        bitrate: &str,
    ) -> Result<(), String> {
        // Check if ffmpeg is installed
        // For now, assume it is in PATH

        let status = Command::new("ffmpeg")
            .arg("-i")
            .arg(input_path)
            .arg("-codec:a")
            .arg("libmp3lame")
            .arg("-b:a")
            .arg(bitrate)
            .arg("-y") // Overwrite
            .arg(output_path)
            .status()
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

        if status.success() {
            Ok(())
        } else {
            Err("FFmpeg conversion failed".to_string())
        }
    }
}
