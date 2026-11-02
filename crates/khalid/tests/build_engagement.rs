//! `khalid build` with the engagement features: charts drawn from the
//! post or a CSV file beside it.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[cfg(test)]
fn khalid(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(args)
        .output()
        .unwrap()
}

#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

/// A new site in `folder` with `site.toml` ending in `settings`, built,
/// and the HTML of its page at `rel` in `public/`.
#[cfg(test)]
fn built(folder: &Path, settings: &str, rel: &str) -> String {
    let site = folder.to_str().unwrap();
    khalid(&["init", site, "--address", "https://www.example.com"]);
    let toml = at(folder, "site.toml");
    let mut text = fs::read_to_string(&toml).unwrap();
    text.push_str(settings);
    fs::write(&toml, text).unwrap();
    let output = khalid(&["--site", site, "build"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    fs::read_to_string(at(folder, &format!("public/{rel}")))
        .unwrap_or_else(|error| panic!("{rel}: {error}"))
}

/// Writes the post `slug` as a folder with `body` and the files `beside`.
#[cfg(test)]
fn post(folder: &Path, slug: &str, body: &str, beside: &[(&str, &str)]) {
    let post = at(folder, &format!("content/en/posts/{slug}"));
    fs::create_dir_all(&post).unwrap();
    let front =
        format!("---\ntitle: {slug}\nslug: {slug}\ndate: 2026-09-14\nauthor: asif\n---\n\n");
    fs::write(at(&post, "index.md"), format!("{front}{body}")).unwrap();
    for (name, text) in beside {
        fs::write(at(&post, name), text).unwrap();
    }
}

#[test]
fn build_draws_a_chart_from_the_csv_file_beside_its_post() {
    let temp = tempfile::tempdir().unwrap();
    let body = "Search brought most readers, as [the home page](/) says.\n\n\
                :::chart{type=\"line\" title=\"Visits by month\" src=\"visits.csv\"}\n:::\n\n\
                :::chart{type=\"bar\" title=\"Clicks by page\"}\nPage, Clicks\nHome, 40\nPricing, 25\n:::\n";
    post(
        temp.path(),
        "visits",
        body,
        &[(
            "visits.csv",
            "Month, Visits\nJan, 120\nFeb, 160\nMar, 210\n",
        )],
    );
    let page = built(temp.path(), "", "visits/index.html");
    assert!(
        page.contains("<figure class=\"chart chart-line\">"),
        "{page}"
    );
    assert!(
        page.contains("<tr><th scope=\"row\">Mar</th><td>210</td></tr>"),
        "{page}"
    );
    assert!(
        page.contains("href=\"visits.csv\" download>CSV</a>"),
        "{page}"
    );
    assert!(
        page.contains("<figure class=\"chart chart-bar\">"),
        "{page}"
    );
    assert!(!page.contains("<script>"), "no inline script: {page}");
    let csv = fs::read_to_string(at(temp.path(), "public/visits/visits.csv")).unwrap();
    assert!(
        csv.starts_with("Month, Visits"),
        "the file is published beside the page"
    );
    let source = fs::read_to_string(at(temp.path(), "content/en/posts/visits/index.md")).unwrap();
    assert!(
        !source.contains("Jan, 120"),
        "the post's file is never changed"
    );
}

#[test]
fn build_frames_a_page_only_on_a_domain_the_site_allows() {
    let temp = tempfile::tempdir().unwrap();
    let body = "Try it, as [the home page](/) says.\n\n\
                ::embed{src=\"https://tools.example.com/roi\" title=\"ROI calculator\" height=\"600\"}\n\n\
                ::embed{src=\"https://other.example.com/roi\" title=\"Another tool\"}\n";
    post(temp.path(), "roi", body, &[]);
    let settings = "\n[media]\nembed_domains = [\"tools.example.com\"]\n";
    let page = built(temp.path(), settings, "roi/index.html");
    let frame = "<iframe src=\"https://tools.example.com/roi\" title=\"ROI calculator\" width=\"560\" height=\"600\" loading=\"lazy\"";
    assert!(page.contains(frame), "{page}");
    let sandbox = "sandbox=\"allow-scripts allow-same-origin allow-forms allow-popups\"";
    assert!(page.contains(sandbox), "{page}");
    let link = "<p class=\"bookmark\"><a href=\"https://other.example.com/roi\"";
    assert!(page.contains(link), "{page}");
}

#[test]
fn build_tags_links_to_the_owners_product_and_no_other_link() {
    let temp = tempfile::tempdir().unwrap();
    let body = "Start on [the app](https://app.example.com/signup#plans), read \
                [the guide](https://developers.google.com/search) or go back to \
                [the home page](/).\n\n\
                ::button{href=\"https://app.example.com/trial\" label=\"Start a trial\"}\n\n::cta\n";
    post(temp.path(), "tagged", body, &[]);
    let settings = "\n[template.cta]\nlabel = \"Try the app\"\nhref = \"https://app.example.com/\"\n\n\
                    [seo.links.tracking]\ndomains = [\"app.example.com\"]\n";
    let page = built(temp.path(), settings, "tagged/index.html");
    let tags = "utm_source=blog&amp;utm_medium=content&amp;utm_campaign=tagged";
    let app = format!("<a href=\"https://app.example.com/signup?{tags}#plans\">the app</a>");
    assert!(page.contains(&app), "{page}");
    let button = format!("href=\"https://app.example.com/trial?{tags}\">Start a trial</a>");
    assert!(page.contains(&button), "{page}");
    let cta = "href=\"https://app.example.com/?utm_source=blog&amp;utm_medium=content&amp;utm_campaign=cta\">Try the app</a>";
    assert!(page.contains(cta), "{page}");
    assert!(
        page.contains("<a href=\"https://developers.google.com/search\">"),
        "{page}"
    );
    assert_eq!(page.matches("utm_source").count(), 3, "{page}");
}

#[test]
fn build_shows_the_preferred_source_button_where_the_table_places_it() {
    let button = "<p class=\"preferred-source\"><a href=\"https://www.google.com/preferences/source?q=www.example.com\" rel=\"noopener\">";
    let words = "<span>Add us as a preferred source on Google</span>";
    let body = "Read on, as [the home page](/) says.\n";
    for (template, place, in_post, in_footer) in [
        ("classic", "", 1, 1),
        ("atlas", "place = \"post\"\n", 1, 0),
        ("ledger", "place = \"footer\"\n", 0, 1),
    ] {
        let temp = tempfile::tempdir().unwrap();
        post(temp.path(), "source", body, &[]);
        let settings =
            format!("\n[template]\nname = \"{template}\"\n\n[template.preferred_source]\n{place}");
        let page = built(temp.path(), &settings, "source/index.html");
        let (main, footer) = page.split_once("<footer class=\"site-footer\">").unwrap();
        assert_eq!(main.matches(button).count(), in_post, "{template}: {main}");
        assert_eq!(
            footer.matches(button).count(),
            in_footer,
            "{template}: {footer}"
        );
        assert!(in_post + in_footer == 0 || page.contains(words), "{page}");
    }
    let temp = tempfile::tempdir().unwrap();
    post(temp.path(), "source", body, &[]);
    let page = built(temp.path(), "", "source/index.html");
    assert!(!page.contains("preferred-source"), "off without the table");
}

/// Writes the main newsletter's page and two sent issues and one planned
/// for next year into the site in `folder`.
#[cfg(test)]
fn newsletter(folder: &Path) {
    let main = at(folder, "content/newsletters/main");
    fs::create_dir_all(at(&main, "issues")).unwrap();
    let page =
        "# The Field Letter\n\nFive links on search every other Tuesday, read in five minutes.\n";
    fs::write(at(&main, "page.md"), page).unwrap();
    for (name, text) in [
        (
            "2026-09-01-first-links.md",
            "# The first links\n\nWhy titles matter.\n",
        ),
        (
            "2026-10-06-sitemaps.md",
            "# Sitemaps, plainly\n\nWhat a sitemap does.\n",
        ),
        ("2099-01-01-planned.md", "# Planned\n\nNot sent yet.\n"),
        ("notes.md", "# Not an issue\n\nNo day in the name.\n"),
    ] {
        fs::write(at(&main, &format!("issues/{name}")), text).unwrap();
    }
}

#[test]
fn build_writes_the_newsletter_page_and_a_page_per_sent_issue() {
    let temp = tempfile::tempdir().unwrap();
    newsletter(temp.path());
    let mail =
        "\n[mail]\nfrom = \"letters@example.com\"\n\n[mail.smtp]\nhost = \"smtp.example.com\"\n";
    let page = built(temp.path(), mail, "newsletter/index.html");
    assert!(page.contains("<h1>The Field Letter</h1>"), "{page}");
    assert!(page.contains("every other Tuesday"), "{page}");
    assert!(
        page.contains("<form class=\"subscribe\""),
        "a server site takes sign-ups: {page}"
    );
    let (_, sample) = page.split_once("<section class=\"issue-sample\"").unwrap();
    assert!(
        sample.contains("<a href=\"/newsletter/sitemaps/\">Sitemaps, plainly</a>"),
        "the newest is the sample: {page}"
    );
    let list = page.split("<ol class=\"issue-list\"").nth(1).unwrap();
    let first = list.find("Sitemaps, plainly").unwrap();
    assert!(
        first < list.find("The first links").unwrap(),
        "newest first: {list}"
    );
    assert!(
        list.contains("<time datetime=\"2026-10-06\">6 October 2026</time>"),
        "{list}"
    );
    assert!(
        !page.contains("Planned") && !page.contains("Not an issue"),
        "{page}"
    );
    let issue =
        fs::read_to_string(at(temp.path(), "public/newsletter/sitemaps/index.html")).unwrap();
    assert!(issue.contains("<h1>Sitemaps, plainly</h1>"), "{issue}");
    assert!(
        issue.contains("Sent on <time datetime=\"2026-10-06\">"),
        "{issue}"
    );
    assert!(
        issue.contains("<a href=\"/newsletter/\">Every issue of the newsletter</a>"),
        "{issue}"
    );
    assert!(!at(temp.path(), "public/newsletter/planned").exists());
}

#[test]
fn a_static_site_shows_the_newsletter_archive_without_the_form() {
    let temp = tempfile::tempdir().unwrap();
    newsletter(temp.path());
    let settings =
        "\n[mail]\nfrom = \"letters@example.com\"\n\n[mail.smtp]\nhost = \"smtp.example.com\"\n";
    let toml = at(temp.path(), "site.toml");
    let site = temp.path().to_str().unwrap();
    khalid(&["init", site, "--address", "https://www.example.com"]);
    let text = fs::read_to_string(&toml)
        .unwrap()
        .replace("mode = \"server\"", "mode = \"static\"");
    fs::write(&toml, format!("{text}{settings}")).unwrap();
    let output = khalid(&["--site", site, "build"]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let page = fs::read_to_string(at(temp.path(), "public/newsletter/index.html")).unwrap();
    assert!(page.contains("Past issues"), "{page}");
    assert!(!page.contains("<form"), "{page}");
}
