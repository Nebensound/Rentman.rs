//! The resource layer: list, read and write Rentman data.
//!
//! Every documented collection is reachable from [`RentmanClient`], e.g.
//! `client.crew()` or, nested, `client.crew().appointments(crew_id)`. A
//! [`Collection`] only offers the operations Rentman documents for it: the
//! methods come from the [`List`], [`Get`], [`Create`], [`Update`] and
//! [`Delete`] traits, which the generated code implements per collection.

mod generated;

pub use generated::*;

use crate::{
    RentmanClient,
    endpoint::EndpointSpec,
    error::Result,
    id::{Id, Resource},
    model::ItemResponse,
    paging::{CollectionRequest, PageSize},
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
    type Item: DeserializeOwned;
    #[doc(hidden)]
    const LIST: Operation;
}

/// A collection whose items can be read one by one.
pub trait Get: sealed::Sealed {
    /// Item type of the collection.
    type Item: Resource + DeserializeOwned;
    #[doc(hidden)]
    const GET: Operation;
}

/// A collection that items can be created in.
pub trait Create: sealed::Sealed {
    /// Write payload of a new item.
    type Payload: Serialize;
    /// Item type Rentman returns for the created item.
    type Item: DeserializeOwned;
    #[doc(hidden)]
    const CREATE: Operation;
}

/// A collection whose items can be updated.
pub trait Update: sealed::Sealed {
    /// Write payload with the fields to change.
    type Payload: Serialize;
    /// Item type of the collection.
    type Item: Resource + DeserializeOwned;
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
            item: PhantomData,
        }
    }
}

impl<M: Get> Collection<M> {
    /// Reads one item. Transient failures are retried.
    pub async fn get(&self, id: Id<M::Item>) -> Result<M::Item> {
        let spec = M::GET.0;
        self.client
            .request(Method::GET, &render_path(spec, Some(id.get())))
            .send_json::<ItemResponse<M::Item>>()
            .await
            .map(|response| response.data)
            .map_err(|error| error.in_operation(spec.operation_id).about(id))
    }
}

impl<M: Create> Collection<M> {
    /// Creates an item and returns it as Rentman stored it.
    ///
    /// The request is sent exactly once, without retrying: when only the
    /// response gets lost, a retry would create the item twice. On an error,
    /// [`Error::outcome`](crate::Error::outcome) says whether it may have been applied; when it is
    /// [`RequestOutcome::Unknown`](crate::RequestOutcome::Unknown), read the
    /// collection with [`Self::list`] before creating the item again.
    pub async fn create(&self, payload: &M::Payload) -> Result<M::Item> {
        let spec = M::CREATE.0;
        let request = self
            .client
            .request(Method::POST, &render_path(spec, self.parent))
            .json(payload);
        let retry_policy = self.client.retry_policy.single_attempt();
        request
            .send_json_with::<ItemResponse<M::Item>>(retry_policy)
            .await
            .map(|response| response.data)
            .map_err(|error| error.in_operation(spec.operation_id))
    }
}

impl<M: Update> Collection<M> {
    /// Updates an item and returns it as Rentman stored it.
    ///
    /// Rentman leaves fields the payload does not send unchanged. `PUT` is
    /// idempotent, so transient failures are retried.
    pub async fn update(&self, id: Id<M::Item>, payload: &M::Payload) -> Result<M::Item> {
        let spec = M::UPDATE.0;
        self.client
            .request(Method::PUT, &render_path(spec, Some(id.get())))
            .json(payload)
            .send_json::<ItemResponse<M::Item>>()
            .await
            .map(|response| response.data)
            .map_err(|error| error.in_operation(spec.operation_id).about(id))
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
    item: PhantomData<fn() -> T>,
}

impl<T> fmt::Debug for ListRequest<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListRequest")
            .field("path", &self.path)
            .field("page_size", &self.page_size)
            .finish()
    }
}

impl<T: DeserializeOwned> ListRequest<T> {
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
        self.client
            .read_all(CollectionRequest {
                path: &self.path,
                query: &[],
                page_size: self.page_size,
                operation: self.operation,
            })
            .await
    }

    /// Reads at most `count` items (1 to 1500) with a single request, in
    /// Rentman's order or the requested sorting.
    pub async fn first(self, count: usize) -> Result<Vec<T>> {
        let limit = PageSize::new(count)
            .map_err(|error| error.in_operation(self.operation))?
            .get()
            .to_string();
        self.client
            .request(Method::GET, &self.path)
            .query(&[("limit", limit)])
            .send_json::<crate::model::CollectionResponse<T>>()
            .await
            .map(|page| page.data)
            .map_err(|error| error.in_operation(self.operation))
    }
}

/// Renders a documented path with the `{id}` of the item or parent item.
fn render_path(spec: EndpointSpec, id: Option<u64>) -> String {
    match id {
        Some(id) => spec.path.replace("{id}", &id.to_string()),
        None => spec.path.to_string(),
    }
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::test_support::*;
    use crate::{
        ErrorKind, InvalidRequest, RentmanApiToken, RentmanClientConfig, RequestOutcome,
        ResourceRef, Url,
        model::{
            ProjectId, ProjectsSetTypePayload, TaskStatus, TaskStatusId, TaskStatusPayload,
            TaskStatusType,
        },
    };
    use reqwest::StatusCode;
    use std::{num::NonZeroUsize, time::Duration};

    const TASK_STATUS: &str = r#"{"id":5,"created":null,"modified":null,"creator":"/crew/1","displayname":"Done","name":"Done","color":"green","type":"done","order":2}"#;

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
        assert_eq!(statuses[0].type_, TaskStatusType::Done);
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
        assert_eq!(status.name, "Done");
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
        assert_eq!(updated.color, "green");
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
            "ListRequest { path: \"/crew/3/appointments\", page_size: PageSize(1500) }"
        );
        assert_eq!(
            <TaskStatus as List>::LIST.operation_id(),
            "getTaskStatusCollection"
        );
    }
}
