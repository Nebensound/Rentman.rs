//! Local HTTP test server and assertions shared by the unit tests.

use reqwest::Url;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver},
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
    spawn_delayed_server(
        responses
            .into_iter()
            .map(|response| (Duration::ZERO, response))
            .collect(),
    )
}

/// Like [`spawn_server`], but waits the given time before writing each
/// response, without blocking later connections. A delayed response may find
/// the client already gone (e.g. after a client timeout), so writing it is
/// allowed to fail.
pub(crate) fn spawn_delayed_server(responses: Vec<(Duration, String)>) -> TestServer {
    let listener = TcpListener::bind("127.0.0.1:0").expect("test server binds");
    let address = listener.local_addr().expect("test server has address");
    let base_url = format!("http://{address}");
    let (request_tx, request_rx) = mpsc::channel();
    let responses = responses
        .into_iter()
        .map(|(delay, response)| {
            (
                delay,
                refresh_content_length(response.replace("{base}", &base_url)),
            )
        })
        .collect::<Vec<_>>();

    thread::spawn(move || {
        for (delay, response) in responses {
            let (mut stream, _) = listener.accept().expect("test server accepts request");
            let request = read_request(&mut stream);
            request_tx
                .send(request)
                .expect("test server sends recorded request");
            if delay.is_zero() {
                stream
                    .write_all(response.as_bytes())
                    .expect("test server writes response");
            } else {
                // Answer late on a separate thread, so the next connection
                // (e.g. the client's retry) is accepted right away.
                thread::spawn(move || {
                    thread::sleep(delay);
                    let _ = stream.write_all(response.as_bytes());
                });
            }
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

thread_local! {
    static WARNINGS: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Starts recording the warnings and errors logged on the current thread and
/// returns a handle to read them.
///
/// The recording subscriber is installed once as the global default: scoped
/// subscribers race with tracing's global callsite cache when tests run in
/// parallel. Events are kept per thread, and each `#[tokio::test]` runs on its
/// own thread, so tests do not see each other's logs.
pub(crate) fn capture_warnings() -> WarningCapture {
    static INSTALL: std::sync::Once = std::sync::Once::new();
    INSTALL.call_once(|| {
        tracing::subscriber::set_global_default(RecordingSubscriber)
            .expect("no other global tracing subscriber is installed in tests");
    });
    WARNINGS.with(|events| events.borrow_mut().clear());
    WarningCapture
}

/// Handle to the warnings recorded on the current thread.
pub(crate) struct WarningCapture;

impl WarningCapture {
    pub(crate) fn events(&self) -> Vec<String> {
        WARNINGS.with(|events| events.borrow().clone())
    }
}

struct RecordingSubscriber;

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
        let text = text.0.trim_end().to_string();
        WARNINGS.with(|events| events.borrow_mut().push(text));
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
