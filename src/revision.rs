//! Typed revisions for detecting changes between Rentman reads.

use crate::{
    id::{Id, Resource},
    projection::{ProjectedItem, Projection},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::{BTreeMap, btree_map},
    fmt,
    hash::{self, Hash},
    marker::PhantomData,
};

/// A revision cannot be constructed because its text is empty.
///
/// ```
/// use rentman_client::{InvalidRevision, Revision, model::Crew};
///
/// assert_eq!(Revision::<Crew>::new("").unwrap_err(), InvalidRevision);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("a Rentman revision (updateHash) must not be empty")]
pub struct InvalidRevision;

/// Revision of an item of resource `R`.
///
/// Every full resource model includes both `id: Id<R>` and
/// `revision: Revision<R>`. Its data and this pair form a read snapshot:
/// the ID identifies the item, and the revision identifies its observed
/// version. No separate revision request is needed when reading a full item.
///
/// Compare revisions for equality only after checking that the IDs match.
/// They are opaque, non-empty values, not ordered counters, timestamps, or
/// globally unique item identifiers. Local field edits do not update the
/// revision, and the client does not use it as an automatic write precondition.
///
/// Rentman sends this value as `updateHash`; full models and
/// [`projection!`](crate::projection) structs expose it as `revision`. A
/// projection includes it only when `revision` is selected. Bare IDs and
/// write payloads are not revisioned snapshots.
///
/// Store the revision with the object to reuse unchanged data or build a
/// [`Revisions`] index for synchronization. See the
/// [objects and revisions guide](crate#change-detection) for complete examples.
///
/// ```
/// # async fn example(client: rentman_client::RentmanClient, stored: rentman_client::model::Crew) -> rentman_client::Result<()> {
/// // `stored` was read earlier and kept together with its revision.
/// let current = client.crew().get(stored.id).await?;
/// if current.revision == stored.revision {
///     println!("crew member {} is unchanged", stored.id);
/// }
/// # Ok(())
/// # }
/// ```
pub struct Revision<R> {
    value: String,
    resource: PhantomData<fn() -> R>,
}

impl<R> Revision<R> {
    /// Creates a revision from its text, e.g. one stored after an earlier read.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use rentman_client::{Revision, model::Crew};
    ///
    /// // E.g. restored from your database.
    /// let revision: Revision<Crew> = Revision::new("a1b2c3")?;
    /// assert_eq!(revision.as_str(), "a1b2c3");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidRevision> {
        let value = value.into();
        if value.is_empty() {
            return Err(InvalidRevision);
        }
        Ok(Self {
            value,
            resource: PhantomData,
        })
    }

    /// Returns the revision text exactly as Rentman sent it.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use rentman_client::{Revision, model::Crew};
    ///
    /// let revision = Revision::<Crew>::new("a1b2c3")?;
    /// let to_store: String = revision.as_str().to_owned();
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl<R> Clone for Revision<R> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            resource: PhantomData,
        }
    }
}

impl<R> PartialEq for Revision<R> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<R> Eq for Revision<R> {}

impl<R> Hash for Revision<R> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<R> fmt::Debug for Revision<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Revision").field(&self.value).finish()
    }
}

impl<R> fmt::Display for Revision<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.value)
    }
}

impl<R> From<Revision<R>> for String {
    fn from(revision: Revision<R>) -> Self {
        revision.value
    }
}

impl<R> Serialize for Revision<R> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.value)
    }
}

impl<'de, R> Deserialize<'de> for Revision<R> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Resource item that exposes its ID and revision for generic synchronization.
///
/// Implemented by every full generated resource model. Both methods describe
/// the same read snapshot; this trait does not fetch data. Custom projections
/// do not implement it automatically. See [`Revision`] and the
/// [synchronization guide](crate#change-detection).
///
/// One generic function serves every resource:
///
/// ```
/// use rentman_client::Revisioned;
///
/// fn changed<T: Revisioned>(stored: &T, current: &T) -> bool {
///     stored.id() == current.id() && stored.revision() != current.revision()
/// }
/// ```
pub trait Revisioned: Resource + Sized {
    /// ID of this item.
    fn id(&self) -> Id<Self>;
    /// Revision as returned by Rentman.
    fn revision(&self) -> &Revision<Self>;
}

