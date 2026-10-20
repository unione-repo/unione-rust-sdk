//! Low level transport: every API method is a POST request with a JSON body that returns JSON.

mod configuration;
mod error;

pub use configuration::*;
pub use error::ApiError;

use serde::de::DeserializeOwned;
use serde_json::Value;

/// The SDK and its version in the "platform" query parameter of every request.
pub const PLATFORM: &str = "rust-sdk.1.1.3";

/// Sends API requests.
#[derive(Debug, Clone)]
pub struct ApiClient {
    config: Configuration,
    http: reqwest::Client,
}

impl ApiClient {
    /// Creates the transport.
    pub fn new(config: Configuration) -> Self {
        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .unwrap_or_default();
        ApiClient { config, http }
    }

    /// Creates the transport with a custom reqwest client (proxy, TLS, connection pool settings);
    /// `Configuration::timeout` still applies to every request.
    pub fn with_http_client(config: Configuration, http: reqwest::Client) -> Self {
        ApiClient { config, http }
    }

    /// Calls an API method, e.g. "/email/send.json", and decodes the response.
    pub async fn call<R: DeserializeOwned>(&self, path: &str, body: Value) -> Result<R, ApiError> {
        let url = format!(
            "{}/{}?platform={}",
            self.config.base_url.trim_end_matches('/'),
            path.trim_start_matches('/'),
            match self.config.platform.as_deref() {
                Some(platform) if !platform.is_empty() => format!("{platform},{PLATFORM}"),
                _ => PLATFORM.to_string(),
            }
        );
        let body = if body.is_null() {
            Value::Object(Default::default())
        } else {
            body
        };
        let response = self
            .http
            .post(url)
            .timeout(self.config.timeout)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("User-Agent", &self.config.user_agent)
            .header("X-API-KEY", &self.config.api_key)
            .body(serde_json::to_string(&body)?)
            .send()
            .await
            .map_err(|e| ApiError::new(format!("HTTP request failed: {e}")).with_source(e))?;

        let status = response.status().as_u16();
        let raw = response.text().await.map_err(|e| {
            ApiError {
                http_status: status,
                ..ApiError::new(format!("can not read the response: {e}"))
            }
            .with_source(e)
        })?;
        let data: Value = serde_json::from_str(&raw).map_err(|e| {
            ApiError {
                http_status: status,
                body: Some(raw.clone()),
                ..ApiError::new(format!("invalid JSON in response: {e}"))
            }
            .with_source(e)
        })?;
        if !data.is_object() {
            return Err(ApiError {
                http_status: status,
                body: Some(raw),
                ..ApiError::new("unexpected response")
            });
        }
        if status >= 400 || data.get("status").and_then(Value::as_str) == Some("error") {
            return Err(ApiError {
                message: data
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("API error")
                    .to_string(),
                http_status: status,
                code: data.get("code").and_then(Value::as_i64),
                body: Some(raw),
                source: None,
            });
        }
        serde_json::from_value(data).map_err(|e| {
            ApiError {
                http_status: status,
                body: Some(raw),
                ..ApiError::new(format!("can not decode the response: {e}"))
            }
            .with_source(e)
        })
    }
}
