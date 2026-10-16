//! Models of the Webhook section.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Webhook status, "active" by default. "disabled" means that webhook has been disabled by the user,
/// "stopped" means that webhook has been stopped by the system after 24h of failed calls (with minimum
/// of 10 distinct events).
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum WebhookStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "stopped")]
    Stopped,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Notification format. "json_post"(default) or "json_post_gzip".
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum WebhookEventFormat {
    #[serde(rename = "json_post")]
    JsonPost,
    #[serde(rename = "json_post_gzip")]
    JsonPostGzip,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Type of event data contained in event_data field, either "transactional_email_status" or
/// "transactional_spam_block".
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EventName {
    #[serde(rename = "transactional_email_status")]
    TransactionalEmailStatus,
    #[serde(rename = "transactional_spam_block")]
    TransactionalSpamBlock,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Recipient's device type, detected from the user agent. Present for "clicked" and "opened" statuses;
/// equals "unknown" if the device type couldn't be detected.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DeviceType {
    #[serde(rename = "desktop")]
    Desktop,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "tablet")]
    Tablet,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "unknown")]
    Unknown,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Traffic quality for "opened" and "clicked" statuses. "human" means a recognized browser or email
/// client without machine signs; "machine" means a bot, link scanner or security proxy; "protected"
/// means a mailbox/OS privacy proxy where a human action cannot be confirmed (Apple Mail Privacy
/// Protection / iCloud Private Relay, Gmail Image Proxy — Apple MPP is protected, not machine);
/// "unknown" means the user agent is missing, broken or not recognized.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TrafficType {
    #[serde(rename = "human")]
    Human,
    #[serde(rename = "machine")]
    Machine,
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "unknown")]
    Unknown,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Spam block type, either single or multiple sending SMTP. For single sending SMTP block in common
/// pool UniOne retries several other SMTPs. This property exists only if
/// event_name="transactional_spam_block".
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BlockType {
    #[serde(rename = "one_smtp")]
    OneSmtp,
    #[serde(rename = "all_smtp")]
    AllSmtp,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Whether it's a block or unblock event. This property exists only if
/// event_name="transactional_spam_block".
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DomainStatus {
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "unblocked")]
    Unblocked,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// WebhookSetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookSetRequest {
    /// URL that will receive the notification when an event occurs. The URL should be unique for a user or
    /// a project. Only ASCII characters are supported for now in URL, please convert to
    /// [Punycode](https://en.wikipedia.org/wiki/Punycode) if you need to use non-ASCII characters.
    #[serde(rename = "url", default)]
    pub url: String,
    /// Webhook status, "active" by default. "disabled" means that webhook has been disabled by the user,
    /// "stopped" means that webhook has been stopped by the system after 24h of failed calls (with minimum
    /// of 10 distinct events).
    /// Default: `active`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WebhookStatus>,
    /// Notification format. "json_post"(default) or "json_post_gzip".
    /// Default: `json_post`.
    #[serde(
        rename = "event_format",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_format: Option<WebhookEventFormat>,
    /// Shall detailed delivery info be returned(1) or not(0). When it's 1, the information about SMTP
    /// response and internal delivery status is returned for "hard_bounced" and "soft_bounced" email
    /// statuses; the user agent, recipient IP address, detected geo fields, device info detected from the
    /// user agent (device type and brand, OS name, browser or email client name) and traffic_type (human,
    /// machine, protected or unknown) are returned for "opened" and "clicked" statuses; URL is returned for
    /// "clicked" status. For "delivered", "soft_bounced" and "hard_bounced" statuses UniOne may also return
    /// sender_ip, the IP address of the SMTP server that sent the message, if available.
    /// Allowed values: `0`, `1`.
    /// Default: `0`.
    #[serde(
        rename = "delivery_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_info: Option<i64>,
    /// 0=several event notifications can be reported in single webhook call, 1=only single event can be
    /// reported in one call (not recommended).
    /// Allowed values: `0`, `1`.
    /// Default: `0`.
    #[serde(
        rename = "single_event",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub single_event: Option<i64>,
    /// Maximum quantity of permitted parallel queries to your server. The more your server can handle - the
    /// better.
    /// Default: `10`.
    #[serde(
        rename = "max_parallel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_parallel: Option<i64>,
    /// Object containing events to notify of.
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<WebhookEventsObject>,
}

