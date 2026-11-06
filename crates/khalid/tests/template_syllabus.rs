//! The Syllabus template end to end (docs/templates/syllabus.md): `khalid
//! build` of the demo site with `[template] name = "syllabus"` and three
//! posts made lessons of one course (a series, with a length as their
//! first fact), then the header and main part of its front page, a
//! lesson, a module and the tutors page, and the progress ruler a lesson
//! shows with no script.

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

const SETTINGS: &str = "\n[template]\nname = \"syllabus\"\n[template.options]\nhero_stats = [\"2 | modules\", \"3 | lessons\"]\ntagline = [\"A short course on search basics.\"]\n[template.cta]\nlabel = \"Enrol\"\nhref = \"https://www.example.com/enrol/\"\n";

/// Three posts that become lessons: their place in the course and their
/// length, the first fact, which the outline shows.
const LESSONS: [(&str, &str); 3] = [
    (
        "content/en/posts/2026-09-14-why-internal-links-matter.md",
        "series:\n  name: Search basics\n  order: 1\nfacts:\n  - label: Length\n    value: \"12 min\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "series:\n  name: Search basics\n  order: 2\nfacts:\n  - label: Length\n    value: \"9 min\"\n  - label: Format\n    value: \"Video\"\n",
    ),
    (
        "content/en/posts/2026-09-28-what-a-sitemap-does.md",
        "series:\n  name: Search basics\n  order: 3\nfacts:\n  - label: Length\n    value: \"15 min\"\n",
    ),
];

/// The demo site built with Syllabus, with three posts made lessons.
#[cfg(test)]
fn built() -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let settings = at(temp.path(), "site.toml");
    let mut toml = fs::read_to_string(&settings).unwrap();
    toml.push_str(SETTINGS);
    fs::write(&settings, toml).unwrap();
    for (rel, lesson) in LESSONS {
        let path = at(temp.path(), rel);
        let text = fs::read_to_string(&path).unwrap();
        fs::write(path, text.replacen('\n', &format!("\n{lesson}"), 1)).unwrap();
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

/// The texts of every element `css` selects in `html`.
#[cfg(test)]
fn texts(html: &str, css: &str) -> Vec<String> {
    let html = scraper::Html::parse_fragment(html);
    let selector = scraper::Selector::parse(css).unwrap();
    html.select(&selector)
        .map(|element| element.text().collect::<String>())
        .collect()
}

#[test]
fn syllabus_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The outline lists the lessons (posts with facts) oldest first, module
/// by module in the order of categories.toml, numbered on across the
/// modules with each lesson's length. A category with no lessons is no
/// module, and the posts without facts are the course news.
#[test]
fn the_outline_numbers_lessons_across_modules_in_course_order() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".module > h3"),
        ["On-page SEO", "Technical SEO"]
    );
    assert_eq!(
        texts(&page, ".module-range"),
        ["Lessons 1 to 2", "Lesson 3"]
    );
    assert_eq!(
        texts(&page, ".module .lesson-row h4"),
        [
            "Why internal links matter",
            "How to write a title tag",
            "What a sitemap does, and what it does not",
        ]
    );
    assert_eq!(
        texts(&page, ".module .lesson-number"),
        ["Lesson 1", "Lesson 2", "Lesson 3"]
    );
    assert_eq!(
        texts(&page, ".module .lesson-length"),
        ["12 min", "9 min", "15 min"]
    );
    assert_eq!(texts(&page, ".course-map .progress-step").len(), 3);
    let news = texts(&page, ".course-news .feed-item h3");
    assert!(
        news.contains(&"Fixing slow pages, starting with the biggest win".to_owned()),
        "{news:?}"
    );
    assert!(page.contains("href=\"/blog/why-internal-links-matter/\">Start with lesson 1</a>"));
    assert_eq!(texts(&page, ".figure dd"), ["2", "3"]);
}

#[test]
fn syllabus_lesson() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "how-to-write-a-title-tag/index.html"));
}

/// A lesson shows its place in the course from its series, with no
/// script: the words, one square per lesson (done, this one, ahead), and
/// the previous and next lessons. A post outside the series has none.
#[test]
fn a_lesson_shows_its_progress_through_the_course() {
    let (_temp, out) = built();
    let page = shown(&out, "how-to-write-a-title-tag/index.html");
    assert_eq!(texts(&page, ".progress-label"), ["Lesson 2 of 3"]);
    assert_eq!(texts(&page, ".progress-note"), ["66% of the course"]);
    assert_eq!(texts(&page, ".lesson-progress .progress-step").len(), 3);
    assert_eq!(texts(&page, ".progress-done").len(), 1);
    assert_eq!(texts(&page, ".progress-now").len(), 1);
    assert!(
        texts(&page, ".lesson-next")
            .concat()
            .contains("What a sitemap does")
    );
    assert!(
        texts(&page, ".lesson-previous")
            .concat()
            .contains("Why internal links matter")
    );
    assert_eq!(texts(&page, ".lesson-facts dd"), ["9 min", "Video"]);
    assert!(!page.contains("<script"), "no script on a lesson");
    let news = shown(&out, "fixing-slow-pages/index.html");
    assert!(
        texts(&news, ".lesson-progress").is_empty(),
        "expected nothing here"
    );
    assert!(
        texts(&news, ".lesson-steps").is_empty(),
        "expected nothing here"
    );
}

#[test]
fn syllabus_module_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/on-page-seo/index.html"));
}

/// A module's page says it is a module, counts its lessons and lists them
/// oldest first, the order of the course.
#[test]
fn a_module_lists_its_lessons_in_course_order() {
    let (_temp, out) = built();
    let page = shown(&out, "category/on-page-seo/index.html");
    assert_eq!(texts(&page, ".list-kind"), ["Module"]);
    assert_eq!(texts(&page, ".list-count"), ["2 lessons"]);
    assert_eq!(
        texts(&page, ".lesson-row h2"),
        ["Why internal links matter", "How to write a title tag"]
    );
}

#[test]
fn syllabus_tutors_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
