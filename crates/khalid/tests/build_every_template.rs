//! Every template the engine has builds the demo site's home page, a post
//! and an author page, each with one `h1`, a label on every `nav` and no
//! id twice. The outline of each page's main content is a snapshot per
//! template, so a template another builder adds gets its own snapshot and
//! the Look screen can offer it (templates spec 4.5 and 4.6, item 8).

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use scraper::{ElementRef, Html, Selector};

/// The pages each template must build, by their file in the output.
#[cfg(test)]
const PAGES: [&str; 3] = [
    "index.html",
    "why-internal-links-matter/index.html",
    "authors/asif/index.html",
];

/// The elements an outline names: the page's landmarks and headings.
#[cfg(test)]
const OUTLINED: [&str; 9] = [
    "header", "nav", "section", "aside", "article", "footer", "h1", "h2", "h3",
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

/// Builds a copy of the demo site in the template `name`.
#[cfg(test)]
fn built(name: &str) -> (tempfile::TempDir, PathBuf) {
    built_with(name, "")
}

/// Builds a copy of the demo site in the template `name`, with `extra`
/// added to its `site.toml`.
#[cfg(test)]
fn built_with(name: &str, extra: &str) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy_dir(&demo, temp.path());
    let toml = at(temp.path(), "site.toml");
    let mut text = fs::read_to_string(&toml).unwrap();
    write!(text, "{extra}\n[template]\nname = \"{name}\"\n").unwrap();
    fs::write(&toml, text).unwrap();
    let out = at(temp.path(), "out");
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", temp.path().to_str().unwrap(), "build", "--out"])
        .arg(&out)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{name}: {stderr}");
    (temp, out)
}

/// Checks `html` the way the demo check reads it, and gives the outline of
/// its main content: one line per landmark and heading, indented by depth,
/// leaving out the post's own text.
#[cfg(test)]
fn outline(rel: &str, html: &str) -> String {
    let page = Html::parse_document(html);
    let all = |css: &str| page.select(&Selector::parse(css).unwrap()).count();
    assert_eq!(all("h1"), 1, "{rel}: one h1");
    assert_eq!(all("nav:not([aria-label])"), 0, "{rel}: every nav labelled");
    let ids: Vec<&str> = (page.select(&Selector::parse("[id]").unwrap()))
        .filter_map(|element| element.value().id())
        .collect();
    let mut unique = ids.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), ids.len(), "{rel}: no id twice");
    let main = page.select(&Selector::parse("main").unwrap()).next();
    let mut lines = String::new();
    walk(main.unwrap(), 0, &mut lines);
    lines
}

/// Adds the outline of `element`'s children at `depth` to `lines`.
#[cfg(test)]
fn walk(element: ElementRef<'_>, depth: usize, lines: &mut String) {
    for child in element.children().filter_map(ElementRef::wrap) {
        let tag = child.value().name();
        let classes: Vec<&str> = child.value().classes().collect();
        if classes
            .iter()
            .any(|class| ["post-body", "page-body"].contains(class))
        {
            writeln!(
                lines,
                "{}{tag}.{}: the text",
                "  ".repeat(depth),
                classes.join(".")
            )
            .unwrap();
            continue;
        }
        let next = if OUTLINED.contains(&tag) {
            let mut line = format!("{}{tag}", "  ".repeat(depth));
            for class in &classes {
                write!(line, ".{class}").unwrap();
            }
            if let Some(label) = child.value().attr("aria-label") {
                write!(line, " \"{label}\"").unwrap();
            }
            if tag.starts_with('h') && tag.len() == 2 {
                let text: String = child.text().collect();
                write!(
                    line,
                    ": {}",
                    text.split_whitespace().collect::<Vec<_>>().join(" ")
                )
                .unwrap();
            }
            writeln!(lines, "{line}").unwrap();
            depth + 1
        } else {
            depth
        };
        walk(child, next, lines);
    }
}

/// The demo's address with its mount, which every link it gives out starts.
#[cfg(test)]
const BLOG: &str = "https://www.example.com/blog/";

/// `[mail]` for a build: the newsletter form shows only on a site that
/// sends mail. Nothing is sent while building.
#[cfg(test)]
const MAIL: &str = "\n[mail]\nfrom = \"news@example.com\"\nname = \"Field Notes\"\n\n\
     [mail.smtp]\nhost = \"127.0.0.1\"\nport = 2525\nsecurity = \"none\"\nprivate_host = true\n";

