//! `khalid import` (KH-API-7): a dry run writes nothing, an import writes
//! the posts with their slugs and stores the redirects the site can answer,
//! and a second import refuses to overwrite.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;
use tempfile::TempDir;

/// A site with the demo's settings and no content yet.
#[cfg(test)]
fn site() -> TempDir {
    let settings = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/demo-site/site.toml"
    );
    let temp = tempfile::tempdir().unwrap();
    fs::copy(settings, file(temp.path(), "site.toml")).unwrap();
    fs::create_dir(file(temp.path(), "content")).unwrap();
    temp
}

/// A site whose linter only warns about a post with no internal link, so
/// a lone imported post goes live and its redirects are stored.
#[cfg(test)]
fn lenient_site() -> TempDir {
    let site = site();
    let rules = "[rules]\nno-internal-link = \"warn\"\n";
    fs::write(file(site.path(), "lint.toml"), rules).unwrap();
    site
}

/// A fixture of the importer crate, by its path inside `fixtures/`.
#[cfg(test)]
fn fixture(name: &str) -> String {
    format!(
        "{}/../khalid-import/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

/// Runs `khalid --site <site>` with `args`, and returns the exit code and
/// what it printed.
#[cfg(test)]
fn khalid(site: &Path, args: &[&str]) -> (i32, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap()])
        .args(args)
        .output()
        .unwrap();
    let mut printed = String::from_utf8(output.stdout).unwrap();
    printed.push_str(&String::from_utf8(output.stderr).unwrap());
    (output.status.code().unwrap(), printed)
}

/// A file inside `site`.
#[cfg(test)]
fn file(site: &Path, path: &str) -> PathBuf {
    let mut file = site.to_path_buf();
    file.push(path);
    file
}

const POST: &str = "content/en/posts/2024-03-12-keep-a-starter-alive.md";

#[test]
fn a_dry_run_reports_and_writes_nothing() {
    let site = site();
    let export = fixture("wordpress/basic.xml");
    let (code, printed) = khalid(site.path(), &["import", "wordpress", &export, "--dry-run"]);
    assert_eq!(code, 0, "{printed}");
    assert!(
        printed.starts_with("This is a dry run: nothing was written."),
        "{printed}"
    );
    assert!(printed.contains("Slugs kept:"), "{printed}");
    assert!(!file(site.path(), "content/en").exists());
    assert!(!file(site.path(), "data").exists());
}

#[test]
fn a_wordpress_import_writes_posts_and_stores_its_redirects() {
    let site = site();
    let export = fixture("wordpress/basic.xml");
    let rules = fixture("wordpress/redirection.json");
    let args = ["import", "wordpress", &export, "--redirection", &rules];
    let (code, printed) = khalid(site.path(), &args);
    assert_eq!(code, 0, "{printed}");
    assert!(!printed.contains("dry run"), "{printed}");
    let post = fs::read_to_string(file(site.path(), POST)).unwrap();
    assert!(post.contains("slug: \"keep-a-starter-alive\""), "{post}");
    let map = fs::read_to_string(file(site.path(), "import/wordpress-redirects.csv")).unwrap();
    // The post is blocked, and its two old paths lie outside /blog/, so
    // they wait in the held file instead of the database.
    let held = fs::read_to_string(file(site.path(), "import/held-redirects.csv")).unwrap();
    assert!(held.contains("/2024/03/keep-a-starter-alive/,"), "{held}");
    let rows = map.lines().count() - held.lines().count();
    assert!(rows > 0, "{map}");
    let noun = if rows == 1 { "redirect" } else { "redirects" };
    let told = format!("Wrote 7 files and stored {rows} {noun}.");
    assert!(printed.starts_with(&told), "{printed}");
    assert!(printed.contains(
        "held in import/held-redirects.csv, because they lie outside the blog's folder: 2."
    ));
    let (code, stored) = khalid(site.path(), &["redirects", "export"]);
    assert_eq!(code, 0, "{stored}");
    assert_eq!(stored.lines().count(), rows + 1, "{stored}\n{map}");
    assert!(!stored.contains("keep-a-starter-alive/,"), "{stored}");

    let (code, again) = khalid(site.path(), &["import", "wordpress", &export]);
    assert_eq!(code, 1, "{again}");
    assert!(again.contains(POST), "{again}");
}

/// The readiness walk's site: the blog moves to a new host, at its root.
#[cfg(test)]
fn site_at_root() -> TempDir {
    let site = site();
    let settings = "name = \"Blog\"\naddress = \"https://blog.example.com/\"\nmount = \"/\"\nmode = \"server\"\ndefault_language = \"en\"\nlanguages = [\"en\"]\n";
    fs::write(file(site.path(), "site.toml"), settings).unwrap();
    site
}

