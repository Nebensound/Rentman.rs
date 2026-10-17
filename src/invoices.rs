//! Invoices and their payments.

use crate::{
    RentmanAmount,
    client::{DEFAULT_PAGE_LIMIT, RentmanClient},
    error::{Error, ErrorKind, Resource, Result},
    ids::{InvoiceId, InvoiceNumber, PaymentId},
    model,
};
use chrono::{DateTime, FixedOffset, SecondsFormat};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Payment status of a Rentman invoice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(from = "bool")]
pub enum InvoicePaymentStatus {
    /// The invoice is paid.
    Paid,
    /// The invoice is open.
    Open,
}

impl From<bool> for InvoicePaymentStatus {
    fn from(is_paid: bool) -> Self {
        if is_paid { Self::Paid } else { Self::Open }
    }
}

/// Import source for Rentman payments written by this client.
///
/// Rentman knows additional values. The domain facade deliberately writes only
/// the values modeled here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentImportSource {
    /// Payment was imported through the public API.
    PublicApi,
    /// Payment is disabled or has no import source.
    None,
}

/// Rentman invoice with the fields needed for payment reconciliation.
#[derive(Debug, Clone, Deserialize)]
pub struct RentmanInvoice {
    /// Internal Rentman invoice ID.
    pub id: InvoiceId,
    /// Domain-level Rentman invoice number.
    pub number: InvoiceNumber,
    /// Payment status translated from Rentman's `is_paid` field.
    #[serde(rename = "is_paid")]
    pub payment_status: InvoicePaymentStatus,
    /// Gross invoice total (`price_invat`). `None` when Rentman omits the
    /// field.
    #[serde(default)]
    pub price_invat: Option<RentmanAmount>,
}

/// Existing Rentman payment.
#[derive(Debug, Clone, Deserialize)]
pub struct RentmanPayment {
    /// Internal Rentman payment ID.
    pub id: PaymentId,
    /// Payment timestamp.
    pub moment: DateTime<FixedOffset>,
    /// Payment amount. `None` when Rentman omits the field.
    pub amount: Option<RentmanAmount>,
    /// Payment description, e.g. to recognize automatically booked payments.
    pub description: Option<String>,
}

/// Payload for creating or updating a Rentman payment.
#[derive(Debug, Clone, Serialize)]
pub struct RentmanPaymentPayload {
    /// Payment timestamp serialized in Rentman's RFC3339 format with milliseconds.
    #[serde(serialize_with = "serialize_moment")]
    pub moment: DateTime<FixedOffset>,
    /// Payment import source.
    pub payment_import_source: PaymentImportSource,
    /// Payment amount. `None` omits the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<RentmanAmount>,
    /// Optional payment description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

fn serialize_moment<S: serde::Serializer>(
    moment: &DateTime<FixedOffset>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&moment.to_rfc3339_opts(SecondsFormat::Millis, false))
}

#[derive(Debug, Deserialize)]
struct PaymentListItem {
    id: PaymentId,
    moment: DateTime<FixedOffset>,
    invoice: String,
    #[serde(default)]
    amount: Option<RentmanAmount>,
    #[serde(default)]
    description: Option<String>,
}

fn parse_invoice_reference(reference: &str) -> Result<InvoiceId> {
    reference
        .strip_prefix("/invoices/")
        .and_then(|id| id.parse::<u64>().ok())
        .map(InvoiceId::new)
        .ok_or_else(|| {
            Error::new(ErrorKind::UnexpectedResponse(format!(
                "invoice reference {reference:?} is not an /invoices/{{id}} link"
            )))
        })
}

impl RentmanClient {
    /// Loads all Rentman invoices through the documented collection endpoint.
    pub async fn all_invoices(&self) -> Result<Vec<RentmanInvoice>> {
        let mut invoices = Vec::new();
        let mut next_url = None;
        let mut offset = 0;

        loop {
            let response: model::CollectionResponse<RentmanInvoice> = self
                .paginated_request(next_url.take(), "invoices", offset)
                .query(&[
                    ("fields", "id,number,is_paid,price_invat".to_string()),
                    ("limit", DEFAULT_PAGE_LIMIT.to_string()),
                ])
                .send_json()
                .await
                .map_err(|error| error.in_operation("getFactuurCollection"))?;

            let should_fallback_to_offset = response.next_page_url.is_none()
                && response.item_count >= response.limit
                && response.limit > 0;
            offset += response.limit;
            next_url = response.next_page_url;
            invoices.extend(response.data);

            if next_url.is_none() && !should_fallback_to_offset {
                break;
            }
        }

        Ok(invoices)
    }

