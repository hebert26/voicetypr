// Device ID command - stubbed for privacy (no hardware fingerprinting)

/// Returns a static device identifier (no hardware fingerprinting)
#[tauri::command]
pub async fn get_device_id() -> Result<String, String> {
    Ok("offline-mode".to_string())
}
