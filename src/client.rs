//! HTTP client for the Rentman API.

use crate::error::{Error, Result};
use crate::endpoints::{
    ProjectsEndpoint,
    ContactsEndpoint,
    EquipmentEndpoint,
};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

const DEFAULT_BASE_URL: &str = "https://api.rentman.net";

/// Rentman API client.
#[derive(Clone)]
pub struct RentmanClient {
    http_client: Client,
    base_url: String,
    token: String,
}

impl RentmanClient {
    /// Create a new client builder.
    pub fn builder() -> RentmanClientBuilder {
        RentmanClientBuilder::default()
    }

    /// Get the projects endpoint.
    pub fn projects(&self) -> ProjectsEndpoint {
        ProjectsEndpoint::new(self.clone())
    }

    /// Get the contacts endpoint.
    pub fn contacts(&self) -> ContactsEndpoint {
        ContactsEndpoint::new(self.clone())
    }

    /// Get the equipment endpoint.
    pub fn equipment(&self) -> EquipmentEndpoint {
        EquipmentEndpoint::new(self.clone())
    }

    /// Make a GET request to the API.
    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .prepare_request(self.http_client.get(&url))
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a POST request to the API.
    pub(crate) async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .prepare_request(self.http_client.post(&url))
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a PUT request to the API.
    pub(crate) async fn put<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .prepare_request(self.http_client.put(&url))
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a DELETE request to the API.
    pub(crate) async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .prepare_request(self.http_client.delete(&url))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            Err(Error::api_error(status, message))
        }
    }

    fn prepare_request(&self, request: RequestBuilder) -> RequestBuilder {
        request
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            Ok(response.json::<T>().await?)
        } else {
            let status_code = status.as_u16();
            let error_body = response.text().await.unwrap_or_default();

            match status_code {
                401 => Err(Error::auth("Invalid or expired token")),
                404 => Err(Error::NotFound(error_body)),
                429 => Err(Error::RateLimitExceeded(60)), // Default retry after 60 seconds
                _ => Err(Error::api_error(status_code, error_body)),
            }
        }
    }
}

/// Builder for configuring a Rentman API client.
#[derive(Default)]
pub struct RentmanClientBuilder {
    base_url: Option<String>,
    token: Option<String>,
}

impl RentmanClientBuilder {
    /// Set the API token.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Set the base URL (optional, defaults to https://api.rentman.net).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<RentmanClient> {
        let token = self.token.ok_or_else(|| Error::config("API token is required"))?;

        let http_client = Client::builder()
            .build()
            .map_err(|e| Error::config(format!("Failed to create HTTP client: {}", e)))?;

        Ok(RentmanClient {
            http_client,
            base_url: self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            token,
        })
    }
}
