//! Typed requests for every documented Rentman API endpoint.

use crate::{
    RentmanClient, client::RentmanRequestBuilder, endpoint, model, transport::RetryPolicy,
};
use anyhow::{Context, Result, anyhow};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::{Method, Url};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{fmt, marker::PhantomData};
use url::form_urlencoded;

/// Request builder for a typed Rentman API endpoint.
pub struct RentmanEndpointRequest<E: endpoint::Endpoint> {
    client: RentmanClient,
    path_params: Vec<(String, String)>,
    query_params: Vec<(String, String)>,
    body: Option<Value>,
    endpoint: PhantomData<E>,
}

impl<E: endpoint::Endpoint> RentmanEndpointRequest<E> {
    pub(crate) fn new(client: RentmanClient) -> Self {
        Self {
            client,
            path_params: Vec::new(),
            query_params: Vec::new(),
            body: None,
            endpoint: PhantomData,
        }
    }

    /// Sets a raw path parameter used by the endpoint path template.
    ///
    /// Prefer typed helpers such as `id` when the endpoint exposes one.
    pub fn path_param(mut self, name: impl Into<String>, value: impl fmt::Display) -> Self {
        self.path_params.push((name.into(), value.to_string()));
        self
    }

    /// Adds query parameters to the request.
    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Result<Self> {
        self.query_params.extend(serialize_query_params(query)?);
        Ok(self)
    }

    /// Adds one query parameter to the request.
    pub fn query_param(mut self, name: impl Into<String>, value: impl fmt::Display) -> Self {
        self.query_params.push((name.into(), value.to_string()));
        self
    }

    /// Sets the typed JSON request body.
    // Coverage is disabled because the error branch is unreachable: the
    // generated request models are plain structs and enums with string keys,
    // which `serde_json::to_value` always serializes.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn body(mut self, body: E::Request) -> Result<Self>
    where
        E::Request: Serialize,
    {
        self.body =
            Some(serde_json::to_value(body).context("Rentman request body is invalid JSON value")?);
        Ok(self)
    }

    /// Sends the request and returns the typed Rentman API response.
    pub async fn send(self) -> Result<E::Response>
    where
        E: endpoint::ExecutableEndpoint,
        E::Request: Serialize,
        E::Response: DeserializeOwned,
    {
        send_typed_endpoint(
            &self.client,
            E::SPEC,
            &self.path_params,
            &self.query_params,
            self.body,
            self.client.retry_policy,
        )
        .await
    }

    /// Sends the request exactly once, without retrying.
    ///
    /// Use this for non-idempotent writes such as creating items: when only
    /// the response gets lost, a retry would write twice. On an error the
    /// outcome is unknown and the caller has to reconcile, e.g. by reading the
    /// affected collection.
    pub async fn send_once(self) -> Result<E::Response>
    where
        E: endpoint::ExecutableEndpoint,
        E::Request: Serialize,
        E::Response: DeserializeOwned,
    {
        send_typed_endpoint(
            &self.client,
            E::SPEC,
            &self.path_params,
            &self.query_params,
            self.body,
            self.client.retry_policy.single_attempt(),
        )
        .await
    }
}

impl<E: endpoint::EndpointWithId> RentmanEndpointRequest<E> {
    /// Sets the documented integer `{id}` path parameter.
    pub fn id(self, id: model::RentmanId) -> Self {
        self.path_param("id", id)
    }
}

async fn send_typed_endpoint<T: DeserializeOwned>(
    client: &RentmanClient,
    spec: endpoint::EndpointSpec,
    path_params: &[(String, String)],
    query_params: &[(String, String)],
    body: Option<Value>,
    retry_policy: RetryPolicy,
) -> Result<T> {
    build_endpoint_request(client, spec, path_params, query_params, body)?
        .send_json_with(retry_policy)
        .await
        .with_context(|| format!("Rentman endpoint {} failed", spec.operation_id))
}

fn build_endpoint_request(
    client: &RentmanClient,
    spec: endpoint::EndpointSpec,
    path_params: &[(String, String)],
    query_params: &[(String, String)],
    body: Option<Value>,
) -> Result<RentmanRequestBuilder> {
    let method =
        Method::from_bytes(spec.method.as_bytes()).expect("generated HTTP method is valid");
    let mut url = endpoint_url(spec, &client.base_url, path_params)?;
    append_query_params(&mut url, query_params);

    let mut request = client.request_url(method, url);
    match (spec.request, body) {
        (endpoint::EndpointRequestSpec::None, None) => {}
        (endpoint::EndpointRequestSpec::None, Some(_)) => {
            return Err(anyhow!(
                "Rentman endpoint {} does not accept a JSON request body",
                spec.operation_id
            ));
        }
        (endpoint::EndpointRequestSpec::Json(_), Some(body)) => {
            request = request.json(&body);
        }
        (endpoint::EndpointRequestSpec::Json(schema), None) => {
            return Err(anyhow!(
                "Rentman endpoint {} requires a {schema} JSON request body",
                spec.operation_id
            ));
        }
    }

    Ok(request)
}

