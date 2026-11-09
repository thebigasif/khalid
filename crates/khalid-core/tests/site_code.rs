//! `[code]` in `site.toml`: the site's own head and body code, each entry
//! tagged with the consent category it waits for (privacy plan U2, KTD2,
//! KTD12). The old single table still reads.

use std::path::Path;

use khalid_core::{
    AbsoluteUrl, BodyCode, BodyCodeError, CodeCategory, Config, Environment, HeadCodeError,
};

const SITE: &str = r#"name = "Field Notes on SEO"
address = "https://www.example.com"
default_language = "en"
languages = ["en"]
"#;

const CODE: &str = r#"
[code]
head = """
<script async src="https://www.googletagmanager.com/gtag/js?id=G-1"></script>
<script>gtag('config', 'G-1');</script>"""
body = "<script src='https://chat.example.com/widget.js' defer></script>"
"#;

#[cfg(test)]
fn load(text: &str) -> Result<Config, String> {
    Config::from_toml(Path::new("site.toml"), text, &Environment::empty())
        .map_err(|error| error.to_string())
}

#[test]
fn a_site_without_the_table_has_no_code() {
    let config = load(SITE).unwrap();
    let code = config.code();
    assert!(code.is_empty());
    assert_eq!(code.entries, []);
    assert!(!code.waits_for_consent());
}

#[test]
fn reads_head_and_body_code_that_waits_for_consent_by_default() {
    let config = load(&format!("{SITE}{CODE}")).unwrap();
    let code = config.code();
    assert!(!code.is_empty());
    assert!(code.waits_for_consent());
    let [entry] = code.entries.as_slice() else {
        panic!("one entry, not {:?}", code.entries);
    };
    assert_eq!(entry.category, CodeCategory::Marketing);
    assert_eq!(entry.vendor, None);
    let head = entry.head.as_ref().unwrap();
    assert_eq!(
        head.to_html("n"),
        "<script async src=\"https://www.googletagmanager.com/gtag/js?id=G-1\" nonce=\"n\"></script>\n<script nonce=\"n\">gtag('config', 'G-1');</script>"
    );
    let body = entry.body.as_ref().unwrap();
    assert_eq!(
        body.to_html("n"),
        "<script src=\"https://chat.example.com/widget.js\" defer nonce=\"n\"></script>"
    );
}

#[test]
fn code_may_run_without_asking() {
    let text = format!("{SITE}{CODE}consent = \"not-needed\"\n");
    let config = load(&text).unwrap();
    let [entry] = config.code().entries.as_slice() else {
        panic!("one entry");
    };
    assert_eq!(entry.category, CodeCategory::NoTracking);
    assert!(entry.head.is_some() && entry.body.is_some());
    assert!(!config.code().waits_for_consent());
}

#[test]
fn a_table_of_comments_only_is_empty_and_asks_nothing() {
    let config = load(&format!("{SITE}[code]\nhead = \"<!-- later -->\"\n")).unwrap();
    assert!(config.code().is_empty());
    assert!(!config.code().waits_for_consent());
}

#[test]
fn body_code_holds_scripts_only() {
    for (html, name) in [
        ("<style>p{color:red}</style>", "style"),
        ("<meta name=a content=b>", "meta"),
        ("<div>hi</div>", "div"),
    ] {
        assert_eq!(
            BodyCode::try_new(html),
            Err(BodyCodeError::Element {
                name: name.to_owned()
            }),
            "{html}"
        );
    }
    let open = BodyCode::try_new("<script>go()").unwrap_err();
    assert_eq!(
        open,
        BodyCodeError::Code(HeadCodeError::Unclosed { name: "script" })
    );
    assert_eq!(
        open.to_string(),
        "body code has a script that is never closed"
    );
    let error = load(&format!("{SITE}[code]\nbody = \"<div>hi</div>\"\n")).unwrap_err();
    assert!(
        error.contains("field \"body\"") && error.contains("body code may hold only script"),
        "{error}"
    );
}

