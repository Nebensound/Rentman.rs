//! Local HTTP test server and assertions shared by the unit tests.

use reqwest::Url;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub(crate) struct TestServer {
    pub(crate) base_url: String,
    pub(crate) requests: Receiver<RecordedRequest>,
}

impl TestServer {
    pub(crate) fn base_url(&self) -> &str {
        &self.base_url
    }

    pub(crate) fn next_request(&self) -> RecordedRequest {
        self.requests
            .recv_timeout(Duration::from_secs(2))
            .expect("test server recorded request")
    }

    /// Asserts that no further request reached the server.
    pub(crate) fn assert_no_more_requests(&self) {
        let extra = self.requests.recv_timeout(Duration::from_millis(200));
        assert!(extra.is_err(), "unexpected extra request: {extra:?}");
    }
}

#[derive(Debug)]
pub(crate) struct RecordedRequest {
    pub(crate) method: String,
    pub(crate) target: String,
    pub(crate) path: String,
    pub(crate) headers: BTreeMap<String, String>,
    pub(crate) body: Vec<u8>,
}

impl RecordedRequest {
    pub(crate) fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .get(&name.to_ascii_lowercase())
            .map(String::as_str)
    }

    pub(crate) fn query_value(&self, name: &str) -> Option<String> {
        let url = Url::parse(&format!("http://example.test{}", self.target)).unwrap();
        url.query_pairs()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.into_owned())
    }

    pub(crate) fn json_body(&self) -> Value {
        serde_json::from_slice(&self.body).expect("request body is valid JSON")
    }
}

pub(crate) fn spawn_server(responses: Vec<String>) -> TestServer {
    let listener = TcpListener::bind("127.0.0.1:0").expect("test server binds");
    let address = listener.local_addr().expect("test server has address");
    let base_url = format!("http://{address}");
    let (request_tx, request_rx) = mpsc::channel();
    let responses = responses
        .into_iter()
        .map(|response| response.replace("{base}", &base_url))
        .map(refresh_content_length)
        .collect::<Vec<_>>();

    thread::spawn(move || {
        for response in responses {
            let (mut stream, _) = listener.accept().expect("test server accepts request");
            let request = read_request(&mut stream);
            request_tx
                .send(request)
                .expect("test server sends recorded request");
            stream
                .write_all(response.as_bytes())
                .expect("test server writes response");
        }
    });

    TestServer {
        base_url,
        requests: request_rx,
    }
}

pub(crate) fn read_request(stream: &mut TcpStream) -> RecordedRequest {
    let mut request = Vec::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes = stream.read(&mut buffer).expect("test server reads request");
        if bytes == 0 {
            break;
        }

        request.extend_from_slice(&buffer[..bytes]);
        if request.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
    }

    let header_end = request
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .expect("request contains headers");
    let mut body = request[(header_end + 4)..].to_vec();
    let headers = String::from_utf8_lossy(&request[..header_end]);
    let mut lines = headers.lines();
    let start_line = lines.next().expect("request has start line");
    let mut start_line_parts = start_line.split_whitespace();
    let method = start_line_parts.next().expect("request has method");
    let target = start_line_parts.next().expect("request has target");
    let path = target.split('?').next().expect("request has path");

    let headers = lines
        .filter_map(|line| {
            let (name, value) = line.split_once(':')?;
            Some((name.trim().to_ascii_lowercase(), value.trim().to_string()))
        })
        .collect::<BTreeMap<_, _>>();
    let content_length = headers
        .get("content-length")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or_default();

    while body.len() < content_length {
        let bytes = stream.read(&mut buffer).expect("test server reads body");
        if bytes == 0 {
            break;
        }
        body.extend_from_slice(&buffer[..bytes]);
    }
    body.truncate(content_length);

    RecordedRequest {
        method: method.to_string(),
        target: target.to_string(),
        path: path.to_string(),
        headers,
        body,
    }
}

pub(crate) fn assert_request(request: &RecordedRequest, method: &str, path: &str) {
    assert_eq!(request.method, method);
    assert_eq!(request.path, path);
}

pub(crate) fn assert_bearer_token(request: &RecordedRequest) {
    assert_eq!(request.header("authorization"), Some("Bearer token"));
}

pub(crate) fn json_response(body: &str) -> String {
    text_response("HTTP/1.1 200 OK", body)
}

pub(crate) fn empty_response(status_line: &str) -> String {
    text_response(status_line, "")
}

pub(crate) fn text_response(status_line: &str, body: &str) -> String {
    format!(
        "{status_line}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

pub(crate) fn refresh_content_length(response: String) -> String {
    let Some((headers, body)) = response.split_once("\r\n\r\n") else {
        return response;
    };
    let headers = headers
        .lines()
        .map(|line| {
            if line.to_ascii_lowercase().starts_with("content-length:") {
                format!("Content-Length: {}", body.len())
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\r\n");

    format!("{headers}\r\n\r\n{body}")
}

/// Tracing subscriber that records warnings and errors as `field=value` text, so tests
/// can assert what the library logs.
#[derive(Clone, Default)]
pub(crate) struct RecordingSubscriber {
    events: Arc<Mutex<Vec<String>>>,
}

impl RecordingSubscriber {
    pub(crate) fn events(&self) -> Vec<String> {
        self.events.lock().unwrap().clone()
    }
}

impl tracing::Subscriber for RecordingSubscriber {
    fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
        // Only warnings and errors; dependencies log a lot at debug level.
        *metadata.level() <= tracing::Level::WARN
    }

    fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(1)
    }

    fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}

    fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}

    fn event(&self, event: &tracing::Event<'_>) {
        let mut text = FieldText::default();
        event.record(&mut text);
        self.events
            .lock()
            .unwrap()
            .push(text.0.trim_end().to_string());
    }

    fn enter(&self, _: &tracing::span::Id) {}

    fn exit(&self, _: &tracing::span::Id) {}
}

#[derive(Default)]
struct FieldText(String);

impl tracing::field::Visit for FieldText {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.push_str(&format!("{}={value:?} ", field.name()));
    }
}
