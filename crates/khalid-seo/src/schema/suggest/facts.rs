//! [`PostFacts`]: what a post's markdown says that schema can carry:
//! ingredients and a method, a byline and dates, prices, addresses, phone
//! numbers, opening hours, questions with answers, a breadcrumb trail and
//! links to social profiles.
//!
//! The reading is line by line and deliberately plain: a heading sets the
//! section, a list item under an "Ingredients" heading is an ingredient,
//! and patterns find prices, addresses and the rest in the text. It
//! suggests; the writer decides.

use std::sync::LazyLock;

use chrono::NaiveDate;
use regex::Regex;

/// A price and its currency, such as `19.99` and `USD`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundPrice {
    /// The amount with a `.` before any cents, such as `1299.00`.
    pub amount: String,
    /// The ISO 4217 code, such as `USD`.
    pub currency: String,
}

/// A postal address found in the text.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FoundAddress {
    /// The number and street, such as `12 High Street`.
    pub street: String,
    /// The town or city, when the text gives one after the street.
    pub locality: Option<String>,
    /// The postal code, when the text gives one.
    pub postal_code: Option<String>,
}

/// Opening hours for one run of days, such as Monday to Friday, 09:00 to
/// 17:00.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundHours {
    /// The days, as schema.org names them, such as `Monday`.
    pub days: Vec<&'static str>,
    /// When it opens, as `HH:MM`.
    pub opens: String,
    /// When it closes, as `HH:MM`.
    pub closes: String,
}

/// What a post's markdown says that schema can carry.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PostFacts {
    /// List items under an ingredients heading.
    pub ingredients: Vec<String>,
    /// Numbered steps under a method heading, or after the ingredients.
    pub steps: Vec<String>,
    /// The name a byline gives, such as `Jane Doe` from "By Jane Doe".
    pub byline: Option<String>,
    /// Dates the text gives, in the order written.
    pub dates: Vec<NaiveDate>,
    /// Prices the text gives, in the order written.
    pub prices: Vec<FoundPrice>,
    /// Postal addresses the text gives.
    pub addresses: Vec<FoundAddress>,
    /// Phone numbers the text gives, as written.
    pub phones: Vec<String>,
    /// Opening hours the text gives.
    pub hours: Vec<FoundHours>,
    /// Questions asked in headings, each with the paragraph that answers it.
    pub questions: Vec<(String, String)>,
    /// A breadcrumb trail written in the text, such as Home > Guides > Links.
    pub breadcrumbs: Vec<String>,
    /// Links to the writer's or business's social profiles.
    pub profiles: Vec<String>,
}

/// The section a line sits in, from the heading above it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Other,
    Ingredients,
    Method,
}

fn section_of(heading: &str) -> Section {
    let lower = heading.to_lowercase();
    let has = |words: &[&str]| words.iter().any(|word| lower.contains(word));
    if has(&[
        "ingredient",
        "you will need",
        "you'll need",
        "shopping list",
    ]) {
        Section::Ingredients
    } else if has(&[
        "method",
        "instruction",
        "direction",
        "steps",
        "how to make",
        "preparation",
    ]) {
        Section::Method
    } else {
        Section::Other
    }
}

static PRICE: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?:([$€£¥₹])\s?(\d{1,3}(?:,\d{3})+(?:\.\d{1,2})?|\d+(?:[.,]\d{1,2})?))",
        r"|(?:\b(\d{1,3}(?:,\d{3})+(?:\.\d{1,2})?|\d+(?:[.,]\d{1,2})?)\s?((?:USD|EUR|GBP|CAD|AUD|INR|JPY|CHF|SEK|NOK|DKK|NZD)\b|€|£))"
    ))
    .ok()
});

static STREET: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(\d{1,5}[A-Za-z]?\s+(?:[A-Z][\w'.-]*\s+){0,4}",
        r"(?:Street|St|Road|Rd|Avenue|Ave|Lane|Ln|Boulevard|Blvd|Drive|Dr|Way|Place|Pl|Court|Ct|",
        r"Square|Sq|Terrace|Close|Crescent|Highway|Hwy|Parkway|Pkwy|Row|Walk|Hill|Gardens)\b\.?)",
        r"(?:,\s*([A-Z][\w'. -]{1,40}?))?",
        r"(?:,?\s*(?:[A-Z]{2}\s+)?(\d{5}(?:-\d{4})?|[A-Z]{1,2}\d[A-Z\d]?\s*\d[A-Z]{2}))?\b"
    ))
    .ok()
});

