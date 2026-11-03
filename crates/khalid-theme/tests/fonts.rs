//! The body and heading fonts, and where each font file comes from.

use khalid_theme::{FontHost, FontSource, Typography};

#[cfg(test)]
fn source(family: &str, url: &str, host: FontHost) -> FontSource {
    FontSource::new(family, url, host)
}

#[test]
fn only_text_fonts_from_an_unknown_server_are_held_and_inline_fonts_never_are() {
    let css = [
        "@font-face { font-family: InterVariable; src: url(https://static.gnome.org/Inter.woff2) }",
        "@font-face { font-family: Pythonicon; src: url(https://cdn.example.org/icons.woff) }",
        "@font-face { font-family: Brand; src: url(data:font/woff2;base64,d09GMgAB) }",
        "body { font-family: InterVariable, sans-serif }",
        "h1 { font-family: Brand, serif }",
    ]
    .join("\n");

    let fonts = Typography::from_css(&[&css], &[]);

    let held: Vec<(&str, &str)> = fonts
        .held
        .iter()
        .map(|font| (font.family(), font.server()))
        .collect();
    assert_eq!(held, [("InterVariable", "static.gnome.org")]);
}

#[test]
fn finds_body_and_heading_families_from_rules_and_properties() {
    let fonts = Typography::from_css(
        &[":root { --font-sans: 'Inter', system-ui, sans-serif; }\n\
           body { font-family: var(--font-sans) }\n\
           h1, h2, h3 { font: 700 2rem/1.2 'Playfair Display', Georgia, serif }"],
        &[],
    );

    assert_eq!(
        fonts.body.as_deref(),
        Some(r#""Inter", system-ui, sans-serif"#)
    );
    assert_eq!(
        fonts.heading.as_deref(),
        Some(r#""Playfair Display", Georgia, serif"#)
    );
}

#[test]
fn named_properties_are_used_when_no_rule_says() {
    let fonts = Typography::from_css(
        &[
            ":root { --bs-body-font-family: Lato, sans-serif; --font-heading: Merriweather, serif;\
           --font-mono: Menlo, monospace }",
        ],
        &[],
    );

    assert_eq!(fonts.body.as_deref(), Some("Lato, sans-serif"));
    assert_eq!(fonts.heading.as_deref(), Some("Merriweather, serif"));
}

#[test]
fn lists_where_each_font_comes_from() {
    let fonts = Typography::from_css(
        &[
            "@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;700&family=Source+Serif+4&display=swap');\n\
             @font-face { font-family: \"Acme Sans\"; src: url(/fonts/acme.woff2) format(\"woff2\"), url(/fonts/acme.woff); }",
            "@font-face { font-family: proxima-nova; src: url(\"https://use.typekit.net/af/1a2b3c/00000000/l?subset_id=2\") }",
        ],
        &["https://fonts.googleapis.com/css?family=Roboto+Slab:400,700|Open+Sans"],
    );

    let google = "https://fonts.googleapis.com/css2?family=Inter:wght@400;700&family=Source+Serif+4&display=swap";
    let roboto = "https://fonts.googleapis.com/css?family=Roboto+Slab:400,700|Open+Sans";
    assert_eq!(
        fonts.sources,
        [
            source("Inter", google, FontHost::Google),
            source("Source Serif 4", google, FontHost::Google),
            source("Acme Sans", "/fonts/acme.woff2", FontHost::SelfHosted),
            source(
                "proxima-nova",
                "https://use.typekit.net/af/1a2b3c/00000000/l?subset_id=2",
                FontHost::Adobe
            ),
            source("Roboto Slab", roboto, FontHost::Google),
            source("Open Sans", roboto, FontHost::Google),
        ]
    );
    assert!(FontHost::Google.can_rehost() && FontHost::SelfHosted.can_rehost());
    assert!(!FontHost::Adobe.can_rehost() && !FontHost::Other.can_rehost());
}

#[test]
fn a_site_that_names_no_font_finds_nothing() {
    let fonts = Typography::from_css(&["p { margin: 0 }"], &["https://cdn.example.net/x.css"]);

    assert_eq!(fonts, Typography::default());
}

#[tokio::test]
async fn rehostable_fonts_are_downloaded_and_returned_as_files() {
    use khalid_theme::download as fonts_download;
    use std::net::{IpAddr, Ipv4Addr};

    let server = wiremock::MockServer::start().await;
    let port = server.address().port();
    let loopback = vec![IpAddr::V4(Ipv4Addr::LOCALHOST)];
    let hosts = [
        ("fonts.googleapis.test".to_owned(), loopback.clone()),
        ("fonts.gstatic.test".to_owned(), loopback),
    ];
    let resolver = khalid_outbound::Resolver::for_tests(
        hosts.into(),
        khalid_outbound::Reach::PublicAndLoopback,
    );
    let outbound =
        khalid_outbound::Outbound::with_resolver(khalid_outbound::Limits::default(), resolver)
            .unwrap();

    let css = format!(
        "/* latin */\n@font-face {{ font-family: 'Inter'; font-style: normal; font-weight: 400; src: url(http://fonts.gstatic.test:{port}/inter-latin.woff2) format('woff2'); unicode-range: U+0000-00FF; }}\n"
    );
    wiremock::Mock::given(wiremock::matchers::path("/css2"))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(css))
        .mount(&server)
        .await;
    wiremock::Mock::given(wiremock::matchers::path("/inter-latin.woff2"))
        .respond_with(
            wiremock::ResponseTemplate::new(200).set_body_bytes(b"wOF2 font bytes".as_slice()),
        )
        .mount(&server)
        .await;

    let source = source(
        "Inter",
        &format!("http://fonts.googleapis.test:{port}/css2?family=Inter"),
        FontHost::Google,
    );
    let into = tempfile::tempdir().unwrap();
    let files = fonts_download(&outbound, std::slice::from_ref(&source), into.path())
        .await
        .unwrap();
    assert_eq!(files.len(), 1, "{files:?}");
    assert_eq!(files[0].family, "Inter");
    assert_eq!(files[0].path, "inter-latin-400.woff2");
    assert_eq!(
        files[0].range.as_deref(),
        Some("U+0000-00FF"),
        "the subset the file covers is kept"
    );
    let saved = std::fs::read(within(into.path(), "inter-latin-400.woff2")).unwrap();
    assert_eq!(saved, b"wOF2 font bytes");
}

#[tokio::test]
async fn a_font_that_cannot_be_read_is_skipped_not_fatal() {
    use khalid_theme::download as fonts_download;
    use std::net::{IpAddr, Ipv4Addr};

    let server = wiremock::MockServer::start().await;
    let port = server.address().port();
    let loopback = vec![IpAddr::V4(Ipv4Addr::LOCALHOST)];
    let hosts = [("fonts.googleapis.test".to_owned(), loopback)];
    let resolver = khalid_outbound::Resolver::for_tests(
        hosts.into(),
        khalid_outbound::Reach::PublicAndLoopback,
    );
    let outbound =
        khalid_outbound::Outbound::with_resolver(khalid_outbound::Limits::default(), resolver)
            .unwrap();

    wiremock::Mock::given(wiremock::matchers::path("/css2"))
        .respond_with(wiremock::ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let source = source(
        "Inter",
        &format!("http://fonts.googleapis.test:{port}/css2?family=Inter"),
        FontHost::Google,
    );
    let into = tempfile::tempdir().unwrap();
    let files = fonts_download(&outbound, std::slice::from_ref(&source), into.path())
        .await
        .unwrap();
    assert!(
        files.is_empty(),
        "a failed download leaves the site working"
    );
}

/// `name` inside `dir`.
#[cfg(test)]
fn within(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
    let mut path = dir.to_path_buf();
    path.push(name);
    path
}

#[tokio::test]
async fn a_font_over_the_cap_is_skipped_in_plain_words() {
    use khalid_theme::{MAX_FONT_BYTES, download_and_report as download};
    use std::net::{IpAddr, Ipv4Addr};

    let server = wiremock::MockServer::start().await;
    let port = server.address().port();
    let loopback = vec![IpAddr::V4(Ipv4Addr::LOCALHOST)];
    let hosts = [
        ("fonts.googleapis.com".to_owned(), loopback.clone()),
        ("fonts.gstatic.com".to_owned(), loopback),
    ];
    let resolver = khalid_outbound::Resolver::for_tests(
        hosts.into(),
        khalid_outbound::Reach::PublicAndLoopback,
    );
    let outbound =
        khalid_outbound::Outbound::with_resolver(khalid_outbound::Limits::default(), resolver)
            .unwrap();

    let css = format!(
        "/* latin */\n@font-face {{ font-family: 'Inter'; font-style: normal; font-weight: 400; src: url(http://fonts.gstatic.com:{port}/huge.woff2) format('woff2'); }}\n"
    );
    wiremock::Mock::given(wiremock::matchers::path("/css2"))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(css))
        .mount(&server)
        .await;
    // One byte over the cap: the file is never read further.
    let huge = vec![0u8; MAX_FONT_BYTES + 1];
    wiremock::Mock::given(wiremock::matchers::path("/huge.woff2"))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_bytes(huge))
        .mount(&server)
        .await;

    let source = source(
        "Inter",
        &format!("http://fonts.googleapis.com:{port}/css2?family=Inter"),
        FontHost::Google,
    );
    let into = tempfile::tempdir().unwrap();
    let (saved, skipped) = download(&outbound, std::slice::from_ref(&source), into.path())
        .await
        .unwrap();
    assert!(saved.is_empty(), "an oversized font is never served");
    assert_eq!(skipped.len(), 1, "{skipped:?}");
    assert!(
        skipped[0].contains("over the 4096 KB a font may be"),
        "the reason is plain words: {}",
        skipped[0]
    );
    assert!(
        !within(into.path(), "inter-latin.woff2").exists(),
        "nothing was written"
    );
}

