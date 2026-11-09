//! What `theme sync` keeps of a Google Fonts stylesheet: it asks as a
//! browser, so it gets WOFF2 files split by subset, keeps the Latin one,
//! saves a variable file once for every weight it carries, and saves
//! nothing that is not a WOFF2 file (G20).

use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};

use khalid_outbound::{Limits, Outbound, Reach, Resolver};
use khalid_theme::{FontHost, FontSource, download_and_report};
use wiremock::matchers::{header_regex, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// A WOFF2 file's first bytes, then filler.
const WOFF2: &[u8] = b"wOF2 stand-in face";

/// The Google Fonts answer to a modern browser for Inter at 400 and 700:
/// one variable file per subset, named once per weight.
const BROWSER_CSS: &str = "/* cyrillic */\n\
@font-face { font-family: 'Inter'; font-style: normal; font-weight: 400; src: url(http://fonts.gstatic.test:PORT/inter-cyrillic.woff2) format('woff2'); unicode-range: U+0301, U+0400-045F; }\n\
/* latin */\n\
@font-face { font-family: 'Inter'; font-style: normal; font-weight: 400; src: url(http://fonts.gstatic.test:PORT/inter-latin.woff2) format('woff2'); unicode-range: U+0000-00FF, U+2000-206F; }\n\
/* cyrillic */\n\
@font-face { font-family: 'Inter'; font-style: normal; font-weight: 700; src: url(http://fonts.gstatic.test:PORT/inter-cyrillic.woff2) format('woff2'); unicode-range: U+0301, U+0400-045F; }\n\
/* latin */\n\
@font-face { font-family: 'Inter'; font-style: normal; font-weight: 700; src: url(http://fonts.gstatic.test:PORT/inter-latin.woff2) format('woff2'); unicode-range: U+0000-00FF, U+2000-206F; }\n";

/// The answer to a client with no user agent: one whole TrueType file per
/// weight, with no subset and no range.
const BARE_CSS: &str = "@font-face { font-family: 'Inter'; font-style: normal; font-weight: 400; src: url(http://fonts.gstatic.test:PORT/inter-400.ttf) format('truetype'); }\n\
@font-face { font-family: 'Inter'; font-style: normal; font-weight: 700; src: url(http://fonts.gstatic.test:PORT/inter-700.ttf) format('truetype'); }\n";

#[cfg(test)]
fn within(dir: &Path, name: &str) -> PathBuf {
    let mut path = dir.to_path_buf();
    path.push(name);
    path
}

/// A client that reaches the stand-in Google hosts on loopback.
#[cfg(test)]
fn outbound() -> Outbound {
    let loopback = vec![IpAddr::V4(Ipv4Addr::LOCALHOST)];
    let hosts = [
        ("fonts.googleapis.test".to_owned(), loopback.clone()),
        ("fonts.gstatic.test".to_owned(), loopback),
    ];
    let resolver = Resolver::for_tests(hosts.into(), Reach::PublicAndLoopback);
    Outbound::with_resolver(Limits::default(), resolver).unwrap()
}

/// A stand-in Google Fonts that answers a browser with `browser_css` and
/// anyone else with [`BARE_CSS`], and serves `files`.
#[cfg(test)]
async fn google(browser_css: &str, files: &[(&str, &[u8])]) -> (MockServer, FontSource) {
    let server = MockServer::start().await;
    let port = server.address().port().to_string();
    Mock::given(path("/css2"))
        .and(header_regex("user-agent", "Chrome/"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(browser_css.replace("PORT", &port)),
        )
        .with_priority(1)
        .mount(&server)
        .await;
    Mock::given(path("/css2"))
        .respond_with(ResponseTemplate::new(200).set_body_string(BARE_CSS.replace("PORT", &port)))
        .mount(&server)
        .await;
    for (name, bytes) in files {
        Mock::given(path(format!("/{name}")))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(bytes.to_vec()))
            .mount(&server)
            .await;
    }
    let url = format!("http://fonts.googleapis.test:{port}/css2?family=Inter:wght@400;700");
    (server, FontSource::new("Inter", &url, FontHost::Google))
}

#[tokio::test]
async fn a_variable_font_is_saved_once_as_its_latin_subset_for_every_weight() {
    let files: [(&str, &[u8]); 4] = [
        ("inter-latin.woff2", WOFF2),
        ("inter-cyrillic.woff2", WOFF2),
        ("inter-400.ttf", b"whole truetype file"),
        ("inter-700.ttf", b"whole truetype file"),
    ];
    let (_server, source) = google(BROWSER_CSS, &files).await;
    let into = tempfile::tempdir().unwrap();

    let (saved, skipped) = download_and_report(&outbound(), &[source], into.path())
        .await
        .unwrap();

    assert!(skipped.is_empty(), "{skipped:?}");
    assert_eq!(saved.len(), 1, "one file carries both weights: {saved:?}");
    let face = &saved[0];
    assert_eq!(face.path, "inter-latin-400-700.woff2");
    assert_eq!(face.weight, "400 700", "body and heading weights");
    assert_eq!(face.range.as_deref(), Some("U+0000-00FF, U+2000-206F"));
    assert_eq!(fs_names(into.path()), ["inter-latin-400-700.woff2"]);
}

#[tokio::test]
async fn static_weights_get_a_latin_file_each_under_their_own_names() {
    let css = BROWSER_CSS.replace(
        "700; src: url(http://fonts.gstatic.test:PORT/inter-latin",
        "700; src: url(http://fonts.gstatic.test:PORT/inter-bold-latin",
    );
    let files: [(&str, &[u8]); 2] = [
        ("inter-latin.woff2", b"wOF2 regular"),
        ("inter-bold-latin.woff2", b"wOF2 bold"),
    ];
    let (_server, source) = google(&css, &files).await;
    let into = tempfile::tempdir().unwrap();

    let (saved, skipped) = download_and_report(&outbound(), &[source], into.path())
        .await
        .unwrap();

    assert!(skipped.is_empty(), "{skipped:?}");
    let names: Vec<(&str, &str)> = saved
        .iter()
        .map(|face| (face.path.as_str(), face.weight.as_str()))
        .collect();
    assert_eq!(
        names,
        [
            ("inter-latin-400.woff2", "400"),
            ("inter-latin-700.woff2", "700")
        ]
    );
    let bold = std::fs::read(within(into.path(), "inter-latin-700.woff2")).unwrap();
    assert_eq!(bold, b"wOF2 bold", "no weight overwrites another");
}

#[tokio::test]
async fn a_file_that_is_not_woff2_is_skipped_in_plain_words() {
    // A stylesheet that offers only TrueType files, as Google does for a
    // client it does not know.
    let files: [(&str, &[u8]); 2] = [
        ("inter-400.ttf", b"whole truetype file"),
        ("inter-700.ttf", b"whole truetype file"),
    ];
    let (_server, source) = google(BARE_CSS, &files).await;
    let into = tempfile::tempdir().unwrap();

    let (saved, skipped) = download_and_report(&outbound(), &[source], into.path())
        .await
        .unwrap();

    assert!(saved.is_empty(), "{saved:?}");
    assert_eq!(skipped.len(), 2, "{skipped:?}");
    assert!(
        skipped[0].contains("Inter 400 is not a WOFF2 web font"),
        "{}",
        skipped[0]
    );
    assert!(fs_names(into.path()).is_empty(), "nothing was written");
}

/// The names of the files in `dir`, sorted.
#[cfg(test)]
fn fs_names(dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}
