//! [`Window`] and [`PageNumbers`]: a run of days, and what Google counted
//! for one page in it (the Search Analytics API).

use chrono::{Days, NaiveDate};
use serde::Deserialize;

use crate::SearchConsoleError;

/// A run of whole days, first and last included.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Window {
    /// The first day.
    pub first: NaiveDate,
    /// The last day.
    pub last: NaiveDate,
}

impl Window {
    /// The `days` days that end on `last`, or `None` for no days or a
    /// window before the calendar starts.
    pub fn ending(last: NaiveDate, days: u32) -> Option<Self> {
        let first = last.checked_sub_days(Days::new(u64::from(days.checked_sub(1)?)))?;
        Some(Self { first, last })
    }

    /// The window just as long that ends the day before this one starts.
    pub fn before(&self) -> Option<Self> {
        let last = self.first.pred_opt()?;
        let length = self.last.signed_duration_since(self.first).num_days();
        let days = u32::try_from(length).ok()?.checked_add(1)?;
        Self::ending(last, days)
    }
}

/// What Google counted for one page, or one query, in a window. Google
/// counts clicks and impressions as whole numbers in decimal form.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PageNumbers {
    /// The page's address as Google saw it, or the query.
    #[serde(rename = "keys", deserialize_with = "first_key")]
    pub key: String,
    /// Times a searcher clicked through.
    pub clicks: f64,
    /// Times the page showed in results.
    pub impressions: f64,
    /// Clicks divided by impressions, from 0 to 1.
    pub ctr: f64,
    /// The average position of its best result, 1 being the top.
    pub position: f64,
}

/// What Google counted for every page together in a window.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[non_exhaustive]
pub struct Totals {
    /// Times a searcher clicked through to any page.
    pub clicks: f64,
    /// Times any page showed in results.
    pub impressions: f64,
    /// Clicks divided by impressions, from 0 to 1, and 0 when no page
    /// showed.
    pub ctr: f64,
    /// The pages' average position, each weighted by how often it showed,
    /// 1 being the top, or `None` when no page showed.
    pub position: Option<f64>,
}

impl Totals {
    /// Adds up the pages in `rows`. A page shown more often weighs more in
    /// the average position, as one shown once tells little.
    pub fn of(rows: &[PageNumbers]) -> Self {
        let clicks = rows.iter().map(|row| row.clicks).sum();
        let impressions: f64 = rows.iter().map(|row| row.impressions).sum();
        if impressions <= 0.0 {
            return Self {
                clicks,
                ..Self::default()
            };
        }
        let weighted: f64 = rows.iter().map(|row| row.position * row.impressions).sum();
        Self {
            clicks,
            impressions,
            ctr: clicks / impressions,
            position: Some(weighted / impressions),
        }
    }
}

/// The first of a row's keys, the one dimension asked for.
fn first_key<'de, D: serde::Deserializer<'de>>(keys: D) -> Result<String, D::Error> {
    let keys = Vec::<String>::deserialize(keys)?;
    keys.into_iter()
        .next()
        .ok_or_else(|| serde::de::Error::custom("a row with no key"))
}

impl PageNumbers {
    /// The rows of the Search Analytics answer `body`, in Google's order,
    /// or none when it has none.
    ///
    /// # Errors
    ///
    /// [`SearchConsoleError::Answer`] when `body` is not the API's JSON.
    pub fn read_all(body: &[u8]) -> Result<Vec<Self>, SearchConsoleError> {
        let rows: Rows = serde_json::from_slice(body).map_err(|_| SearchConsoleError::Answer)?;
        Ok(rows.rows)
    }
}

/// The Search Analytics API's answer. `rows` is missing when there are
/// none.
#[derive(Deserialize)]
struct Rows {
    #[serde(default)]
    rows: Vec<PageNumbers>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(text: &str) -> NaiveDate {
        text.parse().unwrap()
    }

    #[test]
    fn a_window_and_the_one_before_it_meet_without_a_gap() {
        let window = Window::ending(day("2026-10-28"), 28).unwrap();
        assert_eq!(window.first, day("2026-10-01"));
        let before = window.before().unwrap();
        assert_eq!(before.last, day("2026-09-30"));
        assert_eq!(before.first, day("2026-09-03"));
        assert_eq!(Window::ending(day("2026-10-28"), 0), None);
    }

    fn row(key: &str, clicks: f64, impressions: f64, position: f64) -> PageNumbers {
        PageNumbers {
            key: key.to_owned(),
            clicks,
            impressions,
            ctr: 0.0,
            position,
        }
    }

    #[test]
    fn totals_add_the_pages_and_weigh_each_position_by_its_impressions() {
        let rows = [row("/a/", 30.0, 300.0, 2.0), row("/b/", 10.0, 100.0, 10.0)];
        let totals = Totals::of(&rows);
        assert_eq!((totals.clicks, totals.impressions), (40.0, 400.0));
        assert!((totals.ctr - 0.1).abs() < 1e-9, "{totals:?}");
        assert!(
            totals.position.is_some_and(|at| (at - 4.0).abs() < 1e-9),
            "{totals:?}"
        );
    }

    #[test]
    fn totals_of_no_pages_have_no_position_and_no_division_by_zero() {
        assert_eq!(Totals::of(&[]), Totals::default());
        let unseen = Totals::of(&[row("/a/", 0.0, 0.0, 0.0)]);
        assert_eq!((unseen.ctr, unseen.position), (0.0, None));
    }

    #[test]
    fn reads_rows_and_no_rows_but_not_odd_answers() {
        let body = br#"{"rows": [{"keys": ["https://www.example.com/a/"],
            "clicks": 3.0, "impressions": 40.0, "ctr": 0.075, "position": 4.5}]}"#;
        let rows = PageNumbers::read_all(body).unwrap();
        assert_eq!(
            rows.first().map(|row| row.key.as_str()),
            Some("https://www.example.com/a/")
        );
        assert_eq!(PageNumbers::read_all(b"{}").unwrap(), []);
        let keyless = br#"{"rows": [{"keys": [], "clicks": 1.0, "impressions": 1.0, "ctr": 1.0, "position": 1.0}]}"#;
        for odd in [&b"<html>"[..], keyless] {
            assert!(matches!(
                PageNumbers::read_all(odd),
                Err(SearchConsoleError::Answer)
            ));
        }
    }
}
