//! Models of the Common section.

use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The [template engine](https://docs.unione.io/en/template-engines) for handling the
/// substitutions(merge tags), "simple", "velocity", "liquid" or "none". Default value is "simple".
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TemplateEngine {
    #[serde(rename = "simple")]
    Simple,
    #[serde(rename = "velocity")]
    Velocity,
    #[serde(rename = "liquid")]
    Liquid,
    #[serde(rename = "none")]
    None,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// EmailStatus
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EmailStatus {
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "clicked")]
    Clicked,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "subscribed")]
    Subscribed,
    #[serde(rename = "soft_bounced")]
    SoftBounced,
    #[serde(rename = "hard_bounced")]
    HardBounced,
    #[serde(rename = "spam")]
    Spam,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Only domains with "confirmed" verification record are allowed as sender domains.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VerificationRecordStatus {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "unconfirmed")]
    Unconfirmed,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Sets the email counter mode. It allows you to choose whether the counter should reset automatically,
/// or remain permanent. The following values are acceptable: 'default' - the counter resets to zero
/// automatically at the beginning of each accounting period, 'permanent' - the counter never resets
/// automatically. The reset can be done only manually by the user in the request with the passes 0
/// value in the email_counter field. If the request doesn't contain the field or contains a null value,
/// the limit value doesn't change.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EmailCounterMode {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "permanent")]
    Permanent,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Contains HTML/plaintext/AMP parts of the email. Either html or plaintext part is required.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BodyObject {
    /// HTML part of the email body
    #[serde(rename = "html", default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Plaintext part of the email body.
    #[serde(rename = "plaintext", default, skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<String>,
    /// Optional [AMP](http://amp.dev/documentation/guides-and-tutorials/learn/email_fundamentals/) part of
    /// the email body.
    #[serde(rename = "amp", default, skip_serializing_if = "Option::is_none")]
    pub amp: Option<String>,
}

impl BodyObject {
    /// An HTML body; with a plain-text part: `BodyObject { plaintext: Some(text), ..BodyObject::html(html)
    /// }`.
    pub fn html(html: impl Into<String>) -> Self {
        BodyObject {
            html: Some(html.into()),
            ..Default::default()
        }
    }

    /// A plain-text body.
    pub fn text(plaintext: impl Into<String>) -> Self {
        BodyObject {
            plaintext: Some(plaintext.into()),
            ..Default::default()
        }
    }
}

/// Contains email headers, maximum 50. Only headers with “X-” name prefix are accepted, all other are
/// ignored, for example X-UNIONE-Global-Language, X-UNIONE-Template-Engine. Standard headers "To,"
/// "CC," and "BCC" are passed without the "X-." Yet, they are processed in a particular way and, as a
/// result, have a number of restrictions. You can find more details about it [here](/cc-and-bcc). If
/// our support have approved omitting standard unsubscription block for you, you can also pass
/// List-Unsubscribe, List-Subscribe, List-Help, List-Owner, List-Archive, In-Reply-To and References
/// headers.
///
/// Any other header goes to `extra`: `X-MyHeader`, `CC`, `List-Unsubscribe`, ...; a key of a named
/// field is sent from the field, not from `extra`. Unknown keys of a response are read into `extra`.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct HeadersObject {
    /// The language of the unsubscribe footer and unsubscribe page. Languages supported are: "be", "de",
    /// "en", "es", "fr", "it", "pl", "pt", "ru", "ua", "kz".
    #[serde(
        rename = "X-UNIONE-Global-Language",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub x_unione_global_language: Option<String>,
    /// The template engine for handling the substitutions(merge tags), “simple”, “velocity”, “liquid” or
    /// “none”. Default value is “simple”. Has a priority over the value allowed in the “template_engine”
    /// parameter.
    #[serde(
        rename = "X-UNIONE-Template-Engine",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub x_unione_template_engine: Option<String>,
    /// Keys beyond the named fields.
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

impl Serialize for HeadersObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if let Some(value) = &self.x_unione_global_language {
            map.serialize_entry("X-UNIONE-Global-Language", value)?;
        }
        if let Some(value) = &self.x_unione_template_engine {
            map.serialize_entry("X-UNIONE-Template-Engine", value)?;
        }
        for (key, value) in &self.extra {
            if !matches!(
                key.as_str(),
                "X-UNIONE-Global-Language" | "X-UNIONE-Template-Engine"
            ) {
                map.serialize_entry(key, value)?;
            }
        }
        map.end()
    }
}

/// AttachmentsArrayItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttachmentsArrayItem {
    /// Attachment type, see [MIME](https://en.wikipedia.org/wiki/MIME). If unsure, use
    /// "application/octet-stream".
    #[serde(rename = "type", default)]
    pub type_: String,
    /// Attachment name in the format: "name.extension". When multiple attachments are passed, their names
    /// must be unique. The symbol ‘/’ is not allowed in attachment names.
    #[serde(rename = "name", default)]
    pub name: String,
    /// File contents in [base64](https://en.wikipedia.org/wiki/Base64). Maximum file size 7MB (9786710
    /// bytes in base64).
    #[serde(rename = "content", default)]
    pub content: String,
}

impl AttachmentsArrayItem {
    /// An attachment from its bytes, `content` is base64-encoded.
    pub fn from_bytes(
        content_type: impl Into<String>,
        name: impl Into<String>,
        content: &[u8],
    ) -> Self {
        AttachmentsArrayItem {
            type_: content_type.into(),
            name: name.into(),
            content: base64(content),
        }
    }
}

/// InlineAttachmentsArrayItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct InlineAttachmentsArrayItem {
    /// Attachment type, see [MIME](https://en.wikipedia.org/wiki/MIME). If unsure, use
    /// "application/octet-stream".
    #[serde(rename = "type", default)]
    pub type_: String,
    /// Attachment content id. For example, "name"="IMAGECID1" should be referenced in HTML like <img
    /// src="cid:IMAGECID1">.
    #[serde(rename = "name", default)]
    pub name: String,
    /// File contents in [base64](https://en.wikipedia.org/wiki/Base64). Maximum file size 7MB (9786710
    /// bytes in base64).
    #[serde(rename = "content", default)]
    pub content: String,
}

impl InlineAttachmentsArrayItem {
    /// An attachment from its bytes, `content` is base64-encoded.
    pub fn from_bytes(
        content_type: impl Into<String>,
        name: impl Into<String>,
        content: &[u8],
    ) -> Self {
        InlineAttachmentsArrayItem {
            type_: content_type.into(),
            name: name.into(),
            content: base64(content),
        }
    }
}

/// SuccessResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuccessResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Standard base64 (with padding) of the bytes: the `content` of an attachment.
fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let mut n = 0u32;
        for (i, &byte) in chunk.iter().enumerate() {
            n |= u32::from(byte) << (16 - 8 * i);
        }
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(char::from(ALPHABET[((n >> (18 - 6 * i)) & 63) as usize]));
            } else {
                out.push('=');
            }
        }
    }
    out
}
