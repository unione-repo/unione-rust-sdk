# EventDump

[Back to the list of sections](../README.md)

These methods are used to obtain a list of events for a given period of time. The data is stored for up to 45 days, depending on your tariff, i. e. you may fetch all event data starting from 00:00:00 31 days ago and up to current time. For custom tarifs with big sending volumes event dumps are available only for the last 7 days.

`unione::api::event_dump::EventDumpApi`, `client.event_dump()`

| Method | Endpoint | Description |
|---|---|---|
| [`create`](#create) | `POST /event-dump/create.json` | This asynchronous method initiates the preparation of data dump in CSV format. The number of dump files created or stored is limited to 10; to order more files, you have to delete one of the previous ones or wait until they will be deleted automatically. A newly created dump file is kept for 8 hours. If the request period is more than a day or there are too many events then several files will be generated and each file will contain no more than 100,000 events. File contains columns with the following information - 'event_time', 'job_id', 'from_email', 'email', 'status', 'delivery_status', 'metadata', 'destination_response', 'user_agent', 'url', 'ip', 'tags' and 'project_id' (for accounts with projects enabled). You can change the fields and include 'subject' field using 'dump_fileds' parameter. |
| [`get`](#get) | `POST /event-dump/get.json` | Returns dump properties for a given identifier |
| [`list`](#list) | `POST /event-dump/list.json` | This method does not require any parameters; returns the full list of existing dumps. |
| [`delete`](#delete) | `POST /event-dump/delete.json` | Removes dump from queue or storage. |

## create

`POST /event-dump/create.json` · [Documentation](https://docs.unione.io/en/web-api-ref#event-dump-create)

This asynchronous method initiates the preparation of data dump in CSV format. The number of dump files created or stored is limited to 10; to order more files, you have to delete one of the previous ones or wait until they will be deleted automatically. A newly created dump file is kept for 8 hours. If the request period is more than a day or there are too many events then several files will be generated and each file will contain no more than 100,000 events. File contains columns with the following information - 'event_time', 'job_id', 'from_email', 'email', 'status', 'delivery_status', 'metadata', 'destination_response', 'user_agent', 'url', 'ip', 'tags' and 'project_id' (for accounts with projects enabled). You can change the fields and include 'subject' field using 'dump_fileds' parameter.

### Request: `EventDumpCreateRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `start_time` | `String` | yes | Date and time in YYYY-MM-DD hh:mm:ss format, specifying period start time. Data is stored for up to 32 days, depending on your tariff. |
| `end_time` | `String` |  | Date and time in YYYY-MM-DD hh:mm:ss format, specifying period end time (non-inclusive). |
| `limit` | `i64` |  | Maximum number of events returned (default is 50). If this value is over 100 000, several files will be created, each having 100 000 events maximum. |
| `all_projects` | `bool` |  | For accounts with projects enabled, the value all_projects=true allows to fetch data for all existing projects. |
| `filter` | `FilterObject` |  | An object with the properties of the event dump filter. |
| `dump_fields` | `Vec<String>` |  | An array of strings containing the names of columns you need in your report, in your preferred order. If the 'dump_fields' array is missing or empty, a default field list and order will be applied. Allowed values, in their default order, are - 'project_id', 'event_time', 'job_id', 'from_email', 'email', 'status', 'delivery_status', 'metadata', 'destination_response', 'user_agent', 'url', 'ip', 'tags'. For maximum security, the 'subject' field is not included by default, but you can add it by explicitly including in the 'dump_fields' array. |
| `aggregate` | `String` |  | Allows you to obtain aggregated statistics for a given time period. The two allowed values are:<br><br>- __day_status__ - fetch aggregated statistics (the filter value must be missing or empty, as filters are not allowed with this option);<br>- __an empty string__ - used by default, event_dump/create works as if the aggregate parameter is omitted. |
| `delimiter` | `String` |  | Field delimiter, can be set to either  ',' or ';', defaults to  ','. |
| `format` | `String` |  | File format, either “csv” (default) or “csv_gzip”. |

### Response: `EventDumpCreateResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `dump_id` | `String` | Dump identifier, used to call the [event-dump/get](https://docs.unione.io/en/web-api-ref#event-dump-get) method. |

### Example

```rust
use unione::api::event_dump::model::EventDumpCreateRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .event_dump()
        .create(&EventDumpCreateRequest {
            start_time: "2022-07-20 00:00:00".to_string(),
            end_time: Some("2022-07-21 00:00:00".to_string()),
            limit: Some(50),
            dump_fields: Some(vec!["string1".to_string()]),
            aggregate: Some("day_status".to_string()),
            delimiter: Some(";".to_string()),
            format: Some("csv".to_string()),
            ..Default::default()
        })
        .await?;
    println!("{:?}", response.dump_id);
    Ok(())
}
```

## get

`POST /event-dump/get.json` · [Documentation](https://docs.unione.io/en/web-api-ref#event-dump-get)

Returns dump properties for a given identifier

### Request: `EventDumpGetRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `dump_id` | `String` | yes | Dump identifier received by the method [event-dump/create](https://docs.unione.io/en/web-api-ref#event-dump-create). |

### Response: `EventDumpGetResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `event_dump` | `EventDumpObject` | An object with the properties of the event dump. |

### Example

```rust
use unione::api::event_dump::model::EventDumpGetRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .event_dump()
        .get(&EventDumpGetRequest {
            dump_id: "Gqfasjh34tlasd".to_string(),
        })
        .await?;
    println!("{:?}", response.event_dump);
    Ok(())
}
```

## list

`POST /event-dump/list.json` · [Documentation](https://docs.unione.io/en/web-api-ref#event-dump-list)

This method does not require any parameters; returns the full list of existing dumps.

### Request

The method has no parameters.

### Response: `EventDumpListResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `event_dumps` | `Vec<EventDumpObject>` | An array of objects describing event dumps. Each object is similar to 'event_dump' object returned by the [event-dump/get](https://docs.unione.io/en/web-api-ref#event-dump-get) method. |

### Example

```rust
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client.event_dump().list().await?;
    println!("{:?}", response.event_dumps);
    Ok(())
}
```

## delete

`POST /event-dump/delete.json` · [Documentation](https://docs.unione.io/en/web-api-ref#event-dump-delete)

Removes dump from queue or storage.

### Request: `EventDumpDeleteRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `dump_id` | `String` | yes | Dump identifier, obtained via the [event-dump/create](https://docs.unione.io/en/web-api-ref#event-dump-create) method. |

### Response: `SuccessResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |

### Example

```rust
use unione::api::event_dump::model::EventDumpDeleteRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .event_dump()
        .delete(&EventDumpDeleteRequest {
            dump_id: "Gqfasjh34tlasd".to_string(),
        })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```
