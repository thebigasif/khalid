//! The webhooks `khalid serve` fires as the files of pages, glossary terms,
//! case studies and changelog entries are saved (audit brief G25). An
//! entry whose file is under `content/` is live: a
//! new file fires `.published`, a changed one `.published.edited` and a
//! removed one `.deleted`. A removed file that left a draft of the same
//! name under `drafts/` fires `.unpublished` instead. A draft whose front
//! matter names a `publish_at` time is scheduled: a new or moved time
//! fires `.scheduled` with it, and a time taken away, or a draft removed
//! without going live, fires `.scheduled` with `publish_at` null.

mod waiting;

use std::collections::BTreeMap;
use std::path::Path;

use chrono::{DateTime, SecondsFormat, Utc};
use khalid_content::{ContentHash, Document, SiteContent};
use khalid_core::Slug;
use khalid_webhooks::Event;
use serde::Serialize;
use waiting::Waiting;
pub(crate) use waiting::due;

use crate::announce::SiteAnnounce;
use crate::index;

/// Which kind of entry a file holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Page,
    GlossaryTerm,
    CaseStudy,
    ChangelogEntry,
}

/// What happened to an entry between two renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Change {
    Published,
    Edited,
    Unpublished,
    Deleted,
}

impl Kind {
    /// The name the entry goes under in a delivery's body.
    fn key(self) -> &'static str {
        match self {
            Self::Page => "page",
            Self::GlossaryTerm => "glossary_term",
            Self::CaseStudy => "case_study",
            Self::ChangelogEntry => "changelog_entry",
        }
    }

    /// The `.scheduled` event of this kind.
    fn scheduled(self) -> Event {
        match self {
            Self::Page => Event::PageScheduled,
            Self::GlossaryTerm => Event::GlossaryTermScheduled,
            Self::CaseStudy => Event::CaseStudyScheduled,
            Self::ChangelogEntry => Event::ChangelogEntryScheduled,
        }
    }

    /// The event for `change` to an entry of this kind.
    fn event(self, change: Change) -> Event {
        match (self, change) {
            (Self::Page, Change::Published) => Event::PagePublished,
            (Self::Page, Change::Edited) => Event::PagePublishedEdited,
            (Self::Page, Change::Unpublished) => Event::PageUnpublished,
            (Self::Page, Change::Deleted) => Event::PageDeleted,
            (Self::GlossaryTerm, Change::Published) => Event::GlossaryTermPublished,
            (Self::GlossaryTerm, Change::Edited) => Event::GlossaryTermPublishedEdited,
            (Self::GlossaryTerm, Change::Unpublished) => Event::GlossaryTermUnpublished,
            (Self::GlossaryTerm, Change::Deleted) => Event::GlossaryTermDeleted,
            (Self::CaseStudy, Change::Published) => Event::CaseStudyPublished,
            (Self::CaseStudy, Change::Edited) => Event::CaseStudyPublishedEdited,
            (Self::CaseStudy, Change::Unpublished) => Event::CaseStudyUnpublished,
            (Self::CaseStudy, Change::Deleted) => Event::CaseStudyDeleted,
            (Self::ChangelogEntry, Change::Published) => Event::ChangelogEntryPublished,
            (Self::ChangelogEntry, Change::Edited) => Event::ChangelogEntryPublishedEdited,
            (Self::ChangelogEntry, Change::Unpublished) => Event::ChangelogEntryUnpublished,
            (Self::ChangelogEntry, Change::Deleted) => Event::ChangelogEntryDeleted,
        }
    }
}

/// One entry as a delivery carries it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct Entry {
    /// `<language>.<slug>`, as posts are named.
    id: String,
    language: String,
    slug: String,
    /// The title, the glossary term, or the changelog entry's version when
    /// it has no title.
    title: Option<String>,
    /// The file, relative to the site folder, with `/` between folders.
    file: String,
}

/// An entry as a `.scheduled` delivery carries it: with the time it goes
/// live, or `null` when it no longer has one.
#[derive(Debug, Clone, Serialize)]
struct Scheduled {
    #[serde(flatten)]
    entry: Entry,
    publish_at: Option<String>,
}

