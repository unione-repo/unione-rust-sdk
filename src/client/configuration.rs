//! API connection settings.

use std::time::Duration;

/// Base URL of the API.
pub const DEFAULT_BASE_URL: &str = "https://api.unione.io/en/transactional/api/v1";

/// API host api.unione.io.
pub const HOST_API: &str = "api.unione.io";

/// API host eu1.unione.io.
pub const HOST_EU1: &str = "eu1.unione.io";

/// API host us1.unione.io.
pub const HOST_US1: &str = "us1.unione.io";

/// Path of the API on every host.
pub const BASE_PATH: &str = "/en/transactional/api/v1";

/// Base URL for an API host: `eu1.unione.io` -> `https://eu1.unione.io` + `BASE_PATH`; a full URL is kept as is.
pub fn base_url_for(host: &str) -> String {
    if host.contains("://") {
        host.trim_end_matches('/').to_string()
    } else {
        format!("https://{}{}", host.trim_matches('/'), BASE_PATH)
    }
}

/// API connection settings.
#[derive(Debug, Clone)]
pub struct Configuration {
    /// User or project API key, sent in the X-API-KEY header.
    pub api_key: String,
    /// Base URL of the API.
    pub base_url: String,
    /// Request timeout.
    pub timeout: Duration,
    /// User-Agent header value.
    pub user_agent: String,
    /// Package that uses the SDK and its version, e.g. "axum.1.0.0": goes to the "platform" query parameter before the SDK one
    /// (`PLATFORM`).
    pub platform: Option<String>,
}

impl Configuration {
    /// Default configuration for the API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Configuration {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: Duration::from_secs(30),
            user_agent: "unione-rust-sdk/1.1.2".to_string(),
            platform: None,
        }
    }
}
