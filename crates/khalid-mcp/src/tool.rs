//! The tools a server offers, and what a call answers.

use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

use serde::Serialize;
use serde_json::{Map, Value, json};

/// A future a tool returns, boxed so a server can hold any `dyn Tools`.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// What the server can do, as `tools/list` describes it and `tools/call`
/// runs it.
pub trait Tools: Debug + Send + Sync {
    /// Every tool, in the order an agent should read them.
    fn list(&self) -> &[Tool];

    /// Runs the tool called `name` with `arguments`, or returns `None` when
    /// no tool has that name. Arguments the tool cannot use are a failed
    /// [`ToolResult`] that says why, so the agent can correct them.
    fn call<'a>(
        &'a self,
        name: &'a str,
        arguments: Map<String, Value>,
    ) -> BoxFuture<'a, Option<ToolResult>>;
}

/// One tool: its name, what it does, the JSON Schema of its arguments,
/// and the hints that tell an agent's client whether to ask before a call.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    name: String,
    description: String,
    input_schema: Value,
    annotations: Hints,
}

/// What a call does to the site, as the HTTP method of its route says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    /// Only reads, like a `GET`.
    Reads,
    /// Adds something new, like a `POST`. Calling it twice adds two.
    Adds,
    /// Overwrites what is there with what it is given, like a `PUT`.
    /// Calling it twice leaves what once does.
    Overwrites,
    /// Changes part of what is there, like a `PATCH`.
    Changes,
    /// Removes something, like a `DELETE`.
    Removes,
}

/// The tool annotations of the Model Context Protocol, made from an
/// [`Effect`] so a tool cannot claim to read and remove at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
struct Hints {
    read_only_hint: bool,
    destructive_hint: bool,
    idempotent_hint: bool,
}

impl From<Effect> for Hints {
    fn from(effect: Effect) -> Self {
        let (read_only_hint, destructive_hint, idempotent_hint) = match effect {
            Effect::Reads => (true, false, true),
            Effect::Adds => (false, false, false),
            Effect::Overwrites | Effect::Removes => (false, true, true),
            Effect::Changes => (false, true, false),
        };
        Self {
            read_only_hint,
            destructive_hint,
            idempotent_hint,
        }
    }
}

impl Tool {
    /// A tool called `name`, described for an agent by `description`,
    /// whose calls have `effect`, taking arguments that match the JSON
    /// Schema `input_schema`.
    pub fn new(name: &str, description: &str, effect: Effect, input_schema: Value) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            input_schema,
            annotations: Hints::from(effect),
        }
    }

    /// The name an agent calls it by.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// What a tool call answers: text for the agent, and whether the call
/// failed in a way the agent can act on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolResult {
    text: String,
    outcome: Outcome,
}

/// Whether a call did what it was asked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Done,
    Failed,
}

impl ToolResult {
    /// The call did what it was asked, and `text` says what came of it.
    pub fn done(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            outcome: Outcome::Done,
        }
    }

    /// The call did not do what it was asked, and `text` says why and what
    /// to change.
    pub fn failed(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            outcome: Outcome::Failed,
        }
    }

    /// The text the agent reads.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Whether the call failed.
    pub fn is_failure(&self) -> bool {
        self.outcome == Outcome::Failed
    }

    /// The result as `tools/call` answers it.
    pub(crate) fn to_json(&self) -> Value {
        json!({
            "content": [{ "type": "text", "text": self.text }],
            "isError": self.is_failure(),
        })
    }
}