/// Object containing events to notify of.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookEventsObject {
    /// If present then spam block events will be reported. Should contain single array element with "*"
    /// string.
    #[serde(
        rename = "spam_block",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub spam_block: Option<Vec<String>>,
    /// If present then email status change events will be reported. Contains names of statuses to notify
    /// of.
    #[serde(
        rename = "email_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_status: Option<Vec<crate::common::model::EmailStatus>>,
}

/// WebhookSetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookSetResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Object with all the webhook properties.
    #[serde(rename = "object", default, skip_serializing_if = "Option::is_none")]
    pub object: Option<WebhookObject>,
}

/// Object with all the webhook properties.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookObject {
    /// Webhook unique identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Webhook URL.
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Webhook status, "active" by default. "disabled" means that webhook has been disabled by the user,
    /// "stopped" means that webhook has been stopped by the system after 24h of failed calls (with minimum
    /// of 10 distinct events).
    /// Default: `active`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WebhookStatus>,
    /// Notification format. "json_post"(default) or "json_post_gzip".
    /// Default: `json_post`.
    #[serde(
        rename = "event_format",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_format: Option<WebhookEventFormat>,
    /// Shall detailed delivery info be returned(1) or not(0). When it's 1, the information about SMTP
    /// response and internal delivery status is returned for "hard_bounced" and "soft_bounced" email
    /// statuses; the user agent, recipient IP address, detected geo fields, device info detected from the
    /// user agent (device type and brand, OS name, browser or email client name) and traffic_type (human,
    /// machine, protected or unknown) are returned for "opened" and "clicked" statuses; URL is returned for
    /// "clicked" status. For "delivered", "soft_bounced" and "hard_bounced" statuses UniOne may also return
    /// sender_ip, the IP address of the SMTP server that sent the message, if available.
    /// Allowed values: `0`, `1`.
    /// Default: `0`.
    #[serde(
        rename = "delivery_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_info: Option<i64>,
    /// 0=several event notifications can be reported in single webhook call, 1=only single event can be
    /// reported in one call (not recommended).
    /// Allowed values: `0`, `1`.
    /// Default: `0`.
    #[serde(
        rename = "single_event",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub single_event: Option<i64>,
    /// Maximum quantity of permitted parallel queries to your server. The more your server can handle - the
    /// better.
    /// Default: `10`.
    #[serde(
        rename = "max_parallel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_parallel: Option<i64>,
    /// Webhook properties last update date and time in UTC timezone in "YYYY-MM-DD hh:mm:ss" format.
    #[serde(
        rename = "updated_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<String>,
    /// Object containing events to notify of.
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<WebhookEventsObject>,
}

/// Webhook callback format description.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookCallback {
    /// MD5-hash of the message body, in which the value "auth" is replaced by api key of the user/project;
    /// with this field the recipient of the notification can both authenticate and verify the notification
    /// integrity
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    /// Array with only one element, containing events of a user/project.
    #[serde(
        rename = "events_by_user",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub events_by_user: Option<Vec<WebhookCallbackEventsByUserItem>>,
}

/// WebhookCallbackEventsByUserItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookCallbackEventsByUserItem {
    /// Unique user identifier.
    #[serde(rename = "user_id", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Project identifier, present only if webhook was registered for the project using project API key.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Project name, present only if webhook was registered for the project using project API key.
    #[serde(
        rename = "project_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_name: Option<String>,
    /// Array of events reported by webhook.
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<WebhookCallbackEventsByUserItemEventsItem>>,
}

/// WebhookCallbackEventsByUserItemEventsItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookCallbackEventsByUserItemEventsItem {
    /// Type of event data contained in event_data field, either "transactional_email_status" or
    /// "transactional_spam_block".
    #[serde(
        rename = "event_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_name: Option<EventName>,
    /// Object with different event properties depending on "event_name". Below you can see all the
    /// properties, "transactional_email_status"-related first and then "transactional_spam_block"-related.
    #[serde(
        rename = "event_data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_data: Option<WebhookCallbackEventsByUserItemEventsItemEventData>,
}

