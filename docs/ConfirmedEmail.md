# ConfirmedEmail

[Back to the list of sections](../README.md)

Manage email addresses confirmed for sandbox-domain sending. These methods are available only with a main account API key.

`unione::api::confirmed_email::ConfirmedEmailApi`, `client.confirmed_email()`

| Method | Endpoint | Description |
|---|---|---|
| [`create`](#create) | `POST /confirmed-email/create.json` | Adds an email address and sends a confirmation link. |
| [`list`](#list) | `POST /confirmed-email/list.json` | Lists confirmed and unconfirmed email addresses for the account. |
| [`delete`](#delete) | `POST /confirmed-email/delete.json` | Deletes an email address from the account's confirmed email list. |

## create

`POST /confirmed-email/create.json` · [Documentation](https://docs.unione.io/en/web-api-ref#confirmed-email-create)

Adds an email address and sends a confirmation link.

### Request: `ConfirmedEmailRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `email` | `String` | yes | Email address to add or delete. |

### Response: `ConfirmedEmailCreateResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `confirmed_email` | `ConfirmedEmail` |  |

### Example

```rust
use unione::api::confirmed_email::model::ConfirmedEmailRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .confirmed_email()
        .create(&ConfirmedEmailRequest {
            email: "dev@example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.confirmed_email);
    Ok(())
}
```

## list

`POST /confirmed-email/list.json` · [Documentation](https://docs.unione.io/en/web-api-ref#confirmed-email-list)

Lists confirmed and unconfirmed email addresses for the account.

### Request

The method has no parameters.

### Response: `ConfirmedEmailListResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `confirmed_emails` | `Vec<ConfirmedEmail>` |  |

### Example

```rust
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client.confirmed_email().list().await?;
    println!("{:?}", response.confirmed_emails);
    Ok(())
}
```

## delete

`POST /confirmed-email/delete.json` · [Documentation](https://docs.unione.io/en/web-api-ref#confirmed-email-delete)

Deletes an email address from the account's confirmed email list.

### Request: `ConfirmedEmailRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `email` | `String` | yes | Email address to add or delete. |

### Response: `ConfirmedEmailDeleteResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `message` | `String` |  |

### Example

```rust
use unione::api::confirmed_email::model::ConfirmedEmailRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .confirmed_email()
        .delete(&ConfirmedEmailRequest {
            email: "dev@example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.message);
    Ok(())
}
```
