//! The Pantry template end to end (docs/templates/pantry.md): `khalid
//! build` of the demo site with `[template] name = "pantry"` and two posts
//! given a recipe's facts (time, yield, oven, level), then the header and
//! main part of its front page, a recipe, a list and the bakers page, and
//! the recipe box, the tickets and the recipe body.

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

const SETTINGS: &str = "\n[template]\nname = \"pantry\"\n[template.options]\nhero_stats = [\"10 | recipes\", \"4 | sections\"]\ntagline = [\"Tested recipes for a home oven.\"]\n[template.cta]\nlabel = \"Start with the tin loaf\"\nhref = \"https://www.example.com/tin-loaf/\"\n";

/// Two posts that become recipes: their facts, in the order Pantry reads
/// them.
const RECIPES: [(&str, &str); 2] = [
    (
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
        "facts:\n  - label: Time\n    value: \"2 days\"\n  - label: Makes\n    value: \"1 loaf\"\n  - label: Oven\n    value: \"230°C\"\n  - label: Level\n    value: \"Steady\"\n",
    ),
    (
        "content/en/posts/2026-09-21-how-to-write-a-title-tag.md",
        "facts:\n  - label: Time\n    value: \"40 min\"\n  - label: Makes\n    value: \"12 buns\"\n",
    ),
];

/// The demo site built with Pantry, with two posts made recipes.
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
    for (rel, facts) in RECIPES {
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
fn pantry_front_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "index.html"));
}

/// The newest recipe (a post with facts) is fresh from the oven with its
/// full ticket, the next recipe an index card with the time and yield,
/// and every other post a baking note, so no post shows twice.
#[test]
fn pantry_front_page_splits_recipes_from_notes() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    assert_eq!(
        texts(&page, ".fresh-card h3"),
        ["Fixing slow pages, starting with the biggest win"]
    );
    assert_eq!(
        texts(&page, ".fresh-card .ticket dd"),
        ["2 days", "1 loaf", "230°C", "Steady"]
    );
    let recipes = texts(&page, ".recipes .ticket-card h3");
    assert_eq!(recipes, ["How to write a title tag"]);
    assert_eq!(texts(&page, ".recipes .ticket dd"), ["40 min", "12 buns"]);
    let notes = texts(&page, ".notes .feed-item h3");
    assert!(!notes.is_empty(), "expected baking notes");
    assert!(
        notes
            .iter()
            .all(|title| !recipes.contains(title) && !title.starts_with("Fixing slow pages")),
        "{notes:?}"
    );
    assert_eq!(texts(&page, ".figure dd"), ["10", "4"]);
    assert!(page.contains("Tested recipes for a home oven."));
}

/// The recipe box lists the categories that hold recipes. A category
/// whose posts are all notes, here Content strategy, is left out by the
/// same rule as the notes, not by its name.
#[test]
fn the_recipe_box_leaves_out_a_category_of_notes_only() {
    let (_temp, out) = built();
    let page = shown(&out, "index.html");
    let tabs = texts(&page, ".recipe-tabs a");
    assert!(tabs.contains(&"Technical SEO".to_owned()), "{tabs:?}");
    assert!(tabs.contains(&"On-page SEO".to_owned()), "{tabs:?}");
    assert!(!tabs.contains(&"Content strategy".to_owned()), "{tabs:?}");
}

#[test]
fn pantry_post() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "fixing-slow-pages/index.html"));
}

/// A recipe opens with its ticket of four facts in the header, and its
/// text is a recipe body, whose lists the stylesheet draws as the
/// ingredients card and the method. A note has neither.
#[test]
fn a_recipe_opens_with_its_ticket() {
    let (_temp, out) = built();
    let page = shown(&out, "fixing-slow-pages/index.html");
    assert_eq!(
        texts(&page, ".post-intro .ticket dt"),
        ["Time", "Makes", "Oven", "Level"]
    );
    assert_eq!(
        texts(&page, ".post-intro .ticket dd"),
        ["2 days", "1 loaf", "230°C", "Steady"]
    );
    assert_eq!(texts(&page, ".post-body.recipe-body").len(), 1);
    let note = shown(&out, "what-a-sitemap-does/index.html");
    assert!(
        texts(&note, ".post-intro .ticket").is_empty(),
        "a note has no ticket"
    );
    assert!(
        texts(&note, ".recipe-body").is_empty(),
        "a note has no recipe body"
    );
}

#[test]
fn pantry_category_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "category/technical-seo/index.html"));
}

#[test]
fn pantry_people_page() {
    let (_temp, out) = built();
    insta::assert_snapshot!(shown(&out, "authors/index.html"));
}