/// An entry and the hash of its file, as one render read it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Seen {
    kind: Kind,
    entry: Entry,
    hash: ContentHash,
}

/// Every entry of one render, by file, and the entries waiting under
/// `drafts/`, by the file each would be under `content/`.
#[derive(Debug, Clone, Default)]
pub(crate) struct Entries(BTreeMap<String, Seen>, BTreeMap<String, Waiting>);

impl Entries {
    /// The entries of `content`. A file whose name is not valid UTF-8 is
    /// left out.
    pub(crate) fn of(content: &SiteContent) -> Self {
        let pages = content.pages.iter().map(|doc| {
            let front = &doc.front;
            seen(Kind::Page, doc, front.slug.as_ref(), front.title.clone())
        });
        let terms = content.glossary.iter().map(|doc| {
            let front = &doc.front;
            seen(
                Kind::GlossaryTerm,
                doc,
                front.slug.as_ref(),
                front.term.clone(),
            )
        });
        let studies = content.case_studies.iter().map(|doc| {
            let front = &doc.front;
            seen(
                Kind::CaseStudy,
                doc,
                front.slug.as_ref(),
                front.title.clone(),
            )
        });
        let changelog = content.changelog.iter().map(|doc| {
            let title = doc
                .front
                .title
                .clone()
                .or_else(|| doc.front.version.clone());
            seen(Kind::ChangelogEntry, doc, None, title)
        });
        let all = pages.chain(terms).chain(studies).chain(changelog);
        Self(all.flatten().collect(), BTreeMap::new())
    }

    /// These entries, with the drafts waiting in the site `folder`: the
    /// entry at `content/<language>/<kind>/<name>.md` waits at
    /// `drafts/<language>/<kind>/<name>.md` (ADR 0030). Reads files, so it
    /// runs where the content is read.
    pub(crate) fn with_drafts(mut self, folder: &Path) -> Self {
        self.1 = waiting::read(folder);
        self
    }

    /// Fires on `announce` the event of each entry that is new, changed or
    /// gone in `self` since `before`, and `.scheduled` for each draft whose
    /// publish time is new, moved or gone.
    pub(crate) fn fire_since(&self, before: &Self, announce: &SiteAnnounce) {
        for (event, kind, entry) in self.changes_since(before) {
            announce.fire(event, BTreeMap::from([(kind.key(), entry.clone())]));
        }
        for (kind, entry, at) in self.schedules_since(before) {
            let publish_at = at.map(|at| at.to_rfc3339_opts(SecondsFormat::Secs, true));
            let entry = entry.clone();
            let scheduled = Scheduled { entry, publish_at };
            announce.fire(kind.scheduled(), BTreeMap::from([(kind.key(), scheduled)]));
        }
    }

    /// Each draft whose publish time is new or moved in `self` since
    /// `before`, with the time, and each whose time went without it going
    /// live, with none: taken out of its front matter, or its draft
    /// removed. A draft whose file is under `content/` now went live, which
    /// fires `.published` alone.
    fn schedules_since<'a>(
        &'a self,
        before: &'a Self,
    ) -> Vec<(Kind, &'a Entry, Option<DateTime<Utc>>)> {
        let set = self.1.iter().filter_map(|(file, now)| {
            let at = now.publish_at?;
            let was = before.1.get(file).and_then(|was| was.publish_at);
            (was != Some(at)).then_some((now.kind, &now.entry, Some(at)))
        });
        let dropped = before.1.iter().filter_map(|(file, was)| {
            was.publish_at?;
            match self.1.get(file) {
                Some(now) if now.publish_at.is_none() => Some((now.kind, &now.entry, None)),
                Some(_) => None,
                None if self.0.contains_key(file) => None,
                None => Some((was.kind, &was.entry, None)),
            }
        });
        set.chain(dropped).collect()
    }

    /// Each entry that is new, changed or gone in `self` since `before`,
    /// with its event: a gone entry as it was last, unpublished when
    /// `self` saw a draft of it and deleted when not.
    fn changes_since<'a>(&'a self, before: &'a Self) -> Vec<(Event, Kind, &'a Entry)> {
        let now = self.0.iter().filter_map(|(file, seen)| {
            let change = match before.0.get(file) {
                None => Change::Published,
                Some(was) if was.hash != seen.hash => Change::Edited,
                Some(_) => return None,
            };
            Some((seen.kind.event(change), seen.kind, &seen.entry))
        });
        let gone = before
            .0
            .iter()
            .filter(|(file, _)| !self.0.contains_key(*file))
            .map(|(file, was)| {
                let change = if self.1.contains_key(file) {
                    Change::Unpublished
                } else {
                    Change::Deleted
                };
                (was.kind.event(change), was.kind, &was.entry)
            });
        now.chain(gone).collect()
    }
}

