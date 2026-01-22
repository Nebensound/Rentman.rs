//! Projects endpoint implementation.

use crate::client::RentmanClient;
use crate::error::Result;
use crate::models::{
    common::{ItemResponse, ListResponse, QueryParams},
    project::{CreateProject, Project, Subproject, UpdateProject},
};

/// Projects endpoint.
#[derive(Clone)]
pub struct ProjectsEndpoint {
    client: RentmanClient,
}

impl ProjectsEndpoint {
    /// Create a new projects endpoint.
    pub fn new(client: RentmanClient) -> Self {
        Self { client }
    }

    /// List all projects.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rentman::RentmanClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RentmanClient::builder().token("token").build()?;
    /// let projects = client.projects().list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<Vec<Project>> {
        self.list_with_params(QueryParams::default()).await
    }

    /// List projects with query parameters.
    pub async fn list_with_params(&self, params: QueryParams) -> Result<Vec<Project>> {
        let path = format!("/projects{}", params.build());
        let response: ListResponse<Project> = self.client.get(&path).await?;
        Ok(response.data)
    }

    /// Get a project by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rentman::RentmanClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RentmanClient::builder().token("token").build()?;
    /// let project = client.projects().get(123).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, id: i32) -> Result<Project> {
        let path = format!("/projects/{}", id);
        let response: ItemResponse<Project> = self.client.get(&path).await?;
        Ok(response.data)
    }

    /// Create a new project.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rentman::{RentmanClient, prelude::CreateProject};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RentmanClient::builder().token("token").build()?;
    /// let new_project = CreateProject {
    ///     name: "New Event".to_string(),
    ///     reference: Some("REF-001".to_string()),
    ///     number: None,
    ///     custom: None,
    /// };
    /// let project = client.projects().create(&new_project).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, project: &CreateProject) -> Result<Project> {
        let response: ItemResponse<Project> = self.client.post("/projects", project).await?;
        Ok(response.data)
    }

    /// Update a project.
    pub async fn update(&self, id: i32, project: &UpdateProject) -> Result<Project> {
        let path = format!("/projects/{}", id);
        let response: ItemResponse<Project> = self.client.put(&path, project).await?;
        Ok(response.data)
    }

    /// Delete a project.
    pub async fn delete(&self, id: i32) -> Result<()> {
        let path = format!("/projects/{}", id);
        self.client.delete(&path).await
    }

    /// Get subprojects of a project.
    pub async fn subprojects(&self, project_id: i32) -> Result<Vec<Subproject>> {
        let path = format!("/projects/{}/subprojects", project_id);
        let response: ListResponse<Subproject> = self.client.get(&path).await?;
        Ok(response.data)
    }
}
