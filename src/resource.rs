//! The resource layer: list, read and write Rentman data.
//!
//! Every documented collection is reachable from [`RentmanClient`], e.g.
//! `client.crew()` or, nested, `client.crew().appointments(crew_id)`. A
//! [`Collection`] only offers the operations Rentman documents for it: the
//! methods come from the [`List`], [`Get`], [`Create`], [`Update`] and
//! [`Delete`] traits, which the generated code implements per collection.
//!
//! The resources are the only way to Rentman data: the endpoint layer below
//! them and the response envelopes are internal.
//!
//! ```compile_fail
//! use rentman_client::endpoint::GetCrewCollectionEndpoint;
//! ```
//!
//! ```compile_fail
//! use rentman_client::model::CollectionResponse;
//! ```

mod generated;
#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod generated_tests;

pub use generated::*;

use crate::{
    CustomFieldsResource, RentmanClient,
    endpoint::EndpointSpec,
    error::{Error, Result},
    filter::{Filter, Sort},
    id::{Id, Resource},
    model::ItemResponse,
    paging::{CollectionRequest, PageSize},
    projection::{self, ProjectedItem, Projection},
    revision::{ItemRevision, Revisioned, Revisions},
};
use reqwest::Method;
use serde::{Serialize, de::DeserializeOwned};
use std::{fmt, marker::PhantomData};

mod sealed {
    /// Only the generated collections implement the operation traits.
    pub trait Sealed {}
}

/// A documented Rentman API operation, used by the generated collections.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Operation(pub(crate) EndpointSpec);

impl Operation {
    /// OpenAPI operation ID, e.g. `getCrewCollection`.
    pub fn operation_id(&self) -> &'static str {
        self.0.operation_id
    }
}

/// A collection whose items can be listed.
pub trait List: sealed::Sealed {
    /// Item type of the collection.
    type Item: Resource + CustomFieldsResource + DeserializeOwned;
    #[doc(hidden)]
    const LIST: Operation;
}

/// A collection whose items can be read one by one.
pub trait Get: sealed::Sealed {
    /// Item type of the collection.
    type Item: Resource + CustomFieldsResource + DeserializeOwned;
    #[doc(hidden)]
    const GET: Operation;
}

/// Internal GET operation for an item, including items created in nested collections.
#[doc(hidden)]
pub trait GetItem: Resource + CustomFieldsResource + DeserializeOwned + sealed::Sealed {
    #[doc(hidden)]
    const GET_ITEM: Operation;
    #[doc(hidden)]
    const ITEM_FIELDS: &'static [&'static str];
}

/// A collection that items can be created in.
pub trait Create: sealed::Sealed {
    /// Write payload of a new item.
    type Payload: Serialize;
    /// Item type Rentman returns for the created item.
    type Item: Resource + CustomFieldsResource + DeserializeOwned;
    #[doc(hidden)]
    const CREATE: Operation;
}

/// A collection whose items can be updated.
pub trait Update: sealed::Sealed {
    /// Write payload with the fields to change.
    type Payload: Serialize;
    /// Item type of the collection.
    type Item: Resource + CustomFieldsResource + DeserializeOwned;
    #[doc(hidden)]
    const UPDATE: Operation;
}

/// A collection whose items can be deleted.
pub trait Delete: sealed::Sealed {
    /// Item type of the collection.
    type Item: Resource;
    #[doc(hidden)]
    const DELETE: Operation;
}

/// A Rentman collection, e.g. all crew members or the appointments of one
/// crew member.
///
/// `M` names the collection; which methods exist depends on what Rentman
/// documents for it.
pub struct Collection<M> {
    client: RentmanClient,
    /// ID of the parent item of a nested collection, e.g. the crew member in
    /// `/crew/{id}/appointments`.
    parent: Option<u64>,
    collection: PhantomData<fn() -> M>,
}

impl<M> Collection<M> {
    pub(crate) fn new(client: &RentmanClient, parent: Option<u64>) -> Self {
        Self {
            client: client.clone(),
            parent,
            collection: PhantomData,
        }
    }
}

impl<M> fmt::Debug for Collection<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Collection")
            .field("collection", &std::any::type_name::<M>())
            .field("parent", &self.parent)
            .finish()
    }
}

impl<M: List> Collection<M> {
    /// Starts a list request; nothing is sent before [`ListRequest::all`] or
    /// [`ListRequest::first`].
    pub fn list(&self) -> ListRequest<M::Item> {
        ListRequest {
            client: self.client.clone(),
            path: render_path(M::LIST.0, self.parent),
            operation: M::LIST.0.operation_id,
            page_size: PageSize::default(),
            query: Vec::new(),
            sort: Vec::new(),
            item: PhantomData,
        }
    }
}

impl<M: Get> Collection<M> {
    /// Reads one item. Transient failures are retried.
    pub async fn get(&self, id: Id<M::Item>) -> Result<M::Item> {
        let spec = M::GET.0;
        let mut item = self
            .client
            .request(Method::GET, &render_path(spec, Some(id.get())))
            .send_json::<ItemResponse<M::Item>>()
            .await
            .map(|response| response.data)
            .map_err(|error| error.in_operation(spec.operation_id).about(id))?;
        self.client
            .apply_custom_fields(&mut item)
            .map_err(|error| error.in_operation(spec.operation_id).about(id))?;
        Ok(item)
    }
}

impl<M: Get> Collection<M> {
    /// Reads one item as projection `P`: only the fields `P` declares are
    /// requested. Transient failures are retried.
    pub async fn get_as<P: Projection<Resource = M::Item>>(&self, id: Id<M::Item>) -> Result<P> {
        let spec = M::GET.0;
        let selected_fields = selected_fields::<M::Item, P>(&self.client);
        let mut item = self
            .client
            .request(Method::GET, &render_path(spec, Some(id.get())))
            .query(&[("fields", selected_fields)])
            .send_json::<ItemResponse<ProjectedItem>>()
            .await
            .map(|response| response.data)
            .map_err(|error| error.in_operation(spec.operation_id).about(id))?;
        item.apply_custom_fields(&self.client.custom_fields, M::Item::COLLECTION)
            .map_err(|error| error.in_operation(spec.operation_id).about(id))?;
        projection::decode(item).map_err(|error| error.in_operation(spec.operation_id).about(id))
    }
}

impl<M> Collection<M>
where
    M: Create,
    M::Item: GetItem,
{
    /// Creates an item and returns it as Rentman stored it.
    ///
    /// The request is sent exactly once, without retrying: when only the
    /// response gets lost, a retry would create the item twice. On an error,
    /// [`Error::outcome`](crate::Error::outcome) says whether it may have been applied; when it is
    /// [`RequestOutcome::Unknown`](crate::RequestOutcome::Unknown), read the
    /// collection with [`Self::list`] before creating the item again.
    pub async fn create(&self, payload: &M::Payload) -> Result<M::Item> {
        let spec = M::CREATE.0;
        let response = self
            .client
            .request(Method::POST, &render_path(spec, self.parent))
            .json(payload);
        let retry_policy = self.client.retry_policy.single_attempt();
        let response = response
            .send_json_with::<ItemResponse<ProjectedItem>>(retry_policy)
            .await
            .map_err(|error| error.in_operation(spec.operation_id))?;
        let mut item = response.data;
        let custom_complete = item
            .custom_fields_complete(&self.client.custom_fields, M::Item::COLLECTION)
            .map_err(|error| {
                Error::new(crate::ErrorKind::Decode(error))
                    .in_operation(spec.operation_id)
                    .after_success()
            })?;
        if item.has_field_names(M::Item::ITEM_FIELDS) && custom_complete {
            item.apply_custom_fields(&self.client.custom_fields, M::Item::COLLECTION)
                .map_err(|error| error.in_operation(spec.operation_id).after_success())?;
            return item
                .deserialize()
                .map_err(|error| error.in_operation(spec.operation_id));
        }
        let id = item
            .resource_id::<M::Item>()
            .map_err(|error| error.in_operation(spec.operation_id).after_success())?;
        self.get_item(id).await.map_err(|error| {
            error
                .in_operation(spec.operation_id)
                .about(id)
                .after_success()
        })
    }
}

impl<M> Collection<M>
where
    M: Create,
    <M as Create>::Item: GetItem,
{
    /// Creates an item and returns projection `P`.
    ///
    /// The create request is sent exactly once. If Rentman's response omits
    /// selected fields, the item is fetched once by ID to complete the projection.
    pub async fn create_as<P>(&self, payload: &M::Payload) -> Result<P>
    where
        P: Projection<Resource = <M as Create>::Item>,
    {
        let operation = M::CREATE.0;
        let response = self
            .client
            .request(Method::POST, &render_path(operation, self.parent))
            .json(payload)
            .send_json_with::<ItemResponse<ProjectedItem>>(
                self.client.retry_policy.single_attempt(),
            )
            .await
            .map_err(|error| error.in_operation(operation.operation_id))?;

        let mut item = response.data;
        let custom_complete = item
            .custom_fields_complete(&self.client.custom_fields, <M as Create>::Item::COLLECTION)
            .map_err(|error| {
                Error::new(crate::ErrorKind::Decode(error))
                    .in_operation(operation.operation_id)
                    .after_success()
            })?;
        if item.has_fields::<P>() && custom_complete {
            item.apply_custom_fields(&self.client.custom_fields, <M as Create>::Item::COLLECTION)
                .map_err(|error| error.in_operation(operation.operation_id).after_success())?;
            return projection::decode(item)
                .map_err(|error| error.in_operation(operation.operation_id));
        }

        let id = item
            .resource_id::<<M as Create>::Item>()
            .map_err(|error| error.in_operation(operation.operation_id).after_success())?;
        self.get_projection::<<M as Create>::Item, P>(id)
            .await
            .map_err(|error| {
                error
                    .in_operation(operation.operation_id)
                    .about(id)
                    .after_success()
            })
    }
}

