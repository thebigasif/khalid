//! The demo publication overlay, `scripts/demo-publication.sh`: a ready
//! publication replaces the showcase's posts, menus and call to action in
//! its template's demo, its photos are checked against `photos.lock`, and
//! a template without a publication keeps the showcase as it is. The
//! sample publication in `fixtures/test-publications/` stands in for a
//! real one.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

/// The tests' own publications and their photo archive.
const SAMPLE: &str = "fixtures/test-publications";

/// The sample publication's template.
const TEMPLATE: &str = "harbor";

#[cfg(test)]
fn repo_file(relative: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../..");
    path.push(relative);
    path
}

#[cfg(test)]
fn within(dir: &Path, relative: &str) -> PathBuf {
    let mut path = dir.to_path_buf();
    path.push(relative);
    path
}

#[cfg(test)]
fn copy_dir(from: &Path, into: &Path) {
    fs::create_dir_all(into).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        if ["public", "data"].contains(&name.as_str()) {
            continue;
        }
        let target = within(into, &name);
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap();
        }
    }
}

/// Every file under `dir`, by its path inside it, with its bytes.
#[cfg(test)]
fn tree(dir: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut files = BTreeMap::new();
    let mut folders = vec![dir.to_path_buf()];
    while let Some(folder) = folders.pop() {
        for entry in fs::read_dir(&folder).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                folders.push(path);
            } else {
                let name = path
                    .strip_prefix(dir)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                files.insert(name, fs::read(&path).unwrap());
            }
        }
    }
    files
}

/// A copy of the showcase site, as the demo scripts make one.
#[cfg(test)]
fn showcase_copy() -> TempDir {
    let temp = tempfile::tempdir().unwrap();
    copy_dir(&repo_file("fixtures/showcase-site"), temp.path());
    temp
}

/// `scripts/demo-publication.sh` on `site` for `template`, with the
/// publications in `publications`, an empty photo cache of its own and
/// the environment variables in `extra`.
#[cfg(test)]
fn overlay(site: &Path, template: &str, publications: &Path, extra: &[(&str, &str)]) -> Output {
    let cache = tempfile::tempdir().unwrap();
    let mut command = Command::new("bash");
    command
        .arg(repo_file("scripts/demo-publication.sh"))
        .arg(site)
        .arg(template)
        .env("DEMO_PUBLICATIONS", publications)
        .env(
            "DEMO_PHOTOS_URL",
            repo_file(&format!("{SAMPLE}/demo-photos.tar.gz")),
        )
        .env("DEMO_PHOTOS_CACHE", cache.path())
        .env_remove("DEMO_PUBLICATION_ALL")
        .envs(extra.iter().copied());
    command.output().unwrap()
}

/// The sample publication's template demo, made as `scripts/pages.sh`
/// makes one: the publication, then the template's sample brand, then the
/// template, built into the returned folder.
#[cfg(test)]
fn sample_demo() -> (TempDir, TempDir) {
    let site = showcase_copy();
    let applied = overlay(site.path(), TEMPLATE, &repo_file(SAMPLE), &[]);
    assert!(applied.status.success(), "{applied:?}");
    let brand = Command::new("bash")
        .arg(repo_file("scripts/demo-brand.sh"))
        .arg(site.path())
        .arg(TEMPLATE)
        .env("DEMO_BRAND_ALL", "1")
        .output()
        .unwrap();
    assert!(brand.status.success(), "{brand:?}");
    let toml = within(site.path(), "site.toml");
    let text = fs::read_to_string(&toml).unwrap();
    fs::write(
        &toml,
        format!("{text}\n[template]\nname = \"{TEMPLATE}\"\n"),
    )
    .unwrap();
    let out = tempfile::tempdir().unwrap();
    let build = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.path().to_str().unwrap()])
        .args(["build", "--out", out.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert!(build.status.success(), "{build:?}");
    (site, out)
}

#[cfg(test)]
fn page(out: &Path, rest: &str) -> String {
    let file = within(out, &format!("{rest}index.html"));
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("no page at {}", file.display()))
}

#[test]
fn a_ready_publication_builds_its_demo_with_its_own_title_posts_menus_and_call_to_action() {
    let (_site, out) = sample_demo();
    let home = page(out.path(), "");
    for wanted in [
        "Harbor Kitchen Notes",
        "How to plan a week of dinners in twenty minutes",
        "Why we pack herbs in paper, not plastic",
        "Weeknights",
        "About the kitchen",
        "See this week",
    ] {
        assert!(home.contains(wanted), "the home page lacks {wanted}");
    }
    for showcase in ["Larkspur", "Feature tour", "content benchmark"] {
        assert!(
            !home.contains(showcase),
            "the home page still shows {showcase}"
        );
    }
    let post = page(out.path(), "plan-a-week-of-dinners/");
    assert!(
        post.contains("/blog/plan-a-week-of-dinners/lead.png"),
        "the post shows no lead photo"
    );
    assert!(within(out.path(), "plan-a-week-of-dinners/lead.png").exists());
    assert!(page(out.path(), "herbs-in-paper/").contains("<table"));
    assert!(page(out.path(), "authors/ruth-calder/").contains("Ruth Calder"));
}

