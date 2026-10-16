//! Models of the ConfirmedEmail section.

use serde::{Deserialize, Serialize};

/// ConfirmedEmailRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfirmedEmailRequest {
    /// Email address to add or delete.
    #[serde(rename = "email", default)]
    pub email: String,
}

/// ConfirmedEmailCreateResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfirmedEmailCreateResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(
        rename = "confirmed_email",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub confirmed_email: Option<ConfirmedEmail>,
}

/// ConfirmedEmail
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfirmedEmail {
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Confirmation status of the email address.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::common::model::VerificationRecordStatus>,
}

/// ConfirmedEmailListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfirmedEmailListResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(
        rename = "confirmed_emails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub confirmed_emails: Option<Vec<ConfirmedEmail>>,
}

/// ConfirmedEmailDeleteResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfirmedEmailDeleteResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
