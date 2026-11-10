//! An item can be deleted with its file, but only from the copy last read
//! (RULES.md BL-API-7), and the library lists the folders its items are in.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::{TimeZone, Utc};
use khalid_core::{FixedClock, Locale};
use khalid_media::{
    ContentKey, DeleteMediaError, Folder, FolderCount, Library, MediaEdit, MediaItem, Storage,
    Upload, Uploaded,
};

const PDF: &[u8] = b"%PDF-1.7\nA checklist for choosing a CRM\n%%EOF\n";
const TEXT: &[u8] = b"Ten questions to ask a CRM vendor.\n";

#[cfg(test)]
async fn open(site: &Path) -> Library {
    let clock = FixedClock::new(Utc.with_ymd_and_hms(2026, 11, 10, 8, 0, 0).unwrap());
    let english = Locale::try_new("en").unwrap();
    Library::open(site, Storage::local(site), &english, Arc::new(clock))
        .await
        .unwrap()
}

#[cfg(test)]
async fn uploaded(library: &Library, name: &str, folder: &str, bytes: &[u8]) -> MediaItem {
    let folder = Folder::try_new(folder).unwrap();
    let upload = Upload::new(name, folder, bytes.to_vec(), "asif");
    match library.upload(upload).await.unwrap() {
        Uploaded::New(item) | Uploaded::Duplicate(item) => item,
    }
}

/// The folder that holds `item`'s file and its `item.toml`.
#[cfg(test)]
fn item_folder(site: &Path, item: &MediaItem) -> PathBuf {
    let mut folder = site.to_path_buf();
    folder.push("media");
    folder.push(item.id.as_str());
    folder
}

/// The PDF's file in `folder`.
#[cfg(test)]
fn checklist(folder: &Path) -> PathBuf {
    let mut path = folder.to_path_buf();
    path.push("checklist.pdf");
    path
}

#[tokio::test]
async fn a_delete_removes_the_item_and_its_file_for_good() {
    let site = tempfile::tempdir().unwrap();
    let library = open(site.path()).await;
    let item = uploaded(&library, "checklist.pdf", "", PDF).await;
    let folder = item_folder(site.path(), &item);
    assert!(checklist(&folder).is_file());
    let deleted = library.delete(&item.id, item.updated_at).await.unwrap();
    assert_eq!(deleted, item);
    assert_eq!(library.get(&item.id), None);
    assert!(!folder.exists(), "the file and its item.toml are gone");
    assert_eq!(open(site.path()).await.get(&item.id), None);
}

#[tokio::test]
async fn a_delete_made_on_an_old_copy_or_an_unknown_item_changes_nothing() {
    let site = tempfile::tempdir().unwrap();
    let library = open(site.path()).await;
    let item = uploaded(&library, "checklist.pdf", "", PDF).await;
    let mut edit = MediaEdit::new(item.updated_at);
    edit.caption = Some("Ten questions for any CRM vendor.".to_owned());
    let saved = library.edit(&item.id, edit, "editor").await.unwrap();
    let error = library.delete(&item.id, item.updated_at).await.unwrap_err();
    assert!(
        matches!(&error, DeleteMediaError::Stale { updated_at, updated_by }
            if *updated_at == saved.updated_at && updated_by == "editor"),
        "{error:?}"
    );
    assert_eq!(library.get(&item.id), Some(saved.clone()));
    let other = ContentKey::parse("0000000000000000").unwrap();
    let error = library.delete(&other, item.updated_at).await.unwrap_err();
    assert!(matches!(error, DeleteMediaError::NotFound), "{error:?}");
    assert!(checklist(&item_folder(site.path(), &item)).is_file());
}

#[tokio::test]
async fn a_delete_whose_file_is_already_gone_still_removes_the_item() {
    let site = tempfile::tempdir().unwrap();
    let library = open(site.path()).await;
    let item = uploaded(&library, "checklist.pdf", "", PDF).await;
    let folder = item_folder(site.path(), &item);
    std::fs::remove_file(checklist(&folder)).unwrap();
    library.delete(&item.id, item.updated_at).await.unwrap();
    assert_eq!(library.get(&item.id), None);
    assert!(!folder.exists());
}

#[tokio::test]
async fn folders_are_listed_by_name_with_their_parents_and_counts() {
    let site = tempfile::tempdir().unwrap();
    let library = open(site.path()).await;
    assert_eq!(library.folders(), Vec::new(), "an empty library has none");
    uploaded(&library, "checklist.pdf", "guides/2026", PDF).await;
    uploaded(&library, "questions.txt", "guides/2026", TEXT).await;
    uploaded(&library, "notes.txt", "", b"Notes.\n").await;
    uploaded(&library, "cover.txt", "archive", b"Cover.\n").await;
    let folders: Vec<(String, usize)> = library
        .folders()
        .into_iter()
        .map(|FolderCount { folder, items, .. }| (folder.as_str().to_owned(), items))
        .collect();
    assert_eq!(
        folders,
        vec![
            ("archive".to_owned(), 1),
            ("guides".to_owned(), 0),
            ("guides/2026".to_owned(), 2),
        ],
        "the top is never listed, and guides holds only a folder"
    );
}

#[test]
fn a_folder_knows_its_parent() {
    let folder = Folder::try_new("guides/2026/crm").unwrap();
    assert_eq!(
        folder.parent(),
        Some(Folder::try_new("guides/2026").unwrap())
    );
    assert_eq!(
        Folder::try_new("guides").unwrap().parent(),
        Some(Folder::root())
    );
    assert_eq!(Folder::root().parent(), None);
}