/// Every file under `folder`, by its path.
#[cfg(test)]
fn files(folder: &Path, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(folder).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files(&path, found);
        } else {
            found.push(path);
        }
    }
}

/// The attributes that hold an address, `srcset` with several.
#[cfg(test)]
const ADDRESSED: [&str; 5] = ["href", "src", "action", "srcset", "poster"];

/// Each address the template puts in `html`: every attribute of
/// [`ADDRESSED`] outside the post's or page's own text, which its writer
/// links as they like.
#[cfg(test)]
fn drawn(html: &str) -> Vec<String> {
    let page = Html::parse_document(html);
    let text = Selector::parse(".post-body, .page-body").unwrap();
    let written: Vec<_> = page.select(&text).map(|body| body.id()).collect();
    let mut found = Vec::new();
    for element in page.select(&Selector::parse("*").unwrap()) {
        if element
            .ancestors()
            .any(|above| written.contains(&above.id()))
        {
            continue;
        }
        for name in ADDRESSED {
            if let Some(value) = element.value().attr(name) {
                let candidates = value.split(", ");
                found.extend(candidates.map(|it| it.split(' ').next().unwrap().to_owned()));
            }
        }
    }
    found
}

/// Each address in the `url(...)` of a stylesheet's `css`.
#[cfg(test)]
fn in_css(css: &str) -> Vec<String> {
    let values = css.split("url(").skip(1);
    let values = values.map(|rest| rest.split(')').next().unwrap());
    values
        .map(|value| value.trim_matches(['"', '\'']).to_owned())
        .collect()
}

/// Whether `address` stays on the blog: under `/blog/` on its host, or
/// under its address, or off the host altogether.
#[cfg(test)]
fn under_the_mount(address: &str) -> bool {
    let path = address.split(['?', '#']).next().unwrap();
    if path.starts_with("//") {
        return true;
    }
    if path.starts_with('/') {
        return path.starts_with("/blog/");
    }
    !address.starts_with("https://www.example.com") || address.starts_with(BLOG)
}

/// Each address in the pages and stylesheets under `out` that leaves the
/// mount, with its file, and how many addresses were read.
#[cfg(test)]
fn outside_the_mount(out: &Path) -> (Vec<String>, usize) {
    let mut all = Vec::new();
    files(out, &mut all);
    let mut outside = Vec::new();
    let mut seen = 0;
    for path in &all {
        let rel = path.strip_prefix(out).unwrap().display().to_string();
        let addresses = match path.extension().and_then(|it| it.to_str()) {
            Some("html") => drawn(&fs::read_to_string(path).unwrap()),
            Some("css") => in_css(&fs::read_to_string(path).unwrap()),
            _ => continue,
        };
        seen += addresses.len();
        let left = addresses.into_iter().filter(|it| !under_the_mount(it));
        outside.extend(left.map(|address| format!("{rel}: {address}")));
    }
    (outside, seen)
}

/// Builds the demo site mounted at /blog/ in `pack`, and checks every link,
/// asset and form stays under the mount.
#[cfg(test)]
fn keeps_everything_under_the_mount(pack: &str) {
    let (_site, out) = built_with(pack, MAIL);
    let (outside, seen) = outside_the_mount(&out);
    assert!(outside.is_empty(), "{pack}: outside /blog/: {outside:#?}");
    assert!(seen > 500, "{pack}: the check read the addresses: {seen}");
}

/// Builds the demo site in `pack`, checks its home page, a post and an
/// author page, and compares their outline with the pack's snapshot.
#[cfg(test)]
fn builds_its_pages(pack: &str) {
    let (_site, out) = built(pack);
    let mut pages = String::new();
    for rel in PAGES {
        let html = fs::read_to_string(at(&out, rel)).unwrap();
        let footer = html.split_once("<footer").unwrap().1;
        let choices = "<a href=\"/blog/privacy-choices/\">";
        assert!(footer.contains(choices), "{pack}: {rel} links the choices");
        writeln!(pages, "# {rel}\n{}", outline(rel, &html)).unwrap();
    }
    let rel = "privacy-choices/index.html";
    outline(rel, &fs::read_to_string(at(&out, rel)).unwrap());
    insta::assert_snapshot!(format!("outline_{pack}"), pages);
}

