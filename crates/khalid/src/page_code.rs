//! The site's own code from `[code]`, a post's head code and the consent
//! banner, as each page of one render carries them (G45, KH-SEO-12, ADRs
//! "consent has categories" and "static sites ask for consent with a small
//! script").

use khalid_core::{BodyCode, CodeCategory, Config, HeadCode, Locale, UserMessage};
use khalid_render::{ConsentBanner, ConsentKeeper, ConsentPart, Renderer};
use khalid_server::{
    ASK_END, ASK_START, CHOSEN_END, CHOSEN_START, HELD_END, HELD_START, NONCE_SLOT, SITE_BODY_SLOT,
    SITE_HEAD_SLOT,
};

use crate::build_message;
use crate::build_pages::privacy_choices::REST as CHOICES;
use crate::privacy_page::REST as PRIVACY;
use crate::site_page::HeadNonce;
use crate::static_mode::consent::{self as static_consent, MARKETING};

/// Where the server's banner goes: right after the skip link, so it is the
/// first thing after it in the tab order and takes its own room in the
/// flow.
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
    /// On a static host, the code that waits, held as plain text.
    held: (String, String),
    /// On a static host, the tag that loads the consent script, set by
    /// [`PageCode::with_consent_script`]. Empty until then.
    script: String,
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
    /// Into every page of a static site as plain text, which the consent
    /// script runs for a reader who allowed its category, with the banner
    /// hidden at the end of the body until the script shows it.
    Waiting,
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
            HeadNonce::Static => Place::Waiting,
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
        let waiting = if place == Place::Waiting {
            static_consent::held(code)
        } else {
            (String::new(), String::new())
        };
        Self {
            place,
            action: format!("{}{CONSENT_FOLDER}/", config.mount()),
            mount: config.mount().to_owned(),
            default: config.default_language().clone(),
            open: (joined(heads.map(head)), joined(bodies.map(body))),
            held: waiting,
            script: String::new(),
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
    /// it has some, then the site's open head code and the code that waits,
    /// or its slot. Empty when none is there.
    pub(crate) fn head(&self, code: Option<&HeadCode>) -> String {
        let post = code.map_or_else(String::new, |code| match self.place {
            Place::Open(HeadNonce::PerRequest) => code.to_html(NONCE_SLOT),
            Place::Open(HeadNonce::Static) => code.to_string(),
            Place::Held => format!("{HELD_START}{}{HELD_END}", code.to_html(NONCE_SLOT)),
            Place::Waiting => code.to_held_html(MARKETING),
        });
        let waiting = match self.place {
            Place::Held => SITE_HEAD_SLOT,
            Place::Open(_) | Place::Waiting => &self.held.0,
        };
        joined([post, self.open.0.clone(), waiting.to_owned()].into_iter())
    }

    /// `html`, a page drawn at `path` in `locale`, with the site's open
    /// body code and then the code that waits, or its slot, before its body
    /// ends. The server's banner follows the skip link; a static page's
    /// banner and the consent script, when the page needs it, end the body.
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
        let (html, end) = match self.place {
            Place::Held => (
                self.with_banner(html, renderer, locale, path)?,
                joined([self.open.1.clone(), SITE_BODY_SLOT.to_owned()].into_iter()),
            ),
            Place::Open(_) => (html, self.open.1.clone()),
            Place::Waiting => {
                let banner = self.static_banner(renderer, locale, path)?;
                let parts = [self.open.1.clone(), self.held.1.clone(), banner];
                (html, joined(parts.into_iter()))
            }
        };
        let needs_script =
            static_consent::needs_script(&end) || static_consent::needs_script(&html);
        let script = if needs_script {
            self.script.clone()
        } else {
            String::new()
        };
        let end = joined([end, script].into_iter());
        Ok(match html.rfind("</body>") {
            Some(at) if !end.is_empty() => {
                let (before, after) = html.split_at(at);
                format!("{before}{end}\n{after}")
            }
            _ => html,
        })
    }

    /// The banner's addresses for a page at `path` in `locale`, answered
    /// by `keeper`.
    fn banner_of(&self, locale: &Locale, path: &str, keeper: ConsentKeeper) -> ConsentBanner {
        let home = if locale == &self.default {
            self.mount.clone()
        } else {
            format!("{}{}/", self.mount, locale.as_str().to_ascii_lowercase())
        };
        ConsentBanner {
            action: self.action.clone(),
            back: path.to_owned(),
            choices: format!("{home}{CHOICES}"),
            privacy: Some(format!("{home}{PRIVACY}")),
            keeper,
        }
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
        let banner = self.banner_of(locale, path, ConsentKeeper::Server);
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

    /// A static page's banner, hidden until the consent script shows it,
    /// and the saved line in an inert template the script shows once the
    /// reader has answered. The choices page asks on its own.
    fn static_banner(
        &self,
        renderer: &Renderer,
        locale: &Locale,
        path: &str,
    ) -> Result<String, UserMessage> {
        let banner = self.banner_of(locale, path, ConsentKeeper::Script);
        let part = |part| {
            (renderer.consent(locale, &banner, part)).map_err(|error| build_message::render(&error))
        };
        let saved = format!(
            "<template data-khalid-saved>{}</template>",
            part(ConsentPart::Saved)?
        );
        if path == banner.choices {
            return Ok(saved);
        }
        Ok(format!("{}\n{saved}", part(ConsentPart::Ask)?))
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
#[path = "page_code_tests.rs"]
mod tests;
