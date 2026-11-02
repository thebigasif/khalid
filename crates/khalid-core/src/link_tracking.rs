//! [`LinkTracking`]: the tracking tags links to the site owner's own
//! products get, from the `[seo.links.tracking]` table of `site.toml`.

use std::fmt;

use serde::Deserialize;

use crate::DomainName;

/// Adds `utm_source`, `utm_medium` and `utm_campaign` to every link in a
/// post or page, and to the call to action, that points to one of
/// `domains` or their subdomains, so the product's analytics show which
/// post sent each visitor. No other link ever gets a tag, and a link that
/// already has `utm_source` keeps its own:
///
/// ```toml
/// [seo.links.tracking]
/// domains = ["app.example.com"]
/// source = "blog"
/// medium = "content"
/// ```
///
/// Without `campaign`, a link in a post takes the post's slug, a link in
/// any other page takes `blog`, and the call to action takes `cta`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct LinkTracking {
    /// The product's domains, each with its subdomains.
    pub domains: Vec<DomainName>,
    /// The `utm_source` value, `blog` when not set.
    #[serde(default = "blog")]
    pub source: TagValue,
    /// The `utm_medium` value, `content` when not set.
    #[serde(default = "content")]
    pub medium: TagValue,
    /// The `utm_campaign` value for every link, or `None` to name each
    /// link's page.
    #[serde(default)]
    pub campaign: Option<TagValue>,
}

/// One tag's value: 1 to 64 ASCII letters, digits, `-`, `_` and `.`, so
/// it goes into an address as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagValue(String);

impl TagValue {
    /// Checks `text`.
    ///
    /// # Errors
    ///
    /// Returns [`ParseTagValueError`] for an empty or longer value, or one
    /// with any other character.
    pub fn try_new(text: &str) -> Result<Self, ParseTagValueError> {
        let plain = |c: char| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.');
        if (1..=64).contains(&text.len()) && text.chars().all(plain) {
            Ok(Self(text.to_owned()))
        } else {
            Err(ParseTagValueError {
                text: text.to_owned(),
            })
        }
    }

    /// The value as written.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for TagValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Self::try_new(&text).map_err(serde::de::Error::custom)
    }
}

/// A tracking tag's value with a character an address cannot hold as is.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error(
    "\"{text}\" cannot be a tracking tag: write 1 to 64 letters, digits, hyphens, underscores or dots"
)]
pub struct ParseTagValueError {
    text: String,
}

impl fmt::Display for TagValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn blog() -> TagValue {
    TagValue("blog".to_owned())
}

fn content() -> TagValue {
    TagValue("content".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_table_with_domains_alone_takes_the_default_tags() {
        let tracking: LinkTracking = toml::from_str("domains = [\"app.example.com\"]").unwrap();
        assert_eq!(tracking.source.as_str(), "blog");
        assert_eq!(tracking.medium.as_str(), "content");
        assert_eq!(tracking.campaign, None);
        let set: LinkTracking =
            toml::from_str("domains = []\nsource = \"ledger\"\ncampaign = \"spring_26\"").unwrap();
        assert_eq!(set.campaign.unwrap().as_str(), "spring_26");
    }

    #[test]
    fn a_tag_with_a_space_or_an_ampersand_is_refused() {
        for text in ["", "two words", "a&b", "x".repeat(65).as_str()] {
            assert!(TagValue::try_new(text).is_err(), "{text}");
        }
        let error =
            toml::from_str::<LinkTracking>("domains = []\nmedium = \"e mail\"").unwrap_err();
        assert!(
            error.to_string().contains("cannot be a tracking tag"),
            "{error}"
        );
    }
}
