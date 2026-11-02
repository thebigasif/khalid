//! Tracking tags on links to the site owner's own products: `utm_source`,
//! `utm_medium` and `utm_campaign`, and never on any other link.

use khalid_core::{AbsoluteUrl, LinkTracking};

use crate::Domain;

/// The tags `[seo.links.tracking]` gives links to the product's domains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tracking {
    domains: Vec<Domain>,
    source: String,
    medium: String,
    campaign: Option<String>,
}

impl From<&LinkTracking> for Tracking {
    fn from(settings: &LinkTracking) -> Self {
        Self {
            domains: (settings.domains.iter())
                .filter_map(|name| Domain::try_new(name.as_str()).ok())
                .collect(),
            source: settings.source.to_string(),
            medium: settings.medium.to_string(),
            campaign: settings.campaign.as_ref().map(ToString::to_string),
        }
    }
}

impl Tracking {
    /// `href` with the three tags added, the campaign set in `site.toml` or
    /// else `campaign`, or `None` when `href` is not an `http` or `https`
    /// link to one of the product's domains, or already has `utm_source`.
    /// A `#section` stays at the end.
    pub fn href(&self, href: &str, campaign: &str) -> Option<String> {
        let url = AbsoluteUrl::parse(href).ok()?;
        let web = href.starts_with("https://") || href.starts_with("http://");
        let ours = self.domains.iter().any(|domain| domain.covers(url.host()));
        if !web || !ours || url.query_value("utm_source").is_some() {
            return None;
        }
        let (address, section) = href
            .split_once('#')
            .map_or((href, None), |(a, s)| (a, Some(s)));
        let joint = if address.contains('?') { '&' } else { '?' };
        let campaign = self.campaign.as_deref().unwrap_or(campaign);
        let mut tagged = format!(
            "{address}{joint}utm_source={}&utm_medium={}&utm_campaign={campaign}",
            self.source, self.medium
        );
        if let Some(section) = section {
            tagged.push('#');
            tagged.push_str(section);
        }
        Some(tagged)
    }

    /// `html` with the `href` of each `<a>` tag to the product tagged, by
    /// [`Tracking::href`]. `html` is a rendered body, whose attribute
    /// values are quoted with `"` and write `&` as `&amp;`.
    pub fn apply_to_html(&self, html: &str, campaign: &str) -> String {
        let mut out = String::with_capacity(html.len() + 256);
        let mut rest = html;
        while let Some(start) = link_value(rest) {
            let (before, from_value) = rest.split_at(start);
            out.push_str(before);
            let Some(end) = from_value.find('"') else {
                rest = from_value;
                break;
            };
            let (value, after) = from_value.split_at(end);
            match self.href(&value.replace("&amp;", "&"), campaign) {
                Some(tagged) => out.push_str(&tagged.replace('&', "&amp;")),
                None => out.push_str(value),
            }
            rest = after;
        }
        out.push_str(rest);
        out
    }
}

/// Where the `href` value of the first `<a>` tag in `html` with one
/// starts, such as in `<a class="button" href="...">`.
fn link_value(html: &str) -> Option<usize> {
    let mut from = 0;
    loop {
        let tag = from + html.get(from..)?.find("<a ")?;
        let end = tag + html.get(tag..)?.find('>')?;
        let inside = html.get(tag..end)?;
        if let Some(at) = inside.find(" href=\"") {
            return Some(tag + at + " href=\"".len());
        }
        from = end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tracking(campaign: Option<&str>) -> Tracking {
        Tracking {
            domains: vec![Domain::try_new("example.com").unwrap()],
            source: "blog".to_owned(),
            medium: "content".to_owned(),
            campaign: campaign.map(str::to_owned),
        }
    }

    #[test]
    fn a_link_to_the_product_gets_the_three_tags_before_its_section() {
        let tags = tracking(None);
        assert_eq!(
            tags.href(
                "https://app.example.com/signup?plan=pro#form",
                "anchor-text"
            )
            .as_deref(),
            Some(
                "https://app.example.com/signup?plan=pro&utm_source=blog&utm_medium=content&utm_campaign=anchor-text#form"
            )
        );
        assert_eq!(
            tracking(Some("spring"))
                .href("https://example.com/", "anchor-text")
                .as_deref(),
            Some("https://example.com/?utm_source=blog&utm_medium=content&utm_campaign=spring")
        );
    }

    #[test]
    fn no_other_link_and_no_tagged_link_gets_a_tag() {
        let tags = tracking(None);
        for href in [
            "https://other.com/",
            "https://notexample.com/",
            "/blog/post/",
            "mailto:hi@example.com",
            "https://example.com/?utm_source=newsletter",
        ] {
            assert_eq!(tags.href(href, "post"), None, "{href}");
        }
    }

    #[test]
    fn a_body_has_only_its_product_links_tagged() {
        let html = "<p><a href=\"https://app.example.com/a?x=1&amp;y=2\">Try</a> or <a href=\"https://other.com/\">read</a>.</p><p><a class=\"button\" href=\"https://example.com/\">Start</a></p>";
        assert_eq!(
            tracking(None).apply_to_html(html, "post"),
            "<p><a href=\"https://app.example.com/a?x=1&amp;y=2&amp;utm_source=blog&amp;utm_medium=content&amp;utm_campaign=post\">Try</a> or <a href=\"https://other.com/\">read</a>.</p><p><a class=\"button\" href=\"https://example.com/?utm_source=blog&amp;utm_medium=content&amp;utm_campaign=post\">Start</a></p>"
        );
        let plain = "<p>No links, and <abbr title=\"x\">a</abbr> tag.</p>";
        assert_eq!(tracking(None).apply_to_html(plain, "post"), plain);
    }
}
