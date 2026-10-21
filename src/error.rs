//! Typed errors of the client.

use crate::id::{Id, Resource};
use reqwest::StatusCode;
use std::fmt;

/// Result of a fallible client operation.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error of a Rentman API call.
///
/// Match on [`Error::kind`] to tell failures apart; [`Error::operation`] and
/// [`Error::resource`] say which call failed. For writes, [`Error::outcome`]
/// says whether Rentman may have applied the request.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    operation: Option<&'static str>,
    resource: Option<ResourceRef>,
    earlier_attempt_uncertain: bool,
    acknowledged: bool,
}

/// What went wrong.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Rentman answered with a non-success HTTP status.
    #[error("Rentman API returned {status}: {body}")]
    Status {
        /// HTTP status of the response.
        status: StatusCode,
        /// Response body, usually Rentman's error message.
        body: String,
    },
    /// The request did not complete: no connection, a timeout, or a response
    /// that broke off.
    #[error("Rentman API request failed: {0}")]
    Transport(#[source] reqwest::Error),
    /// Rentman answered successfully, but the body is not valid JSON for the
    /// expected type.
    #[error("Rentman API response body is invalid JSON: {0}")]
    Decode(#[source] serde_json::Error),
    /// Rentman answered with valid JSON that contains a value the client
    /// cannot interpret.
    #[error("unexpected Rentman API response: {0}")]
    UnexpectedResponse(String),
    /// The request could not be built; nothing was sent.
    #[error("invalid Rentman API request: {0}")]
    InvalidRequest(#[source] InvalidRequest),
}

/// Why a request could not be built.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum InvalidRequest {
    /// A path parameter was set that the endpoint path does not contain.
    #[error("the endpoint has no {{{name}}} path parameter")]
    UnknownPathParameter {
        /// Name of the parameter.
        name: String,
    },
    /// The endpoint path still contains unset path parameters.
    #[error("missing path parameters for {path}")]
    MissingPathParameters {
        /// Documented path template of the endpoint.
        path: &'static str,
    },
    /// The endpoint requires a JSON body, but none was set.
    #[error("a {schema} JSON request body is required")]
    MissingBody {
        /// Name of the documented request schema.
        schema: &'static str,
    },
    /// A body was set for an endpoint that does not accept one.
    #[error("the endpoint does not accept a JSON request body")]
    UnexpectedBody,
    /// The query parameters cannot be encoded into the URL.
    #[error("the query cannot be encoded: {0}")]
    Query(#[source] serde_urlencoded::ser::Error),
    /// The request body cannot be encoded as JSON.
    #[error("the body cannot be encoded as JSON: {0}")]
    Body(#[source] serde_json::Error),
    /// The page size is outside the range Rentman documents (1 to 1500).
    #[error("page size {requested} is outside the documented range 1..=1500")]
    PageSize {
        /// Requested number of items per page.
        requested: usize,
    },
    /// The HTTP request is malformed, e.g. a header value with a line break.
    #[error("{0}")]
    Http(#[source] reqwest::Error),
}

/// Rentman item a failed call was about, e.g. `/payments/7`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceRef {
    collection: &'static str,
    id: u64,
}

impl ResourceRef {
    /// Collection of the item, e.g. `payments`.
    pub fn collection(&self) -> &'static str {
        self.collection
    }

    /// Typed ID of the item, when it belongs to resource `R`.
    pub fn id<R: Resource>(&self) -> Option<Id<R>> {
        (self.collection == R::COLLECTION).then_some(Id::new(self.id))
    }
}

impl<R: Resource> From<Id<R>> for ResourceRef {
    fn from(id: Id<R>) -> Self {
        Self {
            collection: R::COLLECTION,
            id: id.get(),
        }
    }
}

/// Displays the item's link, e.g. `/payments/7`.
impl fmt::Display for ResourceRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/{}/{}", self.collection, self.id)
    }
}

