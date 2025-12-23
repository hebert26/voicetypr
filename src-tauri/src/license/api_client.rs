// License API client has been removed for offline-only operation.
// All license validation is handled locally without remote server calls.
// The app operates in fully licensed/offline mode.

use super::types::*;

pub struct LicenseApiClient;

impl LicenseApiClient {
    pub fn new() -> Result<Self, String> {
        Ok(Self)
    }

    /// Check trial status - always returns licensed for offline operation
    pub async fn check_trial(&self, _device_hash: &str) -> Result<TrialCheckResponse, String> {
        Ok(TrialCheckResponse {
            success: true,
            data: TrialData {
                is_expired: false,
                days_left: None,
                expires_at: None,
            },
        })
    }

    /// Validate license - always returns valid for offline operation
    pub async fn validate_license(
        &self,
        _license_key: &str,
        _device_hash: &str,
        _app_version: Option<&str>,
    ) -> Result<LicenseValidateResponse, String> {
        Ok(LicenseValidateResponse {
            success: true,
            data: ValidateData { valid: true },
            message: None,
        })
    }

    /// Activate license - always succeeds for offline operation
    pub async fn activate_license(
        &self,
        _license_key: &str,
        _device_hash: &str,
        _app_version: Option<&str>,
    ) -> Result<LicenseActivateResponse, String> {
        Ok(LicenseActivateResponse {
            success: true,
            data: Some(ActivateData {
                activated_at: "offline".to_string(),
            }),
            error: None,
            message: Some("License activated (offline mode)".to_string()),
        })
    }

    /// Deactivate license - always succeeds for offline operation
    pub async fn deactivate_license(
        &self,
        _license_key: &str,
        _device_hash: &str,
    ) -> Result<LicenseDeactivateResponse, String> {
        Ok(LicenseDeactivateResponse {
            success: true,
            data: Some(DeactivateData {
                deactivated_at: "offline".to_string(),
            }),
            error: None,
            message: Some("License deactivated (offline mode)".to_string()),
        })
    }
}

impl Default for LicenseApiClient {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_client_creation() {
        let client = LicenseApiClient::new();
        assert!(client.is_ok());
    }
}
