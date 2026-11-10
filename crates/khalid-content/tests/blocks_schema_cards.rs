//! Event, course, book and dataset cards: each describes one thing search
//! engines know by type (brief G15, KH-CON-5).

use khalid_content::{
    BlockKind, BookBlock, CourseBlock, DatasetBlock, EventBlock, EventPlace, EventState,
    SchemaCard, StructuredBlock, parse_blocks, print_blocks, render,
};
use khalid_core::{Locale, SlugMode};

const EVENT: &str = "\
:::event{name=\"Internal links clinic\" start=\"2026-11-12T18:00+00:00\" end=\"2026-11-12T19:00Z\" venue=\"The Mill\" street=\"2 Mill Lane\" locality=\"Leeds\" postcode=\"LS1 4AB\" country=\"GB\" price=\"0\" currency=\"GBP\" image=\"clinic.jpg\" performer=\"Asif\"}
An hour of fixing internal links on **your own** posts.
:::
";

const COURSE: &str = "\
:::course{name=\"SEO writing\" href=\"/blog/seo-writing-course/\"}
Twelve lessons on writing posts that rank.
:::
";

const BOOK: &str = "::book{name=\"Everybody Writes\" author=\"Ann Handley\" isbn=\"978-1-119-85487-9\" href=\"https://www.example.com/books/everybody-writes\"}\n";

const DATASET: &str = "\
:::dataset{name=\"Content benchmark 2026\" keywords=\"word count, rankings\" license=\"https://creativecommons.org/licenses/by/4.0/\" download=\"benchmark.csv\"}
Word counts and positions for 1,200 posts.
:::
";

#[cfg(test)]
fn card(text: &str) -> SchemaCard {
    let blocks = parse_blocks(text).unwrap();
    assert_eq!(print_blocks(&blocks), text, "round trip");
    let BlockKind::Card(card) = &blocks[0].kind else {
        panic!("{blocks:?}");
    };
    card.clone()
}

#[cfg(test)]
fn error(text: &str) -> String {
    parse_blocks(text).unwrap_err().to_string()
}

#[test]
fn an_event_round_trips() {
    let SchemaCard::Event(event) = card(EVENT) else {
        panic!("not an event");
    };
    let EventBlock {
        start, end, place, ..
    } = &*event;
    assert_eq!(start.to_rfc3339(), "2026-11-12T18:00:00+00:00");
    assert_eq!(end.unwrap().to_rfc3339(), "2026-11-12T19:00:00+00:00");
    assert_eq!(
        *place,
        EventPlace::Venue {
            name: "The Mill".to_owned(),
            street: "2 Mill Lane".to_owned(),
            locality: "Leeds".to_owned(),
            postal_code: "LS1 4AB".to_owned(),
            country: "GB".to_owned(),
        }
    );
    assert_eq!(event.state, EventState::Scheduled);
    assert_eq!(event.price.as_deref(), Some("0"));
    assert_eq!(
        event.description,
        "An hour of fixing internal links on **your own** posts."
    );
}

#[test]
fn an_online_event_needs_no_venue() {
    let SchemaCard::Event(event) = card(
        ":::event{name=\"Clinic\" start=\"2026-11-12T18:00+01:00\" online=\"https://meet.example.com/clinic\" status=\"moved-online\"}\nOn a call.\n:::\n",
    ) else {
        panic!("not an event");
    };
    assert_eq!(
        event.place,
        EventPlace::Online("https://meet.example.com/clinic".to_owned())
    );
    assert_eq!(event.state, EventState::MovedOnline);
    assert_eq!(event.start.to_rfc3339(), "2026-11-12T18:00:00+01:00");
}

#[test]
fn an_event_refuses_a_time_without_its_offset_and_a_missing_place() {
    assert_eq!(
        error(
            ":::event{name=\"Clinic\" start=\"2026-11-12 18:00\" online=\"https://meet.example.com/\"}\nA call.\n:::\n"
        ),
        "line 1: an event's start and end are a date and time with the offset from UTC, such as 2026-11-12T18:00+00:00"
    );
    assert_eq!(
        error(":::event{name=\"Clinic\" start=\"2026-11-12T18:00Z\"}\nA call.\n:::\n"),
        "line 1: an event needs venue, street, locality, postcode and country, or online, the address it is held at"
    );
    assert_eq!(
        error(
            ":::event{name=\"Clinic\" start=\"2026-11-12T18:00Z\" online=\"https://meet.example.com/\" price=\"5\"}\nA call.\n:::\n"
        ),
        "line 1: an event's price is a number such as 15.00 or 0 with a three-letter currency such as GBP"
    );
}

#[test]
fn a_course_round_trips_and_needs_a_body() {
    assert_eq!(
        card(COURSE),
        SchemaCard::Course(CourseBlock {
            name: "SEO writing".to_owned(),
            href: Some("/blog/seo-writing-course/".to_owned()),
            description: "Twelve lessons on writing posts that rank.".to_owned(),
        })
    );
    assert_eq!(
        error(":::course{name=\"SEO writing\"}\n:::\n"),
        "line 1: a course needs a body that says what it teaches"
    );
}

#[test]
fn a_book_round_trips_and_checks_its_isbn() {
    assert_eq!(
        card(BOOK),
        SchemaCard::Book(BookBlock {
            name: "Everybody Writes".to_owned(),
            author: "Ann Handley".to_owned(),
            isbn: Some("978-1-119-85487-9".to_owned()),
            href: Some("https://www.example.com/books/everybody-writes".to_owned()),
        })
    );
    assert_eq!(
        error("::book{name=\"Notes\" author=\"Asif\" isbn=\"12\"}\n"),
        "line 1: a book's isbn is its 10 or 13 digits, with hyphens if you like, such as 978-0-00-000000-2"
    );
    assert_eq!(
        error(":::book{name=\"Notes\" author=\"Asif\"}\n:::\n"),
        "line 1: book has no body; write it on one line as ::book"
    );
}

#[test]
fn a_dataset_round_trips_with_its_keywords() {
    assert_eq!(
        card(DATASET),
        SchemaCard::Dataset(DatasetBlock {
            name: "Content benchmark 2026".to_owned(),
            keywords: vec!["word count".to_owned(), "rankings".to_owned()],
            license: Some("https://creativecommons.org/licenses/by/4.0/".to_owned()),
            download: Some("benchmark.csv".to_owned()),
            description: "Word counts and positions for 1,200 posts.".to_owned(),
        })
    );
    assert_eq!(
        error(":::dataset{name=\"Benchmark\" license=\"CC BY\"}\nNumbers.\n:::\n")
            .split(':')
            .nth(1),
        Some(" a dataset's license is wrong")
    );
}

#[test]
fn renders_each_card_and_its_structured_data_in_plain_text() {
    let body = [EVENT, COURSE, BOOK, DATASET].join("\n");
    let page = render(&body, &Locale::try_new("en").unwrap(), SlugMode::Ascii).unwrap();
    let [StructuredBlock::Card(SchemaCard::Event(event)), _, _, _] = page.structured.as_slice()
    else {
        panic!("{:?}", page.structured);
    };
    assert_eq!(
        event.description,
        "An hour of fixing internal links on your own posts."
    );
    assert_eq!(page.images.len(), 1, "the event's picture");
    assert_eq!(page.links.len(), 3, "the course, the book and the download");
    insta::assert_snapshot!(page.html);
}
