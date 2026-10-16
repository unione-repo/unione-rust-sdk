//! Models of the Tag section.

use serde::{Deserialize, Serialize};

/// TagListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TagListResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// An array of objects containing data for user-defined tags.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagsResponseObject>>,
}

/// An object with the properties of the user tags.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TagsResponseObject {
    /// Unique tag id.
    #[serde(rename = "tag_id", default, skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<i64>,
    /// Tag name.
    #[serde(rename = "tag", default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// TagDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TagDeleteRequest {
    /// Unique tag id.
    #[serde(rename = "tag_id", default)]
    pub tag_id: i64,
}
