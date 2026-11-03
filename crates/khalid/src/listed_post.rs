//! A dated post as the site's lists and the related-posts choice read it,
//! without its body, so `khalid serve` can keep every post of the last full
//! render and render a saved post's page alone with the same related posts.

use std::path::PathBuf;

use khalid_core::{Locale, Slug, SlugMode};
use khalid_render::{PostCard, PostTopics};

/// One dated post: its card in lists, what its related posts are chosen
/// by, and its author, whose page lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListedPost {
    /// The content file, relative to the site folder.
    pub(crate) path: PathBuf,
    pub(crate) locale: Locale,
    /// The collection folder it sits in, such as `posts`, whose page lists it.
    pub(crate) collection: String,
    /// Whether its front matter says `featured: true`, which channels read.
    pub(crate) featured: bool,
    /// Its tags, in the order written. `topics` holds their slugs too.
    pub(crate) tags: Vec<ListedTag>,
    pub(crate) topics: PostTopics,
    pub(crate) card: PostCard,
    pub(crate) author: Option<Slug>,
}

/// A tag as a post names it, and the slug its page is found by.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListedTag {
    /// The slug made from the name, such as `internal-links`.
    pub(crate) slug: Slug,
    /// The name as written, such as `internal links`, which titles its page.
    pub(crate) name: String,
}

impl ListedTag {
    /// The tags `names` of a post in `locale`, each slug made from its name
    /// as a title's is, in `mode`. A name no slug can be made from is left
    /// out, and the linter names it.
    pub(crate) fn all(names: &[String], locale: &Locale, mode: SlugMode) -> Vec<Self> {
        let tag = |name: &String| {
            let slug = Slug::from_title(name, locale, mode).ok()?;
            Some(Self {
                slug,
                name: name.clone(),
            })
        };
        names.iter().filter_map(tag).collect()
    }

    /// The name its page is titled with: as written when the writer gave
    /// it any capital, such as `SEO tools`, or else each word started with
    /// a capital, so `internal links` reads `Internal Links`.
    pub(crate) fn display_name(&self) -> String {
        if self.name.chars().any(char::is_uppercase) {
            return self.name.clone();
        }
        let mut shown = String::with_capacity(self.name.len());
        let mut start = true;
        for letter in self.name.chars() {
            if start {
                shown.extend(letter.to_uppercase());
            } else {
                shown.push(letter);
            }
            start = letter.is_whitespace() || letter == '-';
        }
        shown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tag(name: &str) -> ListedTag {
        let english = Locale::try_new("en").unwrap();
        let names = [name.to_owned()];
        ListedTag::all(&names, &english, SlugMode::Ascii).remove(0)
    }

    #[test]
    fn a_lowercase_tag_reads_in_title_case_and_a_capital_keeps_the_writers_name() {
        assert_eq!(tag("internal links").display_name(), "Internal Links");
        assert_eq!(tag("e-commerce").display_name(), "E-Commerce");
        assert_eq!(tag("ähnliche seiten").display_name(), "Ähnliche Seiten");
        assert_eq!(tag("SEO tools").display_name(), "SEO tools");
    }
}
