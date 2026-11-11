//! [`CopiedSchema`]: the JSON-LD another page carries, read as a starting
//! point for a custom JSON-LD field ("copy schema from a URL", KH-SEO-4).

use khalid_core::AbsoluteUrl;
use khalid_outbound::{Outbound, OutboundError};
use serde_json::{Map, Value};

use crate::CustomSchema;

/// The context the copy names, the one a custom field takes.
const CONTEXT: &str = "https://schema.org";

/// The script type that marks JSON-LD in a page.
const JSON_LD: &str = "application/ld+json";

/// The JSON-LD of one page, as text a custom JSON-LD field takes.
///
/// Every JSON-LD script on the page joins one graph. The other site's
/// `@id`s are left out, since they name that site's pages, and so is a value
/// that only pointed to one, or was empty. A list of types keeps its first.
/// What is left is a template to edit, not a copy to publish as it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopiedSchema {
    text: String,
    types: Vec<String>,
    problem: Option<String>,
}

impl CopiedSchema {
    /// Fetches `url` through `outbound` and copies its JSON-LD.
    ///
    /// # Errors
    ///
    /// [`CopySchemaError::Fetch`] when the request fails or is refused,
    /// such as an address inside the network; [`CopySchemaError::Status`]
    /// when the page answers with a status other than 2xx; and
    /// [`CopySchemaError::NoSchema`] when it carries no JSON-LD node.
    pub async fn fetch(outbound: &Outbound, url: &AbsoluteUrl) -> Result<Self, CopySchemaError> {
        let fetched = outbound
            .get(url)
            .await
            .map_err(|source| CopySchemaError::Fetch { source })?;
        let status = fetched.status();
        if !(200..300).contains(&status) {
            return Err(CopySchemaError::Status { status });
        }
        Self::from_html(&String::from_utf8_lossy(fetched.body())).ok_or(CopySchemaError::NoSchema)
    }

    /// Copies the JSON-LD of the page `html`, or `None` when it has no
    /// script that holds a node with an `@type`. A script that is not JSON
    /// is skipped.
    pub fn from_html(html: &str) -> Option<Self> {
        let mut nodes: Vec<Value> = scripts(html)
            .filter_map(|text| serde_json::from_str::<Value>(text.trim()).ok())
            .flat_map(top_nodes)
            .filter_map(|node| cleaned(node).filter(|node| node.get("@type").is_some()))
            .collect();
        let types = nodes
            .iter()
            .filter_map(|node| node.get("@type").and_then(Value::as_str))
            .map(str::to_owned)
            .collect();
        let mut root = Map::new();
        root.insert("@context".to_owned(), CONTEXT.into());
        match nodes.len() {
            0 => return None,
            1 => {
                if let Some(Value::Object(node)) = nodes.pop() {
                    root.extend(node);
                }
            }
            _ => {
                root.insert("@graph".to_owned(), Value::Array(nodes));
            }
        }
        let text = serde_json::to_string_pretty(&Value::Object(root)).ok()?;
        let problem = CustomSchema::parse(&text)
            .err()
            .map(|error| error.to_string());
        Some(Self {
            text,
            types,
            problem,
        })
    }

    /// The JSON-LD, pretty-printed, ready to edit in a custom field.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The `@type` of each node at the top, in page order.
    pub fn types(&self) -> &[String] {
        &self.types
    }

    /// Why a custom field would refuse the copy as it is, or `None` when it
    /// takes it.
    pub fn problem(&self) -> Option<&str> {
        self.problem.as_deref()
    }
}

/// Why no schema could be copied from a page.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CopySchemaError {
    /// The request failed or the guard refused it.
    #[error("the page could not be fetched: {source}")]
    Fetch {
        /// What the outbound client reported.
        #[source]
        source: OutboundError,
    },
    /// The page answered with a status other than 2xx.
    #[error("the page answered with status {status}")]
    Status {
        /// The HTTP status code.
        status: u16,
    },
    /// The page holds no JSON-LD node with an `@type`.
    #[error("the page has no JSON-LD schema")]
    NoSchema,
}

impl CopySchemaError {
    /// The host the guard refused because it points inside the network,
    /// or `None` for any other failure.
    pub fn inward_host(&self) -> Option<&str> {
        match self {
            Self::Fetch {
                source: OutboundError::Blocked { host, .. },
            } => Some(host),
            _ => None,
        }
    }
}

/// The text inside each `<script type="application/ld+json">` of `html`.
fn scripts(html: &str) -> impl Iterator<Item = &str> {
    // ASCII lowercasing keeps every byte where it was, so offsets agree.
    let lower = html.to_ascii_lowercase();
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(start) = lower.get(from..).and_then(|rest| rest.find("<script")) {
        let open = from.saturating_add(start);
        let Some(close_tag) = lower.get(open..).and_then(|rest| rest.find('>')) else {
            break;
        };
        let body = open.saturating_add(close_tag).saturating_add(1);
        let tag = lower.get(open..body).unwrap_or_default();
        let named = tag
            .get("<script".len()..)
            .is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_whitespace()));
        let Some(length) = lower.get(body..).and_then(|rest| rest.find("</script")) else {
            break;
        };
        let end = body.saturating_add(length);
        if named && tag.contains(JSON_LD) {
            found.push(html.get(body..end).unwrap_or_default());
        }
        from = end;
    }
    found.into_iter()
}

/// The nodes at the top of one script: the object, each object of a list,
/// and each object of a `@graph`.
fn top_nodes(value: Value) -> Vec<Value> {
    match value {
        Value::Array(items) => items.into_iter().flat_map(top_nodes).collect(),
        Value::Object(mut object) => match object.shift_remove("@graph") {
            Some(graph) => top_nodes(graph),
            None => vec![Value::Object(object)],
        },
        _ => Vec::new(),
    }
}

/// `value` without the other site's ids, its other keywords, empty values,
/// and values that only pointed to another node, or `None` when nothing is
/// left of it.
fn cleaned(value: Value) -> Option<Value> {
    match value {
        Value::Null => None,
        Value::Array(items) => {
            let kept: Vec<Value> = items.into_iter().filter_map(cleaned).collect();
            (!kept.is_empty()).then_some(Value::Array(kept))
        }
        Value::Object(object) => {
            let mut kept = Map::new();
            for (name, value) in object {
                let value = match name.as_str() {
                    "@type" => first_type(value),
                    _ if name.starts_with('@') => None,
                    _ => cleaned(value),
                };
                if let Some(value) = value {
                    kept.insert(name, value);
                }
            }
            (!kept.is_empty()).then_some(Value::Object(kept))
        }
        Value::String(text) if text.trim().is_empty() => None,
        other => Some(other),
    }
}

/// One type name, the first of a list.
fn first_type(value: Value) -> Option<Value> {
    match value {
        Value::Array(items) => items.into_iter().find(Value::is_string),
        Value::String(_) => Some(value),
        _ => None,
    }
}
