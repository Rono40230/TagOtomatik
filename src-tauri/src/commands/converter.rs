use crate::services::converter::ConverterService;
use std::path::Path;

#[tauri::command]
pub async fn convert_file(input_path: String, bitrate: String) -> Result<String, String> {
    let service = ConverterService::new();

    let input = Path::new(&input_path);
    if !input.exists() {
        return Err("Input file does not exist".to_string());
    }

    let output_path = input.with_extension("mp3");
    let output_str = output_path.to_str().ok_or("Invalid path")?.to_string();
    let output_str_clone = output_str.clone();

    // Run in blocking thread to avoid blocking async runtime
    tauri::async_runtime::spawn_blocking(move || {
        service.convert_to_mp3(&input_path, &output_str_clone, &bitrate)
    })
    .await
    .map_err(|e| e.to_string())??;

    Ok(output_str)
}

#[tauri::command]
pub async fn delete_file(path: String) -> Result<(), String> {
    std::fs::remove_file(path).map_err(|e| e.to_string())
}
