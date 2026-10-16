//! Tag section: TagApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// This family of methods is used to manage user-defined tags. Tags are used to categorize letters
/// based on your own criteria, e. g. to create a distinction between marketing and transactional
/// letters or group together all notifications for a particular order type. UniOne allows you to assign
/// tags to all outgoing emails, and the tags you’ve set may then be used to filter and analyze delivery
/// statistics.
#[derive(Debug, Clone, Copy)]
pub struct TagApi<'a> {
    client: &'a ApiClient,
}

impl<'a> TagApi<'a> {
    /// Creates the Tag API.
    pub fn new(client: &'a ApiClient) -> Self {
        TagApi { client }
    }

    /// This method returns the full list of user-defined tags. Does not receive any parameters.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#tag-list>
    pub async fn list(&self) -> Result<model::TagListResponse, ApiError> {
        self.client
            .call("/tag/list.json", serde_json::Value::Null)
            .await
    }

    /// Removes tag.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#tag-delete>
    pub async fn delete(
        &self,
        request: &model::TagDeleteRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/tag/delete.json", serde_json::to_value(request)?)
            .await
    }
}
