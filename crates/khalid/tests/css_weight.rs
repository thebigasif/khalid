//! The CSS budget (docs/standards/performance.md, "CSS weight"): on every
//! template's showcase site, in its sample brand, the stylesheets each of
//! the home page, a post, a category page and an author page links weigh
//! at most 10 KB together, compressed with Brotli as Khalid serves them.
//! Each page links only the rules its HTML can use, so a template that
//! adds rules for one kind of page does not make the others heavier.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The most CSS one page may download, in Brotli bytes. A budget is never
/// raised to make this check pass.
#[cfg(test)]
const BUDGET: usize = 10 * 1024;

/// The pages measured, by what they show and their file in the output.
#[cfg(test)]
const PAGES: [(&str, &str); 4] = [
    ("home", "index.html"),
    ("post", "ai-search-guide/index.html"),
    ("category", "category/seo-writing/index.html"),
    ("author", "authors/maren-okafor/index.html"),
];

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

/// Builds the showcase site in the template `pack`, dressed in the pack's
/// sample brand when it has a ready one, as `scripts/demo-brand.sh` does,
/// as a static site.
#[cfg(test)]
fn built(pack: &str) -> (tempfile::TempDir, PathBuf) {
    let root = at(Path::new(env!("CARGO_MANIFEST_DIR")), "../..");
    let temp = tempfile::tempdir().unwrap();
    copy_dir(&at(&root, "fixtures/showcase-site"), temp.path());
    let brand = at(&root, &format!("fixtures/showcase-brands/{pack}"));
    if at(&brand, "ready").is_file() {
        copy_dir(&at(&brand, "theme"), &at(temp.path(), "theme"));
        let logo = at(temp.path(), "theme/logo.svg");
        if logo.is_file() {
            let about = at(temp.path(), "content/en/pages/about/logo.svg");
            fs::rename(&logo, about).unwrap();
        }
    }
    let toml = at(temp.path(), "site.toml");
    let text = fs::read_to_string(&toml).unwrap();
    let text = text.replace("mode = \"server\"", "mode = \"static\"");
    fs::write(&toml, format!("{text}\n[template]\nname = \"{pack}\"\n")).unwrap();
    let out = at(temp.path(), "out");
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", temp.path().to_str().unwrap(), "build", "--out"])
        .arg(&out)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{pack}: {stderr}");
    (temp, out)
}

/// The Brotli bytes of every stylesheet `html` links, read from the
/// compressed copies the build wrote beside them in `out`.
#[cfg(test)]
fn css_bytes(out: &Path, html: &str) -> usize {
    let mut bytes = 0;
    for piece in html.split("<link rel=\"stylesheet\" href=\"").skip(1) {
        let href = piece.split('"').next().unwrap();
        let name = href.rsplit("/assets/").next().unwrap();
        let file = at(out, &format!("assets/{name}.br"));
        bytes += fs::metadata(&file)
            .unwrap_or_else(|_| panic!("{href}"))
            .len();
    }
    usize::try_from(bytes).unwrap()
}

/// Builds `pack`'s showcase and fails when one of its measured pages
/// downloads more than [`BUDGET`] bytes of CSS.
#[cfg(test)]
fn within_budget(pack: &str) {
    let (_temp, out) = built(pack);
    let mut over = Vec::new();
    for (what, file) in PAGES {
        let html = fs::read_to_string(at(&out, file)).unwrap();
        let bytes = css_bytes(&out, &html);
        println!("{pack} {what}: {bytes} bytes of CSS with Brotli");
        if bytes > BUDGET {
            over.push(format!("{what}: {bytes} bytes"));
        }
    }
    assert!(over.is_empty(), "{pack} is over {BUDGET} bytes: {over:?}");
}

/// One test per template, so the builds run side by side, and the list
/// they cover.
macro_rules! every_template {
    ($($pack:ident),* $(,)?) => {
        $(
            #[test]
            fn $pack() {
                within_budget(stringify!($pack));
            }
        )*

        /// The templates the tests above cover.
        #[cfg(test)]
        const COVERED: &[&str] = &[$(stringify!($pack)),*];
    };
}

every_template!(
    almanac,
    aperture,
    atelier,
    atlas,
    beacon,
    brief,
    channel,
    chapter,
    classic,
    compass,
    dialogue,
    dispatch,
    encore,
    fixture,
    folio,
    frequency,
    gallery,
    gazette,
    glossy,
    harbor,
    hearth,
    hub,
    inkwell,
    itinerary,
    keystone,
    lattice,
    ledger,
    letterpress,
    locale,
    manual,
    margin,
    masthead,
    meridian,
    mosaic,
    notebook,
    pantry,
    plinth,
    quill,
    reel,
    research,
    respawn,
    spectrum,
    summit,
    syllabus,
    ticker,
    versions,
    vital,
    wire,
    workbench,
);

/// A new template gets its line in `every_template!` above. The trial and
/// sampler packs are the tests' own and never ship.
#[test]
fn every_template_is_held_to_the_budget() {
    let mut shipped: Vec<&str> = (khalid_theme::packs().iter())
        .map(|pack| pack.name)
        .filter(|name| !["trial", "sampler"].contains(name))
        .collect();
    shipped.sort_unstable();
    assert_eq!(shipped, COVERED);
}
