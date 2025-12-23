// Remote cloud STT services have been removed for offline-only operation.
// All transcription is handled locally via Whisper or Parakeet models.

use tauri::AppHandle;

#[tauri::command]
#[allow(non_snake_case)]
pub async fn validate_and_cache_soniox_key(
    _api_key: Option<String>,
    _apiKey: Option<String>,
) -> Result<(), String> {
    // Cloud STT services are disabled - app operates fully offline
    Err("Cloud transcription services are disabled. Use local Whisper or Parakeet models instead.".to_string())
}

#[tauri::command]
pub async fn clear_soniox_key_cache(_app: AppHandle) -> Result<(), String> {
    Ok(())
}
