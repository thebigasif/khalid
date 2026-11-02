//! A person a post names: an author or a reviewer, and their profiles
//! elsewhere.

use khalid_core::AbsoluteUrl;
use serde::Serialize;

use crate::{BodyHtml, RenderImage};

/// An author as a post shows them: in the byline, and in the author box at
/// the end.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Person {
    /// The name a reader sees.
    pub name: String,
    /// Their author page, such as `/blog/authors/lena-vogt/`.
    pub href: String,
    /// What they do, such as `Technical SEO editor`.
    pub role: Option<String>,
    /// Why to trust them, such as `Ten years running crawls for news sites`.
    pub credentials: Option<String>,
    /// A small photo, always lazy.
    pub image: Option<RenderImage>,
    /// A short bio for the author box, already sanitised.
    pub bio: Option<BodyHtml>,
    /// Their profiles elsewhere, marked `rel="me"` on their own page.
    pub profiles: Vec<Profile>,
}

/// A profile of a person on another site, with the icon that site is
/// known by.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Profile {
    /// What a reader sees or hears for the link, such as `LinkedIn`, or
    /// the site's host for one without a known icon.
    pub name: String,
    /// The profile's address.
    pub href: String,
    /// The icon drawn for it. `None` draws the name as a plain link.
    pub icon: Option<ProfileIcon>,
}

/// The sites whose icon a profile link shows, drawn as inline SVG by the
/// `profile_icon` macro in `icons.html`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum ProfileIcon {
    /// `linkedin.com`.
    Linkedin,
    /// `x.com`, and `twitter.com` before it.
    X,
    /// `github.com`.
    Github,
    /// `youtube.com` and `youtu.be`.
    Youtube,
    /// `instagram.com`.
    Instagram,
    /// `facebook.com`.
    Facebook,
    /// A Mastodon server, known by its name or an `@name` address.
    Mastodon,
    /// A personal site: a whole site rather than a page on one.
    Website,
}

/// The known sites, by the host they answer on, and the name each shows.
const KNOWN: [(&str, ProfileIcon, &str); 9] = [
    ("linkedin.com", ProfileIcon::Linkedin, "LinkedIn"),
    ("x.com", ProfileIcon::X, "X"),
    ("twitter.com", ProfileIcon::X, "X"),
    ("github.com", ProfileIcon::Github, "GitHub"),
    ("youtube.com", ProfileIcon::Youtube, "YouTube"),
    ("youtu.be", ProfileIcon::Youtube, "YouTube"),
    ("instagram.com", ProfileIcon::Instagram, "Instagram"),
    ("facebook.com", ProfileIcon::Facebook, "Facebook"),
    ("mastodon.social", ProfileIcon::Mastodon, "Mastodon"),
];

impl Profile {
    /// The profile at `url`, with the icon of the site it is on: a known
    /// network's own, a globe for a personal site's home page, and none
    /// for any other page.
    #[must_use]
    pub fn of(url: &AbsoluteUrl) -> Self {
        let host = url.host().trim_start_matches("www.");
        let on = |site: &str| host == site || host.ends_with(&format!(".{site}"));
        let segments = url.segments();
        let known = KNOWN.iter().find(|(site, _, _)| on(site));
        let (icon, name) = match known {
            Some(&(_, icon, name)) => (Some(icon), name.to_owned()),
            None if host.contains("mastodon")
                || segments.first().is_some_and(|first| first.starts_with('@')) =>
            {
                (Some(ProfileIcon::Mastodon), "Mastodon".to_owned())
            }
            None if segments.is_empty() => (Some(ProfileIcon::Website), host.to_owned()),
            None => (None, host.to_owned()),
        };
        Self {
            name,
            href: url.as_str().to_owned(),
            icon,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn of(text: &str) -> Profile {
        Profile::of(&AbsoluteUrl::parse(text).unwrap())
    }

    #[test]
    fn a_known_network_gets_its_icon_and_name() {
        let cases = [
            (
                "https://www.linkedin.com/in/lena",
                ProfileIcon::Linkedin,
                "LinkedIn",
            ),
            ("https://x.com/lena", ProfileIcon::X, "X"),
            ("https://twitter.com/lena", ProfileIcon::X, "X"),
            ("https://github.com/lena", ProfileIcon::Github, "GitHub"),
            (
                "https://www.youtube.com/@lena",
                ProfileIcon::Youtube,
                "YouTube",
            ),
            (
                "https://www.instagram.com/lena/",
                ProfileIcon::Instagram,
                "Instagram",
            ),
            (
                "https://m.facebook.com/lena",
                ProfileIcon::Facebook,
                "Facebook",
            ),
            (
                "https://fosstodon.org/@lena",
                ProfileIcon::Mastodon,
                "Mastodon",
            ),
        ];
        for (href, icon, name) in cases {
            let profile = of(href);
            assert_eq!(profile.icon, Some(icon), "{href}");
            assert_eq!(profile.name, name, "{href}");
            assert_eq!(profile.href, href);
        }
    }

    #[test]
    fn a_home_page_is_a_personal_site_and_any_other_page_a_plain_link() {
        let site = of("https://www.lenavogt.example/");
        assert_eq!(site.icon, Some(ProfileIcon::Website));
        assert_eq!(site.name, "lenavogt.example");
        let page = of("https://speakerdeck.com/lena");
        assert_eq!(page.icon, None);
        assert_eq!(page.name, "speakerdeck.com");
    }
}