static PHONE: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:call|phone|tel|telephone|ring|whatsapp)\b[:.]?\s*(?:us\s+)?(?:on|at)?\s*(\+?[\d(][\d\s().-]{7,20}\d)").ok()
});

static HOURS: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)\b(mon|tue|wed|thu|fri|sat|sun)[a-z]*\.?",
        r"(?:\s*(?:-|–|to|through|until)\s*(mon|tue|wed|thu|fri|sat|sun)[a-z]*\.?)?",
        r"[\s:,]+(\d{1,2}(?:[:.]\d{2})?\s*(?:am|pm)?)\s*(?:-|–|to|until)\s*(\d{1,2}(?:[:.]\d{2})?\s*(?:am|pm)?)"
    ))
    .ok()
});

static DATE: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(\d{4}-\d{2}-\d{2})\b",
        r"|\b(\d{1,2}\s+(?:January|February|March|April|May|June|July|August|September|October|November|December)\s+\d{4})\b",
        r"|\b((?:January|February|March|April|May|June|July|August|September|October|November|December)\s+\d{1,2},\s+\d{4})\b"
    ))
    .ok()
});

static BYLINE: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(r"^(?:[*_]{0,2})(?i:by|written by|words by|author:)\s+([\p{Lu}][\p{L}'.-]+(?:\s+[\p{Lu}][\p{L}'.-]+){0,3})").ok()
});

static PROFILE: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(concat!(
        r"https?://(?:www\.)?(?:twitter\.com|x\.com|facebook\.com|instagram\.com|linkedin\.com|",
        r"youtube\.com|github\.com|tiktok\.com|pinterest\.com|threads\.net|bsky\.app|mastodon\.social)",
        r"/[^\s)\]>]+"
    ))
    .ok()
});

/// Reads the facts of a post's `markdown` body.
pub fn read_facts(markdown: &str) -> PostFacts {
    let mut facts = PostFacts::default();
    let mut section = Section::Other;
    let mut question: Option<String> = None;
    for line in markdown.lines().map(str::trim) {
        if line.is_empty() {
            continue;
        }
        if let Some(heading) = heading(line) {
            section = section_of(heading);
            question = heading.ends_with('?').then(|| heading.to_owned());
            continue;
        }
        if let Some(item) = list_item(line) {
            match section {
                Section::Ingredients => facts.ingredients.push(plain(item)),
                Section::Method if ordered(line) || !facts.ingredients.is_empty() => {
                    facts.steps.push(plain(item));
                }
                _ => {}
            }
        } else if let Some(asked) = question.take() {
            facts.questions.push((asked, plain(line)));
        }
        read_line(line, &mut facts);
    }
    facts
}

/// The facts patterns find in one line.
fn read_line(line: &str, facts: &mut PostFacts) {
    if facts.byline.is_none()
        && let Some(found) = BYLINE.as_ref().and_then(|re| re.captures(line))
    {
        facts.byline = found
            .get(1)
            .map(|name| name.as_str().trim_end_matches('.').to_owned());
    }
    facts.prices.extend(prices(line));
    facts.addresses.extend(addresses(line));
    facts.hours.extend(hours(line));
    facts.dates.extend(dates(line));
    if let Some(re) = PHONE.as_ref() {
        let found = re.captures_iter(line).filter_map(|c| c.get(1));
        facts
            .phones
            .extend(found.map(|m| m.as_str().trim().to_owned()));
    }
    if let Some(re) = PROFILE.as_ref() {
        for found in re.find_iter(line) {
            let profile = found.as_str().trim_end_matches(['.', ',']).to_owned();
            if !facts.profiles.contains(&profile) {
                facts.profiles.push(profile);
            }
        }
    }
    if facts.breadcrumbs.is_empty() {
        facts.breadcrumbs = trail(line);
    }
}

fn heading(line: &str) -> Option<&str> {
    let text = line.trim_start_matches('#');
    if text.len() < line.len() && text.starts_with(' ') {
        return Some(text.trim());
    }
    // A Khalid block such as :::ingredients opens a section too.
    line.strip_prefix(":::")
        .map(|name| name.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|name| !name.is_empty())
}

