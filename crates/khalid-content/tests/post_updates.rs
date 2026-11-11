//! Reading a post's Updates list: dated notes of what changed, newest
//! first, with the newest date standing as the post's last real update.

use std::path::Path;

use chrono::NaiveDate;
use khalid_content::parse_post;

#[cfg(test)]
fn front(fields: &str) -> khalid_content::PostFront {
    parse_post(
        Path::new("posts/a.md"),
        &format!("---\n{fields}---\nText.\n"),
    )
    .unwrap()
    .front
}

#[cfg(test)]
fn message(fields: &str) -> String {
    parse_post(
        Path::new("posts/a.md"),
        &format!("---\n{fields}---\nText.\n"),
    )
    .unwrap_err()
    .to_string()
}

#[cfg(test)]
fn day(text: &str) -> NaiveDate {
    NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap()
}

#[test]
fn reads_the_updates_newest_first_and_trims_each_note() {
    let front = front(
        "updates:\n  - date: 2026-03-02\n    note: Added the 2026 survey.\n  \
         - date: 2026-10-20\n    note: '  Refreshed the numbers. '\n",
    );
    let read: Vec<_> = front
        .updates
        .iter()
        .map(|update| (update.date.to_string(), update.note.as_str()))
        .collect();
    assert_eq!(
        read,
        [
            ("2026-10-20".to_owned(), "Refreshed the numbers."),
            ("2026-03-02".to_owned(), "Added the 2026 survey."),
        ]
    );
}

#[test]
fn the_newest_update_is_the_last_real_update() {
    let front = front("updated: 2026-05-01\nupdates:\n  - date: 2026-10-20\n    note: New data.\n");
    assert_eq!(front.last_updated(), Some(day("2026-10-20")));
}

#[test]
fn a_later_updated_date_still_wins_over_older_updates() {
    let front = front("updated: 2026-11-01\nupdates:\n  - date: 2026-10-20\n    note: New data.\n");
    assert_eq!(front.last_updated(), Some(day("2026-11-01")));
}

#[test]
fn no_updates_and_no_updated_date_means_no_last_update() {
    let front = front("updates:\n");
    assert_eq!(front.updates, []);
    assert_eq!(front.last_updated(), None);
}

#[test]
fn refuses_an_update_with_no_note() {
    let message = message("updates:\n  - date: 2026-10-20\n    note: ' '\n");
    assert!(
        message.contains(
            "an update needs a note that says what changed, such as note: Refreshed the 2026 numbers"
        ),
        "{message}"
    );
}

#[test]
fn refuses_an_update_with_no_date() {
    let message = message("updates:\n  - note: New data.\n");
    assert!(message.contains("date"), "{message}");
}

#[test]
fn refuses_an_update_on_a_day_that_does_not_exist() {
    let message = message("updates:\n  - date: 2026-02-30\n    note: New data.\n");
    assert!(
        message.contains("the date must be a real day written as YYYY-MM-DD"),
        "{message}"
    );
}

#[test]
fn refuses_two_updates_on_one_day() {
    let message = message(
        "updates:\n  - date: 2026-10-20\n    note: One.\n  - date: 2026-10-20\n    note: Two.\n",
    );
    assert!(
        message.contains("two updates are dated 2026-10-20; join them into one note"),
        "{message}"
    );
}

#[test]
fn refuses_a_note_over_more_than_one_line() {
    let message = message("updates:\n  - date: 2026-10-20\n    note: \"One.\\nTwo.\"\n");
    assert!(
        message.contains("runs over more than one line; write it on one line"),
        "{message}"
    );
}

#[test]
fn refuses_an_unknown_field_in_an_update() {
    let message =
        message("updates:\n  - date: 2026-10-20\n    note: New data.\n    who: lena-vogt\n");
    assert!(message.contains("who"), "{message}");
}
