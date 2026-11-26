//! The Google report: the markup read leniently into schema nodes and
//! measured against Google's required and recommended properties, as the
//! rich results Google shows need them.

use std::collections::BTreeMap;

use khalid_core::AbsoluteUrl;

use super::tree::Json;
use super::{FindingLevel, SchemaFinding, SchemaReport, finding};
use crate::schema::google_rules::rule_for;
use crate::schema::vocabulary::Vocabulary;
use crate::{IssueLevel, SchemaGraph, SchemaNode, SchemaValue, check_graph};

/// Every property Google needs or recommends that a node at the top of
/// `root` lacks, with the path of the node.
pub(super) fn google_findings(root: &Json) -> Vec<SchemaFinding> {
    let mut found = Vec::new();
    for (at, object) in top_nodes(root) {
        let Some(node) = node_of(object) else {
            continue;
        };
        for issue in check_graph(&SchemaGraph::new(vec![node])) {
            let level = match issue.level() {
                IssueLevel::Blocks => FindingLevel::Error,
                _ => FindingLevel::Warning,
            };
            let message = issue.message();
            found.push(finding(
                (SchemaReport::Google, level),
                &at,
                (
                    message.what().to_owned(),
                    if level == FindingLevel::Error {
                        "Google shows this rich result only when the property is there."
                    } else {
                        "Google can still show the rich result without it, but shows less."
                    },
                    message.fix().to_owned(),
                ),
            ));
        }
    }
    found
}

/// The `@type` of each node at the top of `root`, in the order written.
pub(super) fn top_types(root: &Json) -> Vec<String> {
    top_nodes(root)
        .into_iter()
        .filter_map(|(_, object)| match object.get("@type") {
            Some(Json::Text(kind)) => Some(kind.clone()),
            Some(Json::List(kinds)) => kinds.iter().find_map(|kind| match kind {
                Json::Text(kind) => Some(kind.clone()),
                _ => None,
            }),
            _ => None,
        })
        .collect()
}

/// Each node at the top of `root` with its path.
fn top_nodes(root: &Json) -> Vec<(String, &BTreeMap<String, Json>)> {
    match root {
        Json::Object(object) => match object.get("@graph") {
            Some(Json::List(items)) => objects(items, "@graph"),
            _ => vec![(String::new(), object)],
        },
        Json::List(items) => objects(items, ""),
        _ => Vec::new(),
    }
}

fn objects<'a>(items: &'a [Json], at: &str) -> Vec<(String, &'a BTreeMap<String, Json>)> {
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| match item {
            Json::Object(object) => Some((format!("{at}[{index}]"), object)),
            _ => None,
        })
        .collect()
}

/// The node `object` describes, its type the one Google has a rule for:
/// its own, or the nearest type it extends that has one, so a
/// `Restaurant` is measured as a `LocalBusiness`. `None` without a type.
fn node_of(object: &BTreeMap<String, Json>) -> Option<SchemaNode> {
    let kinds: Vec<&str> = match object.get("@type")? {
        Json::Text(kind) => vec![kind.as_str()],
        Json::List(kinds) => kinds
            .iter()
            .filter_map(|kind| match kind {
                Json::Text(kind) => Some(kind.as_str()),
                _ => None,
            })
            .collect(),
        _ => return None,
    };
    let first = kinds.first()?;
    let kind = kinds.iter().find_map(|kind| ruled(kind)).unwrap_or(first);
    let mut node = SchemaNode::new(kind);
    for (name, value) in object.iter().filter(|(name, _)| !name.starts_with('@')) {
        if let Some(value) = value_of(value) {
            node.set(name, value);
        }
    }
    Some(node)
}

/// `kind`, or the nearest type it extends, that Google has a rule for.
fn ruled(kind: &str) -> Option<&str> {
    let vocabulary = Vocabulary::get();
    let mut level = vec![kind];
    for _ in 0..8 {
        if let Some(found) = level.iter().find(|kind| rule_for(kind).is_some()) {
            return Some(found);
        }
        level = level
            .iter()
            .flat_map(|kind| vocabulary.parents(kind).iter().copied())
            .collect();
        if level.is_empty() {
            break;
        }
    }
    None
}

fn value_of(value: &Json) -> Option<SchemaValue> {
    Some(match value {
        Json::Null => return None,
        Json::Bool(flag) => SchemaValue::Boolean(*flag),
        Json::Number(number) => match number.as_i64() {
            Some(whole) => SchemaValue::Integer(whole),
            None => SchemaValue::Number(number.as_f64().unwrap_or_default()),
        },
        Json::Text(text) => SchemaValue::text(text),
        Json::List(items) => SchemaValue::List(items.iter().filter_map(value_of).collect()),
        Json::Object(object) => match object.get("@id") {
            Some(Json::Text(id)) if object.len() == 1 => {
                SchemaValue::Reference(AbsoluteUrl::parse(id).ok()?)
            }
            _ => SchemaValue::Node(Box::new(node_of(object)?)),
        },
    })
}
