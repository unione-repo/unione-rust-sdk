//! Low level transport: every API method is a POST request with a JSON body that returns JSON.

mod configuration;
mod error;
mod error_info;

pub use configuration::*;
pub use error::{ApiError, ErrorKind};

use serde::de::DeserializeOwned;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

/// The SDK and its version in the "platform" query parameter of every request.
pub const PLATFORM: &str = "rust-sdk.1.3.1";

const EMPTY_KEY: &str =
    "the API key is empty: pass the API key of the user or of the project to the client";

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
    ///
    /// Errors name the method: "email/send: Unknown recipient (API error 204, HTTP 400). Fix: ...".
    /// An empty API key or a missing required field (`message.recipients`) is an
    /// `ErrorKind::InvalidRequest` error: no request is sent then.
    pub async fn call<R: DeserializeOwned>(&self, path: &str, body: Value) -> Result<R, ApiError> {
        let method = error_info::method_of(path);
        if self.config.api_key.is_empty() {
            let message = EMPTY_KEY.to_string();
            return Err(fail(ErrorKind::InvalidRequest, &method, message));
        }
        let body = if body.is_null() {
            Value::Object(Default::default())
        } else {
            body
        };
        if let Some(field) = error_info::missing(&method, &body) {
            let message = format!("{field} is required");
            return Err(fail(ErrorKind::InvalidRequest, &method, message));
        }
        let json = match serde_json::to_string(&body) {
            Ok(json) => json,
            Err(e) => {
                let message = format!("can not encode the request: {e}");
                let error = fail(ErrorKind::InvalidRequest, &method, message);
                return Err(error.with_source(e));
            }
        };
        let url = format!(
            "{}/{}?platform={}",
            self.config.base_url.trim_end_matches('/'),
            path.trim_start_matches('/'),
            match self.config.platform.as_deref() {
                Some(platform) if !platform.is_empty() => format!("{platform},{PLATFORM}"),
                _ => PLATFORM.to_string(),
            }
        );
        let response = self
            .http
            .post(url)
            .timeout(self.config.timeout)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("User-Agent", &self.config.user_agent)
            .header("X-API-KEY", &self.config.api_key)
            .body(json)
            .send()
            .await
            .map_err(|e| self.transport_error(&method, e))?;

        let status = response.status().as_u16();
        let raw = match response.text().await {
            Ok(raw) => raw,
            Err(e) => {
                let error = self.transport_error(&method, e);
                return Err(ApiError {
                    http_status: status,
                    ..error
                });
            }
        };
        let data: Value = match serde_json::from_str(&raw) {
            Ok(data) => data,
            Err(e) => {
                let quoted = error_info::quote(&raw);
                let message = format!("invalid JSON in response (HTTP {status}): {quoted}");
                let error = fail(ErrorKind::InvalidResponse, &method, message);
                return Err(ApiError {
                    http_status: status,
                    body: Some(raw),
                    ..error.with_source(e)
                });
            }
        };
        if !data.is_object() {
            let quoted = error_info::quote(&raw);
            let message = format!("unexpected response (HTTP {status}): {quoted}");
            return Err(ApiError {
                http_status: status,
                body: Some(raw),
                ..fail(ErrorKind::InvalidResponse, &method, message)
            });
        }
        if status >= 400 || data.get("status").and_then(Value::as_str) == Some("error") {
            let code = error_info::code_of(data.get("code"));
            let message = data.get("message").and_then(Value::as_str);
            let text = error_info::api_message(&method, message.unwrap_or(""), status, code);
            let error = ApiError::new(text).with_method(&method);
            return Err(ApiError {
                http_status: status,
                code,
                body: Some(raw),
                hint: error_info::hint(code),
                api_message: message.map(str::to_string),
                ..error
            });
        }
        serde_json::from_value(data).map_err(|e| {
            let message = format!("can not decode the response: {e}");
            let error = fail(ErrorKind::InvalidResponse, &method, message);
            ApiError {
                http_status: status,
                body: Some(raw),
                ..error.with_source(e)
            }
        })
    }

    /// The error of a request that failed: a timeout or a network error.
    fn transport_error(&self, method: &str, e: reqwest::Error) -> ApiError {
        let error = if e.is_timeout() {
            let secs = self.config.timeout.as_secs_f64();
            let message = format!("HTTP request timed out after {secs} s");
            fail(ErrorKind::Timeout, method, message)
        } else {
            let message = format!("HTTP request failed: {e}");
            fail(ErrorKind::Network, method, message)
        };
        error.with_source(e)
    }
}

/// An error of a method: the message starts with the method.
fn fail(kind: ErrorKind, method: &str, message: String) -> ApiError {
    ApiError::new(format!("{method}: {message}"))
        .with_kind(kind)
        .with_method(method)
}

/// Date and time in the format of the API, UTC "YYYY-MM-DD hh:mm:ss", e.g. `options.send_at` of `email/send` in
/// an hour: `utc_datetime(SystemTime::now() + Duration::from_secs(3600))`.
pub fn utc_datetime(time: SystemTime) -> String {
    let secs = time.duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let (days, rest) = (secs / 86_400, secs % 86_400);
    // the civil date of a day number (Howard Hinnant's civil_from_days)
    let z = days + 719_468;
    let era = z / 146_097;
    let doe = z % 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + u64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02} {:02}:{:02}:{:02}",
        rest / 3_600,
        rest / 60 % 60,
        rest % 60
    )
}
