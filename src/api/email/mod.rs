//! Email section: EmailApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// This group of methods is related with email sending.
#[derive(Debug, Clone, Copy)]
pub struct EmailApi<'a> {
    client: &'a ApiClient,
}

impl<'a> EmailApi<'a> {
    /// Creates the Email API.
    pub fn new(client: &'a ApiClient) -> Self {
        EmailApi { client }
    }

    /// Sends an email or a bunch of emails
    ///
    /// While sending you can provide substitutions (merge tags), use a template, turn on read or click
    /// tracking, etc.
    ///
    /// Restrictions:
    /// * maximum request size is 10 MB
    /// * maximum number of recipients is 500
    /// * The symbol '/' is not allowed in attachment names
    ///
    /// If the list of recipients contains invalid, non-existent, duplicate, or unsubscribed emails, those
    /// emails will be returned in the output parameter failed_emails, and the email will be sent to all
    /// other valid emails addresses. If all the emails addresses fall into failed_emails, sending will not
    /// be carried out and API error 204 will be returned.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#email-send>
    pub async fn send(
        &self,
        request: &model::EmailSendRequest,
    ) -> Result<model::EmailSendResponse, ApiError> {
        self.client
            .call("/email/send.json", serde_json::to_value(request)?)
            .await
    }

    /// Sends an email with (re)subscribe link.
    ///
    /// This method allows subscription renewal by a former subscriber who has previously unsubscribed, or
    /// who is blocked because of complaints. This method sends an email with a new subscription link to a
    /// former subscriber. Once the recipient clicks the link, their subscription status is restored.
    /// There is a default restriction that limits this method to being used once a day. If you need to
    /// resubscribe more often, please ask support about it.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#email-subscribe>
    pub async fn subscribe(
        &self,
        request: &model::EmailSubscribeRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/email/subscribe.json", serde_json::to_value(request)?)
            .await
    }
}
