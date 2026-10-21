//! Equipment and its prices.

use crate::{
    RentmanAmount,
    client::{DEFAULT_PAGE_LIMIT, RentmanClient},
    error::{Resource, Result},
    ids::EquipmentId,
    model,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Rentman equipment with the fields needed for price maintenance.
#[derive(Debug, Clone, Deserialize)]
pub struct RentmanEquipment {
    /// Internal Rentman equipment ID.
    pub id: EquipmentId,
    /// Rental price per unit. `None` when Rentman sends `null` or omits the
    /// field.
    #[serde(default)]
    pub price: Option<RentmanAmount>,
    /// List price (new price). `None` when Rentman sends `null` or omits the
    /// field.
    #[serde(default)]
    pub list_price: Option<RentmanAmount>,
    /// Subrental costs. `None` when Rentman sends `null` or omits the field.
    #[serde(default)]
    pub subrental_costs: Option<RentmanAmount>,
}

/// Payload for updating the price fields of an equipment item.
///
/// All fields are optional: Rentman leaves fields that are not sent untouched
/// on `PUT`, so callers only send the fields that actually change.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct EquipmentPricePayload {
    /// New rental price. `None` omits the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<RentmanAmount>,
    /// New list price. `None` omits the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_price: Option<RentmanAmount>,
    /// New subrental costs. `None` omits the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subrental_costs: Option<RentmanAmount>,
}

impl EquipmentPricePayload {
    /// Returns whether the payload would not change any field.
    pub fn is_empty(&self) -> bool {
        self.price.is_none() && self.list_price.is_none() && self.subrental_costs.is_none()
    }
}

impl RentmanClient {
    /// Loads all equipment that is not archived, with its price fields.
    pub async fn all_active_equipment(&self) -> Result<Vec<RentmanEquipment>> {
        let mut equipment = Vec::new();
        let mut next_url = None;
        let mut offset = 0;

        loop {
            let response: model::CollectionResponse<RentmanEquipment> = self
                .paginated_request(next_url.take(), "equipment", offset)
                .query(&[
                    ("fields", "id,price,list_price,subrental_costs".to_string()),
                    ("in_archive", "0".to_string()),
                    ("limit", DEFAULT_PAGE_LIMIT.to_string()),
                ])
                .send_json()
                .await
                .map_err(|error| error.in_operation("getEquipmentCollection"))?;

            let should_fallback_to_offset = response.next_page_url.is_none()
                && response.item_count >= response.limit
                && response.limit > 0;
            offset += response.limit;
            next_url = response.next_page_url;
            equipment.extend(response.data);

            if next_url.is_none() && !should_fallback_to_offset {
                break;
            }
        }

        Ok(equipment)
    }

