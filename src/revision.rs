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

/// Revision of one resource item, as returned by Rentman's `updateHash`.
///
/// The hash is opaque: its algorithm is undocumented, so revisions can only
/// be compared. It is typed by resource and cannot be empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("a Rentman revision (updateHash) must not be empty")]
pub struct InvalidRevision;

/// Revision of an item of resource `R`.
pub struct Revision<R> {
    value: String,
    resource: PhantomData<fn() -> R>,
}

impl<R> Revision<R> {
    /// Creates a revision from its text, e.g. one stored after an earlier read.
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
pub trait Revisioned: Resource + Sized {
    /// ID of this item.
    fn id(&self) -> Id<Self>;
    /// Revision as returned by Rentman.
    fn revision(&self) -> &Revision<Self>;
}

/// Revisions of all items in one resource listing, keyed by item ID.
pub struct Revisions<R> {
    items: BTreeMap<Id<R>, Revision<R>>,
}

impl<R> Revisions<R> {
    /// Creates an empty revision set.
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    /// Inserts a revision, returning the revision it replaces.
    pub fn insert(&mut self, id: Id<R>, revision: Revision<R>) -> Option<Revision<R>> {
        self.items.insert(id, revision)
    }

    /// Gets the revision of one item.
    pub fn get(&self, id: Id<R>) -> Option<&Revision<R>> {
        self.items.get(&id)
    }

    /// Number of items in the set.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Iterates over item IDs and revisions in ID order.
    pub fn iter(&self) -> impl Iterator<Item = (Id<R>, &Revision<R>)> {
        self.items.iter().map(|(id, revision)| (*id, revision))
    }

    /// Compares this listing with a prior one; both must use the same filters.
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

/// Created, updated, and deleted IDs between two revision sets.
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
