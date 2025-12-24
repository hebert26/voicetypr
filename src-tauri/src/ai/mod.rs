use async_trait::async_trait;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

pub mod config;
pub mod openai;
pub mod prompts;

/// Global HTTP client for AI requests - reused across all requests to avoid
/// connection setup overhead. Configured with shorter timeout for local Ollama.
pub static AI_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(config::LOCAL_TIMEOUT_SECS))
        .pool_idle_timeout(Duration::from_secs(60))
        .pool_max_idle_per_host(2)
        .build()
        .expect("Failed to create AI HTTP client")
});

pub use config::MAX_TEXT_LENGTH;
pub use prompts::EnhancementOptions;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderConfig {
    pub provider: String,
    pub model: String,
    #[serde(skip_serializing)] // Don't serialize API key
    pub api_key: String,
    pub enabled: bool,
    #[serde(default)]
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIEnhancementRequest {
    pub text: String,
    pub context: Option<String>,
    #[serde(default)]
    pub options: Option<EnhancementOptions>,
}

impl AIEnhancementRequest {
    pub fn validate(&self) -> Result<(), AIError> {
        if self.text.trim().is_empty() {
            return Err(AIError::ValidationError("Text cannot be empty".to_string()));
        }

        if self.text.len() > MAX_TEXT_LENGTH {
            return Err(AIError::ValidationError(format!(
                "Text exceeds maximum length of {} characters",
                MAX_TEXT_LENGTH
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIEnhancementResponse {
    pub enhanced_text: String,
    pub original_text: String,
    pub provider: String,
    pub model: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AIError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Provider not found: {0}")]
    ProviderNotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn enhance_text(
        &self,
        request: AIEnhancementRequest,
    ) -> Result<AIEnhancementResponse, AIError>;

    fn name(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModel {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

pub struct AIProviderFactory;

impl AIProviderFactory {
    pub fn create(config: &AIProviderConfig) -> Result<Box<dyn AIProvider>, AIError> {
        // Validate provider name - only local OpenAI-compatible (Ollama) is supported
        if !Self::is_valid_provider(&config.provider) {
            return Err(AIError::ProviderNotFound(config.provider.clone()));
        }

        match config.provider.as_str() {
            // Only OpenAI-compatible local providers (Ollama) are supported
            "openai" => Ok(Box::new(openai::OpenAIProvider::new(
                config.api_key.clone(),
                config.model.clone(),
                config.options.clone(),
            )?)),
            provider => Err(AIError::ProviderNotFound(provider.to_string())),
        }
    }

    fn is_valid_provider(provider: &str) -> bool {
        // Only local OpenAI-compatible provider (used by Ollama) is supported
        matches!(provider, "openai")
    }
}
