//! Who writes the editor's suggestions, from the `[ai]` table of
//! `site.toml` (Yoast package WP4).
//!
//! Khalid holds no model of its own. The editor's "Suggest" buttons send a
//! post's words to the service named here, and only when the owner names
//! one: without the table nothing is sent anywhere.

use serde::{Deserialize, Serialize};

use crate::AbsoluteUrl;

/// Which service writes suggestions, and with which model. Off unless the
/// table names a provider:
///
/// ```toml
/// [ai]
/// provider = "anthropic"          # or "serpdad", "openai", "off"
/// model = "claude-haiku-5-5"      # optional for anthropic and serpdad
/// endpoint = "https://api.example.com/v1/chat/completions"  # optional
/// ```
///
/// The key the service is asked with never goes in the file: it comes from
/// the environment, see [`crate::Environment::ai_key`].
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct AiSettings {
    /// The service that writes suggestions, `off` unless set.
    pub provider: AiProvider,
    /// The model's name as the service spells it. Anthropic and `SerpDad`
    /// choose one when it is left out; an OpenAI-compatible service needs
    /// it.
    pub model: Option<String>,
    /// Where requests go, when not the provider's usual address: a proxy,
    /// or any service that speaks `OpenAI`'s chat completions.
    pub endpoint: Option<AbsoluteUrl>,
}

/// The services that can write suggestions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum AiProvider {
    /// Nothing is sent, and the editor shows no "Suggest" button.
    #[default]
    Off,
    /// The site's `SerpDad` account, through the Connect to `SerpDad`
    /// page. It works only while that connection is on.
    Serpdad,
    /// Anthropic's Messages API, with the owner's own key.
    Anthropic,
    /// Any service that speaks `OpenAI`'s chat completions, with the
    /// owner's own key.
    Openai,
}

impl AiProvider {
    /// The provider's name as people know it, for labels.
    pub fn label(self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::Serpdad => "SerpDad",
            Self::Anthropic => "Anthropic",
            Self::Openai => "OpenAI-compatible service",
        }
    }
}

impl AiSettings {
    /// Whether the owner turned suggestions on.
    pub fn is_on(&self) -> bool {
        self.provider != AiProvider::Off
    }

    /// The key that is wrong and why.
    pub(crate) fn mistake(&self) -> Option<(&'static str, &'static str)> {
        if self
            .model
            .as_deref()
            .is_some_and(|model| model.trim().is_empty())
        {
            return Some((
                "model",
                "a model's name cannot be blank; remove the line to use the provider's usual model",
            ));
        }
        if self.provider == AiProvider::Openai && self.model.is_none() {
            return Some((
                "provider",
                "an OpenAI-compatible service needs model, the model's name as the service spells it",
            ));
        }
        None
    }
}
