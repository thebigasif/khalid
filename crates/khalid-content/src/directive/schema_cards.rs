//! Reads the cards that describe one thing: an event and a course
//! (brief G15).

use chrono::{DateTime, FixedOffset};

use super::Attributes;
use crate::{BlockError, BlockKind, CourseBlock, EventBlock, EventPlace, EventState, SchemaCard};

/// Reads the card `name`, `event` or `course`, with `body` as its
/// description.
pub(super) fn card(
    name: &str,
    attributes: &Attributes,
    body: &[&str],
) -> Result<BlockKind, BlockError> {
    let card = match name {
        "event" => SchemaCard::Event(Box::new(event(attributes, body)?)),
        _ => SchemaCard::Course(course(attributes, body)?),
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
