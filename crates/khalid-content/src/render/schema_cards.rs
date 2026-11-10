//! The HTML of the event, course, book and dataset cards. Each is drawn
//! like a product card, and its data goes to the structured blocks with
//! its description as plain text.

use chrono::{DateTime, FixedOffset};

use super::steps::plain;
use super::{Image, Link, Renderer, attribute};
use crate::{
    BookBlock, CourseBlock, DatasetBlock, EventBlock, EventPlace, SchemaCard, StructuredBlock,
};

impl Renderer<'_> {
    /// Writes a card and hands its data to the structured blocks.
    pub(super) fn card(&mut self, card: &SchemaCard) {
        let plain_card = match card {
            SchemaCard::Event(event) => {
                self.event(event);
                SchemaCard::Event(Box::new(EventBlock {
                    description: plain(&event.description),
                    ..(**event).clone()
                }))
            }
            SchemaCard::Course(course) => {
                self.course(course);
                SchemaCard::Course(CourseBlock {
                    description: plain(&course.description),
                    ..course.clone()
                })
            }
            SchemaCard::Book(book) => {
                self.book(book);
                card.clone()
            }
            SchemaCard::Dataset(dataset) => {
                self.dataset(dataset);
                SchemaCard::Dataset(DatasetBlock {
                    description: plain(&dataset.description),
                    ..dataset.clone()
                })
            }
        };
        self.facts
            .structured
            .push(StructuredBlock::Card(plain_card));
    }

    /// Writes an event: picture, name, when, where, price, performer and
    /// what happens.
    fn event(&mut self, event: &EventBlock) {
        self.open("event", &event.name, event.image.as_deref());
        let mut when = time(&event.start, DAY_AND_HOUR);
        if let Some(end) = &event.end {
            let same_day = end.date_naive() == event.start.date_naive();
            let shown = if same_day { "%H:%M" } else { DAY_AND_HOUR };
            when = [when, " – ".to_owned(), time(end, shown)].concat();
        }
        self.push(&["<p class=\"event-time\">", &when, "</p>\n"]);
        match &event.place {
            EventPlace::Venue {
                name,
                street,
                locality,
                postal_code,
                ..
            } => {
                let town = [attribute(locality), attribute(postal_code)].join(" ");
                let lines = [attribute(name), attribute(street), town];
                self.push(&["<address>", &lines.join("<br />\n"), "</address>\n"]);
            }
            EventPlace::Online(href) => {
                self.push(&["<p>"]);
                self.link(href, href);
                self.push(&["</p>\n"]);
            }
        }
        if let (Some(price), Some(currency)) = (&event.price, &event.currency) {
            let (price, currency) = (attribute(price), attribute(currency));
            self.push(&[
                "<p class=\"product-price\"><data value=\"",
                &price,
                "\">",
                &currency,
                " ",
                &price,
                "</data></p>\n",
            ]);
        }
        if let Some(performer) = &event.performer {
            self.push(&["<p>", &attribute(performer), "</p>\n"]);
        }
        self.markdown(&event.description);
        self.html.push_str("</section>\n");
    }

    /// Writes a course: its name, what it teaches and a link to its start.
    fn course(&mut self, course: &CourseBlock) {
        self.open("course", &course.name, None);
        self.markdown(&course.description);
        if let Some(href) = &course.href {
            self.push(&["<p>"]);
            self.link(href, &course.name);
            self.push(&["</p>\n"]);
        }
        self.html.push_str("</section>\n");
    }

    /// Writes a book: its title, author and ISBN, and a link to it.
    fn book(&mut self, book: &BookBlock) {
        self.open("book", &book.name, None);
        self.push(&["<p>", &attribute(&book.author), "</p>\n"]);
        if let Some(isbn) = &book.isbn {
            self.push(&["<p>ISBN ", &attribute(isbn), "</p>\n"]);
        }
        if let Some(href) = &book.href {
            self.push(&["<p>"]);
            self.link(href, &book.name);
            self.push(&["</p>\n"]);
        }
        self.html.push_str("</section>\n");
    }

    /// Writes a dataset: its name, what it holds and a link to download it.
    fn dataset(&mut self, dataset: &DatasetBlock) {
        self.open("dataset", &dataset.name, None);
        self.markdown(&dataset.description);
        if let Some(download) = &dataset.download {
            self.push(&["<p>"]);
            self.link(download, download);
            self.push(&["</p>\n"]);
        }
        self.html.push_str("</section>\n");
    }

    /// Opens a card of `kind` with its picture, when it has one, and `name`.
    fn open(&mut self, kind: &str, name: &str, image: Option<&str>) {
        let name_text = attribute(name);
        self.push(&["<section class=\"product ", kind, "\">\n"]);
        if let Some(image) = image {
            self.push(&["<img src=\"", &attribute(image), "\" alt=\"", &name_text]);
            self.html
                .push_str("\" loading=\"lazy\" decoding=\"async\" />\n");
            self.facts.images.push(Image {
                src: image.to_owned(),
                alt: Some(name.to_owned()),
            });
        }
        self.push(&["<p class=\"product-name\">", &name_text, "</p>\n"]);
    }

    /// Writes a link to `href` with `text`, and counts it among the page's
    /// links.
    fn link(&mut self, href: &str, text: &str) {
        self.push(&[
            "<a href=\"",
            &attribute(href),
            "\">",
            &attribute(text),
            "</a>",
        ]);
        self.facts.links.push(Link {
            href: href.to_owned(),
            text: text.to_owned(),
            internal: !href.contains(':'),
        });
    }
}

/// How an event's start shows, such as `2026-11-12 18:00`: the same in
/// every language.
const DAY_AND_HOUR: &str = "%Y-%m-%d %H:%M";

/// A `<time>` for `moment`, shown in the chrono format `shown`.
fn time(moment: &DateTime<FixedOffset>, shown: &str) -> String {
    [
        "<time datetime=\"",
        &moment.to_rfc3339(),
        "\">",
        &moment.format(shown).to_string(),
        "</time>",
    ]
    .concat()
}
