//! The site's core settings as the Admin API reads and changes them.

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use utoipa::ToSchema;

use super::TemplateSetting;

/// The site's core settings from `site.toml`, in Admin API shape. The
/// running server puts most of a saved change in use at once; the settings
/// it reads only at start wait for a restart.
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
    /// How the site reaches readers: `server`, `static` or `headless`.
    /// It is read here and changed only in `site.toml`.
    pub mode: String,
    /// The template the site uses and the options it sets.
    pub template: TemplateSetting,
    /// The `[seo]` table as `site.toml` writes it, one key per part such
    /// as `titles` or `crawlers`. A setting left out of the file is left
    /// out here too and keeps its default; `docs/seo-settings.md` lists
    /// every part and its defaults.
    #[schema(value_type = Object)]
    pub seo: Map<String, Value>,
    /// The `[privacy]` table as `site.toml` writes it, one key per part
    /// such as `lifetime_months`, `gpc`, `counts`, `external_tool`, `text`
    /// or `contact`. A setting left out keeps its default.
    #[schema(value_type = Object)]
    pub privacy: Map<String, Value>,
    /// The site's own code from `[code]`, one object per entry as written:
    /// `head`, `body`, the `category` it waits for (`preferences`,
    /// `analytics`, `marketing` or `no-tracking`), the `vendor` and its
    /// `policy` link. An entry with no `category` counts as Marketing. The
    /// old single `[code]` table reads as one entry.
    #[schema(value_type = Vec<Object>)]
    pub code: Vec<Map<String, Value>>,
    /// The home page's sections from `[reader] home_sections`, top first.
    /// Empty means one list of the newest posts.
    pub home_sections: Vec<HomeSectionSetting>,
    /// The header and footer menus from `[nav]`.
    pub nav: NavSetting,
    /// When `site.toml` last changed. A change sends it back.
    pub updated_at: DateTime<Utc>,
    /// Whether `site.toml` changes a setting the running server reads only
    /// at start, such as `mount`, `mode`, `address`, `languages` or
    /// `[code]`, so a restart is needed to apply it. A change to `name`,
    /// `[template]`, `[seo]` or `[privacy]` alone is in use once saved.
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
    /// New parts of the `[privacy]` table, each replacing the part of that
    /// name whole, such as `{"lifetime_months": 12}`. `null` removes a
    /// part, so it goes back to its default. Parts left out stay as they
    /// are.
    #[schema(value_type = Option<Object>)]
    pub privacy: Option<BTreeMap<String, Option<Value>>>,
    /// Every entry of `[code]`, in place of the old ones, written as a
    /// `[[code]]` list. An empty list removes `[code]`. A list that differs
    /// from the saved one needs `admin:all`.
    #[schema(value_type = Option<Vec<Object>>)]
    pub code: Option<Vec<Map<String, Value>>>,
    /// Every home section, top first, in place of the old ones. An empty
    /// list goes back to one list of the newest posts.
    pub home_sections: Option<Vec<HomeSectionSetting>>,
    /// New menus. Each menu sent replaces the old one whole; a menu left
    /// out stays as it is.
    pub nav: Option<NavChange>,
    /// The `updated_at` of the copy this change was made on.
    pub updated_at: DateTime<Utc>,
}

/// The menus every page shows, from `[nav]` in `site.toml`. The footer's
/// columns, `[[nav.footer_groups]]`, are changed only in the file.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, ToSchema)]
pub struct NavSetting {
    /// The main menu in the header, in the order shown.
    pub primary: Vec<MenuLinkSetting>,
    /// The menu in the footer, in the order shown.
    pub footer: Vec<MenuLinkSetting>,
}

/// New menus for `[nav]`: only the menus sent change.
#[derive(Debug, Clone, Default, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct NavChange {
    /// Every link of the header menu, in order. An empty list removes the
    /// menu.
    pub primary: Option<Vec<MenuLinkSetting>>,
    /// Every link of the footer menu, in order. An empty list removes the
    /// menu.
    pub footer: Option<Vec<MenuLinkSetting>>,
}

/// One link in a menu, such as `{"label": "Guides", "href": "/blog/guides/"}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct MenuLinkSetting {
    /// The words a reader sees, on one line of at most 160 characters.
    pub label: String,
    /// Where it goes: a path on this site, such as `/blog/about/`, or a
    /// full `http` or `https` address.
    pub href: String,
    /// The one language whose pages show the link, such as `de`. Left out,
    /// every page shows it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// One section of the home page: `{"kind": "latest"}`, or a list named
/// by its slug, such as `{"kind": "category", "slug": "technical-seo"}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "kind", rename_all = "lowercase")]
#[non_exhaustive]
pub enum HomeSectionSetting {
    /// The newest posts.
    Latest,
    /// The posts marked `featured`.
    Featured,
    /// The newsletter sign-up form.
    Newsletter,
    /// A category from `categories.toml`.
    Category {
        /// Its slug, such as `technical-seo`.
        slug: String,
    },
    /// A collection, such as `guides`.
    Collection {
        /// Its name, such as `guides`.
        slug: String,
    },
    /// A channel from `routes.toml`.
    Channel {
        /// Its name, such as `featured`.
        slug: String,
    },
    /// A series, in reading order.
    Series {
        /// The slug of its page, such as `link-building-101`.
        slug: String,
    },
}