    /// Loads all Rentman payments and groups them by invoice.
    ///
    /// The payments for each invoice are sorted by ID to keep creation order
    /// stable.
    pub async fn all_payments_by_invoice(&self) -> Result<HashMap<InvoiceId, Vec<RentmanPayment>>> {
        let mut result: HashMap<InvoiceId, Vec<RentmanPayment>> = HashMap::new();
        let mut next_url = None;
        let mut offset = 0;

        loop {
            let response: model::CollectionResponse<PaymentListItem> = self
                .paginated_request(next_url.take(), "payments", offset)
                .query(&[
                    ("fields", "id,moment,invoice,amount,description".to_string()),
                    ("limit", DEFAULT_PAGE_LIMIT.to_string()),
                ])
                .send_json()
                .await
                .map_err(|error| error.in_operation("getPaymentCollection"))?;

            let should_fallback_to_offset = response.next_page_url.is_none()
                && response.item_count >= response.limit
                && response.limit > 0;
            offset += response.limit;
            next_url = response.next_page_url;

            for item in response.data {
                let invoice_id = parse_invoice_reference(&item.invoice)
                    .map_err(|error| error.in_operation("getPaymentCollection"))?;
                result.entry(invoice_id).or_default().push(RentmanPayment {
                    id: item.id,
                    moment: item.moment,
                    amount: item.amount,
                    description: item.description,
                });
            }

            if next_url.is_none() && !should_fallback_to_offset {
                break;
            }
        }

        for payments in result.values_mut() {
            payments.sort_by_key(|payment| payment.id);
        }

        Ok(result)
    }

    /// Updates an existing Rentman payment.
    ///
    /// `PUT` is idempotent, so transient failures are retried.
    pub async fn update_payment(
        &self,
        payment_id: PaymentId,
        payload: &RentmanPaymentPayload,
    ) -> Result<()> {
        self.request(Method::PUT, &format!("payments/{payment_id}"))
            .json(payload)
            .send_unit()
            .await
            .map_err(|error| {
                error
                    .in_operation("updatePayment")
                    .about(Resource::Payment(payment_id))
            })
    }