impl<M> Collection<M>
where
    M: Update,
    M::Item: GetItem,
{
    /// Updates an item and returns it as Rentman stored it.
    ///
    /// Rentman leaves fields the payload does not send unchanged. `PUT` is
    /// idempotent, so transient failures are retried.
    pub async fn update(&self, id: Id<M::Item>, payload: &M::Payload) -> Result<M::Item> {
        let spec = M::UPDATE.0;
        let response = self
            .client
            .request(Method::PUT, &render_path(spec, Some(id.get())))
            .json(payload)
            .send_json::<ItemResponse<ProjectedItem>>()
            .await
            .map_err(|error| error.in_operation(spec.operation_id).about(id))?;
        let mut item = response.data;
        let custom_complete = item
            .custom_fields_complete(&self.client.custom_fields, M::Item::COLLECTION)
            .map_err(|error| {
                Error::new(crate::ErrorKind::Decode(error))
                    .in_operation(spec.operation_id)
                    .about(id)
                    .after_success()
            })?;
        if item.has_field_names(M::Item::ITEM_FIELDS) && custom_complete {
            item.apply_custom_fields(&self.client.custom_fields, M::Item::COLLECTION)
                .map_err(|error| {
                    error
                        .in_operation(spec.operation_id)
                        .about(id)
                        .after_success()
                })?;
            return item
                .deserialize()
                .map_err(|error| error.in_operation(spec.operation_id).about(id));
        }
        self.get_item(id).await.map_err(|error| {
            error
                .in_operation(spec.operation_id)
                .about(id)
                .after_success()
        })
    }
}

impl<M> Collection<M>
where
    M: Update,
    M::Item: GetItem,
{
    /// Updates an item and returns projection `P`.
    ///
    /// If Rentman's update response omits selected fields, the item is fetched
    /// once by ID to complete the projection. The PUT is safe to retry.
    pub async fn update_as<P>(&self, id: Id<M::Item>, payload: &M::Payload) -> Result<P>
    where
        P: Projection<Resource = M::Item>,
    {
        let operation = M::UPDATE.0;
        let response = self
            .client
            .request(Method::PUT, &render_path(operation, Some(id.get())))
            .json(payload)
            .send_json::<ItemResponse<ProjectedItem>>()
            .await
            .map_err(|error| error.in_operation(operation.operation_id).about(id))?;

        let mut item = response.data;
        let custom_complete = item
            .custom_fields_complete(&self.client.custom_fields, M::Item::COLLECTION)
            .map_err(|error| {
                Error::new(crate::ErrorKind::Decode(error))
                    .in_operation(operation.operation_id)
                    .about(id)
                    .after_success()
            })?;
        if item.has_fields::<P>() && custom_complete {
            item.apply_custom_fields(&self.client.custom_fields, M::Item::COLLECTION)
                .map_err(|error| {
                    error
                        .in_operation(operation.operation_id)
                        .about(id)
                        .after_success()
                })?;
            return projection::decode(item)
                .map_err(|error| error.in_operation(operation.operation_id).about(id));
        }

        self.get_projection::<M::Item, P>(id)
            .await
            .map_err(|error| {
                error
                    .in_operation(operation.operation_id)
                    .about(id)
                    .after_success()
            })
    }
}

impl<M: Delete> Collection<M> {
    /// Deletes an item. `DELETE` is idempotent, so transient failures are
    /// retried.
    pub async fn delete(&self, id: Id<M::Item>) -> Result<()> {
        let spec = M::DELETE.0;
        self.client
            .request(Method::DELETE, &render_path(spec, Some(id.get())))
            .send_unit()
            .await
            .map_err(|error| error.in_operation(spec.operation_id).about(id))
    }
}

impl<M> Collection<M> {
    async fn get_item<R: GetItem>(&self, id: Id<R>) -> Result<R> {
        let spec = R::GET_ITEM.0;
        let mut item = self
            .client
            .request(Method::GET, &render_path(spec, Some(id.get())))
            .send_json::<ItemResponse<R>>()
            .await
            .map(|response| response.data)
            .map_err(|error| error.in_operation(spec.operation_id).about(id))?;
        self.client
            .apply_custom_fields(&mut item)
            .map_err(|error| error.in_operation(spec.operation_id).about(id))?;
        Ok(item)
    }

    async fn get_projection<R, P>(&self, id: Id<R>) -> Result<P>
    where
        R: GetItem,
        P: Projection<Resource = R>,
    {
        let spec = R::GET_ITEM.0;
        let fields = selected_fields::<R, P>(&self.client);
        let mut item = self
            .client
            .request(Method::GET, &render_path(spec, Some(id.get())))
            .query(&[("fields", fields)])
            .send_json::<ItemResponse<ProjectedItem>>()
            .await
            .map(|response| response.data)
            .map_err(|error| error.in_operation(spec.operation_id).about(id))?;
        item.apply_custom_fields(&self.client.custom_fields, R::COLLECTION)
            .map_err(|error| error.in_operation(spec.operation_id).about(id))?;
        projection::decode(item).map_err(|error| error.in_operation(spec.operation_id).about(id))
    }

    /// Runs an action on one item, e.g. `POST /projects/{id}/:setClient`.
    /// Like [`Collection::create`], it is sent exactly once.
    pub(crate) async fn item_action<P: Serialize, T: DeserializeOwned, R: Resource>(
        &self,
        operation: Operation,
        id: Id<R>,
        payload: &P,
    ) -> Result<T> {
        self.send_once(operation, Some(id.get()), payload)
            .await
            .map_err(|error| error.about(id))
    }

    /// Runs an action on the collection, e.g.
    /// `POST /projects/:createFromTemplate`. It is sent exactly once.
    pub(crate) async fn collection_action<P: Serialize, T: DeserializeOwned>(
        &self,
        operation: Operation,
        payload: &P,
    ) -> Result<T> {
        self.send_once(operation, self.parent, payload).await
    }

    async fn send_once<P: Serialize, T: DeserializeOwned>(
        &self,
        operation: Operation,
        id: Option<u64>,
        payload: &P,
    ) -> Result<T> {
        let spec = operation.0;
        self.client
            .request(Method::POST, &render_path(spec, id))
            .json(payload)
            .send_json_with::<ItemResponse<T>>(self.client.retry_policy.single_attempt())
            .await
            .map(|response| response.data)
            .map_err(|error| error.in_operation(spec.operation_id))
    }
}

/// A request for the items of a collection, built step by step.
pub struct ListRequest<T> {
    client: RentmanClient,
    path: String,
    operation: &'static str,
    page_size: PageSize,
    query: Vec<(String, String)>,
    sort: Vec<String>,
    item: PhantomData<fn() -> T>,
}

impl<T> fmt::Debug for ListRequest<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListRequest")
            .field("path", &self.path)
            .field("page_size", &self.page_size)
            .field("query", &self.query)
            .field("sort", &self.sort)
            .finish()
    }
}

impl<T: Resource + CustomFieldsResource + DeserializeOwned> ListRequest<T> {
    /// Keeps only items matching the filter; several filters all apply.
    ///
    /// Filters are built from the field constants in [`crate::fields`], e.g.
    /// `fields::contact::COUNTRY.eq(model::ContactCountry::Gb)`. Rentman
    /// carries them over to every page.
    pub fn filter(mut self, filter: Filter<T>) -> Self {
        self.query.push((filter.key, filter.value));
        self
    }

    /// Sorts by a field; the first `sort` call has the highest priority.
    ///
    /// When Rentman pages a sorted collection, only the first sort field is
    /// applied before paging, so items sorted by later fields can be ordered
    /// differently across pages. [`Self::first`] reads a single page and is
    /// not affected.
    pub fn sort(mut self, sort: Sort<T>) -> Self {
        self.sort.push(sort.term);
        self
    }

    /// Sets how many items each page holds (default 300, at most 1500).
    pub fn page_size(mut self, page_size: PageSize) -> Self {
        self.page_size = page_size;
        self
    }

    /// Reads every page and returns all items.
    ///
    /// Follows Rentman's `next_page_url` until the collection is complete;
    /// each page is retried on transient failures.
    pub async fn all(self) -> Result<Vec<T>> {
        let sort = self.sort.join(",");
        let query = self.query_pairs(&sort);
        let mut items = self
            .client
            .read_all(CollectionRequest {
                path: &self.path,
                query: &query,
                page_size: self.page_size,
                operation: self.operation,
            })
            .await?;
        for item in &mut items {
            self.client
                .apply_custom_fields(item)
                .map_err(|error| error.in_operation(self.operation))?;
        }
        Ok(items)
    }

    /// Reads only item IDs and revisions for change detection.
    ///
    /// Filters and sorting are preserved. Items that stop matching the filters
    /// appear in `deleted` when compared with a prior snapshot.
    pub async fn revisions(self) -> Result<Revisions<T>>
    where
        T: Revisioned,
    {
        let sort = self.sort.join(",");
        let mut fields = projection::fields::<ItemRevision<T>>();
        if self.client.custom_fields_registered::<T>() {
            fields.push_str(",custom");
        }
        let mut query = self.query_pairs(&sort);
        query.push(("fields", &fields));
        let items: Vec<ProjectedItem> = self
            .client
            .read_all(CollectionRequest {
                path: &self.path,
                query: &query,
                page_size: self.page_size,
                operation: self.operation,
            })
            .await?;
        let mut revisions = Revisions::new();
        for mut item in items {
            item.apply_custom_fields(&self.client.custom_fields, T::COLLECTION)
                .map_err(|error| error.in_operation(self.operation))?;
            let item = ItemRevision::<T>::decode(&mut item)
                .map_err(|error| Error::new(crate::ErrorKind::Decode(error)))
                .map_err(|error| error.in_operation(self.operation))?;
            revisions.insert(item.id, item.revision);
        }
        Ok(revisions)
    }

