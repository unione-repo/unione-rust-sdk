//! Models of the Domain section.

use serde::{Deserialize, Serialize};

/// Only domains with "active" DKIM record are allowed as sender domains.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DkimStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// DomainGetDnsRecordsRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainGetDnsRecordsRequest {
    /// Domain to get DNS records for.
    #[serde(rename = "domain", default)]
    pub domain: String,
}

/// DomainGetDnsRecordsResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainGetDnsRecordsResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Domain to get DNS records for.
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Record to be added "as is" to verify ownership of this domain.
    #[serde(
        rename = "verification-record",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_record: Option<String>,
    /// DKIM signature for the domain. This property contains only the key part, you should prepend it with
    /// "v=DKIM1; k=rsa; p=" for the TXT record to be valid.
    #[serde(rename = "dkim", default, skip_serializing_if = "Option::is_none")]
    pub dkim: Option<String>,
    /// Target of the CNAME record for the "_dmarc" host of the domain.
    #[serde(rename = "dmarc", default, skip_serializing_if = "Option::is_none")]
    pub dmarc: Option<String>,
}

/// DomainValidateVerificationRecordRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainValidateVerificationRecordRequest {
    /// Domain to validate verification record for.
    #[serde(rename = "domain", default)]
    pub domain: String,
}

/// DomainValidateVerificationRecordResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainValidateVerificationRecordResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Debug message.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// DomainValidateDkimRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainValidateDkimRequest {
    /// Domain to validate DKIM record for.
    #[serde(rename = "domain", default)]
    pub domain: String,
}

/// DomainValidateDkimResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainValidateDkimResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Debug message.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// DomainListRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainListRequest {
    /// Optional domain name if you want to get the status of single domain only.
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Limits the number of domains returned. Default is 50.
    /// Default: `50`.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first domain in the list, starting with 0.
    /// Default: `0`.
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

/// DomainListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainListResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Array of objects describing domains.
    #[serde(rename = "domains", default, skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<DomainListResponseDomainsItem>>,
}

/// DomainListResponseDomainsItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainListResponseDomainsItem {
    /// Domain name.
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Object describing verification record value and status.
    #[serde(
        rename = "verification-record",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_record: Option<DomainListResponseDomainsItemVerificationRecord>,
    /// Object describing DKIM record value and status.
    #[serde(rename = "dkim", default, skip_serializing_if = "Option::is_none")]
    pub dkim: Option<DomainListResponseDomainsItemDkim>,
}

/// Object describing verification record value and status.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainListResponseDomainsItemVerificationRecord {
    /// Record to be added "as is" to verify ownership of this domain.
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Only domains with "confirmed" verification record are allowed as sender domains.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::common::model::VerificationRecordStatus>,
}

/// Object describing DKIM record value and status.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainListResponseDomainsItemDkim {
    /// DKIM signature for the domain. This property contains only the key part, you should prepend it with
    /// "k=rsa, p=" part for the record to be valid (see
    /// [domain/get-dns-records](https://docs.unione.io/en/web-api-ref#domain-get-dns-records)).
    #[serde(rename = "key", default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Only domains with "active" DKIM record are allowed as sender domains.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DkimStatus>,
}

/// DomainGetSandboxDomainResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainGetSandboxDomainResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Sandbox domain name.
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

/// DomainDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainDeleteRequest {
    /// Domain name to delete.
    #[serde(rename = "domain", default)]
    pub domain: String,
}

/// DomainDeleteResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainDeleteResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Domain deleted
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
