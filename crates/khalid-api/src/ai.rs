//! Writing suggestions for the editor (Yoast package WP4,
//! `docs/adr/0000-ai-suggestions-ask-a-service-the-owner-names.md`).
//!
//! Khalid holds no model. An [`AiWriter`] asks the service the owner named
//! in `[ai]`, and the `ai` route of the Admin API checks each option it
//! answers against the length rules before a writer sees it.

use std::error::Error;
use std::sync::Arc;

use khalid_core::{AiProvider, UserMessage};

use crate::content_store::BoxFuture;

mod task;

pub use task::{AiTask, AiWords, LengthRule, Lengths};

/// One request to the service: what to write, and the words to write it
/// from.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct AiAsk {
    /// What is being written.
    pub task: AiTask,
    /// The instructions every request of this task carries.
    pub system: String,
    /// The post's words and anything else this request needs.
    pub prompt: String,
    /// How many options to answer, one per line.
    pub count: u8,
}

/// The service that writes suggestions. The HTTP writers ask Anthropic, an
/// OpenAI-compatible service or `SerpDad`; `FakeWriter`, from the
/// `test-util` feature, answers set options for tests.
pub trait AiWriter: Send + Sync {
    /// The options the service wrote for `ask`, at most `ask.count`.
    ///
    /// # Errors
    ///
    /// [`AiError`] when the service could not be reached or refused.
    fn write<'a>(&'a self, ask: &'a AiAsk) -> BoxFuture<'a, Result<Vec<String>, AiError>>;
}

/// Why the service wrote nothing.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AiError {
    /// The service could not be reached, or did not answer in time.
    #[error("the AI service could not be reached")]
    Unreachable(#[source] Box<dyn Error + Send + Sync>),
    /// The service refused the key.
    #[error("the AI service refused the key, with status {status}")]
    Refused {
        /// The HTTP status it answered.
        status: u16,
    },
    /// The service answered something other than suggestions.
    #[error("the AI service answered with status {status} and no suggestions")]
    Answer {
        /// The HTTP status it answered.
        status: u16,
    },
}

impl AiError {
    /// What the writer reads.
    pub fn message(&self) -> UserMessage {
        match self {
            Self::Refused { .. } => UserMessage::new(
                "The AI service did not accept the key.",
                "The key in KHALID_AI_KEY is wrong, has expired or has no credit left.",
                "Ask the site's owner to make a new key and set it on the server.",
            ),
            Self::Unreachable(_) | Self::Answer { .. } => UserMessage::new(
                "The AI service did not answer with suggestions.",
                "It may be busy or down for a moment.",
                "Try again in a minute. Nothing in the post changed.",
            ),
        }
    }
}

/// What the `ai` route needs: the provider the owner named, the writer
/// that asks it, and the length rules each option is checked against.
#[derive(Clone)]
pub struct AiDesk {
    pub(crate) provider: AiProvider,
    pub(crate) writer: Option<Arc<dyn AiWriter>>,
    pub(crate) lengths: Lengths,
}

impl std::fmt::Debug for AiDesk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AiDesk")
            .field("provider", &self.provider)
            .field("writer", &self.writer.is_some())
            .field("lengths", &self.lengths)
            .finish_non_exhaustive()
    }
}

impl AiDesk {
    /// Suggestions off: nothing is ever sent.
    pub fn off() -> Self {
        Self {
            provider: AiProvider::Off,
            writer: None,
            lengths: Lengths::default(),
        }
    }

    /// Suggestions from `provider`, asked through `writer`, or `None` when
    /// the server holds no key for it; each option checked against
    /// `lengths`.
    pub fn new(provider: AiProvider, writer: Option<Arc<dyn AiWriter>>, lengths: Lengths) -> Self {
        Self {
            provider,
            writer,
            lengths,
        }
    }
}

/// A writer that answers set options and remembers what it was asked,
/// for tests. It never sends anything.
#[cfg(any(test, feature = "test-util"))]
#[derive(Debug, Default)]
pub struct FakeWriter {
    options: Vec<String>,
    asked: std::sync::Mutex<Vec<AiAsk>>,
}

#[cfg(any(test, feature = "test-util"))]
impl FakeWriter {
    /// A writer that answers `options` to every request.
    pub fn answering(options: &[&str]) -> Self {
        Self {
            options: options.iter().map(|&option| option.to_owned()).collect(),
            asked: std::sync::Mutex::default(),
        }
    }

    /// Every request it was sent, in order.
    pub fn asked(&self) -> Vec<AiAsk> {
        self.asked
            .lock()
            .map(|asked| asked.clone())
            .unwrap_or_default()
    }
}

#[cfg(any(test, feature = "test-util"))]
impl AiWriter for FakeWriter {
    fn write<'a>(&'a self, ask: &'a AiAsk) -> BoxFuture<'a, Result<Vec<String>, AiError>> {
        if let Ok(mut asked) = self.asked.lock() {
            asked.push(ask.clone());
        }
        let options = self.options.iter().take(usize::from(ask.count));
        let options = options.cloned().collect();
        Box::pin(async move { Ok(options) })
    }
}
