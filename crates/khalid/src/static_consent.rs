//! Consent on a static site (privacy plan U6, KTD7, KTD9): the site's own
//! code held as plain text, what the site asks its readers, the tag that
//! loads the consent script, and the file that says Global Privacy Control
//! is honoured. With scripts off, the banner stays hidden and nothing
//! optional ever loads.

use khalid_core::{CodeCategory, Config, GpcMode, SiteCode, UserMessage};
use khalid_render::{Asset, BodyHtml, PageBody, consent_script};

use crate::build_pages::LanguagePages;
use crate::post_lock::escaped;
use crate::{assets, build_files, build_message, consent_route};

/// The category a post's own head code waits for: Marketing, as owner code
/// with no category does (KTD2).
pub(crate) const MARKETING: &str = "marketing";

/// Days in a month of the consent lifetime, as the server counts them.
const DAYS_A_MONTH: u16 = 30;

/// The file that tells browsers the site honours Global Privacy Control,
/// at the root of the host.
const GPC_FILE: &str = ".well-known/gpc.json";

/// How an embed's player starts in a page.
const EMBED: &str = "<figure class=\"embed";

/// The marks of what the consent script answers in a page: the banner, an
/// embed and the choices buttons. A page with none of them does not load
/// it.
const NEEDS_SCRIPT: [&str; 3] = ["data-khalid-banner", EMBED, "data-khalid-choices"];

/// The code of `code` that waits for consent, as a static page carries it:
/// each entry held as plain text under its category (R1). Code that tracks
/// no one is open and written beside it.
pub(crate) fn held(code: &SiteCode) -> (String, String) {
    let waiting = (code.entries.iter()).filter(|entry| entry.category != CodeCategory::NoTracking);
    let mut head = Vec::new();
    let mut body = Vec::new();
    for entry in waiting {
        let category = entry.category.key();
        if let Some(code) = entry.head.as_ref().filter(|code| !code.is_empty()) {
            head.push(code.to_held_html(category));
        }
        if let Some(code) = entry.body.as_ref().filter(|code| !code.is_empty()) {
            body.push(code.to_held_html(category));
        }
    }
    (head.join("\n"), body.join("\n"))
}

/// Whether the page `html` needs the consent script.
pub(crate) fn needs_script(html: &str) -> bool {
    NEEDS_SCRIPT.iter().any(|mark| html.contains(mark))
}

/// What a static site asks its readers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Asking {
    /// Nothing: no owner code waits and no page shows an embed, so no page
    /// loads the consent script.
    Nothing,
    /// Only whether to load the players of embeds.
    Media,
    /// Whether to run the site's own code, and to load the players of any
    /// embeds.
    Code,
}

impl Asking {
    /// What the site with `config` and `languages` asks: about its own
    /// code when it waits for consent and no outside tool asks instead,
    /// else about embeds when a post or page shows one.
    pub(crate) fn of(config: &Config, languages: &[LanguagePages]) -> Self {
        let embeds = |body: &BodyHtml| body.as_str().contains(EMBED);
        let shows_embed = (languages.iter())
            .flat_map(|language| &language.pages)
            .any(|page| match &page.body {
                PageBody::Post(post) => embeds(&post.body),
                PageBody::Page(page) => embeds(&page.body),
                _ => false,
            });
        let tool = config.privacy().external_tool.is_some();
        if config.code().waits_for_consent() && !tool {
            Self::Code
        } else if shows_embed {
            Self::Media
        } else {
            Self::Nothing
        }
    }
}

/// The consent script and the tag that loads it, for the site with
/// `config` that asks as `asking` says, or `None` when it asks nothing.
/// The tag carries the cookie's `rev`, the lifetime in days, what Global
/// Privacy Control turns off and the cookie's path.
///
/// # Errors
///
/// Returns what, why and fix when the script cannot be compressed.
pub(crate) fn script(
    config: &Config,
    asking: Asking,
) -> Result<Option<(Asset, String)>, UserMessage> {
    if asking == Asking::Nothing {
        return Ok(None);
    }
    let script = consent_script().map_err(|error| build_message::render(&error))?;
    let privacy = config.privacy();
    let gpc = match privacy.gpc {
        GpcMode::RejectAll => "reject-all",
        _ => "marketing",
    };
    let tag = format!(
        "<script src=\"{src}\" integrity=\"{integrity}\" data-khalid-rev=\"{rev}\" data-khalid-days=\"{days}\" data-khalid-gpc=\"{gpc}\" data-khalid-path=\"{path}\"></script>",
        src = escaped(&assets::href(config, &script)),
        integrity = script.integrity(),
        rev = consent_route::rev(config),
        days = u16::from(privacy.lifetime_months).saturating_mul(DAYS_A_MONTH),
        path = escaped(config.mount()),
    );
    Ok(Some((script, tag)))
}

/// Where the file that says the site honours Global Privacy Control goes
/// in the output of the site with `config`, and what it says, as the
/// server answers it: at the top for a blog at the root of its host, else
/// in the folder of files for the root.
pub(crate) fn gpc_file(config: &Config) -> (String, &'static str) {
    let file = if config.mount() == "/" {
        GPC_FILE.to_owned()
    } else {
        format!("{}/{GPC_FILE}", build_files::ROOT_FOLDER)
    };
    (file, consent_route::GPC_HONOURED)
}
