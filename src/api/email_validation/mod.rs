//! EmailValidation section: EmailValidationApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Email validation methods. As a result of the check, you’ll get detailed information on email
/// validity. It will help eliminate nonexistent emails, decrease the number of complaints, clean up the
/// subscribers list and eventually enhance your sender reputation score. For example, you may add
/// validation to a subscription form on your site to let users know they’ve just entered an invalid
/// address.
#[derive(Debug, Clone, Copy)]
pub struct EmailValidationApi<'a> {
    client: &'a ApiClient,
}

impl<'a> EmailValidationApi<'a> {
    /// Creates the EmailValidation API.
    pub fn new(client: &'a ApiClient) -> Self {
        EmailValidationApi { client }
    }

    /// A maximum of two concurrent requests are allowed; if you send more requests, an error will be
    /// returned. Since the verification process should be completed promptly, we've set a time limit on
    /// email check. If we encounter a slow email server that does not respond in 5 seconds, you may get an
    /// “unknown” status.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#email-validation-single>
    pub async fn single(
        &self,
        request: &model::EmailValidationSingleRequest,
    ) -> Result<model::EmailValidationSingleResponse, ApiError> {
        self.client
            .call(
                "/email-validation/single.json",
                serde_json::to_value(request)?,
            )
            .await
    }
}
