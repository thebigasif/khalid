//! The site's core settings as the Admin API reads and changes them.

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use utoipa::ToSchema;

use super::TemplateSetting;

/// The site's core settings from `site.toml`, in Admin API shape. The
/// running server reads `site.toml` once at start, so a change applies
/// after a restart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct AdminSettings {
    /// The site's name.
    pub name: String,
    /// The address of the main site, such as `https://www.example.com`.
    pub address: String,
    /// The path the blog lives under, such as `/blog/`.
    pub mount: String,
    /// The language a reader gets first, such as `en`.
    pub default_language: String,
    /// Every language the site is written in.
    pub languages: Vec<String>,
    /// The template the site uses and the options it sets.
    pub template: TemplateSetting,
    /// The `[seo]` table as `site.toml` writes it, one key per part such
    /// as `titles` or `crawlers`. A setting left out of the file is left
    /// out here too and keeps its default; `docs/seo-settings.md` lists
    /// every part and its defaults.
    #[schema(value_type = Object)]
    pub seo: Map<String, Value>,
    /// When `site.toml` last changed. A change sends it back.
    pub updated_at: DateTime<Utc>,
    /// Whether `site.toml` says something the running server does not use
    /// yet, so a restart is needed to apply it.
    pub restart_needed: bool,
}

/// A change to the core settings: only the fields sent change.
#[derive(Debug, Clone, Default, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SettingsChange {
    /// A new name.
    pub name: Option<String>,
    /// A new address of the main site.
    pub address: Option<String>,
    /// A new path for the blog, such as `/blog/`.
    pub mount: Option<String>,
    /// A new default language, which `languages` must include.
    pub default_language: Option<String>,
    /// A new list of languages.
    pub languages: Option<Vec<String>>,
    /// A new template with the options it sets. It replaces the template's
    /// name and every option; the call to action, `[template.cta]`, stays.
    pub template: Option<TemplateSetting>,
    /// New parts of the `[seo]` table, each replacing the part of that
    /// name whole, such as `{"titles": {"separator": "·"}}`. `null`
    /// removes a part, so its settings go back to their defaults. Parts
    /// left out stay as they are.
    #[schema(value_type = Option<Object>)]
    pub seo: Option<BTreeMap<String, Option<Value>>>,
    /// The `updated_at` of the copy this change was made on.
    pub updated_at: DateTime<Utc>,
}
