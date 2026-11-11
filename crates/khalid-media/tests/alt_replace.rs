//! One find-and-replace changes the alt text of every library item that
//! holds the words, after a preview that changes nothing (plan 5.1).

use std::path::Path;
use std::sync::Arc;

use chrono::{TimeZone, Utc};
use khalid_core::{FixedClock, Locale};
use khalid_media::{
    AltReplace, Folder, Library, MediaEdit, MediaItem, ReplaceAltError, Storage, Upload, Uploaded,
};

#[cfg(test)]
async fn open(site: &Path, hour: u32) -> Library {
    let clock = FixedClock::new(Utc.with_ymd_and_hms(2026, 11, 10, hour, 0, 0).unwrap());
    let english = Locale::try_new("en").unwrap();
    Library::open(site, Storage::local(site), &english, Arc::new(clock))
        .await
        .unwrap()
}

/// A file in `library` whose alt text is `alt`.
#[cfg(test)]
async fn with_alt(library: &Library, name: &str, alt: &str) -> MediaItem {
    let bytes = format!("%PDF-1.7\n{name}\n%%EOF\n").into_bytes();
    let upload = Upload::new(name, Folder::root(), bytes, "asif");
    let item = match library.upload(upload).await.unwrap() {
        Uploaded::New(item) | Uploaded::Duplicate(item) => item,
    };
    let mut edit = MediaEdit::new(item.updated_at);
    edit.alt = Some(alt.to_owned());
    library.edit(&item.id, edit, "asif").await.unwrap()
}

#[tokio::test]
async fn a_preview_lists_every_change_and_writes_nothing() {
    let site = tempfile::tempdir().unwrap();
    let library = open(site.path(), 8).await;
    let crm = with_alt(&library, "crm.pdf", "A CRM checklist from Acme Ltd").await;
    let plain = with_alt(&library, "plain.pdf", "A blank page").await;
    let replace = AltReplace::new("Acme Ltd", "Acme").unwrap();
    let changes = library.alt_changes(&replace).unwrap();
    assert_eq!(changes.len(), 1, "only the item that holds the words");
    let change = changes.first().unwrap();
    assert_eq!(change.id, crm.id);
    assert_eq!(change.before, "A CRM checklist from Acme Ltd");
    assert_eq!(change.after, "A CRM checklist from Acme");
    assert_eq!(library.get(&crm.id), Some(crm), "a preview changes nothing");
    assert_eq!(library.get(&plain.id), Some(plain));
}

#[tokio::test]
async fn a_replace_changes_every_match_and_survives_a_restart() {
    let site = tempfile::tempdir().unwrap();
    let first = open(site.path(), 8).await;
    let one = with_alt(
        &first,
        "one.pdf",
        "Acme Ltd logo beside the Acme Ltd office",
    )
    .await;
    let two = with_alt(&first, "two.pdf", "Staff at Acme Ltd").await;
    let other = with_alt(&first, "other.pdf", "acme ltd in small letters").await;
    let library = open(site.path(), 10).await;
    let replace = AltReplace::new("Acme Ltd", "Acme").unwrap();
    let changes = library.replace_alt(&replace, "editor").await.unwrap();
    assert_eq!(changes.len(), 2, "matching is exact, so small letters stay");
    let reopened = open(site.path(), 11).await;
    let saved = reopened.get(&one.id).unwrap();
    assert_eq!(saved.alt, "Acme logo beside the Acme office");
    assert_eq!(saved.updated_by, "editor");
    assert_eq!(
        saved.updated_at,
        Utc.with_ymd_and_hms(2026, 11, 10, 10, 0, 0).unwrap()
    );
    assert_eq!(reopened.get(&two.id).unwrap().alt, "Staff at Acme");
    assert_eq!(reopened.get(&other.id), Some(other));
}

#[tokio::test]
async fn a_replace_that_empties_alt_text_trims_it() {
    let site = tempfile::tempdir().unwrap();
    let library = open(site.path(), 8).await;
    let item = with_alt(&library, "photo.pdf", "IMG_0042 ").await;
    let replace = AltReplace::new("IMG_0042", "").unwrap();
    let changes = library.replace_alt(&replace, "editor").await.unwrap();
    assert_eq!(
        changes.first().map(|change| change.after.as_str()),
        Some("")
    );
    assert_eq!(library.get(&item.id).unwrap().alt, "");
}

#[tokio::test]
async fn a_replace_that_makes_alt_text_too_long_changes_nothing() {
    let site = tempfile::tempdir().unwrap();
    let library = open(site.path(), 8).await;
    let short = with_alt(&library, "short.pdf", "x").await;
    let long = with_alt(&library, "long.pdf", &"x".repeat(11)).await;
    let replace = AltReplace::new("x", &"y".repeat(100)).unwrap();
    let error = library.replace_alt(&replace, "editor").await.unwrap_err();
    assert!(
        matches!(&error, ReplaceAltError::TooLong { id, limit: 1000 } if *id == long.id),
        "{error:?}"
    );
    assert_eq!(library.get(&short.id), Some(short), "nothing is written");
}

#[test]
fn an_empty_find_is_refused() {
    assert!(AltReplace::new("", "Acme").is_none());
    assert!(AltReplace::new("   ", "Acme").is_none());
}
