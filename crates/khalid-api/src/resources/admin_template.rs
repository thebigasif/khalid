//! The built-in templates a site chooses from, as the Admin API lists
//! them (templates spec 4.1). The binary fills the list from the engine's
//! packs, so a template another builder adds is listed with no change here.

use serde::Serialize;
use utoipa::ToSchema;

/// One built-in template: what it is for, what it turns on and off, and
/// the options `[template.options]` may set for it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct AdminTemplate {
    /// The name `[template]` uses, such as `classic`.
    #[schema(example = "classic")]
    pub name: String,
    /// One plain sentence on what the template is for.
    pub summary: String,
    /// The reader features the template turns on or off. A key the site
    /// writes in its own `[reader]` table wins.
    pub reader: Vec<ReaderDefault>,
    /// Every option the template takes, in the order its docs list them.
    pub options: Vec<TemplateOptionSpec>,
}

/// One reader feature a template turns on or off.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct ReaderDefault {
    /// The key in `[reader]`, such as `reading_time`.
    #[schema(example = "reading_time")]
    pub key: String,
    /// Whether the template turns it on.
    pub on: bool,
}

/// One option a template takes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct TemplateOptionSpec {
    /// The key in `[template.options]`, such as `contents`.
    #[schema(example = "contents")]
    pub key: String,
    /// One plain sentence on what the option changes.
    pub help: String,
    /// The values it takes, and the one it has when the site sets none.
    pub takes: OptionTakes,
}

/// The values one option takes, and its default.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OptionTakes {
    /// `true` or `false`.
    Switch {
        /// The value when not set.
        default: bool,
    },
    /// One word from `allowed`.
    Choice {
        /// Every word allowed, in the order the docs list them.
        allowed: Vec<String>,
        /// The word when not set.
        default: String,
    },
    /// A whole number from `least` to `most`.
    Count {
        /// The smallest allowed.
        least: i64,
        /// The largest allowed.
        most: i64,
        /// The number when not set.
        default: i64,
    },
    /// Up to `most` lines of text, none when not set.
    Lines {
        /// The most lines allowed.
        most: usize,
    },
    /// One or more words from `allowed`, each once, in the order written.
    Picks {
        /// Every word allowed, in the order the docs list them.
        allowed: Vec<String>,
        /// The words when not set.
        default: Vec<String>,
    },
}
