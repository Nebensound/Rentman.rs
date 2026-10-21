//! Revisions: which state of an item was read, for change detection.

use crate::id::{Id, Resource};
use crate::projection::{ProjectedItem, Projection};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::BTreeMap, fmt, hash, marker::PhantomData};

/// Revision of an item of resource `R`: which state of the item was read.
///
/// Rentman sends it as `updateHash` on every item, also when `fields=`
/// selects only some fields: a hash of the item's `id` and `modified`
/// timestamp, unique per item and per update. A different revision of the
/// same item means it changed in between. Rentman does not document the
/// algorithm, so a revision is only compared, never interpreted; like
/// [`Id`], it belongs to one resource, so revisions of crew members and of
/// projects cannot be mixed up. Rentman cannot filter or sort by it, and it
/// is never sent in a write.
///
/// ```
/// use rentman_client::{Revision, model::Crew};
///
/// let stored = Revision::<Crew>::new("d21b3fd9d594582874d23f8d26234732")?;
/// let read = Revision::<Crew>::new("2b94ecc682536e7f05f84cb7bef57cb1")?;
/// assert_ne!(stored, read, "the crew member changed");
/// assert_eq!(stored.as_str(), "d21b3fd9d594582874d23f8d26234732");
/// # Ok::<(), rentman_client::InvalidRevision>(())
/// ```
pub struct Revision<R> {
    value: String,
    resource: PhantomData<fn() -> R>,
}

impl<R> Revision<R> {
    /// Creates a revision from its text, e.g. one stored after an earlier
    /// read.
    ///
    /// Fails for an empty text, which could not tell two states apart.
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

    /// Returns the revision as Rentman sent it, e.g. to store it.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

/// Why a text is not a [`Revision`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("a Rentman revision (updateHash) must not be empty")]
pub struct InvalidRevision;

// Manual impls: derives would require `R` itself to implement the traits.
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

impl<R> hash::Hash for Revision<R> {
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

/// Serializes the text Rentman sent.
impl<R> Serialize for Revision<R> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.value)
    }
}

/// Deserializes Rentman's `updateHash`; an empty text is rejected.
impl<'de, R> Deserialize<'de> for Revision<R> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// An item of a resource: every generated model, e.g. `model::Crew`.
///
/// Generic sync code reads the ID and revision of any resource through it.
pub trait Revisioned: Resource + Sized {
    /// ID of the item.
    fn id(&self) -> Id<Self>;
    /// Revision of the item as read.
    fn revision(&self) -> &Revision<Self>;
}

/// The revision of every item of a listing, by ID.
///
/// [`ListRequest::revisions`](crate::resource::ListRequest::revisions) reads
/// them without the items' fields; [`Revisions::changes_since`] compares two
/// of them. Revisions stored after an earlier sync are collected back from
/// `(Id, Revision)` pairs:
///
/// ```
/// use rentman_client::{Id, Revision, Revisions, model::Crew};
///
/// let stored = [(1, "a1"), (2, "b1")];
/// let known = stored
///     .into_iter()
///     .map(|(id, revision)| Ok((Id::new(id), Revision::new(revision)?)))
///     .collect::<Result<Revisions<Crew>, rentman_client::InvalidRevision>>()?;
/// assert_eq!(known.len(), 2);
/// # Ok::<(), rentman_client::InvalidRevision>(())
/// ```
pub struct Revisions<R> {
    items: BTreeMap<Id<R>, Revision<R>>,
}

impl<R> Revisions<R> {
    /// An empty set, e.g. before the first sync.
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    /// Records the revision of an item and returns the one it replaces.
    pub fn insert(&mut self, id: Id<R>, revision: Revision<R>) -> Option<Revision<R>> {
        self.items.insert(id, revision)
    }

    /// Revision of the item with this ID, if it is listed.
    pub fn get(&self, id: Id<R>) -> Option<&Revision<R>> {
        self.items.get(&id)
    }

    /// Number of listed items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether no item is listed.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Every item's ID and revision, ordered by ID.
    pub fn iter(&self) -> impl Iterator<Item = (Id<R>, &Revision<R>)> {
        self.items.iter().map(|(id, revision)| (*id, revision))
    }

