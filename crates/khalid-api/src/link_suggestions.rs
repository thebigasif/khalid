//! Internal link suggestions for one post, as Yoast Premium's internal
//! linking suggestions give them: the blog's other posts in the same
//! language that this post does not link to yet, ranked by what they
//! share with it, each with the words of this post that name it when its
//! text has them.

use khalid_lint::{ClusterRole, LintPost};
use serde::Serialize;
use utoipa::ToSchema;

use crate::link_map::LinkMap;

/// What a shared category adds to a suggestion's score.
const CATEGORY: u32 = 3;
/// What each shared tag adds.
const TAG: u32 = 2;
/// What the same topic cluster adds.
const CLUSTER: u32 = 4;
/// What it adds when the other post is the pillar of this post's cluster.
const PILLAR: u32 = 3;
/// What it adds when this post's text names the other post's focus keyword.
const MENTION: u32 = 3;
/// What each word of this post's focus keyword in the other's title adds.
const WORD: u32 = 1;

/// The most suggestions one post gets.
pub const MOST_SUGGESTIONS: usize = 10;

/// What a post is about beside its text: its category and its tags.
#[derive(Debug, Clone, Copy, Default)]
pub struct Topics<'a> {
    category: Option<&'a str>,
    tags: &'a [String],
}

impl<'a> Topics<'a> {
    /// A post in `category`, tagged `tags`.
    pub fn new(category: Option<&'a str>, tags: &'a [String]) -> Self {
        Self { category, tags }
    }
}

/// What a suggested post shares with the post being written.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, ToSchema)]
pub struct Shares {
    /// The category both are in.
    category: Option<String>,
    /// The tags both have.
    tags: Vec<String>,
    /// The topic cluster both are in.
    cluster: Option<String>,
    /// Whether the suggested post is that cluster's pillar.
    pillar: bool,
    /// The suggested post's focus keyword, when this post's text names it.
    keyword: Option<String>,
}

/// One post worth linking to from the post being written.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct LinkSuggestion {
    /// The post's identity, as in `/posts/{id}/`.
    #[schema(example = "en.choosing-anchor-text")]
    id: String,
    /// Its title.
    title: String,
    /// The address to link to.
    #[schema(example = "/blog/choosing-anchor-text/")]
    path: String,
    /// How closely it matches; the list sends the highest first.
    score: u32,
    /// What it shares with the post being written.
    shares: Shares,
    /// The words of the post being written that name it, as written,
    /// ready to become the link's words.
    anchor: Option<String>,
}

/// Up to [`MOST_SUGGESTIONS`] posts of `map` that `post`, about `topics`,
/// should link to, best first. `site` holds each mapped post's topics, in
/// the map's order.
pub fn suggest(
    post: &LintPost,
    topics: Topics<'_>,
    map: &LinkMap<'_>,
    site: &[Topics<'_>],
) -> Vec<LinkSuggestion> {
    let linked = map.outbound(post);
    let mut found: Vec<LinkSuggestion> = (map.posts().iter().enumerate())
        .filter(|(at, other)| {
            other.locale == post.locale && other.slug != post.slug && !linked.contains(at)
        })
        .filter_map(|(at, other)| {
            let theirs = site.get(at).copied().unwrap_or_default();
            weigh(post, topics, other, theirs)
        })
        .collect();
    found.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.title.cmp(&b.title)));
    found.truncate(MOST_SUGGESTIONS);
    found
}

/// `other` as a suggestion for `post`, or `None` when they share nothing.
fn weigh(
    post: &LintPost,
    topics: Topics<'_>,
    other: &LintPost,
    theirs: Topics<'_>,
) -> Option<LinkSuggestion> {
    let mut shares = Shares::default();
    let mut score: u32 = 0;
    if topics.category.is_some() && topics.category == theirs.category {
        shares.category = topics.category.map(str::to_owned);
        score = score.saturating_add(CATEGORY);
    }
    shares.tags = (topics.tags.iter())
        .filter(|tag| theirs.tags.contains(tag))
        .cloned()
        .collect();
    let tags = u32::try_from(shares.tags.len()).unwrap_or(u32::MAX);
    score = score.saturating_add(TAG.saturating_mul(tags));
    if let (Some(mine), Some(their)) = (&post.cluster, &other.cluster)
        && mine.name == their.name
    {
        shares.cluster = Some(their.name.clone());
        shares.pillar = their.role == ClusterRole::Pillar;
        let pillar = if shares.pillar { PILLAR } else { 0 };
        score = score.saturating_add(CLUSTER + pillar);
    }
    let title = other
        .title
        .clone()
        .unwrap_or_else(|| other.slug.to_string());
    let anchor = (other.focus_keyword.as_deref()).and_then(|k| mention(&post.body_text, k));
    if anchor.is_some() {
        shares.keyword.clone_from(&other.focus_keyword);
        score = score.saturating_add(MENTION);
    }
    let words = shared_words(post.focus_keyword.as_deref(), &title);
    score = score.saturating_add(WORD.saturating_mul(words));
    (score > 0).then(|| LinkSuggestion {
        // The identity the Admin API gives a post: its language, a full stop, its slug.
        id: format!("{}.{}", other.locale, other.slug),
        path: (other.path.clone()).unwrap_or_else(|| format!("/{}/", other.slug)),
        title,
        score,
        shares,
        anchor,
    })
}

