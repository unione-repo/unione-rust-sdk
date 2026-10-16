# Obsolete

[Back to the list of sections](../README.md)

Obsolete methods dealing with unsubscribes. We recommend using the more flexible [suppression](https://docs.unione.io/en/web-api-ref#suppression) methods instead. Note that you can't resubscribe an unsubscribed recipient using WebAPI, but you can send him an email with subscription link using email/subscribe method.

`unione::api::obsolete::ObsoleteApi`, `client.obsolete()`

| Method | Endpoint | Description |
|---|---|---|
| [`unsubscribed_set`](#unsubscribed_set) | `POST /unsubscribed/set.json` | Registers a new unsubscribed email. |
| [`unsubscribed_check`](#unsubscribed_check) | `POST /unsubscribed/check.json` | Checks if an email is unsubscribed. |
| [`unsubscribed_list`](#unsubscribed_list) | `POST /unsubscribed/list.json` | Returns a list of all unsubscribed emails since provided date. |

## unsubscribed_set

`POST /unsubscribed/set.json` · [Documentation](https://docs.unione.io/en/web-api-ref#unsubscribed-set)

Registers a new unsubscribed email.

After registering an email as unsubscribed all future attempts to send a message to this email will be rejected. Please note that there's no way to remove an email from unsubscribed thru the API. But you can send an email with a link to resubscribe using [email/subscribe](https://docs.unione.io/en/web-api-ref#email-subscribe) method or remove "unsubscribed" status manually on [Email Search page](https://cp.unione.io/en/user/email) in control panel (but only a limited number of unsubscribed email per day can be removed).

### Request: `ObsoleteUnsubscribedSetRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `address` | `String` | yes | Email to be unsubscribed. |

### Response: `ObsoleteUnsubscribedSetResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `address` | `String` | The unsubscribed email. |
| `message` | `ObsoleteUnsubscribedSetMessage` | Unsubscription process details:<br><br>- "unsubscribed" if email has just been unsubscribed,<br>- "was inactive" if it was inactive due some reason,<br>- "was unsubscribed" if it already was unsubscribed before the call,<br>- "added and unsubscribed" if it's a new email without status history and it was added and then unsubscribed immediately. (`unsubscribed`, `was inactive`, `was unsubscribed`, `added and unsubscribed`) |

### Example

```rust
use unione::api::obsolete::model::ObsoleteUnsubscribedSetRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .obsolete()
        .unsubscribed_set(&ObsoleteUnsubscribedSetRequest {
            address: "email@example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.address);
    Ok(())
}
```

## unsubscribed_check

`POST /unsubscribed/check.json` · [Documentation](https://docs.unione.io/en/web-api-ref#unsubscribed-check)

Checks if an email is unsubscribed.

### Request: `ObsoleteUnsubscribedCheckRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `address` | `String` | yes | Email to check. |

### Response: `ObsoleteUnsubscribedCheckResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `address` | `String` | Email that was checked. |
| `is_unsubscribed` | `bool` | true if unsubscribed, false if not. |

### Example

```rust
use unione::api::obsolete::model::ObsoleteUnsubscribedCheckRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .obsolete()
        .unsubscribed_check(&ObsoleteUnsubscribedCheckRequest {
            address: "email@example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.address);
    Ok(())
}
```

## unsubscribed_list

`POST /unsubscribed/list.json` · [Documentation](https://docs.unione.io/en/web-api-ref#unsubscribed-list)

Returns a list of all unsubscribed emails since provided date.

### Request: `ObsoleteUnsubscribedListRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `date_from` | `String` |  | Date in the format YYYY-MM-DD to get all unsubscribed emails from the "date_from" to the present day. If the parameter is not specified, the default period is today (from 00:00 UTC). |

### Response: `ObsoleteUnsubscribedListResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `unsubscribed` | `Vec<ObsoleteUnsubscribedListResponseUnsubscribedItem>` | Array of object describing unsubscribed emails. |

### Example

```rust
use unione::api::obsolete::model::ObsoleteUnsubscribedListRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .obsolete()
        .unsubscribed_list(&ObsoleteUnsubscribedListRequest {
            date_from: Some("2020-10-14".to_string()),
        })
        .await?;
    println!("{:?}", response.unsubscribed);
    Ok(())
}
```