/// Whether Rentman applied a request that failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestOutcome {
    /// Rentman did not apply the request; repeating it is safe.
    NotApplied,
    /// Rentman applied the request: it answered with a success status, and only
    /// receiving or decoding the response body failed.
    Applied,
    /// Rentman may or may not have applied the request. Reconcile by reading
    /// the affected data before repeating a request that creates data.
    Unknown,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            operation: None,
            resource: None,
            earlier_attempt_uncertain: false,
            acknowledged: false,
        }
    }

    pub(crate) fn invalid_request(reason: InvalidRequest) -> Self {
        Self::new(ErrorKind::InvalidRequest(reason))
    }

    /// Records the OpenAPI operation the error belongs to.
    pub(crate) fn in_operation(mut self, operation: &'static str) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Records the resource the failed call was about.
    pub(crate) fn about(mut self, resource: impl Into<ResourceRef>) -> Self {
        self.resource = Some(resource.into());
        self
    }

    /// Records that an earlier attempt of the same request, or an earlier
    /// server in a redirect chain, may have applied it, so this error alone
    /// does not prove nothing happened. Once recorded, it stays recorded.
    pub(crate) fn after_uncertain_attempt(mut self, uncertain: bool) -> Self {
        self.earlier_attempt_uncertain |= uncertain;
        self
    }

    /// Records that Rentman answered the request with a success status and
    /// only receiving or reading the response failed afterwards.
    pub(crate) fn after_success(mut self) -> Self {
        self.acknowledged = true;
        self
    }

    /// What went wrong.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// OpenAPI operation ID of the failed call, e.g. `updatePayment`.
    pub fn operation(&self) -> Option<&'static str> {
        self.operation
    }

    /// Resource the failed call was about, if it was about a single one.
    pub fn resource(&self) -> Option<ResourceRef> {
        self.resource
    }

    /// HTTP status, when Rentman answered with a non-success status.
    pub fn status(&self) -> Option<StatusCode> {
        match &self.kind {
            ErrorKind::Status { status, .. } => Some(*status),
            _ => None,
        }
    }

    /// Whether Rentman may have applied the failed request.
    pub fn outcome(&self) -> RequestOutcome {
        if self.acknowledged {
            return RequestOutcome::Applied;
        }
        let outcome = match &self.kind {
            ErrorKind::InvalidRequest(_) => RequestOutcome::NotApplied,
            // Writes do not follow redirects; a redirect answer gives no
            // proof either way whether the write was applied.
            ErrorKind::Status { status, .. }
                if status.is_server_error() || status.is_redirection() =>
            {
                RequestOutcome::Unknown
            }
            ErrorKind::Status { .. } => RequestOutcome::NotApplied,
            ErrorKind::Transport(error) if error.is_connect() => RequestOutcome::NotApplied,
            ErrorKind::Transport(_) => RequestOutcome::Unknown,
            ErrorKind::Decode(_) | ErrorKind::UnexpectedResponse(_) => RequestOutcome::Applied,
        };
        if self.earlier_attempt_uncertain && outcome == RequestOutcome::NotApplied {
            RequestOutcome::Unknown
        } else {
            outcome
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.operation, self.resource) {
            (Some(operation), Some(resource)) => {
                write!(
                    f,
                    "Rentman {operation} failed for {resource}: {}",
                    self.kind
                )
            }
            (Some(operation), None) => write!(f, "Rentman {operation} failed: {}", self.kind),
            (None, _) => self.kind.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        std::error::Error::source(&self.kind)
    }
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::model::{Invoice, Payment, PaymentId};
    use std::error::Error as _;

    fn status(code: u16) -> Error {
        Error::new(ErrorKind::Status {
            status: StatusCode::from_u16(code).unwrap(),
            body: "body".to_string(),
        })
    }

    fn decode_error() -> serde_json::Error {
        serde_json::from_str::<u8>("x").unwrap_err()
    }

    fn builder_error() -> reqwest::Error {
        reqwest::Client::new()
            .get("http://example.test/")
            .header("x-invalid", "line\nbreak")
            .build()
            .unwrap_err()
    }

    async fn connect_error() -> reqwest::Error {
        let address = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap();
        reqwest::get(format!("http://{address}/"))
            .await
            .unwrap_err()
    }

    #[test]
    fn describes_operation_and_resource() {
        let error = status(404)
            .in_operation("updatePayment")
            .about(PaymentId::new(7));
        assert_eq!(
            error.to_string(),
            "Rentman updatePayment failed for /payments/7: Rentman API returned 404 Not Found: body"
        );
        assert_eq!(error.operation(), Some("updatePayment"));
        let resource = error.resource().unwrap();
        assert_eq!(resource, ResourceRef::from(PaymentId::new(7)));
        assert_eq!(resource.collection(), "payments");
        assert_eq!(resource.id::<Payment>(), Some(PaymentId::new(7)));
        assert_eq!(resource.id::<Invoice>(), None);
        assert_eq!(resource.to_string(), "/payments/7");

        assert_eq!(
            status(404).in_operation("getFactuurCollection").to_string(),
            "Rentman getFactuurCollection failed: Rentman API returned 404 Not Found: body"
        );
        assert_eq!(
            status(404).to_string(),
            "Rentman API returned 404 Not Found: body"
        );
    }

    #[test]
    fn exposes_the_status_only_for_status_errors() {
        assert_eq!(status(409).status(), Some(StatusCode::CONFLICT));
        assert_eq!(Error::new(ErrorKind::Decode(decode_error())).status(), None);
    }

    #[tokio::test]
    async fn classifies_whether_a_request_may_have_been_applied() {
        assert_eq!(status(400).outcome(), RequestOutcome::NotApplied);
        assert_eq!(status(429).outcome(), RequestOutcome::NotApplied);
        assert_eq!(status(500).outcome(), RequestOutcome::Unknown);
        assert_eq!(status(303).outcome(), RequestOutcome::Unknown);
        assert_eq!(
            Error::invalid_request(InvalidRequest::UnexpectedBody).outcome(),
            RequestOutcome::NotApplied
        );
        assert_eq!(
            Error::new(ErrorKind::Transport(connect_error().await)).outcome(),
            RequestOutcome::NotApplied
        );
        assert_eq!(
            Error::new(ErrorKind::Transport(builder_error())).outcome(),
            RequestOutcome::Unknown
        );
        assert_eq!(
            Error::new(ErrorKind::Decode(decode_error())).outcome(),
            RequestOutcome::Applied
        );
        assert_eq!(
            Error::new(ErrorKind::UnexpectedResponse("odd".into())).outcome(),
            RequestOutcome::Applied
        );

        // An earlier attempt that may have been applied overrides "not
        // applied", but not the other outcomes.
        assert_eq!(
            status(400).after_uncertain_attempt(true).outcome(),
            RequestOutcome::Unknown
        );
        // Recorded uncertainty is not cleared by a later "certain" call.
        assert_eq!(
            status(400)
                .after_uncertain_attempt(true)
                .after_uncertain_attempt(false)
                .outcome(),
            RequestOutcome::Unknown
        );
        // A success status was received: a failure afterwards means applied.
        assert_eq!(
            Error::new(ErrorKind::Transport(builder_error()))
                .after_success()
                .outcome(),
            RequestOutcome::Applied
        );
        assert_eq!(
            Error::new(ErrorKind::Decode(decode_error()))
                .after_uncertain_attempt(true)
                .outcome(),
            RequestOutcome::Applied
        );
    }

    #[test]
    fn describes_every_invalid_request() {
        let cases = [
            (
                InvalidRequest::UnknownPathParameter { name: "x".into() },
                "the endpoint has no {x} path parameter",
            ),
            (
                InvalidRequest::MissingPathParameters { path: "/a/{id}" },
                "missing path parameters for /a/{id}",
            ),
            (
                InvalidRequest::MissingBody {
                    schema: "PaymentRequest",
                },
                "a PaymentRequest JSON request body is required",
            ),
            (
                InvalidRequest::UnexpectedBody,
                "the endpoint does not accept a JSON request body",
            ),
            (
                InvalidRequest::PageSize { requested: 0 },
                "page size 0 is outside the documented range 1..=1500",
            ),
        ];
        for (reason, text) in cases {
            let error = Error::invalid_request(reason);
            assert_eq!(
                error.to_string(),
                format!("invalid Rentman API request: {text}")
            );
            assert!(error.source().is_some());
        }

        let error = Error::invalid_request(InvalidRequest::Body(decode_error()));
        assert!(
            error
                .to_string()
                .starts_with("invalid Rentman API request: the body cannot be encoded as JSON: ")
        );
        let error = Error::invalid_request(InvalidRequest::Http(builder_error()));
        assert!(
            error
                .to_string()
                .starts_with("invalid Rentman API request: ")
        );
    }

    #[test]
    fn exposes_the_underlying_cause_as_source() {
        let error = Error::new(ErrorKind::Decode(decode_error()));
        assert!(
            error
                .to_string()
                .starts_with("Rentman API response body is invalid JSON: ")
        );
        let source = error.source().expect("decode errors have a source");
        assert!(source.is::<serde_json::Error>());

        let error = Error::new(ErrorKind::UnexpectedResponse("odd".into()));
        assert_eq!(error.to_string(), "unexpected Rentman API response: odd");
        assert!(error.source().is_none());
    }

    #[test]
    fn errors_can_cross_threads_and_convert_into_boxed_errors() {
        fn assert_thread_safe_error<T: std::error::Error + Send + Sync + 'static>() {}
        assert_thread_safe_error::<Error>();

        let boxed: Box<dyn std::error::Error + Send + Sync> = status(500).into();
        assert_eq!(
            boxed.to_string(),
            "Rentman API returned 500 Internal Server Error: body"
        );
    }
}
