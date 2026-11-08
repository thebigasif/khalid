//! The site's own code from `[code]`, a post's head code and the consent
//! banner, as each page of one render carries them (G45, KH-SEO-12, ADR
//! "consent has categories").

use khalid_core::{BodyCode, CodeCategory, Config, HeadCode, Locale, UserMessage};
use khalid_render::{ConsentBanner, ConsentPart, Renderer};
use khalid_server::{
    ASK_END, ASK_START, CHOSEN_END, CHOSEN_START, HELD_END, HELD_START, NONCE_SLOT, SITE_BODY_SLOT,
    SITE_HEAD_SLOT,
};

use crate::build_message;
use crate::build_pages::privacy_choices::REST as CHOICES;
use crate::site_page::HeadNonce;

/// Where the banner goes: right after the skip link, so it is the first
/// thing after it in the tab order and takes its own room in the flow.
const SKIP_LINK: &str = "<a class=\"skip-link\"";

/// The folder under the mount the consent form posts to. Slugs have no
/// `_`, so no post can take this address.
pub(crate) const CONSENT_FOLDER: &str = "_consent";

/// How the code reaches the pages of one render.
#[derive(Debug, Clone)]
pub(crate) struct PageCode {
    place: Place,
    /// Where the consent form posts, such as `/blog/_consent/`.
    action: String,
    /// Where the blog lives, such as `/blog/`.
    mount: String,
    /// The site's first language, whose pages have no language folder.
    default: Locale,
    /// The head and body code every page carries: code that tracks no one,
    /// or all of it when an outside consent tool asks instead of Khalid.
    open: (String, String),
}

/// Where the code that waits for consent goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Place {
    /// Nowhere, since none waits: every page carries the open code, with
    /// the nonce slot or, on a static host, without.
    Open(HeadNonce),
    /// Into the marked places of every page, which the server fills for a
    /// reader who allowed its category, with the banner after the skip
    /// link.
    Held,
    /// Nowhere: the code waits for consent, which a static host cannot ask.
    Left,
}

impl PageCode {
    /// The code of the site with `config`, for pages whose head code gets
    /// its nonce as `nonce` says.
    pub(crate) fn new(config: &Config, nonce: HeadNonce) -> Self {
        let code = config.code();
        let outside_tool = config.privacy().external_tool.is_some();
        let place = match nonce {
            _ if outside_tool || !code.waits_for_consent() => Place::Open(nonce),
            HeadNonce::PerRequest => Place::Held,
            HeadNonce::Static => Place::Left,
        };
        let open = code
            .entries
            .iter()
            .filter(|entry| outside_tool || entry.category == CodeCategory::NoTracking);
        let head = |head: &HeadCode| match nonce {
            HeadNonce::PerRequest => head.to_html(NONCE_SLOT),
            HeadNonce::Static => head.to_string(),
        };
        let body = |body: &BodyCode| match nonce {
            HeadNonce::PerRequest => body.to_html(NONCE_SLOT),
            HeadNonce::Static => body.to_string(),
        };
        let heads = open.clone().filter_map(|entry| entry.head.as_ref());
        let bodies = open.filter_map(|entry| entry.body.as_ref());
        Self {
            place,
            action: format!("{}{CONSENT_FOLDER}/", config.mount()),
            mount: config.mount().to_owned(),
            default: config.default_language().clone(),
            open: (joined(heads.map(head)), joined(bodies.map(body))),
        }
    }

    /// What goes after a page's SEO head: the post's own head `code`, when
    /// it has some, then the site's open head code and the slot of the code
    /// that waits. Empty when none is there.
    pub(crate) fn head(&self, code: Option<&HeadCode>) -> String {
        let post = code.map_or_else(String::new, |code| match self.place {
            Place::Open(HeadNonce::PerRequest) => code.to_html(NONCE_SLOT),
            Place::Open(HeadNonce::Static) => code.to_string(),
            Place::Held => format!("{HELD_START}{}{HELD_END}", code.to_html(NONCE_SLOT)),
            Place::Left => String::new(),
        });
        let slot = if self.place == Place::Held {
            SITE_HEAD_SLOT
        } else {
            ""
        };
        joined([post, self.open.0.clone(), slot.to_owned()].into_iter())
    }

