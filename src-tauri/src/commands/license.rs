// License commands - stubbed for fully offline/private operation
// All functions return "Licensed" status immediately without any network calls

use crate::license::{LicenseState, LicenseStatus};
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::time::Instant;
use tauri::AppHandle;

/// Cached license status (kept for compatibility but not really used)
#[derive(Clone, Debug)]
pub struct CachedLicense {
    pub status: LicenseStatus,
    cached_at: Instant,
}

impl CachedLicense {
    pub fn new(status: LicenseStatus) -> Self {
        Self {
            status,
            cached_at: Instant::now(),
        }
    }

    pub fn is_valid(&self) -> bool {
        true // Always valid in offline mode
    }

    pub fn age(&self) -> std::time::Duration {
        self.cached_at.elapsed()
    }
}

impl UnwindSafe for CachedLicense {}
impl RefUnwindSafe for CachedLicense {}

/// Helper to create a licensed status
fn licensed_status() -> LicenseStatus {
    LicenseStatus {
        status: LicenseState::Licensed,
        trial_days_left: None,
        license_type: Some("offline".to_string()),
        license_key: None,
        expires_at: None,
    }
}

/// Check license status - always returns Licensed (offline mode)
#[tauri::command]
pub async fn check_license_status(_app: AppHandle) -> Result<LicenseStatus, String> {
    log::debug!("License check: returning Licensed (offline mode)");
    Ok(licensed_status())
}

/// Internal version for other commands
pub async fn check_license_status_internal(_app: &AppHandle) -> Result<LicenseStatus, String> {
    Ok(licensed_status())
}

/// Restore license - returns Licensed (offline mode)
#[tauri::command]
pub async fn restore_license(_app: AppHandle) -> Result<LicenseStatus, String> {
    log::debug!("Restore license: returning Licensed (offline mode)");
    Ok(licensed_status())
}

/// Activate license - returns Licensed (offline mode)
#[tauri::command]
pub async fn activate_license(
    _license_key: String,
    _app: AppHandle,
) -> Result<LicenseStatus, String> {
    log::debug!("Activate license: returning Licensed (offline mode)");
    Ok(licensed_status())
}

/// Deactivate license - no-op in offline mode
#[tauri::command]
pub async fn deactivate_license(_app: AppHandle) -> Result<(), String> {
    log::debug!("Deactivate license: no-op (offline mode)");
    Ok(())
}

/// Open purchase page - no-op in offline mode
#[tauri::command]
pub async fn open_purchase_page() -> Result<(), String> {
    log::debug!("Open purchase page: no-op (offline mode)");
    Ok(())
}

/// Invalidate cache - no-op in offline mode
#[tauri::command]
pub async fn invalidate_license_cache(_app: AppHandle) -> Result<(), String> {
    log::debug!("Invalidate cache: no-op (offline mode)");
    Ok(())
}
