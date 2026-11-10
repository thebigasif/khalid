//! The entries waiting under `drafts/`, each at the same place it would
//! have under `content/`, and when each goes live if its front matter
//! names a `publish_at` time.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{Entry, Kind};
use crate::entry_files::front;
use crate::post_text::PostText;

/// The folder of each kind under `content/<language>/`, which a draft of
/// that kind keeps under `drafts/<language>/`.
const KIND_FOLDERS: [(&str, Kind); 4] = [
    ("pages", Kind::Page),
    ("glossary", Kind::GlossaryTerm),
    ("case-studies", Kind::CaseStudy),
    ("changelog", Kind::ChangelogEntry),
];

/// An entry waiting under `drafts/`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Waiting {
    pub(super) kind: Kind,
    /// The entry, with `file` its draft's file.
    pub(super) entry: Entry,
    /// When it goes live, or `None` when it waits for no time.
    pub(super) publish_at: Option<DateTime<Utc>>,
}

/// The fields of a draft's front matter that name it and its time. Any
/// other field is left for the content reader.
#[derive(Debug, Default, Deserialize)]
struct DraftFront {
    title: Option<String>,
    term: Option<String>,
    version: Option<String>,
    slug: Option<String>,
    publish_at: Option<String>,
}

/// Every entry waiting under `drafts/` in the site `folder`, by the file it
/// would be under `content/`: `drafts/<language>/<kind>/<name>.md` waits
/// to be `content/<language>/<kind>/<name>.md`. Hidden files and folders
/// that cannot be read are left out. A draft whose front matter does not
/// read waits for no time. Reads files, so it runs where the content is
/// read.
pub(crate) fn read(folder: &Path) -> BTreeMap<String, Waiting> {
    let mut drafts = folder.to_path_buf();
    drafts.push("drafts");
    let mut found = BTreeMap::new();
    let Ok(languages) = fs::read_dir(&drafts) else {
        return found;
    };
    for language in languages.filter_map(Result::ok) {
        let Ok(name) = language.file_name().into_string() else {
            continue;
        };
        for (kind_folder, kind) in KIND_FOLDERS {
            let mut path = language.path();
            path.push(kind_folder);
            let Ok(files) = fs::read_dir(&path) else {
                continue;
            };
            for file in files.filter_map(Result::ok) {
                let Ok(file_name) = file.file_name().into_string() else {
                    continue;
                };
                if !super::is_markdown(&file_name) || file_name.starts_with('.') {
                    continue;
                }
                let text = fs::read_to_string(file.path()).unwrap_or_default();
                let at = format!("{name}/{kind_folder}/{file_name}");
                let waiting = waiting(kind, (&name, &at, &file_name), &text);
                found.insert(format!("content/{at}"), waiting);
            }
        }
    }
    found
}

/// Each draft in the site `folder` whose publish time is at or before
/// `now`, as its file and the file it goes live as, both relative to
/// `folder`, such as `drafts/en/pages/about.md` and
/// `content/en/pages/about.md`, oldest time first.
pub(crate) fn due(folder: &Path, now: DateTime<Utc>) -> Vec<(String, String)> {
    let mut due: Vec<_> = read(folder)
        .into_iter()
        .filter_map(|(live, draft)| {
            let at = draft.publish_at.filter(|at| *at <= now)?;
            Some((at, draft.entry.file, live))
        })
        .collect();
    due.sort();
    due.into_iter()
        .map(|(_, draft, live)| (draft, live))
        .collect()
}

/// The draft of `kind` in `language`, at `drafts/<at>` and named
/// `file_name`, whose file holds `text`.
fn waiting(kind: Kind, (language, at, file_name): (&str, &str, &str), text: &str) -> Waiting {
    let front: DraftFront = PostText::parse(text)
        .and_then(|text| front(&text).ok())
        .unwrap_or_default();
    let stem = Path::new(file_name)
        .file_stem()
        .and_then(|stem| stem.to_str());
    let slug = front
        .slug
        .unwrap_or_else(|| stem.unwrap_or_default().to_owned());
    let publish_at = front.publish_at.as_deref().and_then(time);
    let entry = Entry {
        id: format!("{language}.{slug}"),
        language: language.to_owned(),
        slug,
        title: front.title.or(front.term).or(front.version),
        file: format!("drafts/{at}"),
    };
    Waiting {
        kind,
        entry,
        publish_at,
    }
}

/// The time `text` names, such as `2026-11-12T09:00:00+01:00`, or `None`
/// with a warning when it names none, so the draft keeps waiting.
fn time(text: &str) -> Option<DateTime<Utc>> {
    let text = text.trim().trim_matches(['"', '\'']);
    if let Ok(at) = DateTime::parse_from_rfc3339(text) {
        return Some(at.to_utc());
    }
    tracing::warn!(
        publish_at = text,
        "a draft's publish_at is not a time such as 2026-11-12T09:00:00Z, so it waits"
    );
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_draft_names_its_entry_and_its_time() {
        let text = "---\nterm: Crawl budget\npublish_at: 2026-11-12T09:00:00+01:00\n---\n\nRead.\n";
        let draft = waiting(
            Kind::GlossaryTerm,
            ("en", "en/glossary/crawl.md", "crawl.md"),
            text,
        );
        assert_eq!(draft.entry.id, "en.crawl");
        assert_eq!(draft.entry.title.as_deref(), Some("Crawl budget"));
        assert_eq!(draft.entry.file, "drafts/en/glossary/crawl.md");
        let at = draft.publish_at.map(|at| at.to_rfc3339());
        assert_eq!(at.as_deref(), Some("2026-11-12T08:00:00+00:00"));
    }

    #[test]
    fn a_draft_with_no_time_or_a_bad_one_waits_for_none() {
        for text in [
            "---\ntitle: Kept back\n---\n",
            "---\ntitle: Kept back\npublish_at: next tuesday\n---\n",
            "no front matter at all",
        ] {
            let draft = waiting(Kind::Page, ("en", "en/pages/a.md", "a.md"), text);
            assert_eq!(draft.publish_at, None, "{text}");
            assert_eq!(draft.entry.id, "en.a", "{text}");
        }
    }
}