    /// Updates the price fields of an equipment item.
    ///
    /// `PUT` is idempotent, so transient failures are retried.
    pub async fn update_equipment_prices(
        &self,
        equipment_id: EquipmentId,
        payload: &EquipmentPricePayload,
    ) -> Result<()> {
        self.request(Method::PUT, &format!("equipment/{equipment_id}"))
            .json(payload)
            .send_unit()
            .await
            .map_err(|error| {
                error
                    .in_operation("updateEquipment")
                    .about(Resource::Equipment(equipment_id))
            })
    }
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {

    use crate::test_support::*;
    use crate::{
        EquipmentId, EquipmentPricePayload, RentmanAmount, RentmanApiToken, RentmanClient, Url,
    };
    use crate::{RentmanClientConfig, RequestOutcome, Resource};
    use reqwest::StatusCode;
    use std::{num::NonZeroUsize, time::Duration};

    use rust_decimal::Decimal;

    #[tokio::test]
    async fn reads_all_active_equipment_from_both_pagination_styles() {
        let server = spawn_server(vec![
            json_response(
                r#"{"itemCount":1,"limit":1500,"next_page_url":"{base}/page-2","data":[{"id":1,"price":12.5,"list_price":null,"subrental_costs":3}]}"#,
            ),
            json_response(
                r#"{"itemCount":1500,"limit":1500,"next_page_url":null,"data":[{"id":2,"price":null,"list_price":499,"subrental_costs":null}]}"#,
            ),
            json_response(r#"{"itemCount":0,"limit":1500,"next_page_url":null,"data":[]}"#),
        ]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let equipment = client.all_active_equipment().await.unwrap();

        assert_eq!(equipment.len(), 2);
        assert_eq!(equipment[0].id, EquipmentId::new(1));
        assert_eq!(
            equipment[0].price,
            Some(RentmanAmount::new(Decimal::new(125, 1)))
        );
        assert_eq!(equipment[0].list_price, None);
        assert_eq!(
            equipment[0].subrental_costs,
            Some(RentmanAmount::new(Decimal::new(3, 0)))
        );
        assert_eq!(
            equipment[1].list_price,
            Some(RentmanAmount::new(Decimal::new(499, 0)))
        );

        let first_request = server.next_request();
        assert_request(&first_request, "GET", "/equipment");
        assert_bearer_token(&first_request);
        assert_eq!(
            first_request.query_value("fields").as_deref(),
            Some("id,price,list_price,subrental_costs")
        );
        assert_eq!(
            first_request.query_value("in_archive").as_deref(),
            Some("0")
        );
        assert_request(&server.next_request(), "GET", "/page-2");
        let third_request = server.next_request();
        assert_request(&third_request, "GET", "/equipment");
        assert_eq!(third_request.query_value("offset").as_deref(), Some("3000"));
    }

    #[tokio::test]
    async fn reads_equipment_with_omitted_price_fields() {
        let server = spawn_server(vec![json_response(
            r#"{"itemCount":2,"limit":1500,"next_page_url":null,"data":[{"id":1},{"id":2,"list_price":499}]}"#,
        )]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let equipment = client.all_active_equipment().await.unwrap();

        assert_eq!(equipment.len(), 2);
        assert_eq!(equipment[0].price, None);
        assert_eq!(equipment[0].list_price, None);
        assert_eq!(equipment[0].subrental_costs, None);
        assert_eq!(equipment[1].price, None);
        assert_eq!(
            equipment[1].list_price,
            Some(RentmanAmount::new(Decimal::new(499, 0)))
        );
        assert_eq!(equipment[1].subrental_costs, None);
    }

    #[tokio::test]
    async fn adds_context_to_equipment_list_errors() {
        let server = spawn_server(vec![text_response("HTTP/1.1 400 Bad Request", "broken")]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();

        let error = client.all_active_equipment().await.unwrap_err();

        assert_eq!(error.operation(), Some("getEquipmentCollection"));
        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        assert_eq!(error.outcome(), RequestOutcome::NotApplied);
    }

    #[tokio::test]
    async fn retries_equipment_price_update_after_transient_error() {
        let server = spawn_server(vec![
            text_response("HTTP/1.1 503 Service Unavailable", "later"),
            empty_response("HTTP/1.1 200 OK"),
        ]);
        // Fixed retry policy, independent of the environment.
        let config = RentmanClientConfig::default()
            .with_max_attempts(NonZeroUsize::new(2).unwrap())
            .with_retry_base_delay(Duration::from_millis(1));
        let client = RentmanClient::with_config(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
            config,
        )
        .unwrap();
        let payload = EquipmentPricePayload {
            price: Some(RentmanAmount::new(Decimal::new(1250, 2))),
            ..EquipmentPricePayload::default()
        };

        client
            .update_equipment_prices(EquipmentId::new(2163), &payload)
            .await
            .unwrap();

        // The idempotent PUT reached the server twice, both times unchanged.
        for _ in 0..2 {
            let request = server.next_request();
            assert_request(&request, "PUT", "/equipment/2163");
            assert_eq!(request.json_body(), serde_json::json!({"price": 12.5}));
            assert_eq!(String::from_utf8_lossy(&request.body), r#"{"price":12.50}"#);
        }
    }

    #[tokio::test]
    async fn updates_only_changed_equipment_prices() {
        let server = spawn_server(vec![
            empty_response("HTTP/1.1 200 OK"),
            text_response("HTTP/1.1 400 Bad Request", "broken"),
        ]);
        let client = RentmanClient::with_base_url(
            Url::parse(server.base_url()).unwrap(),
            RentmanApiToken::new("token"),
        )
        .unwrap();
        let payload = EquipmentPricePayload {
            price: Some(RentmanAmount::new(Decimal::new(1250, 2))),
            ..EquipmentPricePayload::default()
        };
        assert!(!payload.is_empty());
        assert!(EquipmentPricePayload::default().is_empty());

        client
            .update_equipment_prices(EquipmentId::new(2163), &payload)
            .await
            .unwrap();

        let request = server.next_request();
        assert_request(&request, "PUT", "/equipment/2163");
        assert_bearer_token(&request);
        assert_eq!(request.json_body(), serde_json::json!({"price": 12.5}));

        let error = client
            .update_equipment_prices(EquipmentId::new(2163), &payload)
            .await
            .unwrap_err();
        assert_eq!(error.operation(), Some("updateEquipment"));
        assert_eq!(
            error.resource(),
            Some(Resource::Equipment(EquipmentId::new(2163)))
        );
        assert_eq!(
            error.to_string(),
            "Rentman updateEquipment failed for equipment 2163: \
             Rentman API returned 400 Bad Request: broken"
        );
    }
}