    /// `html`, a page drawn at `path` in `locale`, with the banner right
    /// after its skip link when code waits for consent, and the site's open
    /// body code, then the slot of the code that waits, before its body
    /// ends.
    ///
    /// # Errors
    ///
    /// Returns what, why and fix when the theme's `consent.html` fails.
    pub(crate) fn with_body_end(
        &self,
        html: String,
        (renderer, locale): (&Renderer, &Locale),
        path: &str,
    ) -> Result<String, UserMessage> {
        let (html, slot) = match self.place {
            Place::Held => (
                self.with_banner(html, renderer, locale, path)?,
                SITE_BODY_SLOT,
            ),
            Place::Open(_) | Place::Left => (html, ""),
        };
        let end = joined([self.open.1.clone(), slot.to_owned()].into_iter());
        Ok(match html.rfind("</body>") {
            Some(at) if !end.is_empty() => {
                let (before, after) = html.split_at(at);
                format!("{before}{end}\n{after}")
            }
            _ => html,
        })
    }

    /// `html` with the banner after its skip link and, hidden until a
    /// reader has chosen, the one line that says the choice was saved. The
    /// choices page asks on its own, so it carries only the line. A theme
    /// with no skip link gets both before its body ends.
    fn with_banner(
        &self,
        html: String,
        renderer: &Renderer,
        locale: &Locale,
        path: &str,
    ) -> Result<String, UserMessage> {
        let home = if locale == &self.default {
            self.mount.clone()
        } else {
            format!("{}{}/", self.mount, locale.as_str().to_ascii_lowercase())
        };
        let banner = ConsentBanner {
            action: self.action.clone(),
            back: path.to_owned(),
            choices: format!("{home}{CHOICES}"),
            // The privacy page comes with privacy plan U9; until then
            // nothing links an address that answers 404.
            privacy: None,
        };
        let part = |part| renderer.consent(locale, &banner, part);
        let saved = part(ConsentPart::Saved).map_err(|error| build_message::render(&error))?;
        let ask = if path == banner.choices {
            String::new()
        } else {
            let ask = part(ConsentPart::Ask).map_err(|error| build_message::render(&error))?;
            format!("{ASK_START}{ask}{ASK_END}\n")
        };
        let parts = format!("\n{ask}{CHOSEN_START}{saved}{CHOSEN_END}");
        let after_skip = html.find(SKIP_LINK).and_then(|at| {
            let end = html.get(at..)?.find("</a>")?;
            at.checked_add(end)?.checked_add("</a>".len())
        });
        let at = after_skip.or_else(|| html.rfind("</body>"));
        Ok(match at {
            Some(at) => {
                let (before, after) = html.split_at(at);
                format!("{before}{parts}{after}")
            }
            None => html,
        })
    }
}

/// The parts that are not empty, one per line.
fn joined(parts: impl Iterator<Item = String>) -> String {
    parts
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
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

    fn post_code() -> HeadCode {
        HeadCode::try_new("<script>post()</script>").unwrap()
    }

    /// A page with a skip link, as `base.html` draws one.
    const SKIPPED: &str = "<html><head></head><body>\n<a class=\"skip-link\" href=\"#main\">Skip</a>\n<header></header><main id=\"main\"><p>Text.</p></main></body></html>";

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
            "value=\"accept-all\">Accept all</button>",
            "value=\"reject-all\">Reject all</button>",
            "href=\"/blog/privacy-choices/\">Choose</a>",
        ] {
            assert!(ask.contains(part), "{part} in {ask}");
        }
        let (chosen, rest) = rest.split_once(CHOSEN_END).unwrap();
        assert!(chosen.contains(CHOSEN_START), "{chosen}");
        assert!(
            chosen.contains("Your privacy choices are saved."),
            "{chosen}"
        );
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
        let tag = format!(
            "<script src=\"https://tags.example.com/t.js\" nonce=\"{NONCE_SLOT}\"></script>"
        );
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
        let code = PageCode::new(&settings, HeadNonce::PerRequest);
        let post = format!("<script nonce=\"{NONCE_SLOT}\">post()</script>");
        assert!(code.head(Some(&post_code())).starts_with(&post));
        let html = ending(&code, "en");
        assert!(html.contains("chat()"), "{html}");
        assert!(!html.contains(ASK_START) && !html.contains(SITE_BODY_SLOT));
    }

    #[test]
    fn a_build_leaves_out_code_that_waits() {
        let code = PageCode::new(&config(CODE), HeadNonce::Static);
        assert_eq!(code.head(Some(&post_code())), "");
        assert_eq!(ending(&code, "en"), PAGE);
    }
}
