//! UniOne Web API client.
//!
//! ```ignore
//! use std::collections::HashMap;
//! use unione::api::email::model::EmailSendRequest;
//! use unione::api::email::model::MessageObject;
//! use unione::api::email::model::MessageObjectRecipientsItem;
//! use unione::common::model::BodyObject;
//! use unione::common::model::TemplateEngine;
//! use unione::UniOneClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), unione::ApiError> {
//!     let client = UniOneClient::new("YOUR_API_KEY");
//!
//!     let response = client
//!         .email()
//!         .send(&EmailSendRequest {
//!             message: MessageObject {
//!                 recipients: vec![
//!                     MessageObjectRecipientsItem {
//!                         email: "user@example.com".to_string(),
//!                         substitutions: Some(HashMap::from([
//!                             ("CustomerId".to_string(), "12452".to_string()),
//!                             ("to_name".to_string(), "John Smith".to_string()),
//!                         ])),
//!                         metadata: Some(HashMap::from([
//!                             ("campaign_id".to_string(), "c77f4f4e-3561-49f7-9f07-c35be01b4f43".to_string()),
//!                             ("customer_hash".to_string(), "b253ac7".to_string()),
//!                         ])),
//!                     },
//!                 ],
//!                 template_id: Some("00000000-0000-0000-0000-000000000000".to_string()),
//!                 global_language: Some("en".to_string()),
//!                 template_engine: Some(TemplateEngine::Velocity),
//!                 body: BodyObject {
//!                     html: Some("<b>Hello, {{to_name}}</b>".to_string()),
//!                     plaintext: Some("Hello, {{to_name}}".to_string()),
//!                     amp: Some("<!doctype html><html amp4email><head> <meta charset=\"utf-8\"><script async src=\"https://cdn.ampproject.org/v0.js\"></script> <style amp4email-boilerplate>body{visibility:hidden}</style></head><body> Hello, AMP4EMAIL world.</body></html>".to_string()),
//!                 },
//!                 subject: "UniOne test email ".to_string(),
//!                 from_email: Some("email@example.com".to_string()),
//!                 from_name: Some("John Smith".to_string()),
//!                 reply_to: Some("email@example.com".to_string()),
//!                 reply_to_name: Some("John Smith".to_string()),
//!                 idempotence_key: Some("SG1VsbG68sIH2dvc5mx890kIQ".to_string()),
//!                 ..Default::default()
//!             },
//!         })
//!         .await?;
//!     println!("{:?}", response.job_id);
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod client;
pub mod common;

pub use client::{ApiClient, ApiError, Configuration};
pub use serde_json;

/// Entry point of the UniOne API: one accessor per section.
#[derive(Debug, Clone)]
pub struct UniOneClient {
    api_client: ApiClient,
}

