//! The site's own code and other sites' players held back until a reader
//! agrees to their category, and each page filled for the reader (KH-SEO-12,
//! privacy plan KTD1, KTD4, KTD6, KTD9, ADR "consent has categories").
//!
//! A page is rendered once, for a reader who has not answered and whose
//! browser sends no GPC: it holds the banner between [`ASK_START`] and
//! [`ASK_END`], the form to change a choice inside an inert `template`
//! between [`CHOSEN_START`] and [`CHOSEN_END`], a post's own code between
//! [`HELD_START`] and [`HELD_END`], each player between [`MEDIA_START`] and
//! [`MEDIA_END`], and the two places the site's own code goes,
//! [`SITE_HEAD_SLOT`] and [`SITE_BODY_SLOT`]. The copy for a reader whose
//! browser sends GPC ([`under_gpc`]) cuts what lies between
//! [`NO_GPC_START`] and [`NO_GPC_END`] and shows what lies between
//! [`GPC_START`] and [`GPC_END`]. A reader who answered gets the page
//! changed for them alone ([`filled`]).

use crate::consent_cookie::{Categories, Reader};

/// Where the banner starts. It is cut once a reader has chosen.
pub const ASK_START: &str = "<!--khalid-ask-->";

/// Where the banner ends.
pub const ASK_END: &str = "<!--/khalid-ask-->";

/// Where the form to change a choice starts, inside a `template` no
/// browser shows. Once a reader has chosen, the two marks go and the form
/// shows.
pub const CHOSEN_START: &str = "<!--khalid-chosen--><template>";

/// Where the form to change a choice ends.
pub const CHOSEN_END: &str = "</template><!--/khalid-chosen-->";

/// Where the one-line "choices saved" notice starts, inside a `template`.
/// It shows on the page a reader returns to after saving, whose address
/// then carries [`SAVED_QUERY`].
pub const SAVED_START: &str = "<!--khalid-saved--><template>";

/// Where the saved notice ends.
pub const SAVED_END: &str = "</template><!--/khalid-saved-->";

/// The query the consent form adds to the page it returns a reader to.
pub const SAVED_QUERY: &str = "consent=saved";

/// Where a post's own code starts when it waits for consent. It counts as
/// Marketing, and is cut for every reader who has not allowed Marketing.
pub const HELD_START: &str = "<!--khalid-held-->";

/// Where a post's own code ends.
pub const HELD_END: &str = "<!--/khalid-held-->";

/// Where the site's head code goes, for a reader who allowed its category.
pub const SITE_HEAD_SLOT: &str = "<!--khalid-site-head-->";

/// Where the site's body code goes, for a reader who allowed its category.
pub const SITE_BODY_SLOT: &str = "<!--khalid-site-body-->";

/// Where another site's player starts, inside a `template` no browser
/// shows. For a reader who allowed External media the two marks go and
/// the player loads.
pub const MEDIA_START: &str = "<!--khalid-external-media--><template>";

/// Where another site's player ends.
pub const MEDIA_END: &str = "</template><!--/khalid-external-media-->";

/// Where a part only a reader without GPC sees starts, such as the
/// banner's Marketing row.
pub const NO_GPC_START: &str = "<!--khalid-no-gpc-->";

/// Where a part only a reader without GPC sees ends.
pub const NO_GPC_END: &str = "<!--/khalid-no-gpc-->";

/// Where a part only a reader with GPC sees starts, inside a `template`,
/// such as the note that the signal was honoured.
pub const GPC_START: &str = "<!--khalid-gpc--><template>";

/// Where a part only a reader with GPC sees ends.
pub const GPC_END: &str = "</template><!--/khalid-gpc-->";

/// The site's own code that waits for consent, each piece with its
/// category, with [`NONCE_SLOT`](crate::NONCE_SLOT) on each `script` and
/// `style`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HeldCode {
    pieces: Vec<(Categories, String, String)>,
}

impl HeldCode {
    /// No code at all.
    pub fn new() -> Self {
        Self::default()
    }

    /// This code with `head` and `body`, already checked and written with
    /// the nonce slot, which run for a reader who allowed `category`.
    #[must_use]
    pub fn with(
        mut self,
        category: Categories,
        head: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        self.pieces.push((category, head.into(), body.into()));
        self
    }

