//! The JSON-LD read into a typed tree, and the walk that checks each node
//! against the vocabulary.

use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

use super::values::check_value;
use super::{FindingLevel, FixTarget, SchemaFinding, SchemaFix, SchemaReport, finding};
use crate::SeoError;
use crate::schema::vocabulary::{Names, Vocabulary, slip_for};

/// The `@context` values that mean schema.org.
const CONTEXTS: [&str; 4] = [
    "https://schema.org",
    "https://schema.org/",
    "http://schema.org",
    "http://schema.org/",
];

/// Any JSON, read as written.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub(crate) enum Json {
    /// `null`.
    Null,
    /// `true` or `false`.
    Bool(bool),
    /// A number.
    Number(serde_json::Number),
    /// A string.
    Text(String),
    /// A list.
    List(Vec<Json>),
    /// An object.
    Object(BTreeMap<String, Json>),
}

/// Reads `text` as JSON.
pub(crate) fn read(text: &str) -> Result<Json, SeoError> {
    serde_json::from_str(text).map_err(|error| SeoError::CustomSchemaSyntax {
        line: error.line(),
        column: error.column(),
    })
}

/// `name` under `parent`, as `parent.name`, or `name` at the top.
pub(crate) fn place(parent: &str, name: &str) -> String {
    if parent.is_empty() {
        name.to_owned()
    } else {
        format!("{parent}.{name}")
    }
}