/// Object with different event properties depending on "event_name". Below you can see all the
/// properties, "transactional_email_status"-related first and then "transactional_spam_block"-related.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookCallbackEventsByUserItemEventsItemEventData {
    /// Job identifier returned earlier by [email/send](https://docs.unione.io/en/web-api-ref#email-send)
    /// method. Property exists only if event_name="transactional_email_status".
    #[serde(rename = "job_id", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// Metadata passed in [email/send](https://docs.unione.io/en/web-api-ref#email-send) method in
    /// recipients.metadata or global_metadata properties. This property exists only if
    /// event_name="transactional_email_status".
    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Recipient's email. This property exists only if event_name="transactional_email_status".
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Email delivery status. This property exists only if event_name="transactional_email_status".
    /// Possible values are:
    ///
    /// __accepted__ — the message has been accepted, but not sent yet.
    /// __sent__ — the message has been sent, but not delivered yet.
    /// __delivered__ — the message has been delivered. It can be changed to “opened”, “clicked”,
    /// “unsubscribed”, “subscribed” or “spam”.
    /// __opened__ — the message has been delivered and read. It can be changed to “clicked”,
    /// “unsubscribed”, “subscribed” or “spam”.
    /// __clicked__ — the message has been delivered, read, and at least one of the links in email has been
    /// clicked. It can be changed to “unsubscribed”, “subscribed” or “spam”.
    /// __unsubscribed__ — the message has been delivered to the recipient and read, but then the recipient
    /// unsubscribed. It can be changed to “subscribed”.
    /// __subscribed__ — the message has been delivered to the recipient and read, but then the recipient
    /// unsubscribed and subscribed again. It can be changed to “unsubscribed”.
    /// __soft_bounced__ — temporary delivery failure. UniOne will try to deliver the message during 48
    /// hours. In case of success the status changes to “delivered”, in case of failure to deliver during 48
    /// hours message status changes to “hard_bounced”.
    /// __hard_bounced__ — failed to deliver the message, there will be no delivery attempts. It's a final
    /// status. There are a lot of possible reasons, you can use SMTP response from
    /// delivery_info.destination_response field or UniOne internal reason classification from
    /// delivery_info.delivery_status field to analyze the reason.
    /// __spam__ — the message has been delivered, but it was reported as spam by the recipient. UniOne can
    /// receive and process the spam complaint from several domains, including msn.com, outlook.com,
    /// hotmail.com, live.com, ukr.net, yahoo.com, aol.com using
    /// [FBL](https://en.wikipedia.org/wiki/Feedback_loop_(email)).
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::common::model::EmailStatus>,
    /// Event date & time in UTC time zone in "YYYY-MM-DD hh:mm:ss" format. This property exists only if
    /// event_name="transactional_email_status".
    #[serde(
        rename = "event_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_time: Option<String>,
    /// URL for "opened" and "clicked" statuses. This property exists only if
    /// event_name="transactional_email_status".
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Object with detailed delivery info.Event date & time in UTC time zone in "YYYY-MM-DD hh:mm:ss"
    /// format. This property exists only if webhook has delivery_info proeprty set to 1 and
    /// event_name="transactional_email_status".
    #[serde(
        rename = "delivery_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_info: Option<WebhookCallbackEventsByUserItemEventsItemEventDataDeliveryInfo>,
    /// Spam block date & time in UTC time zone in "YYYY-MM-DD hh:mm:ss" format. This property exists only
    /// if event_name="transactional_spam_block".
    #[serde(
        rename = "block_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_time: Option<String>,
    /// Spam block type, either single or multiple sending SMTP. For single sending SMTP block in common
    /// pool UniOne retries several other SMTPs. This property exists only if
    /// event_name="transactional_spam_block".
    #[serde(
        rename = "block_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_type: Option<BlockType>,
    /// Domain that blocked sending. This property exists only if event_name="transactional_spam_block".
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Number of sending SMTPs blocked. This property exists only if event_name="transactional_spam_block".
    #[serde(
        rename = "SMTP_blocks_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub smtp_blocks_count: Option<i64>,
    /// Whether it's a block or unblock event. This property exists only if
    /// event_name="transactional_spam_block".
    #[serde(
        rename = "domain_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub domain_status: Option<DomainStatus>,
}

