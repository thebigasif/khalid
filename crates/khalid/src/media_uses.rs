//! Which of a site's own files still name a media library file, read from
//! `site.toml`, `content/` and `theme/`, so `khalid serve` never deletes a
//! file a post, a page or the theme shows.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use khalid_api::content_store::BoxFuture;
use khalid_api::media_uses::{MediaUses, MediaUsesError};
use khalid_media::MediaItem;

/// The folders and files of a site that may name a library file.
const LOOKED_IN: [&str; 3] = ["site.toml", "content", "theme"];

/// The kinds of file read; pictures and other files cannot name one.
const TEXT: [&str; 10] = [
    "md", "markdown", "toml", "yaml", "yml", "json", "html", "css", "txt", "jinja",
];

/// The site in a folder, read afresh each time, since a delete is rare and
/// a post saved a moment ago must count.
#[derive(Debug, Clone)]
pub(crate) struct SiteMediaUses {
    site: PathBuf,
}

impl SiteMediaUses {
    /// The site in the `site` folder.
    pub(crate) fn new(site: &Path) -> Self {
        Self {
            site: site.to_path_buf(),
        }
    }
}

impl MediaUses for SiteMediaUses {
    fn using<'a>(
        &'a self,
        item: &'a MediaItem,
    ) -> BoxFuture<'a, Result<Vec<String>, MediaUsesError>> {
        // Every address of the file ends in `<key>/<name>`, on the site or
        // in a bucket.
        let needle = format!("{}/{}", item.id, item.name.as_str());
        let site = self.site.clone();
        Box::pin(async move {
            let found = tokio::task::spawn_blocking(move || naming(&site, &needle)).await;
            match found {
                Ok(found) => found.map_err(|error| MediaUsesError(Box::new(error))),
                Err(error) => Err(MediaUsesError(Box::new(error))),
            }
        })
    }
}

/// The site's text files that hold `needle`, as paths from `site` joined
/// by `/`, in order.
fn naming(site: &Path, needle: &str) -> io::Result<Vec<String>> {
    let mut found = Vec::new();
    for top in LOOKED_IN {
        let mut path = site.to_path_buf();
        path.push(top);
        look(&path, top, needle, &mut found)?;
    }
    found.sort();
    Ok(found)
}

/// Adds `path`, shown as `shown`, to `found` when it is a text file that
/// holds `needle`, or every such file inside it when it is a folder. A
/// link is never followed, so nothing outside the site is read.
fn look(path: &Path, shown: &str, needle: &str, found: &mut Vec<String>) -> io::Result<()> {
    let kind = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata.file_type(),
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };
    if kind.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
                continue;
            };
            look(&entry.path(), &format!("{shown}/{name}"), needle, found)?;
        }
    } else if kind.is_file() && is_text(path) && holds(&fs::read(path)?, needle.as_bytes()) {
        found.push(shown.to_owned());
    }
    Ok(())
}

/// Whether `bytes` hold `needle`. Bytes, not text, so one file saved in
/// another encoding cannot stop a delete.
fn holds(bytes: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && bytes.windows(needle.len()).any(|window| window == needle)
}

fn is_text(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| TEXT.contains(&extension.to_ascii_lowercase().as_str()))
}

#[cfg(test)]
mod tests {
    use super::naming;

    /// Writes `text` to `relative` inside `site`, making its folders.
    fn write(site: &std::path::Path, relative: &str, text: &str) {
        let mut path = site.to_path_buf();
        path.push(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }

    #[test]
    fn finds_every_text_file_that_names_the_file_and_nothing_else() {
        let site = tempfile::tempdir().unwrap();
        let file = "media/library/3f2a9c1e0b57d0aa/choosing-a-crm.jpg";
        let picture = format!("![Two CRM dashboards](/blog/{file})\n");
        write(site.path(), "content/posts/choosing-a-crm.md", &picture);
        write(site.path(), "content/pages/about.md", "No pictures.\n");
        write(
            site.path(),
            "theme/tokens.toml",
            &format!("logo = \"{file}\"\n"),
        );
        write(site.path(), "content/posts/photo.jpg", &picture);
        write(site.path(), "site.toml", "name = \"Field Notes\"\n");
        let found = naming(site.path(), "3f2a9c1e0b57d0aa/choosing-a-crm.jpg").unwrap();
        assert_eq!(
            found,
            vec!["content/posts/choosing-a-crm.md", "theme/tokens.toml"]
        );
        let other = naming(site.path(), "3f2a9c1e0b57d0aa/another-name.jpg").unwrap();
        assert_eq!(other, Vec::<String>::new());
    }

    #[test]
    fn a_file_that_is_not_utf_8_is_still_read() {
        let site = tempfile::tempdir().unwrap();
        let mut latin = b"Photo \xe9 here: ".to_vec();
        latin.extend_from_slice(b"/blog/media/library/3f2a9c1e0b57d0aa/cafe.jpg\n");
        let mut path = site.path().to_path_buf();
        path.push("content");
        std::fs::create_dir_all(&path).unwrap();
        path.push("old.md");
        std::fs::write(&path, latin).unwrap();
        write(site.path(), "content/note.txt", "Nothing here.\n");
        let found = naming(site.path(), "3f2a9c1e0b57d0aa/cafe.jpg").unwrap();
        assert_eq!(found, vec!["content/old.md"]);
    }

    #[test]
    fn a_site_with_no_content_names_nothing() {
        let site = tempfile::tempdir().unwrap();
        assert_eq!(
            naming(site.path(), "a/b.jpg").unwrap(),
            Vec::<String>::new()
        );
    }
}