fn list_item(line: &str) -> Option<&str> {
    if let Some(rest) = line.strip_prefix(['-', '*', '+'])
        && rest.starts_with(' ')
    {
        return Some(rest.trim());
    }
    let digits = line.bytes().take_while(u8::is_ascii_digit).count();
    let rest = line.get(digits..)?;
    (digits > 0)
        .then(|| rest.strip_prefix(['.', ')']))
        .flatten()
        .filter(|rest| rest.starts_with(' '))
        .map(str::trim)
}

fn ordered(line: &str) -> bool {
    line.bytes().next().is_some_and(|b| b.is_ascii_digit())
}

/// `text` without markdown emphasis and with links as their words.
fn plain(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find('[') {
        let (before, after) = rest.split_at(open);
        out.push_str(before);
        match after.find("](").zip(after.find(')')) {
            Some((middle, close)) if middle < close => {
                out.push_str(after.get(1..middle).unwrap_or_default());
                rest = after.get(close + 1..).unwrap_or_default();
            }
            _ => {
                out.push('[');
                rest = after.get(1..).unwrap_or_default();
            }
        }
    }
    out.push_str(rest);
    out.replace(['*', '_', '`'], "").trim().to_owned()
}

fn prices(line: &str) -> Vec<FoundPrice> {
    let Some(re) = PRICE.as_ref() else {
        return Vec::new();
    };
    re.captures_iter(line)
        .filter_map(|found| {
            let (sign, amount) = match (found.get(1), found.get(2), found.get(3), found.get(4)) {
                (Some(sign), Some(amount), _, _) => (sign.as_str(), amount.as_str()),
                (_, _, Some(amount), Some(code)) => (code.as_str(), amount.as_str()),
                _ => return None,
            };
            let currency = match sign {
                "$" => "USD",
                "€" => "EUR",
                "£" => "GBP",
                "¥" => "JPY",
                "₹" => "INR",
                code => code,
            };
            Some(FoundPrice {
                amount: amount_of(amount),
                currency: currency.to_owned(),
            })
        })
        .collect()
}

/// `1,299.00` as `1299.00`, and `19,99` as `19.99`.
fn amount_of(written: &str) -> String {
    match written.rsplit_once(',') {
        Some((whole, cents)) if cents.len() <= 2 && !written.contains('.') => {
            format!("{whole}.{cents}")
        }
        _ => written.replace(',', ""),
    }
}

fn addresses(line: &str) -> Vec<FoundAddress> {
    let Some(re) = STREET.as_ref() else {
        return Vec::new();
    };
    re.captures_iter(line)
        .filter_map(|found| {
            let street = found.get(1)?.as_str().trim().to_owned();
            Some(FoundAddress {
                street,
                locality: found.get(2).map(|m| m.as_str().trim().to_owned()),
                postal_code: found.get(3).map(|m| m.as_str().trim().to_owned()),
            })
        })
        .collect()
}

const DAYS: [(&str, &str); 7] = [
    ("mon", "Monday"),
    ("tue", "Tuesday"),
    ("wed", "Wednesday"),
    ("thu", "Thursday"),
    ("fri", "Friday"),
    ("sat", "Saturday"),
    ("sun", "Sunday"),
];

fn hours(line: &str) -> Vec<FoundHours> {
    let Some(re) = HOURS.as_ref() else {
        return Vec::new();
    };
    re.captures_iter(line)
        .filter_map(|found| {
            let day = |at: usize| {
                let short = found.get(at)?.as_str().to_lowercase();
                DAYS.iter().position(|(key, _)| *key == short)
            };
            let first = day(1)?;
            let last = day(2).unwrap_or(first);
            let days = DAYS
                .iter()
                .skip(first)
                .take(last.checked_sub(first)? + 1)
                .map(|(_, name)| *name)
                .collect();
            Some(FoundHours {
                days,
                opens: clock(found.get(3)?.as_str())?,
                closes: clock(found.get(4)?.as_str())?,
            })
        })
        .collect()
}

/// `9am`, `9:30 pm` or `17.00` as `HH:MM`.
fn clock(written: &str) -> Option<String> {
    let lower = written.to_lowercase().replace(' ', "");
    let (time, later) = match (lower.strip_suffix("pm"), lower.strip_suffix("am")) {
        (Some(time), _) => (time.to_owned(), Some(true)),
        (_, Some(time)) => (time.to_owned(), Some(false)),
        _ => (lower, None),
    };
    let (hour, minute) = time.split_once([':', '.']).unwrap_or((&time, "00"));
    let mut hour: u32 = hour.parse().ok()?;
    let minute: u32 = minute.parse().ok()?;
    match later {
        Some(true) if hour < 12 => hour += 12,
        Some(false) if hour == 12 => hour = 0,
        _ => {}
    }
    (hour < 24 && minute < 60).then(|| format!("{hour:02}:{minute:02}"))
}

