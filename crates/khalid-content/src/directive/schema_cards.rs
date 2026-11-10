//! Reads the cards that describe one thing: an event, a course, a book and
//! a dataset (brief G15).

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};

use super::Attributes;
use crate::{
    BlockError, BlockKind, BookBlock, CourseBlock, DatasetBlock, EventBlock, EventPlace,
    EventState, SchemaCard,
};

/// Reads the card `name`, one of `event`, `course`, `book` and `dataset`,
/// with `body` as its description.
pub(super) fn card(
    name: &str,
    attributes: &Attributes,
    body: &[&str],
) -> Result<BlockKind, BlockError> {
    let card = match name {
        "event" => SchemaCard::Event(Box::new(event(attributes, body)?)),
        "course" => SchemaCard::Course(course(attributes, body)?),
        "book" => SchemaCard::Book(book(attributes)?),
        _ => SchemaCard::Dataset(dataset(attributes, body)?),
    };
    Ok(BlockKind::Card(card))
}

const EVENT_ATTRIBUTES: [&str; 14] = [
    "name",
    "start",
    "end",
    "venue",
    "street",
    "locality",
    "postcode",
    "country",
    "online",
    "status",
    "price",
    "currency",
    "image",
    "performer",
];

fn event(attributes: &Attributes, body: &[&str]) -> Result<EventBlock, BlockError> {
    attributes.only(
        "event",
        &EVENT_ATTRIBUTES,
        "name, start, end, venue, street, locality, postcode, country, online, status, price, currency, image and performer",
    )?;
    let name = attributes.need("name", "an event needs name, what it is called")?;
    let when = "an event's start and end are a date and time with the offset from UTC, such as 2026-11-12T18:00+00:00";
    let start = attributes.need("start", when)?;
    let start = moment(&start).ok_or_else(|| value(attributes, when))?;
    let end = attributes.parsed("end", moment, when)?;
    let state = attributes.parsed(
        "status",
        state,
        "an event's status is scheduled, cancelled, postponed, rescheduled or moved-online",
    )?;
    let (price, currency) = (attributes.get("price"), attributes.get("currency"));
    if !money(price.as_deref(), currency.as_deref()) {
        return Err(value(
            attributes,
            "an event's price is a number such as 15.00 or 0 with a three-letter currency such as GBP",
        ));
    }
    Ok(EventBlock {
        name,
        start,
        end,
        place: place(attributes)?,
        state: state.unwrap_or_default(),
        price,
        currency,
        image: attributes
            .get("image")
            .map(|image| attributes.media("event", "image", image))
            .transpose()?,
        performer: attributes.get("performer"),
        description: body.join("\n"),
    })
}

/// Where an event happens: `online`, a full address, or a venue with its
/// street address.
fn place(attributes: &Attributes) -> Result<EventPlace, BlockError> {
    if let Some(online) = attributes.get("online") {
        attributes.address("event", "online", &online)?;
        return Ok(EventPlace::Online(online));
    }
    let needs = "an event needs venue, street, locality, postcode and country, or online, the address it is held at";
    let country = attributes.need("country", needs)?;
    if country.len() != 2 || !country.bytes().all(|b| b.is_ascii_uppercase()) {
        return Err(value(
            attributes,
            "an event's country is a two-letter code in capitals, such as GB or DE",
        ));
    }
    Ok(EventPlace::Venue {
        name: attributes.need("venue", needs)?,
        street: attributes.need("street", needs)?,
        locality: attributes.need("locality", needs)?,
        postal_code: attributes.need("postcode", needs)?,
        country,
    })
}

fn course(attributes: &Attributes, body: &[&str]) -> Result<CourseBlock, BlockError> {
    attributes.only("course", &["name", "href"], "name and href")?;
    let name = attributes.need("name", "a course needs name, what it is called")?;
    let description = body.join("\n");
    if description.trim().is_empty() {
        return Err(BlockError::Missing {
            line: attributes.line,
            problem: "a course needs a body that says what it teaches",
        });
    }
    Ok(CourseBlock {
        name,
        href: attributes
            .get("href")
            .map(|href| attributes.media("course", "href", href))
            .transpose()?,
        description,
    })
}

