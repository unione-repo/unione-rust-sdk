//! Project section: ProjectApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Projects allow you to group several independent domains, webhooks, templates, and unsubscription
/// lists together. It's very useful if you have your own clients and manage their data from a single
/// UniOne account. Each project receives its own API key, and all API calls with that API key only deal
/// with the data of that project. This way, you can assign a separate project for each client and be
/// sure that they won't conflict; e.g. unsubscribing from one project won't affect another.
///
/// Projects methods are disabled by default due to antispam measures. To enable them, [contact
/// support](https://cp.unione.io/en/support).
///
/// Please note that sending domains are confirmed and used on per-project basis by default. But it's
/// possible to allow projects to send emails from domains confirmed for main account - just [ask
/// support](https://cp.unione.io/en/support) to enable it.
#[derive(Debug, Clone, Copy)]
pub struct ProjectApi<'a> {
    client: &'a ApiClient,
}

impl<'a> ProjectApi<'a> {
    /// Creates the Project API.
    pub fn new(client: &'a ApiClient) -> Self {
        ProjectApi { client }
    }

    /// Creates a new project.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#project-create>
    pub async fn create(
        &self,
        request: &model::ProjectCreateRequest,
    ) -> Result<model::ProjectCreateResponse, ApiError> {
        self.client
            .call("/project/create.json", serde_json::to_value(request)?)
            .await
    }

    /// Updates a project.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#project-update>
    pub async fn update(
        &self,
        request: &model::ProjectUpdateRequest,
    ) -> Result<model::ProjectUpdateResponse, ApiError> {
        self.client
            .call("/project/update.json", serde_json::to_value(request)?)
            .await
    }

    /// Lists projects.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#project-list>
    pub async fn list(
        &self,
        request: &model::ProjectListRequest,
    ) -> Result<model::ProjectListResponse, ApiError> {
        self.client
            .call("/project/list.json", serde_json::to_value(request)?)
            .await
    }

    /// Deletes a project.
    ///
    /// The project will be moved to the "trash bin" before being permanently deleted. You can restore the
    /// project by contacting support.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#project-delete>
    pub async fn delete(
        &self,
        request: &model::ProjectDeleteRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/project/delete.json", serde_json::to_value(request)?)
            .await
    }
}
