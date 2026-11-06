//! Theme sync: the blog takes on the look of the main site it lives beside.
//!
//! This crate owns reading a site's home page and stylesheets, finding its
//! colours, fonts, logo and icon, writing them as theme tokens, and fixing
//! text colours that fail WCAG contrast (plan section 4.9). The proxy rule
//! that mounts the blog under a folder lives in `khalid-mount` (ADR 0017).
//!
//! It also holds the template packs, the built-in looks a site picks in
//! `[template]` of `site.toml` (templates spec 4.1).
//!
//! Every request goes through `khalid_outbound::Outbound` (RULES.md
//! BL-RUN-8). The crate must not know about content, posts or the database.

mod almanac;
mod alto;
mod atlas;
mod brand;
mod bulletin;
mod casper;
mod color;
mod compass;
mod contrast;
mod css;
mod dawn;
mod digest;
mod dope;
mod ease;
mod edge;
mod edition;
mod encore;
mod episode;
mod fetch;
mod fetch_error;
mod folio;
mod font_files;
mod fonts;
mod harbor;
mod headline;
mod journal;
mod keystone;
mod ledger;
mod links;
mod london;
mod look;
mod meridian;
mod pack;
mod palette;
pub mod parts;
mod properties;
mod research;
mod respawn;
mod resync;
mod ruby;
mod site_theme;
pub mod slot;
mod solo;
mod source;
mod split;
mod taste;
mod ticker;
mod token_name;
mod token_values;
mod tokens;
mod tokens_error;
#[cfg(feature = "test-util")]
mod trial;
mod wave;
mod wire;
mod woff2;
mod workbench;

pub use brand::Brand;
pub use color::Rgb;
pub use contrast::{ColorToken, ContrastFix, MARK_CONTRAST, TEXT_CONTRAST};
pub use fetch::{
    MAX_CSS_BYTES, MAX_HTML_BYTES, MAX_STYLESHEETS, Origin, SiteSource, Stylesheet, fetch_site,
};
pub use fetch_error::FetchError;
pub use font_files::{FontMetrics, MAX_FONT_BYTES, SavedFont, download, download_and_report};
pub use fonts::{FontHost, FontSource, HeldFont, Typography};
pub use links::{PageStyle, SkipReason, Skipped, page_styles};
pub use look::{CheckedOption, Look, LookError};
pub(crate) use pack::PackText;
pub use pack::{OptionKind, OptionSpec, Pack, PackTemplate, classic, packs};
pub use palette::Palette;
pub use properties::CustomProperties;
pub use resync::{ThemeDiff, TokenChange};
pub use site_theme::SiteTheme;
pub use token_name::TokenName;
pub use token_values::{FontStack, Length};
pub use tokens::{BrandImages, Colors, FontFileEntry, Fonts, Shape, ThemeTokens};
pub use tokens_error::TokensError;