    /// Reads at most `count` items (1 to 1500) with a single request, in
    /// Rentman's order or the requested sorting.
    pub async fn first(self, count: usize) -> Result<Vec<T>> {
        let limit = PageSize::new(count)
            .map_err(|error| error.in_operation(self.operation))?
            .get()
            .to_string();
        let sort = self.sort.join(",");
        let mut query = self.query_pairs(&sort);
        query.push(("limit", &limit));
        let mut items = self
            .client
            .request(Method::GET, &self.path)
            .query(&query)
            .send_json::<crate::model::CollectionResponse<T>>()
            .await
            .map(|page| page.data)
            .map_err(|error| error.in_operation(self.operation))?;
        for item in &mut items {
            self.client
                .apply_custom_fields(item)
                .map_err(|error| error.in_operation(self.operation))?;
        }
        Ok(items)
    }

    /// Reads every page as projection `P`: only the fields `P` declares are
    /// requested (`fields=`), and Rentman carries the selection through every
    /// page.
    pub async fn all_as<P: Projection<Resource = T>>(self) -> Result<Vec<P>> {
        let fields = selected_fields::<T, P>(&self.client);
        let sort = self.sort.join(",");
        let mut query = self.query_pairs(&sort);
        query.push(("fields", &fields));
        let items: Vec<ProjectedItem> = self
            .client
            .read_all(CollectionRequest {
                path: &self.path,
                query: &query,
                page_size: self.page_size,
                operation: self.operation,
            })
            .await?;
        items
            .into_iter()
            .map(|mut item| {
                item.apply_custom_fields(&self.client.custom_fields, T::COLLECTION)?;
                projection::decode(item)
            })
            .collect::<Result<_>>()
            .map_err(|error| error.in_operation(self.operation))
    }

    /// Reads at most `count` items (1 to 1500) as projection `P` with a single
    /// request.
    pub async fn first_as<P: Projection<Resource = T>>(self, count: usize) -> Result<Vec<P>> {
        let limit = PageSize::new(count)
            .map_err(|error| error.in_operation(self.operation))?
            .get()
            .to_string();
        let fields = selected_fields::<T, P>(&self.client);
        let sort = self.sort.join(",");
        let mut query = self.query_pairs(&sort);
        query.push(("limit", &limit));
        query.push(("fields", &fields));
        let mut items = self
            .client
            .request(Method::GET, &self.path)
            .query(&query)
            .send_json::<crate::model::CollectionResponse<ProjectedItem>>()
            .await
            .map(|page| page.data)
            .map_err(|error| error.in_operation(self.operation))?;
        items
            .drain(..)
            .map(|mut item| {
                item.apply_custom_fields(&self.client.custom_fields, T::COLLECTION)?;
                projection::decode(item)
            })
            .collect::<Result<_>>()
            .map_err(|error| error.in_operation(self.operation))
    }

    /// Filters and the joined sort terms (`sort=+a,-b`) as query pairs.
    fn query_pairs<'a>(&'a self, sort: &'a str) -> Vec<(&'a str, &'a str)> {
        let mut pairs = self
            .query
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect::<Vec<_>>();
        if !sort.is_empty() {
            pairs.push(("sort", sort));
        }
        pairs
    }
}

/// Renders a documented path with the `{id}` of the item or parent item.
fn render_path(spec: EndpointSpec, id: Option<u64>) -> String {
    match id {
        Some(id) => spec.path.replace("{id}", &id.to_string()),
        None => spec.path.to_string(),
    }
}

fn selected_fields<R, P>(client: &RentmanClient) -> String
where
    R: Resource,
    P: Projection<Resource = R>,
{
    let mut fields = projection::fields::<P>();
    if client.custom_fields_registered::<R>() && !P::FIELDS.contains(&"custom") {
        fields.push_str(",custom");
    }
    fields
}

