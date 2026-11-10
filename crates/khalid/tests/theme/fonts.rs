//! Fonts in `khalid theme sync`: a font the blog may rehost is downloaded
//! beside the tokens, listed in them, and served by the build.

use std::fs;
use std::path::Path;
use std::process::Command;

use wiremock::matchers::path;
use wiremock::{Mock, MockServer, ResponseTemplate};

use super::{HOME, HOST, Names, STYLE, khalid, site, text, within};

#[tokio::test(flavor = "multi_thread")]
async fn sync_downloads_rehostable_fonts_and_the_build_serves_them() {
    let site = site();
    // A tokens file with no font files is there already, so this sync is
    // a resync that must still list the font files.
    tokens_without_fonts(site.path()).await;
    // The home links a stylesheet that names a Google font, and the font's
    // CSS and file are on the same stand-in server.
    let server = MockServer::start().await;
    let port = server.address().port();
    let style = format!(
        "body {{ background: #ffffff; color: #1f2328; font-family: Inter, sans-serif; }}\n\
         @import url(\"http://fonts.googleapis.com:{port}/css2?family=Inter\");\n"
    );
    Mock::given(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(HOME))
        .mount(&server)
        .await;
    Mock::given(path("/style.css"))
        .respond_with(ResponseTemplate::new(200).set_body_string(style))
        .mount(&server)
        .await;
    google_fonts(&server).await;

    let home = format!("http://{HOST}:{port}/");
    let mut command = Command::new(env!("CARGO_BIN_EXE_khalid"));
    command
        .arg("--site")
        .arg(site.path())
        .args(["theme", "sync", &home])
        .arg("--test-hosts")
        .arg(format!(
            "{HOST}=127.0.0.1,fonts.googleapis.com=127.0.0.1,fonts.gstatic.com=127.0.0.1"
        ));
    let output = command.output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    fonts_saved_listed_and_built(site.path());
}

/// Answers on `server` like Google Fonts: a browser gets WOFF2 split by
/// subset, anyone else a whole TrueType file.
#[cfg(test)]
async fn google_fonts(server: &MockServer) {
    let port = server.address().port();
    let css = "/* cyrillic */\n@font-face { font-family: 'Inter'; font-style: normal; font-weight: 400; src: url(http://fonts.gstatic.com:{PORT}/inter-cyrillic.woff2) format('woff2'); unicode-range: U+0400-045F; }\n/* latin */\n@font-face { font-family: 'Inter'; font-style: normal; font-weight: 400; src: url(http://fonts.gstatic.com:{PORT}/inter-latin.woff2) format('woff2'); unicode-range: U+0000-00FF; }\n";
    let css = css.replace("{PORT}", &port.to_string());
    Mock::given(path("/css2"))
        .and(wiremock::matchers::header_regex("user-agent", "Chrome/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(css))
        .with_priority(1)
        .mount(server)
        .await;
    let bare = "@font-face { font-family: 'Inter'; font-style: normal; font-weight: 400; src: url(http://fonts.gstatic.com:{PORT}/inter.ttf) format('truetype'); }\n";
    Mock::given(path("/css2"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(bare.replace("{PORT}", &port.to_string())),
        )
        .mount(server)
        .await;
    for file in ["inter-latin.woff2", "inter-cyrillic.woff2"] {
        Mock::given(path(format!("/{file}")))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"wOF2 font bytes".as_slice()))
            .mount(server)
            .await;
    }
}

/// Checks that the sync in `site` saved the Latin file alone, listed it in
/// the tokens, and that a build serves it where the stylesheet says.
#[cfg(test)]
fn fonts_saved_listed_and_built(site: &Path) {
    let saved = within(site, "theme/fonts/inter-latin-400.woff2");
    assert!(saved.is_file(), "the Latin file is saved beside the tokens");
    assert!(
        !within(site, "theme/fonts/inter-cyrillic-400.woff2").exists(),
        "other subsets are left out"
    );
    let tokens = fs::read_to_string(within(site, "theme/tokens.toml")).unwrap();
    assert!(
        tokens.contains("[[fonts.files]]"),
        "the tokens name the file:\n{tokens}"
    );
    assert!(
        tokens.contains("path = \"inter-latin-400.woff2\"")
            && tokens.contains("range = \"U+0000-00FF\""),
        "the token names the file and its subset:\n{tokens}"
    );

    // And a build serves it where the stylesheet says it is.
    let built_site = tempfile::tempdir().unwrap();
    let mut build = Command::new(env!("CARGO_BIN_EXE_khalid"));
    build
        .arg("--site")
        .arg(site)
        .args(["build", "--out"])
        .arg(built_site.path());
    let finished = build.output().unwrap();
    assert_eq!(
        finished.status.code(),
        Some(0),
        "{}",
        text(&finished.stderr)
    );
    let font_path = within(built_site.path(), "assets/fonts/inter-latin-400.woff2");
    assert_eq!(
        fs::read(&font_path).unwrap(),
        b"wOF2 font bytes",
        "the built site serves the font file"
    );
}

/// Writes `theme/tokens.toml` in `site` from CSS alone, so it lists no
/// font file.
#[cfg(test)]
async fn tokens_without_fonts(site: &Path) {
    let first = within(site, "first.css");
    fs::write(&first, STYLE).unwrap();
    let args = ["theme", "sync", "https://www.example.com/", "--from-css"];
    let made = khalid(
        site,
        &[&args[..], &[first.to_str().unwrap()]].concat(),
        Names::Real,
    )
    .await;
    assert_eq!(made.status.code(), Some(0), "{}", text(&made.stderr));
}