/// Revisions of all items in one resource listing, keyed by item ID.
///
/// Full objects already contain their revisions: collect their references
/// into this index, or call [`ListRequest::revisions`](crate::resource::ListRequest::revisions)
/// to read an ID/revision listing without full bodies. Persist the previous
/// index and compare equivalent listings with [`Self::changes_since`].
///
/// ```
/// use rentman_client::{Revisions, model::Crew};
///
/// fn index(stored: &[Crew]) -> Revisions<Crew> {
///     stored.iter().collect()
/// }
/// ```
pub struct Revisions<R> {
    items: BTreeMap<Id<R>, Revision<R>>,
}

impl<R> Revisions<R> {
    /// Creates an empty revision set.
    ///
    /// ```
    /// use rentman_client::{Revisions, model::Crew};
    ///
    /// let first_sync: Revisions<Crew> = Revisions::new();
    /// assert!(first_sync.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    /// Inserts a revision, returning the revision it replaces.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use rentman_client::{Revision, Revisions, model::CrewId};
    ///
    /// let mut known = Revisions::new();
    /// assert!(known.insert(CrewId::new(12), Revision::new("a")?).is_none());
    /// let replaced = known.insert(CrewId::new(12), Revision::new("b")?);
    /// assert_eq!(replaced.unwrap().as_str(), "a");
    /// # Ok(())
    /// # }
    /// ```
    pub fn insert(&mut self, id: Id<R>, revision: Revision<R>) -> Option<Revision<R>> {
        self.items.insert(id, revision)
    }

    /// Gets the revision of one item.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use rentman_client::{Revision, Revisions, model::CrewId};
    ///
    /// let mut known = Revisions::new();
    /// known.insert(CrewId::new(12), Revision::new("a")?);
    /// assert_eq!(known.get(CrewId::new(12)).map(Revision::as_str), Some("a"));
    /// assert!(known.get(CrewId::new(13)).is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(&self, id: Id<R>) -> Option<&Revision<R>> {
        self.items.get(&id)
    }

    /// Number of items in the set.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use rentman_client::{Revision, Revisions, model::CrewId};
    ///
    /// let mut known = Revisions::new();
    /// known.insert(CrewId::new(12), Revision::new("a")?);
    /// assert_eq!(known.len(), 1);
    /// # Ok(())
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether the set is empty.
    ///
    /// ```
    /// assert!(rentman_client::Revisions::<rentman_client::model::Crew>::new().is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Iterates over item IDs and revisions in ID order.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use rentman_client::{Revision, Revisions, model::CrewId};
    ///
    /// let mut known = Revisions::new();
    /// known.insert(CrewId::new(12), Revision::new("a")?);
    /// for (id, revision) in known.iter() {
    ///     println!("{id}: {revision}"); // store these pairs for the next sync
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (Id<R>, &Revision<R>)> {
        self.items.iter().map(|(id, revision)| (*id, revision))
    }

    /// Compares this listing with a prior one; both must use the same filters.
    ///
    /// New IDs are `created`, changed revisions of known IDs are `updated`,
    /// and missing IDs are `deleted`. Missing items may have stopped matching
    /// a filter rather than been deleted from Rentman. Both indexes must cover
    /// the same account and complete listing scope; a partial list gives
    /// misleading deletions. This compares observed states, not a history of
    /// intermediate edits or an atomic snapshot across paginated reads.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use rentman_client::{Revision, Revisions, model::CrewId};
    ///
    /// let mut before = Revisions::new();
    /// before.insert(CrewId::new(1), Revision::new("a")?);
    /// before.insert(CrewId::new(2), Revision::new("b")?);
    /// let mut now = Revisions::new();
    /// now.insert(CrewId::new(1), Revision::new("a2")?);
    /// now.insert(CrewId::new(3), Revision::new("c")?);
    ///
    /// let changes = now.changes_since(&before);
    /// assert_eq!(changes.created, [CrewId::new(3)]);
    /// assert_eq!(changes.updated, [CrewId::new(1)]);
    /// assert_eq!(changes.deleted, [CrewId::new(2)]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn changes_since(&self, known: &Self) -> Changes<R> {
        let mut changes = Changes::default();
        for (id, revision) in &self.items {
            match known.items.get(id) {
                None => changes.created.push(*id),
                Some(previous) if previous != revision => changes.updated.push(*id),
                Some(_) => {}
            }
        }
        changes.deleted = known
            .items
            .keys()
            .filter(|id| !self.items.contains_key(id))
            .copied()
            .collect();
        changes
    }
}

