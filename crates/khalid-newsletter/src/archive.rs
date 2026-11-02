//! The newsletter's public page and its archive of past issues, read from
//! `content/newsletters/main/`: `page.md` says what readers get and how
//! often, and each file in `issues/` is one issue that went out, which
//! gets a web page of its own.

use chrono::NaiveDate;
use khalid_core::Slug;

/// The newsletter's page: its title and what it tells readers, from a
/// file whose first line is a `# ` heading and the rest markdown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewsletterPage {
    /// The heading, such as "The Larkspur Letter".
    pub title: String,
    /// What readers get and how often, in markdown.
    pub body: String,
}

impl NewsletterPage {
    /// The page `markdown` holds, or `None` when its first line is not a
    /// `# ` heading or nothing follows it.
    pub fn read(markdown: &str) -> Option<Self> {
        let (title, body) = titled(markdown)?;
        Some(Self { title, body })
    }
}

/// One past issue: its day and slug from its file's name, such as
/// `2026-10-14-five-links-worth-your-time.md`, and its subject line and
/// text from the file, as [`NewsletterPage`] reads them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedIssue {
    /// The day it went out.
    pub date: NaiveDate,
    /// The last part of its page's address.
    pub slug: Slug,
    /// Its subject line.
    pub title: String,
    /// Its text, in markdown.
    pub body: String,
}

impl ArchivedIssue {
    /// The issue in the file named `file_name` holding `markdown`, or
    /// `None` when the name is not a day, a hyphen, a slug and `.md`, or
    /// the text has no `# ` subject line and body.
    pub fn read(file_name: &str, markdown: &str) -> Option<Self> {
        let stem = file_name.strip_suffix(".md")?;
        let (day, slug) = (stem.get(..10)?, stem.get(11..)?);
        let date = NaiveDate::parse_from_str(day, "%Y-%m-%d").ok()?;
        let slug = Slug::try_new(slug).ok().filter(|s| s.as_str() == slug)?;
        let (title, body) = titled(markdown)?;
        (stem.get(10..11) == Some("-")).then_some(Self {
            date,
            slug,
            title,
            body,
        })
    }
}

/// The issues that went out by `today`, newest first, from `issues`. Two
/// issues with the same slug keep the newer one, so no address is used
/// twice.
pub fn archive(mut issues: Vec<ArchivedIssue>, today: NaiveDate) -> Vec<ArchivedIssue> {
    issues.retain(|issue| issue.date <= today);
    issues.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.slug.cmp(&b.slug)));
    let mut seen = Vec::new();
    issues.retain(|issue| {
        let fresh = !seen.contains(&issue.slug);
        seen.push(issue.slug.clone());
        fresh
    });
    issues
}

/// The `# ` heading on the first line of `markdown` and the text after
/// it, both trimmed, when both are there.
fn titled(markdown: &str) -> Option<(String, String)> {
    let markdown = markdown.trim_start_matches('\u{feff}').trim_start();
    let (first, rest) = markdown.split_once('\n')?;
    let title = first.strip_prefix("# ")?.trim();
    let body = rest.trim();
    (!title.is_empty() && !body.is_empty()).then(|| (title.to_owned(), format!("{body}\n")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(text: &str) -> NaiveDate {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap()
    }

    fn issue(name: &str) -> ArchivedIssue {
        ArchivedIssue::read(name, "# Subject\n\nText.\n").unwrap()
    }

    #[test]
    fn an_issue_takes_its_day_and_slug_from_its_file_name() {
        let issue = ArchivedIssue::read(
            "2026-10-14-five-links.md",
            "\u{feff}# Five links worth your time\n\nThe first link.\n",
        )
        .unwrap();
        assert_eq!(issue.date, day("2026-10-14"));
        assert_eq!(issue.slug.as_str(), "five-links");
        assert_eq!(issue.title, "Five links worth your time");
        assert_eq!(issue.body, "The first link.\n");
    }

    #[test]
    fn a_file_without_a_day_a_slug_or_a_subject_is_no_issue() {
        let text = "# Subject\n\nText.\n";
        for name in [
            "five-links.md",
            "2026-13-01-five-links.md",
            "2026-10-14_five-links.md",
            "2026-10-14-Five Links.md",
            "2026-10-14-five-links.txt",
            "2026-10-14-.md",
        ] {
            assert_eq!(ArchivedIssue::read(name, text), None, "{name}");
        }
        assert_eq!(
            ArchivedIssue::read("2026-10-14-a.md", "Subject\n\nText.\n"),
            None
        );
        assert_eq!(ArchivedIssue::read("2026-10-14-a.md", "# Subject\n"), None);
    }

    #[test]
    fn the_archive_is_newest_first_without_future_issues_or_a_slug_twice() {
        let issues = vec![
            issue("2026-09-01-first.md"),
            issue("2026-11-30-planned.md"),
            issue("2026-10-01-second.md"),
            issue("2026-08-01-second.md"),
        ];
        let slugs: Vec<String> = archive(issues, day("2026-10-31"))
            .iter()
            .map(|issue| format!("{} {}", issue.date, issue.slug))
            .collect();
        assert_eq!(slugs, ["2026-10-01 second", "2026-09-01 first"]);
    }

    #[test]
    fn the_page_needs_a_heading_and_text() {
        let page = NewsletterPage::read("# The Letter\n\nEvery Tuesday.\n").unwrap();
        assert_eq!(page.title, "The Letter");
        assert_eq!(page.body, "Every Tuesday.\n");
        assert_eq!(NewsletterPage::read("The Letter\n\nEvery Tuesday.\n"), None);
    }
}
