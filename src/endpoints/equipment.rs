//! Equipment endpoint implementation.

use crate::client::RentmanClient;
use crate::error::Result;
use crate::models::{
    common::{ItemResponse, ListResponse, QueryParams},
    equipment::{Equipment, SerialNumber},
};

/// Equipment endpoint.
#[derive(Clone)]
pub struct EquipmentEndpoint {
    client: RentmanClient,
}

impl EquipmentEndpoint {
    /// Create a new equipment endpoint.
    pub fn new(client: RentmanClient) -> Self {
        Self { client }
    }

    /// List all equipment.
    pub async fn list(&self) -> Result<Vec<Equipment>> {
        self.list_with_params(QueryParams::default()).await
    }

    /// List equipment with query parameters.
    pub async fn list_with_params(&self, params: QueryParams) -> Result<Vec<Equipment>> {
        let path = format!("/equipment{}", params.build());
        let response: ListResponse<Equipment> = self.client.get(&path).await?;
        Ok(response.data)
    }

    /// Get equipment by ID.
    pub async fn get(&self, id: i32) -> Result<Equipment> {
        let path = format!("/equipment/{}", id);
        let response: ItemResponse<Equipment> = self.client.get(&path).await?;
        Ok(response.data)
    }

    /// Get serial numbers for equipment.
    pub async fn serial_numbers(&self, equipment_id: i32) -> Result<Vec<SerialNumber>> {
        let path = format!("/equipment/{}/serialnumbers", equipment_id);
        let response: ListResponse<SerialNumber> = self.client.get(&path).await?;
        Ok(response.data)
    }
}

/// Serial numbers endpoint.
#[derive(Clone)]
pub struct SerialNumbersEndpoint {
    client: RentmanClient,
}

impl SerialNumbersEndpoint {
    /// Create a new serial numbers endpoint.
    pub fn new(client: RentmanClient) -> Self {
        Self { client }
    }

    /// List all serial numbers.
    pub async fn list(&self) -> Result<Vec<SerialNumber>> {
        self.list_with_params(QueryParams::default()).await
    }

    /// List serial numbers with query parameters.
    pub async fn list_with_params(&self, params: QueryParams) -> Result<Vec<SerialNumber>> {
        let path = format!("/serialnumbers{}", params.build());
        let response: ListResponse<SerialNumber> = self.client.get(&path).await?;
        Ok(response.data)
    }

    /// Get serial number by ID.
    pub async fn get(&self, id: i32) -> Result<SerialNumber> {
        let path = format!("/serialnumbers/{}", id);
        let response: ItemResponse<SerialNumber> = self.client.get(&path).await?;
        Ok(response.data)
    }
}
