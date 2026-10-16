# System

[Back to the list of sections](../README.md)

System operations.

`unione::api::system::SystemApi`, `client.system()`

| Method | Endpoint | Description |
|---|---|---|
| [`ping`](#ping) | `POST /system/ping.json` | Checks API key validity and returns user ID. |
| [`info`](#info) | `POST /system/info.json` | Gets user or project info by API key. |

## ping

`POST /system/ping.json` · [Documentation](https://docs.unione.io/en/web-api-ref#system-ping)

Checks API key validity and returns user ID.

### Request

The method has no parameters.

### Response: `SystemPingResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `user_id` | `i64` | Unique user identifier. |

### Example

```rust
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client.system().ping().await?;
    println!("{:?}", response.user_id);
    Ok(())
}
```

## info

`POST /system/info.json` · [Documentation](https://docs.unione.io/en/web-api-ref#system-info)

Gets user or project info by API key.

### Request

The method has no parameters.

### Response: `SystemInfoResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `user_id` | `i64` | Unique user identifier. |
| `email` | `String` | Email of the user. |
| `project_id` | `String` | Unqiue project identifier, ASCII string up to 36 characters long. Present only if the API key used for request is the project API key. |
| `project_name` | `String` | Project name, unique for user account. Present only if the API key used for request is the project API key. |
| `project_accounting` | `ProjectAccountingObject` | Object with email counter properties. Present only if the API key used for request is the user API key. |
| `accounting` | `SystemInfoResponseAccounting` | Object with accounting period properties. Present only if the API key used for request is the user API key. |

### Example

```rust
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client.system().info().await?;
    println!("{:?}", response.user_id);
    Ok(())
}
```
