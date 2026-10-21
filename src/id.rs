//! Typed Rentman IDs and the links between resources.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{cmp::Ordering, fmt, hash, marker::PhantomData};

/// A Rentman resource: the item type of one collection, e.g. `/crew`.
pub trait Resource {
    /// Collection path segment without slashes, e.g. `crew`.
    const COLLECTION: &'static str;
}

/// ID of an item of resource `R`, e.g. a crew member.
///
/// An item's own `id` and every link to it have the same type, so `crew.id`
/// can be used wherever a link to a crew member is expected, and a crew ID
/// cannot be passed where a project ID is expected.
///
/// In an `id` field Rentman sends a number (`12`); in a link field it sends a
/// path (`"/crew/12"`). The generated models read and write both forms;
/// [`Id::from_link`] and [`Id::to_link`] convert links by hand.
pub struct Id<R> {
    value: u64,
    resource: PhantomData<fn() -> R>,
}

impl<R> Id<R> {
    /// Creates an ID from Rentman's numeric value.
    pub const fn new(value: u64) -> Self {
        Self {
            value,
            resource: PhantomData,
        }
    }

    /// Rentman's numeric value of the ID.
    pub const fn get(self) -> u64 {
        self.value
    }
}

impl<R: Resource> Id<R> {
    /// Parses a link such as `/crew/12`.
    ///
    /// Fails when the link points to another collection, has no numeric ID,
    /// or is not a `/{collection}/{id}` path.
    pub fn from_link(link: &str) -> Result<Self, LinkError> {
        let malformed = || LinkError::Malformed {
            link: link.to_string(),
        };
        let (collection, id) = link
            .strip_prefix('/')
            .and_then(|path| path.split_once('/'))
            .ok_or_else(malformed)?;
        if collection.is_empty() || id.contains('/') {
            return Err(malformed());
        }
        if collection != R::COLLECTION {
            return Err(LinkError::WrongCollection {
                expected: R::COLLECTION,
                found: collection.to_string(),
            });
        }
        // `u64::from_str` accepts a leading `+`; Rentman never sends one.
        if !id.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(LinkError::InvalidId { id: id.to_string() });
        }
        id.parse()
            .map(Self::new)
            .map_err(|_| LinkError::InvalidId { id: id.to_string() })
    }

    /// Formats the ID as a link, e.g. `/crew/12`.
    pub fn to_link(self) -> String {
        format!("/{}/{}", R::COLLECTION, self.value)
    }
}

/// Why a link could not be read as an [`Id`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum LinkError {
    /// The link points to a different collection than expected.
    #[error("link to /{found} where a link to /{expected} was expected")]
    WrongCollection {
        /// Collection the field links to.
        expected: &'static str,
        /// Collection found in the link.
        found: String,
    },
    /// The ID part of the link is not a non-negative integer.
    #[error("link ID {id:?} is not a non-negative integer")]
    InvalidId {
        /// ID part as found in the link.
        id: String,
    },
    /// The link is not a `/{collection}/{id}` path.
    #[error("link {link:?} is not a /{{collection}}/{{id}} path")]
    Malformed {
        /// Link as found.
        link: String,
    },
}

// Manual impls: derives would require `R` itself to implement the traits.
impl<R> Clone for Id<R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> Copy for Id<R> {}

impl<R> PartialEq for Id<R> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<R> Eq for Id<R> {}

impl<R> PartialOrd for Id<R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<R> Ord for Id<R> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<R> hash::Hash for Id<R> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<R: Resource> fmt::Debug for Id<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({})", self.to_link())
    }
}

/// Displays Rentman's numeric value, e.g. `12`.
impl<R> fmt::Display for Id<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<R> From<Id<R>> for u64 {
    fn from(id: Id<R>) -> Self {
        id.value
    }
}

/// Serializes the numeric form used by `id` fields.
impl<R> Serialize for Id<R> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.value)
    }
}

