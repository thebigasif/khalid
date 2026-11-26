//! [`Vocabulary`]: every schema.org type, enumeration member and property,
//! what each property belongs to and expects, and what each type extends.
//!
//! The data is schema.org's own published vocabulary, release 30.1, written
//! one line a name into `vocabulary.txt` by `scripts/schema-vocabulary.py`.
//! It is about 128 KB of text in the binary and is read into maps the first
//! time a check asks for it, so a site that never checks schema never pays
//! for it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::LazyLock;

use crate::redirect_moves::edit_distance;

/// The vocabulary as `scripts/schema-vocabulary.py` writes it.
const DATA: &str = include_str!("vocabulary.txt");

/// The schema.org release the data comes from.
const RELEASE: &str = "30.1";

static VOCABULARY: LazyLock<Vocabulary> = LazyLock::new(|| Vocabulary::read(DATA));

/// One schema.org type, such as `Event`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct TypeEntry {
    parents: Vec<&'static str>,
    newer: Option<&'static str>,
}

/// One member of an enumeration, such as `InStock` of `ItemAvailability`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct MemberEntry {
    enumeration: &'static str,
    newer: Option<&'static str>,
}

/// One schema.org property, such as `startDate`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct PropertyEntry {
    domains: Vec<&'static str>,
    ranges: Vec<&'static str>,
    newer: Option<&'static str>,
}

/// The whole schema.org vocabulary: types, enumeration members and
/// properties.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Vocabulary {
    types: BTreeMap<&'static str, TypeEntry>,
    members: BTreeMap<&'static str, MemberEntry>,
    properties: BTreeMap<&'static str, PropertyEntry>,
}

/// Which list of names to look in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Names {
    /// Type names and enumeration members, such as `Event` or `InStock`.
    Types,
    /// Property names, such as `startDate`.
    Properties,
}

