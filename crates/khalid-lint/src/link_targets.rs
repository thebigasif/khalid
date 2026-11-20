//! Rules about where a post's links lead: old addresses, addresses that
//! answer nothing, and sister sites.

use crate::link_graph::page_key;
use crate::link_hygiene::{bare_host, web_host};
use crate::{Hit, LinkKind, LintLink, LintPost, Rule, RuleId, Severity, SiteView};

/// An internal link to an old address that now redirects: an old path of a
/// post, listed in its `aliases`, or a move stored in the site's database.
/// Suggests where it now leads. Outside redirects need a fetch, so
/// `khalid audit` checks those.
#[derive(Debug)]
pub(crate) struct RedirectingLink;

impl Rule for RedirectingLink {
    fn id(&self) -> RuleId {
        RuleId::new("redirecting-link")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, site: &dyn SiteView) -> Vec<Hit> {
        post.links
            .iter()
            .filter(|link| link.kind == LinkKind::Internal)
            .filter_map(|link| {
                let current = moved_to(page_key(&link.target), site)?;
                Some(Hit::about(link.target.clone()).suggesting(current))
            })
            .collect()
    }
}

/// Where the old address `page` now leads, by a post's `aliases` or a
/// stored move: the current address, or none when the post has none yet.
/// `None` when `page` has not moved.
fn moved_to(page: &str, site: &dyn SiteView) -> Option<Vec<String>> {
    let by_alias = site
        .posts()
        .iter()
        .find(|other| other.aliases.iter().any(|alias| page_key(alias) == page));
    if let Some(moved) = by_alias {
        return Some(moved.path.iter().cloned().collect());
    }
    let stored = site.redirects().iter().find(|r| page_key(&r.from) == page)?;
    Some(vec![stored.to.clone()])
}

/// A link to an address on the site that answers nothing: a plain path, or
/// a full address on the site's own host, in a folder that holds posts or
/// pages, that is no post, page or list the site writes, no folder above
/// one, no file and no old address that redirects. Runs only where the
/// site knows its addresses.
#[derive(Debug)]
pub(crate) struct BrokenInternalLink;

impl Rule for BrokenInternalLink {
    fn id(&self) -> RuleId {
        RuleId::new("broken-internal-link")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, site: &dyn SiteView) -> Vec<Hit> {
        let Some(known) = site.known_paths() else {
            return Vec::new();
        };
        let host = site.host().map(bare_host);
        post.links
            .iter()
            .filter(|link| {
                let Some(path) = site_path(link, host.as_deref()) else {
                    return false;
                };
                let page = page_key(path.split('?').next().unwrap_or_default());
                let (folder, last) = page.rsplit_once('/').unwrap_or_default();
                let below = format!("{page}/");
                let above_one = (known.paths.range(below.clone()..).next())
                    .is_some_and(|path| path.starts_with(&below));
                known.folders.contains(folder)
                    && !last.contains('.')
                    && !known.paths.contains(page)
                    && !above_one
                    && moved_to(page, site).is_none()
            })
            .map(|link| Hit::about(link.target.clone()))
            .collect()
    }
}

/// The path of `link` from the site root, when it stays on the site: an
/// internal link written as a path, or a full address on `host`, the
/// site's own host without `www.`. Relative links are left alone.
fn site_path<'l>(link: &'l LintLink, host: Option<&str>) -> Option<&'l str> {
    let target = link.target.as_str();
    if link.kind == LinkKind::Internal && target.starts_with('/') && !target.starts_with("//") {
        return Some(target);
    }
    let on = web_host(target)?;
    if Some(bare_host(on).as_str()) != host {
        return None;
    }
    let (_, rest) = target.split_once("://")?;
    rest.find('/').and_then(|at| rest.get(at..))
}

/// A link to a sister site served by the same Khalid process, over either
/// scheme and with or without `www.`.
#[derive(Debug)]
pub(crate) struct CrossSiteLink;

impl Rule for CrossSiteLink {
    fn id(&self) -> RuleId {
        RuleId::new("cross-site-link")
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, post: &LintPost, site: &dyn SiteView) -> Vec<Hit> {
        let sisters: Vec<String> = site.sibling_hosts().iter().map(|h| bare_host(h)).collect();
        if sisters.is_empty() {
            return Vec::new();
        }
        post.links
            .iter()
            .filter(|link| {
                web_host(&link.target).is_some_and(|host| sisters.contains(&bare_host(host)))
            })
            .map(|link| Hit::about(link.target.clone()))
            .collect()
    }
}