#[test]
fn a_blocked_post_holds_its_old_addresses_until_it_goes_live() {
    let site = site_at_root();
    let export = fixture("wordpress/basic.xml");
    let (code, printed) = khalid(site.path(), &["import", "wordpress", &export]);
    assert_eq!(code, 0, "{printed}");
    assert!(printed.starts_with("Wrote 7 files and stored 1 redirect."));
    assert!(printed.contains(&format!("so readers do not see it yet: {POST}.")));
    assert!(printed.contains("held in their aliases, the list in each post's header: 2."));
    let (_, stored) = khalid(site.path(), &["redirects", "export"]);
    assert!(!stored.contains("keep-a-starter-alive"), "{stored}");
    let post = fs::read_to_string(file(site.path(), POST)).unwrap();
    let aliases = "aliases: [\"/2024/03/keep-a-starter-alive/\", \"/2024/03/keep-a-starter-alive/starter-jar/\"]\n";
    assert!(post.contains(aliases), "{post}");

    let (code, built) = khalid(site.path(), &["build"]);
    assert_eq!(code, 0, "{built}");
    let moves = fs::read_to_string(file(site.path(), "public/_redirects")).unwrap();
    assert!(!moves.contains("keep-a-starter-alive"), "{moves}");
    let fixed = post.replace("every day**.", "every day**. Start at [our home page](/).");
    fs::write(file(site.path(), POST), fixed).unwrap();
    let (code, built) = khalid(site.path(), &["build"]);
    assert_eq!(code, 0, "{built}");
    let moves = fs::read_to_string(file(site.path(), "public/_redirects")).unwrap();
    assert!(
        moves.contains("/2024/03/keep-a-starter-alive/ /keep-a-starter-alive/ 301"),
        "{moves}"
    );
    let asked = ["redirects", "explain", "/2024/03/keep-a-starter-alive/"];
    let (code, explained) = khalid(site.path(), &asked);
    assert_eq!(code, 0, "{explained}");
    assert!(
        explained.ends_with("moves for good (301) to /keep-a-starter-alive/.\n"),
        "{explained}"
    );
}

#[test]
fn an_address_that_keeps_its_path_is_not_stored_as_a_redirect() {
    let site = site_at_root();
    let export = fixture("wordpress/basic.xml");
    let (code, printed) = khalid(site.path(), &["import", "wordpress", &export]);
    assert_eq!(code, 0, "{printed}");
    assert!(
        printed.contains("Old addresses mapped: 5, of which 3 move and 2 stay the same."),
        "{printed}"
    );
    let (_, stored) = khalid(site.path(), &["redirects", "export"]);
    for path in ["/about/,", "/category/baking/,"] {
        assert!(!stored.contains(path), "{stored}");
    }
    assert!(stored.contains("/category/baking/sourdough/,"), "{stored}");
    let (_, explained) = khalid(site.path(), &["redirects", "explain", "/about/"]);
    assert!(!explained.contains("moves for good"), "{explained}");
}

#[test]
fn a_ghost_import_answers_in_json() {
    let site = site();
    let export = fixture("ghost/ghost-5.json");
    let args = [
        "--json",
        "import",
        "ghost",
        &export,
        "--old-site",
        "https://old.example.com",
    ];
    let (code, printed) = khalid(site.path(), &args);
    assert_eq!(code, 0, "{printed}");
    let body: Value = serde_json::from_str(&printed).unwrap();
    assert_eq!(body["dry_run"], false);
    let written: Vec<&str> = body["written"]
        .as_array()
        .unwrap()
        .iter()
        .map(|path| path.as_str().unwrap())
        .collect();
    assert!(written.contains(&"import/ghost-redirects.csv"), "{body}");
    assert!(written.iter().all(|path| file(site.path(), path).is_file()));
}

#[test]
fn a_medium_import_keeps_addresses_on_medium_in_the_file_only() {
    let site = site();
    let posts = file(site.path(), "medium-export/posts");
    fs::create_dir_all(&posts).unwrap();
    let post = file(&posts, "2024-03-12_How-to-bake-rye-bread-3f1a8c4e2b9d.html");
    fs::write(post, MEDIUM_POST).unwrap();
    let export = file(site.path(), "medium-export");
    let (code, printed) = khalid(site.path(), &["import", "medium", export.to_str().unwrap()]);
    assert_eq!(code, 0, "{printed}");
    assert!(printed.contains("stored 0 redirects"), "{printed}");
    let map = fs::read_to_string(file(site.path(), "import/medium-redirects.csv")).unwrap();
    assert!(map.contains(",https://www.example.com/blog/"), "{map}");
    assert!(map.contains("https://medium.com/@mara/how-to-bake-rye-bread-3f1a8c4e2b9d,"));
}

