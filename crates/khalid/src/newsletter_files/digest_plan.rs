//! The digest a newsletter asks `khalid serve` to send on its own (backlog
//! row 16): `content/newsletters/<name>/digest.toml`, such as
//!
//! ```toml
//! every = "week"       # or "day"
//! labels = ["paid"]    # only readers with every label; leave out for all
//! language = "en"      # the posts' language; leave out for the default
//! ```
//!
//! A newsletter without the file gets no digest from the server, and
//! `khalid newsletter digest` still sends one by hand or from a timer.

use std::fs;
use std::path::Path;

use chrono::{DateTime, TimeDelta, Utc};
use khalid_core::{Locale, SitePath, Slug};
use khalid_newsletter::SendNumbers;
use serde::Deserialize;

use super::{FOLDER, lists};

/// The file a newsletter's folder holds to get digests.
const FILE: &str = "digest.toml";

/// How often a newsletter's digest goes out, at most.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Every {
    /// At most once a day.
    Day,
    /// At most once a week.
    Week,
}

impl Every {
    /// The time between two digests.
    fn span(self) -> TimeDelta {
        match self {
            Self::Day => TimeDelta::days(1),
            Self::Week => TimeDelta::days(7),
        }
    }
}

/// One newsletter's `digest.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DigestPlan {
    /// How often it goes out, at most.
    pub(crate) every: Every,
    /// The labels every reader who gets it carries.
    #[serde(default)]
    pub(crate) labels: Vec<Slug>,
    /// The language of the posts it lists, or the site's default.
    pub(crate) language: Option<Locale>,
}

impl DigestPlan {
    /// Whether the digest of `list` is due at `now`: no digest of it in
    /// `sends` started within the last day or week.
    pub(crate) fn due(&self, list: &Slug, sends: &[SendNumbers], now: DateTime<Utc>) -> bool {
        let prefix = format!("digest:{list}:");
        let since = now - self.every.span();
        !sends
            .iter()
            .any(|send| send.key.starts_with(&prefix) && send.started_at > since)
    }
}

/// Each newsletter of the site in `folder` with a `digest.toml`, and what
/// it asks. A file that does not read is logged and skipped, so one typo
/// never stops the other digests.
pub(crate) fn plans(folder: &Path) -> Vec<(Slug, DigestPlan)> {
    let mut plans = Vec::new();
    for list in lists(folder).iter() {
        let Ok(file) = SitePath::within(folder, &format!("{FOLDER}/{list}/{FILE}")) else {
            continue;
        };
        let Ok(text) = fs::read_to_string(file.as_path()) else {
            continue;
        };
        match toml::from_str::<DigestPlan>(&text) {
            Ok(plan) => plans.push((list.clone(), plan)),
            Err(error) => tracing::warn!(
                %list,
                error = error.message(),
                "a newsletter's digest.toml does not read, so its digest is not sent; fix the file as docs/newsletters.md shows"
            ),
        }
    }
    plans
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use khalid_newsletter::SendTotals;

    use super::*;

    #[cfg(test)]
    fn site(files: &[(&str, &str)]) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        for (path, text) in files {
            let file = SitePath::within(dir.path(), path).unwrap();
            fs::create_dir_all(file.as_path().parent().unwrap()).unwrap();
            fs::write(file.as_path(), text).unwrap();
        }
        dir
    }

    #[test]
    fn each_newsletter_with_a_digest_file_is_planned_and_a_broken_one_skipped() {
        let dir = site(&[
            ("content/newsletters/main/digest.toml", "every = \"week\"\n"),
            (
                "content/newsletters/paid/digest.toml",
                "every = \"day\"\nlabels = [\"paid\"]\nlanguage = \"de\"\n",
            ),
            (
                "content/newsletters/broken/digest.toml",
                "every = \"hour\"\n",
            ),
            ("content/newsletters/quiet/welcome.md", "# Hello\n"),
        ]);
        let plans = plans(dir.path());
        let names: Vec<&str> = plans.iter().map(|(list, _)| list.as_str()).collect();
        assert_eq!(names, ["main", "paid"]);
        let paid = &plans[1].1;
        assert_eq!(paid.every, Every::Day);
        assert_eq!(paid.labels[0].as_str(), "paid");
        assert_eq!(paid.language.as_ref().map(Locale::as_str), Some("de"));
    }

    #[test]
    fn a_digest_is_due_once_its_span_passed_since_the_last_one() {
        let list: Slug = "main".parse().unwrap();
        let plan = DigestPlan {
            every: Every::Week,
            labels: Vec::new(),
            language: None,
        };
        let at = Utc.with_ymd_and_hms(2026, 11, 20, 9, 0, 0).unwrap();
        assert!(plan.due(&list, &[], at), "never sent");
        let sent = |key: &str, days| {
            let started = at - TimeDelta::days(days);
            SendNumbers::new(key.to_owned(), list.clone(), started, SendTotals::default())
        };
        assert!(!plan.due(&list, &[sent("digest:main:2026-11-15", 3)], at));
        assert!(plan.due(&list, &[sent("digest:main:2026-11-10", 8)], at));
        assert!(
            plan.due(&list, &[sent("post:en/hello", 1)], at),
            "a post sent by hand is not a digest"
        );
        assert!(
            plan.due(&list, &[sent("digest:weekly:2026-11-19", 1)], at),
            "another newsletter's digest"
        );
    }
}
