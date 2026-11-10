//! The business, event, course, book and dataset cards as their
//! schema.org types (brief G15).

use khalid_content::{
    BookBlock, BusinessBlock, CourseBlock, DatasetBlock, EventBlock, EventPlace, EventState,
    SchemaCard,
};
use khalid_core::{AbsoluteUrl, UserMessage};
use khalid_seo::{
    Book, Course, Dataset, Event, EventLocation, EventStatus, LocalBusiness, Money, Offer,
    PostalAddress, SchemaBlock,
};

use crate::build_message;

/// The card as its [`SchemaBlock`], its addresses made full by `full`.
///
/// # Errors
///
/// A [`UserMessage`] when an address cannot be made a full address on the
/// site, or an event's price cannot be read as money.
pub(super) fn card_of(
    card: &SchemaCard,
    full: impl Fn(&str) -> Result<AbsoluteUrl, UserMessage>,
) -> Result<Option<SchemaBlock>, UserMessage> {
    Ok(Some(match card {
        SchemaCard::Event(event) => SchemaBlock::Event(event_of(event, full)?),
        SchemaCard::Course(course) => SchemaBlock::Course(course_of(course)),
        SchemaCard::Book(book) => SchemaBlock::Book(book_of(book, full)?),
        SchemaCard::Dataset(dataset) => SchemaBlock::Dataset(dataset_of(dataset, full)?),
        _ => return Ok(None),
    }))
}

/// The business card as a `LocalBusiness`, its website made full by `full`.
pub(super) fn business_of(
    card: &BusinessBlock,
    full: impl Fn(&str) -> Result<AbsoluteUrl, UserMessage>,
) -> Result<LocalBusiness, UserMessage> {
    let address = PostalAddress::new(
        &card.street,
        &card.locality,
        &card.postal_code,
        &card.country,
    );
    let mut business = LocalBusiness::new(&card.business_type, &card.name, address);
    business.telephone.clone_from(&card.phone);
    business.email.clone_from(&card.email);
    business.url = card.website.as_deref().map(full).transpose()?;
    Ok(business)
}

fn event_of(
    block: &EventBlock,
    full: impl Fn(&str) -> Result<AbsoluteUrl, UserMessage>,
) -> Result<Event, UserMessage> {
    let location = match &block.place {
        EventPlace::Venue {
            name,
            street,
            locality,
            postal_code,
            country,
        } => EventLocation::Venue {
            name: name.clone(),
            address: PostalAddress::new(street, locality, postal_code, country),
        },
        EventPlace::Online(href) => EventLocation::Online(full(href)?),
    };
    let mut event = Event::new(&block.name, block.start.to_utc(), location);
    event.description.clone_from(&block.description);
    event.end = block.end.map(|end| end.to_utc());
    event.status = match block.state {
        EventState::Scheduled => EventStatus::Scheduled,
        EventState::Cancelled => EventStatus::Cancelled,
        EventState::Postponed => EventStatus::Postponed,
        EventState::Rescheduled => EventStatus::Rescheduled,
        EventState::MovedOnline => EventStatus::MovedOnline,
    };
    event.images = block
        .image
        .as_deref()
        .map(&full)
        .transpose()?
        .into_iter()
        .collect();
    if let (Some(price), Some(currency)) = (&block.price, &block.currency) {
        let money = Money::parse(price, currency).map_err(|error| build_message::seo(&error))?;
        event.offer = Some(Offer::new(money));
    }
    event.performer.clone_from(&block.performer);
    Ok(event)
}

fn course_of(block: &CourseBlock) -> Course {
    Course::new(&block.name, &block.description)
}

fn book_of(
    block: &BookBlock,
    full: impl Fn(&str) -> Result<AbsoluteUrl, UserMessage>,
) -> Result<Book, UserMessage> {
    let mut book = Book::new(&block.name, &block.author);
    book.isbn.clone_from(&block.isbn);
    book.url = block.href.as_deref().map(full).transpose()?;
    Ok(book)
}

fn dataset_of(
    block: &DatasetBlock,
    full: impl Fn(&str) -> Result<AbsoluteUrl, UserMessage>,
) -> Result<Dataset, UserMessage> {
    let mut dataset = Dataset::new(&block.name, &block.description);
    dataset.keywords.clone_from(&block.keywords);
    dataset.license = block.license.as_deref().map(&full).transpose()?;
    dataset.download = block.download.as_deref().map(&full).transpose()?;
    Ok(dataset)
}