impl UniOneClient {
    /// Client with the default configuration.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_config(Configuration::new(api_key))
    }

    /// Client for an API host (`client::HOST_US1`, "us1.unione.io") or a full base URL.
    pub fn with_host(api_key: impl Into<String>, host: &str) -> Self {
        let mut config = Configuration::new(api_key);
        config.base_url = client::base_url_for(host);
        Self::with_config(config)
    }

    /// Client with a custom configuration.
    pub fn with_config(config: Configuration) -> Self {
        Self::from_api_client(ApiClient::new(config))
    }

    /// Client on a custom transport, e.g. with a custom reqwest client (proxy, TLS):
    /// `ApiClient::with_http_client(config, http)`.
    pub fn from_api_client(api_client: ApiClient) -> Self {
        UniOneClient { api_client }
    }

    /// Low level transport.
    pub fn api_client(&self) -> &ApiClient {
        &self.api_client
    }

    /// This group of methods is related with email sending.
    pub fn email(&self) -> api::email::EmailApi<'_> {
        api::email::EmailApi::new(&self.api_client)
    }

    /// Email validation methods. As a result of the check, you’ll get detailed information on email
    /// validity. It will help eliminate nonexistent emails, decrease the number of complaints, clean up the
    /// subscribers list and eventually enhance your sender reputation score. For example, you may add
    /// validation to a subscription form on your site to let users know they’ve just entered an invalid
    /// address.
    pub fn email_validation(&self) -> api::email_validation::EmailValidationApi<'_> {
        api::email_validation::EmailValidationApi::new(&self.api_client)
    }

    /// Manage email addresses confirmed for sandbox-domain sending. These methods are available only with a
    /// main account API key.
    pub fn confirmed_email(&self) -> api::confirmed_email::ConfirmedEmailApi<'_> {
        api::confirmed_email::ConfirmedEmailApi::new(&self.api_client)
    }

    /// You have the option of storing a template on the UniOne side and then using it by id, which is
    /// highly recommended for sending repetitive messages or high volume traffic. UniOne supports two
    /// different [template engines](https://docs.unione.io/en/template-engines).
    pub fn template(&self) -> api::template::TemplateApi<'_> {
        api::template::TemplateApi::new(&self.api_client)
    }

    /// UniOne lets you recieve notifications about email status changes or spam blocks. You provide an URL
    /// and every time a watched event occurs we will notify you by submitting a request with JSON data to
    /// this URL.
    pub fn webhook(&self) -> api::webhook::WebhookApi<'_> {
        api::webhook::WebhookApi::new(&self.api_client)
    }

    /// Methods for managing suppression lists. An email can be added to suppression list because of
    /// unsubscription, bounce or complain.
    pub fn suppression(&self) -> api::suppression::SuppressionApi<'_> {
        api::suppression::SuppressionApi::new(&self.api_client)
    }

    /// You can use these methods to setup sender domains and check their statuses. They might be useful in
    /// case you want to automate adding domains for your own clients.
    pub fn domain(&self) -> api::domain::DomainApi<'_> {
        api::domain::DomainApi::new(&self.api_client)
    }

    /// These methods are used to obtain a list of events for a given period of time. The data is stored for
    /// up to 45 days, depending on your tariff, i. e. you may fetch all event data starting from 00:00:00
    /// 31 days ago and up to current time. For custom tarifs with big sending volumes event dumps are
    /// available only for the last 7 days.
    pub fn event_dump(&self) -> api::event_dump::EventDumpApi<'_> {
        api::event_dump::EventDumpApi::new(&self.api_client)
    }

    /// This family of methods is used to manage user-defined tags. Tags are used to categorize letters
    /// based on your own criteria, e. g. to create a distinction between marketing and transactional
    /// letters or group together all notifications for a particular order type. UniOne allows you to assign
    /// tags to all outgoing emails, and the tags you’ve set may then be used to filter and analyze delivery
    /// statistics.
    pub fn tag(&self) -> api::tag::TagApi<'_> {
        api::tag::TagApi::new(&self.api_client)
    }

    /// Projects allow you to group several independent domains, webhooks, templates, and unsubscription
    /// lists together. It's very useful if you have your own clients and manage their data from a single
    /// UniOne account. Each project receives its own API key, and all API calls with that API key only deal
    /// with the data of that project. This way, you can assign a separate project for each client and be
    /// sure that they won't conflict; e.g. unsubscribing from one project won't affect another.
    pub fn project(&self) -> api::project::ProjectApi<'_> {
        api::project::ProjectApi::new(&self.api_client)
    }

    /// System operations.
    pub fn system(&self) -> api::system::SystemApi<'_> {
        api::system::SystemApi::new(&self.api_client)
    }

    /// Obsolete methods dealing with unsubscribes. We recommend using the more flexible
    /// [suppression](https://docs.unione.io/en/web-api-ref#suppression) methods instead. Note that you
    /// can't resubscribe an unsubscribed recipient using WebAPI, but you can send him an email with
    /// subscription link using email/subscribe method.
    pub fn obsolete(&self) -> api::obsolete::ObsoleteApi<'_> {
        api::obsolete::ObsoleteApi::new(&self.api_client)
    }
}