impl<R> Default for Revisions<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> Clone for Revisions<R> {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
        }
    }
}

impl<R> PartialEq for Revisions<R> {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl<R> Eq for Revisions<R> {}

impl<R: Resource> fmt::Debug for Revisions<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.items.iter()).finish()
    }
}

impl<R> FromIterator<(Id<R>, Revision<R>)> for Revisions<R> {
    fn from_iter<I: IntoIterator<Item = (Id<R>, Revision<R>)>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

impl<'a, R: Revisioned> FromIterator<&'a R> for Revisions<R> {
    fn from_iter<I: IntoIterator<Item = &'a R>>(iter: I) -> Self {
        iter.into_iter()
            .map(|item| (item.id(), item.revision().clone()))
            .collect()
    }
}

impl<R> IntoIterator for Revisions<R> {
    type Item = (Id<R>, Revision<R>);
    type IntoIter = btree_map::IntoIter<Id<R>, Revision<R>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

/// Created, updated, and deleted IDs between two revision sets, returned by
/// [`Revisions::changes_since`].
///
/// ```
/// # async fn example(client: rentman_client::RentmanClient, known: rentman_client::Revisions<rentman_client::model::Crew>) -> rentman_client::Result<()> {
/// let changes = client.crew().list().revisions().await?.changes_since(&known);
/// for id in changes.created.iter().chain(&changes.updated) {
///     let member = client.crew().get(*id).await?; // fetch only what changed
///     # let _ = member;
/// }
/// for id in &changes.deleted {
///     println!("crew member {id} left the listing");
/// }
/// # Ok(())
/// # }
/// ```
pub struct Changes<R> {
    /// Items present now but not previously.
    pub created: Vec<Id<R>>,
    /// Items whose revision changed.
    pub updated: Vec<Id<R>>,
    /// Items absent now; they may be deleted or no longer match filters.
    pub deleted: Vec<Id<R>>,
}

impl<R> Changes<R> {
    /// Whether no IDs changed.
    ///
    /// ```
    /// use rentman_client::{Revisions, model::Crew};
    ///
    /// let same: Revisions<Crew> = Revisions::new();
    /// assert!(same.changes_since(&Revisions::new()).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.created.is_empty() && self.updated.is_empty() && self.deleted.is_empty()
    }
}

impl<R> Default for Changes<R> {
    fn default() -> Self {
        Self {
            created: Vec::new(),
            updated: Vec::new(),
            deleted: Vec::new(),
        }
    }
}

impl<R> Clone for Changes<R> {
    fn clone(&self) -> Self {
        Self {
            created: self.created.clone(),
            updated: self.updated.clone(),
            deleted: self.deleted.clone(),
        }
    }
}

impl<R> PartialEq for Changes<R> {
    fn eq(&self, other: &Self) -> bool {
        self.created == other.created
            && self.updated == other.updated
            && self.deleted == other.deleted
    }
}

impl<R> Eq for Changes<R> {}

impl<R: Resource> fmt::Debug for Changes<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Changes")
            .field("created", &self.created)
            .field("updated", &self.updated)
            .field("deleted", &self.deleted)
            .finish()
    }
}

/// ID and revision read from a projected item.
pub(crate) struct ItemRevision<R> {
    pub(crate) id: Id<R>,
    pub(crate) revision: Revision<R>,
}

