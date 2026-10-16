//! Webhook section: WebhookApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// UniOne lets you recieve notifications about email status changes or spam blocks. You provide an URL
/// and every time a watched event occurs we will notify you by submitting a request with JSON data to
/// this URL.
///
/// You can configure the format of webhook calls (JSON or gzipped JSON), which events you want to be
/// notified of, whether or not you let us groups several events in one request, do you want to receive
/// extra details like SMTP answer or recipient's user agent, etc.
///
/// Please note that your webhook handler should be able to process at least 5 parallel requests, up to
/// 100 events each (but you can configure the maximum number of parallel connections up to 100, which
/// is recommended).
///
/// If the URL of your webhook handler is not available (the response is not HTTP 200 OK within 3
/// seconds timeout), the system will repeat requests each 10 minutes during 24 hours, passing extra
/// parameter retry_count. If there were at least 10 events during last 24 hours and all the webhook
/// requests about these events have failed then we consider webhook handler broken and change it's
/// status to inactive automatically. A notification about that is sent to an email [configured in
/// control panel](https://cp.unione.io/en/user/alerting/technical/).
///
/// The format of webhook request and details of each event are described
/// [here](https://docs.unione.io/en/web-api-ref#callback-format).
#[derive(Debug, Clone, Copy)]
pub struct WebhookApi<'a> {
    client: &'a ApiClient,
}

impl<'a> WebhookApi<'a> {
    /// Creates the Webhook API.
    pub fn new(client: &'a ApiClient) -> Self {
        WebhookApi { client }
    }

    /// Sets or edits a [webhook](https://docs.unione.io/en/web-api-ref#webhook), i.e. an event notification
    /// handler.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#webhook-set>
    pub async fn set(
        &self,
        request: &model::WebhookSetRequest,
    ) -> Result<model::WebhookSetResponse, ApiError> {
        self.client
            .call("/webhook/set.json", serde_json::to_value(request)?)
            .await
    }

    /// Gets properties of a [webhook](https://docs.unione.io/en/web-api-ref#webhook).
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#webhook-get>
    pub async fn get(
        &self,
        request: &model::WebhookGetRequest,
    ) -> Result<model::WebhookGetResponse, ApiError> {
        self.client
            .call("/webhook/get.json", serde_json::to_value(request)?)
            .await
    }

    /// List all or some [webhooks](https://docs.unione.io/en/web-api-ref#webhook) (event notification
    /// handlers) of a user or a project.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#webhook-list>
    pub async fn list(
        &self,
        request: &model::WebhookListRequest,
    ) -> Result<model::WebhookListResponse, ApiError> {
        self.client
            .call("/webhook/list.json", serde_json::to_value(request)?)
            .await
    }

    /// Deletes an event notification handler
    ///
    /// You can also temporarily deactivate webhook without deleting it by setting it's status to "disabled"
    /// in [webhook/set](https://docs.unione.io/en/web-api-ref#webhook-set) method.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#webhook-delete>
    pub async fn delete(
        &self,
        request: &model::WebhookDeleteRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/webhook/delete.json", serde_json::to_value(request)?)
            .await
    }

    /// Webhook callback format description.
    pub fn parse_callback(body: &str) -> Result<model::WebhookCallback, ApiError> {
        Ok(serde_json::from_str(body)?)
    }
}
