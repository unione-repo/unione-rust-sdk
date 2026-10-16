//! Models of the EmailValidation section.

use serde::{Deserialize, Serialize};

/// EmailValidationSingleRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmailValidationSingleRequest {
    /// Email address to be checked.
    #[serde(rename = "email", default)]
    pub email: String,
}

/// EmailValidationSingleResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmailValidationSingleResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Email address to be checked.
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Possible statuses:
    ///
    /// - "valid" - the address is valid,
    /// - "invalid" - the address is invalid,
    /// - "suspicious" - the address looks suspicious,
    /// - "unknown" - could not perform validation, the domain’s mail server has not responded within the time limit.
    #[serde(rename = "result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// Possible statuses:
    ///
    /// - "no_mx_record" - no MX record found for the target domain,
    /// - "syntax_error" - the address syntax is invalid,
    /// - "possible_typo" - the address is likely to have a typo,
    /// - "mailbox_not_found" - the address does not exist,
    /// - "global_suppression" - the address has been marked as unreachable due to multiple previous delivery errors,
    /// - "disposable" - this is a disposable one-time email address which is usually valid for a few minutes only,
    /// - "role" - the address is not likely to belong to an actual person, but rather to a certain business staff role,
    /// - "abuse" - the address is known to be a source of a large number of complaints, sometimes issued automatically,
    /// - "spamtrap" - this email is a spam trap, it is published openly but never used for actual emails. Sending messages to such addresses has a huge negative impact on reputation score,
    /// - "smtp_connection_failed" - the domain’s SMTP server does not respond; the address may contain a typo.
    #[serde(rename = "cause", default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// Validity score, from 0 to 100 (0 – invalid, 100 – valid).
    #[serde(rename = "validity", default, skip_serializing_if = "Option::is_none")]
    pub validity: Option<i64>,
    /// Local part (everything preceding the @ sign).
    #[serde(
        rename = "local_part",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub local_part: Option<String>,
    /// Domain name part.
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// True if the address’ domain does have an MX record, false if does not.
    #[serde(rename = "mx_found", default, skip_serializing_if = "Option::is_none")]
    pub mx_found: Option<bool>,
    /// Preferred MX record for the domain.
    #[serde(rename = "mx_record", default, skip_serializing_if = "Option::is_none")]
    pub mx_record: Option<String>,
    /// For addresses which are likely to have typing errors (cause=possible_typo), a suggested variant with
    /// a fixed typo.
    #[serde(
        rename = "did_you_mean",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub did_you_mean: Option<String>,
    /// Email check date and time, YYYY-MM-DD hh:mm:ss UTC.
    #[serde(
        rename = "processed_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub processed_at: Option<String>,
}
