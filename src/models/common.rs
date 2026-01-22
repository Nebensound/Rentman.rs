//! Common types used across multiple resources.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standard API response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// The response data
    pub data: T,
    /// Number of items in the response
    #[serde(rename = "itemCount")]
    pub item_count: Option<i32>,
    /// Maximum number of items per page
    pub limit: Option<i32>,
    /// Number of items skipped
    pub offset: Option<i32>,
}

/// Paginated list response.
pub type ListResponse<T> = ApiResponse<Vec<T>>;

/// Single item response.
pub type ItemResponse<T> = ApiResponse<T>;

/// Resource reference (e.g., "/projects/123").
pub type ResourceRef = String;

/// Custom fields as a flexible map.
pub type CustomFields = HashMap<String, serde_json::Value>;

/// Common fields present in most resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonFields {
    /// Unique identifier
    pub id: i32,
    /// Creation timestamp
    pub created: DateTime<Utc>,
    /// Last modification timestamp
    pub modified: DateTime<Utc>,
    /// Creator reference
    pub creator: Option<ResourceRef>,
    /// Display name
    pub displayname: Option<String>,
}

/// Query parameters for list requests.
#[derive(Debug, Clone, Default)]
pub struct QueryParams {
    /// Maximum number of items to return
    pub limit: Option<i32>,
    /// Number of items to skip
    pub offset: Option<i32>,
    /// Filter query
    pub filter: Option<String>,
    /// Sort field and direction
    pub sort: Option<String>,
}

impl QueryParams {
    /// Create a new query params builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit.
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset.
    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set the filter.
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// Set the sort.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Build the query string.
    pub fn build(&self) -> String {
        let mut params = Vec::new();

        if let Some(limit) = self.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = self.offset {
            params.push(format!("offset={}", offset));
        }
        if let Some(ref filter) = self.filter {
            params.push(format!("filter={}", urlencoding::encode(filter)));
        }
        if let Some(ref sort) = self.sort {
            params.push(format!("sort={}", urlencoding::encode(sort)));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}