/// Deserializes the numeric form used by `id` fields.
impl<'de, R> Deserialize<'de> for Id<R> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        u64::deserialize(deserializer).map(Self::new)
    }
}

/// Serde adapter for link fields (`"/crew/12"`), used by the generated
/// models with `#[serde(with = "crate::id::link")]`.
pub(crate) mod link {
    use super::{Id, Resource};
    use serde::{Deserialize, Deserializer, Serializer, de::Error};

    pub(crate) fn serialize<R: Resource, S: Serializer>(
        id: &Id<R>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&id.to_link())
    }

    pub(crate) fn deserialize<'de, R: Resource, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Id<R>, D::Error> {
        let link = String::deserialize(deserializer)?;
        Id::from_link(&link).map_err(D::Error::custom)
    }
}

/// Serde adapter for optional or nullable link fields.
pub(crate) mod optional_link {
    use super::{Id, Resource};
    use serde::{Deserialize, Deserializer, Serializer, de::Error};

    pub(crate) fn serialize<R: Resource, S: Serializer>(
        id: &Option<Id<R>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match id {
            Some(id) => super::link::serialize(id, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub(crate) fn deserialize<'de, R: Resource, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Id<R>>, D::Error> {
        Option::<String>::deserialize(deserializer)?
            .map(|link| Id::from_link(&link).map_err(D::Error::custom))
            .transpose()
    }
}

/// ID of an item whose collection is named by a separate `itemtype` field,
/// e.g. the item a file is attached to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ItemId(u64);

impl ItemId {
    /// Creates an item ID from Rentman's numeric value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Rentman's numeric value of the ID.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
// Coverage is disabled for test code so the production 100% gate measures only
// library behavior.
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use serde_json::json;
    use std::collections::{BTreeSet, HashSet};

    struct Crew;
    impl Resource for Crew {
        const COLLECTION: &'static str = "crew";
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Links {
        id: Id<Crew>,
        #[serde(with = "link")]
        crew: Id<Crew>,
        #[serde(with = "optional_link")]
        replacement: Option<Id<Crew>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "optional_link"
        )]
        optional: Option<Id<Crew>>,
    }

    #[test]
    fn reads_and_writes_ids_as_numbers_and_links_as_paths() {
        let wire = json!({"id": 7, "crew": "/crew/12", "replacement": null});
        let links: Links = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(
            links,
            Links {
                id: Id::new(7),
                crew: Id::new(12),
                replacement: None,
                optional: None,
            }
        );
        assert_eq!(serde_json::to_value(&links).unwrap(), wire);

        let links: Links = serde_json::from_value(
            json!({"id": 7, "crew": "/crew/12", "replacement": "/crew/13", "optional": "/crew/14"}),
        )
        .unwrap();
        assert_eq!(links.replacement, Some(Id::new(13)));
        assert_eq!(links.optional, Some(Id::new(14)));
        assert_eq!(
            serde_json::to_value(&links).unwrap(),
            json!({"id": 7, "crew": "/crew/12", "replacement": "/crew/13", "optional": "/crew/14"})
        );
    }

    #[test]
    fn rejects_links_to_another_collection_or_malformed_links() {
        for (wire, message) in [
            (
                json!({"id": 7, "crew": "/projects/12", "replacement": null}),
                "link to /projects where a link to /crew was expected",
            ),
            (
                json!({"id": 7, "crew": "/crew/12", "replacement": "/crew/x"}),
                "link ID \"x\" is not a non-negative integer",
            ),
            (
                json!({"id": 7, "crew": 12, "replacement": null}),
                "invalid type",
            ),
            (
                json!({"id": "7", "crew": "/crew/12", "replacement": null}),
                "invalid type",
            ),
            (
                json!({"id": 7, "crew": "/crew/12", "replacement": 3}),
                "invalid type",
            ),
        ] {
            let error = serde_json::from_value::<Links>(wire).unwrap_err();
            assert!(error.to_string().contains(message), "{error}");
        }
    }

    #[test]
    fn classifies_invalid_links() {
        let cases = [
            (
                "/projects/12",
                LinkError::WrongCollection {
                    expected: "crew",
                    found: "projects".into(),
                },
            ),
            ("/crew/", LinkError::InvalidId { id: String::new() }),
            ("/crew/-1", LinkError::InvalidId { id: "-1".into() }),
            ("/crew/+1", LinkError::InvalidId { id: "+1".into() }),
            (
                "/crew/18446744073709551616",
                LinkError::InvalidId {
                    id: "18446744073709551616".into(),
                },
            ),
            (
                "crew/12",
                LinkError::Malformed {
                    link: "crew/12".into(),
                },
            ),
            (
                "/crew",
                LinkError::Malformed {
                    link: "/crew".into(),
                },
            ),
            (
                "//12",
                LinkError::Malformed {
                    link: "//12".into(),
                },
            ),
            (
                "/crew/12/appointments",
                LinkError::Malformed {
                    link: "/crew/12/appointments".into(),
                },
            ),
            (
                "",
                LinkError::Malformed {
                    link: String::new(),
                },
            ),
        ];
        for (link, expected) in cases {
            assert_eq!(Id::<Crew>::from_link(link).unwrap_err(), expected, "{link}");
        }
        assert_eq!(
            LinkError::Malformed { link: "x".into() }.to_string(),
            "link \"x\" is not a /{collection}/{id} path"
        );
        assert_eq!(Id::<Crew>::from_link("/crew/0").unwrap().get(), 0);
        assert_eq!(
            Id::<Crew>::from_link("/crew/18446744073709551615")
                .unwrap()
                .get(),
            u64::MAX
        );
    }

    #[test]
    fn behaves_like_its_number() {
        let id = Id::<Crew>::new(12);
        let copy = id;
        assert_eq!(id.clone(), copy);
        assert_ne!(id, Id::new(13));
        assert!(id < Id::new(13));
        assert_eq!(id.partial_cmp(&Id::new(12)), Some(Ordering::Equal));
        assert_eq!(id.to_string(), "12");
        assert_eq!(format!("{id:?}"), "Id(/crew/12)");
        assert_eq!(id.to_link(), "/crew/12");
        assert_eq!(u64::from(id), 12);
        assert_eq!(HashSet::from([id, Id::new(12)]).len(), 1);
        assert_eq!(
            BTreeSet::from([Id::<Crew>::new(2), Id::new(1)]).first(),
            Some(&Id::new(1))
        );
    }

    #[test]
    fn item_ids_are_plain_numbers() {
        let id: ItemId = serde_json::from_value(json!(5)).unwrap();
        assert_eq!(id, ItemId::new(5));
        assert_eq!(id.get(), 5);
        assert_eq!(id.to_string(), "5");
        assert_eq!(serde_json::to_value(id).unwrap(), json!(5));
    }

    proptest! {
        #[test]
        fn links_round_trip(value in any::<u64>()) {
            let id = Id::<Crew>::new(value);
            prop_assert_eq!(Id::<Crew>::from_link(&id.to_link()), Ok(id));
            let wire = serde_json::to_value(Links { id, crew: id, replacement: Some(id), optional: None }).unwrap();
            let links: Links = serde_json::from_value(wire).unwrap();
            prop_assert_eq!(links.crew, id);
            prop_assert_eq!(links.replacement, Some(id));
        }

        #[test]
        fn parsing_arbitrary_links_never_panics(link in ".*") {
            if let Ok(id) = Id::<Crew>::from_link(&link) {
                prop_assert_eq!(Id::<Crew>::from_link(&id.to_link()), Ok(id));
            }
        }

        #[test]
        fn links_to_other_collections_are_rejected(collection in "[a-z_]{1,20}", value in any::<u64>()) {
            prop_assume!(collection != "crew");
            let link = format!("/{collection}/{value}");
            prop_assert_eq!(
                Id::<Crew>::from_link(&link),
                Err(LinkError::WrongCollection { expected: "crew", found: collection })
            );
        }
    }
}