#[tokio::test]
async fn a_stylesheet_that_answers_an_error_is_skipped_in_plain_words() {
    use khalid_theme::download_and_report as download;
    use std::net::{IpAddr, Ipv4Addr};

    let server = wiremock::MockServer::start().await;
    let port = server.address().port();
    let loopback = vec![IpAddr::V4(Ipv4Addr::LOCALHOST)];
    let hosts = [("fonts.googleapis.com".to_owned(), loopback)];
    let resolver = khalid_outbound::Resolver::for_tests(
        hosts.into(),
        khalid_outbound::Reach::PublicAndLoopback,
    );
    let outbound =
        khalid_outbound::Outbound::with_resolver(khalid_outbound::Limits::default(), resolver)
            .unwrap();

    wiremock::Mock::given(wiremock::matchers::path("/css2"))
        .respond_with(wiremock::ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let source = source(
        "Inter",
        &format!("http://fonts.googleapis.com:{port}/css2?family=Inter"),
        FontHost::Google,
    );
    let into = tempfile::tempdir().unwrap();
    let (saved, skipped) = download(&outbound, std::slice::from_ref(&source), into.path())
        .await
        .unwrap();
    assert!(
        saved.is_empty(),
        "a failed stylesheet serves nothing: {saved:?}"
    );
    assert_eq!(skipped.len(), 1, "{skipped:?}");
    assert!(
        skipped[0].contains("answered 500"),
        "the reason names the status: {}",
        skipped[0]
    );
}

#[tokio::test]
async fn a_real_face_is_measured_into_metric_overrides() {
    use khalid_theme::download_and_report;

    let server = wiremock::MockServer::start().await;
    let port = server.address().port();
    let loopback = vec![std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)];
    let hosts = [
        ("fonts.googleapis.test".to_owned(), loopback.clone()),
        ("fonts.gstatic.test".to_owned(), loopback),
    ];
    let resolver = khalid_outbound::Resolver::for_tests(
        hosts.into(),
        khalid_outbound::Reach::PublicAndLoopback,
    );
    let outbound =
        khalid_outbound::Outbound::with_resolver(khalid_outbound::Limits::default(), resolver)
            .unwrap();

    // The DejaVu Sans Bold face from the crate's fixtures, a real font with
    // real tables packed as WOFF2 here, so the measurements are the true
    // ones, not a mock's.
    let face = woff2_of(include_bytes!("../fixtures/real/fonts/DejaVuSans-Bold.ttf"));
    let css = format!(
        "/* latin */\n@font-face {{ font-family: 'DejaVu Sans'; font-style: normal; font-weight: 700; src: url(http://fonts.gstatic.test:{port}/dejavu.woff2) format('woff2'); }}\n"
    );
    wiremock::Mock::given(wiremock::matchers::path("/css2"))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(css))
        .mount(&server)
        .await;
    wiremock::Mock::given(wiremock::matchers::path("/dejavu.woff2"))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_bytes(face.as_slice()))
        .mount(&server)
        .await;

    let source = source(
        "DejaVu Sans",
        &format!("http://fonts.googleapis.test:{port}/css2?family=DejaVu+Sans"),
        FontHost::Google,
    );
    let into = tempfile::tempdir().unwrap();
    let (saved, skipped) =
        download_and_report(&outbound, std::slice::from_ref(&source), into.path())
            .await
            .unwrap();
    assert_eq!(saved.len(), 1, "skipped: {skipped:?}");
    let metrics = saved[0].metrics.as_ref().expect("a real face is measured");
    // DejaVu Sans Bold: 2048 units per em, ascent 1901, descent -483, line
    // gap 0, average character width 1173. Each override is the face's own
    // value over Arial's, as a percentage.
    assert_eq!(metrics.ascent, "102.54%");
    assert_eq!(metrics.descent, "111.29%");
    assert_eq!(metrics.line_gap, "0.00%");
    assert_eq!(metrics.size_adjust, "60.65%");
}

