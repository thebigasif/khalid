//! The words title templates need in a page's language: month names, the
//! order of a date, and the "Page 2 of 5" line of a long list.

use chrono::{Datelike, NaiveDate};

/// The month names of [`TemplateWords::default`].
#[rustfmt::skip]
const ENGLISH_MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June", "July", "August", "September",
    "October", "November", "December",
];

/// The words the `{date}`, `{modified}`, `{currentmonth}` and `{page}`
/// variables are written with, in one language. English by default; the
/// wiring fills the other languages from the theme's strings files.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct TemplateWords {
    /// The month names, January first.
    pub months: [String; 12],
    /// How a date is written, with `{day}`, `{month}` and `{year}`, such as
    /// `{day} {month} {year}`.
    pub date_format: String,
    /// The page line, with `{current}` and `{total}`, such as
    /// `Page {current} of {total}`.
    pub page_of: String,
}

impl Default for TemplateWords {
    fn default() -> Self {
        Self {
            months: ENGLISH_MONTHS.map(str::to_owned),
            date_format: "{day} {month} {year}".to_owned(),
            page_of: "Page {current} of {total}".to_owned(),
        }
    }
}

impl TemplateWords {
    /// The name of the month of `date`.
    pub fn month(&self, date: NaiveDate) -> &str {
        let index = usize::try_from(date.month0()).unwrap_or_default();
        self.months.get(index).map_or("", String::as_str)
    }

    /// `date` written the way this language writes a date.
    pub fn date(&self, date: NaiveDate) -> String {
        self.date_format
            .replace("{day}", &date.day().to_string())
            .replace("{month}", self.month(date))
            .replace("{year}", &date.year().to_string())
    }

    /// The page line for `number`: empty on the first page, which needs none.
    pub fn page(&self, number: PageNumber) -> String {
        if number.current <= 1 {
            return String::new();
        }
        self.page_of
            .replace("{current}", &number.current.to_string())
            .replace("{total}", &number.total.to_string())
    }
}

/// Which page of a list a page is, such as page 2 of 5.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageNumber {
    current: u32,
    total: u32,
}

impl PageNumber {
    /// Page `current` of `total`, counting from 1. `None` when `current` is
    /// 0 or past the last page.
    pub fn new(current: u32, total: u32) -> Option<Self> {
        (1..=total)
            .contains(&current)
            .then_some(Self { current, total })
    }
}