impl Vocabulary {
    /// The vocabulary of schema.org release 30.1, read once.
    pub fn get() -> &'static Self {
        &VOCABULARY
    }

    /// The schema.org release the vocabulary comes from, such as `30.1`.
    pub fn release(&self) -> &'static str {
        RELEASE
    }

    /// Reads `data`, one record a line. A line it cannot read is skipped;
    /// a test proves the shipped data has none.
    fn read(data: &'static str) -> Self {
        let mut vocabulary = Self::default();
        for line in data.lines().filter(|line| !line.starts_with('#')) {
            let mut fields = line.split(' ');
            let (Some(kind), Some(name)) = (fields.next(), fields.next()) else {
                continue;
            };
            let rest: Vec<&'static str> = fields.collect();
            let newer = rest
                .iter()
                .find_map(|field| field.strip_prefix('>'))
                .filter(|newer| !newer.is_empty());
            let plain: Vec<&'static str> = rest
                .iter()
                .copied()
                .filter(|field| !field.starts_with('>'))
                .collect();
            match (kind, plain.as_slice()) {
                ("T", [parents]) => {
                    let parents = list(parents);
                    vocabulary.types.insert(name, TypeEntry { parents, newer });
                }
                ("E", [enumeration]) => {
                    let member = MemberEntry { enumeration, newer };
                    vocabulary.members.insert(name, member);
                }
                ("P", [domains, ranges]) => {
                    let entry = PropertyEntry {
                        domains: list(domains),
                        ranges: list(ranges),
                        newer,
                    };
                    vocabulary.properties.insert(name, entry);
                }
                _ => {}
            }
        }
        vocabulary
    }

    /// How many types, enumeration members and properties it holds.
    pub fn counts(&self) -> (usize, usize, usize) {
        (self.types.len(), self.members.len(), self.properties.len())
    }

    /// Whether `name` is a schema.org type, such as `Event`.
    pub fn is_type(&self, name: &str) -> bool {
        self.types.contains_key(name)
    }

    /// Whether `name` is a member of an enumeration, such as `InStock`.
    pub fn is_member(&self, name: &str) -> bool {
        self.members.contains_key(name)
    }

    /// Whether `name` is a schema.org property, such as `startDate`.
    pub fn is_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// The types `kind` directly extends, such as `Thing` for `Event`.
    /// Empty for `Thing`, the data types and names it does not know.
    pub fn parents(&self, kind: &str) -> &[&'static str] {
        self.types
            .get(kind)
            .map_or(&[], |entry| entry.parents.as_slice())
    }

    /// The enumeration `member` belongs to, such as `ItemAvailability` for
    /// `InStock`.
    pub fn enumeration_of(&self, member: &str) -> Option<&'static str> {
        self.members.get(member).map(|entry| entry.enumeration)
    }

    /// The members of `enumeration` and of the enumerations that extend it,
    /// in name order.
    pub fn members(&self, enumeration: &str) -> Vec<&'static str> {
        self.members
            .iter()
            .filter(|(_, entry)| self.is_a(entry.enumeration, enumeration))
            .map(|(name, _)| *name)
            .collect()
    }

    /// Whether `kind` is `ancestor` or extends it at any depth. A member of
    /// an enumeration is a kind of its enumeration, so `InStock` is an
    /// `ItemAvailability` and an `Enumeration`.
    pub fn is_a(&self, kind: &str, ancestor: &str) -> bool {
        let start = self.enumeration_of(kind).unwrap_or(kind);
        if kind == ancestor || start == ancestor {
            return true;
        }
        let mut seen = BTreeSet::new();
        let mut queue = vec![start];
        while let Some(next) = queue.pop() {
            if !seen.insert(next) {
                continue;
            }
            for parent in self.parents(next) {
                if *parent == ancestor {
                    return true;
                }
                queue.push(parent);
            }
        }
        false
    }

    /// The types `property` belongs to as schema.org lists them, such as
    /// `Event` and `Schedule` for `startDate`.
    pub fn domains(&self, property: &str) -> &[&'static str] {
        self.properties
            .get(property)
            .map_or(&[], |entry| entry.domains.as_slice())
    }

    /// The types a value of `property` may take, such as `Date` and
    /// `DateTime` for `startDate`.
    pub fn ranges(&self, property: &str) -> &[&'static str] {
        self.properties
            .get(property)
            .map_or(&[], |entry| entry.ranges.as_slice())
    }

    /// Whether `property` belongs to `kind`: one of its domains is `kind`
    /// or a type `kind` extends. A property or type it does not know never
    /// belongs.
    pub fn belongs_to(&self, property: &str, kind: &str) -> bool {
        self.domains(property)
            .iter()
            .any(|domain| self.is_a(kind, domain))
    }

    /// The name that replaces `name` when schema.org has superseded it,
    /// such as `SoftwareSourceCode` for `Code`.
    pub fn newer(&self, name: &str) -> Option<&'static str> {
        self.types
            .get(name)
            .and_then(|entry| entry.newer)
            .or_else(|| self.members.get(name).and_then(|entry| entry.newer))
            .or_else(|| self.properties.get(name).and_then(|entry| entry.newer))
    }

    /// Every name in `names`, in name order.
    fn names(&self, names: Names) -> Box<dyn Iterator<Item = &'static str> + '_> {
        match names {
            Names::Types => Box::new(self.types.keys().chain(self.members.keys()).copied()),
            Names::Properties => Box::new(self.properties.keys().copied()),
        }
    }

    fn knows(&self, name: &str, names: Names) -> bool {
        match names {
            Names::Types => self.is_type(name) || self.is_member(name),
            Names::Properties => self.is_property(name),
        }
    }
}

/// A comma list from the data, where `-` means none.
fn list(field: &'static str) -> Vec<&'static str> {
    field
        .split(',')
        .filter(|name| !name.is_empty() && *name != "-")
        .collect()
}

/// The schema.org name `name` is most likely a slip for: the same name in
/// other capitals, two letters swapped, or one letter added, dropped or
/// changed, and two for a name of ten letters or more. Names under four
/// letters are only matched by their capitals. `None` when `name` is a
/// schema.org name, or is not that close to one.
pub(crate) fn slip_for(name: &str, names: Names) -> Option<&'static str> {
    let vocabulary = Vocabulary::get();
    if vocabulary.knows(name, names) {
        return None;
    }
    let reach = match name.len() {
        0..4 => 0,
        4..10 => 1,
        _ => 2,
    };
    let lower = name.to_ascii_lowercase();
    vocabulary
        .names(names)
        .filter(|word| word.len().abs_diff(name.len()) <= reach)
        .filter_map(|word| {
            let other = word.to_ascii_lowercase();
            let apart = if swapped(&lower, &other) {
                1
            } else {
                edit_distance(&lower, &other)
            };
            (apart <= reach).then_some((apart, word))
        })
        .min_by_key(|(apart, _)| *apart)
        .map(|(_, word)| word)
}

