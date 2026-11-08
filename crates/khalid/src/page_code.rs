//! The site's own code from `[code]`, a post's head code and the consent
//! banner, as each page of one render carries them (G45, KH-SEO-12, ADR
//! "consent is a form and a cookie", privacy plan U6).

use khalid_core::{Config, HeadCode, Locale};
use khalid_render::Renderer;
use khalid_server::{
    ASK_END, ASK_START, CHOSEN_END, CHOSEN_START, HELD_END, HELD_START, NONCE_SLOT, SITE_BODY_SLOT,
    SITE_HEAD_SLOT,
};

use crate::build_pages;
use crate::post_lock::escaped;
use crate::site_page::HeadNonce;
use crate::static_mode::consent::{self as static_consent, MARKETING};

/// The folder under the mount the consent form posts to. Slugs have no
/// `_`, so no post can take this address.
pub(crate) const CONSENT_FOLDER: &str = "_consent";

/// How the code reaches the pages of one render.
#[derive(Debug, Clone)]
pub(crate) struct PageCode {
    place: Place,
    /// Where the consent form posts, such as `/blog/_consent/`.
    action: String,
    /// The site's head and body code as each page carries it, open or, on
    /// a static host, held as plain text.
    site: (String, String),
    /// On a static host, the tag that loads the consent script, set by
    /// [`PageCode::with_consent_script`]. Empty until then.
    script: String,
    /// The choices page of each language, such as `/blog/de/privacy-choices/`.
    choices: Vec<(Locale, String)>,
}

/// Where the code goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Place {
    /// Into every page, with the nonce slot or, on a static host, without.
    Open(HeadNonce),
    /// Into the marked places of every page, which the server fills only
    /// for a reader who agreed, with the banner before the body ends.
    Held,
    /// Into every page of a static site as plain text, which the consent
    /// script runs only for a reader who agreed, with the banner hidden
    /// until the script shows it.
    Waiting,
}

impl PageCode {
    /// The code of the site with `config`, for pages whose head code gets
    /// its nonce as `nonce` says.
    pub(crate) fn new(config: &Config, nonce: HeadNonce) -> Self {
        let code = config.code();
        let place = match nonce {
            _ if !code.waits_for_consent() => Place::Open(nonce),
            HeadNonce::PerRequest => Place::Held,
            HeadNonce::Static => Place::Waiting,
        };
        let written = |html: Option<String>| html.unwrap_or_default();
        let site = match place {
            Place::Open(HeadNonce::PerRequest) => (
                written(code.head.as_ref().map(|head| head.to_html(NONCE_SLOT))),
                written(code.body.as_ref().map(|body| body.to_html(NONCE_SLOT))),
            ),
            Place::Open(HeadNonce::Static) => (
                written(code.head.as_ref().map(ToString::to_string)),
                written(code.body.as_ref().map(ToString::to_string)),
            ),
            Place::Waiting => (
                written(code.head.as_ref().map(|head| head.to_held_html(MARKETING))),
                written(code.body.as_ref().map(|body| body.to_held_html(MARKETING))),
            ),
            Place::Held => (String::new(), String::new()),
        };
        let choices = (config.languages().iter())
            .map(|locale| {
                let path = build_pages::path_of(config, locale, static_consent::CHOICES);
                (locale.clone(), path)
            })
            .collect();
        Self {
            place,
            action: format!("{}{CONSENT_FOLDER}/", config.mount()),
            site,
            script: String::new(),
            choices,
        }
    }

    /// The code with `tag`, which loads the consent script, at the end of
    /// each static page that needs it.
    pub(crate) fn with_consent_script(self, tag: String) -> Self {
        Self {
            script: tag,
            ..self
        }
    }

