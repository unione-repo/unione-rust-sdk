//! System section: SystemApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// System operations.
#[derive(Debug, Clone, Copy)]
pub struct SystemApi<'a> {
    client: &'a ApiClient,
}

impl<'a> SystemApi<'a> {
    /// Creates the System API.
    pub fn new(client: &'a ApiClient) -> Self {
        SystemApi { client }
    }

    /// Checks API key validity and returns user ID.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#system-ping>
    pub async fn ping(&self) -> Result<model::SystemPingResponse, ApiError> {
        self.client
            .call("/system/ping.json", serde_json::Value::Null)
            .await
    }

    /// Gets user or project info by API key.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#system-info>
    pub async fn info(&self) -> Result<model::SystemInfoResponse, ApiError> {
        self.client
            .call("/system/info.json", serde_json::Value::Null)
            .await
    }
}
