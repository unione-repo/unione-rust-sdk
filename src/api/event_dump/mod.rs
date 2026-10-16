//! EventDump section: EventDumpApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// These methods are used to obtain a list of events for a given period of time. The data is stored for
/// up to 45 days, depending on your tariff, i. e. you may fetch all event data starting from 00:00:00
/// 31 days ago and up to current time. For custom tarifs with big sending volumes event dumps are
/// available only for the last 7 days.
#[derive(Debug, Clone, Copy)]
pub struct EventDumpApi<'a> {
    client: &'a ApiClient,
}

impl<'a> EventDumpApi<'a> {
    /// Creates the EventDump API.
    pub fn new(client: &'a ApiClient) -> Self {
        EventDumpApi { client }
    }

    /// This asynchronous method initiates the preparation of data dump in CSV format. The number of dump
    /// files created or stored is limited to 10; to order more files, you have to delete one of the
    /// previous ones or wait until they will be deleted automatically. A newly created dump file is kept
    /// for 8 hours. If the request period is more than a day or there are too many events then several
    /// files will be generated and each file will contain no more than 100,000 events. File contains
    /// columns with the following information - 'event_time', 'job_id', 'from_email', 'email', 'status',
    /// 'delivery_status', 'metadata', 'destination_response', 'user_agent', 'url', 'ip', 'tags' and
    /// 'project_id' (for accounts with projects enabled). You can change the fields and include 'subject'
    /// field using 'dump_fileds' parameter.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#event-dump-create>
    pub async fn create(
        &self,
        request: &model::EventDumpCreateRequest,
    ) -> Result<model::EventDumpCreateResponse, ApiError> {
        self.client
            .call("/event-dump/create.json", serde_json::to_value(request)?)
            .await
    }

    /// Returns dump properties for a given identifier
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#event-dump-get>
    pub async fn get(
        &self,
        request: &model::EventDumpGetRequest,
    ) -> Result<model::EventDumpGetResponse, ApiError> {
        self.client
            .call("/event-dump/get.json", serde_json::to_value(request)?)
            .await
    }

    /// This method does not require any parameters; returns the full list of existing dumps.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#event-dump-list>
    pub async fn list(&self) -> Result<model::EventDumpListResponse, ApiError> {
        self.client
            .call("/event-dump/list.json", serde_json::Value::Null)
            .await
    }

    /// Removes dump from queue or storage.
    ///
    /// Documentation: <https://docs.unione.io/en/web-api-ref#event-dump-delete>
    pub async fn delete(
        &self,
        request: &model::EventDumpDeleteRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/event-dump/delete.json", serde_json::to_value(request)?)
            .await
    }
}
