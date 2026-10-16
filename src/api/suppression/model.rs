//! Models of the Suppression section.

use serde::{Deserialize, Serialize};

/// SuppressionSetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionSetRequest {
    /// Email to add in suppression list.
    #[serde(rename = "email", default)]
    pub email: String,
    /// Cause of email being suppressed. One of:
    ///
    /// - __unsubscribed__ - email is unsubscribed;
    /// - __temporary_unavailable__ - the email address is unavailable. This means that over the next three days sending to this address will return an error. Email may be temporarily unavailable due to several reasons, e.g.:
    ///
    /// - a previous email has been rejected by the recipient's server for spam;
    /// - the recipient's mailbox is full or is not used;
    /// - recipient's domain does not accept mail;
    /// - sending server was rejected due to blacklisting;
    ///
    /// - __permanent_unavailable__ - the email address is permanently unavailable due to multiple hard bounces;
    /// - __complained__ -  the recipient reported spam in the previous emails;
    #[serde(rename = "cause", default)]
    pub cause: String,
    /// When suppression was created, in UTC timezone in "YYYY-MM-DD hh:mm:ss" format.
    #[serde(rename = "created", default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

/// SuppressionGetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionGetRequest {
    /// Email to get suppression details for.
    #[serde(rename = "email", default)]
    pub email: String,
    /// If a user has projects functionality enabled, he/she can pass all_projects=true to search in all
    /// projects' data.
    /// Default: `False`.
    #[serde(
        rename = "all_projects",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub all_projects: Option<bool>,
}

/// SuppressionGetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionGetResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The email for which suppression details were requested.
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Array of suppression objects.
    #[serde(
        rename = "suppressions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub suppressions: Option<Vec<SuppressionGetObject>>,
}

/// Object with suppression details.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionGetObject {
    /// Unqiue project identifier, ASCII string up to 36 characters long.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Cause of email being suppressed. One of:
    ///
    /// - __unsubscribed__ - email is unsubscribed;
    /// - __temporary_unavailable__ - the email address is unavailable. This means that over the next three days sending to this address will return an error. Email may be temporarily unavailable due to several reasons, e.g.:
    ///
    /// - a previous email has been rejected by the recipient's server for spam;
    /// - the recipient's mailbox is full or is not used;
    /// - recipient's domain does not accept mail;
    /// - sending server was rejected due to blacklisting;
    ///
    /// - __permanent_unavailable__ - the email address is permanently unavailable due to multiple hard bounces;
    /// - __complained__ -  the recipient reported spam in the previous emails;
    /// - __blocked__ -  sending to the email is prohibited by administration of UniOne.
    ///
    /// We may add some new causes in the future.
    #[serde(rename = "cause", default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// Source of email being suppressed. One of:
    ///
    /// - __user__ - suppressed by user with    [suppression/set](https://docs.unione.io/en/web-api-ref#suppression-set);
    /// - __system__ - sending to the email is prohibited by system, for example due to multiple hard bounces;
    /// - __subscriber__ - the recipient reported spam or unsubscribed in the previous emails.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Is it possible to delete this suppression by calling suppression/delete method.
    #[serde(
        rename = "is_deletable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_deletable: Option<bool>,
    /// When suppression was created, in UTC timezone in "YYYY-MM-DD hh:mm:ss" format.
    #[serde(rename = "created", default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

/// SuppressionListRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionListRequest {
    /// Cause of email being suppressed. One of:
    ///
    /// - __unsubscribed__ - email is unsubscribed;
    /// - __temporary_unavailable__ - the email address is unavailable. This means that over the next three days sending to this address will return an error. Email may be temporarily unavailable due to several reasons, e.g.:
    ///
    /// - a previous email has been rejected by the recipient's server for spam;
    /// - the recipient's mailbox is full or is not used;
    /// - recipient's domain does not accept mail;
    /// - sending server was rejected due to blacklisting;
    ///
    /// - __permanent_unavailable__ - the email address is permanently unavailable due to multiple hard bounces;
    /// - __complained__ -  the recipient reported spam in the previous emails;
    /// - __blocked__ -  sending to the email is prohibited by administration of UniOne.
    ///
    /// We may add some new causes in the future.
    #[serde(rename = "cause", default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// Source of email being suppressed. One of:
    ///
    /// - __user__ - suppressed by user with    [suppression/set](https://docs.unione.io/en/web-api-ref#suppression-set);
    /// - __system__ - sending to the email is prohibited by system, for example due to multiple hard bounces;
    /// - __subscriber__ - the recipient reported spam or unsubscribed in the previous emails.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Date in the format YYYY-MM-DD hh:mm:ss to get suppression list from the "start_time" to the present
    /// day. Ignored if "cursor" is not empty.
    #[serde(
        rename = "start_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_time: Option<String>,
    /// The parameter indicates from which position the selection is to be started. Must be empty or omitted
    /// for the first data chunk. In order to get subsequent chunks, you must set the "cursor" parameter in
    /// your request, using the value received in response to the previous request.
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Limits the number of records to be returned at one time, default is 50.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// SuppressionListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionListResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Array of suppression objects.
    #[serde(
        rename = "suppressions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub suppressions: Option<Vec<SuppressionListObject>>,
    /// The parameter indicates from which position the selection is to be started. Must be empty or omitted
    /// for the first data chunk. In order to get subsequent chunks, you must set the "cursor" parameter in
    /// your request, using the value received in response to the previous request.
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Object with suppression details.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionListObject {
    /// Email to get suppression details for.
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Cause of email being suppressed. One of:
    ///
    /// - __unsubscribed__ - email is unsubscribed;
    /// - __temporary_unavailable__ - the email address is unavailable. This means that over the next three days sending to this address will return an error. Email may be temporarily unavailable due to several reasons, e.g.:
    ///
    /// - a previous email has been rejected by the recipient's server for spam;
    /// - the recipient's mailbox is full or is not used;
    /// - recipient's domain does not accept mail;
    /// - sending server was rejected due to blacklisting;
    ///
    /// - __permanent_unavailable__ - the email address is permanently unavailable due to multiple hard bounces;
    /// - __complained__ -  the recipient reported spam in the previous emails;
    /// - __blocked__ -  sending to the email is prohibited by administration of UniOne.
    ///
    /// We may add some new causes in the future.
    #[serde(rename = "cause", default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// Source of email being suppressed. One of:
    ///
    /// - __user__ - suppressed by user with    [suppression/set](https://docs.unione.io/en/web-api-ref#suppression-set);
    /// - __system__ - sending to the email is prohibited by system, for example due to multiple hard bounces;
    /// - __subscriber__ - the recipient reported spam or unsubscribed in the previous emails.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Is it possible to delete this suppression by calling suppression/delete method.
    #[serde(
        rename = "is_deletable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_deletable: Option<bool>,
    /// When suppression was created, in UTC timezone in "YYYY-MM-DD hh:mm:ss" format.
    #[serde(rename = "created", default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

/// SuppressionDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionDeleteRequest {
    /// An email to delete from suppression list.
    #[serde(rename = "email", default)]
    pub email: String,
}
