//! Cards for things search engines know by type: an event, a course, a
//! book and a dataset (brief G15). Each writes one schema.org node.

use chrono::{DateTime, FixedOffset};

/// A card that describes one thing, from `:::event`, `:::course`,
/// `::book` or `:::dataset`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SchemaCard {
    /// An event, which becomes an `Event`.
    Event(Box<EventBlock>),
    /// A course, which becomes a `Course`.
    Course(CourseBlock),
    /// A book, which becomes a `Book`.
    Book(BookBlock),
    /// A dataset, which becomes a `Dataset`.
    Dataset(DatasetBlock),
}

/// An event, from `:::event{...}` with what happens there as the body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventBlock {
    /// The event.
    pub name: String,
    /// When it starts, with its offset from UTC as written.
    pub start: DateTime<FixedOffset>,
    /// When it ends, if the block says.
    pub end: Option<DateTime<FixedOffset>>,
    /// Where it happens.
    pub place: EventPlace,
    /// Whether it goes ahead.
    pub state: EventState,
    /// The ticket price as written, such as `0` or `15.00`, always with a
    /// currency.
    pub price: Option<String>,
    /// The price's three-letter currency code, such as `GBP`.
    pub currency: Option<String>,
    /// A picture of it: a path on this site or a full address.
    pub image: Option<String>,
    /// Who speaks or performs.
    pub performer: Option<String>,
    /// What happens: markdown in a block, plain text in structured data.
    pub description: String,
}

/// Where an event happens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventPlace {
    /// A venue people go to.
    Venue {
        /// The venue, such as `The Mill`.
        name: String,
        /// The street and number.
        street: String,
        /// The town or city.
        locality: String,
        /// The postcode.
        postal_code: String,
        /// The two-letter country code, such as `GB`.
        country: String,
    },
    /// Online, at this full address.
    Online(String),
}

/// Whether an event goes ahead, written `status=` on the block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventState {
    /// As planned; when the block does not say.
    #[default]
    Scheduled,
    /// It will not happen: `cancelled`.
    Cancelled,
    /// Later, on a date not yet known: `postponed`.
    Postponed,
    /// On the new date the block now gives: `rescheduled`.
    Rescheduled,
    /// Moved from a venue to online: `moved-online`.
    MovedOnline,
}

/// A course the site teaches, from `:::course{name="..."}` with what it
/// teaches as the body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseBlock {
    /// The course.
    pub name: String,
    /// Where it starts: a path on this site or a full address.
    pub href: Option<String>,
    /// What it teaches: markdown in a block, plain text in structured data.
    pub description: String,
}

/// A book, from `::book{name="..." author="..."}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookBlock {
    /// The title.
    pub name: String,
    /// Who wrote it.
    pub author: String,
    /// Its ISBN, as written.
    pub isbn: Option<String>,
    /// Where to read or buy it: a full address.
    pub href: Option<String>,
}

/// Data the post publishes, from `:::dataset{name="..."}` with what it
/// holds as the body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetBlock {
    /// The dataset.
    pub name: String,
    /// Words people search for it by, from `keywords="a, b"`.
    pub keywords: Vec<String>,
    /// The licence it is shared under: a full address.
    pub license: Option<String>,
    /// Where to download it: a path on this site or a full address.
    pub download: Option<String>,
    /// What it holds and how it was gathered: markdown in a block, plain
    /// text in structured data.
    pub description: String,
}
