//! Models of the System section.

use serde::{Deserialize, Serialize};

/// SystemPingResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemPingResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Unique user identifier.
    #[serde(rename = "user_id", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// SystemInfoResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemInfoResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Unique user identifier.
    #[serde(rename = "user_id", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Email of the user.
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Unqiue project identifier, ASCII string up to 36 characters long. Present only if the API key used
    /// for request is the project API key.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Project name, unique for user account. Present only if the API key used for request is the project
    /// API key.
    #[serde(
        rename = "project_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_name: Option<String>,
    /// Object with email counter properties. Present only if the API key used for request is the user API
    /// key.
    #[serde(
        rename = "project_accounting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_accounting: Option<ProjectAccountingObject>,
    /// Object with accounting period properties. Present only if the API key used for request is the user
    /// API key.
    #[serde(
        rename = "accounting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub accounting: Option<SystemInfoResponseAccounting>,
}

/// Object with email counter properties. Present only if the API key used for request is the user API
/// key.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectAccountingObject {
    /// Project's email counter value. It increases on every email sent.
    #[serde(
        rename = "email_counter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter: Option<i64>,
    /// Allows you to limit the number of emails sent by the project. The sending stops when the limit is
    /// reached. Any non-negative 64-bit integer is acceptable. 0 means no limit.
    #[serde(
        rename = "email_counter_limit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter_limit: Option<i64>,
    /// 'default' - the counter resets to zero automatically at the beginning of each accounting period.
    /// 'permanent' - the counter never resets automatically.
    /// Default: `default`.
    #[serde(
        rename = "email_counter_mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter_mode: Option<crate::common::model::EmailCounterMode>,
}

/// Object with accounting period properties. Present only if the API key used for request is the user
/// API key.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemInfoResponseAccounting {
    /// Date and time of accounting period start in UTC in "YYYY-MM-DD hh:mm:ss" format.
    #[serde(
        rename = "period_start",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub period_start: Option<String>,
    /// Date and time of accounting period end in UTC in "YYYY-MM-DD hh:mm:ss" format.
    #[serde(
        rename = "period_end",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub period_end: Option<String>,
    /// Number of emails included into accounting period.
    #[serde(
        rename = "emails_included",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub emails_included: Option<i64>,
    /// Number of emails sent during accounting period. Can be bigger than emails_included in case of
    /// overage.
    #[serde(
        rename = "emails_sent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub emails_sent: Option<i64>,
}