/// `phrase` in `text` as written there, matched without regard to case and
/// only as whole words.
fn mention(text: &str, phrase: &str) -> Option<String> {
    let (lower, wanted) = (text.to_lowercase(), phrase.trim().to_lowercase());
    // Lowercasing changed a byte length, so positions no longer line up.
    if wanted.is_empty() || lower.len() != text.len() {
        return None;
    }
    let edge = |c: Option<char>| c.is_none_or(|c| !c.is_alphanumeric());
    lower.match_indices(&wanted).find_map(|(at, _)| {
        let end = at + wanted.len();
        let before = lower.get(..at).and_then(|head| head.chars().next_back());
        let after = lower.get(end..).and_then(|tail| tail.chars().next());
        (edge(before) && edge(after)).then(|| text.get(at..end).map(str::to_owned))?
    })
}

/// How many words of four or more letters in `keyword` are in `title`.
fn shared_words(keyword: Option<&str>, title: &str) -> u32 {
    let title = title.to_lowercase();
    let words = title
        .split(|c: char| !c.is_alphanumeric())
        .collect::<Vec<_>>();
    let keyword = keyword.unwrap_or_default().to_lowercase();
    let shared = (keyword.split_whitespace())
        .filter(|word| word.chars().count() >= 4 && words.contains(word))
        .count();
    u32::try_from(shared).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use khalid_core::{Locale, Slug};
    use khalid_lint::{Cluster, LinkKind, LintLink};

    use super::*;

    #[cfg(test)]
    fn post(language: &str, slug: &str, cluster: Option<(&str, ClusterRole)>) -> LintPost {
        let locale = Locale::try_new(language).unwrap();
        let mut post = LintPost::new(locale, Slug::try_new(slug).unwrap());
        post.path = Some(format!("/blog/{slug}/"));
        post.title = Some(slug.replace('-', " "));
        post.cluster = cluster.map(|(name, role)| Cluster::new(name, role));
        post
    }

    #[test]
    fn the_pillar_comes_first_with_the_words_that_name_it() {
        let mut pillar = post(
            "en",
            "internal-links-guide",
            Some(("links", ClusterRole::Pillar)),
        );
        pillar.focus_keyword = Some("internal links".to_owned());
        let site = vec![
            pillar,
            post("en", "anchor-text", Some(("links", ClusterRole::Member))),
            post("en", "page-speed", None),
            post("de", "interne-links", None),
            post("en", "redirects", None),
        ];
        let tags = ["links".to_owned()];
        let topics = [
            Topics::new(Some("technical-seo"), &[]),
            Topics::new(None, &tags),
            Topics::new(Some("performance"), &[]),
            Topics::new(Some("technical-seo"), &[]),
            Topics::new(Some("technical-seo"), &[]),
        ];
        let mut draft = post("en", "orphan-pages", Some(("links", ClusterRole::Member)));
        draft.body_text = "Most blogs forget Internal Links, sadly.".to_owned();
        draft.links = vec![LintLink::new(
            LinkKind::Internal,
            "/blog/redirects/",
            "redirects",
        )];
        let map = LinkMap::new(&site);
        let found = suggest(
            &draft,
            Topics::new(Some("technical-seo"), &tags),
            &map,
            &topics,
        );
        let ids: Vec<&str> = found.iter().map(|each| each.id.as_str()).collect();
        assert_eq!(
            ids,
            ["en.internal-links-guide", "en.anchor-text"],
            "{found:?}"
        );
        let first = found.first().unwrap();
        assert_eq!(first.score, CATEGORY + CLUSTER + PILLAR + MENTION);
        assert_eq!(first.anchor.as_deref(), Some("Internal Links"));
        assert!(first.shares.pillar);
        assert_eq!(found.get(1).unwrap().shares.tags, ["links"]);
    }

    #[test]
    fn a_mention_counts_only_as_whole_words() {
        assert_eq!(
            mention("Read about anchor text.", "Anchor Text"),
            Some("anchor text".to_owned())
        );
        assert_eq!(mention("Read about anchor texts.", "anchor text"), None);
        assert_eq!(
            shared_words(Some("orphan pages"), "Finding orphan pages fast"),
            2
        );
    }
}
