//! Puts live each page, glossary term, case study and changelog entry
//! whose `publish_at` time has come: its draft moves from `drafts/` to the
//! same place under `content/`, without the `publish_at` line, which the
//! content reader does not take. The next render sees the file and fires
//! `.published`, or `.published.edited` when it replaced a live copy.

use std::fs;
use std::io;
use std::path::Path;

use chrono::{DateTime, Utc};

use crate::atomic;
use crate::entry_events;
use crate::post_text::PostText;

/// Moves every draft in the site `folder` whose time is at or before `now`
/// live, oldest first, and gives back the file each went live as. A draft
/// that cannot be moved is logged and tried again next round. Reads and
/// writes files, so it runs on a blocking thread (RULES.md BL-RUN-7).
pub(crate) fn publish_due(folder: &Path, now: DateTime<Utc>) -> Vec<String> {
    let mut live = Vec::new();
    for (draft, file) in entry_events::due(folder, now) {
        match go_live(folder, &draft, &file) {
            Ok(()) => live.push(file),
            Err(error) => tracing::warn!(
                draft,
                %error,
                "a draft whose time came could not go live, so it is tried again next round"
            ),
        }
    }
    live
}

/// Writes the draft `draft` without its `publish_at` line as `file`, both
/// relative to `folder`, then removes the draft. The live file is written
/// whole before the draft goes, so a render in between sees both, and the
/// draft's time unchanged, and fires only `.published`.
fn go_live(folder: &Path, draft: &str, file: &str) -> io::Result<()> {
    let mut from = folder.to_path_buf();
    from.push(draft);
    let mut to = folder.to_path_buf();
    to.push(file);
    let text = fs::read_to_string(&from)?;
    let mut text = PostText::parse(&text).ok_or(io::ErrorKind::InvalidData)?;
    text.remove("publish_at");
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent)?;
    }
    atomic::replace(&to, &text.to_text())?;
    fs::remove_file(&from)
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    fn site(files: &[(&str, &str)]) -> tempfile::TempDir {
        let site = tempfile::tempdir().unwrap();
        for (name, text) in files {
            let mut file = site.path().to_path_buf();
            file.push(name);
            fs::create_dir_all(file.parent().unwrap()).unwrap();
            fs::write(&file, text).unwrap();
        }
        site
    }

    fn read(site: &tempfile::TempDir, name: &str) -> Option<String> {
        let mut file = site.path().to_path_buf();
        file.push(name);
        fs::read_to_string(file).ok()
    }

    const DUE: &str = "---\ntitle: Contact\npublish_at: 2026-11-12T09:00:00Z\nslug: contact\n---\n\nWrite to us.\n";
    const LATER: &str = "---\ntitle: Terms\npublish_at: 2026-11-12T09:00:01Z\n---\n\nRead.\n";
    const NO_TIME: &str = "---\ntitle: Notes\n---\n\nKept back.\n";

    #[test]
    fn a_draft_whose_time_came_goes_live_without_its_time() {
        let site = site(&[
            ("drafts/en/pages/contact.md", DUE),
            ("drafts/en/pages/terms.md", LATER),
            ("drafts/en/pages/notes.md", NO_TIME),
        ]);
        let nine = Utc.with_ymd_and_hms(2026, 11, 12, 9, 0, 0).unwrap();

        let live = publish_due(site.path(), nine);
        assert_eq!(live, ["content/en/pages/contact.md"]);
        let text = read(&site, "content/en/pages/contact.md").unwrap();
        assert_eq!(
            text,
            "---\ntitle: Contact\nslug: contact\n---\n\nWrite to us.\n"
        );
        assert_eq!(read(&site, "drafts/en/pages/contact.md"), None);
        assert!(read(&site, "drafts/en/pages/terms.md").is_some(), "not yet");
        assert!(read(&site, "drafts/en/pages/notes.md").is_some(), "no time");
        assert_eq!(publish_due(site.path(), nine), Vec::<String>::new());
    }

    #[test]
    fn a_scheduled_draft_of_a_live_entry_replaces_it() {
        let site = site(&[
            (
                "content/en/glossary/crawl.md",
                "---\nterm: Old\n---\n\nOld.\n",
            ),
            ("drafts/en/glossary/crawl.md", DUE),
        ]);
        let later = Utc.with_ymd_and_hms(2026, 11, 13, 0, 0, 0).unwrap();
        assert_eq!(
            publish_due(site.path(), later),
            ["content/en/glossary/crawl.md"]
        );
        let text = read(&site, "content/en/glossary/crawl.md").unwrap();
        assert!(text.contains("Write to us."), "{text}");
    }
}
