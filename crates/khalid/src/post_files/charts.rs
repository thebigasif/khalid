//! The CSV files charts read from beside their posts: a `:::chart` with
//! `src="traffic.csv"` and no lines of its own is drawn from the file in
//! the post's folder.

use std::borrow::Cow;
use std::fs;
use std::path::Path;

use khalid_content::{SiteContent, with_chart_files};
use khalid_core::SitePath;

/// The largest CSV file a chart reads. A chart draws at most 40 rows, so a
/// bigger file is a mistake, and it is left unread.
const MOST_BYTES: u64 = 64 * 1024;

/// `content` with each chart that names a CSV file beside its post given
/// the file's lines, read from the site in `folder`, so the build draws
/// it. The files on disk are never changed. Content with no such chart,
/// most sites, is not copied.
pub(crate) fn read<'content>(
    folder: &Path,
    content: &'content SiteContent,
) -> Cow<'content, SiteContent> {
    let wants = |body: &str| body.contains(":::chart{") && body.contains(" src=\"");
    if !content.posts.iter().any(|doc| wants(&doc.body)) {
        return Cow::Borrowed(content);
    }
    let mut content = content.clone();
    for doc in content.posts.iter_mut().filter(|doc| wants(&doc.body)) {
        let beside = doc.path.parent().and_then(Path::to_str).unwrap_or_default();
        let read = |name: &str| file(folder, &format!("{beside}/{name}"));
        if let Cow::Owned(body) = with_chart_files(&doc.body, read) {
            doc.body = body;
        }
    }
    Cow::Owned(content)
}

/// The text of the file at `relative` in the site in `folder`, when it is
/// there, inside the folder, small enough and text.
fn file(folder: &Path, relative: &str) -> Option<String> {
    let path = SitePath::within(folder, relative).ok()?;
    let size = fs::metadata(path.as_path()).ok()?.len();
    (size <= MOST_BYTES)
        .then(|| fs::read_to_string(path.as_path()).ok())
        .flatten()
}

#[cfg(test)]
mod tests {
    use khalid_content::{ContentTree, read_content};

    use super::*;

    #[cfg(test)]
    fn site(csv: &str) -> tempfile::TempDir {
        let site = tempfile::tempdir().unwrap();
        let mut post = site.path().to_path_buf();
        post.push("content/en/posts/numbers");
        fs::create_dir_all(&post).unwrap();
        let body = "---\ntitle: Numbers\ndate: 2026-10-01\n---\n\n:::chart{title=\"Visits\" src=\"visits.csv\"}\n:::\n";
        fs::write(SitePath::within(&post, "index.md").unwrap().as_path(), body).unwrap();
        fs::write(
            SitePath::within(&post, "visits.csv").unwrap().as_path(),
            csv,
        )
        .unwrap();
        site
    }

    #[test]
    fn a_chart_gets_the_lines_of_the_file_beside_its_post() {
        let site = site("Month, Visits\nJan, 3\n");
        let content = read_content(&ContentTree::walk(site.path()).unwrap()).unwrap();
        let drawn = read(site.path(), &content);
        assert!(matches!(drawn, Cow::Owned(_)));
        assert!(
            drawn.posts[0]
                .body
                .contains("src=\"visits.csv\"}\nMonth, Visits\nJan, 3\n:::")
        );
        assert!(
            !content.posts[0].body.contains("Jan, 3"),
            "the content read stays as written"
        );
    }

    #[test]
    fn content_without_file_charts_is_not_copied_and_big_files_are_not_read() {
        let site = site(&"Jan, 3\n".repeat(20_000));
        let content = read_content(&ContentTree::walk(site.path()).unwrap()).unwrap();
        let drawn = read(site.path(), &content);
        assert!(!drawn.posts[0].body.contains("Jan, 3"));
        assert!(matches!(
            read(site.path(), &SiteContent::default()),
            Cow::Borrowed(_)
        ));
    }
}
