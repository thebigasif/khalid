//! Newsletter readers from a CSV file (backlog row 9, D-4): Khalid's own
//! export, Substack's `email_list` file, or Ghost's members export. Each
//! reader keeps the status its source gave them: confirmed stays
//! confirmed, unsubscribed stays unsubscribed.

use chrono::{DateTime, NaiveDateTime, Utc};
use khalid_core::{Locale, Slug, SlugMode};

use crate::csv::{CsvError, rows};

/// The columns of Khalid's own export, in order.
const KHALID_COLUMNS: [&str; 5] = ["email", "status", "created_at", "lists", "labels"];

/// Where a reader stands, as the source said.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
    /// Signed up and never confirmed.
    Pending,
    /// Gets the newsletter.
    Confirmed,
    /// Left, and gets nothing more.
    Unsubscribed,
}

impl Standing {
    /// The name Khalid's export writes, such as `confirmed`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Confirmed => "confirmed",
            Self::Unsubscribed => "unsubscribed",
        }
    }
}

/// One reader from a CSV file.
///
/// Exhaustive on purpose: the reader and the writer fill it in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reader {
    /// Their address, as the file wrote it.
    pub email: String,
    /// Where they stand.
    pub standing: Standing,
    /// When they signed up, when the file says.
    pub since: Option<DateTime<Utc>>,
    /// The newsletters they get. Only Khalid's export names them.
    pub lists: Vec<Slug>,
    /// Their labels, such as `paid`.
    pub labels: Vec<Slug>,
}

/// Which system wrote a reader file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReaderSource {
    /// Khalid's own export.
    Khalid,
    /// Substack's `email_list` file.
    Substack,
    /// Ghost's members export.
    Ghost,
}

/// The readers of one file, and the rows that could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Readers {
    /// Who wrote the file.
    pub source: ReaderSource,
    /// The readers, in the file's order, each with the line it starts on.
    pub readers: Vec<(usize, Reader)>,
    /// Each row left out: its line and why.
    pub unread: Vec<(usize, ReaderRowError)>,
}

/// Why [`read_readers`] refused a file.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ReadersError {
    /// The file is not CSV.
    #[error("the file cannot be read as CSV: {0}")]
    Csv(#[from] CsvError),
    /// Its first row is not the header of a file Khalid knows.
    #[error("its first row is not the header of a Khalid, Substack or Ghost reader list")]
    Unknown,
}

/// Why one row was left out.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ReaderRowError {
    /// The row has no address.
    #[error("it has no email address")]
    NoEmail,
    /// Its status is not pending, confirmed or unsubscribed.
    #[error("its status {0:?} is not pending, confirmed or unsubscribed")]
    Status(String),
    /// Its date is not one Khalid reads.
    #[error("its date {0:?} is not a date")]
    Date(String),
    /// A newsletter or label in it is not a slug.
    #[error("{0:?} is not a newsletter or label name")]
    Slug(String),
}

/// Reads the readers in `text`, telling the source from the header.
///
/// # Errors
///
/// Returns [`ReadersError::Csv`] when `text` is not CSV, and
/// [`ReadersError::Unknown`] when its header is not one Khalid knows.
pub fn read_readers(text: &str) -> Result<Readers, ReadersError> {
    let mut all = rows(text)?.into_iter();
    let (_, header) = all.next().ok_or(ReadersError::Unknown)?;
    let has = |name: &str| header.iter().any(|column| column.trim() == name);
    if !has("email") {
        return Err(ReadersError::Unknown);
    }
    let source = if has("subscribed_to_emails") {
        ReaderSource::Ghost
    } else if has("email_disabled") || has("active_subscription") {
        ReaderSource::Substack
    } else if has("status") {
        ReaderSource::Khalid
    } else {
        return Err(ReadersError::Unknown);
    };
    let mut readers = Readers {
        source,
        readers: Vec::new(),
        unread: Vec::new(),
    };
    for (line, fields) in all {
        let row = Fields {
            header: &header,
            fields: &fields,
        };
        match reader(source, &row) {
            Ok(reader) => readers.readers.push((line, reader)),
            Err(why) => readers.unread.push((line, why)),
        }
    }
    Ok(readers)
}

/// One row's fields, read by the header's column names.
struct Fields<'r> {
    header: &'r [String],
    fields: &'r [String],
}

impl<'r> Fields<'r> {
    /// The trimmed field under column `name`, or empty when there is none.
    fn get(&self, name: &str) -> &'r str {
        let index = self.header.iter().position(|column| column.trim() == name);
        let field = index.and_then(|index| self.fields.get(index));
        field.map_or("", |value| unguarded(value.trim()))
    }
}

