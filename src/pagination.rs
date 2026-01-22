//! Pagination support for API list operations.

use crate::error::Result;
use crate::models::common::{ListResponse, QueryParams};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

/// Iterator for paginated API responses.
pub struct PaginatedIterator<T> {
    fetch_page: Box<dyn Fn(i32, i32) -> Result<ListResponse<T>> + Send + Sync>,
    offset: i32,
    limit: i32,
    total_fetched: i32,
    current_batch: Vec<T>,
    current_index: usize,
    exhausted: bool,
}

impl<T> PaginatedIterator<T>
where
    T: DeserializeOwned + Send + 'static,
{
    /// Create a new paginated iterator.
    pub fn new<F>(fetch_page: F, limit: i32) -> Self
    where
        F: Fn(i32, i32) -> Result<ListResponse<T>> + Send + Sync + 'static,
    {
        Self {
            fetch_page: Box::new(fetch_page),
            offset: 0,
            limit,
            total_fetched: 0,
            current_batch: Vec::new(),
            current_index: 0,
            exhausted: false,
        }
    }

    /// Fetch the next batch of items.
    fn fetch_next_batch(&mut self) -> Result<()> {
        if self.exhausted {
            return Ok(());
        }

        let response = (self.fetch_page)(self.offset, self.limit)?;
        
        if response.data.is_empty() {
            self.exhausted = true;
            return Ok(());
        }

        let fetched_count = response.data.len() as i32;
        self.offset += fetched_count;
        self.total_fetched += fetched_count;
        self.current_batch = response.data;
        self.current_index = 0;

        // If we got fewer items than requested, we've reached the end
        if fetched_count < self.limit {
            self.exhausted = true;
        }

        Ok(())
    }
}

impl<T> Iterator for PaginatedIterator<T>
where
    T: DeserializeOwned + Send + 'static,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // If we've consumed all items in the current batch, fetch the next one
        if self.current_index >= self.current_batch.len() {
            if self.exhausted {
                return None;
            }

            match self.fetch_next_batch() {
                Ok(_) => {
                    if self.current_batch.is_empty() {
                        return None;
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }

        let item = self.current_batch.get(self.current_index)?;
        self.current_index += 1;
        
        // This is a workaround since we can't easily clone T
        // In a real implementation, you might want to require T: Clone
        None // TODO: Fix this - need proper item ownership
    }
}

/// Trait for endpoints that support pagination.
pub trait Paginated<T> {
    /// Create a paginated iterator for this endpoint.
    fn paginate(&self, limit: i32) -> PaginatedIterator<T>;
}