#[tokio::test]
async fn a_file_that_is_not_a_font_is_saved_but_not_measured() {
    use khalid_theme::download_and_report;

    let server = wiremock::MockServer::start().await;
    let port = server.address().port();
    let loopback = vec![std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)];
    let hosts = [
        ("fonts.googleapis.test".to_owned(), loopback.clone()),
        ("fonts.gstatic.test".to_owned(), loopback),
    ];
    let resolver = khalid_outbound::Resolver::for_tests(
        hosts.into(),
        khalid_outbound::Reach::PublicAndLoopback,
    );
    let outbound =
        khalid_outbound::Outbound::with_resolver(khalid_outbound::Limits::default(), resolver)
            .unwrap();

    // A WOFF2 signature on bytes that are no font: the parse fails, the
    // file is still saved and served, but no fallback tuning is claimed.
    let css = format!(
        "/* latin */\n@font-face {{ font-family: 'Not A Font'; font-style: normal; font-weight: 400; src: url(http://fonts.gstatic.test:{port}/broken.woff2) format('woff2'); }}\n"
    );
    wiremock::Mock::given(wiremock::matchers::path("/css2"))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(css))
        .mount(&server)
        .await;
    wiremock::Mock::given(wiremock::matchers::path("/broken.woff2"))
        .respond_with(
            wiremock::ResponseTemplate::new(200)
                .set_body_bytes(b"wOF2 but not a font file".as_slice()),
        )
        .mount(&server)
        .await;

    let source = source(
        "Not A Font",
        &format!("http://fonts.googleapis.test:{port}/css2?family=Not+A+Font"),
        FontHost::Google,
    );
    let into = tempfile::tempdir().unwrap();
    let (saved, skipped) =
        download_and_report(&outbound, std::slice::from_ref(&source), into.path())
            .await
            .unwrap();
    assert_eq!(saved.len(), 1, "skipped: {skipped:?}");
    assert!(
        saved[0].metrics.is_none(),
        "a file that does not parse as a face is never tuned"
    );
    assert!(
        within(into.path(), "not-a-font-latin-400.woff2").exists(),
        "the file itself is still saved and served"
    );
}

