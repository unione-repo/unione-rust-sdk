# Template

[Back to the list of sections](../README.md)

You have the option of storing a template on the UniOne side and then using it by id, which is highly recommended for sending repetitive messages or high volume traffic. UniOne supports two different [template engines](https://docs.unione.io/en/template-engines).

`unione::api::template::TemplateApi`, `client.template()`

| Method | Endpoint | Description |
|---|---|---|
| [`set`](#set) | `POST /template/set.json` | Creates or updates an email template. |
| [`get`](#get) | `POST /template/get.json` | Gets template properties by it's id. |
| [`list`](#list) | `POST /template/list.json` | Returns the list of all or some templates of the account. |
| [`delete`](#delete) | `POST /template/delete.json` | Deletes a template by id. |

## set

`POST /template/set.json` · [Documentation](https://docs.unione.io/en/web-api-ref#template-set)

Creates or updates an email template.

Restrictions:
* The maximum request size is 10 MB.
* The number of templates is limited to 10000.

### Request: `TemplateSetRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `template` | `TemplateObject` | yes | Object with all the template properties. |

### Response: `TemplateSetResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `template` | `TemplateObject` | Object with all the template properties. |

### Example

```rust
use unione::api::template::model::EditorType;
use unione::api::template::model::TemplateObject;
use unione::api::template::model::TemplateSetRequest;
use unione::common::model::TemplateEngine;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .template()
        .set(&TemplateSetRequest {
            template: TemplateObject {
                name: "name".to_string(),
                editor_type: Some(EditorType::Html),
                template_engine: Some(TemplateEngine::Velocity),
                subject: Some("UniOne test email ".to_string()),
                from_email: "user@example.com".to_string(),
                from_name: Some("John Smith".to_string()),
                reply_to: Some("email@example.com".to_string()),
                reply_to_name: Some("John Smith".to_string()),
                ..Default::default()
            },
        })
        .await?;
    println!("{:?}", response.template);
    Ok(())
}
```

## get

`POST /template/get.json` · [Documentation](https://docs.unione.io/en/web-api-ref#template-get)

Gets template properties by it's id.

### Request: `TemplateGetRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | `String` | yes | Template id. |

### Response: `TemplateGetResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `template` | `TemplateObject` | Object with all the template properties. |

### Example

```rust
use unione::api::template::model::TemplateGetRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .template()
        .get(&TemplateGetRequest {
            id: "00000000-0000-0000-0000-000000000000".to_string(),
        })
        .await?;
    println!("{:?}", response.template);
    Ok(())
}
```

## list

`POST /template/list.json` · [Documentation](https://docs.unione.io/en/web-api-ref#template-list)

Returns the list of all or some templates of the account.

You can iterate thru large list using "offset" and "limit" parameters. When less than "limit" templates are returned then the end of list is reached.

### Request: `TemplateListRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `limit` | `i64` |  | Limits the number of templates returned, default is 50. |
| `offset` | `i64` |  | Index of the first template in the list, starting with 0. |

### Response: `TemplateListResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `templates` | `Vec<TemplateListObject>` | Array of template objects. |

### Example

```rust
use unione::api::template::model::TemplateListRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .template()
        .list(&TemplateListRequest {
            ..Default::default()
        })
        .await?;
    println!("{:?}", response.templates);
    Ok(())
}
```

## delete

`POST /template/delete.json` · [Documentation](https://docs.unione.io/en/web-api-ref#template-delete)

Deletes a template by id.

### Request: `TemplateDeleteRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | `String` | yes | Template id. |

### Response: `SuccessResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |

### Example

```rust
use unione::api::template::model::TemplateDeleteRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .template()
        .delete(&TemplateDeleteRequest {
            id: "00000000-0000-0000-0000-000000000000".to_string(),
        })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```
