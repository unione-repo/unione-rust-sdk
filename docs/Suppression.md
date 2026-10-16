# Suppression

[Back to the list of sections](../README.md)

Methods for managing suppression lists. An email can be added to suppression list because of unsubscription, bounce or complain.

`unione::api::suppression::SuppressionApi`, `client.suppression()`

| Method | Endpoint | Description |
|---|---|---|
| [`set`](#set) | `POST /suppression/set.json` | Adds an email address to the suppression list. You can always remove this address from the suppression list later using the [suppression/delete](https://docs.unione.io/en/web-api-ref#suppression-delete) method. |
| [`get`](#get) | `POST /suppression/get.json` | Gets a reason and date of email suppression. |
| [`list`](#list) | `POST /suppression/list.json` | Returns a suppression list since provided date. |
| [`delete`](#delete) | `POST /suppression/delete.json` | Deletes an email from suppression list (only records with is_deletable=true). If there are no entries of this email in the suppression list, API error 3003 is returned. If such entries exist but no one can be deleted, API error 3004 is returned. If daily limit of deletes exceeded, API error 906 is returned. |

## set

`POST /suppression/set.json` · [Documentation](https://docs.unione.io/en/web-api-ref#suppression-set)

Adds an email address to the suppression list. You can always remove this address from the suppression list later using the [suppression/delete](https://docs.unione.io/en/web-api-ref#suppression-delete) method.

### Request: `SuppressionSetRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `email` | `String` | yes | Email to add in suppression list. |
| `cause` | `String` | yes | Cause of email being suppressed. One of:<br><br>- __unsubscribed__ - email is unsubscribed;<br>- __temporary_unavailable__ - the email address is unavailable. This means that over the next three days sending to this address will return an error. Email may be temporarily unavailable due to several reasons, e.g.:<br><br>- a previous email has been rejected by the recipient's server for spam;<br>- the recipient's mailbox is full or is not used;<br>- recipient's domain does not accept mail;<br>- sending server was rejected due to blacklisting;<br><br>- __permanent_unavailable__ - the email address is permanently unavailable due to multiple hard bounces;<br>- __complained__ -  the recipient reported spam in the previous emails; |
| `created` | `String` |  | When suppression was created, in UTC timezone in "YYYY-MM-DD hh:mm:ss" format. |

### Response: `SuccessResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |

### Example

```rust
use unione::api::suppression::model::SuppressionSetRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .suppression()
        .set(&SuppressionSetRequest {
            email: "email@example.com".to_string(),
            cause: "unsubscribed".to_string(),
            created: Some("2021-12-19 10:15:49".to_string()),
        })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```

## get

`POST /suppression/get.json` · [Documentation](https://docs.unione.io/en/web-api-ref#suppression-get)

Gets a reason and date of email suppression.

### Request: `SuppressionGetRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `email` | `String` | yes | Email to get suppression details for. |
| `all_projects` | `bool` |  | If a user has projects functionality enabled, he/she can pass all_projects=true to search in all projects' data. |

### Response: `SuppressionGetResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `email` | `String` | The email for which suppression details were requested. |
| `suppressions` | `Vec<SuppressionGetObject>` | Array of suppression objects. |

### Example

```rust
use unione::api::suppression::model::SuppressionGetRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .suppression()
        .get(&SuppressionGetRequest {
            email: "email@example.com".to_string(),
            ..Default::default()
        })
        .await?;
    println!("{:?}", response.email);
    Ok(())
}
```

## list

`POST /suppression/list.json` · [Documentation](https://docs.unione.io/en/web-api-ref#suppression-list)

Returns a suppression list since provided date.

### Request: `SuppressionListRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `cause` | `String` |  | Cause of email being suppressed. One of:<br><br>- __unsubscribed__ - email is unsubscribed;<br>- __temporary_unavailable__ - the email address is unavailable. This means that over the next three days sending to this address will return an error. Email may be temporarily unavailable due to several reasons, e.g.:<br><br>- a previous email has been rejected by the recipient's server for spam;<br>- the recipient's mailbox is full or is not used;<br>- recipient's domain does not accept mail;<br>- sending server was rejected due to blacklisting;<br><br>- __permanent_unavailable__ - the email address is permanently unavailable due to multiple hard bounces;<br>- __complained__ -  the recipient reported spam in the previous emails;<br>- __blocked__ -  sending to the email is prohibited by administration of UniOne.<br>We may add some new causes in the future. |
| `source` | `String` |  | Source of email being suppressed. One of:<br><br>- __user__ - suppressed by user with    [suppression/set](https://docs.unione.io/en/web-api-ref#suppression-set);<br>- __system__ - sending to the email is prohibited by system, for example due to multiple hard bounces;<br>- __subscriber__ - the recipient reported spam or unsubscribed in the previous emails. |
| `start_time` | `String` |  | Date in the format YYYY-MM-DD hh:mm:ss to get suppression list from the "start_time" to the present day. Ignored if "cursor" is not empty. |
| `cursor` | `String` |  | The parameter indicates from which position the selection is to be started. Must be empty or omitted for the first data chunk. In order to get subsequent chunks, you must set the "cursor" parameter in your request, using the value received in response to the previous request. |
| `limit` | `i64` |  | Limits the number of records to be returned at one time, default is 50. |

### Response: `SuppressionListResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `suppressions` | `Vec<SuppressionListObject>` | Array of suppression objects. |
| `cursor` | `String` | The parameter indicates from which position the selection is to be started. Must be empty or omitted for the first data chunk. In order to get subsequent chunks, you must set the "cursor" parameter in your request, using the value received in response to the previous request. |

### Example

```rust
use unione::api::suppression::model::SuppressionListRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .suppression()
        .list(&SuppressionListRequest {
            cause: Some("unsubscribed".to_string()),
            source: Some("user".to_string()),
            start_time: Some("2020-10-14 17:00:00".to_string()),
            cursor: Some("Ajfb6Hvdkn3hdhhvG57xbdufhG5".to_string()),
            limit: Some(50),
        })
        .await?;
    println!("{:?}", response.suppressions);
    Ok(())
}
```

## delete

`POST /suppression/delete.json` · [Documentation](https://docs.unione.io/en/web-api-ref#suppression-delete)

Deletes an email from suppression list (only records with is_deletable=true). If there are no entries of this email in the suppression list, API error 3003 is returned. If such entries exist but no one can be deleted, API error 3004 is returned. If daily limit of deletes exceeded, API error 906 is returned.

### Request: `SuppressionDeleteRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `email` | `String` | yes | An email to delete from suppression list. |

### Response: `SuccessResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |

### Example

```rust
use unione::api::suppression::model::SuppressionDeleteRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .suppression()
        .delete(&SuppressionDeleteRequest {
            email: "email@example.com".to_string(),
        })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```
