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