fn serialize_query_params<T: Serialize + ?Sized>(query: &T) -> Result<Vec<(String, String)>> {
    let encoded = serde_urlencoded::to_string(query).context("Rentman query is invalid")?;
    Ok(parse_query_params(&encoded))
}

fn parse_query_params(encoded: &str) -> Vec<(String, String)> {
    form_urlencoded::parse(encoded.as_bytes())
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect()
}

fn append_query_params(url: &mut Url, query_params: &[(String, String)]) {
    let mut pairs = url.query_pairs_mut();
    for (name, value) in query_params {
        pairs.append_pair(name, value);
    }
}

fn endpoint_url(
    spec: endpoint::EndpointSpec,
    base_url: &Url,
    path_params: &[(String, String)],
) -> Result<Url> {
    let mut path = spec.path.to_string();
    for (name, value) in path_params {
        let placeholder = format!("{{{name}}}");
        if !path.contains(&placeholder) {
            return Err(anyhow!(
                "Rentman endpoint {} has no {{{name}}} path parameter",
                spec.operation_id
            ));
        }
        let encoded = utf8_percent_encode(value, NON_ALPHANUMERIC).to_string();
        path = path.replace(&placeholder, &encoded);
    }

    if path.contains('{') || path.contains('}') {
        return Err(anyhow!(
            "Rentman endpoint {} is missing path parameters for {}",
            spec.operation_id,
            spec.path
        ));
    }

    base_url
        .join(path.trim_start_matches('/'))
        .context("generated Rentman endpoint path is valid")
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {

    use crate::test_support::*;
    use crate::{
        RentmanApiToken, RentmanClient, Url,
        endpoint::{
            CreatePaymentEndpoint, DeleteAccessoryEndpoint, GetAccessoryCollectionEndpoint,
            GetAccessoryItemEndpoint,
        },
        model::RentmanId,
        model::{
            EmailAddress, ItemResponse, NoContent, NoRequest, PaymentRequest,
            PaymentRequestPaymentImportSource, ResourceReference,
        },
    };
    use chrono::DateTime;

    use rust_decimal::Decimal;

    #[tokio::test]
    async fn executes_typed_endpoint_with_path_query_body_and_response() {
        let server = spawn_server(vec![json_response(
            r#"{"data":{"id":99,"created":null,"modified":null,"creator":null,"displayname":"Payment 99","invoice":"/invoices/7","moment":"2026-07-05T00:00:00.000+02:00","amount":42.5,"description":"Manual booking","payment_import_source":"publicapi"},"itemCount":1,"limit":1}"#,
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );
        let payload = PaymentRequest {
            moment: DateTime::parse_from_rfc3339("2026-07-05T00:00:00+02:00").unwrap(),
            amount: Some(Decimal::new(425, 1)),
            description: Some("Manual booking".to_string()),
            payment_import_source: Some(PaymentRequestPaymentImportSource::Publicapi),
        };

        let response: ItemResponse<_> = client
            .endpoint::<CreatePaymentEndpoint>()
            .id(RentmanId(7))
            .query(&[("limit", "1")])
            .unwrap()
            .query_param("fields", "id,moment,invoice,amount,description")
            .body(payload)
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(response.data.id.to_string(), "99");
        assert_eq!(response.data.invoice.as_str(), "/invoices/7");
        assert_eq!(response.data.description, "Manual booking");

        let request = server.next_request();
        assert_request(&request, "POST", "/invoices/7/payments");
        assert_bearer_token(&request);
        assert_eq!(request.query_value("limit").as_deref(), Some("1"));
        assert_eq!(
            request.query_value("fields").as_deref(),
            Some("id,moment,invoice,amount,description")
        );
        let body = request.json_body();
        assert_eq!(body["moment"], "2026-07-05T00:00:00+02:00");
        assert_eq!(body["amount"], serde_json::json!(42.5));
        assert_eq!(body["description"], "Manual booking");
        assert_eq!(body["payment_import_source"], "publicapi");
    }

    #[tokio::test]
    async fn executes_typed_endpoint_without_response_body() {
        let server = spawn_server(vec![empty_response("HTTP/1.1 204 No Content")]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let response = client
            .endpoint::<DeleteAccessoryEndpoint>()
            .id(RentmanId(10))
            .send()
            .await
            .unwrap();

        assert_eq!(response, NoContent);
        let request = server.next_request();
        assert_request(&request, "DELETE", "/accessories/10");
        assert_bearer_token(&request);
    }

    #[tokio::test]
    async fn rejects_typed_endpoint_without_required_body() {
        let client = RentmanClient::with_base_url(
            Url::parse("http://example.test/").unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client
            .endpoint::<CreatePaymentEndpoint>()
            .id(RentmanId(7))
            .send()
            .await
            .unwrap_err();

        assert!(error.to_string().contains(
            "Rentman endpoint createPayment requires a PaymentRequest JSON request body"
        ));
    }

    #[tokio::test]
    async fn rejects_typed_endpoint_body_when_endpoint_has_no_body() {
        let client = RentmanClient::with_base_url(
            Url::parse("http://example.test/").unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client
            .endpoint::<DeleteAccessoryEndpoint>()
            .id(RentmanId(10))
            .body(NoRequest)
            .unwrap()
            .send()
            .await
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman endpoint deleteAccessory does not accept a JSON request body")
        );
    }

    #[tokio::test]
    async fn rejects_typed_endpoint_missing_path_parameter() {
        let client = RentmanClient::with_base_url(
            Url::parse("http://example.test/").unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client
            .endpoint::<GetAccessoryItemEndpoint>()
            .send()
            .await
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman endpoint getAccessoryItem is missing path parameters")
        );
    }

    #[tokio::test]
    async fn rejects_unknown_typed_endpoint_path_parameter() {
        let client = RentmanClient::with_base_url(
            Url::parse("http://example.test/").unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client
            .endpoint::<DeleteAccessoryEndpoint>()
            .path_param("unknown", 10)
            .send()
            .await
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman endpoint deleteAccessory has no {unknown} path parameter")
        );
    }

    #[tokio::test]
    async fn send_once_does_not_retry_transient_errors() {
        let server = spawn_server(vec![
            text_response("HTTP/1.1 503 Service Unavailable", "later"),
            empty_response("HTTP/1.1 204 No Content"),
        ]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client
            .endpoint::<DeleteAccessoryEndpoint>()
            .id(RentmanId(10))
            .send_once()
            .await
            .unwrap_err();
        assert!(format!("{error:#}").contains("503"), "{error:#}");
        assert_request(&server.next_request(), "DELETE", "/accessories/10");

        // Without a retry the second prepared response is still unused.
        let response = client
            .endpoint::<DeleteAccessoryEndpoint>()
            .id(RentmanId(10))
            .send_once()
            .await
            .unwrap();
        assert_eq!(response, NoContent);
    }

    #[test]
    fn typed_query_accepts_flat_values_and_reports_serialization_errors() {
        let client = RentmanClient::with_base_url(
            Url::parse("http://example.test/").unwrap(),
            RentmanApiToken::new("token"),
        );
        // One query type for both outcomes: flat fields serialize, nested
        // objects cannot be expressed as URL query parameters.
        let flat = serde_json::json!({"limit": 5, "fields": "id,name"});
        let nested = serde_json::json!({"filter": {"name": "x"}});

        let request = client
            .endpoint::<GetAccessoryCollectionEndpoint>()
            .query(&flat)
            .unwrap();
        assert_eq!(
            request.query_params,
            vec![
                ("fields".to_string(), "id,name".to_string()),
                ("limit".to_string(), "5".to_string()),
            ]
        );

        let error = match client
            .endpoint::<GetAccessoryCollectionEndpoint>()
            .query(&nested)
        {
            Ok(_) => panic!("nested query should fail"),
            Err(error) => error,
        };
        assert!(error.to_string().contains("Rentman query is invalid"));
    }

    #[test]
    fn typed_models_reject_invalid_json_types() {
        let error = serde_json::from_str::<ResourceReference>("42").unwrap_err();
        assert!(error.to_string().contains("invalid type"));

        let error = serde_json::from_str::<EmailAddress>("42").unwrap_err();
        assert!(error.to_string().contains("invalid type"));
    }

    #[tokio::test]
    async fn typed_endpoint_errors_include_operation_context() {
        let server = spawn_server(vec![text_response(
            "HTTP/1.1 500 Internal Server Error",
            "rentman broken",
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        );

        let error = client
            .endpoint::<GetAccessoryCollectionEndpoint>()
            .send()
            .await
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("Rentman endpoint getAccessoryCollection failed")
        );
        assert_request(&server.next_request(), "GET", "/accessories");
    }
}
