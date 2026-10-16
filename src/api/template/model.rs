//! Models of the Template section.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Editor type, "html" or "visual". Default is "html". You can create a template with "visual" editor
/// type only in web interface.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EditorType {
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "visual")]
    Visual,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Object with all the properties of the template.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateSetRequest {
    /// Object with all the template properties.
    #[serde(rename = "template", default)]
    pub template: TemplateObject,
}

/// Object with all the template properties.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateObject {
    /// Template unique identifier. If you omit the "id" field then new template will be created. If you
    /// specify the "id" field, the system will try to find a template with the specified id and edit it.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Template name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Editor type, "html" or "visual". Default is "html". You can create a template with "visual" editor
    /// type only in web interface.
    #[serde(
        rename = "editor_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub editor_type: Option<EditorType>,
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
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<crate::common::model::BodyObject>,
    /// Email subject.
    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Sender's email.
    #[serde(rename = "from_email", default)]
    pub from_email: String,
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
}

/// TemplateSetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateSetResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Object with all the template properties.
    #[serde(rename = "template", default, skip_serializing_if = "Option::is_none")]
    pub template: Option<TemplateObject>,
}

/// TemplateGetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateGetRequest {
    /// Template id.
    #[serde(rename = "id", default)]
    pub id: String,
}

/// TemplateGetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateGetResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Object with all the template properties.
    #[serde(rename = "template", default, skip_serializing_if = "Option::is_none")]
    pub template: Option<TemplateObject>,
}

/// TemplateListRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateListRequest {
    /// Limits the number of templates returned, default is 50.
    /// Default: `50`.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first template in the list, starting with 0.
    /// Default: `0`.
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

/// TemplateListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateListResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Array of template objects.
    #[serde(rename = "templates", default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<TemplateListObject>>,
}

/// TemplateListObject
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateListObject {
    /// Template unique identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Template name.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Editor type, "html" or "visual". Default is "html". You can create a template with "visual" editor
    /// type only in web interface.
    #[serde(
        rename = "editor_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub editor_type: Option<EditorType>,
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
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<crate::common::model::BodyObject>,
    /// Email subject.
    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Sender's email.
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
    /// Template creation date and time in UTC timezone in format "YYYY-MM-DD hh:mm:ss"
    #[serde(rename = "created", default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Unique user identifier.
    #[serde(rename = "user_id", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Unqiue project identifier, ASCII string up to 36 characters long.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Project name, unique for user account.
    #[serde(
        rename = "project_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_name: Option<String>,
}

/// TemplateDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateDeleteRequest {
    /// Template id.
    #[serde(rename = "id", default)]
    pub id: String,
}