#[test]
fn a_static_site_asks_for_consent_with_its_script() {
    let text = format!("{SITE}mode = \"static\"\n{CODE}");
    assert!(load(&text).unwrap().code().waits_for_consent());
    let text = format!("{text}consent = \"not-needed\"\n");
    assert!(!load(&text).unwrap().code().waits_for_consent());
}

#[test]
fn held_code_is_plain_text_and_inert_templates_until_a_reader_agrees() {
    let head = khalid_core::HeadCode::try_new(
        "<script type=\"module\" src=\"/m.js\"></script><style>p{}</style><meta name=\"x\" content=\"y\">",
    )
    .unwrap();
    let expected = [
        "<script type=\"text/plain\" data-khalid-consent=\"analytics\" data-type=\"module\" src=\"/m.js\"></script>",
        "<template data-khalid-consent=\"analytics\"><style>p{}</style></template>",
        "<template data-khalid-consent=\"analytics\"><meta name=\"x\" content=\"y\"></template>",
    ];
    assert_eq!(head.to_held_html("analytics"), expected.join("\n"));
    let body = BodyCode::try_new("<script>chat()</script>").unwrap();
    assert_eq!(
        body.to_held_html("marketing"),
        "<script type=\"text/plain\" data-khalid-consent=\"marketing\">chat()</script>"
    );
}

#[test]
fn refuses_an_unknown_consent() {
    let error = load(&format!("{SITE}[code]\nconsent = \"maybe\"\n")).unwrap_err();
    assert!(error.contains("field \"consent\""), "{error}");
}

const ENTRIES: &str = r#"
[[code]]
head = "<script src='https://stats.example.net/a.js'></script>"
category = "analytics"
vendor = "Example Stats"
policy = "https://stats.example.net/privacy"

[[code]]
body = "<script src='https://ads.example.org/t.js'></script>"

[[code]]
head = "<script src='/counter.js'></script>"
category = "no-tracking"
vendor = "Own counter"
"#;

const QUIET: &str = r#"
[[code]]
head = "<script src='/counter.js'></script>"
category = "no-tracking"
"#;

#[test]
fn reads_a_list_of_tagged_entries_and_an_untagged_one_is_marketing() {
    let config = load(&format!("{SITE}{ENTRIES}")).unwrap();
    let code = config.code();
    let categories: Vec<CodeCategory> = code.entries.iter().map(|e| e.category).collect();
    assert_eq!(
        categories,
        [
            CodeCategory::Analytics,
            CodeCategory::Marketing,
            CodeCategory::NoTracking
        ]
    );
    let first = code.entries.first().unwrap();
    assert_eq!(first.vendor.as_deref(), Some("Example Stats"));
    assert_eq!(
        first.policy.as_ref().map(AbsoluteUrl::as_str),
        Some("https://stats.example.net/privacy")
    );
    assert!(code.waits_for_consent());
    assert_eq!(code.heads().count(), 2);
    assert_eq!(code.bodies().count(), 1);
}

#[test]
fn only_no_tracking_code_asks_nothing() {
    let config = load(&format!("{SITE}{QUIET}")).unwrap();
    assert!(!config.code().is_empty());
    assert!(!config.code().waits_for_consent());
    let static_site = load(&format!("mode = \"static\"\n{SITE}{QUIET}")).unwrap();
    assert_eq!(static_site.code().heads().count(), 1);
}

#[test]
fn refuses_necessary_naming_the_allowed_categories() {
    let text = QUIET.replace("no-tracking", "necessary");
    let error = load(&format!("{SITE}{text}")).unwrap_err();
    assert!(
        error.contains("field \"category\"")
            && error.contains("preferences, analytics, marketing or no-tracking"),
        "{error}"
    );
}

#[test]
fn refuses_consent_inside_a_list_entry() {
    let text = QUIET.replace("category = \"no-tracking\"", "consent = \"ask\"");
    let error = load(&format!("{SITE}{text}")).unwrap_err();
    assert!(error.contains("consent"), "{error}");
}

#[test]
fn a_static_site_holds_listed_code_too() {
    let text = format!("mode = \"static\"\n{SITE}{ENTRIES}");
    assert!(load(&text).unwrap().code().waits_for_consent());
}
