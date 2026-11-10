//! What `IndexNow` last heard about a site: the address of each live post
//! and its `updated:` date, kept in `data/indexnow-sent.txt`. `khalid
//! serve` compares its first read of the site with it, and `khalid build`
//! each build, so a change made while nothing ran is still sent once (task
//! C8, ADR "`IndexNow` without sitemap pings"). It is derived data: deleting
//! it costs one run that sends nothing.

use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use khalid_seo::UrlPath;

use crate::atomic;

/// The file in `data/` that holds the record.
const FILE: &str = "indexnow-sent.txt";

/// The first line of the file, for whoever opens it.
const HEADER: &str = "# What IndexNow last heard about, written by Khalid. Deleting it is safe: the next run sends nothing and starts again.";

/// Each live post's address from the site root, with its `updated:` date
/// when it has one.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Record(BTreeMap<String, Option<NaiveDate>>);

impl Record {
    /// The record of the posts at `addresses`, by file, whose `updated:`
    /// dates `updated` holds by file.
    pub(crate) fn of(
        addresses: &BTreeMap<String, UrlPath>,
        updated: &BTreeMap<String, NaiveDate>,
    ) -> Self {
        let pages = addresses
            .iter()
            .map(|(file, path)| (path.as_str().to_owned(), updated.get(file).copied()))
            .collect();
        Self(pages)
    }

    /// The addresses in this record that are not in `before`, or whose
    /// `updated:` date is new or later. The date changes only on a real
    /// update (decision D4), so any other edit sends nothing.
    pub(crate) fn news_since(&self, before: &Self) -> Vec<&str> {
        self.0
            .iter()
            .filter(|(path, day)| match before.0.get(*path) {
                None => true,
                Some(was) => *day > was,
            })
            .map(|(path, _)| path.as_str())
            .collect()
    }

    /// The record kept for the `site` folder, or `None` when there is none
    /// yet or it cannot be read, which a run treats as a first run.
    pub(crate) fn read(site: &Path) -> Option<Self> {
        let text = std::fs::read_to_string(file(site)).ok()?;
        let pages = text
            .lines()
            .filter(|line| !line.starts_with('#'))
            .filter_map(|line| {
                let (path, day) = line.split_once('\t')?;
                let day = NaiveDate::parse_from_str(day, "%Y-%m-%d").ok();
                Some((path.to_owned(), day))
            })
            .collect();
        Some(Self(pages))
    }

    /// Keeps this record for the `site` folder, replacing the last in one
    /// step, so a reader sees the old record or the new.
    ///
    /// # Errors
    ///
    /// Returns the error when `data/` cannot be made or the file written.
    pub(crate) fn write(&self, site: &Path) -> io::Result<()> {
        let path = file(site);
        if let Some(folder) = path.parent() {
            std::fs::create_dir_all(folder)?;
        }
        let lines: String = self
            .0
            .iter()
            .map(|(address, day)| match day {
                Some(day) => format!("{address}\t{day}\n"),
                None => format!("{address}\t-\n"),
            })
            .collect();
        atomic::replace(&path, &format!("{HEADER}\n{lines}"))
    }
}

/// `data/indexnow-sent.txt` in the `site` folder.
fn file(site: &Path) -> PathBuf {
    let mut path = site.to_path_buf();
    path.push("data");
    path.push(FILE);
    path
}

/// What a first run with no record says, once: nothing is sent this time,
/// so a new site does not send every address it has.
pub(crate) const FIRST_RUN: &str = "IndexNow: this is the first run with IndexNow on, so no addresses were sent. Khalid now keeps a list of what search engines heard, in data/indexnow-sent.txt, and from the next run sends each post that is new or has a new updated: date.";

#[cfg(test)]
mod tests {
    use super::*;

    fn record(pages: &[(&str, Option<u32>)]) -> Record {
        let mut addresses = BTreeMap::new();
        let mut updated = BTreeMap::new();
        for &(slug, day) in pages {
            let file = format!("{slug}.md");
            addresses.insert(
                file.clone(),
                UrlPath::try_new(&format!("/blog/{slug}/")).unwrap(),
            );
            if let Some(day) = day.and_then(|day| NaiveDate::from_ymd_opt(2026, 10, day)) {
                updated.insert(file, day);
            }
        }
        Record::of(&addresses, &updated)
    }

    #[test]
    fn a_new_updated_date_or_a_new_live_post_is_news_and_nothing_else() {
        let before = record(&[
            ("dated", None),
            ("same", Some(1)),
            ("later", Some(5)),
            ("dropped", Some(5)),
        ]);
        let now = record(&[
            ("dated", Some(9)),
            ("same", Some(1)),
            ("later", Some(7)),
            ("dropped", None),
            ("new", None),
        ]);
        let news = now.news_since(&before);
        assert_eq!(news, ["/blog/dated/", "/blog/later/", "/blog/new/"]);
        assert!(now.news_since(&now).is_empty(), "no new date sends nothing");
    }

    #[test]
    fn the_record_kept_is_the_one_read_back() {
        let site = tempfile::tempdir().unwrap();
        assert_eq!(Record::read(site.path()), None, "a first run");
        let kept = record(&[("a", Some(3)), ("b", None)]);
        kept.write(site.path()).unwrap();
        assert_eq!(Record::read(site.path()), Some(kept));
        let again = record(&[("a", Some(4))]);
        again.write(site.path()).unwrap();
        assert_eq!(Record::read(site.path()), Some(again), "replaced whole");
    }
}
