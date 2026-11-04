//! `/settings/` in Ghost's shape, from `site.toml`.

use khalid_core::{Config, MenuLink};
use serde::Serialize;

/// Ghost's time zone for every Khalid site: times are kept in UTC.
const TIME_ZONE: &str = "Etc/UTC";

/// The site as Ghost's `/settings/` describes it, made once from the
/// site's settings when the router is built.
#[derive(Debug, Clone, Serialize)]
pub struct GhostSite {
    title: String,
    description: String,
    logo: Option<String>,
    icon: Option<String>,
    accent_color: Option<String>,
    cover_image: Option<String>,
    facebook: Option<String>,
    twitter: Option<String>,
    lang: String,
    locale: String,
    timezone: &'static str,
    codeinjection_head: Option<String>,
    codeinjection_foot: Option<String>,
    navigation: Vec<GhostLink>,
    secondary_navigation: Vec<GhostLink>,
    meta_title: Option<String>,
    meta_description: Option<String>,
    url: String,
}

/// One menu link as Ghost writes it.
#[derive(Debug, Clone, Serialize)]
struct GhostLink {
    label: String,
    url: String,
}

impl GhostSite {
    /// The settings Ghost's front ends read, from `config`: the name, the
    /// tagline as the description, the publisher's logo, the share image
    /// as the cover, the X account, the default language, the header menu
    /// as `navigation` and the footer menu as `secondary_navigation`.
    pub fn from_config(config: &Config) -> Self {
        let seo = config.seo();
        let language = config.default_language();
        let links = |links: &[MenuLink]| -> Vec<GhostLink> {
            let shown = links
                .iter()
                .filter(|link| link.language.as_ref().is_none_or(|only| only == language));
            shown
                .map(|link| GhostLink {
                    label: link.label.as_str().to_owned(),
                    url: link.href.as_str().to_owned(),
                })
                .collect()
        };
        let tagline = seo.titles.tagline.trim();
        let address = config.address().as_str().trim_end_matches('/');
        Self {
            title: config.name().to_owned(),
            description: tagline.to_owned(),
            logo: seo.publisher.logo.as_ref().map(|logo| logo.url.to_string()),
            icon: None,
            accent_color: None,
            cover_image: seo.social.image.as_ref().map(|image| image.url.to_string()),
            facebook: None,
            twitter: seo.social.twitter.clone(),
            lang: language.as_str().to_owned(),
            locale: language.as_str().to_owned(),
            timezone: TIME_ZONE,
            codeinjection_head: None,
            codeinjection_foot: None,
            navigation: links(&config.nav().primary),
            secondary_navigation: links(&config.nav().footer),
            meta_title: None,
            meta_description: (!tagline.is_empty()).then(|| tagline.to_owned()),
            url: format!("{address}{}", config.mount()),
        }
    }
}
