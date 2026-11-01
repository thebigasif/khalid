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

mod brand;
mod color;
mod contrast;
mod css;
mod fetch;
mod fetch_error;
mod font_files;
mod fonts;
mod links;
mod look;
mod pack;
mod palette;
mod properties;
mod resync;
mod site_theme;
mod split;
mod token_name;
mod token_values;
mod tokens;
mod tokens_error;
#[cfg(feature = "test-util")]
mod trial;

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
pub use pack::{OptionKind, OptionSpec, Pack, PackTemplate, classic, packs};
pub use palette::Palette;
pub use properties::CustomProperties;
pub use resync::{ThemeDiff, TokenChange};
pub use site_theme::SiteTheme;
pub use token_name::TokenName;
pub use token_values::{FontStack, Length};
pub use tokens::{BrandImages, Colors, FontFileEntry, Fonts, Shape, ThemeTokens};
pub use tokens_error::TokensError;
