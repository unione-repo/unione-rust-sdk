//! Models of the Obsolete section.

use serde::{Deserialize, Serialize};

/// Unsubscription process details:
///
/// - "unsubscribed" if email has just been unsubscribed,
/// - "was inactive" if it was inactive due some reason,
/// - "was unsubscribed" if it already was unsubscribed before the call,
/// - "added and unsubscribed" if it's a new email without status history and it was added and then unsubscribed immediately.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ObsoleteUnsubscribedSetMessage {
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "was inactive")]
    WasInactive,
    #[serde(rename = "was unsubscribed")]
    WasUnsubscribed,
    #[serde(rename = "added and unsubscribed")]
    AddedAndUnsubscribed,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// ObsoleteUnsubscribedSetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedSetRequest {
    /// Email to be unsubscribed.
    #[serde(rename = "address", default)]
    pub address: String,
}

/// ObsoleteUnsubscribedSetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedSetResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The unsubscribed email.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Unsubscription process details:
    ///
    /// - "unsubscribed" if email has just been unsubscribed,
    /// - "was inactive" if it was inactive due some reason,
    /// - "was unsubscribed" if it already was unsubscribed before the call,
    /// - "added and unsubscribed" if it's a new email without status history and it was added and then unsubscribed immediately.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<ObsoleteUnsubscribedSetMessage>,
}

/// ObsoleteUnsubscribedCheckRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedCheckRequest {
    /// Email to check.
    #[serde(rename = "address", default)]
    pub address: String,
}

/// ObsoleteUnsubscribedCheckResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedCheckResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Email that was checked.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// true if unsubscribed, false if not.
    #[serde(
        rename = "is_unsubscribed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_unsubscribed: Option<bool>,
}

/// ObsoleteUnsubscribedListRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedListRequest {
    /// Date in the format YYYY-MM-DD to get all unsubscribed emails from the "date_from" to the present
    /// day. If the parameter is not specified, the default period is today (from 00:00 UTC).
    #[serde(rename = "date_from", default, skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
}

/// ObsoleteUnsubscribedListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedListResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Array of object describing unsubscribed emails.
    #[serde(
        rename = "unsubscribed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribed: Option<Vec<ObsoleteUnsubscribedListResponseUnsubscribedItem>>,
}

/// ObsoleteUnsubscribedListResponseUnsubscribedItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedListResponseUnsubscribedItem {
    /// Unsubscribed email.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Date and time when a recipient unsubscribed, in UTC timezone and "YYYY-MM-DD hh:mm:ss" format.
    #[serde(
        rename = "unsubscribed_on",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribed_on: Option<String>,
}