fn book(attributes: &Attributes) -> Result<BookBlock, BlockError> {
    attributes.only(
        "book",
        &["name", "author", "isbn", "href"],
        "name, author, isbn and href",
    )?;
    let name = attributes.need("name", "a book needs name, its title")?;
    let author = attributes.need("author", "a book needs author, who wrote it")?;
    let isbn = attributes.get("isbn");
    let digits = |isbn: &str| isbn.chars().filter(char::is_ascii_digit).count();
    if isbn.as_deref().is_some_and(|isbn| {
        !isbn
            .chars()
            .all(|c| c.is_ascii_digit() || c == '-' || c == 'X')
            || !matches!(digits(isbn), 9 | 10 | 13)
    }) {
        return Err(value(
            attributes,
            "a book's isbn is its 10 or 13 digits, with hyphens if you like, such as 978-0-00-000000-2",
        ));
    }
    let href = attributes.get("href");
    if let Some(href) = &href {
        attributes.address("book", "href", href)?;
    }
    Ok(BookBlock {
        name,
        author,
        isbn,
        href,
    })
}

fn dataset(attributes: &Attributes, body: &[&str]) -> Result<DatasetBlock, BlockError> {
    attributes.only(
        "dataset",
        &["name", "keywords", "license", "download"],
        "name, keywords, license and download",
    )?;
    let name = attributes.need("name", "a dataset needs name, what it is called")?;
    let description = body.join("\n");
    if description.trim().is_empty() {
        return Err(BlockError::Missing {
            line: attributes.line,
            problem: "a dataset needs a body that says what it holds and how it was gathered",
        });
    }
    let license = attributes.get("license");
    if let Some(license) = &license {
        attributes.address("dataset", "license", license)?;
    }
    let keywords = attributes.get("keywords").unwrap_or_default();
    Ok(DatasetBlock {
        name,
        keywords: keywords
            .split(',')
            .map(str::trim)
            .filter(|word| !word.is_empty())
            .map(str::to_owned)
            .collect(),
        license,
        download: attributes
            .get("download")
            .map(|download| attributes.media("dataset", "download", download))
            .transpose()?,
        description,
    })
}

/// The `uploaded` date of a video or an embed: a day such as `2024-05-16`,
/// read as midnight UTC like a post's date, or a [`moment`].
///
/// # Errors
///
/// [`BlockError::Value`] when the value is neither.
pub(super) fn uploaded(
    attributes: &Attributes,
) -> Result<Option<DateTime<FixedOffset>>, BlockError> {
    attributes.parsed(
        "uploaded",
        |text| {
            let day = NaiveDate::parse_from_str(text, "%Y-%m-%d").ok();
            day.map(|day| day.and_time(NaiveTime::MIN).and_utc().fixed_offset())
                .or_else(|| moment(text))
        },
        "an uploaded date is written as 2024-05-16, or with its time, such as 2024-05-16T09:30+02:00",
    )
}

/// Reads `2026-11-12T18:00+00:00`, with seconds or without, and `Z` for UTC.
fn moment(text: &str) -> Option<DateTime<FixedOffset>> {
    let text = text
        .strip_suffix('Z')
        .map_or_else(|| text.to_owned(), |t| format!("{t}+00:00"));
    ["%Y-%m-%dT%H:%M%:z", "%Y-%m-%dT%H:%M:%S%:z"]
        .iter()
        .find_map(|format| DateTime::parse_from_str(&text, format).ok())
}

fn state(text: &str) -> Option<EventState> {
    match text {
        "scheduled" => Some(EventState::Scheduled),
        "cancelled" => Some(EventState::Cancelled),
        "postponed" => Some(EventState::Postponed),
        "rescheduled" => Some(EventState::Rescheduled),
        "moved-online" => Some(EventState::MovedOnline),
        _ => None,
    }
}

/// Whether a price and its currency come together and read as money: a
/// whole number or one with two decimals, and a three-letter code.
fn money(price: Option<&str>, currency: Option<&str>) -> bool {
    match (price, currency) {
        (Some(price), Some(code)) => {
            let (units, cents) = price.split_once('.').unwrap_or((price, "00"));
            units.parse::<u64>().is_ok()
                && cents.len() == 2
                && cents.parse::<u8>().is_ok()
                && code.len() == 3
                && code.bytes().all(|b| b.is_ascii_uppercase())
        }
        (None, None) => true,
        _ => false,
    }
}

fn value(attributes: &Attributes, problem: &'static str) -> BlockError {
    BlockError::Value {
        line: attributes.line,
        problem,
    }
}
