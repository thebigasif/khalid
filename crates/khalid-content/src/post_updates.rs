//! A post's Updates list: dated notes of what changed, shown to readers
//! and feeding the post's last real update (plan 4.12).

use std::cmp::Reverse;

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

/// One dated note of what changed in a post, such as `2026-10-20:
/// Refreshed the 2026 numbers.`
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "RawUpdate")]
#[non_exhaustive]
pub struct PostUpdate {
    /// The day the change happened, never the day it was saved.
    pub date: NaiveDate,
    /// What changed, in one plain sentence with no spaces at either end.
    pub note: String,
}

/// An update as written, before its rules are checked.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawUpdate {
    #[serde(deserialize_with = "crate::post::date")]
    date: Option<NaiveDate>,
    note: String,
}

impl TryFrom<RawUpdate> for PostUpdate {
    type Error = String;

    fn try_from(raw: RawUpdate) -> Result<Self, Self::Error> {
        let date = raw
            .date
            .ok_or("an update needs a date written as YYYY-MM-DD, such as date: 2026-10-20")?;
        let note = raw.note.trim();
        if note.is_empty() {
            return Err(
                "an update needs a note that says what changed, such as note: Refreshed the 2026 numbers"
                    .to_owned(),
            );
        }
        if note.contains(['\n', '\r']) {
            return Err(format!(
                "the update note {note:?} runs over more than one line; write it on one line"
            ));
        }
        Ok(Self {
            date,
            note: note.to_owned(),
        })
    }
}

/// Reads the Updates list, newest first, refusing two updates on one day.
/// An empty list, or none at all, is the same as no field.
pub(crate) fn updates<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<PostUpdate>, D::Error> {
    let mut updates: Vec<PostUpdate> = crate::blank::or_empty(deserializer)?;
    updates.sort_by_key(|update| Reverse(update.date));
    let days = updates.iter().map(|update| update.date);
    if let Some((twice, _)) = days.clone().zip(days.skip(1)).find(|(a, b)| a == b) {
        return Err(serde::de::Error::custom(format!(
            "two updates are dated {twice}; join them into one note"
        )));
    }
    Ok(updates)
}
