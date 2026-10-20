//! Reading complete collections page by page.

use crate::{
    RentmanClient,
    error::{Error, ErrorKind, InvalidRequest, Result},
    model::CollectionResponse,
};
use reqwest::{Method, Url};
use serde::de::DeserializeOwned;
use std::collections::HashSet;

/// Number of items Rentman returns per page of a collection.
///
/// Rentman documents a default of 300 and a maximum of 1500 items per page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PageSize(usize);

impl PageSize {
    /// Rentman's page size when a request sets no `limit`.
    pub const DEFAULT: Self = Self(300);
    /// Largest page size Rentman accepts.
    pub const MAX: Self = Self(1500);

    /// Creates a page size between 1 and [`Self::MAX`].
    ///
    /// Fails with [`InvalidRequest::PageSize`] outside that range.
    pub fn new(items: usize) -> Result<Self> {
        if (1..=Self::MAX.0).contains(&items) {
            Ok(Self(items))
        } else {
            Err(Error::invalid_request(InvalidRequest::PageSize {
                requested: items,
            }))
        }
    }

    /// Number of items per page.
    pub fn get(self) -> usize {
        self.0
    }
}

impl Default for PageSize {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// A collection read from its first to its last page.
pub(crate) struct CollectionRequest<'a> {
    /// Collection path relative to the base URL, e.g. `equipment`.
    pub(crate) path: &'a str,
    /// Query parameters of the first page, e.g. `fields` and filters. Rentman
    /// carries them over into every `next_page_url`.
    pub(crate) query: &'a [(&'a str, &'a str)],
    pub(crate) page_size: PageSize,
    /// OpenAPI operation ID, attached to every error.
    pub(crate) operation: &'static str,
}

/// Where the next page comes from.
#[derive(Debug, PartialEq, Eq)]
enum NextPage {
    /// Follow Rentman's `next_page_url` as it is.
    Cursor(Url),
    /// Request the collection again from this offset (older responses without
    /// `next_page_url`).
    Offset(usize),
    /// The collection is complete.
    Done,
}

impl RentmanClient {
    /// Reads every page of a collection and returns all items.
    ///
    /// Follows `next_page_url` until it is `null`, as Rentman documents. A
    /// full page without `next_page_url` falls back to the next offset, for
    /// older responses. Each page is retried like any other read.
    pub(crate) async fn read_all<T: DeserializeOwned>(
        &self,
        request: CollectionRequest<'_>,
    ) -> Result<Vec<T>> {
        let page_size = request.page_size.get().to_string();
        let mut items = Vec::new();
        let mut seen = HashSet::new();
        let mut offset = 0;
        let mut next = NextPage::Offset(0);

        loop {
            let page_request = match next {
                NextPage::Cursor(url) => self.request_url(Method::GET, url),
                NextPage::Offset(offset) => self
                    .request(Method::GET, request.path)
                    .query(request.query)
                    .query(&[
                        ("limit", page_size.as_str()),
                        ("offset", &offset.to_string()),
                    ]),
                NextPage::Done => return Ok(items),
            };
            let page: CollectionResponse<T> = page_request
                .send_json()
                .await
                .map_err(|error| error.in_operation(request.operation))?;

            offset += page.limit;
            next = next_page(
                &self.base_url,
                &mut seen,
                page.next_page_url,
                page.item_count,
                page.limit,
                offset,
            )
            .map_err(|error| error.in_operation(request.operation))?;
            items.extend(page.data);
        }
    }
}

/// Decides where the page after the current one comes from.
///
/// A `next_page_url` that was already followed, or that points away from the
/// configured API (it would receive the API token), is an unexpected response
/// instead of an endless or leaking loop.
fn next_page(
    base_url: &Url,
    seen: &mut HashSet<Url>,
    next_page_url: Option<Url>,
    item_count: usize,
    limit: usize,
    offset: usize,
) -> Result<NextPage> {
    match next_page_url {
        Some(url) if url.origin() != base_url.origin() => Err(unexpected(format!(
            "next_page_url {url} leaves the API at {base_url}"
        ))),
        Some(url) if !seen.insert(url.clone()) => Err(unexpected(format!(
            "next_page_url {url} repeats an earlier page"
        ))),
        Some(url) => Ok(NextPage::Cursor(url)),
        None if limit > 0 && item_count >= limit => Ok(NextPage::Offset(offset)),
        None => Ok(NextPage::Done),
    }
}