#[cfg(test)]
mod projection_test_types {
    crate::projection! {
        #[allow(missing_docs)]
        pub(super) struct StatusColor from TaskStatus { id, color, r#type }
    }

    crate::projection! {
        #[allow(missing_docs)]
        pub(super) struct EquipmentName from Equipment { id, name }
    }

    crate::projection! {
        #[allow(missing_docs)]
        pub(super) struct EquipmentCustom from Equipment { id, custom }
    }

    crate::projection! {
        #[allow(missing_docs)]
        pub(super) struct InvoiceRefTest from Invoice { id }
    }

    crate::projection! {
        #[allow(missing_docs)]
        pub(super) struct PaymentRefTest from Payment { id, invoice }
    }

    crate::projection! {
        #[allow(missing_docs)]
        pub(super) struct EquipmentPricesTest from Equipment { id, price, list_price, subrental_costs }
    }
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::projection_test_types::{EquipmentCustom, EquipmentName, StatusColor};
    use super::*;
    use crate::test_support::*;
    use crate::{
        ErrorKind, InvalidRequest, RentmanApiToken, RentmanClientConfig, RequestOutcome,
        ResourceRef, Url,
        model::{
            Equipment, EquipmentId, ProjectId, ProjectsSetTypePayload, TaskStatus, TaskStatusId,
            TaskStatusPayload, TaskStatusType,
        },
    };
    use reqwest::StatusCode;
    use std::{num::NonZeroUsize, time::Duration};

    const TASK_STATUS: &str = r#"{"id":5,"updateHash":"hash-5","created":null,"modified":null,"creator":"/crew/1","displayname":"Done","name":"Done","color":"green","type":"done","order":2}"#;

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

    fn item(body: &str) -> String {
        json_response(&format!(r#"{{"data":{body}}}"#))
    }

    fn complete_equipment_response(id: u64, custom: serde_json::Value) -> serde_json::Value {
        let mut item = serde_json::json!({});
        for field in <Equipment as GetItem>::ITEM_FIELDS {
            item[*field] = serde_json::Value::Null;
        }
        item["id"] = serde_json::json!(id);
        item["updateHash"] = serde_json::json!(format!("equipment-{id}"));
        item["created"] = serde_json::Value::Null;
        item["modified"] = serde_json::Value::Null;
        item["creator"] = serde_json::Value::Null;
        item["custom"] = custom;
        item
    }

    fn item_value(value: serde_json::Value) -> String {
        item(&value.to_string())
    }

    #[tokio::test]
    async fn lists_every_page_of_a_collection() {
        let server = spawn_server(vec![
            json_response(&format!(
                r#"{{"data":[{TASK_STATUS}],"itemCount":1,"limit":1,"next_page_url":"{{base}}/taskstatuses?cursor=2"}}"#
            )),
            json_response(&format!(
                r#"{{"data":[{}],"itemCount":1,"limit":2,"next_page_url":null}}"#,
                TASK_STATUS.replace(r#""id":5"#, r#""id":6"#)
            )),
        ]);
        let client = client(&server);

        let statuses: Vec<TaskStatus> = client
            .task_statuses()
            .list()
            .page_size(PageSize::new(1).unwrap())
            .all()
            .await
            .unwrap();

        assert_eq!(statuses.len(), 2);
        assert_eq!(statuses[0].id, TaskStatusId::new(5));
        assert_eq!(statuses[0].type_, Some(TaskStatusType::Done));
        assert_eq!(statuses[1].id, TaskStatusId::new(6));
        let first = server.next_request();
        assert_request(&first, "GET", "/taskstatuses");
        assert_eq!(first.query_value("limit").as_deref(), Some("1"));
        assert_eq!(
            server.next_request().query_value("cursor").as_deref(),
            Some("2")
        );
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn reads_revision_snapshots_through_every_page() {
        use crate::fields::task_status;
        use crate::revision::Revision;

        let server = spawn_server(vec![
            json_response(
                r#"{"data":[{"id":6,"updateHash":"h6"}],"itemCount":2,"limit":1,"next_page_url":"{base}/taskstatuses?fields=id&type=done&cursor=2"}"#,
            ),
            json_response(
                r#"{"data":[{"id":5,"updateHash":"h5"}],"itemCount":1,"limit":2,"next_page_url":null}"#,
            ),
            json_response(r#"{"data":[{"id":7}],"itemCount":1,"limit":300}"#),
            text_response("HTTP/1.1 400 Bad Request", "broken"),
        ]);
        let client = client(&server);

        let revisions = client
            .task_statuses()
            .list()
            .filter(task_status::TYPE.eq(TaskStatusType::Done))
            .page_size(PageSize::new(1).unwrap())
            .revisions()
            .await
            .unwrap();
        assert_eq!(
            revisions.iter().collect::<Vec<_>>(),
            [
                (TaskStatusId::new(5), &Revision::new("h5").unwrap()),
                (TaskStatusId::new(6), &Revision::new("h6").unwrap()),
            ]
        );
        let first = server.next_request();
        assert_request(&first, "GET", "/taskstatuses");
        assert_eq!(first.query_value("fields").as_deref(), Some("id"));
        assert_eq!(first.query_value("type").as_deref(), Some("done"));
        assert_eq!(
            server.next_request().query_value("cursor").as_deref(),
            Some("2")
        );

        let error = client.task_statuses().list().revisions().await.unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.operation(), Some("getTaskStatusCollection"));
        assert_request(&server.next_request(), "GET", "/taskstatuses");

        let error = client.task_statuses().list().revisions().await.unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        assert_eq!(error.operation(), Some("getTaskStatusCollection"));
        assert_request(&server.next_request(), "GET", "/taskstatuses");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn all_returns_paging_failures_with_the_collection_operation() {
        let server = spawn_server(vec![text_response("HTTP/1.1 400 Bad Request", "broken")]);
        let error = client(&server)
            .task_statuses()
            .list()
            .all()
            .await
            .unwrap_err();

        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        assert_eq!(error.operation(), Some("getTaskStatusCollection"));
        assert_request(&server.next_request(), "GET", "/taskstatuses");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn revisions_validate_registered_custom_fields_and_ids() {
        use crate::{CustomFieldDefinition, CustomFieldRegistry, model::ExtraInputFieldType};

        let server = spawn_server(vec![
            json_response(
                r#"{"data":[{"id":1,"updateHash":"h1","custom":{"custom_8":true}}],"itemCount":1,"limit":300}"#,
            ),
            json_response(
                r#"{"data":[{"id":"not-an-id","updateHash":"h2"}],"itemCount":1,"limit":300}"#,
            ),
        ]);
        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<TaskStatus>(
            8,
            ExtraInputFieldType::Text,
        )])
        .unwrap();
        let configured_client = client(&server).with_custom_fields(registry);

        let error = configured_client
            .task_statuses()
            .list()
            .revisions()
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::CustomField(_)));
        assert_request(&server.next_request(), "GET", "/taskstatuses");

        let error = client(&server)
            .task_statuses()
            .list()
            .revisions()
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.operation(), Some("getTaskStatusCollection"));
        let request = server.next_request();
        assert_request(&request, "GET", "/taskstatuses");
        assert_eq!(request.query_value("fields").as_deref(), Some("id"));
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn sends_filters_and_sorting_and_keeps_them_across_pages() {
        use crate::fields::task_status;
        let server = spawn_server(vec![
            json_response(&format!(
                r#"{{"data":[{TASK_STATUS}],"itemCount":1,"limit":1,"next_page_url":"{{base}}/taskstatuses?type=done&order%5Bgte%5D=2&sort=%2Border%2C-name&cursor=2"}}"#
            )),
            json_response(r#"{"data":[],"itemCount":0,"limit":1,"next_page_url":null}"#),
            json_response(&format!(
                r#"{{"data":[{TASK_STATUS}],"itemCount":1,"limit":5}}"#
            )),
        ]);
        let client = client(&server);

        let statuses = client
            .task_statuses()
            .list()
            .filter(task_status::TYPE.eq(TaskStatusType::Done))
            .filter(task_status::ORDER.gte(2))
            .sort(task_status::ORDER.asc())
            .sort(task_status::NAME.desc())
            .page_size(PageSize::new(1).unwrap())
            .all()
            .await
            .unwrap();
        assert_eq!(statuses.len(), 1);

        let first = server.next_request();
        assert_request(&first, "GET", "/taskstatuses");
        assert_eq!(first.query_value("type").as_deref(), Some("done"));
        assert_eq!(first.query_value("order[gte]").as_deref(), Some("2"));
        assert_eq!(first.query_value("sort").as_deref(), Some("+order,-name"));
        assert_eq!(first.query_value("limit").as_deref(), Some("1"));
        // The cursor carries the filters; it is followed unchanged.
        let second = server.next_request();
        assert_eq!(second.query_value("cursor").as_deref(), Some("2"));
        assert_eq!(second.query_value("type").as_deref(), Some("done"));
        assert_eq!(second.query_value("order[gte]").as_deref(), Some("2"));

        let top = client
            .task_statuses()
            .list()
            .filter(task_status::COLOR.neq("red".to_string()))
            .sort(task_status::ORDER.desc())
            .first(5)
            .await
            .unwrap();
        assert_eq!(top.len(), 1);
        let request = server.next_request();
        assert_eq!(request.query_value("color[neq]").as_deref(), Some("red"));
        assert_eq!(request.query_value("sort").as_deref(), Some("-order"));
        assert_eq!(request.query_value("limit").as_deref(), Some("5"));
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn projection_requests_registered_custom_fields_but_not_update_hash() {
        use crate::{CustomFieldDefinition, CustomFieldRegistry, model::ExtraInputFieldType};

        let server = spawn_server(vec![json_response(
            r#"{"data":{"id":5,"updateHash":"hash-5","name":"Mixer","custom":{"custom_8":"blue","custom_9":false}}}"#,
        )]);
        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<Equipment>(
            8,
            ExtraInputFieldType::Text,
        )])
        .unwrap();
        let client = client(&server).with_custom_fields(registry);

        let equipment = client
            .equipment()
            .get_as::<EquipmentName>(crate::model::EquipmentId::new(5))
            .await
            .unwrap();

        assert_eq!(equipment.name, "Mixer");
        let request = server.next_request();
        assert_eq!(
            request.query_value("fields").as_deref(),
            Some("id,name,custom")
        );
        assert!(
            !request
                .query_value("fields")
                .as_deref()
                .unwrap()
                .contains("updateHash")
        );
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn projection_with_custom_field_does_not_duplicate_the_selection() {
        use crate::{CustomFieldDefinition, CustomFieldRegistry, model::ExtraInputFieldType};

        let server = spawn_server(vec![json_response(
            r#"{"data":{"id":5,"custom":{"custom_8":"blue"}}}"#,
        )]);
        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<Equipment>(
            8,
            ExtraInputFieldType::Text,
        )])
        .unwrap();
        let client = client(&server).with_custom_fields(registry);

        let equipment = client
            .equipment()
            .get_as::<EquipmentCustom>(crate::model::EquipmentId::new(5))
            .await
            .unwrap();
        assert_eq!(equipment.id, crate::model::EquipmentId::new(5));
        assert_eq!(equipment.custom.unwrap().get::<String>(8).unwrap(), "blue");
        let request = server.next_request();
        assert_eq!(request.query_value("fields").as_deref(), Some("id,custom"));
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn live_projection_types_cover_collection_and_item_error_paths() {
        use super::projection_test_types::{
            EquipmentName, EquipmentPricesTest, InvoiceRefTest, PaymentRefTest,
        };

        let server = spawn_server(vec![
            text_response("HTTP/1.1 400 Bad Request", "rejected"),
            text_response("HTTP/1.1 400 Bad Request", "rejected"),
            text_response("HTTP/1.1 400 Bad Request", "rejected"),
            text_response("HTTP/1.1 400 Bad Request", "rejected"),
        ]);
        let client = client(&server);

        for error in [
            client
                .invoices()
                .list()
                .all_as::<InvoiceRefTest>()
                .await
                .unwrap_err(),
            client
                .payments()
                .list()
                .all_as::<PaymentRefTest>()
                .await
                .unwrap_err(),
            client
                .equipment()
                .list()
                .all_as::<EquipmentName>()
                .await
                .unwrap_err(),
            client
                .equipment()
                .get_as::<EquipmentPricesTest>(EquipmentId::new(7))
                .await
                .unwrap_err(),
        ] {
            assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        }

        let invoice = server.next_request();
        assert_request(&invoice, "GET", "/invoices");
        assert_eq!(invoice.query_value("fields").as_deref(), Some("id"));
        let payment = server.next_request();
        assert_request(&payment, "GET", "/payments");
        assert_eq!(payment.query_value("fields").as_deref(), Some("id,invoice"));
        let equipment = server.next_request();
        assert_request(&equipment, "GET", "/equipment");
        assert_eq!(equipment.query_value("fields").as_deref(), Some("id,name"));
        let prices = server.next_request();
        assert_request(&prices, "GET", "/equipment/7");
        assert_eq!(
            prices.query_value("fields").as_deref(),
            Some("id,price,list_price,subrental_costs")
        );
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn first_items_cover_read_and_custom_validation_errors() {
        use crate::{CustomFieldDefinition, CustomFieldRegistry, model::ExtraInputFieldType};

        let server = spawn_server(vec![
            text_response("HTTP/1.1 400 Bad Request", "broken"),
            json_response(
                r#"{"data":[{"id":1,"updateHash":"h1","created":null,"modified":null,"creator":null}],"itemCount":1,"limit":1}"#,
            ),
        ]);
        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<Equipment>(
            8,
            ExtraInputFieldType::Text,
        )])
        .unwrap();
        let client = client(&server).with_custom_fields(registry);

        let error = client.task_statuses().list().first(1).await.unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        assert_eq!(error.operation(), Some("getTaskStatusCollection"));
        assert_request(&server.next_request(), "GET", "/taskstatuses");

        let error = client.equipment().list().first(1).await.unwrap_err();
        assert!(matches!(
            error.kind(),
            ErrorKind::CustomField(crate::CustomFieldError::MissingValue { id: 8, .. })
        ));
        assert_eq!(error.operation(), Some("getEquipmentCollection"));
        assert_request(&server.next_request(), "GET", "/equipment");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn reads_projections_with_their_field_selection() {
        use crate::fields::task_status;
        let server = spawn_server(vec![
            json_response(
                r#"{"data":[{"id":5,"color":"green","type":"done","created":null,"custom":{}}],"itemCount":1,"limit":1,"next_page_url":"{base}/taskstatuses?fields=id%2Ccolor%2Ctype&type=done&cursor=2"}"#,
            ),
            json_response(
                r#"{"data":[{"id":6,"color":"red","type":"todo"}],"itemCount":1,"limit":2,"next_page_url":null}"#,
            ),
            json_response(
                r#"{"data":[{"id":5,"color":"green","type":"done"}],"itemCount":1,"limit":3}"#,
            ),
            item(r#"{"id":5,"color":"green","type":"done"}"#),
            json_response(r#"{"data":[{"id":5}],"itemCount":1,"limit":1500,"next_page_url":null}"#),
            json_response(r#"{"data":[{"id":5}],"itemCount":1,"limit":3}"#),
            item(r#"{"id":5}"#),
            text_response("HTTP/1.1 400 Bad Request", "broken"),
        ]);
        let client = client(&server);

        let all = client
            .task_statuses()
            .list()
            .filter(task_status::TYPE.eq(TaskStatusType::Done))
            .sort(task_status::ORDER.asc())
            .page_size(PageSize::new(1).unwrap())
            .all_as::<StatusColor>()
            .await
            .unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[1].color, "red");
        assert_eq!(all[1].r#type, TaskStatusType::Todo);
        let first = server.next_request();
        assert_request(&first, "GET", "/taskstatuses");
        assert_eq!(
            first.query_value("fields").as_deref(),
            Some("id,color,type")
        );
        assert_eq!(first.query_value("type").as_deref(), Some("done"));
        assert_eq!(first.query_value("sort").as_deref(), Some("+order"));
        // The cursor carries the selection; it is followed unchanged.
        let second = server.next_request();
        assert_eq!(
            second.query_value("fields").as_deref(),
            Some("id,color,type")
        );
        assert_eq!(second.query_value("cursor").as_deref(), Some("2"));

        let top = client
            .task_statuses()
            .list()
            .first_as::<StatusColor>(3)
            .await
            .unwrap();
        assert_eq!(top[0].id, TaskStatusId::new(5));
        let request = server.next_request();
        assert_eq!(
            request.query_value("fields").as_deref(),
            Some("id,color,type")
        );
        assert_eq!(request.query_value("limit").as_deref(), Some("3"));

        let one: StatusColor = client
            .task_statuses()
            .get_as(TaskStatusId::new(5))
            .await
            .unwrap();
        assert_eq!(one.color, "green");
        let request = server.next_request();
        assert_request(&request, "GET", "/taskstatuses/5");
        assert_eq!(
            request.query_value("fields").as_deref(),
            Some("id,color,type")
        );

        // Items without the selected required fields fail with the operation.
        let error = client
            .task_statuses()
            .list()
            .all_as::<StatusColor>()
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.operation(), Some("getTaskStatusCollection"));
        server.next_request();
        let error = client
            .task_statuses()
            .list()
            .first_as::<StatusColor>(3)
            .await
            .unwrap_err();
        assert_eq!(error.operation(), Some("getTaskStatusCollection"));
        server.next_request();
        let error = client
            .task_statuses()
            .get_as::<StatusColor>(TaskStatusId::new(5))
            .await
            .unwrap_err();
        assert_eq!(error.operation(), Some("getTaskStatusItem"));
        assert_eq!(
            error.resource(),
            Some(ResourceRef::from(TaskStatusId::new(5)))
        );
        server.next_request();

        let error = client
            .task_statuses()
            .list()
            .all_as::<StatusColor>()
            .await
            .unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        server.next_request();

        let error = client
            .task_statuses()
            .list()
            .first_as::<StatusColor>(0)
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::InvalidRequest(_)));
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn first_projection_covers_http_custom_and_projection_errors() {
        use crate::{CustomFieldDefinition, CustomFieldRegistry, model::ExtraInputFieldType};

        let server = spawn_server(vec![
            text_response("HTTP/1.1 400 Bad Request", "broken"),
            json_response(
                r#"{"data":[{"id":1,"color":"green","type":"done"}],"itemCount":1,"limit":1}"#,
            ),
            json_response(
                r#"{"data":[{"id":2,"custom":{"custom_8":"blue"}}],"itemCount":1,"limit":1}"#,
            ),
        ]);
        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<Equipment>(
            8,
            ExtraInputFieldType::Text,
        )])
        .unwrap();
        let configured_client = client(&server).with_custom_fields(registry);

        let error = configured_client
            .task_statuses()
            .list()
            .first_as::<StatusColor>(1)
            .await
            .unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        assert_eq!(error.operation(), Some("getTaskStatusCollection"));
        assert_request(&server.next_request(), "GET", "/taskstatuses");

        let error = configured_client
            .equipment()
            .list()
            .first_as::<EquipmentName>(1)
            .await
            .unwrap_err();
        assert!(matches!(
            error.kind(),
            ErrorKind::CustomField(crate::CustomFieldError::MissingValue { id: 8, .. })
        ));
        assert_request(&server.next_request(), "GET", "/equipment");

        let client = client(&server);
        let error = client
            .equipment()
            .list()
            .first_as::<EquipmentName>(1)
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.operation(), Some("getEquipmentCollection"));
        assert_request(&server.next_request(), "GET", "/equipment");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn reads_the_first_items_with_one_request() {
        let server = spawn_server(vec![
            json_response(&format!(
                r#"{{"data":[{TASK_STATUS}],"itemCount":1,"limit":1,"next_page_url":"{{base}}/taskstatuses?cursor=2"}}"#
            )),
            text_response("HTTP/1.1 400 Bad Request", "broken"),
        ]);
        let client = client(&server);

        let statuses = client.task_statuses().list().first(1).await.unwrap();

        assert_eq!(statuses.len(), 1);
        let request = server.next_request();
        assert_request(&request, "GET", "/taskstatuses");
        assert_eq!(request.query_value("limit").as_deref(), Some("1"));
        assert_eq!(request.query_value("offset"), None);

        let error = client.task_statuses().list().first(2).await.unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        assert_eq!(error.operation(), Some("getTaskStatusCollection"));
        server.next_request();

        for count in [0, 1501] {
            let error = client
                .task_statuses()
                .list()
                .first(count)
                .await
                .unwrap_err();
            assert!(matches!(
                error.kind(),
                ErrorKind::InvalidRequest(InvalidRequest::PageSize { requested }) if *requested == count
            ));
            assert_eq!(error.operation(), Some("getTaskStatusCollection"));
        }
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn gets_one_item_and_names_it_in_errors() {
        let server = spawn_server(vec![
            item(TASK_STATUS),
            text_response("HTTP/1.1 404 Not Found", "gone"),
        ]);
        let client = client(&server);

        let status = client
            .task_statuses()
            .get(TaskStatusId::new(5))
            .await
            .unwrap();
        assert_eq!(status.name.as_deref(), Some("Done"));
        assert_request(&server.next_request(), "GET", "/taskstatuses/5");

        let error = client
            .task_statuses()
            .get(TaskStatusId::new(6))
            .await
            .unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::NOT_FOUND));
        assert_eq!(error.operation(), Some("getTaskStatusItem"));
        assert_eq!(
            error.resource(),
            Some(ResourceRef::from(TaskStatusId::new(6)))
        );
        assert_request(&server.next_request(), "GET", "/taskstatuses/6");
    }

    #[tokio::test]
    async fn get_as_attaches_request_and_custom_field_errors() {
        use crate::{CustomFieldDefinition, CustomFieldRegistry, model::ExtraInputFieldType};

        let server = spawn_server(vec![
            text_response("HTTP/1.1 404 Not Found", "gone"),
            item(
                r#"{"id":5,"updateHash":"h5","created":null,"modified":null,"creator":null,"name":"Mixer"}"#,
            ),
        ]);
        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<Equipment>(
            8,
            ExtraInputFieldType::Text,
        )])
        .unwrap();
        let client = client(&server).with_custom_fields(registry);

        let error = client
            .equipment()
            .get_as::<EquipmentName>(crate::model::EquipmentId::new(5))
            .await
            .unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::NOT_FOUND));
        assert_eq!(error.operation(), Some("getEquipmentItem"));
        assert_eq!(
            error.resource(),
            Some(ResourceRef::from(crate::model::EquipmentId::new(5)))
        );
        server.next_request();

        let error = client
            .equipment()
            .get_as::<EquipmentName>(crate::model::EquipmentId::new(5))
            .await
            .unwrap_err();
        assert!(matches!(
            error.kind(),
            ErrorKind::CustomField(crate::CustomFieldError::MissingValue { id: 8, .. })
        ));
        assert_eq!(error.operation(), Some("getEquipmentItem"));
        assert_request(&server.next_request(), "GET", "/equipment/5");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn validates_and_filters_registered_custom_fields_on_item_reads() {
        use crate::{CustomFieldDefinition, CustomFieldRegistry, model::ExtraInputFieldType};

        let server = spawn_server(vec![
            json_response(
                r#"{"data":{"id":5,"updateHash":"hash-5","created":null,"modified":null,"creator":null,"custom":{"custom_8":"ok","custom_9":false}}}"#,
            ),
            json_response(
                r#"{"data":{"id":6,"updateHash":"hash-6","created":null,"modified":null,"creator":null,"custom":{"custom_9":false}}}"#,
            ),
        ]);
        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<
            crate::model::Equipment,
        >(8, ExtraInputFieldType::Text)])
        .unwrap();
        let client = client(&server).with_custom_fields(registry);

        let equipment = client
            .equipment()
            .get(crate::model::EquipmentId::new(5))
            .await
            .unwrap();
        let custom = equipment.custom.as_ref().unwrap();
        assert_eq!(custom.0.len(), 1);
        assert_eq!(
            custom.0.get("custom_8"),
            Some(&crate::model::CustomFieldValue::Text("ok".to_string()))
        );
        assert_request(&server.next_request(), "GET", "/equipment/5");

        let error = client
            .equipment()
            .get(crate::model::EquipmentId::new(6))
            .await
            .unwrap_err();
        assert!(matches!(
            error.kind(),
            ErrorKind::CustomField(crate::CustomFieldError::MissingValue { id: 8, .. })
        ));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_eq!(error.operation(), Some("getEquipmentItem"));
        assert_request(&server.next_request(), "GET", "/equipment/6");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn creates_exactly_once() {
        let server = spawn_server(vec![
            item(TASK_STATUS),
            text_response("HTTP/1.1 503 Service Unavailable", "later"),
            item(TASK_STATUS),
        ]);
        let client = client(&server);
        let payload = TaskStatusPayload::new("green".to_string());

        let created = client.task_statuses().create(&payload).await.unwrap();
        assert_eq!(created.id, TaskStatusId::new(5));
        let request = server.next_request();
        assert_request(&request, "POST", "/taskstatuses");
        assert_eq!(request.json_body(), serde_json::json!({"color": "green"}));

        // A transient failure is not retried: the outcome is unknown.
        let error = client.task_statuses().create(&payload).await.unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::SERVICE_UNAVAILABLE));
        assert_eq!(error.outcome(), RequestOutcome::Unknown);
        assert_eq!(error.operation(), Some("createTaskStatus"));
        assert_request(&server.next_request(), "POST", "/taskstatuses");
        // The third prepared response is still unused.
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn create_marks_all_success_response_decode_failures_applied() {
        let invalid_value = TASK_STATUS.replace(r#""color":"green""#, r#""color":1"#);
        let server = spawn_server(vec![
            item(&invalid_value),
            item(r#"{"id":7,"updateHash":"h7","created":null,"modified":null,"creator":null}"#),
            item(
                r#"{"id":7,"updateHash":"h7","created":null,"modified":null,"creator":null,"custom":"invalid"}"#,
            ),
            item(r#"{"id":8,"updateHash":"h8","created":null,"modified":null,"creator":null}"#),
            item(r#"{"id":8,"updateHash":"h8","created":null,"modified":null,"creator":null}"#),
        ]);
        let client = client(&server);
        let status_payload = TaskStatusPayload::new("green".to_string());

        let decode = client
            .task_statuses()
            .create(&status_payload)
            .await
            .unwrap_err();
        assert!(matches!(decode.kind(), ErrorKind::Decode(_)));
        assert_eq!(decode.outcome(), RequestOutcome::Applied);
        server.next_request();

        let registry =
            crate::CustomFieldRegistry::new([crate::CustomFieldDefinition::new::<Equipment>(
                8,
                crate::model::ExtraInputFieldType::Text,
            )])
            .unwrap();
        let client = client.with_custom_fields(registry);
        let payload = crate::model::EquipmentPayload::default();
        let malformed = client.equipment().create(&payload).await.unwrap_err();
        assert!(matches!(malformed.kind(), ErrorKind::Decode(_)));
        assert_eq!(malformed.outcome(), RequestOutcome::Applied);
        server.next_request();
        server.next_request();

        let invalid_registry_target = client.equipment().create(&payload).await.unwrap_err();
        assert!(matches!(
            invalid_registry_target.kind(),
            ErrorKind::CustomField(crate::CustomFieldError::MissingValue { id: 8, .. })
        ));
        assert_eq!(invalid_registry_target.outcome(), RequestOutcome::Applied);
        server.next_request();
        server.next_request();
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn create_complete_response_covers_deserialization_and_registry_errors() {
        use crate::{CustomFieldDefinition, CustomFieldRegistry, model::ExtraInputFieldType};

        let mut invalid_price = complete_equipment_response(1, serde_json::json!(null));
        invalid_price["price"] = serde_json::json!("not a number");
        let wrong_custom_type =
            complete_equipment_response(2, serde_json::json!({"custom_8": true}));
        let malformed_custom = complete_equipment_response(3, serde_json::json!("invalid"));
        let server = spawn_server(vec![
            item_value(invalid_price),
            item_value(wrong_custom_type),
            item_value(malformed_custom),
        ]);
        let payload = crate::model::EquipmentPayload::default();
        let client = client(&server);

        let error = client.equipment().create(&payload).await.unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_request(&server.next_request(), "POST", "/equipment");

        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<Equipment>(
            8,
            ExtraInputFieldType::Text,
        )])
        .unwrap();
        let client = client.with_custom_fields(registry);
        let error = client.equipment().create(&payload).await.unwrap_err();
        assert!(matches!(
            error.kind(),
            ErrorKind::CustomField(crate::CustomFieldError::TypeMismatch { .. })
        ));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_request(&server.next_request(), "POST", "/equipment");

        let error = client.equipment().create(&payload).await.unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_request(&server.next_request(), "POST", "/equipment");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn create_and_update_fetch_the_complete_item_after_partial_responses() {
        let partial = TASK_STATUS.replace(r#""color":"green","#, "");
        let server = spawn_server(vec![
            json_response(&format!(r#"{{"data":{partial}}}"#)),
            item(TASK_STATUS),
            json_response(&format!(r#"{{"data":{partial}}}"#)),
            item(TASK_STATUS),
        ]);
        let client = client(&server);
        let payload = TaskStatusPayload::new("green".to_string());

        let created = client.task_statuses().create(&payload).await.unwrap();
        assert_eq!(created.name.as_deref(), Some("Done"));
        assert_eq!(created.color.as_deref(), Some("green"));
        assert_request(&server.next_request(), "POST", "/taskstatuses");
        assert_request(&server.next_request(), "GET", "/taskstatuses/5");

        let updated = client
            .task_statuses()
            .update(TaskStatusId::new(6), &payload)
            .await
            .unwrap();
        assert_eq!(updated.name.as_deref(), Some("Done"));
        assert_eq!(updated.color.as_deref(), Some("green"));
        assert_request(&server.next_request(), "PUT", "/taskstatuses/6");
        assert_request(&server.next_request(), "GET", "/taskstatuses/6");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn create_reports_applied_decode_and_fallback_failures() {
        let malformed = TASK_STATUS.replace(r#""color":"green""#, r#""color":1"#);
        let server = spawn_server(vec![
            item(&malformed),
            item(r#"{"updateHash":"hash","created":null,"modified":null,"creator":null}"#),
            item(r#"{"id":7,"updateHash":"hash","created":null,"modified":null,"creator":null}"#),
            text_response("HTTP/1.1 404 Not Found", "gone"),
        ]);
        let client = client(&server);
        let payload = TaskStatusPayload::new("green".to_string());

        let decode = client.task_statuses().create(&payload).await.unwrap_err();
        assert!(matches!(decode.kind(), ErrorKind::Decode(_)));
        assert_eq!(decode.outcome(), RequestOutcome::Applied);
        assert_eq!(decode.operation(), Some("createTaskStatus"));
        server.next_request();

        let missing_id = client.task_statuses().create(&payload).await.unwrap_err();
        assert!(matches!(missing_id.kind(), ErrorKind::Decode(_)));
        assert_eq!(missing_id.outcome(), RequestOutcome::Applied);
        server.next_request();

        let fallback = client.task_statuses().create(&payload).await.unwrap_err();
        assert_eq!(fallback.status(), Some(StatusCode::NOT_FOUND));
        assert_eq!(fallback.outcome(), RequestOutcome::Applied);
        assert_eq!(fallback.operation(), Some("createTaskStatus"));
        assert_eq!(
            fallback.resource(),
            Some(ResourceRef::from(TaskStatusId::new(7)))
        );
        server.next_request();
        server.next_request();
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn create_as_validates_custom_types_and_fetches_registered_values() {
        use crate::{CustomFieldDefinition, CustomFieldRegistry, model::ExtraInputFieldType};

        let server = spawn_server(vec![
            item(r#"{"id":5,"updateHash":"h5","name":"Mixer","custom":{"custom_8":true}}"#),
            item(r#"{"id":6,"updateHash":"h6"}"#),
            item(
                r#"{"id":6,"updateHash":"h6","created":null,"modified":null,"creator":null,"name":"Mixer","custom":{"custom_8":"blue","custom_9":false}}"#,
            ),
            item(r#"{"updateHash":"h7","name":"Mixer"}"#),
        ]);
        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<Equipment>(
            8,
            ExtraInputFieldType::Text,
        )])
        .unwrap();
        let client = client(&server).with_custom_fields(registry);
        let payload = crate::model::EquipmentPayload::default();

        let error = client
            .equipment()
            .create_as::<EquipmentName>(&payload)
            .await
            .unwrap_err();
        assert!(matches!(
            error.kind(),
            ErrorKind::CustomField(crate::CustomFieldError::TypeMismatch { .. })
        ));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        server.next_request();

        let created = client
            .equipment()
            .create_as::<EquipmentName>(&payload)
            .await
            .unwrap();
        assert_eq!(created.id, crate::model::EquipmentId::new(6));
        assert_eq!(created.name, "Mixer");
        let post = server.next_request();
        assert_request(&post, "POST", "/equipment");
        let get = server.next_request();
        assert_request(&get, "GET", "/equipment/6");
        assert_eq!(get.query_value("fields").as_deref(), Some("id,name,custom"));

        let error = client
            .equipment()
            .create_as::<EquipmentName>(&payload)
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_eq!(error.operation(), Some("createEquipment"));
        server.next_request();
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn update_as_validates_custom_types_and_marks_fallback_failures_applied() {
        use crate::{CustomFieldDefinition, CustomFieldRegistry, model::ExtraInputFieldType};

        let server = spawn_server(vec![
            item(r#"{"id":5,"updateHash":"h5","name":"Mixer","custom":{"custom_8":true}}"#),
            item(r#"{"id":6,"updateHash":"h6","name":"Mixer"}"#),
            item(
                r#"{"id":6,"updateHash":"h6","created":null,"modified":null,"creator":null,"name":"Mixer","custom":{"custom_8":"blue","custom_9":false}}"#,
            ),
            item(r#"{"id":7,"updateHash":"h7"}"#),
            text_response("HTTP/1.1 503 Service Unavailable", "later"),
            text_response("HTTP/1.1 503 Service Unavailable", "later"),
        ]);
        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<Equipment>(
            8,
            ExtraInputFieldType::Text,
        )])
        .unwrap();
        let client = client(&server).with_custom_fields(registry);
        let payload = crate::model::EquipmentPayload::default();

        let error = client
            .equipment()
            .update_as::<EquipmentName>(crate::model::EquipmentId::new(5), &payload)
            .await
            .unwrap_err();
        assert!(matches!(
            error.kind(),
            ErrorKind::CustomField(crate::CustomFieldError::TypeMismatch { .. })
        ));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        server.next_request();

        let updated = client
            .equipment()
            .update_as::<EquipmentName>(crate::model::EquipmentId::new(6), &payload)
            .await
            .unwrap();
        assert_eq!(updated.name, "Mixer");
        assert_request(&server.next_request(), "PUT", "/equipment/6");
        let get = server.next_request();
        assert_request(&get, "GET", "/equipment/6");
        assert_eq!(get.query_value("fields").as_deref(), Some("id,name,custom"));

        let error = client
            .equipment()
            .update_as::<EquipmentName>(crate::model::EquipmentId::new(7), &payload)
            .await
            .unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::SERVICE_UNAVAILABLE));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_eq!(error.operation(), Some("updateEquipment"));
        assert_eq!(
            error.resource(),
            Some(ResourceRef::from(crate::model::EquipmentId::new(7)))
        );
        server.next_request();
        server.next_request();
        server.next_request();
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn projection_writes_cover_request_decode_and_fallback_errors() {
        use crate::{CustomFieldDefinition, CustomFieldRegistry, model::ExtraInputFieldType};

        let invalid_status = TASK_STATUS.replace(r#""color":"green""#, r#""color":1"#);
        let server = spawn_server(vec![
            text_response("HTTP/1.1 400 Bad Request", "rejected"),
            item(r#"{"id":6,"updateHash":"h6","name":"Mixer","custom":"invalid"}"#),
            item(r#"{"id":7,"updateHash":"h7","name":"Mixer"}"#),
            item(r#"{"id":7,"updateHash":"h7","name":"Mixer","custom":{"custom_8":true}}"#),
            item(r#"{"id":9,"updateHash":"h9","name":"Mixer"}"#),
            item(r#"{"id":9,"updateHash":"h9","custom":{"custom_8":"blue"}}"#),
            item(&invalid_status),
            text_response("HTTP/1.1 400 Bad Request", "rejected"),
            item(r#"{"id":8,"updateHash":"h8","name":"Mixer","custom":"invalid"}"#),
            item(&invalid_status),
            item(TASK_STATUS),
        ]);
        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<Equipment>(
            8,
            ExtraInputFieldType::Text,
        )])
        .unwrap();
        let client = client(&server).with_custom_fields(registry);
        let equipment_payload = crate::model::EquipmentPayload::default();

        let error = client
            .equipment()
            .create_as::<EquipmentName>(&equipment_payload)
            .await
            .unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        assert_eq!(error.operation(), Some("createEquipment"));
        assert_request(&server.next_request(), "POST", "/equipment");

        let error = client
            .equipment()
            .create_as::<EquipmentName>(&equipment_payload)
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_request(&server.next_request(), "POST", "/equipment");

        let error = client
            .equipment()
            .create_as::<EquipmentName>(&equipment_payload)
            .await
            .unwrap_err();
        assert!(matches!(
            error.kind(),
            ErrorKind::CustomField(crate::CustomFieldError::TypeMismatch { .. })
        ));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_request(&server.next_request(), "POST", "/equipment");
        assert_request(&server.next_request(), "GET", "/equipment/7");

        let error = client
            .equipment()
            .create_as::<EquipmentName>(&equipment_payload)
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_request(&server.next_request(), "POST", "/equipment");
        assert_request(&server.next_request(), "GET", "/equipment/9");

        let status_payload = TaskStatusPayload::new("green".to_string());
        let error = client
            .task_statuses()
            .create_as::<StatusColor>(&status_payload)
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_request(&server.next_request(), "POST", "/taskstatuses");

        let error = client
            .equipment()
            .update_as::<EquipmentName>(crate::model::EquipmentId::new(8), &equipment_payload)
            .await
            .unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        assert_eq!(error.operation(), Some("updateEquipment"));
        assert_request(&server.next_request(), "PUT", "/equipment/8");

        let error = client
            .equipment()
            .update_as::<EquipmentName>(crate::model::EquipmentId::new(8), &equipment_payload)
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_request(&server.next_request(), "PUT", "/equipment/8");

        let error = client
            .task_statuses()
            .update_as::<StatusColor>(TaskStatusId::new(5), &status_payload)
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_request(&server.next_request(), "PUT", "/taskstatuses/5");

        let updated = client
            .task_statuses()
            .update_as::<StatusColor>(TaskStatusId::new(5), &status_payload)
            .await
            .unwrap();
        assert_eq!(updated.color, "green");
        assert_request(&server.next_request(), "PUT", "/taskstatuses/5");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn list_methods_reject_missing_registered_custom_fields() {
        use crate::{CustomFieldDefinition, CustomFieldRegistry, model::ExtraInputFieldType};

        let server = spawn_server(vec![
            json_response(
                r#"{"data":[{"id":1,"updateHash":"h1","created":null,"modified":null,"creator":null,"name":"Mixer"}],"itemCount":1,"limit":300}"#,
            ),
            json_response(
                r#"{"data":[{"id":1,"updateHash":"h1","created":null,"modified":null,"creator":null,"name":"Mixer"}],"itemCount":1,"limit":1}"#,
            ),
            json_response(
                r#"{"data":[{"id":1,"updateHash":"h1","name":"Mixer"}],"itemCount":1,"limit":300}"#,
            ),
        ]);
        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<Equipment>(
            8,
            ExtraInputFieldType::Text,
        )])
        .unwrap();
        let client = client(&server).with_custom_fields(registry);

        let error = client.equipment().list().all().await.unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::CustomField(_)));
        server.next_request();
        let error = client.equipment().list().first(1).await.unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::CustomField(_)));
        server.next_request();
        let error = client
            .equipment()
            .list()
            .all_as::<EquipmentName>()
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::CustomField(_)));
        server.next_request();
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn creates_as_uses_the_write_response_when_projection_is_complete() {
        let server = spawn_server(vec![item(TASK_STATUS)]);
        let client = client(&server);
        let payload = TaskStatusPayload::new("green".to_string());

        let status = client
            .task_statuses()
            .create_as::<StatusColor>(&payload)
            .await
            .unwrap();

        assert_eq!(status.id, TaskStatusId::new(5));
        assert_eq!(status.color, "green");
        assert_request(&server.next_request(), "POST", "/taskstatuses");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn creates_as_fetches_missing_projection_fields_once() {
        let partial = json_response(
            r#"{"data":{"id":5,"updateHash":"hash-5","created":null,"modified":null,"creator":"/crew/1","displayname":"Done","name":"Done","type":"done","order":2}}"#,
        );
        let server = spawn_server(vec![
            partial.clone(),
            item(TASK_STATUS),
            partial,
            text_response("HTTP/1.1 404 Not Found", "gone"),
        ]);
        let client = client(&server);
        let payload = TaskStatusPayload::new("green".to_string());

        let status = client
            .task_statuses()
            .create_as::<StatusColor>(&payload)
            .await
            .unwrap();

        assert_eq!(status.id, TaskStatusId::new(5));
        assert_eq!(status.color, "green");
        let create = server.next_request();
        assert_request(&create, "POST", "/taskstatuses");
        let fetch = server.next_request();
        assert_request(&fetch, "GET", "/taskstatuses/5");
        assert_eq!(
            fetch.query_value("fields").as_deref(),
            Some("id,color,type")
        );

        let error = client
            .task_statuses()
            .create_as::<StatusColor>(&payload)
            .await
            .unwrap_err();
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_eq!(error.operation(), Some("createTaskStatus"));
        assert_eq!(
            error.resource(),
            Some(ResourceRef::from(TaskStatusId::new(5)))
        );
        server.next_request();
        server.next_request();
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn updates_as_fetches_missing_projection_fields_once() {
        let partial = json_response(
            r#"{"data":{"id":5,"updateHash":"hash-5","created":null,"modified":null,"creator":"/crew/1","displayname":"Done","name":"Done","type":"done","order":2}}"#,
        );
        let server = spawn_server(vec![partial, item(TASK_STATUS)]);
        let client = client(&server);
        let payload = TaskStatusPayload::new("green".to_string());

        let status = client
            .task_statuses()
            .update_as::<StatusColor>(TaskStatusId::new(5), &payload)
            .await
            .unwrap();

        assert_eq!(status.id, TaskStatusId::new(5));
        assert_eq!(status.color, "green");
        assert_request(&server.next_request(), "PUT", "/taskstatuses/5");
        let fetch = server.next_request();
        assert_request(&fetch, "GET", "/taskstatuses/5");
        assert_eq!(
            fetch.query_value("fields").as_deref(),
            Some("id,color,type")
        );
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn update_reports_applied_when_the_fallback_get_fails() {
        let partial = item(
            r#"{"id":5,"updateHash":"hash-5","created":null,"modified":null,"creator":"/crew/1"}"#,
        );
        let server = spawn_server(vec![
            partial,
            text_response("HTTP/1.1 503 Service Unavailable", "later"),
            text_response("HTTP/1.1 503 Service Unavailable", "later"),
        ]);
        let client = client(&server);
        let payload = TaskStatusPayload::new("green".to_string());

        let error = client
            .task_statuses()
            .update(TaskStatusId::new(5), &payload)
            .await
            .unwrap_err();

        assert_eq!(error.status(), Some(StatusCode::SERVICE_UNAVAILABLE));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_eq!(error.operation(), Some("updateTaskStatus"));
        assert_eq!(
            error.resource(),
            Some(ResourceRef::from(TaskStatusId::new(5)))
        );
        assert_request(&server.next_request(), "PUT", "/taskstatuses/5");
        assert_request(&server.next_request(), "GET", "/taskstatuses/5");
        assert_request(&server.next_request(), "GET", "/taskstatuses/5");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn retries_updates_and_deletes() {
        let server = spawn_server(vec![
            text_response("HTTP/1.1 503 Service Unavailable", "later"),
            item(TASK_STATUS),
            text_response("HTTP/1.1 503 Service Unavailable", "later"),
            empty_response("HTTP/1.1 204 No Content"),
            text_response("HTTP/1.1 400 Bad Request", "broken"),
            text_response("HTTP/1.1 400 Bad Request", "broken"),
        ]);
        let client = client(&server);
        let payload = TaskStatusPayload {
            name: Some("Done".to_string()),
            ..TaskStatusPayload::new("green".to_string())
        };

        let updated = client
            .task_statuses()
            .update(TaskStatusId::new(5), &payload)
            .await
            .unwrap();
        assert_eq!(updated.color.as_deref(), Some("green"));
        for _ in 0..2 {
            let request = server.next_request();
            assert_request(&request, "PUT", "/taskstatuses/5");
            assert_eq!(
                request.json_body(),
                serde_json::json!({"name": "Done", "color": "green"})
            );
        }

        client
            .task_statuses()
            .delete(TaskStatusId::new(5))
            .await
            .unwrap();
        for _ in 0..2 {
            assert_request(&server.next_request(), "DELETE", "/taskstatuses/5");
        }

        let error = client
            .task_statuses()
            .update(TaskStatusId::new(5), &payload)
            .await
            .unwrap_err();
        assert_eq!(error.operation(), Some("updateTaskStatus"));
        assert_eq!(
            error.resource(),
            Some(ResourceRef::from(TaskStatusId::new(5)))
        );
        server.next_request();

        let error = client
            .task_statuses()
            .delete(TaskStatusId::new(5))
            .await
            .unwrap_err();
        assert_eq!(error.operation(), Some("deleteTaskStatus"));
        assert_eq!(
            error.resource(),
            Some(ResourceRef::from(TaskStatusId::new(5)))
        );
        server.next_request();
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn update_marks_all_success_response_decode_failures_applied() {
        let invalid_value = TASK_STATUS.replace(r#""color":"green""#, r#""color":1"#);
        let server = spawn_server(vec![
            item(&invalid_value),
            item(r#"{"id":7,"updateHash":"h7","created":null,"modified":null,"creator":null}"#),
            item(
                r#"{"id":7,"updateHash":"h7","created":null,"modified":null,"creator":null,"custom":"invalid"}"#,
            ),
            item(r#"{"id":8,"updateHash":"h8","created":null,"modified":null,"creator":null}"#),
            item(r#"{"id":8,"updateHash":"h8","created":null,"modified":null,"creator":null}"#),
        ]);
        let client = client(&server);
        let status_payload = TaskStatusPayload::new("green".to_string());

        let decode = client
            .task_statuses()
            .update(TaskStatusId::new(5), &status_payload)
            .await
            .unwrap_err();
        assert!(matches!(decode.kind(), ErrorKind::Decode(_)));
        assert_eq!(decode.outcome(), RequestOutcome::Applied);
        server.next_request();

        let registry =
            crate::CustomFieldRegistry::new([crate::CustomFieldDefinition::new::<Equipment>(
                8,
                crate::model::ExtraInputFieldType::Text,
            )])
            .unwrap();
        let client = client.with_custom_fields(registry);
        let payload = crate::model::EquipmentPayload::default();
        let malformed = client
            .equipment()
            .update(crate::model::EquipmentId::new(7), &payload)
            .await
            .unwrap_err();
        assert!(matches!(malformed.kind(), ErrorKind::Decode(_)));
        assert_eq!(malformed.outcome(), RequestOutcome::Applied);
        server.next_request();
        server.next_request();

        let invalid_registry_target = client
            .equipment()
            .update(crate::model::EquipmentId::new(8), &payload)
            .await
            .unwrap_err();
        assert!(matches!(
            invalid_registry_target.kind(),
            ErrorKind::CustomField(crate::CustomFieldError::MissingValue { id: 8, .. })
        ));
        assert_eq!(invalid_registry_target.outcome(), RequestOutcome::Applied);
        server.next_request();
        server.next_request();
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn update_complete_response_covers_deserialization_and_registry_errors() {
        use crate::{CustomFieldDefinition, CustomFieldRegistry, model::ExtraInputFieldType};

        let mut invalid_price = complete_equipment_response(1, serde_json::json!(null));
        invalid_price["price"] = serde_json::json!("not a number");
        let wrong_custom_type =
            complete_equipment_response(2, serde_json::json!({"custom_8": true}));
        let malformed_custom = complete_equipment_response(3, serde_json::json!("invalid"));
        let server = spawn_server(vec![
            item_value(invalid_price),
            item_value(wrong_custom_type),
            item_value(malformed_custom),
        ]);
        let payload = crate::model::EquipmentPayload::default();
        let client = client(&server);

        let error = client
            .equipment()
            .update(EquipmentId::new(1), &payload)
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_request(&server.next_request(), "PUT", "/equipment/1");

        let registry = CustomFieldRegistry::new([CustomFieldDefinition::new::<Equipment>(
            8,
            ExtraInputFieldType::Text,
        )])
        .unwrap();
        let client = client.with_custom_fields(registry);
        let error = client
            .equipment()
            .update(EquipmentId::new(2), &payload)
            .await
            .unwrap_err();
        assert!(matches!(
            error.kind(),
            ErrorKind::CustomField(crate::CustomFieldError::TypeMismatch { .. })
        ));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_request(&server.next_request(), "PUT", "/equipment/2");

        let error = client
            .equipment()
            .update(EquipmentId::new(3), &payload)
            .await
            .unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_request(&server.next_request(), "PUT", "/equipment/3");
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn runs_actions_exactly_once() {
        let server = spawn_server(vec![
            item(r#"{"path":"/projects/7"}"#),
            text_response("HTTP/1.1 503 Service Unavailable", "later"),
            item("{}"),
        ]);
        let client = client(&server);
        let payload = ProjectsSetTypePayload::new(crate::Id::new(3));

        let reference = client
            .projects()
            .set_type(ProjectId::new(7), &payload)
            .await
            .unwrap();
        assert_eq!(reference.path, Some(ProjectId::new(7)));
        let request = server.next_request();
        assert_request(&request, "POST", "/projects/7/:setType");
        assert_eq!(request.json_body(), serde_json::json!({"type": 3}));

        let error = client
            .projects()
            .set_type(ProjectId::new(7), &payload)
            .await
            .unwrap_err();
        assert_eq!(error.outcome(), RequestOutcome::Unknown);
        assert_eq!(error.operation(), Some("setType"));
        assert_eq!(error.resource(), Some(ResourceRef::from(ProjectId::new(7))));
        server.next_request();
        server.assert_no_more_requests();
    }

    // llvm-cov counts a generic function as covered only if one instantiation
    // runs every branch, so these exercise the paths `Equipment` missed.
    #[tokio::test]
    async fn equipment_requests_report_http_and_page_size_errors() {
        let server = spawn_server(vec![
            text_response("HTTP/1.1 400 Bad Request", "rejected"),
            text_response("HTTP/1.1 400 Bad Request", "rejected"),
            text_response("HTTP/1.1 400 Bad Request", "rejected"),
            text_response("HTTP/1.1 400 Bad Request", "rejected"),
        ]);
        let client = client(&server);
        let id = EquipmentId::new(5);
        let payload = crate::model::EquipmentPayload::default();

        let error = client.equipment().get(id).await.unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        assert_eq!(error.operation(), Some("getEquipmentItem"));
        assert_eq!(error.resource(), Some(ResourceRef::from(id)));
        assert_request(&server.next_request(), "GET", "/equipment/5");

        let error = client.equipment().create(&payload).await.unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        assert_eq!(error.operation(), Some("createEquipment"));
        assert_request(&server.next_request(), "POST", "/equipment");

        let error = client.equipment().update(id, &payload).await.unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        assert_eq!(error.operation(), Some("updateEquipment"));
        assert_eq!(error.resource(), Some(ResourceRef::from(id)));
        assert_request(&server.next_request(), "PUT", "/equipment/5");

        let error = client.equipment().list().all().await.unwrap_err();
        assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
        assert_eq!(error.operation(), Some("getEquipmentCollection"));
        assert_request(&server.next_request(), "GET", "/equipment");

        let error = client.equipment().list().first(0).await.unwrap_err();
        assert!(matches!(
            error.kind(),
            ErrorKind::InvalidRequest(InvalidRequest::PageSize { requested: 0 })
        ));
        assert_eq!(error.operation(), Some("getEquipmentCollection"));
        server.assert_no_more_requests();
    }

    #[tokio::test]
    async fn equipment_reads_and_writes_decode_complete_and_fallback_responses() {
        let complete = complete_equipment_response(6, serde_json::Value::Null);
        let server = spawn_server(vec![
            item(r#"{"id":6,"updateHash":"h6"}"#),
            item_value(complete.clone()),
            item(r#"{"updateHash":"h7"}"#),
            item(r#"{"id":5,"updateHash":"h5","name":"Mixer"}"#),
            item(r#"{"id":5,"updateHash":"h5","name":"Desk"}"#),
            json_response(
                &serde_json::json!({"data":[complete],"itemCount":1,"limit":1}).to_string(),
            ),
            json_response(r#"{"data":[{"id":5,"name":"Mixer"}],"itemCount":1,"limit":300}"#),
        ]);
        let client = client(&server);
        let payload = crate::model::EquipmentPayload::default();

        let created = client.equipment().create(&payload).await.unwrap();
        assert_eq!(created.id, EquipmentId::new(6));
        assert_request(&server.next_request(), "POST", "/equipment");
        assert_request(&server.next_request(), "GET", "/equipment/6");

        let error = client.equipment().create(&payload).await.unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Decode(_)));
        assert_eq!(error.outcome(), RequestOutcome::Applied);
        assert_eq!(error.operation(), Some("createEquipment"));
        assert_request(&server.next_request(), "POST", "/equipment");

        let created = client
            .equipment()
            .create_as::<EquipmentName>(&payload)
            .await
            .unwrap();
        assert_eq!(created.id, EquipmentId::new(5));
        assert_eq!(created.name, "Mixer");
        assert_request(&server.next_request(), "POST", "/equipment");

        let updated = client
            .equipment()
            .update_as::<EquipmentName>(EquipmentId::new(5), &payload)
            .await
            .unwrap();
        assert_eq!(updated.name, "Desk");
        assert_request(&server.next_request(), "PUT", "/equipment/5");

        let first = client.equipment().list().first(1).await.unwrap();
        assert_eq!(first.len(), 1);
        assert_eq!(first[0].id, EquipmentId::new(6));
        let request = server.next_request();
        assert_request(&request, "GET", "/equipment");
        assert_eq!(request.query_value("limit").as_deref(), Some("1"));

        let names = client
            .equipment()
            .list()
            .all_as::<EquipmentName>()
            .await
            .unwrap();
        assert_eq!(names.len(), 1);
        assert_eq!(names[0].name, "Mixer");
        let request = server.next_request();
        assert_request(&request, "GET", "/equipment");
        assert_eq!(request.query_value("fields").as_deref(), Some("id,name"));
        server.assert_no_more_requests();
    }

    #[test]
    fn describes_collections_and_requests() {
        let client = RentmanClient::new(RentmanApiToken::new("token"));
        let nested = client.crew().appointments(crate::Id::new(3));
        assert_eq!(
            format!("{nested:?}"),
            "Collection { collection: \"rentman_client::resource::generated::CrewAppointments\", parent: Some(3) }"
        );
        let request = nested.list().page_size(PageSize::MAX);
        assert_eq!(
            format!("{request:?}"),
            "ListRequest { path: \"/crew/3/appointments\", page_size: PageSize(1500), query: [], sort: [] }"
        );
        assert_eq!(
            <TaskStatus as List>::LIST.operation_id(),
            "getTaskStatusCollection"
        );
    }
}
