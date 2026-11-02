//! The location card: one of the site's `[[seo.locations]]`, drawn where a
//! body places `::location`, as Yoast Local's address and hours blocks do.
//! It reads completely without scripts; the reader script only fills in
//! whether the location is open now.

use std::collections::BTreeMap;

use khalid_core::{Locale, LocationSettings, WeekdaySetting};

use crate::escape::attribute;
use crate::{RenderError, builtin_strings};

/// How `khalid-content` starts the mark of a `::location` block. A test
/// checks it against that crate's own.
const LOCATION_MARK: &str = "<!--khalid-location:";

/// The days of the week, Monday first, as the hours are listed.
const WEEK: [WeekdaySetting; 7] = [
    WeekdaySetting::Mon,
    WeekdaySetting::Tue,
    WeekdaySetting::Wed,
    WeekdaySetting::Thu,
    WeekdaySetting::Fri,
    WeekdaySetting::Sat,
    WeekdaySetting::Sun,
];

/// The words a location card shows, in one language.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct LocationWords {
    /// The caption of the opening hours, such as "Opening hours".
    pub hours: String,
    /// What the card says while the location is open.
    pub open_now: String,
    /// What the card says while it is closed.
    pub closed_now: String,
    /// What a day with no hours says.
    pub closed: String,
    /// The map link's words.
    pub map: String,
    /// One time range, with `{opens}` and `{closes}`.
    pub time_range: String,
    /// The names of the days, Monday first.
    pub days: [String; 7],
}

impl LocationWords {
    /// The built-in words of `locale`'s language, or English.
    ///
    /// # Errors
    ///
    /// [`RenderError::Strings`] when a built-in strings file cannot be read.
    pub fn for_locale(locale: &Locale) -> Result<Self, RenderError> {
        let table = builtin_strings(locale)?;
        let word = |key: &str| table.get(key).cloned().unwrap_or_default();
        Ok(Self {
            hours: word("location_hours"),
            open_now: word("location_open_now"),
            closed_now: word("location_closed_now"),
            closed: word("location_closed"),
            map: word("location_map"),
            time_range: word("location_time_range"),
            days: ["1", "2", "3", "4", "5", "6", "7"].map(|day| word(&format!("weekday_{day}"))),
        })
    }
}

/// `location` as a card: its name and address, its phone number as a
/// link, its opening hours day by day, and a link to a map.
pub fn location_card(location: &LocationSettings, words: &LocationWords) -> String {
    let mut out = String::from("<div class=\"location\"");
    let open: Vec<String> = (location.hours.iter())
        .flat_map(|hours| {
            (hours.days.iter()).map(|day| format!("{} {} {}", key(*day), hours.opens, hours.closes))
        })
        .collect();
    if !open.is_empty() {
        out.extend([" data-hours=\"", &attribute(&open.join(",")), "\""]);
    }
    if let Some(geo) = location.geo {
        let point = format!("{},{}", geo.latitude(), geo.longitude());
        out.extend([" data-geo=\"", &point, "\""]);
    }
    out.extend([
        ">\n<address>\n<strong>",
        &escape(&location.name),
        "</strong><br />\n",
    ]);
    out.extend([escape(&location.street).as_str(), "<br />\n"]);
    let mut town = location.locality.clone();
    if let Some(region) = &location.region {
        town = format!("{town}, {region}");
    }
    out.extend([escape(&town).as_str(), " ", &escape(&location.postal_code)]);
    if let Some(phone) = &location.telephone {
        let dial: String = phone.chars().filter(|c| !c.is_whitespace()).collect();
        out.extend([
            "<br />\n<a href=\"tel:",
            &attribute(&dial),
            "\">",
            &escape(phone),
            "</a>",
        ]);
    }
    out.push_str("\n</address>\n");
    if !open.is_empty() {
        out.extend([
            "<p class=\"location-now\" data-open=\"",
            &attribute(&words.open_now),
            "\" data-closed=\"",
            &attribute(&words.closed_now),
            "\" hidden></p>\n",
        ]);
        hours_table(location, words, &mut out);
    }
    let map = location
        .map
        .as_ref()
        .map(|map| map.as_str().to_owned())
        .or_else(|| {
            location.geo.map(|geo| {
                let (lat, lon) = (geo.latitude(), geo.longitude());
                format!("https://www.openstreetmap.org/?mlat={lat}&mlon={lon}#map=17/{lat}/{lon}")
            })
        });
    if let Some(map) = map {
        out.extend([
            "<p><a href=\"",
            &attribute(&map),
            "\">",
            &escape(&words.map),
            "</a></p>\n",
        ]);
    }
    out.push_str("</div>\n");
    out
}