fn unexpected(message: String) -> Error {
    Error::new(ErrorKind::UnexpectedResponse(message))
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::test_support::*;
    use crate::{RentmanApiToken, RentmanClientConfig, RequestOutcome};
    use reqwest::StatusCode;
    use serde::Deserialize;
    use std::{num::NonZeroUsize, time::Duration};

    #[derive(Debug, PartialEq, Deserialize)]
    struct Item {
        id: u64,
    }

    fn client(server: &TestServer) -> RentmanClient {
        let config = RentmanClientConfig::default()
            .with_max_attempts(NonZeroUsize::new(2).unwrap())
            .with_retry_base_delay(Duration::from_millis(1));
        RentmanClient::with_config(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
            config,
        )
        .unwrap()
    }

    fn request<'a>(query: &'a [(&'a str, &'a str)], page_size: PageSize) -> CollectionRequest<'a> {
        CollectionRequest {
            path: "contacts",
            query,
            page_size,
            operation: "getContactCollection",
        }
    }

    fn ids(items: &[Item]) -> Vec<u64> {
        items.iter().map(|item| item.id).collect()
    }

    #[test]
    fn validates_the_documented_page_size_range() {
        assert_eq!(PageSize::default(), PageSize::DEFAULT);
        assert_eq!(PageSize::DEFAULT.get(), 300);
        assert_eq!(PageSize::MAX.get(), 1500);
        assert_eq!(PageSize::new(1).unwrap().get(), 1);
        assert_eq!(PageSize::new(1500).unwrap(), PageSize::MAX);

        for requested in [0, 1501, usize::MAX] {
            let error = PageSize::new(requested).unwrap_err();
            assert!(matches!(
                error.kind(),
                ErrorKind::InvalidRequest(InvalidRequest::PageSize { requested: r }) if *r == requested
            ));
            assert_eq!(error.outcome(), RequestOutcome::NotApplied);
        }
    }

    #[tokio::test]
    async fn follows_the_cursor_unchanged_until_it_is_null() {
        let server = spawn_server(vec![
            json_response(
                r#"{"itemCount":2,"limit":2,"offset":0,"next_page_url":"{base}/contacts?country=gb&fields=id&cursor=abc","data":[{"id":1},{"id":2}]}"#,
            ),
            json_response(
                r#"{"itemCount":1,"limit":2,"offset":0,"next_page_url":null,"data":[{"id":3}]}"#,
            ),
        ]);
        let client = client(&server);
        let query = [("country", "gb"), ("fields", "id")];

        let items: Vec<Item> = client
            .read_all(request(&query, PageSize::new(2).unwrap()))
            .await
            .unwrap();

        assert_eq!(ids(&items), [1, 2, 3]);
        let first = server.next_request();
        assert_request(&first, "GET", "/contacts");
        assert_bearer_token(&first);
        assert_eq!(first.query_value("country").as_deref(), Some("gb"));
        assert_eq!(first.query_value("fields").as_deref(), Some("id"));
        assert_eq!(first.query_value("limit").as_deref(), Some("2"));
        assert_eq!(first.query_value("offset").as_deref(), Some("0"));

        // The cursor carries filters and page size; nothing is added to it.
        let second = server.next_request();
        assert_request(&second, "GET", "/contacts");
        assert_bearer_token(&second);
        assert_eq!(second.query_value("cursor").as_deref(), Some("abc"));
        assert_eq!(second.query_value("country").as_deref(), Some("gb"));
        assert_eq!(second.query_value("fields").as_deref(), Some("id"));
        assert_eq!(second.query_value("limit"), None);
        assert_eq!(second.query_value("offset"), None);
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn falls_back_to_offsets_and_keeps_the_filters() {
        let server = spawn_server(vec![
            json_response(r#"{"itemCount":2,"limit":2,"data":[{"id":1},{"id":2}]}"#),
            json_response(r#"{"itemCount":2,"limit":2,"data":[{"id":3},{"id":4}]}"#),
            json_response(r#"{"itemCount":1,"limit":2,"data":[{"id":5}]}"#),
        ]);
        let client = client(&server);
        let query = [("country", "gb"), ("distance[lte]", "300")];

        let items: Vec<Item> = client
            .read_all(request(&query, PageSize::new(2).unwrap()))
            .await
            .unwrap();

        assert_eq!(ids(&items), [1, 2, 3, 4, 5]);
        for offset in ["0", "2", "4"] {
            let page = server.next_request();
            assert_request(&page, "GET", "/contacts");
            assert_eq!(page.query_value("offset").as_deref(), Some(offset));
            assert_eq!(page.query_value("limit").as_deref(), Some("2"));
            assert_eq!(page.query_value("country").as_deref(), Some("gb"));
            assert_eq!(page.query_value("distance[lte]").as_deref(), Some("300"));
        }
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn sends_the_default_and_the_maximum_page_size() {
        let server = spawn_server(vec![
            json_response(r#"{"itemCount":0,"limit":300,"data":[]}"#),
            json_response(r#"{"itemCount":0,"limit":1500,"data":[]}"#),
        ]);
        let client = client(&server);

        for page_size in [PageSize::default(), PageSize::MAX] {
            let items: Vec<Item> = client.read_all(request(&[], page_size)).await.unwrap();
            assert!(items.is_empty());
            let page = server.next_request();
            assert_eq!(page.query_value("limit"), Some(page_size.get().to_string()));
        }
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn stops_on_an_empty_collection_and_on_limit_zero() {
        let server = spawn_server(vec![
            json_response(r#"{"itemCount":0,"limit":300,"next_page_url":null,"data":[]}"#),
            json_response(r#"{"itemCount":0,"limit":0,"data":[]}"#),
        ]);
        let client = client(&server);

        for _ in 0..2 {
            let items: Vec<Item> = client
                .read_all(request(&[], PageSize::default()))
                .await
                .unwrap();
            assert!(items.is_empty());
            assert_request(&server.next_request(), "GET", "/contacts");
        }
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn rejects_a_repeated_cursor() {
        let page = r#"{"itemCount":1,"limit":1,"next_page_url":"{base}/contacts?cursor=same","data":[{"id":1}]}"#;
        let server = spawn_server(vec![json_response(page), json_response(page)]);
        let client = client(&server);

        let error = client
            .read_all::<Item>(request(&[], PageSize::default()))
            .await
            .unwrap_err();

        assert!(matches!(error.kind(), ErrorKind::UnexpectedResponse(_)));
        assert_eq!(error.operation(), Some("getContactCollection"));
        assert_request(&server.next_request(), "GET", "/contacts");
        assert_eq!(
            server.next_request().query_value("cursor").as_deref(),
            Some("same")
        );
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn rejects_a_cursor_that_leaves_the_api() {
        let server = spawn_server(vec![json_response(
            r#"{"itemCount":1,"limit":1,"next_page_url":"http://elsewhere.test/contacts?cursor=x","data":[{"id":1}]}"#,
        )]);
        let client = client(&server);

        let error = client
            .read_all::<Item>(request(&[], PageSize::default()))
            .await
            .unwrap_err();

        assert!(matches!(error.kind(), ErrorKind::UnexpectedResponse(_)));
        assert_eq!(error.operation(), Some("getContactCollection"));
        assert_request(&server.next_request(), "GET", "/contacts");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn fails_with_the_error_of_a_later_page() {
        let server = spawn_server(vec![
            json_response(
                r#"{"itemCount":1,"limit":1,"next_page_url":"{base}/contacts?cursor=2","data":[{"id":1}]}"#,
            ),
            text_response("HTTP/1.1 400 Bad Request", "broken cursor"),
        ]);
        let client = client(&server);

        let error = client
            .read_all::<Item>(request(&[], PageSize::default()))
            .await
            .unwrap_err();

        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        assert_eq!(error.operation(), Some("getContactCollection"));
        server.next_request();
        assert_eq!(
            server.next_request().query_value("cursor").as_deref(),
            Some("2")
        );
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn retries_a_page_after_a_transient_error() {
        let server = spawn_server(vec![
            json_response(
                r#"{"itemCount":1,"limit":1,"next_page_url":"{base}/contacts?cursor=2","data":[{"id":1}]}"#,
            ),
            text_response("HTTP/1.1 503 Service Unavailable", "later"),
            json_response(r#"{"itemCount":1,"limit":2,"next_page_url":null,"data":[{"id":2}]}"#),
        ]);
        let client = client(&server);

        let items: Vec<Item> = client
            .read_all(request(&[], PageSize::default()))
            .await
            .unwrap();

        assert_eq!(ids(&items), [1, 2]);
        server.next_request();
        for _ in 0..2 {
            assert_eq!(
                server.next_request().query_value("cursor").as_deref(),
                Some("2")
            );
        }
        server.assert_no_more_requests();
    }
}
