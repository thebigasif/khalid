//! Event and course cards: each describes one thing search engines know
//! by type (brief G15, KH-CON-5).

use khalid_content::{
    BlockKind, CourseBlock, EventBlock, EventPlace, EventState, SchemaCard, parse_blocks,
    print_blocks,
};

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
