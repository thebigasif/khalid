//! `khalid import substack` and `khalid import ghost` bring the reader list
//! that sits beside the export into `data/site.db` (backlog row 9): each
//! reader keeps their status and gets no email, and a dry run only counts.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use khalid_api::resources::SubscriberStatus;
use khalid_core::Slug;
use khalid_db::{Database, SubscriberTable};
use tempfile::TempDir;

/// Substack's `email_list` file: a reader, a paying one, one who turned
/// emails off, and a row with no address.
const EMAIL_LIST: &str = "email,active_subscription,expiry,plan,email_disabled,created_at\n\
lena@example.com,false,,other,false,2021-03-04T05:06:07.000Z\n\
omar@example.com,true,2027-01-01T00:00:00.000Z,yearly,false,2022-01-01T00:00:00.000Z\n\
ines@example.org,false,,other,true,2020-05-06T07:08:09.000Z\n\
not-an-address,false,,other,false,\n";

/// A file inside `site`.
#[cfg(test)]
fn file(site: &Path, path: &str) -> PathBuf {
    let mut file = site.to_path_buf();
    file.push(path);
    file
}

/// A site with the demo's settings, and a Substack export with one post
/// and its `email_list` file.
#[cfg(test)]
fn site_and_export() -> (TempDir, PathBuf) {
    let dir = env!("CARGO_MANIFEST_DIR");
    let temp = tempfile::tempdir().unwrap();
    let settings = format!("{dir}/../../fixtures/demo-site/site.toml");
    fs::copy(settings, file(temp.path(), "site.toml")).unwrap();
    fs::create_dir(file(temp.path(), "content")).unwrap();
    let posts = file(temp.path(), "substack/posts");
    fs::create_dir_all(&posts).unwrap();
    fs::write(file(&posts, "140062712.why-rye.html"), "<p>Rye.</p>").unwrap();
    let list = "post_id,post_date,is_published,type,audience,title,subtitle\n\
        140062712.why-rye,2024-03-12T09:30:44.123Z,true,newsletter,everyone,Why rye,\n";
    fs::write(file(temp.path(), "substack/posts.csv"), list).unwrap();
    let readers = file(temp.path(), "substack/email_list.rye-notes.csv");
    fs::write(readers, EMAIL_LIST).unwrap();
    let export = file(temp.path(), "substack");
    (temp, export)
}

/// Runs `khalid --site <site>` with `args`, and returns the exit code and
/// what it printed.
#[cfg(test)]
fn khalid(site: &Path, args: &[&str]) -> (Option<i32>, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap()])
        .args(args)
        .output()
        .unwrap();
    let mut printed = String::from_utf8_lossy(&output.stdout).into_owned();
    printed.push_str(&String::from_utf8_lossy(&output.stderr));
    (output.status.code(), printed)
}

#[test]
fn a_dry_run_counts_the_readers_and_opens_no_database() {
    let (site, export) = site_and_export();
    let args = ["import", "substack", export.to_str().unwrap(), "--dry-run"];
    let (code, printed) = khalid(site.path(), &args);
    assert_eq!(code, Some(0), "{printed}");
    assert!(
        printed.contains("email_list.rye-notes.csv holds 3 readers, and 1 row Khalid leaves out."),
        "{printed}"
    );
    assert!(!file(site.path(), "data/site.db").exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn a_substack_import_brings_its_readers_with_their_status() {
    let (site, export) = site_and_export();
    let (code, printed) = khalid(
        site.path(),
        &["import", "substack", export.to_str().unwrap()],
    );
    assert_eq!(code, Some(0), "{printed}");
    assert!(
        printed.contains("Brought in 3 readers from email_list.rye-notes.csv, each as they stood on Substack, and sent them no email."),
        "{printed}"
    );
    assert!(
        printed.contains("Left out 1 row: line 5, its address cannot take mail."),
        "{printed}"
    );

    let db = Database::open(&file(site.path(), "data/site.db"))
        .await
        .unwrap();
    db.migrate().await.unwrap();
    let id = db.site(&Slug::try_new("main").unwrap()).await.unwrap();
    let everyone = SubscriberTable::new(db.clone(), id)
        .everyone()
        .await
        .unwrap();
    let standing: Vec<_> = everyone
        .iter()
        .map(|each| (each.email.as_str(), each.status))
        .collect();
    assert_eq!(
        standing,
        [
            ("lena@example.com", SubscriberStatus::Confirmed),
            ("omar@example.com", SubscriberStatus::Confirmed),
            ("ines@example.org", SubscriberStatus::Unsubscribed),
        ]
    );
    db.close().await;
}

#[test]
fn a_reader_list_that_is_none_stops_the_import_before_it_writes() {
    let (site, export) = site_and_export();
    let readers = file(site.path(), "substack/email_list.rye-notes.csv");
    fs::write(readers, "post_id,title\n1,Why rye\n").unwrap();
    let (code, printed) = khalid(
        site.path(),
        &["import", "substack", export.to_str().unwrap()],
    );
    assert_ne!(code, Some(0), "{printed}");
    assert!(
        printed.contains("Khalid cannot read the reader list email_list.rye-notes.csv"),
        "{printed}"
    );
    assert!(!file(site.path(), "content/en/posts/2024-03-12-why-rye.md").exists());
}

#[test]
fn a_ghost_import_finds_the_members_file_beside_its_export() {
    let (site, _) = site_and_export();
    let dir = env!("CARGO_MANIFEST_DIR");
    let sample = format!("{dir}/../khalid-import/fixtures/ghost/ghost-5.json");
    let export = file(site.path(), "ghost/rye-notes.ghost.2026-11-20.json");
    fs::create_dir_all(file(site.path(), "ghost")).unwrap();
    fs::copy(sample, &export).unwrap();
    let members = "id,email,name,note,subscribed_to_emails,complimentary_plan,\
        stripe_customer_id,created_at,deleted_at,labels,tiers\n\
        6512,kim@example.com,Kim,,true,false,,2023-01-02T03:04:05.000Z,,,\n";
    fs::write(
        file(site.path(), "ghost/rye-notes-members.2026-11-20.csv"),
        members,
    )
    .unwrap();
    let args = ["import", "ghost", export.to_str().unwrap(), "--dry-run"];
    let (code, printed) = khalid(site.path(), &args);
    assert_eq!(code, Some(0), "{printed}");
    assert!(
        printed.contains("rye-notes-members.2026-11-20.csv holds 1 reader."),
        "{printed}"
    );
}