/// `html` with each location mark swapped for the card of the location it
/// names, and the names `locations` does not hold, whose marks are dropped.
/// `None` when `html` has no mark, so most bodies are never copied.
pub fn place_locations(
    html: &str,
    locations: &[LocationSettings],
    words: &LocationWords,
) -> Option<(String, Vec<String>)> {
    if !html.contains(LOCATION_MARK) {
        return None;
    }
    let by_mark: BTreeMap<String, &LocationSettings> = (locations.iter())
        .map(|location| (mark_name(&location.name), location))
        .collect();
    let mut parts = html.split(LOCATION_MARK);
    let mut out = String::with_capacity(html.len().saturating_mul(2));
    out.push_str(parts.next()?);
    let mut missing = Vec::new();
    for rest in parts {
        let Some((name, after)) = rest.split_once("-->\n") else {
            out.push_str(rest);
            continue;
        };
        match by_mark.get(name) {
            Some(location) => out.push_str(&location_card(location, words)),
            None => missing.push(unescape(name)),
        }
        out.push_str(after);
    }
    Some((out, missing))
}

fn hours_table(location: &LocationSettings, words: &LocationWords, out: &mut String) {
    out.extend([
        "<table class=\"location-hours\">\n<caption>",
        &escape(&words.hours),
        "</caption>\n",
    ]);
    for (day, name) in WEEK.iter().zip(&words.days) {
        let ranges: Vec<String> = (location.hours.iter())
            .filter(|hours| hours.days.contains(day))
            .map(|hours| {
                (words.time_range.replace("{opens}", &hours.opens))
                    .replace("{closes}", &hours.closes)
            })
            .collect();
        let said = if ranges.is_empty() {
            words.closed.clone()
        } else {
            ranges.join(", ")
        };
        out.extend([
            "<tr><th scope=\"row\">",
            &escape(name),
            "</th><td>",
            &escape(&said),
            "</td></tr>\n",
        ]);
    }
    out.push_str("</table>\n");
}

/// The day as the reader script reads it.
const fn key(day: WeekdaySetting) -> &'static str {
    match day {
        WeekdaySetting::Mon => "mon",
        WeekdaySetting::Tue => "tue",
        WeekdaySetting::Wed => "wed",
        WeekdaySetting::Thu => "thu",
        WeekdaySetting::Fri => "fri",
        WeekdaySetting::Sat => "sat",
        WeekdaySetting::Sun => "sun",
    }
}

/// `text` as element content.
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// `name` as `khalid-content` writes it in a mark, escaped as an attribute
/// with the apostrophe too.
fn mark_name(name: &str) -> String {
    attribute(name).replace('\'', "&#39;")
}

/// The name in a mark, as written.
fn unescape(name: &str) -> String {
    (name
        .replace("&#39;", "'")
        .replace("&quot;", "\"")
        .replace("&lt;", "<"))
    .replace("&gt;", ">")
    .replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
    use super::{LOCATION_MARK, mark_name, unescape};

    #[test]
    fn the_mark_is_the_content_crates_own() {
        assert_eq!(LOCATION_MARK, khalid_content::LOCATION_MARK);
    }

    #[test]
    fn a_name_survives_its_mark() {
        let name = "Joe's \"Best\" <Fish> & Chips";
        assert_eq!(unescape(&mark_name(name)), name);
    }
}