    /// What goes after a page's SEO head: the post's own head `code`, when
    /// it has some, then the site's head code or its slot. Empty when
    /// neither is there.
    pub(crate) fn head(&self, code: Option<&HeadCode>) -> String {
        let post = code.map_or_else(String::new, |code| match self.place {
            Place::Open(HeadNonce::PerRequest) => code.to_html(NONCE_SLOT),
            Place::Open(HeadNonce::Static) => code.to_string(),
            Place::Held => format!("{HELD_START}{}{HELD_END}", code.to_html(NONCE_SLOT)),
            Place::Waiting => code.to_held_html(MARKETING),
        });
        let site = match self.place {
            Place::Held => SITE_HEAD_SLOT,
            Place::Open(_) | Place::Waiting => &self.site.0,
        };
        [post.as_str(), site]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// `html`, a page drawn at `path` in `locale`, with the site's body
    /// code, or its slot and the banner, before its body ends. A static
    /// page that asks for consent, shows an embed or is the choices page
    /// loads the consent script last.
    pub(crate) fn with_body_end(
        &self,
        html: String,
        (renderer, locale): (&Renderer, &Locale),
        path: &str,
    ) -> String {
        let mut end = match self.place {
            Place::Held => self.banner(renderer, locale, path),
            Place::Open(_) => self.site.1.clone(),
            Place::Waiting => {
                let choices = (self.choices.iter())
                    .find(|(language, _)| language == locale)
                    .map_or("", |(_, path)| path.as_str());
                let banner = static_consent::banner(renderer, locale, choices);
                [self.site.1.as_str(), &banner]
                    .into_iter()
                    .filter(|part| !part.is_empty())
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        };
        if !self.script.is_empty()
            && (static_consent::needs_script(&end) || static_consent::needs_script(&html))
        {
            if !end.is_empty() {
                end.push('\n');
            }
            end.push_str(&self.script);
        }
        match html.rfind("</body>") {
            Some(at) if !end.is_empty() => {
                let (before, after) = html.split_at(at);
                format!("{before}{end}\n{after}")
            }
            _ => html,
        }
    }

    /// The body code's slot, the banner and, hidden until a reader has
    /// chosen, the smaller form to change the choice.
    fn banner(&self, renderer: &Renderer, locale: &Locale, path: &str) -> String {
        let words = |key: &str| escaped(renderer.text(locale, key).unwrap_or_default());
        let form = |class: &str, note: &str| {
            format!(
                "<aside class=\"{class}\" aria-label=\"{label}\"><form method=\"post\" action=\"{action}\">\
                 <p>{note}</p><input type=\"hidden\" name=\"back\" value=\"{back}\">\
                 <button type=\"submit\" name=\"choice\" value=\"yes\">{allow}</button>\
                 <button type=\"submit\" name=\"choice\" value=\"no\">{refuse}</button></form></aside>",
                label = words("consent_label"),
                action = escaped(&self.action),
                back = escaped(path),
                allow = words("consent_allow"),
                refuse = words("consent_refuse"),
            )
        };
        format!(
            "{SITE_BODY_SLOT}\n{ASK_START}{}{ASK_END}\n{CHOSEN_START}{}{CHOSEN_END}",
            form("consent", &words("consent_note")),
            form("consent consent-chosen", &words("consent_change")),
        )
    }
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

    const PAGE: &str = "<html><head></head><body><p>Text.</p></body></html>";

    fn post_code() -> HeadCode {
        HeadCode::try_new("<script>post()</script>").unwrap()
    }

    fn ending(code: &PageCode, locale: &str) -> String {
        let renderer = Renderer::new().unwrap();
        let locale: Locale = locale.parse().unwrap();
        code.with_body_end(PAGE.to_owned(), (&renderer, &locale), "/blog/a-post/")
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
    fn code_that_waits_is_marked_and_the_banner_ends_the_body() {
        let code = PageCode::new(&config(CODE), HeadNonce::PerRequest);
        let post = format!("<script nonce=\"{NONCE_SLOT}\">post()</script>");
        assert_eq!(
            code.head(Some(&post_code())),
            format!("{HELD_START}{post}{HELD_END}\n{SITE_HEAD_SLOT}")
        );
        let html = ending(&code, "en");
        assert!(!html.contains("chat()"), "{html}");
        let (body, end) = html.split_once(SITE_BODY_SLOT).unwrap();
        assert!(body.ends_with("<p>Text.</p>"), "{html}");
        assert!(end.ends_with("</body></html>"), "{html}");
        let (ask, chosen) = end.split_once(ASK_END).unwrap();
        for part in [
            "<form method=\"post\" action=\"/blog/_consent/\">",
            "name=\"back\" value=\"/blog/a-post/\"",
            "It runs only if you allow it.",
            "value=\"yes\">Allow</button>",
            "value=\"no\">Do not allow</button>",
        ] {
            assert!(ask.contains(part), "{part} in {ask}");
        }
        assert!(chosen.contains(CHOSEN_START), "{chosen}");
        assert!(chosen.contains("You can change your choice"), "{chosen}");
        assert!(ending(&code, "de").contains("Erlauben</button>"));
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
    fn a_build_holds_code_that_waits_as_plain_text() {
        let code = PageCode::new(&config(CODE), HeadNonce::Static);
        assert_eq!(
            code.head(Some(&post_code())),
            "<script type=\"text/plain\" data-khalid-consent=\"marketing\">post()</script>\n\
             <script type=\"text/plain\" data-khalid-consent=\"marketing\" src=\"https://tags.example.com/t.js\"></script>"
        );
        let html = ending(&code, "de");
        let held = "<p>Text.</p><script type=\"text/plain\" data-khalid-consent=\"marketing\">chat()</script>\n<section class=\"consent\"";
        assert!(html.contains(held), "{html}");
        assert!(
            html.contains("<a href=\"/blog/de/privacy-choices/\">Auswählen</a>"),
            "{html}"
        );
        assert!(!html.contains("<script src"), "{html}");
        let loaded = code.with_consent_script("<script src=\"/c.js\" defer></script>".to_owned());
        let html = ending(&loaded, "en");
        assert!(
            html.ends_with("<script src=\"/c.js\" defer></script>\n</body></html>"),
            "{html}"
        );
    }

    #[test]
    fn a_build_without_code_loads_the_script_only_where_a_page_needs_it() {
        let tag = "<script src=\"/c.js\" defer></script>";
        let code =
            PageCode::new(&config(""), HeadNonce::Static).with_consent_script(tag.to_owned());
        assert_eq!(ending(&code, "en"), PAGE);
        let renderer = Renderer::new().unwrap();
        let locale: Locale = "en".parse().unwrap();
        let embed =
            "<html><body><figure class=\"embed embed-wide\"></figure></body></html>".to_owned();
        let html = code.with_body_end(embed, (&renderer, &locale), "/blog/a-post/");
        assert!(
            html.ends_with(&format!("</figure>{tag}\n</body></html>")),
            "{html}"
        );
    }
}