    /// The head and body code of the pieces in `released`, in the order
    /// written.
    fn released(&self, released: Categories) -> (String, String) {
        let pieces = self
            .pieces
            .iter()
            .filter(|(category, ..)| released.contains(*category));
        let (heads, bodies): (Vec<&str>, Vec<&str>) = pieces
            .map(|(_, head, body)| (head.as_str(), body.as_str()))
            .unzip();
        let joined = |parts: Vec<&str>| {
            let parts: Vec<&str> = parts.into_iter().filter(|p| !p.is_empty()).collect();
            parts.join("\n")
        };
        (joined(heads), joined(bodies))
    }
}

/// What the server needs to fill pages for a site: the short hash of the
/// consent settings readers answer, the held code, and the categories a
/// browser's GPC keeps off.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConsentSite {
    rev: String,
    held: HeldCode,
    locked: Categories,
}

impl ConsentSite {
    /// A site whose consent settings hash to `rev`, holding `held`, where
    /// GPC keeps `locked` off: Marketing, or every category for an owner
    /// who reads the signal as Reject all.
    pub fn new(rev: impl Into<String>, held: HeldCode, locked: Categories) -> Self {
        Self {
            rev: rev.into(),
            held,
            locked,
        }
    }

    /// The short hash a current cookie carries.
    pub fn rev(&self) -> &str {
        &self.rev
    }

    /// The categories GPC keeps off.
    pub fn locked(&self) -> Categories {
        self.locked
    }
}

/// `html`, a page as a reader with GPC and no answer sees it: the parts
/// for readers without GPC cut and the parts for readers with it shown.
pub fn under_gpc(html: &str) -> String {
    unwrapped(&cut(html, NO_GPC_START, NO_GPC_END), GPC_START, GPC_END)
}

/// Whether a page shows the notice that the reader's choices were saved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedNotice {
    /// The page the consent form returned the reader to.
    Shown,
    /// Any other page.
    Hidden,
}

/// `html`, a page rendered for a reader who has not answered, as `reader`
/// sees it on `site`, with the saved notice as `saved` says.
///
/// A reader who has not answered keeps the banner, and no held code. One
/// who allowed only External media from a card keeps the banner too, with
/// the players loaded. One who chose gets the form to change it in place
/// of the banner, the code of each category released to them, and the
/// players when External media is.
///
/// The router applies it to every HTML page; a page drawn outside the
/// router, such as a page of search results, applies it too.
pub fn filled(html: &str, reader: &Reader, site: &ConsentSite, saved: SavedNotice) -> String {
    let mut html = if reader.gpc() {
        under_gpc(html)
    } else {
        html.to_owned()
    };
    let released = reader.released(site.locked);
    if !released.contains(Categories::MARKETING) {
        html = cut(&html, HELD_START, HELD_END);
    }
    if released.contains(Categories::EXTERNAL_MEDIA) {
        html = unwrapped(&html, MEDIA_START, MEDIA_END);
    }
    let Some(answer) = reader.answer() else {
        return html;
    };
    if answer.media_only {
        return html;
    }
    let (head, body) = site.held.released(released);
    html = cut(&html, ASK_START, ASK_END)
        .replacen(SITE_HEAD_SLOT, &head, 1)
        .replacen(SITE_BODY_SLOT, &body, 1);
    html = unwrapped(&html, CHOSEN_START, CHOSEN_END);
    if saved == SavedNotice::Shown {
        html = unwrapped(&html, SAVED_START, SAVED_END);
    }
    html
}

/// `html` without each part from `start` to `end`, marks included. A part
/// never closed is cut to the end, so held code never runs by mistake.
fn cut(html: &str, start: &str, end: &str) -> String {
    let mut kept = String::with_capacity(html.len());
    let mut rest = html;
    while let Some((before, after)) = rest.split_once(start) {
        kept.push_str(before);
        rest = after.split_once(end).map_or("", |(_, after)| after);
    }
    kept.push_str(rest);
    kept
}

/// `html` with every `start` and `end` mark taken out, so what lay between
/// them shows. A part never closed is cut to the end instead, so a broken
/// page never shows half a player.
fn unwrapped(html: &str, start: &str, end: &str) -> String {
    let mut kept = String::with_capacity(html.len());
    let mut rest = html;
    while let Some((before, after)) = rest.split_once(start) {
        kept.push_str(before);
        let Some((inside, after)) = after.split_once(end) else {
            return kept;
        };
        kept.push_str(inside);
        rest = after;
    }
    kept.push_str(rest);
    kept
}
