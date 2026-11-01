//! A list of posts: the front page, a collection, a tag or a category.

use chrono::NaiveDate;
use serde::{Serialize, Serializer};

use crate::serde_display::display;
use crate::{BodyHtml, Link, Person, ReadingTime, RenderImage};

/// One page of a list of posts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderListing {
    /// The heading, shown as the page's one `h1`.
    pub title: String,
    /// Text above the list, such as the body of a collection's `_index.md`.
    pub intro: Option<BodyHtml>,
    /// The posts on this page, in the order they are shown.
    pub posts: Vec<PostCard>,
    /// Links to the other pages of the list, when there is more than one.
    pub pagination: Option<Pagination>,
    /// The newest post of the whole list marked `featured`, on its first
    /// page only. A template with its `featured` part on shows it above
    /// the list and leaves it out of the list below.
    pub featured: Option<PostCard>,
}

/// A post as a list shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PostCard {
    /// The post's title.
    pub title: String,
    /// The post's path, such as `/blog/what-a-sitemap-does/`.
    pub href: String,
    /// The day it was published.
    #[serde(serialize_with = "display")]
    pub date: NaiveDate,
    /// One or two sentences that say what the post is about.
    pub excerpt: String,
    /// A small image, always lazy.
    pub image: Option<RenderImage>,
    /// The first author, whose name and photo the card shows.
    pub author: Option<Person>,
    /// The post's primary category.
    pub category: Option<Link>,
    /// Whether its front matter marks it `featured`, for the featured slot
    /// and the editor's picks.
    pub featured: bool,
    /// How long it takes to read. `None` on a card made before its body
    /// is rendered, such as the home page `khalid serve` answers first.
    pub reading_time: Option<ReadingTime>,
}

/// The pages of a list and which one this is. The current page is always one
/// of them, so the links can never point past either end.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pagination {
    pages: Vec<String>,
    current: usize,
}

impl Pagination {
    /// The pagination for page `current`, counted from 1, of the pages whose
    /// paths are `pages`. `None` when `current` is not one of them, or when
    /// there is only one page and nothing to link to.
    pub fn new(pages: Vec<String>, current: usize) -> Option<Self> {
        let index = current.checked_sub(1)?;
        (pages.len() > 1 && index < pages.len()).then_some(Self {
            pages,
            current: index,
        })
    }

    /// The numbered links: the first and last pages, the pages beside this
    /// one, and one gap for each run of two or more pages left out. A run
    /// of one page shows the page, since its number takes no more room.
    fn links(&self) -> Vec<PageLink<'_>> {
        let near = |index: usize| {
            index == 0
                || index.saturating_add(1) == self.pages.len()
                || index.abs_diff(self.current) <= NEAR
        };
        let shown = |index: usize| {
            near(index) || (near(index.saturating_sub(1)) && near(index.saturating_add(1)))
        };
        let mut links = Vec::new();
        for (index, href) in self.pages.iter().enumerate() {
            if shown(index) {
                links.push(PageLink {
                    number: Some(index.saturating_add(1)),
                    href: Some(href),
                    current: index == self.current,
                });
            } else if index > 0 && shown(index.saturating_sub(1)) {
                links.push(PageLink {
                    number: None,
                    href: None,
                    current: false,
                });
            }
        }
        links
    }
}

/// How many pages on each side of this one the numbered links show.
const NEAR: usize = 1;

/// One numbered page link, or a gap where pages are left out, which has
/// no number and no address.
#[derive(Serialize)]
struct PageLink<'a> {
    number: Option<usize>,
    href: Option<&'a String>,
    current: bool,
}

impl Serialize for Pagination {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Shown<'a> {
            current: usize,
            total: usize,
            previous: Option<&'a String>,
            next: Option<&'a String>,
            links: Vec<PageLink<'a>>,
        }
        Shown {
            current: self.current.saturating_add(1),
            total: self.pages.len(),
            links: self.links(),
            previous: self
                .current
                .checked_sub(1)
                .and_then(|index| self.pages.get(index)),
            next: self.pages.get(self.current.saturating_add(1)),
        }
        .serialize(serializer)
    }
}
