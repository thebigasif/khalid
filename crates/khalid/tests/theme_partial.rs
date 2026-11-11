//! `khalid theme sync` and `khalid theme check` over the short
//! `theme/tokens.toml` the Design screen writes: the values a person set,
//! with the template and the light default under them (backlog row 13).
//! Nothing is fetched from the internet.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use wiremock::matchers::path;
use wiremock::{Mock, MockServer, ResponseTemplate};

/// What the Design screen writes after a person picks a link colour and
/// locks the accent, with a comment of their own.
const PARTIAL: &str = "locked = [\"colors.accent\"]\n\n[colors]\n\
    link = \"#b3261e\" # brand red\naccent = \"#6b21a8\"\n";

/// The main site's colours.
const STYLE: &str = "body { background: #fdfcfa; color: #1e2a3a; }\na { color: #2255aa; }\n";

/// The host name the binary resolves to the stand-in, through the test hook.
const HOST: &str = "bakery.test";

/// The demo site's settings and [`PARTIAL`] in a fresh folder.
#[cfg(test)]
fn site() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let settings = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/demo-site/site.toml"
    );
    fs::copy(settings, within(dir.path(), "site.toml")).unwrap();
    fs::create_dir(within(dir.path(), "content")).unwrap();
    fs::create_dir(within(dir.path(), "theme")).unwrap();
    fs::write(within(dir.path(), "theme/tokens.toml"), PARTIAL).unwrap();
    dir
}

/// `name` inside `dir`.
#[cfg(test)]
fn within(dir: &Path, name: &str) -> PathBuf {
    let mut path = dir.to_path_buf();
    path.push(name);
    path
}

/// Runs the binary on `site` with `args`, off the async thread so a
/// stand-in keeps serving, with [`HOST`] answering on loopback.
#[cfg(test)]
async fn khalid(site: &Path, args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_khalid"));
    command.arg("--site").arg(site).args(args);
    command.arg("--test-hosts").arg(format!("{HOST}=127.0.0.1"));
    tokio::task::spawn_blocking(move || command.output().unwrap())
        .await
        .unwrap()
}

#[cfg(test)]
fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[tokio::test(flavor = "multi_thread")]
async fn sync_over_a_design_screen_file_changes_it_in_place() {
    let site = site();
    let css = within(site.path(), "main.css");
    fs::write(&css, STYLE).unwrap();
    let css = css.display().to_string();
    let args = [
        "theme",
        "sync",
        "https://northwind.invalid/",
        "--from-css",
        &css,
    ];

    let output = khalid(site.path(), &args).await;

    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let said = text(&output.stdout);
    assert!(
        said.contains("colors.link changed from #b3261e to #2255aa."),
        "{said}"
    );
    assert!(
        said.contains("colors.accent stays #6b21a8, because it is locked."),
        "{said}"
    );
    let tokens = fs::read_to_string(within(site.path(), "theme/tokens.toml")).unwrap();
    assert!(
        tokens.starts_with("locked = [\"colors.accent\"]"),
        "{tokens}"
    );
    assert!(
        tokens.contains("link = \"#2255aa\" # brand red"),
        "{tokens}"
    );
    assert!(tokens.contains("background = \"#fdfcfa\""), "{tokens}");
    assert!(
        !tokens.contains("[fonts]"),
        "fonts the site shares stay out: {tokens}"
    );
    assert!(within(site.path(), "data/theme-preview/index.html").exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn check_reads_a_design_screen_file_and_reports_drift() {
    let site = site();
    let server = MockServer::start().await;
    let home = "<!doctype html><html><head><title>Bakery</title>\
        <link rel=\"stylesheet\" href=\"/style.css\"></head><body><h1>Bread</h1></body></html>";
    Mock::given(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(home))
        .mount(&server)
        .await;
    Mock::given(path("/style.css"))
        .respond_with(ResponseTemplate::new(200).set_body_string(STYLE))
        .mount(&server)
        .await;
    let url = format!("http://{HOST}:{}/", server.address().port());

    let output = khalid(site.path(), &["theme", "check", &url]).await;

    assert_eq!(output.status.code(), Some(1), "{}", text(&output.stderr));
    let said = text(&output.stdout);
    assert!(
        said.contains("colors.link is #b3261e here, and the main site now uses #2255aa."),
        "{said}"
    );
    let tokens = fs::read_to_string(within(site.path(), "theme/tokens.toml")).unwrap();
    assert_eq!(tokens, PARTIAL, "check writes nothing");
}