/// Whether `a` is `b` with two letters side by side swapped.
fn swapped(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let differ: Vec<usize> = (0..a.len()).filter(|&i| a.get(i) != b.get(i)).collect();
    match differ.as_slice() {
        [i, j] if *j == i + 1 => a.get(*i) == b.get(*j) && a.get(*j) == b.get(*i),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{DATA, Names, Vocabulary, slip_for};
    use crate::schema::google_rules::TABLE;
    use crate::schema::template::is_word;

    #[test]
    fn every_line_of_the_data_is_read() {
        let records = DATA.lines().filter(|line| !line.starts_with('#')).count();
        let (types, members, properties) = Vocabulary::get().counts();
        assert_eq!(types + members + properties, records);
        assert_eq!((types, members, properties), (942, 544, 1540));
    }

    #[test]
    fn the_data_is_sorted_and_holds_names_only() {
        let mut last = ("", "");
        for line in DATA.lines().filter(|line| !line.starts_with('#')) {
            let mut fields = line.split(' ');
            let kind = fields.next().unwrap();
            let name = fields.next().unwrap();
            if kind == last.0 {
                assert!(last.1 < name, "{name} after {}", last.1);
            }
            last = (kind, name);
            let first = if kind == "P" {
                char::is_ascii_lowercase
            } else {
                char::is_ascii_alphanumeric
            };
            assert!(is_word(name, first), "{name}");
        }
    }

    #[test]
    fn every_name_the_data_points_to_is_in_it() {
        let vocabulary = Vocabulary::get();
        for (name, entry) in &vocabulary.types {
            for parent in &entry.parents {
                assert!(vocabulary.is_type(parent), "{name} extends {parent}");
            }
        }
        for (name, entry) in &vocabulary.members {
            assert!(vocabulary.is_type(entry.enumeration), "{name}");
        }
        for (name, entry) in &vocabulary.properties {
            for kind in entry.domains.iter().chain(&entry.ranges) {
                assert!(vocabulary.is_type(kind), "{name} names {kind}");
            }
        }
    }

    #[test]
    fn types_inherit_and_properties_belong_through_their_parents() {
        let vocabulary = Vocabulary::get();
        assert!(vocabulary.is_a("BlogPosting", "CreativeWork"));
        assert!(vocabulary.is_a("Restaurant", "Place"));
        assert!(vocabulary.is_a("Restaurant", "Organization"));
        assert!(!vocabulary.is_a("Person", "Organization"));
        assert!(vocabulary.belongs_to("startDate", "MusicEvent"));
        assert!(vocabulary.belongs_to("name", "Recipe"));
        assert!(!vocabulary.belongs_to("recipeIngredient", "Event"));
        assert_eq!(vocabulary.ranges("startDate"), ["Date", "DateTime"]);
        assert!(vocabulary.is_a("InStock", "ItemAvailability"));
        assert!(vocabulary.members("ItemAvailability").contains(&"InStock"));
        assert!(vocabulary.is_a("URL", "Text"));
    }

    #[test]
    fn superseded_names_point_to_their_replacement() {
        let vocabulary = Vocabulary::get();
        assert_eq!(vocabulary.newer("Code"), Some("SoftwareSourceCode"));
        assert_eq!(vocabulary.newer("Event"), None);
        assert!(
            vocabulary
                .properties
                .values()
                .any(|entry| entry.newer.is_some())
        );
    }

    #[test]
    fn every_name_in_google_s_rules_is_known() {
        let vocabulary = Vocabulary::get();
        for row in TABLE.lines() {
            let mut columns = row.split(';');
            let kinds = columns.next().unwrap_or_default();
            for kind in kinds.split_ascii_whitespace() {
                assert!(vocabulary.is_type(kind), "{kind}");
            }
            let properties = columns.skip(1).flat_map(|c| c.split([' ', '|']));
            for name in properties.filter(|n| !n.is_empty()) {
                assert!(vocabulary.is_property(name), "{name}");
            }
        }
    }

    #[test]
    fn a_swap_counts_as_one_slip_and_a_far_name_as_none() {
        assert_eq!(slip_for("Eevnt", Names::Types), Some("Event"));
        assert_eq!(slip_for("Ventures", Names::Types), None);
        assert_eq!(slip_for("URL", Names::Properties), Some("url"));
        assert_eq!(slip_for("uri", Names::Properties), None);
        assert_eq!(
            slip_for("TouristAtraction", Names::Types),
            Some("TouristAttraction")
        );
        assert_eq!(slip_for("instock", Names::Types), Some("InStock"));
    }
}