#[test]
fn a_substack_import_moves_old_addresses_on_the_same_host() {
    let site = site();
    let posts = file(site.path(), "substack/posts");
    fs::create_dir_all(&posts).unwrap();
    fs::write(
        file(&posts, "140062712.why-rye.html"),
        "<p>Rye needs time.</p>",
    )
    .unwrap();
    let list = "post_id,post_date,is_published,type,audience,title,subtitle\n\
        140062712.why-rye,2024-03-12T09:30:44.123Z,true,newsletter,only_paid,Why rye,\n";
    fs::write(file(site.path(), "substack/posts.csv"), list).unwrap();
    let export = file(site.path(), "substack");
    let (code, printed) = khalid(
        site.path(),
        &["import", "substack", export.to_str().unwrap()],
    );
    assert_eq!(code, 0, "{printed}");
    // The lone post is blocked, and /p/why-rye lies outside /blog/.
    assert!(
        printed.starts_with("Wrote 2 files and stored 0 redirects."),
        "{printed}"
    );
    let post = fs::read_to_string(file(site.path(), "content/en/posts/2024-03-12-why-rye.md"));
    assert!(post.unwrap().contains("access: \"members\""));
    let held = fs::read_to_string(file(site.path(), "import/held-redirects.csv")).unwrap();
    assert!(held.contains("/p/why-rye,https://www.example.com/blog/why-rye/,301"));
}

#[test]
fn a_jekyll_folder_comes_in_and_skips_built_folders() {
    let site = site();
    for folder in ["jekyll/_posts", "jekyll/_site/2024"] {
        fs::create_dir_all(file(site.path(), folder)).unwrap();
    }
    let post = "---\ntitle: Why rye\n---\n{% include ad.html %}\nRye needs time.\n";
    fs::write(
        file(site.path(), "jekyll/_posts/2024-03-12-why-rye.md"),
        post,
    )
    .unwrap();
    fs::write(file(site.path(), "jekyll/_site/2024/copy.md"), "Built.\n").unwrap();
    let export = file(site.path(), "jekyll");
    let args = ["import", "markdown", export.to_str().unwrap(), "--dry-run"];
    let (code, printed) = khalid(site.path(), &args);
    assert_eq!(code, 0, "{printed}");
    assert!(
        printed.contains("Coming in: 1 post, 0 pages and 0 drafts."),
        "{printed}"
    );
    assert!(
        printed.contains("Left out: include tag (1 file)."),
        "{printed}"
    );
    let (code, printed) = khalid(site.path(), &args[..3]);
    assert_eq!(code, 0, "{printed}");
    assert!(
        printed.starts_with("Wrote 2 files and stored 0 redirects."),
        "{printed}"
    );
    let held = fs::read_to_string(file(site.path(), "import/held-redirects.csv")).unwrap();
    let row = "/2024/03/12/why-rye.html,https://www.example.com/blog/why-rye/,301";
    assert!(held.contains(row), "{held}");
}

#[test]
fn redirects_that_clash_with_stored_ones_are_named() {
    let site = lenient_site();
    let clash = file(site.path(), "clash.csv");
    let row = "/keep-a-starter-alive/,/blog/elsewhere/,301";
    fs::write(&clash, format!("from,to,status\n{row}\n")).unwrap();
    let (code, done) = khalid(
        site.path(),
        &["redirects", "import", clash.to_str().unwrap()],
    );
    assert_eq!(code, 0, "{done}");
    let export = fixture("ghost/ghost-5.json");
    let (code, printed) = khalid(site.path(), &["import", "ghost", &export]);
    assert_eq!(code, 1, "{printed}");
    assert!(printed.contains("were not stored"), "{printed}");
    assert!(printed.contains("khalid redirects import import/ghost-redirects.csv"));
}

#[test]
fn an_export_that_is_not_one_imports_nothing() {
    let site = site();
    let export = fixture("ghost/ghost-5.json");
    let (code, printed) = khalid(site.path(), &["import", "wordpress", &export]);
    assert_eq!(code, 1, "{printed}");
    assert!(printed.contains("Nothing was imported"), "{printed}");
    assert!(!file(site.path(), "content/en").exists());
}

const MEDIUM_POST: &str = r#"<html><body><article class="h-entry"><header><h1 class="p-name">How to bake rye bread</h1></header><section data-field="subtitle" class="p-summary">A wetter dough</section><section data-field="body" class="e-content"><section class="section"><div class="section-inner"><h3 class="graf graf--h3 graf--title">How to bake rye bread</h3><p class="graf graf--p">Rye needs time.</p></div></section></section><footer><p>By <a href="https://medium.com/@mara" class="p-author h-card">Mara Lind</a> on <time class="dt-published" datetime="2024-03-12T09:30:44.123Z">then</time>.</p><p><a href="https://medium.com/@mara/how-to-bake-rye-bread-3f1a8c4e2b9d" class="p-canonical">Canonical link</a></p></footer></article></body></html>"#;
