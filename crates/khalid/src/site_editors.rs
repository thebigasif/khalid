//! Who last changed each post through Khalid: the Admin API, the admin,
//! `khalid mcp`, `khalid publish` or the scheduler. The post files cannot
//! say, and nothing derived goes into `content/` (RULES.md BL-FS-1), so it
//! is kept in `data/post-editors.txt` (backlog row 6, G52).
//!
//! Each line names a post file, the time the file was last written, and
//! who wrote it. A name counts only while the file still has that time, so
//! a post changed by hand afterwards shows no editor rather than the wrong
//! one. It is derived data: deleting the file only forgets the names.

use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};

use chrono::{DateTime, SecondsFormat, Utc};

use crate::atomic;

/// The file in `data/` that holds the record.
const FILE: &str = "post-editors.txt";

/// The first line of the file, for whoever opens it.
const HEADER: &str = "# Who last changed each post through Khalid, written by Khalid. Deleting it is safe: the admin then shows no name until the next change.";

/// Each post file, from the site folder, with the time it was written and
/// who wrote it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Editors(BTreeMap<String, (DateTime<Utc>, String)>);

impl Editors {
    /// The record kept for the `site` folder. A record that is missing or
    /// cannot be read names nobody.
    pub(crate) fn read(site: &Path) -> Self {
        let Ok(text) = std::fs::read_to_string(file(site)) else {
            return Self::default();
        };
        let lines = text
            .lines()
            .filter(|line| !line.starts_with('#'))
            .filter_map(|line| {
                let mut parts = line.splitn(3, '\t');
                let (post, at, by) = (parts.next()?, parts.next()?, parts.next()?);
                let at = DateTime::parse_from_rfc3339(at).ok()?.to_utc();
                Some((post.to_owned(), (at, by.to_owned())))
            })
            .collect();
        Self(lines)
    }

    /// Who wrote the post file at `post`, a path from the site folder, when
    /// it was last written at `changed`; `None` when the record does not
    /// know, or the file changed after.
    pub(crate) fn of(&self, post: &Path, changed: DateTime<Utc>) -> Option<&str> {
        let (at, by) = self.0.get(&key(post))?;
        (*at == changed).then_some(by.as_str())
    }

    /// Notes that `by` wrote the post file at `post` at `changed`, and
    /// forgets every file the `site` folder no longer has, such as a draft
    /// that went live. Writes the record in one step.
    ///
    /// # Errors
    ///
    /// Returns the error when `data/` cannot be made or the file written.
    pub(crate) fn note(
        mut self,
        site: &Path,
        post: &Path,
        (changed, by): (DateTime<Utc>, &str),
    ) -> io::Result<()> {
        let by: String = by
            .chars()
            .map(|c| if c.is_control() { ' ' } else { c })
            .collect();
        self.0.insert(key(post), (changed, by));
        self.0.retain(|post, _| {
            let mut path = site.to_path_buf();
            path.push(post);
            path.is_file()
        });
        let path = file(site);
        if let Some(folder) = path.parent() {
            std::fs::create_dir_all(folder)?;
        }
        let mut text = format!("{HEADER}\n");
        for (post, (at, by)) in &self.0 {
            let at = at.to_rfc3339_opts(SecondsFormat::Nanos, true);
            text.push_str(&[post.as_str(), &at, by].join("\t"));
            text.push('\n');
        }
        atomic::replace(&path, &text)
    }
}

/// The key of the post file at `post`, from the site folder, with `/`
/// between its folders on every system.
fn key(post: &Path) -> String {
    let parts: Vec<_> = post.iter().map(|part| part.to_string_lossy()).collect();
    parts.join("/")
}

/// `data/post-editors.txt` in the `site` folder.
fn file(site: &Path) -> PathBuf {
    let mut path = site.to_path_buf();
    path.push("data");
    path.push(FILE);
    path
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn a_name_is_read_back_only_while_the_file_keeps_its_time() {
        let site = tempfile::tempdir().unwrap();
        let post = Path::new("drafts/en/hello.md");
        let mut draft = site.path().to_path_buf();
        draft.push(post);
        std::fs::create_dir_all(draft.parent().unwrap()).unwrap();
        std::fs::write(&draft, "---\ntitle: Hello\n---\n").unwrap();
        let at = Utc.with_ymd_and_hms(2026, 11, 10, 9, 0, 0).unwrap();
        let gone = Path::new("drafts/en/gone.md");
        let none = Editors::read(site.path());
        assert_eq!(none.of(post, at), None, "no record names nobody");
        none.note(site.path(), gone, (at, "nobody")).unwrap();
        let at = at + chrono::Duration::nanoseconds(1);
        Editors::read(site.path())
            .note(site.path(), post, (at, "maria\tlopez\n"))
            .unwrap();

        let record = Editors::read(site.path());
        assert_eq!(record.of(post, at), Some("maria lopez "));
        let later = at + chrono::Duration::nanoseconds(1);
        assert_eq!(record.of(post, later), None, "changed since");
        assert_eq!(record.0.len(), 1, "a file the site lost is forgotten");
    }
}