/// `ttf` packed as a WOFF2 file with no table transforms: each table's
/// tag given in full, the glyph and glyph location tables marked as kept
/// as they are, and the tables one Brotli stream.
#[cfg(test)]
fn woff2_of(ttf: &[u8]) -> Vec<u8> {
    let be32 = |at: usize| u32::from_be_bytes(ttf[at..at + 4].try_into().unwrap());
    let count = u16::from_be_bytes([ttf[4], ttf[5]]);
    let (mut directory, mut tables) = (Vec::new(), Vec::new());
    for index in 0..usize::from(count) {
        let entry = 12 + 16 * index;
        let tag = &ttf[entry..entry + 4];
        let offset = usize::try_from(be32(entry + 8)).unwrap();
        let length = be32(entry + 12);
        let glyph_locations = [0x6c, 0x6f, 0x63, 0x61];
        let kept_as_is = if tag == b"glyf" || tag == glyph_locations {
            0xc0
        } else {
            0
        };
        directory.push(0x3f | kept_as_is);
        directory.extend_from_slice(tag);
        directory.extend(base128(length));
        tables.extend_from_slice(&ttf[offset..offset + usize::try_from(length).unwrap()]);
    }
    let mut stream = Vec::new();
    let params = brotli::enc::BrotliEncoderParams::default();
    brotli::BrotliCompress(&mut tables.as_slice(), &mut stream, &params).unwrap();
    let size = |n: usize| u32::try_from(n).unwrap().to_be_bytes();
    let mut file = b"wOF2".to_vec();
    file.extend(0x0001_0000_u32.to_be_bytes());
    file.extend(size(48 + directory.len() + stream.len()));
    file.extend(count.to_be_bytes());
    file.extend([0, 0]);
    file.extend(size(ttf.len()));
    file.extend(size(stream.len()));
    // Version 1.0, then no metadata block and no private block.
    file.extend([0, 1, 0, 0]);
    file.extend([0; 20]);
    file.extend(directory);
    file.extend(stream);
    file
}

/// `value` as WOFF2's `UIntBase128`: seven bits a byte, high bits first.
#[cfg(test)]
fn base128(value: u32) -> Vec<u8> {
    let mut bytes = vec![u8::try_from(value & 0x7f).unwrap()];
    let mut rest = value >> 7;
    while rest > 0 {
        bytes.insert(0, u8::try_from(rest & 0x7f).unwrap() | 0x80);
        rest >>= 7;
    }
    bytes
}
