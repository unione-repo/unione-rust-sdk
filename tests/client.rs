//! Tests of the client against a mock of the API on 127.0.0.1: no network, no real API.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use serde_json::Value;
use unione::api::email::model::EmailSendRequest;
use unione::api::email::model::MessageObject;
use unione::api::email::model::MessageObjectRecipientsItem as Recipient;
use unione::client;
use unione::common::model::BodyObject;
use unione::ApiClient;
use unione::ApiError;
use unione::Configuration;
use unione::ErrorKind;
use unione::UniOneClient;

const KEY: &str = "test-key";
/// The answer of the API to email/send.
const SENT: &str = r#"{
    "status": "success",
    "job_id": "1ZymBc",
    "emails": [
        "john@example.com"
    ],
    "failed_emails": {
        "x@example.com": "unsubscribed"
    }
}"#;
/// The JSON that `request()` must post.
const EXPECTED: &str = r#"{
    "message": {
        "recipients": [
            {
                "email": "john@example.com",
                "substitutions": {
                    "to_name": "John"
                }
            }
        ],
        "body": {
            "html": "<b>Hello</b>"
        },
        "subject": "Hello",
        "from_email": "shop@example.com"
    }
}"#;
const INFO: &str = r#"{"status":"success","user_id":11344,"email":"user@example.com"}"#;
const API_ERROR: &str = r#"{"status":"error","message":"Unknown recipient","code":204}"#;
const NO_RECIPIENTS: &str = "email/send: message.recipients is required";
const NO_EMAIL: &str = "email/send: message.recipients[0].email is required";

/// A request the mock received.
#[derive(Clone)]
struct Received {
    line: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Received {
    fn header(&self, name: &str) -> Option<&str> {
        let (_, value) = self.headers.iter().find(|(key, _)| key == name)?;
        Some(value.as_str())
    }
}

/// Mock of the API on 127.0.0.1: answers each request with the status and the body after the delay.
struct Mock {
    url: String,
    log: Arc<Mutex<Vec<Received>>>,
}

impl Mock {
    fn start(status: u16, body: &str) -> Mock {
        Mock::with_delay(status, body, Duration::ZERO)
    }

    fn with_delay(status: u16, body: &str, delay: Duration) -> Mock {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("http://{addr}{}", client::BASE_PATH);
        let log = Arc::new(Mutex::new(Vec::new()));
        let received = Arc::clone(&log);
        let answer = response(status, body);
        thread::spawn(move || {
            for mut stream in listener.incoming().flatten() {
                if let Some(request) = read_request(&mut stream) {
                    received.lock().unwrap().push(request);
                }
                thread::sleep(delay);
                let _ = stream.write_all(answer.as_bytes());
            }
        });
        Mock { url, log }
    }

