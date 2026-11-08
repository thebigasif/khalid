use std::path::Path;

use khalid_core::Environment;

use super::*;

/// The demo's settings with `rest` after them.
fn config(rest: &str) -> Config {
    let text = format!(
        "name = \"Field Notes on SEO\"\naddress = \"https://www.example.com\"\nmount = \"/blog/\"\n\
         default_language = \"en\"\nlanguages = [\"en\", \"de\"]\n{rest}"
    );
    Config::from_toml(Path::new("site.toml"), &text, &Environment::empty()).unwrap()
}

const CODE: &str = "[code]\nhead = \"<script src='https://tags.example.com/t.js'></script>\"\nbody = \"<script>chat()</script>\"\n";

const NOT_NEEDED: &str = "consent = \"not-needed\"\n";

/// One piece that tracks no one and one that waits for Analytics.
const MIXED: &str = "[[code]]\nhead = \"<script>count()</script>\"\ncategory = \"no-tracking\"\n\
                     [[code]]\nbody = \"<script>stats()</script>\"\ncategory = \"analytics\"\n";

const PAGE: &str = "<html><head></head><body><p>Text.</p></body></html>";

/// A page with a skip link, as `base.html` draws one.
const SKIPPED: &str = "<html><head></head><body>\n<a class=\"skip-link\" href=\"#main\">Skip</a>\n<header></header><main id=\"main\"><p>Text.</p></main></body></html>";

fn post_code() -> HeadCode {
    HeadCode::try_new("<script>post()</script>").unwrap()
}

fn drawn(code: &PageCode, page: &str, (locale, path): (&str, &str)) -> String {
    let renderer = Renderer::new().unwrap();
    let locale: Locale = locale.parse().unwrap();
    code.with_body_end(page.to_owned(), (&renderer, &locale), path)
        .unwrap()
}

fn ending(code: &PageCode, locale: &str) -> String {
    drawn(code, PAGE, (locale, "/blog/a-post/"))
}

#[test]
fn a_site_without_code_draws_pages_as_before() {
    let code = PageCode::new(&config(""), HeadNonce::PerRequest);
    assert_eq!(code.head(None), "");
    let post = format!("<script nonce=\"{NONCE_SLOT}\">post()</script>");
    assert_eq!(code.head(Some(&post_code())), post);
    assert_eq!(ending(&code, "en"), PAGE);
}

#[test]
fn code_that_waits_is_marked_and_the_banner_follows_the_skip_link() {
    let code = PageCode::new(&config(CODE), HeadNonce::PerRequest);
    let post = format!("<script nonce=\"{NONCE_SLOT}\">post()</script>");
    assert_eq!(
        code.head(Some(&post_code())),
        format!("{HELD_START}{post}{HELD_END}\n{SITE_HEAD_SLOT}")
    );
    let html = drawn(&code, SKIPPED, ("en", "/blog/a-post/"));
    assert!(!html.contains("chat()"), "{html}");
    let (top, rest) = html.split_once(ASK_START).unwrap();
    assert!(top.ends_with("Skip</a>\n"), "{html}");
    let (ask, rest) = rest.split_once(ASK_END).unwrap();
    for part in [
        "<form method=\"post\" action=\"/blog/_consent/\">",
        "name=\"back\" value=\"/blog/a-post/\"",
        "type=\"submit\" name=\"choice\" value=\"accept-all\">Accept all</button>",
        "value=\"reject-all\">Reject all</button>",
        "href=\"/blog/privacy-choices/\">Choose</a>",
    ] {
        assert!(ask.contains(part), "{part} in {ask}");
    }
    assert!(!ask.contains("data-khalid-banner"), "{ask}");
    let (chosen, rest) = rest.split_once(CHOSEN_END).unwrap();
    assert!(chosen.contains(CHOSEN_START), "{chosen}");
    assert!(
        rest.starts_with("\n<header>"),
        "the page comes after: {rest}"
    );
    assert!(rest.ends_with(&format!("{SITE_BODY_SLOT}\n</body></html>")));
    let german = drawn(&code, SKIPPED, ("de", "/blog/de/a-post/"));
    assert!(german.contains("href=\"/blog/de/privacy-choices/\">Auswählen</a>"));
}

#[test]
fn the_choices_page_carries_the_saved_line_but_no_question() {
    let code = PageCode::new(&config(CODE), HeadNonce::PerRequest);
    let html = drawn(&code, SKIPPED, ("en", "/blog/privacy-choices/"));
    assert!(!html.contains(ASK_START), "{html}");
    assert!(html.contains(CHOSEN_START), "{html}");
}

#[test]
fn a_theme_without_a_skip_link_gets_the_banner_before_the_body_ends() {
    let code = PageCode::new(&config(CODE), HeadNonce::PerRequest);
    let html = ending(&code, "en");
    let (body, end) = html.split_once(ASK_START).unwrap();
    assert!(body.ends_with("<p>Text.</p>\n"), "{html}");
    assert!(end.ends_with(&format!("{SITE_BODY_SLOT}\n</body></html>")));
}

