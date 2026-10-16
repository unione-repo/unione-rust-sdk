//! Suppression section: SuppressionApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Methods for managing suppression lists. An email can be added to suppression list because of
/// unsubscription, bounce or complain.
#[derive(Debug, Clone, Copy)]
pub struct SuppressionApi<'a> {
    client: &'a ApiClient,
}

impl<'a> SuppressionApi<'a> {
    /// Creates the Suppression API.
    pub fn new(client: &'a ApiClient) -> Self {
        SuppressionApi { client }
    }

    /// Adds an email address to the suppression list. You can always remove this address from the
    /// suppression list later using the
    /// [suppression/delete](https://docs.unione.io/en/web-api-ref#suppression-delete) method.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#suppression-set>
    pub async fn set(
        &self,
        request: &model::SuppressionSetRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/suppression/set.json", serde_json::to_value(request)?)
            .await
    }

    /// Gets a reason and date of email suppression.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#suppression-get>
    pub async fn get(
        &self,
        request: &model::SuppressionGetRequest,
    ) -> Result<model::SuppressionGetResponse, ApiError> {
        self.client
            .call("/suppression/get.json", serde_json::to_value(request)?)
            .await
    }

    /// Returns a suppression list since provided date.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#suppression-list>
    pub async fn list(
        &self,
        request: &model::SuppressionListRequest,
    ) -> Result<model::SuppressionListResponse, ApiError> {
        self.client
            .call("/suppression/list.json", serde_json::to_value(request)?)
            .await
    }

    /// Deletes an email from suppression list (only records with is_deletable=true). If there are no
    /// entries of this email in the suppression list, API error 3003 is returned. If such entries exist but
    /// no one can be deleted, API error 3004 is returned. If daily limit of deletes exceeded, API error 906
    /// is returned.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#suppression-delete>
    pub async fn delete(
        &self,
        request: &model::SuppressionDeleteRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/suppression/delete.json", serde_json::to_value(request)?)
            .await
    }
}
