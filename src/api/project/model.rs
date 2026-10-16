//! Models of the Project section.

use serde::{Deserialize, Serialize};

/// ProjectCreateRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectCreateRequest {
    /// Object with project properties.
    #[serde(rename = "project", default)]
    pub project: ProjectObject,
}

/// Object with project properties.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectObject {
    /// Project name, unique for user account.
    #[serde(rename = "name", default)]
    pub name: String,
    /// ISO-3166 alpha-2 country code. If set, UniOne treats project personal data according to country
    /// laws, e.g. GDPR for european countries.
    #[serde(rename = "country", default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Whether email sending is enabled for this project or not.
    /// Default: `True`.
    #[serde(
        rename = "send_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_enabled: Option<bool>,
    /// If it's false then UniOne adds default unsubscribe footer to each email sent with API key of the
    /// project. If it's true then appending default unsubscribe footer for this project is avoided and
    /// sending with custom unsubscribe url or even without unsubscribe url is permitted for the project.
    /// Please note that custom_unsubscribe_url_enabled=true is available only if removing unsubscribe link
    /// is approved for the user account by support. More on this
    /// [here](https://docs.unione.io/en/unsubscribe-link). When custom_unsubscribe_url_enabled is skipped
    /// on creating a project, it's value is taken from user
    #[serde(
        rename = "custom_unsubscribe_url_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_unsubscribe_url_enabled: Option<bool>,
    /// A unique domain identifier which will determine the [tracking
    /// domain](https://docs.unione.io/en/tracking-domains) or [dedicated IP](/dedicated-ip) pool to be used
    /// by default. If the value is not specified, a default backend domain will be assigned for your
    /// account or project instead.
    #[serde(
        rename = "backend_domain_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_domain_id: Option<i64>,
    /// A unique identifier that defines the default unsubscribe page. If absent, the account/project will
    /// be assigned the system default unsubscribe page.
    #[serde(
        rename = "unsubscribe_page_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribe_page_id: Option<i64>,
}

/// ProjectCreateResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectCreateResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Unqiue project identifier, ASCII string up to 36 characters long.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// API key of the project. You can use it instead of the user API key parameter in all methods except
    /// project/* methods.
    #[serde(
        rename = "project_api_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_api_key: Option<String>,
}

/// ProjectUpdateRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectUpdateRequest {
    /// Unique project identifier. You should identify project to update either with project_id or with
    /// project_api_key field. But we recommend usage of project_id as a more secure option.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Optional project API key. You should identify project to update either with project_id or with
    /// project_api_key field. But we recommend usage of project_id as a more secure option.
    #[serde(
        rename = "project_api_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_api_key: Option<String>,
    /// Object with project properties.
    #[serde(rename = "project", default)]
    pub project: ProjectUpdateObject,
}

/// Object with project properties.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectUpdateObject {
    /// Project name, unique for user account.
    #[serde(rename = "name", default)]
    pub name: String,
    /// ISO-3166 alpha-2 country code. If set, UniOne treats project personal data according to country
    /// laws, e.g. GDPR for european countries.
    #[serde(rename = "country", default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Whether email sending is enabled for this project or not.
    /// Default: `True`.
    #[serde(
        rename = "send_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_enabled: Option<bool>,
    /// If it's false then UniOne adds default unsubscribe footer to each email sent with API key of the
    /// project. If it's true then appending default unsubscribe footer for this project is avoided and
    /// sending with custom unsubscribe url or even without unsubscribe url is permitted for the project.
    /// Please note that custom_unsubscribe_url_enabled=true is available only if removing unsubscribe link
    /// is approved for the user account by support. More on this
    /// [here](https://docs.unione.io/en/unsubscribe-link). When custom_unsubscribe_url_enabled is skipped
    /// on creating a project, it's value is taken from user
    #[serde(
        rename = "custom_unsubscribe_url_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_unsubscribe_url_enabled: Option<bool>,
    /// A unique domain identifier which will determine the [tracking
    /// domain](https://docs.unione.io/en/tracking-domains) or [dedicated IP](/dedicated-ip) pool to be used
    /// by default. If the value is not specified, a default backend domain will be assigned for your
    /// account or project instead.
    #[serde(
        rename = "backend_domain_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_domain_id: Option<i64>,
    /// A unique identifier that defines the default unsubscribe page. If absent, the account/project will
    /// be assigned the system default unsubscribe page.
    #[serde(
        rename = "unsubscribe_page_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribe_page_id: Option<i64>,
    /// Project's email counter value. 0 value only is accepted (which means resetting the counter to zero),
    /// or refreshes automatically depending on the counter mode set with the 'email_counter_mode'
    /// parameter. If the request doesn't contain the field or contains null, the limit value doesn't
    /// change.
    #[serde(
        rename = "email_counter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter: Option<i64>,
    /// Allows you to limit the number of emails sent by the project. The sending stops when the limit is
    /// reached. Any non-negative 64-bit integer is acceptable. Setting to 0 means no limit. If the request
    /// doesn't contain the field or contains null, the limit value doesn't change.
    /// Default: `0`.
    #[serde(
        rename = "email_counter_limit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter_limit: Option<i64>,
    /// Sets the email counter mode. It allows you to choose whether the counter should reset automatically,
    /// or remain permanent. The following values are acceptable: 'default' - the counter resets to zero
    /// automatically at the beginning of each accounting period, 'permanent' - the counter never resets
    /// automatically. The reset can be done only manually by the user in the request with the passes 0
    /// value in the email_counter field. If the request doesn't contain the field or contains a null value,
    /// the limit value doesn't change.
    /// Default: `default`.
    #[serde(
        rename = "email_counter_mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter_mode: Option<crate::common::model::EmailCounterMode>,
}

/// ProjectUpdateResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectUpdateResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// API key of the project. Returned only if the project to update was identified by project_api_key.
    /// Not returned in case of updating by project_id.
    #[serde(
        rename = "project_api_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_api_key: Option<String>,
}

/// ProjectListRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectListRequest {
    /// Optional possibility to specify project_id to include only this project to list.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Optional possibility to specify project_api_key to include only this project to list.
    #[serde(
        rename = "project_api_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_api_key: Option<String>,
}

/// ProjectListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectListResponse {
    /// "success" string
    /// Allowed values: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Array of objects where each object represents one project.
    #[serde(rename = "projects", default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectListObject>>,
}

/// ProjectListObject
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectListObject {
    /// Unqiue project identifier, ASCII string up to 36 characters long.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// API key of the project. You can use it instead of the user API key parameter in all methods except
    /// project/* methods.
    #[serde(rename = "api_key", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Project name, unique for user account.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ISO-3166 alpha-2 country code. If set, UniOne treats project personal data according to country
    /// laws, e.g. GDPR for european countries.
    #[serde(rename = "country", default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Date and time of project creation in UTC timezone in "YYYY-MM-DD hh:mm:ss" format.
    #[serde(rename = "reg_time", default, skip_serializing_if = "Option::is_none")]
    pub reg_time: Option<String>,
    /// Whether email sending is enabled for this project or not.
    /// Default: `True`.
    #[serde(
        rename = "send_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_enabled: Option<bool>,
    /// If it's false then UniOne adds default unsubscribe footer to each email sent with API key of the
    /// project. If it's true then appending default unsubscribe footer for this project is avoided and
    /// sending with custom unsubscribe url or even without unsubscribe url is permitted for the project.
    /// Please note that custom_unsubscribe_url_enabled=true is available only if removing unsubscribe link
    /// is approved for the user account by support. More on this
    /// [here](https://docs.unione.io/en/unsubscribe-link). When custom_unsubscribe_url_enabled is skipped
    /// on creating a project, it's value is taken from user
    #[serde(
        rename = "custom_unsubscribe_url_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_unsubscribe_url_enabled: Option<bool>,
    /// A unique domain identifier which will determine the [tracking
    /// domain](https://docs.unione.io/en/tracking-domains) or [dedicated IP](/dedicated-ip) pool to be used
    /// by default. If the value is not specified, a default backend domain will be assigned for your
    /// account or project instead.
    #[serde(
        rename = "backend_domain_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_domain_id: Option<i64>,
    /// Project's email counter value. It increases on every email sent.
    #[serde(
        rename = "email_counter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter: Option<i64>,
    /// Allows you to limit the number of emails sent by the project. The sending stops when the limit is
    /// reached. Any non-negative 64-bit integer is acceptable. 0 means no limit.
    #[serde(
        rename = "email_counter_limit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter_limit: Option<i64>,
    /// 'default' - the counter resets to zero automatically at the beginning of each accounting period.
    /// 'permanent' - the counter never resets automatically.
    /// Default: `default`.
    #[serde(
        rename = "email_counter_mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter_mode: Option<crate::common::model::EmailCounterMode>,
    /// A unique identifier that defines the default unsubscribe page. If absent, the account/project will
    /// be assigned the system default unsubscribe page.
    #[serde(
        rename = "unsubscribe_page_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribe_page_id: Option<i64>,
}

/// ProjectDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectDeleteRequest {
    /// Unique project identifier. You should identify project to delete either with project_id or with
    /// project_api_key field. We recommend usage of project_id as a more secure option.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Optional project API key. You should identify project to delete either with project_id or with
    /// project_api_key field. We recommend usage of project_id as a more secure option.
    #[serde(
        rename = "project_api_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_api_key: Option<String>,
}
