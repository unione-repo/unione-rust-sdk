# Email

[Back to the list of sections](../README.md)

This group of methods is related with email sending.

`unione::api::email::EmailApi`, `client.email()`

| Method | Endpoint | Description |
|---|---|---|
| [`send`](#send) | `POST /email/send.json` | Sends an email or a bunch of emails |
| [`subscribe`](#subscribe) | `POST /email/subscribe.json` | Sends an email with (re)subscribe link. |

## send

`POST /email/send.json` · [Documentation](https://docs.unione.io/en/web-api-ref#email-send)

Sends an email or a bunch of emails

While sending you can provide substitutions (merge tags), use a template, turn on read or click tracking, etc.

Restrictions:
* maximum request size is 10 MB
* maximum number of recipients is 500
* The symbol '/' is not allowed in attachment names

If the list of recipients contains invalid, non-existent, duplicate, or unsubscribed emails, those emails will be returned in the output parameter failed_emails, and the email will be sent to all other valid emails addresses. If all the emails addresses fall into failed_emails, sending will not be carried out and API error 204 will be returned.

### Request: `EmailSendRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | `MessageObject` | yes | Object containing all message properties. |

### Response: `EmailSendResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `job_id` | `String` | Job identifier, might be useful for investigating errors. |
| `emails` | `Vec<String>` | Array of recipients emails successfully accepted for sending. |
| `failed_emails` | `HashMap<String, String>` | Object with emails rejected for sending as property names and their statuses as property values, e.g.: {"email1@gmail.com": "temporary_unavailable"}. Possible statuses:<br><br>- __unsubscribed__ - specified email is unsubscribed;<br>- __invalid__ - the email does not exist or has been entered incorrectly;<br>- __duplicate__ - the email already exists in the request, email duplicating is prevented;<br>- __temporary_unavailable__ - the email address is unavailable. This means that over the next three days sending to this address will return an error. Email may be temporarily unavailable due to several reasons, e.g.:<br><br>- a previous email has been rejected by the recipient's server for spam;<br>- the recipient's mailbox is full or is not used;<br>- recipient's domain does not accept mail;<br>- sending server was rejected due to blacklisting;<br><br>- __permanent_unavailable__ - the email address is permanently unavailable or unsubscribed globally.<br>- __complained__ -  the recipient reported spam in the previous emails;<br>- __blocked__ -  sending to the email is prohibited by administration of UniOne.<br>We may added new statuses in the future. |

### Example

```rust
use std::collections::HashMap;
use unione::api::email::model::EmailSendRequest;
use unione::api::email::model::MessageObject;
use unione::api::email::model::MessageObjectRecipientsItem;
use unione::common::model::BodyObject;
use unione::common::model::TemplateEngine;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .email()
        .send(&EmailSendRequest {
            message: MessageObject {
                recipients: vec![
                    MessageObjectRecipientsItem {
                        email: "user@example.com".to_string(),
                        substitutions: Some(HashMap::from([
                            ("CustomerId".to_string(), "12452".to_string()),
                            ("to_name".to_string(), "John Smith".to_string()),
                        ])),
                        metadata: Some(HashMap::from([
                            ("campaign_id".to_string(), "c77f4f4e-3561-49f7-9f07-c35be01b4f43".to_string()),
                            ("customer_hash".to_string(), "b253ac7".to_string()),
                        ])),
                    },
                ],
                template_id: Some("00000000-0000-0000-0000-000000000000".to_string()),
                global_language: Some("en".to_string()),
                template_engine: Some(TemplateEngine::Velocity),
                body: BodyObject {
                    html: Some("<b>Hello, {{to_name}}</b>".to_string()),
                    plaintext: Some("Hello, {{to_name}}".to_string()),
                    amp: Some("<!doctype html><html amp4email><head> <meta charset=\"utf-8\"><script async src=\"https://cdn.ampproject.org/v0.js\"></script> <style amp4email-boilerplate>body{visibility:hidden}</style></head><body> Hello, AMP4EMAIL world.</body></html>".to_string()),
                },
                subject: "UniOne test email ".to_string(),
                from_email: Some("email@example.com".to_string()),
                from_name: Some("John Smith".to_string()),
                reply_to: Some("email@example.com".to_string()),
                reply_to_name: Some("John Smith".to_string()),
                idempotence_key: Some("SG1VsbG68sIH2dvc5mx890kIQ".to_string()),
                ..Default::default()
            },
        })
        .await?;
    println!("{:?}", response.job_id);
    Ok(())
}
```

## subscribe

`POST /email/subscribe.json` · [Documentation](https://docs.unione.io/en/web-api-ref#email-subscribe)

Sends an email with (re)subscribe link.

This method allows subscription renewal by a former subscriber who has previously unsubscribed, or who is blocked because of complaints. This method sends an email with a new subscription link to a former subscriber. Once the recipient clicks the link, their subscription status is restored.
There is a default restriction that limits this method to being used once a day. If you need to resubscribe more often, please ask support about it.

### Request: `EmailSubscribeRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `from_email` | `String` | yes | Sender’s email on a confirmed domain. (Field alias "email_address_from" is supported, too). |
| `from_name` | `String` | yes | Sender’s name. (Field alias "name_from" is supported, too). |
| `to_email` | `String` | yes | Recipient’s email. (Field alias "email_address_to" is supported, too). |

### Response: `SuccessResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |

### Example

```rust
use unione::api::email::model::EmailSubscribeRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .email()
        .subscribe(&EmailSubscribeRequest {
            from_email: "email@example.com".to_string(),
            from_name: "Example".to_string(),
            to_email: "email@example.com".to_string(),
        })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```