fn dates(line: &str) -> Vec<NaiveDate> {
    let Some(re) = DATE.as_ref() else {
        return Vec::new();
    };
    re.captures_iter(line)
        .filter_map(|found| {
            let text = found.get(0)?.as_str();
            ["%Y-%m-%d", "%d %B %Y", "%B %d, %Y"]
                .iter()
                .find_map(|format| NaiveDate::parse_from_str(text, format).ok())
        })
        .collect()
}

/// The parts of a breadcrumb trail such as `Home > Guides > Links`, when
/// `line` is one: two or more short parts split by `>`, `›` or `»`.
fn trail(line: &str) -> Vec<String> {
    let parts: Vec<String> = line.split(['>', '›', '»']).map(plain).collect();
    let short = parts
        .iter()
        .all(|part| !part.is_empty() && part.chars().count() <= 40);
    if parts.len() >= 3 && short && !line.starts_with('>') {
        parts
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{amount_of, clock, read_facts};

    #[test]
    fn a_recipe_reads_as_ingredients_and_steps() {
        let facts = read_facts(
            "## Ingredients\n\n- 250 g rolled oats\n- 125 g **butter**\n\n## Method\n\n\
             1. Melt the butter.\n2. Stir in the [oats](/oats/).\n",
        );
        assert_eq!(facts.ingredients, ["250 g rolled oats", "125 g butter"]);
        assert_eq!(facts.steps, ["Melt the butter.", "Stir in the oats."]);
    }

    #[test]
    fn a_business_page_reads_as_address_phone_hours_and_profiles() {
        let facts = read_facts(
            "Visit us at 12 High Street, Leeds LS1 4DY. Call us on +44 113 496 0000.\n\n\
             Open Monday to Friday, 9am to 5:30pm, and Sat 10:00-16:00.\n\n\
             Follow us on https://www.instagram.com/noodlebar and https://x.com/noodlebar.",
        );
        let address = &facts.addresses[0];
        assert_eq!(address.street, "12 High Street");
        assert_eq!(address.locality.as_deref(), Some("Leeds"));
        assert_eq!(address.postal_code.as_deref(), Some("LS1 4DY"));
        assert_eq!(facts.phones, ["+44 113 496 0000"]);
        assert_eq!(facts.hours.len(), 2);
        assert_eq!(facts.hours[0].days.len(), 5);
        assert_eq!(
            (
                facts.hours[0].opens.as_str(),
                facts.hours[0].closes.as_str()
            ),
            ("09:00", "17:30")
        );
        assert_eq!(facts.hours[1].days, ["Saturday"]);
        assert_eq!(facts.profiles.len(), 2);
    }

    #[test]
    fn prices_bylines_dates_questions_and_trails_are_found() {
        let facts = read_facts(
            "Home > Guides > Soup kits\n\n*By Jane Doe*\n\nPublished 12 November 2026.\n\n\
             The kit costs $19.99, or €18,50 in Europe, or 1,299.00 INR.\n\n\
             ## How long does it keep?\n\nSix months in a dry cupboard.\n",
        );
        assert_eq!(facts.breadcrumbs, ["Home", "Guides", "Soup kits"]);
        assert_eq!(facts.byline.as_deref(), Some("Jane Doe"));
        assert_eq!(facts.dates.len(), 1);
        let prices: Vec<(&str, &str)> = facts
            .prices
            .iter()
            .map(|p| (p.amount.as_str(), p.currency.as_str()))
            .collect();
        assert_eq!(
            prices,
            [("19.99", "USD"), ("18.50", "EUR"), ("1299.00", "INR")]
        );
        assert_eq!(
            facts.questions,
            [(
                "How long does it keep?".to_owned(),
                "Six months in a dry cupboard.".to_owned()
            )]
        );
    }

    #[test]
    fn amounts_and_clocks_are_normalised() {
        assert_eq!(amount_of("1,299"), "1299");
        assert_eq!(amount_of("19,99"), "19.99");
        assert_eq!(clock("9am").as_deref(), Some("09:00"));
        assert_eq!(clock("12 am").as_deref(), Some("00:00"));
        assert_eq!(clock("5.30pm").as_deref(), Some("17:30"));
        assert_eq!(clock("25:00"), None);
    }
}