impl<R: Resource> Projection for ItemRevision<R> {
    type Resource = R;
    const FIELDS: &'static [&'static str] = &["id"];

    fn decode(item: &mut ProjectedItem) -> Result<Self, serde_json::Error> {
        Ok(Self {
            id: item.take_named("id")?,
            revision: item.take_named("updateHash")?,
        })
    }
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashSet;

    struct Crew;
    impl Resource for Crew {
        const COLLECTION: &'static str = "crew";
    }

    struct Item {
        id: Id<Item>,
        revision: Revision<Item>,
    }
    impl Resource for Item {
        const COLLECTION: &'static str = "items";
    }
    impl Revisioned for Item {
        fn id(&self) -> Id<Self> {
            self.id
        }
        fn revision(&self) -> &Revision<Self> {
            &self.revision
        }
    }

    fn revision(value: &str) -> Revision<Crew> {
        Revision::new(value).unwrap()
    }
    fn revisions(values: &[(u64, &str)]) -> Revisions<Crew> {
        values
            .iter()
            .map(|(id, value)| (Id::new(*id), revision(value)))
            .collect()
    }

    #[test]
    fn revision_is_opaque_nonempty_and_json_string() {
        let first = revision("d21b3fd9");
        assert_eq!(first.as_str(), "d21b3fd9");
        assert_eq!(first.to_string(), "d21b3fd9");
        assert_eq!(format!("{first:?}"), "Revision(\"d21b3fd9\")");
        assert_eq!(String::from(first.clone()), "d21b3fd9");
        assert_eq!(serde_json::to_value(&first).unwrap(), json!("d21b3fd9"));
        assert_eq!(
            serde_json::from_value::<Revision<Crew>>(json!("x")).unwrap(),
            revision("x")
        );
        assert_eq!(Revision::<Crew>::new(""), Err(InvalidRevision));
        assert!(serde_json::from_value::<Revision<Crew>>(json!("")).is_err());
        assert!(serde_json::from_value::<Revision<Crew>>(json!(42)).is_err());
        assert!(serde_json::from_value::<Revision<Crew>>(json!(null)).is_err());
        assert_ne!(first, revision("other"));
        let values = [revision("x"), revision("x"), revision("y")]
            .into_iter()
            .collect::<HashSet<_>>();
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn revision_sets_and_change_detection_cover_all_outcomes() {
        let known = revisions(&[(1, "a1"), (2, "b1"), (3, "c1")]);
        let current = revisions(&[(1, "a1"), (2, "b2"), (4, "d1")]);
        let changes = current.changes_since(&known);
        assert_eq!(changes.created, [Id::new(4)]);
        assert_eq!(changes.updated, [Id::new(2)]);
        assert_eq!(changes.deleted, [Id::new(3)]);
        assert!(!changes.is_empty());
        assert_eq!(Revisions::<Crew>::default(), Revisions::new());
        assert!(known.changes_since(&known).is_empty());
        assert_eq!(known.changes_since(&Revisions::new()).created.len(), 3);
        assert_eq!(Revisions::new().changes_since(&known).deleted.len(), 3);
        assert_eq!(changes.clone(), changes);

        let mut values = Revisions::new();
        assert!(values.is_empty());
        assert_eq!(values.insert(Id::new(2), revision("b1")), None);
        assert_eq!(
            values.insert(Id::new(2), revision("b2")),
            Some(revision("b1"))
        );
        assert_eq!(values.get(Id::new(2)), Some(&revision("b2")));
        assert_eq!(values.get(Id::new(1)), None);
        assert_eq!(values.len(), 1);
        assert!(!values.is_empty());
        assert_eq!(
            values
                .iter()
                .map(|(id, value)| (id.get(), value.as_str()))
                .collect::<Vec<_>>(),
            [(2, "b2")]
        );
        assert_eq!(
            values.clone(),
            [(Id::new(2), revision("b2"))].into_iter().collect()
        );
        assert_eq!(
            values
                .into_iter()
                .map(|(id, value)| (id.get(), String::from(value)))
                .collect::<Vec<_>>(),
            [(2, "b2".to_string())]
        );
        assert_eq!(
            format!("{:?}", changes),
            "Changes { created: [Id(/crew/4)], updated: [Id(/crew/2)], deleted: [Id(/crew/3)] }"
        );
        assert!(format!("{:?}", revisions(&[(1, "a")])).contains("Id(/crew/1)"));
    }

    #[test]
    fn collects_revisioned_items_and_decodes_projection_revision() {
        let items = [
            Item {
                id: Id::new(2),
                revision: Revision::new("b").unwrap(),
            },
            Item {
                id: Id::new(1),
                revision: Revision::new("a").unwrap(),
            },
        ];
        let collected = items.iter().collect::<Revisions<Item>>();
        assert_eq!(collected.len(), 2);

        let mut item: ProjectedItem =
            serde_json::from_value(json!({"id": 7, "updateHash": "h7"})).unwrap();
        let revision = ItemRevision::<Crew>::decode(&mut item).unwrap();
        assert_eq!(revision.id, Id::new(7));
        assert_eq!(revision.revision, self::revision("h7"));
        assert_eq!(ItemRevision::<Crew>::FIELDS, ["id"]);
        for value in [json!({"id": 7}), json!({"id": 7, "updateHash": ""})] {
            let mut item: ProjectedItem = serde_json::from_value(value).unwrap();
            assert!(ItemRevision::<Crew>::decode(&mut item).is_err());
        }
    }
}
