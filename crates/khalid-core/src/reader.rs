//! The reader features of the default templates, from the `[reader]` table
//! of `site.toml` (plan section 4.12).

use serde::Deserialize;

use crate::Slug;

/// Which reader features the default templates show, and the home page's
/// sections. Every feature is on unless `site.toml` switches it off:
///
/// ```toml
/// [reader]
/// table_of_contents = false
///
/// [[reader.home_sections]]
/// category = "technical-seo"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
#[expect(
    clippy::struct_excessive_bools,
    reason = "each field is one independent on or off key of site.toml, and every combination is valid"
)]
pub struct Reader {
    /// Each author's photo, name and job title above a post.
    pub byline: bool,
    /// "Reviewed by" in the byline, and `reviewedBy` in the page's JSON-LD.
    pub reviewed_by: bool,
    /// "8 min read" next to a post's date.
    pub reading_time: bool,
    /// The table of contents of a post with three or more H2 headings.
    pub table_of_contents: bool,
    /// A link to each H2 and H3 heading of a post.
    pub heading_links: bool,
    /// A link from a body image to its largest version.
    pub enlarge_images: bool,
    /// The authors' photos, roles and bios at the end of a post.
    pub author_box: bool,
    /// Three to five related posts under a post.
    pub related_posts: bool,
    /// The author, the date and the category on every post card.
    pub rich_cards: bool,
    /// A row of links to every category on the home page.
    pub topic_links: bool,
    /// The `/authors/` page, and the other authors on each author page.
    pub authors_page: bool,
    /// The `/archive/` page of every post.
    pub archive: bool,
    /// The search link in the header, and the `/` and Ctrl+K keys.
    pub search_shortcut: bool,
    /// The list of the site's posts that link to a post, under it.
    pub linked_mentions: bool,
    /// The title and summary of a linked post, shown when a reader points
    /// at or focuses a link to it in a body.
    pub link_previews: bool,
    /// The sections of the home page, in order. Empty means one list of the
    /// newest posts.
    pub home_sections: Vec<HomeSection>,
}

impl Default for Reader {
    fn default() -> Self {
        Self {
            byline: true,
            reviewed_by: true,
            reading_time: true,
            table_of_contents: true,
            heading_links: true,
            enlarge_images: true,
            author_box: true,
            related_posts: true,
            rich_cards: true,
            topic_links: true,
            authors_page: true,
            archive: true,
            search_shortcut: true,
            linked_mentions: true,
            link_previews: true,
            home_sections: Vec::new(),
        }
    }
}

/// One on or off key of [`Reader`], so a template can name the switches it
/// turns on or off by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ReaderSwitch {
    /// `byline`.
    Byline,
    /// `reviewed_by`.
    ReviewedBy,
    /// `reading_time`.
    ReadingTime,
    /// `table_of_contents`.
    TableOfContents,
    /// `heading_links`.
    HeadingLinks,
    /// `enlarge_images`.
    EnlargeImages,
    /// `author_box`.
    AuthorBox,
    /// `related_posts`.
    RelatedPosts,
    /// `rich_cards`.
    RichCards,
    /// `topic_links`.
    TopicLinks,
    /// `authors_page`.
    AuthorsPage,
    /// `archive`.
    Archive,
    /// `search_shortcut`.
    SearchShortcut,
    /// `linked_mentions`.
    LinkedMentions,
    /// `link_previews`.
    LinkPreviews,
}

impl ReaderSwitch {
    /// Every switch, in the order of [`Reader`].
    pub const ALL: [Self; 15] = [
        Self::Byline,
        Self::ReviewedBy,
        Self::ReadingTime,
        Self::TableOfContents,
        Self::HeadingLinks,
        Self::EnlargeImages,
        Self::AuthorBox,
        Self::RelatedPosts,
        Self::RichCards,
        Self::TopicLinks,
        Self::AuthorsPage,
        Self::Archive,
        Self::SearchShortcut,
        Self::LinkedMentions,
        Self::LinkPreviews,
    ];

    /// The key in `[reader]`, such as `reading_time`.
    pub fn key(self) -> &'static str {
        match self {
            Self::Byline => "byline",
            Self::ReviewedBy => "reviewed_by",
            Self::ReadingTime => "reading_time",
            Self::TableOfContents => "table_of_contents",
            Self::HeadingLinks => "heading_links",
            Self::EnlargeImages => "enlarge_images",
            Self::AuthorBox => "author_box",
            Self::RelatedPosts => "related_posts",
            Self::RichCards => "rich_cards",
            Self::TopicLinks => "topic_links",
            Self::AuthorsPage => "authors_page",
            Self::Archive => "archive",
            Self::SearchShortcut => "search_shortcut",
            Self::LinkedMentions => "linked_mentions",
            Self::LinkPreviews => "link_previews",
        }
    }

    /// The switch's field in `reader`.
    pub(crate) fn field(self, reader: &mut Reader) -> &mut bool {
        match self {
            Self::Byline => &mut reader.byline,
            Self::ReviewedBy => &mut reader.reviewed_by,
            Self::ReadingTime => &mut reader.reading_time,
            Self::TableOfContents => &mut reader.table_of_contents,
            Self::HeadingLinks => &mut reader.heading_links,
            Self::EnlargeImages => &mut reader.enlarge_images,
            Self::AuthorBox => &mut reader.author_box,
            Self::RelatedPosts => &mut reader.related_posts,
            Self::RichCards => &mut reader.rich_cards,
            Self::TopicLinks => &mut reader.topic_links,
            Self::AuthorsPage => &mut reader.authors_page,
            Self::Archive => &mut reader.archive,
            Self::SearchShortcut => &mut reader.search_shortcut,
            Self::LinkedMentions => &mut reader.linked_mentions,
            Self::LinkPreviews => &mut reader.link_previews,
        }
    }

    /// The switches whose keys the `[reader]` table of `text` writes. A
    /// file that is not TOML writes none; [`crate::Config`] refuses it first.
    pub(crate) fn written_in(text: &str) -> Vec<Self> {
        let table: toml::Table = toml::from_str(text).unwrap_or_default();
        let Some(toml::Value::Table(reader)) = table.get("reader") else {
            return Vec::new();
        };
        Self::ALL
            .into_iter()
            .filter(|switch| reader.contains_key(switch.key()))
            .collect()
    }
}

/// One section of the home page: the newest posts of a category, a
/// collection or a channel, with a "View all" link to its own page.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum HomeSection {
    /// A category from `categories.toml`, by slug.
    Category(Slug),
    /// A collection from `routes.toml`, such as `guides`.
    Collection(Slug),
    /// A channel from `routes.toml`, such as `featured`.
    Channel(Slug),
}