    fn received(&self) -> Vec<Received> {
        self.log.lock().unwrap().clone()
    }
}

/// HTTP/1.1 response of the mock: JSON or an HTML page (of a proxy).
fn response(status: u16, body: &str) -> String {
    let kind = if body.starts_with('{') {
        "application/json"
    } else {
        "text/html"
    };
    let length = body.len();
    let mut head = format!("HTTP/1.1 {status} Mock\r\nContent-Type: {kind}\r\n");
    head += &format!("Content-Length: {length}\r\nConnection: close\r\n\r\n");
    head + body
}

/// Reads a request: the head, then the body of its Content-Length.
fn read_request(stream: &mut TcpStream) -> Option<Received> {
    let timeout = Some(Duration::from_secs(5));
    stream.set_read_timeout(timeout).ok()?;
    let mut data: Vec<u8> = Vec::new();
    let mut chunk = [0u8; 4096];
    let head_end = loop {
        if let Some(end) = data.windows(4).position(|w| w == b"\r\n\r\n") {
            break end + 4;
        }
        let n = stream.read(&mut chunk).ok()?;
        if n == 0 {
            return None;
        }
        data.extend_from_slice(&chunk[..n]);
    };
    let head = String::from_utf8_lossy(&data[..head_end]).into_owned();
    let mut lines = head.split("\r\n");
    let line = lines.next()?.to_string();
    let mut headers = Vec::new();
    for header in lines {
        if let Some((name, value)) = header.split_once(':') {
            let name = name.trim().to_ascii_lowercase();
            headers.push((name, value.trim().to_string()));
        }
    }
    let mut received = Received {
        line,
        headers,
        body: String::new(),
    };
    let length = received
        .header("content-length")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    while data.len() < head_end + length {
        let n = stream.read(&mut chunk).ok()?;
        if n == 0 {
            break;
        }
        data.extend_from_slice(&chunk[..n]);
    }
    received.body = String::from_utf8_lossy(&data[head_end..]).into_owned();
    Some(received)
}

/// Client of the mock: its URL is the base URL, no proxy.
fn mock_client(url: &str) -> UniOneClient {
    client_with(url, KEY, Duration::from_secs(10))
}

fn client_with(url: &str, key: &str, timeout: Duration) -> UniOneClient {
    let mut config = Configuration::new(key);
    config.base_url = url.to_string();
    config.timeout = timeout;
    let http = reqwest::Client::builder().no_proxy().build().unwrap();
    UniOneClient::from_api_client(ApiClient::with_http_client(config, http))
}

/// The message of the tests: one recipient with a name, an HTML body.
fn request() -> EmailSendRequest {
    EmailSendRequest {
        message: MessageObject {
            recipients: vec![Recipient::from_address("John <john@example.com>", None)],
            from_email: Some("shop@example.com".to_string()),
            subject: "Hello".to_string(),
            body: BodyObject::html("<b>Hello</b>"),
            ..Default::default()
        },
    }
}

/// email/send to the mock, which must fail.
async fn send_error(url: &str) -> ApiError {
    let api = mock_client(url);
    api.email().send(&request()).await.unwrap_err()
}

#[tokio::test]
async fn send_posts_the_message() {
    let mock = Mock::start(200, SENT);
    let api = mock_client(&mock.url);
    let response = api.email().send(&request()).await.unwrap();
    assert_eq!(response.job_id.as_deref(), Some("1ZymBc"));
    let emails = response.emails.unwrap_or_default();
    assert_eq!(emails, ["john@example.com"]);
    let failed = response.failed_emails.unwrap_or_default();
    let reason = failed.get("x@example.com").map(String::as_str);
    assert_eq!(reason, Some("unsubscribed"));

    let received = mock.received();
    assert_eq!(received.len(), 1);
    let path = format!("{}/email/send.json", client::BASE_PATH);
    let line = format!("POST {path}?platform={} HTTP/1.1", client::PLATFORM);
    assert_eq!(received[0].line, line);
    assert_eq!(received[0].header("x-api-key"), Some(KEY));
    let content_type = received[0].header("content-type");
    assert_eq!(content_type, Some("application/json"));
    let sent: Value = serde_json::from_str(&received[0].body).unwrap();
    let expected: Value = serde_json::from_str(EXPECTED).unwrap();
    assert_eq!(sent, expected);
}

#[tokio::test]
async fn info_parses_the_response() {
    let mock = Mock::start(200, INFO);
    let info = mock_client(&mock.url).system().info().await.unwrap();
    assert_eq!(info.user_id, Some(11344));
    assert_eq!(info.email.as_deref(), Some("user@example.com"));
    let received = mock.received();
    let path = format!("POST {}/system/info.json?", client::BASE_PATH);
    assert!(received[0].line.starts_with(&path));
    assert_eq!(received[0].body, "{}");
}

#[tokio::test]
async fn api_error_names_the_method_code_and_fix() {
    let mock = Mock::start(400, API_ERROR);
    let error = send_error(&mock.url).await;
    assert_eq!(error.kind, ErrorKind::Api);
    assert!(!error.retryable());
    assert_eq!(error.code, Some(204));
    assert_eq!(error.http_status, 400);
    assert_eq!(error.method, "email/send");
    assert_eq!(error.api_message.as_deref(), Some("Unknown recipient"));
    assert_eq!(error.body.as_deref(), Some(API_ERROR));
    let hint = error.hint.clone().unwrap_or_default();
    assert!(!hint.is_empty());
    assert!(error.message.starts_with("email/send: "));
    assert!(error.message.contains("Unknown recipient"));
    assert!(error.message.contains("(API error 204, HTTP 400)"));
    assert!(error.message.contains(" Fix: "));
}

#[tokio::test]
async fn rate_limit_and_server_errors_are_retryable() {
    for status in [429, 503] {
        let mock = Mock::start(status, API_ERROR);
        let error = send_error(&mock.url).await;
        assert_eq!(error.kind, ErrorKind::Api);
        assert_eq!(error.http_status, status);
        assert!(error.retryable(), "HTTP {status}");
    }
}

#[tokio::test]
async fn proxy_page_is_a_retryable_invalid_response() {
    let mock = Mock::start(502, "<html>Bad Gateway</html>");
    let error = send_error(&mock.url).await;
    assert_eq!(error.kind, ErrorKind::InvalidResponse);
    assert_eq!(error.http_status, 502);
    assert!(error.retryable());
    assert!(error.message.contains("Bad Gateway"));
}

#[tokio::test]
async fn non_json_success_is_an_invalid_response() {
    let mock = Mock::start(200, "OK");
    let error = send_error(&mock.url).await;
    assert_eq!(error.kind, ErrorKind::InvalidResponse);
    assert_eq!(error.http_status, 200);
    assert!(!error.retryable());
}

#[tokio::test]
async fn refused_connection_is_a_network_error() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    drop(listener);
    let error = send_error(&url).await;
    assert_eq!(error.kind, ErrorKind::Network);
    assert!(error.retryable());
    assert!(error.is_connect());
    assert_eq!(error.http_status, 0);
}

