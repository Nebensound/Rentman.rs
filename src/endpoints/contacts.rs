//! Contacts endpoint implementation.

use crate::client::RentmanClient;
use crate::error::Result;
use crate::models::{
    common::{ItemResponse, ListResponse, QueryParams},
    contact::{Contact, ContactPerson},
};

/// Contacts endpoint.
#[derive(Clone)]
pub struct ContactsEndpoint {
    client: RentmanClient,
}

impl ContactsEndpoint {
    /// Create a new contacts endpoint.
    pub fn new(client: RentmanClient) -> Self {
        Self { client }
    }

    /// List all contacts.
    pub async fn list(&self) -> Result<Vec<Contact>> {
        self.list_with_params(QueryParams::default()).await
    }

    /// List contacts with query parameters.
    pub async fn list_with_params(&self, params: QueryParams) -> Result<Vec<Contact>> {
        let path = format!("/contacts{}", params.build());
        let response: ListResponse<Contact> = self.client.get(&path).await?;
        Ok(response.data)
    }

    /// Get a contact by ID.
    pub async fn get(&self, id: i32) -> Result<Contact> {
        let path = format!("/contacts/{}", id);
        let response: ItemResponse<Contact> = self.client.get(&path).await?;
        Ok(response.data)
    }

    /// Get contact persons for a contact.
    pub async fn contact_persons(&self, contact_id: i32) -> Result<Vec<ContactPerson>> {
        let path = format!("/contacts/{}/contactpersons", contact_id);
        let response: ListResponse<ContactPerson> = self.client.get(&path).await?;
        Ok(response.data)
    }
}