#[test]
fn code_that_needs_no_consent_goes_into_every_page() {
    let settings = config(&format!("{CODE}{NOT_NEEDED}"));
    let code = PageCode::new(&settings, HeadNonce::PerRequest);
    let tag =
        format!("<script src=\"https://tags.example.com/t.js\" nonce=\"{NONCE_SLOT}\"></script>");
    assert_eq!(code.head(None), tag);
    let chat = format!("<script nonce=\"{NONCE_SLOT}\">chat()</script>");
    let page = format!("<html><head></head><body><p>Text.</p>{chat}\n</body></html>");
    assert_eq!(ending(&code, "en"), page);
    let built = PageCode::new(&settings, HeadNonce::Static);
    assert_eq!(
        built.head(Some(&post_code())),
        "<script>post()</script>\n<script src=\"https://tags.example.com/t.js\"></script>"
    );
}

#[test]
fn code_that_tracks_no_one_runs_beside_the_slot_of_code_that_waits() {
    let code = PageCode::new(&config(MIXED), HeadNonce::PerRequest);
    let count = format!("<script nonce=\"{NONCE_SLOT}\">count()</script>");
    assert_eq!(code.head(None), format!("{count}\n{SITE_HEAD_SLOT}"));
    let html = ending(&code, "en");
    assert!(
        html.contains(ASK_START) && !html.contains("stats()"),
        "{html}"
    );
}

#[test]
fn an_outside_consent_tool_gets_every_piece_unheld_and_no_banner() {
    let settings = config(&format!("{CODE}[privacy]\nexternal_tool = \"Cookiebot\"\n"));
    for nonce in [HeadNonce::PerRequest, HeadNonce::Static] {
        let code = PageCode::new(&settings, nonce);
        let html = ending(&code, "en");
        assert!(
            html.contains("chat()") && !html.contains("text/plain"),
            "{html}"
        );
        assert!(!html.contains("consent-button") && !html.contains(SITE_BODY_SLOT));
    }
}

#[test]
fn a_build_holds_code_that_waits_as_plain_text_under_its_category() {
    let code = PageCode::new(&config(CODE), HeadNonce::Static);
    assert_eq!(
        code.head(Some(&post_code())),
        "<script type=\"text/plain\" data-khalid-consent=\"marketing\">post()</script>\n\
         <script type=\"text/plain\" data-khalid-consent=\"marketing\" src=\"https://tags.example.com/t.js\"></script>"
    );
    let mixed = PageCode::new(&config(MIXED), HeadNonce::Static);
    assert_eq!(mixed.head(None), "<script>count()</script>");
    let html = ending(&mixed, "en");
    assert!(
        html.contains(
            "<script type=\"text/plain\" data-khalid-consent=\"analytics\">stats()</script>"
        ),
        "{html}"
    );
}

#[test]
fn a_static_banner_is_hidden_buttons_at_the_end_of_the_body() {
    let code = PageCode::new(&config(CODE), HeadNonce::Static);
    let html = drawn(&code, SKIPPED, ("de", "/blog/de/a-post/"));
    assert!(html.contains("Skip</a>\n<header>"), "{html}");
    let (_, banner) = html.split_once("data-khalid-banner").unwrap();
    assert!(banner.starts_with(" hidden>"), "{banner}");
    assert!(!html.contains("<form"), "{html}");
    for part in [
        "type=\"button\" name=\"choice\" value=\"accept-all\"",
        "href=\"/blog/de/privacy-choices/\">Auswählen</a>",
        "<template data-khalid-saved>",
    ] {
        assert!(banner.contains(part), "{part} in {banner}");
    }
    assert!(
        !html.contains(ASK_START) && !html.contains("<script src"),
        "{html}"
    );
    let loaded = code.with_consent_script("<script src=\"/c.js\" defer></script>".to_owned());
    let html = drawn(&loaded, SKIPPED, ("en", "/blog/a-post/"));
    assert!(
        html.ends_with("<script src=\"/c.js\" defer></script>\n</body></html>"),
        "{html}"
    );
    let choices = drawn(&loaded, SKIPPED, ("en", "/blog/privacy-choices/"));
    assert!(!choices.contains("data-khalid-banner"), "{choices}");
}

#[test]
fn a_build_without_code_loads_the_script_only_where_a_page_needs_it() {
    let tag = "<script src=\"/c.js\" defer></script>";
    let code = PageCode::new(&config(""), HeadNonce::Static).with_consent_script(tag.to_owned());
    assert_eq!(ending(&code, "en"), PAGE);
    let embed = "<html><body><figure class=\"embed embed-wide\"></figure></body></html>";
    let html = drawn(&code, embed, ("en", "/blog/a-post/"));
    assert!(
        html.ends_with(&format!("</figure>{tag}\n</body></html>")),
        "{html}"
    );
}