/// Home sections for the demo site: a category, the newsletter, then the
/// newest posts.
#[cfg(test)]
const SECTIONS: &str =
    "\n[reader]\nhome_sections = [{ category = \"technical-seo\" }, \"newsletter\", \"latest\"]\n";

/// The packs whose home page always carries its own sign-up form, so a
/// newsletter section adds none.
#[cfg(test)]
const OWN_SIGN_UP: [&str; 2] = ["dispatch", "versions"];

/// Builds the demo site in `pack` with [`SECTIONS`], and checks its home
/// page shows them, the newsletter form once and between the other two.
/// A title may show above its section too, in a menu or a row of jump
/// links, so the check looks for one before the form and one after it.
#[cfg(test)]
fn shows_the_home_sections(pack: &str) {
    let (_site, out) = built_with(pack, &format!("{MAIL}{SECTIONS}"));
    let html = fs::read_to_string(at(&out, "index.html")).unwrap();
    outline("index.html", &html);
    let forms = html.matches("<form class=\"subscribe\"").count();
    assert_eq!(forms, 1, "{pack}: the newsletter form shows once");
    assert!(html.contains("href=\"/blog/archive/\""), "{pack}: View all");
    let category = html.find(">Technical SEO<");
    let latest = html.rfind(">Latest posts<");
    let (Some(category), Some(latest)) = (category, latest) else {
        panic!("{pack}: both sections show");
    };
    if OWN_SIGN_UP.contains(&pack) {
        return;
    }
    let form = html.find("id=\"home-newsletter-title\"");
    let form = form.unwrap_or_else(|| panic!("{pack}: the newsletter section"));
    // Manual's sections are tiles in one grid, and the form follows it.
    let before_latest = pack == "manual" || form < latest;
    assert!(category < form && before_latest, "{pack}: in order");
}

/// Three tests per template, so the builds run side by side: each loop over
/// every template took 22 minutes. Each test's name starts with its
/// module's, the name the one looping test had.
macro_rules! every_pack {
    ($($pack:ident),* $(,)?) => {
        mod a_template_keeps_every_link_asset_and_form_under_the_mount {
            $(
                #[test]
                fn $pack() {
                    super::keeps_everything_under_the_mount(stringify!($pack));
                }
            )*
        }

        mod every_template_builds_its_post_index_and_author_pages {
            $(
                #[test]
                fn $pack() {
                    super::builds_its_pages(stringify!($pack));
                }
            )*
        }

        mod every_template_shows_the_home_sections {
            $(
                #[test]
                fn $pack() {
                    super::shows_the_home_sections(stringify!($pack));
                }
            )*
        }

        /// The packs the tests above cover.
        #[cfg(test)]
        const COVERED: &[&str] = &[$(stringify!($pack)),*];
    };
}

// Every pack, the tests' own trial and sampler among them.
every_pack!(
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
    sampler,
    spectrum,
    summit,
    syllabus,
    ticker,
    trial,
    versions,
    vital,
    wire,
    workbench,
);

/// A new pack gets its line in `every_pack!` above.
#[test]
fn every_pack_has_its_tests() {
    let mut packs: Vec<&str> = khalid_theme::packs().iter().map(|pack| pack.name).collect();
    packs.sort_unstable();
    assert_eq!(packs, COVERED);
}

#[test]
fn the_newsletter_form_and_share_links_stay_under_the_mount() {
    let (_site, out) = built_with("harbor", MAIL);
    let post = fs::read_to_string(at(&out, "why-internal-links-matter/index.html")).unwrap();
    assert!(
        post.contains("<form class=\"subscribe\" method=\"post\" action=\"/blog/subscribe\">"),
        "the template's newsletter form posts under the mount"
    );
    let share = format!("{BLOG}why-internal-links-matter/");
    let encoded = share.replace(':', "%3A").replace('/', "%2F");
    assert!(
        drawn(&post)
            .iter()
            .any(|link| link.starts_with("https://www.linkedin.com/") && link.contains(&encoded)),
        "the share links name the post under the mount"
    );
}
