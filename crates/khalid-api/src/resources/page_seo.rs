//! A page's search settings in Admin API shape: its robots directives,
//! canonical address, feature image and share cards (plan section 4.6).

use khalid_core::{AbsoluteUrl, RobotsDirectives};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::settings_edit::Change;
use super::{FeatureImage, ShareCards, display_option};

/// A page's search settings. Each is `null`, or a card with nothing set,
/// until someone sets it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PageSeo {
    /// The page's own robots directives, such as `noindex`, in place of
    /// the site's.
    #[serde(default, serialize_with = "display_option")]
    #[schema(value_type = Option<String>, example = "noindex")]
    pub robots: Option<RobotsDirectives>,
    /// The address search engines should credit instead of this page.
    #[serde(default, serialize_with = "display_option")]
    #[schema(value_type = Option<String>, example = "https://example.com/original/")]
    pub canonical: Option<AbsoluteUrl>,
    /// The image under the page's heading, also used in link previews.
    #[serde(default)]
    pub feature_image: Option<FeatureImage>,
    /// How the page looks when shared on X and Facebook.
    #[serde(default)]
    pub share_cards: ShareCards,
}

/// New values for some of a page's search settings. A setting left out
/// keeps its value, and `null` clears it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PageSeoEdit {
    /// New robots directives, or `null` for the site's.
    #[serde(default)]
    #[schema(value_type = Option<String>, example = "noindex")]
    robots: Change<RobotsDirectives>,
    /// A new canonical address, or `null` for the page's own.
    #[serde(default)]
    #[schema(value_type = Option<String>, example = "https://example.com/original/")]
    canonical: Change<AbsoluteUrl>,
    /// A new feature image, or `null` for none.
    #[serde(default)]
    #[schema(value_type = Option<FeatureImage>)]
    feature_image: Change<FeatureImage>,
    /// Both share cards; a card left out uses the page's own title,
    /// description and image.
    share_cards: Option<ShareCards>,
}

impl PageSeoEdit {
    /// The settings `seo` becomes after this edit.
    #[must_use]
    pub fn applied_to(self, seo: PageSeo) -> PageSeo {
        PageSeo {
            robots: self.robots.applied_to(seo.robots),
            canonical: self.canonical.applied_to(seo.canonical),
            feature_image: self.feature_image.applied_to(seo.feature_image),
            share_cards: self.share_cards.unwrap_or(seo.share_cards),
        }
    }
}