/// Every link and picture from a page of the demo to the demo itself
/// leads to a page or file it built, so no showcase-only page is left
/// behind.
#[test]
fn the_built_demo_links_to_no_showcase_only_page() {
    let (_site, out) = sample_demo();
    let mut broken = BTreeMap::new();
    for (name, bytes) in tree(out.path()) {
        if Path::new(&name).extension().is_none_or(|ext| ext != "html") {
            continue;
        }
        let html = String::from_utf8(bytes).unwrap();
        let links = html.split("href=\"").skip(1);
        for rest in links.chain(html.split("src=\"").skip(1)) {
            let target = rest.split('"').next().unwrap_or_default();
            let target = target
                .strip_prefix("https://www.example.com")
                .unwrap_or(target);
            let Some(path) = target.strip_prefix("/blog/") else {
                continue;
            };
            let path = path.split(['#', '?']).next().unwrap_or_default();
            let file = if path.is_empty() || path.ends_with('/') {
                format!("{path}index.html")
            } else {
                path.to_owned()
            };
            if !within(out.path(), &file).exists() {
                broken.insert(target.to_owned(), name.clone());
            }
        }
    }
    assert_eq!(broken, BTreeMap::new(), "link and the page that holds it");
}

#[test]
fn the_brand_logo_lands_on_the_publications_about_page() {
    let (site, out) = sample_demo();
    let logo = fs::read_to_string(repo_file(&format!(
        "fixtures/showcase-brands/{TEMPLATE}/theme/logo.svg"
    )))
    .unwrap();
    let placed =
        fs::read_to_string(within(site.path(), "content/en/pages/about/logo.svg")).unwrap();
    // The brand's logo spells the publication's name, not the showcase's,
    // and is otherwise the brand's own.
    assert!(logo.contains(">Larkspur Ledger<"), "{logo}");
    assert!(placed.contains(">Harbor Kitchen Notes<"), "{placed}");
    assert!(!placed.contains("Larkspur Ledger"), "{placed}");
    let shape = |svg: &str| {
        svg.lines()
            .filter(|line| !line.contains("<text"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(shape(&placed), shape(&logo));
    assert_eq!(
        fs::read_to_string(within(out.path(), "about/logo.svg")).unwrap(),
        placed
    );
    assert!(page(out.path(), "about/").contains("About Harbor Kitchen"));
}

/// Wire's logo sets the showcase's name in capitals, so the publication's
/// name goes in as capitals too.
#[test]
fn a_logo_in_capitals_spells_the_publications_name_in_capitals() {
    let site = showcase_copy();
    let toml = within(site.path(), "site.toml");
    let settings = fs::read_to_string(&toml).unwrap();
    let renamed = settings.replacen("name = \"Larkspur Ledger\"", "name = \"Port City Wire\"", 1);
    assert_ne!(renamed, settings);
    fs::write(&toml, renamed).unwrap();
    let brand = Command::new("bash")
        .arg(repo_file("scripts/demo-brand.sh"))
        .arg(site.path())
        .arg("wire")
        .env("DEMO_BRAND_ALL", "1")
        .output()
        .unwrap();
    assert!(brand.status.success(), "{brand:?}");
    let logo =
        fs::read_to_string(repo_file("fixtures/showcase-brands/wire/theme/logo.svg")).unwrap();
    assert!(logo.contains(">LARKSPUR LEDGER<"), "{logo}");
    let placed =
        fs::read_to_string(within(site.path(), "content/en/pages/about/logo.svg")).unwrap();
    assert!(placed.contains(">PORT CITY WIRE<"), "{placed}");
    assert!(!placed.to_uppercase().contains("LARKSPUR"), "{placed}");
}

/// A template whose sample brand is not ready keeps the showcase's theme,
/// which names its logo on the about page, so the logo stays there when
/// the publication's about page brings none.
#[test]
fn without_a_brand_the_showcase_logo_stays_on_the_about_page() {
    let site = showcase_copy();
    let applied = overlay(site.path(), TEMPLATE, &repo_file(SAMPLE), &[]);
    assert!(applied.status.success(), "{applied:?}");
    let logo = "content/en/pages/about/logo.svg";
    assert_eq!(
        fs::read(within(site.path(), logo)).unwrap(),
        fs::read(within(&repo_file("fixtures/showcase-site"), logo)).unwrap()
    );
    assert!(within(site.path(), "content/en/pages/about/index.md").exists());
}

#[test]
fn a_template_without_a_publication_builds_from_the_showcase_unchanged() {
    let showcase = tree(&repo_file("fixtures/showcase-site"));
    let site = showcase_copy();
    let output = overlay(site.path(), "wire", &repo_file(SAMPLE), &[]);
    assert!(output.status.success(), "{output:?}");
    assert_eq!(tree(site.path()), showcase);
}

/// A publication without the `ready` file applies only for review, with
/// `DEMO_PUBLICATION_ALL=1`.
#[test]
fn a_publication_not_yet_ready_applies_only_for_review() {
    let publications = tempfile::tempdir().unwrap();
    copy_dir(&repo_file(SAMPLE), publications.path());
    fs::remove_file(within(publications.path(), "harbor/ready")).unwrap();
    let showcase = tree(&repo_file("fixtures/showcase-site"));
    let site = showcase_copy();
    let output = overlay(site.path(), TEMPLATE, publications.path(), &[]);
    assert!(output.status.success(), "{output:?}");
    assert_eq!(tree(site.path()), showcase);
    let output = overlay(
        site.path(),
        TEMPLATE,
        publications.path(),
        &[("DEMO_PUBLICATION_ALL", "1")],
    );
    assert!(output.status.success(), "{output:?}");
    let toml = fs::read_to_string(within(site.path(), "site.toml")).unwrap();
    assert!(toml.contains("name = \"Harbor Kitchen Notes\""), "{toml}");
}

#[test]
fn a_photo_whose_checksum_differs_from_photos_lock_stops_with_a_plain_message() {
    let publications = tempfile::tempdir().unwrap();
    copy_dir(&repo_file(SAMPLE), publications.path());
    let lock = within(publications.path(), "harbor/photos.lock");
    let text = fs::read_to_string(&lock).unwrap().replace(
        "23d50f6150653f170992cd42c935e47de7e6c532c93f01943a36b99bdc892747",
        "0000000000000000000000000000000000000000000000000000000000000000",
    );
    fs::write(&lock, text).unwrap();
    let showcase = tree(&repo_file("fixtures/showcase-site"));
    let site = showcase_copy();
    let output = overlay(site.path(), TEMPLATE, publications.path(), &[]);
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    let said = String::from_utf8_lossy(&output.stdout);
    assert!(
        said.contains("en/posts/2026-10-06-plan-a-week-of-dinners/lead.png")
            && said.contains("does not match its checksum in photos.lock"),
        "{said}"
    );
    assert!(!said.contains('\u{2014}'), "{said}");
    assert_eq!(
        tree(site.path()),
        showcase,
        "the copy changed before the stop"
    );
}

#[test]
fn a_publication_without_photos_lock_fetches_nothing() {
    let publications = tempfile::tempdir().unwrap();
    copy_dir(&repo_file(SAMPLE), publications.path());
    fs::remove_file(within(publications.path(), "harbor/photos.lock")).unwrap();
    let site = showcase_copy();
    let cache = tempfile::tempdir().unwrap();
    let output = Command::new("bash")
        .arg(repo_file("scripts/demo-publication.sh"))
        .arg(site.path())
        .arg(TEMPLATE)
        .env("DEMO_PUBLICATIONS", publications.path())
        .env(
            "DEMO_PHOTOS_URL",
            within(cache.path(), "no-such-archive.tar.gz"),
        )
        .env("DEMO_PHOTOS_CACHE", cache.path())
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert_eq!(fs::read_dir(cache.path()).unwrap().count(), 0);
}

/// The Pages build and the picture review put the publication on before
/// the sample brand, so the brand's logo lands on the publication's about
/// page, and the review shows publications that are not ready yet.
#[test]
fn the_pages_build_and_the_checks_apply_the_publication_before_the_brand() {
    for script in ["scripts/pages.sh", "scripts/demo-check/showcase.sh"] {
        let text = fs::read_to_string(repo_file(script)).unwrap();
        let publication = text.find("scripts/demo-publication.sh\"");
        let brand = text.find("scripts/demo-brand.sh\"");
        assert!(
            matches!((publication, brand), (Some(p), Some(b)) if p < b),
            "{script}"
        );
    }
    let pictures = fs::read_to_string(repo_file("scripts/demo-pictures.sh")).unwrap();
    assert!(pictures.contains("export DEMO_PUBLICATION_ALL=1"));
}