/// One row of a file from `source`.
fn reader(source: ReaderSource, row: &Fields<'_>) -> Result<Reader, ReaderRowError> {
    let field = |name: &str| row.get(name);
    let email = field("email");
    if email.is_empty() {
        return Err(ReaderRowError::NoEmail);
    }
    let since = date(field("created_at"))?;
    let mut lists = Vec::new();
    let mut labels = Vec::new();
    let standing = match source {
        ReaderSource::Khalid => {
            lists = slugs(field("lists"))?;
            labels = slugs(field("labels"))?;
            match field("status") {
                "pending" => Standing::Pending,
                "confirmed" => Standing::Confirmed,
                "unsubscribed" => Standing::Unsubscribed,
                other => return Err(ReaderRowError::Status(other.to_owned())),
            }
        }
        ReaderSource::Substack => {
            if yes(field("active_subscription")) {
                labels.extend(Slug::try_new("paid").ok());
            }
            // Substack and Ghost keep no reader waiting for a confirmation.
            if yes(field("email_disabled")) {
                Standing::Unsubscribed
            } else {
                Standing::Confirmed
            }
        }
        ReaderSource::Ghost => {
            labels = ghost_labels(field("labels"));
            let paid = !field("stripe_customer_id").is_empty();
            if paid || yes(field("complimentary_plan")) {
                labels.extend(Slug::try_new("paid").ok());
            }
            if yes(field("subscribed_to_emails")) {
                Standing::Confirmed
            } else {
                Standing::Unsubscribed
            }
        }
    };
    labels.sort();
    labels.dedup();
    Ok(Reader {
        email: email.to_owned(),
        standing,
        since,
        lists,
        labels,
    })
}

/// Whether a Substack or Ghost yes-or-no column says yes.
fn yes(value: &str) -> bool {
    matches!(value.to_ascii_lowercase().as_str(), "true" | "1" | "yes")
}

/// The date in `value`: RFC 3339 as Khalid, Substack and Ghost write it,
/// or `2024-01-02 03:04:05` in UTC as older Ghost writes it.
fn date(value: &str) -> Result<Option<DateTime<Utc>>, ReaderRowError> {
    if value.is_empty() {
        return Ok(None);
    }
    if let Ok(at) = DateTime::parse_from_rfc3339(value) {
        return Ok(Some(at.with_timezone(&Utc)));
    }
    NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
        .map(|at| Some(at.and_utc()))
        .map_err(|_| ReaderRowError::Date(value.to_owned()))
}

/// The slugs in `value`, with spaces between, as Khalid writes them.
fn slugs(value: &str) -> Result<Vec<Slug>, ReaderRowError> {
    let names = value.split_whitespace();
    names
        .map(|name| Slug::try_new(name).map_err(|_| ReaderRowError::Slug(name.to_owned())))
        .collect()
}

/// Ghost's labels, names with commas between, as slugs: `VIP` is `vip`.
/// A name with no letter or digit to keep, such as an emoji, is left out.
fn ghost_labels(value: &str) -> Vec<Slug> {
    let Ok(english) = Locale::try_new("en") else {
        return Vec::new();
    };
    let names = value
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty());
    names
        .filter_map(|name| Slug::from_title(name, &english, SlugMode::Ascii).ok())
        .collect()
}

/// `value` without the quote [`write_readers`] puts before a field a
/// spreadsheet would run as a formula.
fn unguarded(value: &str) -> &str {
    match value.strip_prefix('\'') {
        Some(rest) if rest.starts_with(['=', '+', '-', '@']) => rest,
        _ => value,
    }
}

/// `readers` as Khalid's export: a header, then one row each, in order.
/// A field a spreadsheet would run as a formula gets a quote before it,
/// which [`read_readers`] takes off again.
pub fn write_readers(readers: &[Reader]) -> String {
    let mut text = KHALID_COLUMNS.join(",");
    text.push('\n');
    for reader in readers {
        let names = |slugs: &[Slug]| {
            let names: Vec<&str> = slugs.iter().map(Slug::as_str).collect();
            names.join(" ")
        };
        let since = reader.since.map(|at| at.to_rfc3339()).unwrap_or_default();
        let fields = [
            reader.email.as_str(),
            reader.standing.as_str(),
            since.as_str(),
            &names(&reader.lists),
            &names(&reader.labels),
        ];
        let fields: Vec<String> = fields.iter().map(|field| csv_field(field)).collect();
        text.push_str(&fields.join(","));
        text.push('\n');
    }
    text
}

/// `value` as one CSV field: quoted when it holds a comma, a quote or a
/// line break, with a quote first when it starts like a formula.
fn csv_field(value: &str) -> String {
    let guarded = if value.starts_with(['=', '+', '-', '@']) {
        format!("'{value}")
    } else {
        value.to_owned()
    };
    if guarded.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", guarded.replace('"', "\"\""))
    } else {
        guarded
    }
}
