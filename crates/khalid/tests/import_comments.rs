//! `khalid import wordpress` brings the comments under each published post
//! into `data/site.db` (decision D-4), with their time, their thread and
//! their place in moderation.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use khalid_comments::{CommentAuthor, CommentStatus, CommentStore};
use khalid_core::Slug;
use khalid_db::{CommentTable, Database};
use tempfile::TempDir;

/// A file inside `site`.
#[cfg(test)]
fn file(site: &Path, path: &str) -> PathBuf {
    let mut file = site.to_path_buf();
    file.push(path);
    file
}

/// A site with the demo's settings and no content yet, and the importer's
/// sample export with a reader's reply to its comment and a spam comment.
#[cfg(test)]
fn site_and_export() -> (TempDir, PathBuf) {
    let dir = env!("CARGO_MANIFEST_DIR");
    let temp = tempfile::tempdir().unwrap();
    let settings = format!("{dir}/../../fixtures/demo-site/site.toml");
    fs::copy(settings, file(temp.path(), "site.toml")).unwrap();
    fs::create_dir(file(temp.path(), "content")).unwrap();
    let sample = format!("{dir}/../khalid-import/fixtures/wordpress/basic.xml");
    let text = fs::read_to_string(sample).unwrap();
    let first = text
        .split("<wp:comment>")
        .nth(1)
        .and_then(|rest| rest.split("</wp:comment>").next())
        .unwrap();
    let reply = first
        .replace("<wp:comment_id>1<", "<wp:comment_id>2<")
        .replace("<wp:comment_parent>0<", "<wp:comment_parent>1<")
        .replace("A reader", "Mara")
        .replace("Thanks!", "Glad it helped.\n\nFeed it twice in summer.")
        .replace(
            "[CDATA[1]]></wp:comment_approved>",
            "[CDATA[0]]></wp:comment_approved>",
        )
        .replace("10:00:00", "12:30:00");
    let spam = first
        .replace("<wp:comment_id>1<", "<wp:comment_id>3<")
        .replace(
            "[CDATA[1]]></wp:comment_approved>",
            "[CDATA[spam]]></wp:comment_approved>",
        );
    let added =
        format!("</wp:comment><wp:comment>{reply}</wp:comment><wp:comment>{spam}</wp:comment>");
    let export = file(temp.path(), "export.xml");
    fs::write(&export, text.replacen("</wp:comment>", &added, 1)).unwrap();
    (temp, export)
}

#[tokio::test(flavor = "multi_thread")]
async fn a_wordpress_import_stores_each_comment_under_its_post() {
    let (site, export) = site_and_export();
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.path().to_str().unwrap()])
        .args(["import", "wordpress", export.to_str().unwrap()])
        .output()
        .unwrap();
    let printed = String::from_utf8_lossy(&output.stdout).into_owned();
    assert_eq!(output.status.code(), Some(0), "{printed}");
    assert!(
        printed.contains("Comments coming in: 2, of which 1 wait for approval.\nComments left behind: spam (1).\n"),
        "{printed}"
    );
    assert!(
        printed.contains("Stored 2 comments under their posts.\n"),
        "{printed}"
    );

    let db = Database::open(&file(site.path(), "data/site.db"))
        .await
        .unwrap();
    db.migrate().await.unwrap();
    let id = db.site(&Slug::try_new("main").unwrap()).await.unwrap();
    let mut stored = CommentTable::new(db.clone(), id)
        .newest(None, 10)
        .await
        .unwrap();
    stored.reverse();
    let [first, reply] = stored.as_slice() else {
        panic!("{stored:?}")
    };
    let post = "content/en/posts/2024-03-12-keep-a-starter-alive.md";
    assert_eq!((first.post.as_str(), reply.post.as_str()), (post, post));
    assert_eq!(first.status, CommentStatus::Approved);
    assert_eq!(first.created_at.to_rfc3339(), "2024-03-13T10:00:00+00:00");
    let CommentAuthor::Reader { name, email } = &first.author else {
        panic!("{:?}", first.author)
    };
    assert_eq!(
        (
            name.as_str(),
            email.as_ref().map(khalid_comments::ReaderEmail::as_str)
        ),
        ("A reader", Some("reader@example.com"))
    );
    assert_eq!(reply.parent, Some(first.id));
    assert_eq!(reply.status, CommentStatus::Pending);
    assert_eq!(
        reply.text.as_str(),
        "Glad it helped.\n\nFeed it twice in summer."
    );
    db.close().await;
}