    /// What changed since `known` was read: items that are new, items whose
    /// revision differs, and items that are gone, each ordered by ID.
    ///
    /// Both sides must come from the same listing with the same filters: an
    /// item that no longer matches a filter is reported as deleted.
    pub fn changes_since(&self, known: &Revisions<R>) -> Changes<R> {
        let mut changes = Changes::default();
        for (id, revision) in &self.items {
            match known.items.get(id) {
                None => changes.created.push(*id),
                Some(old) if old != revision => changes.updated.push(*id),
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

/// The revisions of items that were read in full.
impl<'a, R: Revisioned> FromIterator<&'a R> for Revisions<R> {
    fn from_iter<I: IntoIterator<Item = &'a R>>(iter: I) -> Self {
        iter.into_iter()
            .map(|item| (item.id(), item.revision().clone()))
            .collect()
    }
}

impl<R> IntoIterator for Revisions<R> {
    type Item = (Id<R>, Revision<R>);
    type IntoIter = std::collections::btree_map::IntoIter<Id<R>, Revision<R>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

/// What changed in a listing between two reads, see
/// [`Revisions::changes_since`].
pub struct Changes<R> {
    /// Items listed now but not before.
    pub created: Vec<Id<R>>,
    /// Items listed both times with a different revision.
    pub updated: Vec<Id<R>>,
    /// Items listed before but not now: deleted, or no longer matching the
    /// listing's filters.
    pub deleted: Vec<Id<R>>,
}

impl<R> Changes<R> {
    /// Whether nothing changed.
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

/// The ID and revision of a listed item: the projection
/// [`ListRequest::revisions`](crate::resource::ListRequest::revisions)
/// reads. It selects only `id`; Rentman adds `updateHash` by itself.
pub(crate) struct ItemRevision<R> {
    pub(crate) id: Id<R>,
    pub(crate) revision: Revision<R>,
}

impl<R> Projection for ItemRevision<R> {
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

    /// A full item, like a generated model.
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

    fn revision(text: &str) -> Revision<Crew> {
        Revision::new(text).unwrap()
    }

    fn revisions(items: &[(u64, &str)]) -> Revisions<Crew> {
        items
            .iter()
            .map(|(id, text)| (Id::new(*id), revision(text)))
            .collect()
    }

    #[test]
    fn keeps_the_text_exactly() {
        let hash = revision("d21b3fd9d594582874d23f8d26234732");
        assert_eq!(hash.as_str(), "d21b3fd9d594582874d23f8d26234732");
        assert_eq!(hash.to_string(), "d21b3fd9d594582874d23f8d26234732");
        assert_eq!(
            format!("{hash:?}"),
            r#"Revision("d21b3fd9d594582874d23f8d26234732")"#
        );
        assert_eq!(String::from(hash), "d21b3fd9d594582874d23f8d26234732");
        // Opaque: any non-empty text is kept, also one that is no hex digest.
        assert_eq!(revision("x").as_str(), "x");
    }

    #[test]
    fn rejects_an_empty_revision() {
        assert_eq!(Revision::<Crew>::new(""), Err(InvalidRevision));
        assert_eq!(
            InvalidRevision.to_string(),
            "a Rentman revision (updateHash) must not be empty"
        );
    }

    #[test]
    fn compares_and_hashes_by_value() {
        let a = revision("a");
        assert_eq!(a.clone(), a);
        assert_ne!(a, revision("b"));
        let set = [revision("a"), revision("a"), revision("b")]
            .into_iter()
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn reads_and_writes_a_json_string() {
        let read: Revision<Crew> = serde_json::from_value(json!("2b94ecc6")).unwrap();
        assert_eq!(read.as_str(), "2b94ecc6");
        assert_eq!(serde_json::to_value(&read).unwrap(), json!("2b94ecc6"));

        let error = serde_json::from_value::<Revision<Crew>>(json!("")).unwrap_err();
        assert!(error.is_data(), "{error}");
        assert!(serde_json::from_value::<Revision<Crew>>(json!(42)).is_err());
        assert!(serde_json::from_value::<Revision<Crew>>(json!(null)).is_err());
    }

    #[test]
    fn collects_and_reads_revisions_by_id() {
        let mut set = Revisions::<Crew>::new();
        assert!(set.is_empty());
        assert_eq!(set, Revisions::default());
        assert_eq!(set.insert(Id::new(2), revision("b1")), None);
        assert_eq!(set.insert(Id::new(1), revision("a1")), None);
        assert_eq!(set.insert(Id::new(2), revision("b2")), Some(revision("b1")));
        assert_eq!(set.len(), 2);
        assert!(!set.is_empty());
        assert_eq!(set.get(Id::new(2)), Some(&revision("b2")));
        assert_eq!(set.get(Id::new(3)), None);
        assert_eq!(
            set.iter()
                .map(|(id, rev)| (id.get(), rev.as_str()))
                .collect::<Vec<_>>(),
            [(1, "a1"), (2, "b2")]
        );
        assert_eq!(set.clone(), revisions(&[(1, "a1"), (2, "b2")]));
        assert_eq!(
            format!("{set:?}"),
            r#"{Id(/crew/1): Revision("a1"), Id(/crew/2): Revision("b2")}"#
        );
        assert_eq!(
            set.into_iter()
                .map(|(id, rev)| (id.get(), String::from(rev)))
                .collect::<Vec<_>>(),
            [(1, "a1".to_string()), (2, "b2".to_string())]
        );
    }

    #[test]
    fn collects_the_revisions_of_full_items() {
        let items = [
            Item {
                id: Id::new(4),
                revision: Revision::new("d").unwrap(),
            },
            Item {
                id: Id::new(3),
                revision: Revision::new("c").unwrap(),
            },
        ];
        let set = items.iter().collect::<Revisions<Item>>();
        assert_eq!(
            set.iter()
                .map(|(id, rev)| (id.get(), rev.as_str()))
                .collect::<Vec<_>>(),
            [(3, "c"), (4, "d")]
        );
    }

    #[test]
    fn reports_created_updated_and_deleted_items() {
        let known = revisions(&[(1, "a1"), (2, "b1"), (3, "c1"), (5, "e1")]);
        let current = revisions(&[(1, "a1"), (2, "b2"), (4, "d1"), (6, "f1"), (5, "e2")]);
        let changes = current.changes_since(&known);
        assert_eq!(changes.created, [Id::new(4), Id::new(6)]);
        assert_eq!(changes.updated, [Id::new(2), Id::new(5)]);
        assert_eq!(changes.deleted, [Id::new(3)]);
        assert!(!changes.is_empty());
        assert_eq!(changes.clone(), changes);
        assert_eq!(
            format!("{changes:?}"),
            "Changes { created: [Id(/crew/4), Id(/crew/6)], updated: [Id(/crew/2), Id(/crew/5)], \
             deleted: [Id(/crew/3)] }"
        );

        // Unchanged, first sync, and everything gone.
        assert!(known.changes_since(&known).is_empty());
        assert_eq!(known.changes_since(&Revisions::new()).created.len(), 4);
        let gone = Revisions::new().changes_since(&known);
        assert_eq!(gone.deleted.len(), 4);
        assert!(gone.created.is_empty() && gone.updated.is_empty());
        assert_ne!(gone, Changes::default());

        // Each list alone makes the changes non-empty.
        for (created, updated, deleted) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
            let changes = Changes::<Crew> {
                created: vec![Id::new(1); created],
                updated: vec![Id::new(1); updated],
                deleted: vec![Id::new(1); deleted],
            };
            assert!(!changes.is_empty());
        }
    }

    #[test]
    fn decodes_the_revision_of_a_listed_item() {
        let mut item: ProjectedItem = serde_json::from_str(
            r#"{"id": 7, "modified": null, "updateHash": "h7", "custom": {}}"#,
        )
        .unwrap();
        let read = ItemRevision::<Crew>::decode(&mut item).unwrap();
        assert_eq!((read.id, read.revision), (Id::new(7), revision("h7")));
        assert_eq!(ItemRevision::<Crew>::FIELDS, ["id"]);

        for wire in [
            r#"{"updateHash": "h7"}"#,
            r#"{"id": 7}"#,
            r#"{"id": 7, "updateHash": ""}"#,
        ] {
            let mut item: ProjectedItem = serde_json::from_str(wire).unwrap();
            assert!(ItemRevision::<Crew>::decode(&mut item).is_err(), "{wire}");
        }
    }
}
