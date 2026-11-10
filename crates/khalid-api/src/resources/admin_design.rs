//! The site's colours, fonts, shape and logo as the Admin API's Design
//! route reads and changes them.

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The values the site's pages use from `theme/tokens.toml`, each over the
/// chosen template's own, with the contrast of each colour pair.
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub struct AdminDesign {
    /// Every token by its name in the file, such as `colors.link`, with
    /// the value pages use: the file's, else the template's, else the
    /// default. A logo or icon that is not set is an empty text.
    #[schema(example = json!({"colors.link": "#0b57d0", "fonts.body": "system-ui, sans-serif", "brand.logo": ""}))]
    pub tokens: BTreeMap<String, String>,
    /// The tokens `khalid theme sync` keeps as they are, such as
    /// `colors.link`.
    pub locked: Vec<String>,
    /// The fonts a body or heading may use: the system fonts, each family
    /// the site serves itself, and the ones the file names now.
    pub fonts: Vec<FontChoice>,
    /// Each colour on the background and on the surface, with its WCAG
    /// contrast.
    pub contrast: Vec<ContrastPair>,
    /// When `theme/tokens.toml` last changed, or `null` when the site has
    /// no such file yet. A change sends it back.
    pub updated_at: Option<DateTime<Utc>>,
}

/// One font a body or heading may use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct FontChoice {
    /// The name a person reads, such as `System font` or `Inter`.
    pub label: String,
    /// The CSS font list the file writes for it.
    pub stack: String,
}

/// One colour pair the templates draw, with its contrast.
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub struct ContrastPair {
    /// The colour drawn, such as `colors.link`.
    pub token: String,
    /// What it is drawn on: `colors.background` or `colors.surface`.
    pub ground: String,
    /// The contrast, such as `6.3` for 6.3 to 1, rounded down so a failing
    /// 4.47 never reads as a passing 4.5.
    pub ratio: f64,
    /// What WCAG 2.2 AA asks: 4.5 for text, 3 for buttons and highlights.
    pub target: f64,
    /// Whether the ratio meets the target.
    pub passes: bool,
    /// For a colour that fails, the nearest shade that passes on both
    /// grounds, such as `#2563eb`.
    pub nearest: Option<String>,
}

/// A change to the design: only the tokens sent change.
#[derive(Debug, Clone, Default, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DesignChange {
    /// New values by token name, such as `{"colors.link": "#b3261e"}`. An
    /// empty logo or icon takes it out of the file.
    #[serde(default)]
    #[schema(example = json!({"colors.link": "#b3261e"}))]
    pub tokens: BTreeMap<String, String>,
    /// The whole list of tokens a sync keeps, when it changes.
    #[serde(default)]
    pub locked: Option<Vec<String>>,
    /// The `updated_at` of the copy the change was made on, `null` when
    /// the site had no file.
    pub updated_at: Option<DateTime<Utc>>,
}
