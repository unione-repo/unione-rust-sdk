# Tag

[Back to the list of sections](../README.md)

This family of methods is used to manage user-defined tags. Tags are used to categorize letters based on your own criteria, e. g. to create a distinction between marketing and transactional letters or group together all notifications for a particular order type. UniOne allows you to assign tags to all outgoing emails, and the tags you’ve set may then be used to filter and analyze delivery statistics.

`unione::api::tag::TagApi`, `client.tag()`

| Method | Endpoint | Description |
|---|---|---|
| [`list`](#list) | `POST /tag/list.json` | This method returns the full list of user-defined tags. Does not receive any parameters. |
| [`delete`](#delete) | `POST /tag/delete.json` | Removes tag. |

## list

`POST /tag/list.json` · [Documentation](https://docs.unione.io/en/web-api-ref#tag-list)

This method returns the full list of user-defined tags. Does not receive any parameters.

### Request

The method has no parameters.

### Response: `TagListResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `tags` | `Vec<TagsResponseObject>` | An array of objects containing data for user-defined tags. |

### Example

```rust
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client.tag().list().await?;
    println!("{:?}", response.tags);
    Ok(())
}
```

## delete

`POST /tag/delete.json` · [Documentation](https://docs.unione.io/en/web-api-ref#tag-delete)

Removes tag.

### Request: `TagDeleteRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `tag_id` | `i64` | yes | Unique tag id. |

### Response: `SuccessResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |

### Example

```rust
use unione::api::tag::model::TagDeleteRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .tag()
        .delete(&TagDeleteRequest { tag_id: 54 })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```