/// Whether the file `name` ends in `.md`, in any case, as the content tree
/// reads them.
fn is_markdown(name: &str) -> bool {
    Path::new(name)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
}

/// `doc` as an entry of `kind`, by its file. The slug is `slug` when the
/// front matter names one, else the file's name.
fn seen<F>(
    kind: Kind,
    doc: &Document<F>,
    slug: Option<&Slug>,
    title: Option<String>,
) -> Option<(String, Seen)> {
    let file = index::slashed(&doc.path)?;
    let slug = match slug {
        Some(slug) => slug.to_string(),
        None => doc.path.file_stem()?.to_str()?.to_owned(),
    };
    let language = doc.locale.as_str().to_owned();
    let entry = Entry {
        id: format!("{language}.{slug}"),
        language,
        slug,
        title,
        file: file.clone(),
    };
    let hash = doc.hash;
    Some((file, Seen { kind, entry, hash }))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use khalid_content::{ContentTree, read_content};

    use super::*;

    fn demo() -> Entries {
        let mut folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        folder.push("../../fixtures/demo-site");
        let content = read_content(&ContentTree::walk(&folder).unwrap()).unwrap();
        Entries::of(&content)
    }

    fn names(changes: &[(Event, Kind, &Entry)]) -> Vec<String> {
        let named = changes
            .iter()
            .map(|(event, _, entry)| format!("{event} {}", entry.id));
        named.collect()
    }

    #[test]
    fn the_demo_entries_are_read_with_their_names() {
        let entries = demo();
        let about = &entries.0["content/en/pages/about.md"];
        assert_eq!(about.kind, Kind::Page);
        assert_eq!(about.entry.id, "en.about");
        let changelog = &entries.0["content/en/changelog/2026-10.md"];
        assert_eq!(changelog.entry.slug, "2026-10");
        assert!(changelog.entry.title.is_some(), "{changelog:?}");
        assert!(
            !entries.0.keys().any(|file| file.contains("/posts/")),
            "posts fire through the publish door"
        );
    }

    #[test]
    fn a_render_with_nothing_changed_fires_nothing() {
        assert_eq!(names(&demo().changes_since(&demo())), Vec::<String>::new());
    }

    #[test]
    fn new_changed_and_gone_entries_fire_their_events() {
        let before = demo();
        let mut now = demo();
        let about = now.0.remove("content/en/pages/about.md").unwrap();
        let mut term = now
            .0
            .remove("content/en/glossary/topical-authority.md")
            .unwrap();
        term.hash = about.hash;
        now.0
            .insert("content/en/glossary/topical-authority.md".to_owned(), term);
        let mut study = before.0["content/en/case-studies/northfield-bakery.md"].clone();
        study.entry.id = "en.harbour-cafe".to_owned();
        now.0
            .insert("content/en/case-studies/harbour-cafe.md".to_owned(), study);

        assert_eq!(
            names(&now.changes_since(&before)),
            [
                "case-study.published en.harbour-cafe",
                "glossary-term.published.edited en.topical-authority",
                "page.deleted en.about",
            ]
        );
    }

    #[test]
    fn an_entry_moved_to_drafts_is_unpublished_and_one_removed_is_deleted() {
        let site = tempfile::tempdir().unwrap();
        for draft in [
            "drafts/en/pages/about.md",
            "drafts/en/glossary/.topical-authority.md",
            "drafts/en/case-studies/notes.txt",
            "drafts/en/a-post-draft.md",
        ] {
            let mut file = site.path().to_path_buf();
            file.push(draft);
            fs::create_dir_all(file.parent().unwrap()).unwrap();
            fs::write(&file, "---\ntitle: Kept back\n---\n").unwrap();
        }
        let before = demo();
        let mut now = demo().with_drafts(site.path());
        now.0.remove("content/en/pages/about.md").unwrap();
        now.0
            .remove("content/en/glossary/topical-authority.md")
            .unwrap();

        assert_eq!(
            names(&now.changes_since(&before)),
            [
                "glossary-term.deleted en.topical-authority",
                "page.unpublished en.about",
            ]
        );
    }

    /// The demo entries with each of `drafts`, a file under `drafts/` and
    /// its text, waiting.
    fn waiting_with(drafts: &[(&str, &str)]) -> Entries {
        let site = tempfile::tempdir().unwrap();
        for (draft, text) in drafts {
            let mut file = site.path().to_path_buf();
            file.push(draft);
            fs::create_dir_all(file.parent().unwrap()).unwrap();
            fs::write(&file, text).unwrap();
        }
        demo().with_drafts(site.path())
    }

    fn schedules(now: &Entries, before: &Entries) -> Vec<String> {
        let told = now.schedules_since(before).into_iter();
        let named = told.map(|(kind, entry, at)| {
            let at = at.map_or_else(|| "none".to_owned(), |at| at.to_rfc3339());
            format!("{} {} {at}", kind.scheduled(), entry.id)
        });
        named.collect()
    }

    const AT_NINE: &str = "---\ntitle: Contact\npublish_at: 2026-11-12T09:00:00Z\n---\n";
    const AT_TEN: &str = "---\ntitle: Contact\npublish_at: 2026-11-12T10:00:00Z\n---\n";
    const NO_TIME: &str = "---\ntitle: Contact\n---\n";

    #[test]
    fn a_draft_given_a_time_moved_or_kept_fires_scheduled_once_per_change() {
        let page = "drafts/en/pages/contact.md";
        let untimed = waiting_with(&[(page, NO_TIME)]);
        let nine = waiting_with(&[(page, AT_NINE)]);
        let ten = waiting_with(&[(page, AT_TEN)]);
        let term = "drafts/en/glossary/crawl.md";
        let new_term = waiting_with(&[(page, AT_TEN), (term, AT_NINE)]);

        assert_eq!(
            schedules(&nine, &untimed),
            ["page.scheduled en.contact 2026-11-12T09:00:00+00:00"]
        );
        assert_eq!(
            schedules(&ten, &nine),
            ["page.scheduled en.contact 2026-11-12T10:00:00+00:00"]
        );
        assert_eq!(schedules(&ten, &ten), Vec::<String>::new());
        assert_eq!(
            schedules(&new_term, &ten),
            ["glossary-term.scheduled en.crawl 2026-11-12T09:00:00+00:00"]
        );
        assert_eq!(schedules(&untimed, &untimed), Vec::<String>::new());
    }

    #[test]
    fn a_time_taken_away_or_a_draft_removed_fires_scheduled_with_none() {
        let page = "drafts/en/pages/contact.md";
        let nine = waiting_with(&[(page, AT_NINE)]);
        let untimed = waiting_with(&[(page, NO_TIME)]);
        let gone = demo();
        assert_eq!(
            schedules(&untimed, &nine),
            ["page.scheduled en.contact none"]
        );
        assert_eq!(schedules(&gone, &nine), ["page.scheduled en.contact none"]);
    }

    #[test]
    fn a_scheduled_draft_that_went_live_fires_no_scheduled() {
        let nine = waiting_with(&[("drafts/en/pages/about.md", AT_NINE)]);
        let live = demo();
        assert!(live.0.contains_key("content/en/pages/about.md"));
        assert_eq!(schedules(&live, &nine), Vec::<String>::new());
    }
}