#[tokio::test]
async fn slow_answer_is_a_timeout() {
    let mock = Mock::with_delay(200, SENT, Duration::from_secs(5));
    let api = client_with(&mock.url, KEY, Duration::from_millis(200));
    let error = api.email().send(&request()).await.unwrap_err();
    assert_eq!(error.kind, ErrorKind::Timeout);
    assert!(error.is_timeout());
    assert!(error.retryable());
}

#[tokio::test]
async fn empty_key_sends_nothing() {
    let mock = Mock::start(200, INFO);
    let api = client_with(&mock.url, "", Duration::from_secs(10));
    let error = api.system().info().await.unwrap_err();
    assert_eq!(error.kind, ErrorKind::InvalidRequest);
    assert!(!error.retryable());
    assert_eq!(error.method, "system/info");
    assert_eq!(error.http_status, 0);
    assert!(error.api_message.is_none());
    assert!(mock.received().is_empty());
}

#[tokio::test]
async fn missing_recipients_send_nothing() {
    let mock = Mock::start(200, SENT);
    let api = mock_client(&mock.url);
    let mut empty = request();
    empty.message.recipients.clear();
    let error = api.email().send(&empty).await.unwrap_err();
    assert_eq!(error.kind, ErrorKind::InvalidRequest);
    assert!(!error.retryable());
    assert_eq!(error.message, NO_RECIPIENTS);

    empty.message.recipients = vec![Recipient::default()];
    let error = api.email().send(&empty).await.unwrap_err();
    assert_eq!(error.kind, ErrorKind::InvalidRequest);
    assert_eq!(error.message, NO_EMAIL);

    let body = r#"{"message":{"subject":"Hello"}}"#;
    let body: Value = serde_json::from_str(body).unwrap();
    let call = api.api_client().call::<Value>("/email/send.json", body);
    let error = call.await.unwrap_err();
    assert_eq!(error.message, NO_RECIPIENTS);
    assert!(mock.received().is_empty());
}

#[tokio::test]
async fn long_api_message_is_cut_in_the_message_only() {
    let text = "x".repeat(600);
    let body = format!(r#"{{"status":"error","message":"{text}","code":204}}"#);
    let mock = Mock::start(400, &body);
    let error = send_error(&mock.url).await;
    assert_eq!(error.api_message.as_deref(), Some(text.as_str()));
    assert!(error.message.contains("... (600 chars)"));
    assert!(!error.message.contains(&text));
}

#[tokio::test]
async fn string_codes_are_read_when_numeric() {
    let body = r#"{"status":"error","message":"Unknown recipient","code":"204"}"#;
    let mock = Mock::start(400, body);
    let error = send_error(&mock.url).await;
    assert_eq!(error.code, Some(204));

    let body = r#"{"status":"error","message":"Unknown recipient","code":"abc"}"#;
    let mock = Mock::start(400, body);
    let error = send_error(&mock.url).await;
    assert!(error.code.is_none());
    assert!(error.hint.is_none());
}

#[test]
fn base_url_for_a_host_or_a_url() {
    let url = client::base_url_for("http://127.0.0.1:8080/api/");
    assert_eq!(url, "http://127.0.0.1:8080/api");
    let url = client::base_url_for("eu1.example.com");
    let expected = format!("https://eu1.example.com{}", client::BASE_PATH);
    assert_eq!(url, expected);
}