/// The `@type`s of `object`, one or a list, as written.
pub(crate) fn types_of(object: &BTreeMap<String, Json>) -> Vec<&str> {
    match object.get("@type") {
        Some(Json::Text(kind)) => vec![kind.as_str()],
        Some(Json::List(kinds)) => kinds
            .iter()
            .filter_map(|kind| match kind {
                Json::Text(kind) => Some(kind.as_str()),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// The walk: what it found, the `@id`s the markup defines and those it
/// points to.
#[derive(Debug, Default)]
pub(crate) struct Audit {
    findings: Vec<SchemaFinding>,
    defined: BTreeSet<String>,
    pointed: Vec<(String, String)>,
}

impl Audit {
    /// Walks the whole markup: one node, a `@graph` of nodes or a list.
    pub(crate) fn root(&mut self, root: &Json) {
        match root {
            Json::Object(object) => {
                if let Some(context) = object.get("@context") {
                    self.context(context);
                }
                match object.get("@graph") {
                    Some(Json::List(items)) => self.items(items, "@graph", None),
                    Some(_) => self.error(
                        "@graph",
                        "is not a list of nodes",
                        "Make @graph a list of objects, each with an @type.",
                    ),
                    None => self.node(object, "", None),
                }
            }
            Json::List(items) => self.items(items, "", None),
            _ => self.error(
                "",
                "is not a JSON-LD object",
                "Start the markup with {\"@context\": \"https://schema.org\", \"@type\": ...}.",
            ),
        }
    }

    /// The `@id`s of the nodes the markup walked so far defines.
    pub(crate) fn defined(&self) -> impl Iterator<Item = &String> {
        self.defined.iter()
    }

    /// The findings, with the `@id`s that point to nothing the markup or
    /// `known` defines.
    pub(crate) fn finish(mut self, known: &[String]) -> Vec<SchemaFinding> {
        let pointed = std::mem::take(&mut self.pointed);
        for (path, id) in pointed {
            let found = self.defined.contains(&id) || known.contains(&id);
            if !found {
                self.findings.push(finding(
                    (SchemaReport::SchemaOrg, FindingLevel::Warning),
                    &path,
                    (
                        format!("The @id {id} points to nothing on this page."),
                        "A search engine cannot join what the reference is meant to name.",
                        format!(
                            "Add a node with \"@id\": \"{id}\", or write the node out in full here."
                        ),
                    ),
                ));
            }
        }
        self.findings
    }

    fn context(&mut self, context: &Json) {
        let named = match context {
            Json::Text(text) => CONTEXTS.contains(&text.as_str()),
            Json::List(items) => items
                .iter()
                .any(|item| matches!(item, Json::Text(text) if CONTEXTS.contains(&text.as_str()))),
            Json::Object(_) => true,
            _ => false,
        };
        if !named {
            self.error(
                "@context",
                "does not name schema.org",
                "Set @context to https://schema.org.",
            );
        }
    }

    fn items(&mut self, items: &[Json], at: &str, ranges: Option<&[&str]>) {
        for (index, item) in items.iter().enumerate() {
            let here = format!("{at}[{index}]");
            match item {
                Json::Object(object) => self.node(object, &here, ranges),
                _ => self.error(
                    &here,
                    "is not a node",
                    "Write each node as an object with an @type.",
                ),
            }
        }
    }

    /// Checks the node `object` at `at`, the value of a property that
    /// expects `ranges`, or a node at the top when `None`.
    pub(crate) fn node(
        &mut self,
        object: &BTreeMap<String, Json>,
        at: &str,
        ranges: Option<&[&str]>,
    ) {
        let vocabulary = Vocabulary::get();
        let kinds = types_of(object);
        if kinds.is_empty() && !object.contains_key("@value") {
            self.error(
                &place(at, "@type"),
                "is missing",
                "Add an @type, such as Event, so search engines know what this is.",
            );
        }
        let known: Vec<&str> = kinds
            .iter()
            .copied()
            .filter(|kind| self.known_type(kind, &place(at, "@type")))
            .collect();
        if let (Some(ranges), false) = (ranges, known.is_empty()) {
            self.fits(&known, ranges, at);
        }
        if let Some(Json::Text(id)) = object.get("@id") {
            self.defined.insert(id.clone());
        }
        for (name, value) in object {
            if name.starts_with('@') || is_annotation(name) {
                continue;
            }
            let here = place(at, name);
            if !self.known_property(name, &here) {
                continue;
            }
            if !known.is_empty() && !known.iter().any(|kind| vocabulary.belongs_to(name, kind)) {
                self.push(
                    FindingLevel::Warning,
                    &here,
                    (
                        format!("{name} is not a property of {}.", known.join(" or ")),
                        "Search engines may skip a property schema.org does not list for the type.",
                        format!("Move {name} to a node it belongs to, or remove it."),
                    ),
                );
            }
            self.value(value, &here, name);
        }
    }

    /// Checks the value of property `name` at `at`.
    fn value(&mut self, value: &Json, at: &str, name: &str) {
        let ranges = Vocabulary::get().ranges(name);
        match value {
            Json::List(items) => {
                for (index, item) in items.iter().enumerate() {
                    self.value(item, &format!("{at}[{index}]"), name);
                }
            }
            Json::Object(object) => match object.get("@id") {
                Some(Json::Text(id)) if object.len() == 1 => {
                    self.pointed.push((place(at, "@id"), id.clone()));
                }
                _ => match object.get("@value") {
                    Some(inner) if !object.contains_key("@type") => self.value(inner, at, name),
                    _ => self.node(object, at, Some(ranges)),
                },
            },
            other => {
                if let Some(found) = check_value(other, at, name, ranges) {
                    self.findings.push(found);
                }
            }
        }
    }

    /// Whether `kind` is a schema.org type, after noting a slip, an unknown
    /// name or a superseded one.
    fn known_type(&mut self, kind: &str, at: &str) -> bool {
        let vocabulary = Vocabulary::get();
        if vocabulary.is_type(kind) || vocabulary.is_member(kind) {
            if let Some(newer) = vocabulary.newer(kind) {
                self.superseded(kind, newer, at, FixTarget::Type);
            }
            return true;
        }
        let slip = slip_for(kind, Names::Types);
        self.unknown(kind, at, slip, FixTarget::Type);
        false
    }

    /// Whether `name` is a schema.org property, after noting a slip, an
    /// unknown name or a superseded one.
    fn known_property(&mut self, name: &str, at: &str) -> bool {
        let vocabulary = Vocabulary::get();
        if vocabulary.is_property(name) {
            if let Some(newer) = vocabulary.newer(name) {
                self.superseded(name, newer, at, FixTarget::Property);
            }
            return true;
        }
        let slip = slip_for(name, Names::Properties);
        self.unknown(name, at, slip, FixTarget::Property);
        false
    }

    fn unknown(&mut self, name: &str, at: &str, slip: Option<&str>, target: FixTarget) {
        let noun = match target {
            FixTarget::Type => "type",
            FixTarget::Property | FixTarget::Value => "property",
        };
        let (what, fix) = match slip {
            Some(meant) => (
                format!("{name} is not a schema.org {noun}; it looks like {meant}."),
                format!("Change {name} to {meant}."),
            ),
            None => (
                format!("{name} is not a schema.org {noun}."),
                format!("Use a {noun} schema.org lists, or remove {name}."),
            ),
        };
        let why = "Search engines skip a name schema.org does not have, without saying so.";
        let mut found = finding(
            (SchemaReport::SchemaOrg, FindingLevel::Error),
            at,
            (what, why, fix),
        );
        found.fix = slip.map(|meant| SchemaFix {
            target,
            path: at.to_owned(),
            from: name.to_owned(),
            to: meant.to_owned(),
        });
        self.findings.push(found);
    }

    fn superseded(&mut self, name: &str, newer: &str, at: &str, target: FixTarget) {
        let mut found = finding(
            (SchemaReport::SchemaOrg, FindingLevel::Warning),
            at,
            (
                format!("schema.org has replaced {name} with {newer}."),
                "Search engines may stop reading the old name.",
                format!("Change {name} to {newer}."),
            ),
        );
        found.fix = Some(SchemaFix {
            target,
            path: at.to_owned(),
            from: name.to_owned(),
            to: newer.to_owned(),
        });
        self.findings.push(found);
    }

    /// Notes a node of types `kinds` at `at` that fits none of `ranges`.
    fn fits(&mut self, kinds: &[&str], ranges: &[&str], at: &str) {
        let vocabulary = Vocabulary::get();
        let fits = ranges.is_empty()
            || kinds
                .iter()
                .any(|kind| ranges.iter().any(|range| vocabulary.is_a(kind, range)));
        if !fits {
            self.push(
                FindingLevel::Error,
                at,
                (
                    format!(
                        "A {} does not fit here; this property expects {}.",
                        kinds.join(" or "),
                        either(ranges)
                    ),
                    "Search engines skip a value of a type the property does not take.",
                    format!("Change the @type to {}, or move the node.", either(ranges)),
                ),
            );
        }
    }

    fn push(&mut self, level: FindingLevel, at: &str, message: (String, &'static str, String)) {
        self.findings
            .push(finding((SchemaReport::SchemaOrg, level), at, message));
    }

    fn error(&mut self, at: &str, problem: &str, fix: &str) {
        let what = if at.is_empty() {
            format!("The markup {problem}.")
        } else {
            format!("{at} {problem}.")
        };
        let why = "Search engines cannot read markup in this shape.";
        self.push(FindingLevel::Error, at, (what, why, fix.to_owned()));
    }
}

/// Whether `name` is an action annotation such as `query-input`, which
/// schema.org's potential actions add to a property name.
fn is_annotation(name: &str) -> bool {
    name.strip_suffix("-input")
        .or_else(|| name.strip_suffix("-output"))
        .is_some_and(|base| Vocabulary::get().is_property(base))
}

/// `a`, `a or b`, `a, b or c`.
pub(crate) fn either(names: &[&str]) -> String {
    match names.split_last() {
        Some((last, [])) => (*last).to_owned(),
        Some((last, rest)) => format!("{} or {last}", rest.join(", ")),
        None => String::new(),
    }
}
