//! `khalid import --download-images` copies each post's images and its
//! feature image from the old site into the post's own folder, through the
//! one outbound client and its private-network guard (RULES.md BL-RUN-8).
//! A local `wiremock` stand-in plays the old site, so nothing is fetched
//! from the internet (RULES.md BL-TST-3).

use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;
use tempfile::TempDir;
use wiremock::matchers::path;
use wiremock::{Mock, MockServer, ResponseTemplate};

/// The host name the binary resolves to the stand-in, through the test hook.
const HOST: &str = "bakery.test";

/// The jar every post shows, at its path on the old site.
const JAR: &str = "/content/images/2024/03/starter-jar.jpg";

/// `name` inside `dir`.
#[cfg(test)]
fn within(dir: &Path, name: &str) -> PathBuf {
    let mut path = dir.to_path_buf();
    path.push(name);
    path
}

/// A site with the demo's settings and no content yet.
#[cfg(test)]
fn site() -> TempDir {
    let settings = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/demo-site/site.toml"
    );
    let dir = tempfile::tempdir().unwrap();
    fs::copy(settings, within(dir.path(), "site.toml")).unwrap();
    fs::create_dir(within(dir.path(), "content")).unwrap();
    dir
}

/// A small PNG, made here rather than taken from anywhere.
#[cfg(test)]
fn png() -> Vec<u8> {
    let image = image::RgbImage::from_pixel(4, 3, image::Rgb([200, 120, 40]));
    let mut bytes = Cursor::new(Vec::new());
    image.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
    bytes.into_inner()
}

/// A stand-in old site that answers the jar once, with a PNG, and every
/// other address with 404. Returns it with its address.
#[cfg(test)]
async fn old_site(jar_fetches: u64) -> (MockServer, String) {
    let server = MockServer::start().await;
    Mock::given(path(JAR))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(png()))
        .expect(jar_fetches)
        .mount(&server)
        .await;
    let address = format!("http://{HOST}:{}", server.address().port());
    (server, address)
}

/// The importer's Ghost sample, with images in the post's and the page's
/// bodies: the jar in both, and one the old site no longer has.
#[cfg(test)]
fn ghost_export(site: &Path) -> PathBuf {
    let sample = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../khalid-import/fixtures/ghost/ghost-5.json"
    );
    let mut export: Value = serde_json::from_str(&fs::read_to_string(sample).unwrap()).unwrap();
    let posts = &mut export["db"][0]["data"]["posts"];
    let jar = format!(r#"<img src="__GHOST_URL__{JAR}" alt="A jar of starter">"#);
    posts[0]["lexical"] = Value::Null;
    posts[0]["html"] = Value::from(format!(
        r#"<p>Feed it every day.</p>{jar}<p>The old loaf:</p><img src="__GHOST_URL__/content/images/gone.jpg" alt="A loaf">"#
    ));
    posts[1]["mobiledoc"] = Value::Null;
    posts[1]["html"] = Value::from(format!("<p>We bake rye.</p>{jar}"));
    let file = within(site, "ghost.json");
    fs::write(&file, export.to_string()).unwrap();
    file
}

/// How the binary finds hosts: the stand-in through the test hook, or the
/// real resolver with its private-network guard.
#[derive(Clone, Copy)]
enum Names {
    StandIn,
    Real,
}

/// Runs `khalid --site <site>` with `args`, finding hosts by `names`.
#[cfg(test)]
async fn khalid(site: &Path, args: Vec<String>, names: Names) -> (i32, String) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_khalid"));
    command.arg("--site").arg(site).args(args);
    if let Names::StandIn = names {
        command.arg("--test-hosts").arg(format!("{HOST}=127.0.0.1"));
    }
    let output = tokio::task::spawn_blocking(move || command.output().unwrap())
        .await
        .unwrap();
    let mut printed = String::from_utf8(output.stdout).unwrap();
    printed.push_str(&String::from_utf8(output.stderr).unwrap());
    (output.status.code().unwrap(), printed)
}

/// `khalid import ghost` of [`ghost_export`] from `old`, with `more`.
#[cfg(test)]
fn ghost_args(export: &Path, old: &str, more: &[&str]) -> Vec<String> {
    let mut args = vec![
        "import".to_owned(),
        "ghost".to_owned(),
        export.display().to_string(),
        "--old-site".to_owned(),
        old.to_owned(),
        "--download-images".to_owned(),
    ];
    args.extend(more.iter().map(|arg| (*arg).to_owned()));
    args
}

