//! Case studies and the changelog: the list of each collection, and the
//! page of one entry.

use chrono::NaiveDate;
use serde::Serialize;

use crate::serde_display::display_option;
use crate::{BodyHtml, Link};

/// Which of the two collections a page shows, which picks its words and
/// its layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum EntryKind {
    /// The business blog's proof pages, each about one client.
    CaseStudies,
    /// Product updates, newest first, grouped by month.
    Changelog,
}

/// The list page of case studies or of the changelog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderEntries {
    /// The heading, shown as the page's one `h1`.
    pub title: String,
    /// Text above the entries, already sanitised, from the collection's
    /// `_index.md`.
    pub intro: Option<BodyHtml>,
    /// Which collection this is.
    pub kind: EntryKind,
    /// The entries, newest first: one group for case studies, one per
    /// month for the changelog. Empty when there is none yet.
    pub groups: Vec<EntryGroup>,
}

/// The entries under one heading, such as a month of the changelog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EntryGroup {
    /// The year of the month the group covers, or `None` for no heading.
    pub year: Option<i32>,
    /// The month the group covers, from 1 to 12, named by the strings
    /// file's `month_1` to `month_12`.
    pub month: Option<u32>,
    /// The entries, newest first.
    pub entries: Vec<EntryCard>,
}

/// One entry as its list shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EntryCard {
    /// The headline.
    pub title: String,
    /// The entry's own page.
    pub href: String,
    /// The day it is dated.
    #[serde(serialize_with = "display_option")]
    pub date: Option<NaiveDate>,
    /// One or two sentences on what it is about. Empty when it has none.
    pub summary: String,
    /// A case study's client.
    pub client: Option<String>,
    /// A case study's result, in one line.
    pub outcome: Option<String>,
    /// A changelog entry's version, such as `2.4.0`.
    pub version: Option<String>,
    /// A changelog entry's whole text, already sanitised, which its list
    /// shows in full.
    pub body: Option<BodyHtml>,
}

/// The page of one case study or changelog entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderEntry {
    /// The heading, shown as the page's one `h1`.
    pub title: String,
    /// Which collection it belongs to.
    pub kind: EntryKind,
    /// The day it is dated.
    #[serde(serialize_with = "display_option")]
    pub date: Option<NaiveDate>,
    /// A case study's client.
    pub client: Option<String>,
    /// A case study's result, in one line.
    pub outcome: Option<String>,
    /// A changelog entry's version.
    pub version: Option<String>,
    /// The list it belongs to, for the link back.
    pub back: Link,
    /// The entry's text, already sanitised.
    pub body: BodyHtml,
}
