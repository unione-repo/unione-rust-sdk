//! Template section: TemplateApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// You have the option of storing a template on the UniOne side and then using it by id, which is
/// highly recommended for sending repetitive messages or high volume traffic. UniOne supports two
/// different [template engines](https://docs.unione.io/en/template-engines).
#[derive(Debug, Clone, Copy)]
pub struct TemplateApi<'a> {
    client: &'a ApiClient,
}

impl<'a> TemplateApi<'a> {
    /// Creates the Template API.
    pub fn new(client: &'a ApiClient) -> Self {
        TemplateApi { client }
    }

    /// Creates or updates an email template.
    ///
    /// Restrictions:
    /// * The maximum request size is 10 MB.
    /// * The number of templates is limited to 10000.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#template-set>
    pub async fn set(
        &self,
        request: &model::TemplateSetRequest,
    ) -> Result<model::TemplateSetResponse, ApiError> {
        self.client
            .call("/template/set.json", serde_json::to_value(request)?)
            .await
    }

    /// Gets template properties by it's id.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#template-get>
    pub async fn get(
        &self,
        request: &model::TemplateGetRequest,
    ) -> Result<model::TemplateGetResponse, ApiError> {
        self.client
            .call("/template/get.json", serde_json::to_value(request)?)
            .await
    }

    /// Returns the list of all or some templates of the account.
    ///
    /// You can iterate thru large list using "offset" and "limit" parameters. When less than "limit"
    /// templates are returned then the end of list is reached.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#template-list>
    pub async fn list(
        &self,
        request: &model::TemplateListRequest,
    ) -> Result<model::TemplateListResponse, ApiError> {
        self.client
            .call("/template/list.json", serde_json::to_value(request)?)
            .await
    }

    /// Deletes a template by id.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#template-delete>
    pub async fn delete(
        &self,
        request: &model::TemplateDeleteRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/template/delete.json", serde_json::to_value(request)?)
            .await
    }
}
