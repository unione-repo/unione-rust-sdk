//! Obsolete section: ObsoleteApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Obsolete methods dealing with unsubscribes. We recommend using the more flexible
/// [suppression](https://docs.unione.io/en/web-api-ref#suppression) methods instead. Note that you
/// can't resubscribe an unsubscribed recipient using WebAPI, but you can send him an email with
/// subscription link using email/subscribe method.
#[derive(Debug, Clone, Copy)]
pub struct ObsoleteApi<'a> {
    client: &'a ApiClient,
}

impl<'a> ObsoleteApi<'a> {
    /// Creates the Obsolete API.
    pub fn new(client: &'a ApiClient) -> Self {
        ObsoleteApi { client }
    }

    /// Registers a new unsubscribed email.
    ///
    /// After registering an email as unsubscribed all future attempts to send a message to this email will
    /// be rejected. Please note that there's no way to remove an email from unsubscribed thru the API. But
    /// you can send an email with a link to resubscribe using
    /// [email/subscribe](https://docs.unione.io/en/web-api-ref#email-subscribe) method or remove
    /// "unsubscribed" status manually on [Email Search page](https://cp.unione.io/en/user/email) in control
    /// panel (but only a limited number of unsubscribed email per day can be removed).
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#unsubscribed-set>
    pub async fn unsubscribed_set(
        &self,
        request: &model::ObsoleteUnsubscribedSetRequest,
    ) -> Result<model::ObsoleteUnsubscribedSetResponse, ApiError> {
        self.client
            .call("/unsubscribed/set.json", serde_json::to_value(request)?)
            .await
    }

    /// Checks if an email is unsubscribed.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#unsubscribed-check>
    pub async fn unsubscribed_check(
        &self,
        request: &model::ObsoleteUnsubscribedCheckRequest,
    ) -> Result<model::ObsoleteUnsubscribedCheckResponse, ApiError> {
        self.client
            .call("/unsubscribed/check.json", serde_json::to_value(request)?)
            .await
    }

    /// Returns a list of all unsubscribed emails since provided date.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#unsubscribed-list>
    pub async fn unsubscribed_list(
        &self,
        request: &model::ObsoleteUnsubscribedListRequest,
    ) -> Result<model::ObsoleteUnsubscribedListResponse, ApiError> {
        self.client
            .call("/unsubscribed/list.json", serde_json::to_value(request)?)
            .await
    }
}