    /// Creates a payment on a Rentman invoice.
    ///
    /// The request is sent exactly once, without retrying: creating a payment
    /// is not idempotent, and when only the response gets lost a retry would
    /// book the payment twice. When [`Error::outcome`] is not
    /// [`RequestOutcome::NotApplied`](crate::RequestOutcome::NotApplied),
    /// reconcile by reading the invoice's payments with
    /// [`Self::all_payments_by_invoice`] before creating it again.
    pub async fn create_payment(
        &self,
        invoice_id: InvoiceId,
        payload: &RentmanPaymentPayload,
    ) -> Result<()> {
        self.request(Method::POST, &format!("invoices/{invoice_id}/payments"))
            .json(payload)
            .send_unit_once()
            .await
            .map_err(|error| {
                error
                    .in_operation("createPayment")
                    .about(Resource::Invoice(invoice_id))
            })
    }
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {

    use crate::test_support::*;
    use crate::{ErrorKind, RequestOutcome, Resource};
    use crate::{
        InvoiceId, InvoiceNumber, InvoicePaymentStatus, PaymentId, PaymentImportSource,
        RentmanAmount, RentmanApiToken, RentmanClient, RentmanPaymentPayload, Url,
    };
    use chrono::DateTime;
    use reqwest::StatusCode;

    use rust_decimal::Decimal;

    #[tokio::test]
    async fn reads_all_invoices_from_next_page_url_pagination() {
        let server = spawn_server(vec![
            json_response(
                r#"{"itemCount":1,"limit":1500,"next_page_url":"{base}/page-2","data":[{"id":1,"number":"RE-1","is_paid":false}]}"#,
            ),
            json_response(
                r#"{"itemCount":1,"limit":1500,"next_page_url":null,"data":[{"id":2,"number":"RE-2","is_paid":true,"price_invat":119.5}]}"#,
            ),
        ]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let invoices = client.all_invoices().await.unwrap();

        assert_eq!(invoices.len(), 2);
        assert_eq!(invoices[0].id, InvoiceId::new(1));
        assert_eq!(invoices[0].number, InvoiceNumber::new("RE-1"));
        assert_eq!(invoices[0].payment_status, InvoicePaymentStatus::Open);
        assert_eq!(invoices[0].price_invat, None);
        assert_eq!(invoices[1].id, InvoiceId::new(2));
        assert_eq!(invoices[1].number, InvoiceNumber::new("RE-2"));
        assert_eq!(invoices[1].payment_status, InvoicePaymentStatus::Paid);
        assert_eq!(
            invoices[1].price_invat,
            Some(RentmanAmount::new(Decimal::new(1195, 1)))
        );

        let first_request = server.next_request();
        assert_request(&first_request, "GET", "/invoices");
        assert_bearer_token(&first_request);
        assert_eq!(first_request.query_value("offset").as_deref(), Some("0"));
        assert_eq!(
            first_request.query_value("fields").as_deref(),
            Some("id,number,is_paid,price_invat")
        );
        assert_eq!(first_request.query_value("limit").as_deref(), Some("1500"));

        let second_request = server.next_request();
        assert_request(&second_request, "GET", "/page-2");
        assert_bearer_token(&second_request);
    }

    #[tokio::test]
    async fn reads_all_invoices_from_offset_fallback_pagination() {
        let server = spawn_server(vec![
            json_response(
                r#"{"itemCount":1500,"limit":1500,"data":[{"id":1,"number":"RE-1","is_paid":false}]}"#,
            ),
            json_response(
                r#"{"itemCount":1,"limit":1500,"data":[{"id":2,"number":"RE-2","is_paid":true}]}"#,
            ),
        ]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let invoices = client.all_invoices().await.unwrap();

        assert_eq!(invoices.len(), 2);
        assert_eq!(invoices[0].id, InvoiceId::new(1));
        assert_eq!(invoices[1].id, InvoiceId::new(2));

        let first_request = server.next_request();
        assert_request(&first_request, "GET", "/invoices");
        assert_eq!(first_request.query_value("offset").as_deref(), Some("0"));

        let second_request = server.next_request();
        assert_request(&second_request, "GET", "/invoices");
        assert_eq!(
            second_request.query_value("offset").as_deref(),
            Some("1500")
        );
    }

    #[tokio::test]
    async fn adds_context_to_invoice_list_errors() {
        let server = spawn_server(vec![text_response(
            "HTTP/1.1 500 Internal Server Error",
            "rentman broken",
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let error = client.all_invoices().await.unwrap_err();

        assert_eq!(error.operation(), Some("getFactuurCollection"));
        assert_eq!(error.resource(), None);
        assert_request(&server.next_request(), "GET", "/invoices");
    }

    #[tokio::test]
    async fn rejects_invalid_invoice_next_page_urls() {
        let server = spawn_server(vec![json_response(
            r#"{"itemCount":1,"limit":1500,"next_page_url":"not-a-url","data":[{"id":1,"number":"RE-1","is_paid":false}]}"#,
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let error = client.all_invoices().await.unwrap_err();

        assert!(matches!(error.kind(), ErrorKind::Decode(_)), "{error:?}");
        assert_eq!(error.operation(), Some("getFactuurCollection"));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_request(&server.next_request(), "GET", "/invoices");
    }

    #[tokio::test]
    async fn groups_all_payments_by_invoice_sorted_by_id() {
        let server = spawn_server(vec![json_response(
            r#"{"itemCount":3,"limit":1500,"next_page_url":null,"data":[{"id":11,"moment":"2026-07-05T00:00:00.000+02:00","invoice":"/invoices/1","amount":0.01,"description":"Rounding"},{"id":10,"moment":"2026-07-04T00:00:00.000+02:00","invoice":"/invoices/1","amount":null,"description":null},{"id":12,"moment":"2026-07-06T00:00:00.000+02:00","invoice":"/invoices/2"}]}"#,
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let payments = client.all_payments_by_invoice().await.unwrap();

        assert_eq!(payments.len(), 2);
        let first_invoice = &payments[&InvoiceId::new(1)];
        assert_eq!(first_invoice.len(), 2);
        assert_eq!(first_invoice[0].id, PaymentId::new(10));
        assert_eq!(first_invoice[1].id, PaymentId::new(11));
        assert_eq!(first_invoice[0].amount, None);
        assert_eq!(first_invoice[0].description, None);
        assert_eq!(
            first_invoice[1].amount,
            Some(RentmanAmount::new(Decimal::new(1, 2)))
        );
        assert_eq!(first_invoice[1].description.as_deref(), Some("Rounding"));
        assert_eq!(
            first_invoice[0].moment,
            DateTime::parse_from_rfc3339("2026-07-04T00:00:00.000+02:00").unwrap()
        );
        assert_eq!(payments[&InvoiceId::new(2)][0].id, PaymentId::new(12));

        let request = server.next_request();
        assert_request(&request, "GET", "/payments");
        assert_bearer_token(&request);
        assert_eq!(request.query_value("offset").as_deref(), Some("0"));
        assert_eq!(
            request.query_value("fields").as_deref(),
            Some("id,moment,invoice,amount,description")
        );
        assert_eq!(request.query_value("limit").as_deref(), Some("1500"));
    }

    #[tokio::test]
    async fn reads_all_payments_from_offset_fallback_pagination() {
        let server = spawn_server(vec![
            json_response(
                r#"{"itemCount":1500,"limit":1500,"data":[{"id":11,"moment":"2026-07-05T00:00:00.000+02:00","invoice":"/invoices/1"}]}"#,
            ),
            json_response(
                r#"{"itemCount":1,"limit":1500,"data":[{"id":12,"moment":"2026-07-06T00:00:00.000+02:00","invoice":"/invoices/2"}]}"#,
            ),
        ]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let payments = client.all_payments_by_invoice().await.unwrap();

        assert_eq!(payments.len(), 2);
        assert_eq!(payments[&InvoiceId::new(1)][0].id, PaymentId::new(11));
        assert_eq!(payments[&InvoiceId::new(2)][0].id, PaymentId::new(12));

        let first_request = server.next_request();
        assert_request(&first_request, "GET", "/payments");
        assert_eq!(first_request.query_value("offset").as_deref(), Some("0"));

        let second_request = server.next_request();
        assert_request(&second_request, "GET", "/payments");
        assert_eq!(
            second_request.query_value("offset").as_deref(),
            Some("1500")
        );
    }

    #[tokio::test]
    async fn adds_context_to_payment_list_errors() {
        let server = spawn_server(vec![text_response(
            "HTTP/1.1 500 Internal Server Error",
            "rentman broken",
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let error = client.all_payments_by_invoice().await.unwrap_err();

        assert_eq!(error.operation(), Some("getPaymentCollection"));
        assert_request(&server.next_request(), "GET", "/payments");
    }

    #[tokio::test]
    async fn rejects_invalid_payment_next_page_urls() {
        let server = spawn_server(vec![json_response(
            r#"{"itemCount":1,"limit":1500,"next_page_url":"not-a-url","data":[{"id":11,"moment":"2026-07-05T00:00:00.000+02:00","invoice":"/invoices/1"}]}"#,
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let error = client.all_payments_by_invoice().await.unwrap_err();

        assert!(matches!(error.kind(), ErrorKind::Decode(_)), "{error:?}");
        assert_eq!(error.operation(), Some("getPaymentCollection"));
        assert_request(&server.next_request(), "GET", "/payments");
    }

    #[tokio::test]
    async fn rejects_unexpected_invoice_references() {
        let server = spawn_server(vec![json_response(
            r#"{"itemCount":1,"limit":1500,"next_page_url":null,"data":[{"id":10,"moment":"2026-07-05T00:00:00.000+02:00","invoice":"/broken/1"}]}"#,
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let error = client.all_payments_by_invoice().await.unwrap_err();

        let ErrorKind::UnexpectedResponse(message) = error.kind() else {
            panic!("expected an unexpected response, got {error:?}");
        };
        assert_eq!(
            message,
            r#"invoice reference "/broken/1" is not an /invoices/{id} link"#
        );
        assert_eq!(error.operation(), Some("getPaymentCollection"));
        assert_request(&server.next_request(), "GET", "/payments");
    }

    #[tokio::test]
    async fn updates_existing_payment() {
        let server = spawn_server(vec![empty_response("HTTP/1.1 200 OK")]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        client
            .update_payment(PaymentId::new(10), &payment_payload())
            .await
            .unwrap();

        let request = server.next_request();
        assert_request(&request, "PUT", "/payments/10");
        assert_bearer_token(&request);
        let body = request.json_body();
        assert_eq!(body["moment"], "2026-07-05T00:00:00.000+02:00");
        assert_eq!(body["payment_import_source"], "publicapi");
        assert_eq!(body["amount"], serde_json::json!(42.5));
        assert_eq!(body["description"], "Manual booking");
    }

    #[tokio::test]
    async fn creates_payment_exactly_once_on_transient_errors() {
        let server = spawn_server(vec![
            text_response("HTTP/1.1 503 Service Unavailable", "later"),
            empty_response("HTTP/1.1 200 OK"),
        ]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let error = client
            .create_payment(InvoiceId::new(1), &payment_payload())
            .await
            .unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::SERVICE_UNAVAILABLE));
        assert_eq!(error.outcome(), RequestOutcome::Unknown);
        assert_request(&server.next_request(), "POST", "/invoices/1/payments");

        // Without a retry the second prepared response is still unused, so the
        // next call receives it: the failed create reached the server once.
        client
            .create_payment(InvoiceId::new(1), &payment_payload())
            .await
            .unwrap();
        assert_request(&server.next_request(), "POST", "/invoices/1/payments");
    }

    #[tokio::test]
    async fn creates_missing_payment() {
        let server = spawn_server(vec![empty_response("HTTP/1.1 200 OK")]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        client
            .create_payment(InvoiceId::new(1), &payment_payload())
            .await
            .unwrap();

        let request = server.next_request();
        assert_request(&request, "POST", "/invoices/1/payments");
        assert_bearer_token(&request);
        let body = request.json_body();
        assert_eq!(body["moment"], "2026-07-05T00:00:00.000+02:00");
        assert_eq!(body["payment_import_source"], "publicapi");
        assert_eq!(body["amount"], serde_json::json!(42.5));
        assert_eq!(body["description"], "Manual booking");
    }

    #[tokio::test]
    async fn adds_context_to_payment_update_errors() {
        let server = spawn_server(vec![text_response(
            "HTTP/1.1 500 Internal Server Error",
            "rentman broken",
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let error = client
            .update_payment(PaymentId::new(10), &payment_payload())
            .await
            .unwrap_err();

        assert_eq!(error.operation(), Some("updatePayment"));
        assert_eq!(
            error.resource(),
            Some(Resource::Payment(PaymentId::new(10)))
        );
        // The first attempt got a 500 and may have been applied; the retries
        // then found the server gone. The error must not claim "not applied".
        assert!(matches!(error.kind(), ErrorKind::Transport(_)), "{error:?}");
        assert_eq!(error.outcome(), RequestOutcome::Unknown);
        assert_request(&server.next_request(), "PUT", "/payments/10");
    }

    #[tokio::test]
    async fn adds_context_to_payment_create_errors() {
        let server = spawn_server(vec![text_response(
            "HTTP/1.1 500 Internal Server Error",
            "rentman broken",
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let error = client
            .create_payment(InvoiceId::new(1), &payment_payload())
            .await
            .unwrap_err();

        assert_eq!(error.operation(), Some("createPayment"));
        assert_eq!(error.resource(), Some(Resource::Invoice(InvoiceId::new(1))));
        assert_eq!(
            error.to_string(),
            "Rentman createPayment failed for invoice 1: \
             Rentman API returned 500 Internal Server Error: rentman broken"
        );
        assert_request(&server.next_request(), "POST", "/invoices/1/payments");
    }

    #[test]
    fn serializes_payload_in_rentman_wire_format() {
        let json = serde_json::to_value(payment_payload()).unwrap();

        assert_eq!(json["moment"], "2026-07-05T00:00:00.000+02:00");
        assert_eq!(json["payment_import_source"], "publicapi");
        assert_eq!(json["amount"], serde_json::json!(42.5));
        assert_eq!(json["description"], "Manual booking");

        let disabled = RentmanPaymentPayload {
            moment: DateTime::parse_from_rfc3339("2026-07-05T00:00:00+02:00").unwrap(),
            payment_import_source: PaymentImportSource::None,
            amount: None,
            description: None,
        };
        let json = serde_json::to_value(&disabled).unwrap();

        assert_eq!(json["payment_import_source"], "none");
        assert!(json.get("amount").is_none());
        assert!(json.get("description").is_none());
    }

    fn payment_payload() -> RentmanPaymentPayload {
        RentmanPaymentPayload {
            moment: DateTime::parse_from_rfc3339("2026-07-05T00:00:00+02:00").unwrap(),
            payment_import_source: PaymentImportSource::PublicApi,
            amount: Some(RentmanAmount::new(Decimal::new(425, 1))),
            description: Some("Manual booking".to_string()),
        }
    }
}
