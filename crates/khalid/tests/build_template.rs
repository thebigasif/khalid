//! `[template]` in `site.toml` end to end: `khalid build` of the demo site
//! with the test-only trial pack differs from classic in its stylesheet,
//! its not-found page and a reader switch, classic builds exactly what a
//! site without the table builds, and a wrong name or option stops the
//! build with its line (templates spec 4.1).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

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

/// The demo post every test reads.
#[cfg(test)]
const POST_FILE: &str = "content/en/posts/2026-09-28-what-a-sitemap-does.md";

/// Builds a copy of the demo site with `table` added to its `site.toml`.
#[cfg(test)]
fn build(table: &str) -> (tempfile::TempDir, PathBuf, Output) {
    build_with(table, "")
}

/// Builds like [`build`], with `post_end` added to the end of the demo
/// post in [`POST_FILE`].
#[cfg(test)]
fn build_with(table: &str, post_end: &str) -> (tempfile::TempDir, PathBuf, Output) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    for (rel, end) in [("site.toml", table), (POST_FILE, post_end)] {
        let file = at(temp.path(), rel);
        let mut text = fs::read_to_string(&file).unwrap();
        text.push_str(end);
        fs::write(&file, text).unwrap();
    }
    let out = at(temp.path(), "out");
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", temp.path().to_str().unwrap(), "build", "--out"])
        .arg(&out)
        .output()
        .unwrap();
    (temp, out, output)
}

/// Builds like [`build`] and expects success.
#[cfg(test)]
fn built(table: &str) -> (tempfile::TempDir, PathBuf) {
    let (temp, out, output) = build(table);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    (temp, out)
}

#[cfg(test)]
fn page(out: &Path, rel: &str) -> String {
    fs::read_to_string(at(out, rel)).unwrap_or_else(|error| panic!("{rel}: {error}"))
}

/// The text of the one stylesheet in `out/assets/`.
#[cfg(test)]
fn stylesheet(out: &Path) -> String {
    let assets = fs::read_dir(at(out, "assets")).unwrap();
    let css = assets
        .map(|entry| entry.unwrap().path())
        .find(|path| path.extension().is_some_and(|ext| ext == "css"))
        .unwrap();
    fs::read_to_string(css).unwrap()
}

#[test]
fn classic_builds_what_a_site_without_the_table_builds() {
    let (_none, without) = built("");
    let (_classic, classic) = built("\n[template]\nname = \"classic\"\n");
    assert_eq!(stylesheet(&without), stylesheet(&classic));
    for rel in ["index.html", "404.html", "what-a-sitemap-does/index.html"] {
        assert_eq!(page(&without, rel), page(&classic, rel), "{rel}");
    }
}

#[test]
fn the_trial_pack_changes_the_build_and_reads_its_options() {
    let (_classic, classic) = built("");
    let table = "\n[template]\nname = \"trial\"\n[template.options]\ngreeting = \"welcome\"\n";
    let (_trial, trial) = built(table);

    let css = stylesheet(&trial);
    assert!(css.contains(".trial-greeting"), "the pack's rules ship");
    assert!(css.contains("--text-h1: clamp(3rem, 2.6667rem + 1.6667vw, 4rem);"));
    assert!(
        !stylesheet(&classic).contains(".trial-greeting"),
        "classic has none"
    );

    let missing = page(&trial, "404.html");
    assert!(
        missing.contains("<p class=\"trial-greeting\">welcome</p>"),
        "{missing}"
    );

    let post = "what-a-sitemap-does/index.html";
    assert!(
        page(&trial, post).contains("<div class=\"share\">"),
        "trial shows share links"
    );
    assert!(!page(&classic, post).contains("class=\"share\""));
    assert!(page(&classic, post).contains("min read</time>"));
    assert!(
        !page(&trial, post).contains("min read</time>"),
        "trial turns reading time off"
    );
}

#[test]
fn the_newsletter_form_shows_only_when_the_site_takes_sign_ups() {
    let post = "what-a-sitemap-does/index.html";
    let (_plain, plain) = built("\n[template]\nname = \"trial\"\n");
    assert!(!page(&plain, post).contains("newsletter-slot"), "no [mail]");

    let mail = "\n[mail]\nfrom = \"news@example.com\"\n\n[mail.smtp]\nhost = \"127.0.0.1\"\n\
                port = 2525\nsecurity = \"none\"\nprivate_host = true\n";
    let (_mail, out) = built(&format!("{mail}\n[template]\nname = \"trial\"\n"));
    let form = "<form class=\"subscribe\" method=\"post\" action=\"/blog/subscribe\">";
    assert!(page(&out, post).contains(form), "{}", page(&out, post));
    assert!(page(&out, "index.html").contains("newsletter-slot"));
    let (_classic, classic) = built(mail);
    assert!(!page(&classic, post).contains("newsletter-slot"), "classic");
}

#[test]
fn the_call_to_action_shows_as_a_band_and_a_header_button() {
    let cta = "\n[template]\nname = \"trial\"\n[template.cta]\nlabel = \"Start a free trial\"\n\
               href = \"https://www.example.com/signup/\"\nnote = \"No card needed.\"\n";
    let (_trial, out) = built(cta);
    let button = "<a class=\"button header-button\" href=\"https://www.example.com/signup/\">";
    for rel in ["index.html", "what-a-sitemap-does/index.html"] {
        let html = page(&out, rel);
        assert!(
            html.contains("<p class=\"cta-note\">No card needed.</p>"),
            "{rel}"
        );
        assert!(html.contains(button), "{rel}");
    }
    assert!(
        page(&out, "404.html").contains(button),
        "every page's header"
    );

    let (_classic, classic) = built(&cta.replace("trial", "classic"));
    let html = page(&classic, "what-a-sitemap-does/index.html");
    assert!(!html.contains("cta-band") && !html.contains("header-button"));
}

#[test]
fn a_cta_block_shows_the_band_in_the_post_even_in_classic() {
    let cta = "\n[template.cta]\nlabel = \"Book a demo\"\nhref = \"/blog/demo/\"\n";
    let end = "\n\n::cta\n\nOne more line.\n";
    let (_temp, out, output) = build_with(cta, end);
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let html = page(&out, "what-a-sitemap-does/index.html");
    let band = "<div class=\"cta-band\">\n<p><a class=\"button\" href=\"/blog/demo/\">Book a demo</a></p>\n</div>\n<p>One more line.</p>";
    assert!(html.contains(band), "{html}");
    assert!(!html.contains("khalid-cta"), "the mark is gone");
}

#[test]
fn the_sites_own_reader_key_wins_over_the_pack() {
    let table = "\n[reader]\nreading_time = true\n\n[template]\nname = \"trial\"\n";
    let (_temp, out) = built(table);
    assert!(page(&out, "what-a-sitemap-does/index.html").contains("min read</time>"));
}

#[test]
fn a_wrong_name_or_option_stops_the_build_with_its_line() {
    let (temp, _, output) = build("\n[template]\nname = \"ledgr\"\n");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(
        stderr.contains("there is no template named ledgr"),
        "{stderr}"
    );
    let text = fs::read_to_string(at(temp.path(), "site.toml")).unwrap();
    let line = text.lines().count();
    assert!(stderr.contains(&format!("line {line}")), "{stderr}");

    let table = "\n[template]\nname = \"trial\"\n[template.options]\ncolumns = 9\n";
    let (_temp, _, output) = build(table);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(
        stderr.contains("takes a whole number from 2 to 4"),
        "{stderr}"
    );
}
