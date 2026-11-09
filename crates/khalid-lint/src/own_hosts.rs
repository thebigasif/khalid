//! The hosts that are the site's own: the host of its address, the `www.`
//! and apex pair of its domain, and the hosts `site.toml` lists in
//! `internal_domains`, with their subdomains (audit 2, G5).

use crate::SiteView;
use crate::link_hygiene::bare_host;

/// Second-level names under a two-letter country ending, such as `co` in
/// `example.co.uk`, where a domain keeps three labels.
const SECOND_LEVELS: &[&str] = &["ac", "co", "com", "edu", "gov", "ne", "net", "or", "org"];

/// Domains whose subdomains belong to different people, such as
/// `name.github.io`, so a site on one has no apex of its own.
const SHARED: &[&str] = &[
    "blogspot.com",
    "fly.dev",
    "github.io",
    "gitlab.io",
    "herokuapp.com",
    "netlify.app",
    "onrender.com",
    "pages.dev",
    "substack.com",
    "vercel.app",
    "wordpress.com",
    "workers.dev",
];

/// The site's own hosts, read once per post.
#[derive(Debug)]
pub(crate) struct OwnHosts {
    /// The host of the site's address, without `www.`, when known.
    own: Option<String>,
    /// The apex of that host's domain, such as `example.com` for
    /// `blog.example.com`, when it has one.
    apex: Option<String>,
    /// The hosts the site lists as its own, in lowercase.
    listed: Vec<String>,
}

impl OwnHosts {
    /// The own hosts of `site`.
    pub(crate) fn of(site: &dyn SiteView) -> Self {
        let own = site.host().map(bare_host);
        let apex = own.as_deref().and_then(apex);
        let listed = site
            .own_hosts()
            .iter()
            .map(|host| host.to_ascii_lowercase())
            .collect();
        Self { own, apex, listed }
    }

    /// Whether `host`, as a link writes it, is one of the site's own: its
    /// address's host or the apex, each with or without `www.`, or a listed
    /// host or one of its subdomains.
    pub(crate) fn hold(&self, host: &str) -> bool {
        let bare = bare_host(host);
        let lower = host.to_ascii_lowercase();
        self.own.as_deref() == Some(bare.as_str())
            || self.apex.as_deref() == Some(bare.as_str())
            || self.listed.iter().any(|listed| {
                lower == *listed
                    || lower
                        .strip_suffix(listed.as_str())
                        .is_some_and(|rest| rest.ends_with('.'))
            })
    }
}

/// The apex of `host`'s domain, the name its owner registered, such as
/// `example.com` for `blog.example.com` and `example.co.uk` for
/// `www.example.co.uk`. `None` for an address by number, a single name such
/// as `localhost`, and a domain on [`SHARED`].
fn apex(host: &str) -> Option<String> {
    let labels: Vec<&str> = host.split('.').collect();
    let keep = match labels.as_slice() {
        [.., tld] if tld.is_empty() || tld.bytes().all(|b| b.is_ascii_digit()) => return None,
        [.., second, tld] if tld.len() == 2 && SECOND_LEVELS.contains(second) => 3,
        _ => 2,
    };
    let apex = labels.get(labels.len().checked_sub(keep)?..)?.join(".");
    (!SHARED.contains(&apex.as_str())).then_some(apex)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LintSite;

    #[test]
    fn a_domain_keeps_the_labels_its_owner_registered() {
        assert_eq!(apex("blog.example.com").as_deref(), Some("example.com"));
        assert_eq!(apex("example.com").as_deref(), Some("example.com"));
        assert_eq!(apex("a.b.example.co.uk").as_deref(), Some("example.co.uk"));
        assert_eq!(apex("example.de").as_deref(), Some("example.de"));
    }

    #[test]
    fn shared_domains_numbers_and_single_names_have_no_apex() {
        assert_eq!(apex("name.github.io"), None);
        assert_eq!(apex("127.0.0.1"), None);
        assert_eq!(apex("localhost"), None);
        assert_eq!(apex("co.uk"), None);
    }

    #[test]
    fn the_own_hosts_are_the_address_its_apex_and_the_listed_hosts() {
        let site = LintSite {
            host: Some("blog.example.com".to_owned()),
            own_hosts: vec!["Shop.Example.net".to_owned()],
            ..LintSite::default()
        };
        let own = OwnHosts::of(&site);
        for host in [
            "blog.example.com",
            "WWW.Blog.Example.com",
            "example.com",
            "www.example.com",
            "shop.example.net",
            "eu.shop.example.net",
        ] {
            assert!(own.hold(host), "{host} is the site's own");
        }
        for host in ["forum.example.com", "myshop.example.net", "example.org"] {
            assert!(!own.hold(host), "{host} is another site");
        }
    }

    #[test]
    fn a_site_with_no_address_owns_only_its_listed_hosts() {
        let site = LintSite {
            own_hosts: vec!["example.com".to_owned()],
            ..LintSite::default()
        };
        let own = OwnHosts::of(&site);
        assert!(own.hold("example.com"));
        assert!(!own.hold("example.org"));
    }
}
