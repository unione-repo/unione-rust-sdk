//! API error.

use std::error::Error;
use std::fmt;
use std::sync::Arc;

/// Returned when the API responds with an error or the response can not be processed.
#[derive(Debug, Clone)]
pub struct ApiError {
    /// Error message.
    pub message: String,
    /// HTTP status of the response, 0 when the request was not sent.
    pub http_status: u16,
    /// API error code (the "code" field of the error response).
    pub code: Option<i64>,
    /// Raw response body.
    pub body: Option<String>,
    /// Underlying error (a `reqwest::Error` of the transport, a `serde_json::Error`), also returned by
    /// `std::error::Error::source()`.
    pub source: Option<Arc<dyn Error + Send + Sync>>,
}

impl ApiError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        ApiError {
            message: message.into(),
            http_status: 0,
            code: None,
            body: None,
            source: None,
        }
    }

    pub(crate) fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.source = Some(Arc::new(source));
        self
    }

    /// The transport (reqwest) error of a request that failed before an API response, if any.
    pub fn transport_error(&self) -> Option<&reqwest::Error> {
        self.source.as_deref()?.downcast_ref::<reqwest::Error>()
    }

    /// The request timed out (`Configuration::timeout` or the timeout of a custom reqwest client).
    pub fn is_timeout(&self) -> bool {
        self.transport_error()
            .is_some_and(reqwest::Error::is_timeout)
    }

    /// The connection to the API host failed.
    pub fn is_connect(&self) -> bool {
        self.transport_error()
            .is_some_and(reqwest::Error::is_connect)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(
                f,
                "{} (HTTP {}, code {})",
                self.message, self.http_status, code
            ),
            None => write!(f, "{} (HTTP {})", self.message, self.http_status),
        }
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|e| e as &(dyn Error + 'static))
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        ApiError::new(format!("JSON error: {e}")).with_source(e)
    }
}
