//! Models of the Email section.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// EmailSendRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmailSendRequest {
    /// Object containing all message properties.
    #[serde(rename = "message", default)]
    pub message: MessageObject,
}

/// Object containing all message properties.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageObject {
    /// Contains recipient emails, substitutions(merge tags) and metadata.
    #[serde(rename = "recipients", default)]
    pub recipients: Vec<MessageObjectRecipientsItem>,
    /// Optional identifier of the template that had been created by
    /// [template/set](https://docs.unione.io/en/web-api-ref#template-set) method. If template_id is passed
    /// then fields from the template are used instead of missed email/send parameters. E.g. if "body"
    /// parameter in email/send is omitted, "body" from template will be used. If "subject" in email/send is
    /// omitted, "subject" from template will be used.
    #[serde(
        rename = "template_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_id: Option<String>,
    /// An array of strings. The maximum string length is 100 characters. You can include up to 4 strings
    /// which must be unique. No more than 10 000 tags are allowed per project; if you try to add more,
    /// email/send will return an error. You may use tags to categorize emails by your own criteria, and
    /// they will be passed along by event-dump methods.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Whether to skip or not appending default unsubscribe footer. 1=skip, 0=append, default is 0. You
    /// should [ask support](https://cp.unione.io/en/support) to approve
    /// usage of skip_unsubscribe=1. [Read more](https://docs.unione.io/en/unsubscribe-link).
    /// Allowed values: `0`, `1`.
    #[serde(
        rename = "skip_unsubscribe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_unsubscribe: Option<i64>,
    /// The language of the unsubscribe footer and unsubscribe page. Languages supported are: "be", "de",
    /// "en", "es", "fr", "it", "pl", "pt", "ru", "ua", "kz".
    #[serde(
        rename = "global_language",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_language: Option<String>,
    /// The [template engine](https://docs.unione.io/en/template-engines) for handling the
    /// substitutions(merge tags), "simple", "velocity", "liquid" or "none". Default value is "simple".
    #[serde(
        rename = "template_engine",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_engine: Option<crate::common::model::TemplateEngine>,
    /// Object for passing the substitutions(merge tags) common for all recipients - e.g., company name. If
    /// the substitution names are duplicated in object "substitutions", the values of the variables will be
    /// taken from the object "substitutions". The substitutions can be used in the following parameters:
    ///
    /// - body.html
    /// - body.plaintext
    /// - body.amp
    /// - subject
    /// - from_name
    /// - options.unsubscribe_url
    ///
    /// A substitution name can consist from latin characeters, numbers and "_" symbol, and should start
    /// with the letter.
    #[serde(
        rename = "global_substitutions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_substitutions: Option<HashMap<String, String>>,
    /// Object for passing the metadata common for all the recipients, such as "key": "value". Max key
    /// quantity: 10. Max key length: 64 symbols. Max value length: 1024 symbols. The metadata is returned
    /// by [webhook](https://docs.unione.io/en/web-api-ref#webhook). To group single emails into a campaign,
    /// you can use the system key 'campaign_id' with a value in the form of a non-negative decimal integer
    /// up to 128 bits or a UUID in the format 'c7703772-453e-4c77-a9ae-9a1b94051b5a'. Incorrect values will
    /// be treated as '0'.
    #[serde(
        rename = "global_metadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_metadata: Option<HashMap<String, String>>,
    /// Contains HTML/plaintext/AMP parts of the email. Either html or plaintext part is required.
    #[serde(rename = "body", default)]
    pub body: crate::common::model::BodyObject,
    /// Email subject.
    #[serde(rename = "subject", default)]
    pub subject: String,
    /// Sender's email. Required only if template_id parameter is empty.
    #[serde(
        rename = "from_email",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_email: Option<String>,
    /// Sender's name.
    #[serde(rename = "from_name", default, skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// Optional Reply-To email (in case it's different to sender's email)
    #[serde(rename = "reply_to", default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Optional Reply-To name (if reply_to email is specified and you want to display not only this email
    /// but also the name)
    #[serde(
        rename = "reply_to_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to_name: Option<String>,
    /// 1 = click tracking is on (default), 0 = click tracking is off. To use track_links = 0, you need to
    /// ask support to enable this feature.
    /// Allowed values: `0`, `1`.
    #[serde(
        rename = "track_links",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub track_links: Option<i64>,
    /// 1 = read tracking is on (default), 0=read tracking is off. To use track_read = 0, you need to ask
    /// support to enable this feature.
    /// Allowed values: `0`, `1`.
    #[serde(
        rename = "track_read",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub track_read: Option<i64>,
    /// (allowed values 0 or 1, default 0) If set to 1, the global unavailability list will be ignored. Even
    /// if the address was found to be unreachable while sending other UniOne users' emails, or its owner
    /// has issued complaints, the message will still be sent. The setting may be ignored for certain
    /// addresses.
    /// Allowed values: `0`, `1`.
    #[serde(
        rename = "bypass_global",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bypass_global: Option<i64>,
    /// (allowed values 0 or 1, default 0) If set to 1, the current user's or project's unavailability list
    /// will be ignored. Works only if bypass_global is set to 1.
    /// Allowed values: `0`, `1`.
    #[serde(
        rename = "bypass_unavailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bypass_unavailable: Option<i64>,
    /// (allowed values 0 or 1, default 0) If set to 1, the current list of unsubscribed addresses for this
    /// account or project will be ignored. Works only if bypass_global is set to 1. The setting is
    /// available only for users that have been granted the right to omit the unsubscribe link (to request,
    /// please contact [support](https://cp.unione.io/en/support)).
    /// Allowed values: `0`, `1`.
    #[serde(
        rename = "bypass_unsubscribed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bypass_unsubscribed: Option<i64>,
    /// (allowed values 0 or 1, default 0) If set to 1, the user's or project's complaint list will be
    /// ignored. Works only if bypass_global is set to 1. The setting is available only for users that have
    /// been granted the right to omit the unsubscribe link (to request, please contact
    /// [support](https://cp.unione.io/en/support)).
    /// Allowed values: `0`, `1`.
    #[serde(
        rename = "bypass_complained",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bypass_complained: Option<i64>,
    /// A string of up to 64 characters containing a unique message key. This can be used to prevent
    /// occasional message duplicates. If you send another API request with the same message key within the
    /// next minute, it will be declined. We can generate a message key for each letter automatically; to
    /// enable this option, please contact our [tech support](https://cp.unione.io/en/support).
    #[serde(
        rename = "idempotence_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub idempotence_key: Option<String>,
    /// Contains email headers, maximum 50. Only headers with “X-” name prefix are accepted, all other are
    /// ignored, for example X-UNIONE-Global-Language, X-UNIONE-Template-Engine. Standard headers "To,"
    /// "CC," and "BCC" are passed without the "X-." Yet, they are processed in a particular way and, as a
    /// result, have a number of restrictions. You can find more details about it [here](/cc-and-bcc). If
    /// our support have approved omitting standard unsubscription block for you, you can also pass
    /// List-Unsubscribe, List-Subscribe, List-Help, List-Owner, List-Archive, In-Reply-To and References
    /// headers.
    #[serde(rename = "headers", default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<crate::common::model::HeadersObject>,
    /// Optional array of attachments
    #[serde(
        rename = "attachments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attachments: Option<Vec<crate::common::model::AttachmentsArrayItem>>,
    /// Optional array of inline attachments, e.g. for including images in email instead of loading them
    /// from external URL.
    #[serde(
        rename = "inline_attachments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_attachments: Option<Vec<crate::common::model::InlineAttachmentsArrayItem>>,
    /// Additional message options.
    #[serde(rename = "options", default, skip_serializing_if = "Option::is_none")]
    pub options: Option<MessageObjectOptions>,
}

/// MessageObjectRecipientsItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageObjectRecipientsItem {
    /// Recipient email
    #[serde(rename = "email", default)]
    pub email: String,
    /// Object to pass the substitutions(merge tags) for the recipient (e.g. recipient name, discount code,
    /// password change link, etc. See [Template engines](https://docs.unione.io/en/template-engines)). The
    /// substitutions can be used in the following parameters:
    ///
    /// - body.html
    /// - body.plaintext
    /// - body.amp
    /// - subject
    /// - from_name
    /// - headers["List-Unsubscribe"]
    /// - options.unsubscribe_url
    ///
    /// A substitution name can consist from latin characters, numbers and "_" symbol, and should start with
    /// the letter. There's a special substitution "to_name" which is used to put recipent's name like "Name
    /// Surname" to include it to SMTP header "To" in the form "Name Surname <email@example.com>". The
    /// "to_name" length is limited to 78 symbols.
    #[serde(
        rename = "substitutions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub substitutions: Option<HashMap<String, String>>,
    /// Object for passing the metadata, such as "key": "value". Max key quantity: 10. Max key length: 64
    /// symbols. Max value length: 1024 symbols. The metadata is returned by
    /// [webhook](https://docs.unione.io/en/web-api-ref#webhook) and
    /// [event-dump](https://docs.unione.io/en/web-api-ref#event-dump) methods. If you pass strign up to 128
    /// bit with "campaign_id" field name (name is configurable thru support), it will be considered as
    /// campaign identifier in statistics.
    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Additional message options.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageObjectOptions {
    /// Date and time in "YYYY-MM-DD hh:mm:ss" format in the UTC timezone. Allows schedule sending up to 24
    /// hours in advance.
    #[serde(rename = "send_at", default, skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,
    /// Custom unsubscribe link. Read more [here](https://docs.unione.io/en/unsubscribe-link).
    #[serde(
        rename = "unsubscribe_url",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribe_url: Option<String>,
    /// Backend-domain identifier to send message with. If custom_backend_id is not set, default one for
    /// your account/project will be used. You can pay for one or more dedicated IPs and use this option.
    #[serde(
        rename = "custom_backend_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_backend_id: Option<i64>,
    /// SMTP pool identifier to send message with. Usually you don't need to pass this parameter because
    /// correct smtp_pool_id is selected automatically based on the default one for your account/project or
    /// according to custom_backend_id parameter.
    #[serde(
        rename = "smtp_pool_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub smtp_pool_id: Option<String>,
}

/// EmailSendResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmailSendResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Job identifier, might be useful for investigating errors.
    #[serde(rename = "job_id", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// Array of recipients emails successfully accepted for sending.
    #[serde(rename = "emails", default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    /// Object with emails rejected for sending as property names and their statuses as property values,
    /// e.g.: {"email1@gmail.com": "temporary_unavailable"}. Possible statuses:
    ///
    /// - __unsubscribed__ - specified email is unsubscribed;
    /// - __invalid__ - the email does not exist or has been entered incorrectly;
    /// - __duplicate__ - the email already exists in the request, email duplicating is prevented;
    /// - __temporary_unavailable__ - the email address is unavailable. This means that over the next three days sending to this address will return an error. Email may be temporarily unavailable due to several reasons, e.g.:
    ///
    /// - a previous email has been rejected by the recipient's server for spam;
    /// - the recipient's mailbox is full or is not used;
    /// - recipient's domain does not accept mail;
    /// - sending server was rejected due to blacklisting;
    ///
    /// - __permanent_unavailable__ - the email address is permanently unavailable or unsubscribed globally.
    /// - __complained__ -  the recipient reported spam in the previous emails;
    /// - __blocked__ -  sending to the email is prohibited by administration of UniOne.
    ///
    /// We may added new statuses in the future.
    #[serde(
        rename = "failed_emails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failed_emails: Option<HashMap<String, String>>,
}

/// EmailSubscribeRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmailSubscribeRequest {
    /// Sender’s email on a confirmed domain. (Field alias "email_address_from" is supported, too).
    #[serde(rename = "from_email", default)]
    pub from_email: String,
    /// Sender’s name. (Field alias "name_from" is supported, too).
    #[serde(rename = "from_name", default)]
    pub from_name: String,
    /// Recipient’s email. (Field alias "email_address_to" is supported, too).
    #[serde(rename = "to_email", default)]
    pub to_email: String,
}
