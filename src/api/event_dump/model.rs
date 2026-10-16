//! Models of the EventDump section.

use serde::{Deserialize, Serialize};

/// EventDumpCreateRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpCreateRequest {
    /// Date and time in YYYY-MM-DD hh:mm:ss format, specifying period start time. Data is stored for up to
    /// 32 days, depending on your tariff.
    #[serde(rename = "start_time", default)]
    pub start_time: String,
    /// Date and time in YYYY-MM-DD hh:mm:ss format, specifying period end time (non-inclusive).
    #[serde(rename = "end_time", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Maximum number of events returned (default is 50). If this value is over 100 000, several files will
    /// be created, each having 100 000 events maximum.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// For accounts with projects enabled, the value all_projects=true allows to fetch data for all
    /// existing projects.
    /// Default: `False`.
    #[serde(
        rename = "all_projects",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub all_projects: Option<bool>,
    /// An object with the properties of the event dump filter.
    #[serde(rename = "filter", default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterObject>,
    /// An array of strings containing the names of columns you need in your report, in your preferred
    /// order. If the 'dump_fields' array is missing or empty, a default field list and order will be
    /// applied. Allowed values, in their default order, are - 'project_id', 'event_time', 'job_id',
    /// 'from_email', 'email', 'status', 'delivery_status', 'metadata', 'destination_response',
    /// 'user_agent', 'url', 'ip', 'tags'. For maximum security, the 'subject' field is not included by
    /// default, but you can add it by explicitly including in the 'dump_fields' array.
    #[serde(
        rename = "dump_fields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dump_fields: Option<Vec<String>>,
    /// Allows you to obtain aggregated statistics for a given time period. The two allowed values are:
    ///
    /// - __day_status__ - fetch aggregated statistics (the filter value must be missing or empty, as filters are not allowed with this option);
    /// - __an empty string__ - used by default, event_dump/create works as if the aggregate parameter is omitted.
    #[serde(rename = "aggregate", default, skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<String>,
    /// Field delimiter, can be set to either  ',' or ';', defaults to  ','.
    #[serde(rename = "delimiter", default, skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// File format, either “csv” (default) or “csv_gzip”.
    /// Default: `csv`.
    #[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// An object with the properties of the event dump filter.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilterObject {
    /// Job identifier returned earlier by [email/send](https://docs.unione.io/en/web-api-ref#email-send)
    /// method.
    #[serde(rename = "job_id", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// Email status. Possible values are:
    ///
    /// __accepted__ — the message has been accepted, but not sent yet.
    /// __sent__ — the message has been sent, but not delivered yet.
    /// __delivered__ — the message has been delivered.
    /// __opened__ — the message has been delivered and read.
    /// __clicked__ — the message has been delivered, read, and at least one of the links in email has been
    /// clicked.
    /// __unsubscribed__ — the message has been delivered to the recipient and read, but then the recipient
    /// unsubscribed.
    /// __subscribed__ — the message has been delivered to the recipient and read, but then the recipient
    /// unsubscribed and subscribed again.
    /// __soft_bounced__ — temporary delivery failure. UniOne will try to deliver the message during 48
    /// hours.
    /// __hard_bounced__ — failed to deliver the message, there will be no delivery attempts.
    /// __spam__ — the message has been delivered, but it was reported as spam by the recipient.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::common::model::EmailStatus>,
    /// UniOne internal detailed delivery status, you can pass several values at once, comma-separated. Some
    /// possible values are:
    ///
    /// __err_user_unknown__ -- email address doesn't exist;
    /// __err_user_inactive__ -- email address isn't used anymore;
    /// __err_will_retry__ -- email was temporary rejected, delivery will be retried later;
    /// __err_mailbox_discarded__ -- email address was active earlier, but now it's deleted;
    /// __err_mailbox_full__ -- mailbox is full;
    /// __err_spam_rejected__ -- email was rejected as a spam;
    /// __err_blacklisted__ -- email was rejected because sender IP or sender domain is found in a black
    /// list;
    /// __err_too_large__ -- email size is over limit, according to recipient's server;
    /// __err_unsubscribed__ -- email was unsubscribed;
    /// __err_unreachable__ -- numerous delivery failures lead to mark this email address as permanently
    /// unavailable;
    /// __err_skip_letter__ -- sending has been canceled because the email address is temporarily
    /// unavailable;
    /// __err_domain_inactive__ -- the domain does not accept mail or does not exist;
    /// __err_destination_misconfigured__ -- the domain does not accept mail due to incorrect settings on
    /// the recipient’s side, and the server’s response contains information about a cause that can be fixed
    /// (for example, an inoperative blacklist is used, etc.);
    /// __err_delivery_failed__ -- delivery failed due to other reasons;
    /// __err_spam_skipped__ -- sending has been canceled because the campaign has been blocked as a spam;
    /// __err_lost__ -- the message was not sent due to inconsistency of its parts or it has been lost
    /// because of the failure on our side. The sender needs to re-send the letter on his own, since the
    /// original has not been saved;
    ///
    /// We reserve the right to change or add internal delivery statuses as we need, so the following list
    /// is provided as an example only, without any description: ok_sent, ok_delivered, ok_read,
    /// ok_link_visited, ok_unsubscribed, ok_resubscribed, ok_spam_folder, ok_fbl, not_sent,
    /// skip_dup_unreachable, skip_dup_temp_unreachable, skip_dup_mailbox_full, err_spam_removed,
    /// err_resend, err_unknown, err_retry_letter, err_src_invalid, err_dest_invalid, err_not_available,
    /// err_internal, err_no_dns, err_no_smtp, err_giveup.
    #[serde(
        rename = "delivery_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_status: Option<String>,
    /// Recipient email address
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Sender email address
    #[serde(
        rename = "email_from",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_from: Option<String>,
    /// Recipient domain
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Campaign identifier, unsigned decimal integer or UUID up to 128-bit, passed in metadata with name
    /// "campaign_id" (name is configurable thru support).
    #[serde(
        rename = "campaign_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub campaign_id: Option<String>,
}

/// EventDumpCreateResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpCreateResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Dump identifier, used to call the
    /// [event-dump/get](https://docs.unione.io/en/web-api-ref#event-dump-get) method.
    #[serde(rename = "dump_id", default, skip_serializing_if = "Option::is_none")]
    pub dump_id: Option<String>,
}

/// EventDumpGetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpGetRequest {
    /// Dump identifier received by the method
    /// [event-dump/create](https://docs.unione.io/en/web-api-ref#event-dump-create).
    #[serde(rename = "dump_id", default)]
    pub dump_id: String,
}

/// EventDumpGetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpGetResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// An object with the properties of the event dump.
    #[serde(
        rename = "event_dump",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_dump: Option<EventDumpObject>,
}

/// An object with the properties of the event dump.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpObject {
    /// Dump ID received by the method
    /// [event-dump/create](https://docs.unione.io/en/web-api-ref#event-dump-create).
    #[serde(rename = "dump_id", default, skip_serializing_if = "Option::is_none")]
    pub dump_id: Option<String>,
    /// Task status, possible values are:
    ///
    /// - __queued__ - the task is in the queue, processing is not started yet;
    /// - __in_process__ - the request is being processed;
    /// - __ready__ - the task is finished, dump file is ready for download;
    /// - __failed__ -  processing failed due to an internal error.
    #[serde(
        rename = "dump_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dump_status: Option<String>,
    /// An array of objects, each representing a file, ready for download. You may start downloading
    /// everything listed in “files” even if "dump_status" is still "in_process". If there are no events,
    /// according to the specified parameters, an empty array will be returned.
    #[serde(rename = "files", default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<EventDumpFilesObject>>,
}

/// An object with the properties of the event dump file.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpFilesObject {
    /// Use this unique URL to download the requested dump file (either .csv or .csv.gz, depending on your
    /// request).
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// File size.
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// EventDumpListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpListResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// An array of objects describing event dumps. Each object is similar to 'event_dump' object returned
    /// by the [event-dump/get](https://docs.unione.io/en/web-api-ref#event-dump-get) method.
    #[serde(
        rename = "event_dumps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_dumps: Option<Vec<EventDumpObject>>,
}

/// EventDumpDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpDeleteRequest {
    /// Dump identifier, obtained via the
    /// [event-dump/create](https://docs.unione.io/en/web-api-ref#event-dump-create) method.
    #[serde(rename = "dump_id", default)]
    pub dump_id: String,
}
