//! The Dispatch template end to end (docs/templates/dispatch.md): `khalid
//! build` of the demo site with `[template] name = "dispatch"`, a call to
//! action and two posts given an issue number, then the header and main
//! part of its front page, an issue, a list and the writers page, the
//! sign-up before the issues, the stamps and the reply slip after an
//! issue, with and without a site that takes sign-ups.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

/// Copies the folder `from` into `into`, file by file.
#[cfg(test)]
fn copy_dir(from: &Path, into: &Path) {
    fs::create_dir_all(into).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = at(into, &entry.file_name().to_string_lossy());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

const SETTINGS: &str = "\n[template]\nname = \"dispatch\"\n[template.options]\ncadence = [\"Every Tuesday\"]\npromises = [\"One tip a week\", \"Short enough to read with coffee\"]\nhero_stats = [\"3,200 | readers\"]\n[template.cta]\nlabel = \"Subscribe free\"\nhref = \"https://www.example.com/newsletter/\"\n";

/// A site that takes sign-ups: served, with mail settings.
const MAIL: &str = "\n[mail]\nfrom = \"news@example.com\"\n\n[mail.smtp]\nhost = \"127.0.0.1\"\nport = 2525\nsecurity = \"none\"\nprivate_host = true\n";

/// Two posts that become numbered issues.
const ISSUES: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Issue\n    value: \"42\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Issue\n    value: \"40\"\n",
    ),
];

/// The demo site built with Dispatch and `extra` settings, with two posts
/// made numbered issues.
#[cfg(test)]
fn built(extra: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str(extra);
    toml.push_str(SETTINGS);
    fs::write(&settings, toml).unwrap();
    for (rel, facts) in ISSUES {
        let path = at(temp.path(), rel);
        let text = fs::read_to_string(&path).unwrap();
        fs::write(path, text.replacen('\n', &format!("\n{facts}"), 1)).unwrap();
    }
    let out = at(temp.path(), "out");
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", temp.path().to_str().unwrap(), "build", "--out"])
        .arg(&out)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    (temp, out)
}

/// The page at `rel`, checked for one `h1` and unique ids, cut to its
/// header and main part: what a template changes.
#[cfg(test)]
fn shown(out: &Path, rel: &str) -> String {
    let page = fs::read_to_string(at(out, rel)).unwrap();
    let html = scraper::Html::parse_document(&page);
    assert!(html.errors.is_empty(), "{rel}: {:?}", html.errors);
    let select = |css: &str| {
        let selector = scraper::Selector::parse(css).unwrap();
        html.select(&selector)
            .filter_map(|element| element.value().attr("id").or(Some("")))
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    assert_eq!(select("h1").len(), 1, "{rel}: one h1");
    let mut ids = select("[id]");
    let count = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), count, "{rel}: unique ids");
    let start = page.find("<header class=\"site-header\">").unwrap();
    let end = page.find("</main>").unwrap();
    page.get(start..end).unwrap().to_owned()
}

/// The texts of every element `css` selects in `html`, with runs of
/// white space made one space.
#[cfg(test)]
fn texts(html: &str, css: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .map(|element| {
            let text = element.text().collect::<String>();
            text.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .collect()
}

/// Where `needle` first stands in `page`.
#[cfg(test)]
fn place(page: &str, needle: &str) -> usize {
    page.find(needle)
        .unwrap_or_else(|| panic!("no {needle} in {page}"))
}

#[test]
fn dispatch_front_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The front page opens on the sign-up, with the cadence, the promises
/// and the call to action as the sign-up button on a site that takes no
/// sign-ups, then the latest issue, then the recent issues, which leave
/// the latest out.
#[test]
fn the_front_page_opens_on_the_sign_up_before_the_issues() {
    let (_temp, out) = built("");
    let page = shown(&out, "index.html");
    let slip = place(&page, "<div class=\"dp-slip\">");
    let latest = place(&page, "<article class=\"dp-latest\"");
    let recent = place(&page, "<section class=\"dp-recent\"");
    assert!(slip < latest && latest < recent, "{page}");
    assert_eq!(texts(&page, ".hero-kicker"), ["Every Tuesday"]);
    assert_eq!(
        texts(&page, ".dp-promises li"),
        ["One tip a week", "Short enough to read with coffee"]
    );
    assert_eq!(
        texts(&page, ".dp-top .dp-slip a.button"),
        ["Subscribe free"]
    );
    assert!(!page.contains("<form class=\"subscribe\""), "{page}");
    assert_eq!(texts(&page, ".figure dd"), ["3,200"]);
    let newest = texts(&page, ".dp-latest h2");
    assert_eq!(newest.len(), 1, "{page}");
    let rows = texts(&page, ".dp-recent .dp-issue h3");
    assert!(!rows.is_empty(), "{page}");
    assert!(!rows.contains(&newest[0]), "{rows:?}");
}

/// An issue with facts shows its first fact in its stamp, and one without
/// shows the month and day it was sent.
#[test]
fn every_issue_carries_a_stamp() {
    let (_temp, out) = built("");
    let page = shown(&out, "index.html");
    let stamps = texts(&page, ".dp-issue .dp-stamp");
    assert!(stamps.contains(&"Issue 42".to_owned()), "{stamps:?}");
    assert!(stamps.contains(&"Issue 40".to_owned()), "{stamps:?}");
    assert!(stamps.contains(&"Sep 28".to_owned()), "{stamps:?}");
}

#[test]
fn dispatch_post() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// An issue opens on its stamp and the day it was sent, and the reply
/// slip follows its text straight away.
#[test]
fn an_issue_opens_on_its_stamp_and_closes_on_the_reply_slip() {
    let (_temp, out) = built("");
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(texts(&page, ".dp-issue-head .dp-stamp"), ["Issue 42"]);
    assert!(
        texts(&page, ".dp-sent")
            .iter()
            .any(|line| line.starts_with("Sent on")),
        "{page}"
    );
    let body = place(&page, "<div class=\"post-body\">");
    let slip = place(&page, "<section class=\"dp-slip\"");
    assert!(body < slip, "{page}");
    assert_eq!(texts(&page, ".dp-slip h2"), ["Get the next issue by email"]);
    assert_eq!(texts(&page, ".dp-slip a.button"), ["Subscribe free"]);
}

/// On a site that takes sign-ups, every reply slip holds the form, each
/// with its own field, and no call to action button.
#[test]
fn a_site_that_takes_sign_ups_shows_the_form_in_every_slip() {
    let (_temp, out) = built(MAIL);
    let home = shown(&out, "index.html");
    assert_eq!(
        texts(&home, ".dp-slip form.subscribe button"),
        ["Subscribe", "Subscribe"]
    );
    assert!(home.contains("id=\"join-email\""), "{home}");
    assert!(home.contains("id=\"join-end-email\""), "{home}");
    assert!(texts(&home, ".dp-slip a.button").is_empty(), "{home}");
    let post = shown(&out, "fixing-slow-pages/index.html");
    assert!(post.contains("id=\"next-issue-email\""), "{post}");
}

#[test]
fn dispatch_category_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn dispatch_people_page() {
    let (_temp, out) = built("");
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