/// Object with detailed delivery info.Event date & time in UTC time zone in "YYYY-MM-DD hh:mm:ss"
/// format. This property exists only if webhook has delivery_info proeprty set to 1 and
/// event_name="transactional_email_status".
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookCallbackEventsByUserItemEventsItemEventDataDeliveryInfo {
    /// UniOne internal detailed delivery status. Some possible values are:
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
    /// SMTP response.
    #[serde(
        rename = "destination_response",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_response: Option<String>,
    /// User agent of recipient. Present only if detected for "clicked" and "opened" statuses.
    #[serde(
        rename = "user_agent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_agent: Option<String>,
    /// Recipient's device type, detected from the user agent. Present for "clicked" and "opened" statuses;
    /// equals "unknown" if the device type couldn't be detected.
    #[serde(
        rename = "device_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub device_type: Option<DeviceType>,
    /// Recipient's device brand, detected from the user agent. Present for "clicked" and "opened" statuses
    /// if detected. For device_type "mobile" and "tablet" it's always present and equals "unknown" when the
    /// brand couldn't be detected.
    #[serde(
        rename = "device_brand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub device_brand: Option<String>,
    /// Name of the operating system of the recipient's device, detected from the user agent. Present for
    /// "clicked" and "opened" statuses; equals "unknown" if the OS couldn't be detected.
    #[serde(rename = "os_name", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    /// Name of the browser the email was opened in, detected from the user agent. Present for "clicked" and
    /// "opened" statuses only if a browser was detected, so it never comes together with email_client_name.
    #[serde(
        rename = "browser_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_name: Option<String>,
    /// Name of the email client application the email was opened in, detected from the user agent. Present
    /// for "clicked" and "opened" statuses only if an email client was detected, so it never comes together
    /// with browser_name.
    #[serde(
        rename = "email_client_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_client_name: Option<String>,
    /// Traffic quality for "opened" and "clicked" statuses. "human" means a recognized browser or email
    /// client without machine signs; "machine" means a bot, link scanner or security proxy; "protected"
    /// means a mailbox/OS privacy proxy where a human action cannot be confirmed (Apple Mail Privacy
    /// Protection / iCloud Private Relay, Gmail Image Proxy — Apple MPP is protected, not machine);
    /// "unknown" means the user agent is missing, broken or not recognized.
    #[serde(
        rename = "traffic_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_type: Option<TrafficType>,
    /// Recipient's IP address. Present only if detected for "clicked" and "opened" statuses.
    #[serde(rename = "ip", default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// ISO 3166-1 alpha-2 country code detected from recipient IP. Present for "clicked" and "opened"
    /// statuses.
    #[serde(
        rename = "country_code",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub country_code: Option<String>,
    /// Country detected from recipient IP. Present for "clicked" and "opened" statuses.
    #[serde(rename = "country", default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// City detected from recipient IP. Present for "clicked" and "opened" statuses.
    #[serde(rename = "city", default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// IP address of the SMTP server that sent the message. Present only for "delivered", "soft_bounced"
    /// and "hard_bounced" statuses, if available.
    #[serde(rename = "sender_ip", default, skip_serializing_if = "Option::is_none")]
    pub sender_ip: Option<String>,
}

/// WebhookGetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookGetRequest {
    /// Webhook URL.
    #[serde(rename = "url", default)]
    pub url: String,
}

/// WebhookGetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookGetResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Object with all the webhook properties.
    #[serde(rename = "object", default, skip_serializing_if = "Option::is_none")]
    pub object: Option<WebhookObject>,
}

/// WebhookListRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookListRequest {
    /// Limits the number of webhooks returned. 50 is recommended.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first webhook in the list, starting with 0.
    /// Default: `0`.
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

/// WebhookListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookListResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Array of webhooks objects.
    #[serde(rename = "objects", default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<WebhookObject>>,
}

/// WebhookDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookDeleteRequest {
    /// Webhook URL.
    #[serde(rename = "url", default)]
    pub url: String,
}