#[tokio::test(flavor = "multi_thread")]
async fn a_ghost_import_brings_each_image_into_its_post_folder_once() {
    let (_server, old) = old_site(1).await;
    let site = site();
    let export = ghost_export(site.path());
    let (code, printed) = khalid(site.path(), ghost_args(&export, &old, &[]), Names::StandIn).await;
    assert_eq!(code, 0, "{printed}");
    assert!(
        printed.contains("Downloaded 1 image from the old site"),
        "{printed}"
    );
    let gone = format!("{old}/content/images/gone.jpg (it answered 404)");
    assert!(printed.contains(&gone), "{printed}");
    // The feature image is the post's lead image now, so no longer held back.
    assert!(!printed.contains("feature image"), "{printed}");

    let folder = within(
        site.path(),
        "content/en/posts/2024-03-12-keep-a-starter-alive",
    );
    assert!(
        !within(
            site.path(),
            "content/en/posts/2024-03-12-keep-a-starter-alive.md"
        )
        .exists()
    );
    let post = fs::read_to_string(within(&folder, "index.md")).unwrap();
    assert_eq!(fs::read(within(&folder, "starter-jar.png")).unwrap(), png());
    assert!(
        post.contains("![A jar of starter](starter-jar.png)"),
        "{post}"
    );
    assert!(
        post.contains(&format!("({old}/content/images/gone.jpg)")),
        "{post}"
    );
    let lead = "image:\n  src: \"starter-jar.png\"\n  alt: \"How to keep a starter alive\"\n";
    assert!(post.contains(lead), "{post}");
    assert!(!post.contains(JAR), "{post}");

    let page = fs::read_to_string(within(site.path(), "content/en/pages/about/index.md")).unwrap();
    assert!(page.contains("](starter-jar.png)"), "{page}");
    let jar = within(site.path(), "content/en/pages/about/starter-jar.png");
    assert_eq!(fs::read(jar).unwrap(), png());
}

#[tokio::test(flavor = "multi_thread")]
async fn a_dry_run_counts_the_images_and_downloads_none() {
    let (_server, old) = old_site(0).await;
    let site = site();
    let export = ghost_export(site.path());
    let args = ghost_args(&export, &old, &["--dry-run"]);
    let (code, printed) = khalid(site.path(), args, Names::StandIn).await;
    assert_eq!(code, 0, "{printed}");
    assert!(
        printed.contains("With --download-images, the import would download 2 images."),
        "{printed}"
    );
    assert!(!within(site.path(), "content/en").exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn an_image_at_a_private_address_is_refused_and_keeps_its_link() {
    let (server, _) = old_site(0).await;
    let site = site();
    let export = ghost_export(site.path());
    let private = format!("http://127.0.0.1:{}", server.address().port());
    let (code, printed) =
        khalid(site.path(), ghost_args(&export, &private, &[]), Names::Real).await;
    assert_eq!(code, 0, "{printed}");
    assert!(
        printed.contains("which is not a public address"),
        "{printed}"
    );
    let post = within(
        site.path(),
        "content/en/posts/2024-03-12-keep-a-starter-alive.md",
    );
    let post = fs::read_to_string(post).unwrap();
    assert!(post.contains(&format!("({private}{JAR})")), "{post}");
    assert!(!post.contains("image:"), "{post}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_wordpress_import_brings_in_its_feature_image() {
    let server = MockServer::start().await;
    let jar = "/wp-content/uploads/2024/03/starter-jar.jpg";
    Mock::given(path(jar))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(png()))
        .expect(1)
        .mount(&server)
        .await;
    let old = format!("http://{HOST}:{}", server.address().port());
    let sample = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../khalid-import/fixtures/wordpress/basic.xml"
    );
    let site = site();
    let export = within(site.path(), "export.xml");
    let text = fs::read_to_string(sample).unwrap();
    fs::write(
        &export,
        text.replace("https://northfield.example.com", &old),
    )
    .unwrap();
    let args = [
        "import",
        "wordpress",
        export.to_str().unwrap(),
        "--download-images",
    ];
    let args = args.iter().map(|arg| (*arg).to_owned()).collect();
    let (code, printed) = khalid(site.path(), args, Names::StandIn).await;
    assert_eq!(code, 0, "{printed}");
    let folder = within(
        site.path(),
        "content/en/posts/2024-03-12-keep-a-starter-alive",
    );
    let post = fs::read_to_string(within(&folder, "index.md")).unwrap();
    assert!(
        post.contains("image:\n  src: \"starter-jar.png\"\n"),
        "{post}"
    );
    assert!(within(&folder, "starter-jar.png").is_file());
}
