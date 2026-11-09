//! Which posts link to which, for the admin's link counts and link
//! suggestions. A link finds its post the way the linter's link graph
//! finds it: by the post's path when the post has one, and otherwise by the
//! last part of the address and the language of the linking post. Links
//! from a post to itself do not count. The map is built once from the
//! site's posts, so counting a post's links costs only its own links.

use std::collections::{BTreeMap, BTreeSet};

use khalid_lint::{LinkKind, LintPost};

/// A link target without its `#section` and trailing slash.
fn page_key(target: &str) -> &str {
    let page = target.split('#').next().unwrap_or_default();
    page.trim_end_matches('/')
}

/// The last part of a page key, a slug for most addresses.
fn last_part(page: &str) -> &str {
    page.rsplit('/').next().unwrap_or_default()
}

/// The internal links between a site's posts, by position in its list.
#[derive(Debug)]
pub struct LinkMap<'site> {
    posts: &'site [LintPost],
    by_path: BTreeMap<&'site str, usize>,
    by_slug: BTreeMap<(&'site str, &'site str), usize>,
    to_page: BTreeMap<&'site str, BTreeSet<usize>>,
    to_slug: BTreeMap<(&'site str, &'site str), BTreeSet<usize>>,
}

impl<'site> LinkMap<'site> {
    /// The map of `posts`' internal links.
    pub fn new(posts: &'site [LintPost]) -> Self {
        let mut map = Self {
            posts,
            by_path: BTreeMap::new(),
            by_slug: BTreeMap::new(),
            to_page: BTreeMap::new(),
            to_slug: BTreeMap::new(),
        };
        for (at, post) in posts.iter().enumerate() {
            match &post.path {
                Some(path) => map.by_path.insert(page_key(path), at),
                None => map
                    .by_slug
                    .insert((post.locale.as_str(), post.slug.as_str()), at),
            };
            for link in posts_links(post) {
                let page = page_key(&link.target);
                map.to_page.entry(page).or_default().insert(at);
                let key = (post.locale.as_str(), last_part(page));
                map.to_slug.entry(key).or_default().insert(at);
            }
        }
        map
    }

    /// The posts of the map.
    pub fn posts(&self) -> &'site [LintPost] {
        self.posts
    }

    /// The position of each post `post` links to, once each, itself left out.
    pub fn outbound(&self, post: &LintPost) -> BTreeSet<usize> {
        posts_links(post)
            .filter_map(|link| {
                let page = page_key(&link.target);
                let slug = (post.locale.as_str(), last_part(page));
                self.by_path.get(page).or_else(|| self.by_slug.get(&slug))
            })
            .copied()
            .filter(|&at| !self.is(at, post))
            .collect()
    }

    /// The position of each post that links to `post`, itself left out.
    /// `post` need not be in the map, such as a draft.
    pub fn inbound(&self, post: &LintPost) -> BTreeSet<usize> {
        let sources = match &post.path {
            Some(path) => self.to_page.get(page_key(path)),
            None => self
                .to_slug
                .get(&(post.locale.as_str(), post.slug.as_str())),
        };
        let sources = sources.into_iter().flatten().copied();
        sources.filter(|&at| !self.is(at, post)).collect()
    }

    /// Whether the post at `at` is `post`, by language and slug.
    fn is(&self, at: usize, post: &LintPost) -> bool {
        self.posts
            .get(at)
            .is_some_and(|other| other.locale == post.locale && other.slug == post.slug)
    }
}

fn posts_links(post: &LintPost) -> impl Iterator<Item = &khalid_lint::LintLink> {
    post.links
        .iter()
        .filter(|link| link.kind == LinkKind::Internal)
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};
    use khalid_lint::LintLink;

    use super::*;

    #[cfg(test)]
    fn post(slug: &str, path: Option<&str>, links: &[&str]) -> LintPost {
        let mut post = LintPost::new(Locale::try_new("en").unwrap(), Slug::try_new(slug).unwrap());
        post.path = path.map(str::to_owned);
        post.links = (links.iter())
            .map(|target| LintLink::new(LinkKind::Internal, *target, "words"))
            .collect();
        post
    }

    #[test]
    fn links_are_found_by_path_or_by_slug_once_each_and_never_to_itself() {
        let posts = vec![
            post(
                "pillar",
                Some("/blog/pillar/"),
                &["/blog/member/", "/blog/pillar/"],
            ),
            post(
                "member",
                None,
                &["/blog/pillar/#start", "/blog/pillar", "/elsewhere/member/"],
            ),
            post("orphan", None, &["/blog/member/"]),
        ];
        let map = LinkMap::new(&posts);
        let at = |found: BTreeSet<usize>| found.into_iter().collect::<Vec<_>>();
        assert_eq!(at(map.outbound(&posts[0])), [1]);
        assert_eq!(at(map.outbound(&posts[1])), [0]);
        assert_eq!(at(map.inbound(&posts[0])), [1]);
        assert_eq!(at(map.inbound(&posts[1])), [0, 2]);
        assert!(map.inbound(&posts[2]).is_empty());
        let draft = post("draft", Some("/blog/draft/"), &["/blog/pillar/"]);
        assert_eq!(at(map.outbound(&draft)), [0]);
        assert!(map.inbound(&draft).is_empty());
    }
}
