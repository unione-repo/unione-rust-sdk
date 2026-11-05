//! API error.

use std::error::Error;
use std::fmt;
use std::sync::Arc;

/// What failed, see `ApiError::kind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The API returned an error (`ApiError::code`, `ApiError::hint`).
    Api,
    /// The request could not be sent or the response could not be received.
    Network,
    /// The request timed out.
    Timeout,
    /// The response is not the JSON of the API (a proxy page, a broken body).
    InvalidResponse,
    /// Found before sending (an empty API key, a required field missing): no request was sent.
    InvalidRequest,
}

impl ErrorKind {
    /// Name of the kind: "api", "network", "timeout", "invalid_response", "invalid_request".
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorKind::Api => "api",
            ErrorKind::Network => "network",
            ErrorKind::Timeout => "timeout",
            ErrorKind::InvalidResponse => "invalid_response",
            ErrorKind::InvalidRequest => "invalid_request",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Returned when the API responds with an error, the request fails or is invalid, or the response
/// can not be processed. The message names the API method and, for an API error, the code, the HTTP
/// status and how to fix it:
/// "email/send: Unknown recipient (API error 204, HTTP 400). Fix: ... See ...#api-errors".
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
    /// API method of the request, e.g. "email/send"; empty when unknown.
    pub method: String,
    /// How to fix the API error (the errors table of the docs), `None` when unknown.
    pub hint: Option<String>,
    /// What failed.
    pub kind: ErrorKind,
    /// Message of the API error response as is (not cut, without the method, code and hint),
    /// `None` when there was no error response of the API.
    pub api_message: Option<String>,
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
            method: String::new(),
            hint: None,
            kind: ErrorKind::Api,
            api_message: None,
            source: None,
        }
    }

    pub(crate) fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.source = Some(Arc::new(source));
        self
    }

    pub(crate) fn with_kind(mut self, kind: ErrorKind) -> Self {
        self.kind = kind;
        self
    }

    pub(crate) fn with_method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }

    /// A repeat of the request may succeed: a network error, a timeout, HTTP 429 or 5xx.
    pub fn retryable(&self) -> bool {
        match self.kind {
            ErrorKind::Network | ErrorKind::Timeout => true,
            ErrorKind::Api | ErrorKind::InvalidResponse => {
                self.http_status == 429 || self.http_status >= 500
            }
            ErrorKind::InvalidRequest => false,
        }
    }

    /// The transport (reqwest) error of a request that failed before an API response, if any.
    pub fn transport_error(&self) -> Option<&reqwest::Error> {
        self.source.as_deref()?.downcast_ref::<reqwest::Error>()
    }

    /// The request timed out (`Configuration::timeout` or the timeout of a custom reqwest client).
    pub fn is_timeout(&self) -> bool {
        self.kind == ErrorKind::Timeout
    }

    /// The connection to the API host failed.
    pub fn is_connect(&self) -> bool {
        self.transport_error()
            .is_some_and(reqwest::Error::is_connect)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|e| e as &(dyn Error + 'static))
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        ApiError::new(format!("JSON error: {e}"))
            .with_kind(ErrorKind::InvalidRequest)
            .with_source(e)
    }
}
