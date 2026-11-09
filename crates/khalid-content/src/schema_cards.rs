//! Cards for things search engines know by type: an event and a course
//! (brief G15). Each writes one schema.org node.

use chrono::{DateTime, FixedOffset};

/// A card that describes one thing, from `:::event` or `:::course`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SchemaCard {
    /// An event, which becomes an `Event`.
    Event(Box<EventBlock>),
    /// A course, which becomes a `Course`.
    Course(CourseBlock),
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
