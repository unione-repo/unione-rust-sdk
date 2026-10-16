# Project

[Back to the list of sections](../README.md)

Projects allow you to group several independent domains, webhooks, templates, and unsubscription lists together. It's very useful if you have your own clients and manage their data from a single UniOne account. Each project receives its own API key, and all API calls with that API key only deal with the data of that project. This way, you can assign a separate project for each client and be sure that they won't conflict; e.g. unsubscribing from one project won't affect another.

Projects methods are disabled by default due to antispam measures. To enable them, [contact support](https://cp.unione.io/en/support).

Please note that sending domains are confirmed and used on per-project basis by default. But it's possible to allow projects to send emails from domains confirmed for main account - just [ask support](https://cp.unione.io/en/support) to enable it.

`unione::api::project::ProjectApi`, `client.project()`

| Method | Endpoint | Description |
|---|---|---|
| [`create`](#create) | `POST /project/create.json` | Creates a new project. |
| [`update`](#update) | `POST /project/update.json` | Updates a project. |
| [`list`](#list) | `POST /project/list.json` | Lists projects. |
| [`delete`](#delete) | `POST /project/delete.json` | Deletes a project. |

## create

`POST /project/create.json` · [Documentation](https://docs.unione.io/en/web-api-ref#project-create)

Creates a new project.

### Request: `ProjectCreateRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `project` | `ProjectObject` | yes | Object with project properties. |

### Response: `ProjectCreateResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `project_id` | `String` | Unqiue project identifier, ASCII string up to 36 characters long. |
| `project_api_key` | `String` | API key of the project. You can use it instead of the user API key parameter in all methods except project/* methods. |

### Example

```rust
use unione::api::project::model::ProjectCreateRequest;
use unione::api::project::model::ProjectObject;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .project()
        .create(&ProjectCreateRequest {
            project: ProjectObject {
                name: "Project 1A".to_string(),
                country: Some("ES".to_string()),
                backend_domain_id: Some(1234),
                unsubscribe_page_id: Some(1),
                ..Default::default()
            },
        })
        .await?;
    println!("{:?}", response.project_id);
    Ok(())
}
```

## update

`POST /project/update.json` · [Documentation](https://docs.unione.io/en/web-api-ref#project-update)

Updates a project.

### Request: `ProjectUpdateRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `project_id` | `String` |  | Unique project identifier. You should identify project to update either with project_id or with project_api_key field. But we recommend usage of project_id as a more secure option. |
| `project_api_key` | `String` |  | Optional project API key. You should identify project to update either with project_id or with project_api_key field. But we recommend usage of project_id as a more secure option. |
| `project` | `ProjectUpdateObject` | yes | Object with project properties. |

### Response: `ProjectUpdateResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `project_api_key` | `String` | API key of the project. Returned only if the project to update was identified by project_api_key. Not returned in case of updating by project_id. |

### Example

```rust
use unione::api::project::model::ProjectUpdateObject;
use unione::api::project::model::ProjectUpdateRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .project()
        .update(&ProjectUpdateRequest {
            project_id: Some("1234567890".to_string()),
            project_api_key: Some("your api_key".to_string()),
            project: ProjectUpdateObject {
                name: "Project 1A".to_string(),
                country: Some("ES".to_string()),
                backend_domain_id: Some(1234),
                unsubscribe_page_id: Some(1),
                ..Default::default()
            },
        })
        .await?;
    println!("{:?}", response.project_api_key);
    Ok(())
}
```

## list

`POST /project/list.json` · [Documentation](https://docs.unione.io/en/web-api-ref#project-list)

Lists projects.

### Request: `ProjectListRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `project_id` | `String` |  | Optional possibility to specify project_id to include only this project to list. |
| `project_api_key` | `String` |  | Optional possibility to specify project_api_key to include only this project to list. |

### Response: `ProjectListResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `projects` | `Vec<ProjectListObject>` | Array of objects where each object represents one project. |

### Example

```rust
use unione::api::project::model::ProjectListRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .project()
        .list(&ProjectListRequest {
            project_id: Some("1234567890".to_string()),
            project_api_key: Some("your api_key".to_string()),
        })
        .await?;
    println!("{:?}", response.projects);
    Ok(())
}
```

## delete

`POST /project/delete.json` · [Documentation](https://docs.unione.io/en/web-api-ref#project-delete)

Deletes a project.

The project will be moved to the "trash bin" before being permanently deleted. You can restore the project by contacting support.

### Request: `ProjectDeleteRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `project_id` | `String` |  | Unique project identifier. You should identify project to delete either with project_id or with project_api_key field. We recommend usage of project_id as a more secure option. |
| `project_api_key` | `String` |  | Optional project API key. You should identify project to delete either with project_id or with project_api_key field. We recommend usage of project_id as a more secure option. |

### Response: `SuccessResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |

### Example

```rust
use unione::api::project::model::ProjectDeleteRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .project()
        .delete(&ProjectDeleteRequest {
            project_id: Some("1234567890".to_string()),
            project_api_key: Some("your api_key".to_string()),
        })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```
