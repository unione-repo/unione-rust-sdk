# Domain

[Back to the list of sections](../README.md)

You can use these methods to setup sender domains and check their statuses. They might be useful in case you want to automate adding domains for your own clients.

To add new sender domain thru API you should take following steps:
- Get DNS records with [domain/get-dns-records](https://docs.unione.io/en/web-api-ref#domain-get-dns-records) method and add them to your DNS provider.
- Every 5-10 minutes call [domain/list](https://docs.unione.io/en/web-api-ref#domain-list) method for your domain (or several domains) and check verification-record.status and dkim.status in the response. You can call it more often if you like, but please not more than once per minute.
- If verification-record.status="unconfirmed" then you should call [domain/validate-verification-record](https://docs.unione.io/en/web-api-ref#domain-validate-verification-record) to trigger verification record validation.
- If dkim.status="inactive" then start DKIM checking by calling [domain/validate-dkim](https://docs.unione.io/en/web-api-ref#domain-validate-dkim) (UniOne itself checks  DKIM every 30 minutes also)
- When you get verification-record.status="confirmed" and dkim.status="active" for the domain then you can stop checking it's status with domain/list and start using it as a sender domain.

`unione::api::domain::DomainApi`, `client.domain()`

| Method | Endpoint | Description |
|---|---|---|
| [`get_dns_records`](#get_dns_records) | `POST /domain/get-dns-records.json` | Returns info on domain DNS setup for using with UniOne. |
| [`validate_verification_record`](#validate_verification_record) | `POST /domain/validate-verification-record.json` | Triggers verification record validation for domain. |
| [`validate_dkim`](#validate_dkim) | `POST /domain/validate-dkim.json` | Validates DKIM record for domain. |
| [`list`](#list) | `POST /domain/list.json` | Returns a list of all registered domains for user or project, together with statuses of last validations. |
| [`get_sandbox_domain`](#get_sandbox_domain) | `POST /domain/get-sandbox-domain.json` | Creates a sandbox domain for the account or returns the existing one. |
| [`delete`](#delete) | `POST /domain/delete.json` | Deletes a domain. Attention: you should use project's API key to delete a project's domain. |

## get_dns_records

`POST /domain/get-dns-records.json` · [Documentation](https://docs.unione.io/en/web-api-ref#domain-get-dns-records)

Returns info on domain DNS setup for using with UniOne.

In order to send emails you have to confirm that you own the domain that you are going to send emails from and setup DKIM signature for the domain. This method allows you to register your domain in UniOne and generate verification record and DKIM key, or return previously generated ones if domain is already registered.
You should add then DNS records like this to all of the domains you want to confirm:

`@             IN TXT "unione-validate-hash=483bb362ebdbeedd755cfb1d4d661"`

`us._domainkey IN TXT "k=rsa; p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDo7"`

`@             IN TXT  "v=spf1 include:spf.unione.io ~all"`

- First record value should contain "verification-record" field returned by get-dns-records method "as is".
- Second record value should contain "dkim" field value after "p=". Mind the name of this record! "us._domainkey." prefix is important for DKIM selector of UniOne to work properly.
- Third record contains optional, but very recommended SPF entry. Probably you already have one, then just add append include:spf.unione.io right after v=spf1.

Syntax requirements of different DNS providers may differ:
- sometimes it's necesarry to put full domain name instead of @ sign.
- some providers require add TTL value (like 3600)
- some providers don't require quotes for enclosing values
Please check requirements of your provider and advice your users to do it.

### Request: `DomainGetDnsRecordsRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `domain` | `String` | yes | Domain to get DNS records for. |

### Response: `DomainGetDnsRecordsResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `domain` | `String` | Domain to get DNS records for. |
| `verification_record` | `String` | Record to be added "as is" to verify ownership of this domain. |
| `dkim` | `String` | DKIM signature for the domain. This property contains only the key part, you should prepend it with "v=DKIM1; k=rsa; p=" for the TXT record to be valid. |
| `dmarc` | `String` | Target of the CNAME record for the "_dmarc" host of the domain. |

### Example

```rust
use unione::api::domain::model::DomainGetDnsRecordsRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .domain()
        .get_dns_records(&DomainGetDnsRecordsRequest {
            domain: "example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.domain);
    Ok(())
}
```

## validate_verification_record

`POST /domain/validate-verification-record.json` · [Documentation](https://docs.unione.io/en/web-api-ref#domain-validate-verification-record)

Triggers verification record validation for domain.

If you get 200 OK then it means the validation process started succesfully, but it doesn't mean the verification record is valid. You should check later verification-record.status field returned by [domain/list](https://docs.unione.io/en/web-api-ref#domain-list) method to get the result. Please read more [here](https://docs.unione.io/en/web-api-ref#domain).

### Request: `DomainValidateVerificationRecordRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `domain` | `String` | yes | Domain to validate verification record for. |

### Response: `DomainValidateVerificationRecordResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `message` | `String` | Debug message. |

### Example

```rust
use unione::api::domain::model::DomainValidateVerificationRecordRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .domain()
        .validate_verification_record(&DomainValidateVerificationRecordRequest {
            domain: "example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.message);
    Ok(())
}
```

## validate_dkim

`POST /domain/validate-dkim.json` · [Documentation](https://docs.unione.io/en/web-api-ref#domain-validate-dkim)

Validates DKIM record for domain.

If you get 200 OK then it means the validation process started succesfully, but it doesn't mean the DKIM record is valid. You should check later dkim.status field returned by [domain/list](https://docs.unione.io/en/web-api-ref#domain-list) method to get the result. Please read more [here](https://docs.unione.io/en/web-api-ref#domain).

### Request: `DomainValidateDkimRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `domain` | `String` | yes | Domain to validate DKIM record for. |

### Response: `DomainValidateDkimResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `message` | `String` | Debug message. |

### Example

```rust
use unione::api::domain::model::DomainValidateDkimRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .domain()
        .validate_dkim(&DomainValidateDkimRequest {
            domain: "example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.message);
    Ok(())
}
```

## list

`POST /domain/list.json` · [Documentation](https://docs.unione.io/en/web-api-ref#domain-list)

Returns a list of all registered domains for user or project, together with statuses of last validations.

Returns last status of ownership validation and DKIM validation for each domain. Read more [here](https://docs.unione.io/en/web-api-ref#domain).

### Request: `DomainListRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `domain` | `String` |  | Optional domain name if you want to get the status of single domain only. |
| `limit` | `i64` |  | Limits the number of domains returned. Default is 50. |
| `offset` | `i64` |  | Index of the first domain in the list, starting with 0. |

### Response: `DomainListResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `domains` | `Vec<DomainListResponseDomainsItem>` | Array of objects describing domains. |

### Example

```rust
use unione::api::domain::model::DomainListRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .domain()
        .list(&DomainListRequest {
            domain: Some("example.com".to_string()),
            ..Default::default()
        })
        .await?;
    println!("{:?}", response.domains);
    Ok(())
}
```

## get_sandbox_domain

`POST /domain/get-sandbox-domain.json` · [Documentation](https://docs.unione.io/en/web-api-ref#domain-get-sandbox-domain)

Creates a sandbox domain for the account or returns the existing one.

Returns an account sandbox domain that is ready for use in the `from_email` field of [email/send](https://docs.unione.io/en/web-api-ref#email-send). If the account has no sandbox domain, UniOne creates one and returns it. Repeated calls return the same domain.

The method is available only with an account API key. Project API keys cannot create sandbox domains.

### Request

The method has no parameters.

### Response: `DomainGetSandboxDomainResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `domain` | `String` | Sandbox domain name. |

### Example

```rust
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client.domain().get_sandbox_domain().await?;
    println!("{:?}", response.domain);
    Ok(())
}
```

## delete

`POST /domain/delete.json` · [Documentation](https://docs.unione.io/en/web-api-ref#domain-delete)

Deletes a domain. Attention: you should use project's API key to delete a project's domain.

Deletes a domain.

### Request: `DomainDeleteRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `domain` | `String` | yes | Domain name to delete. |

### Response: `DomainDeleteResponse`

| Field | Type | Description |
|---|---|---|
| `status` | `String` | "success" string (`success`) |
| `message` | `String` | Domain deleted |

### Example

```rust
use unione::api::domain::model::DomainDeleteRequest;
use unione::UniOneClient;

#[tokio::main]
async fn main() -> Result<(), unione::ApiError> {
    let client = UniOneClient::new("YOUR_API_KEY");

    let response = client
        .domain()
        .delete(&DomainDeleteRequest {
            domain: "example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.message);
    Ok(())
}
```
