//! Players from other sites held back until a reader allows External
//! media (privacy plan KTD6, R1, R12).
//!
//! An `::embed` from a provider is written as a card between
//! [`MEDIA_CARD_START`] and [`MEDIA_CARD_END`], and its player inside an
//! inert `template` between [`MEDIA_START`] and [`MEDIA_END`], so a page
//! kept for every reader loads nothing from the provider. The server calls
//! [`release_external_media`] for a reader who allowed External media; a
//! feed calls [`external_media_as_links`], which leaves the title and link.

/// Where an embed's card starts. It is cut once the player is released.
pub const MEDIA_CARD_START: &str = "<!--khalid-media-card-->";

/// Where an embed's card ends.
pub const MEDIA_CARD_END: &str = "<!--/khalid-media-card-->";

/// Where an embed's player starts, inside a `template` no browser shows or
/// loads. Releasing it takes the marks and the `template` away.
pub const MEDIA_START: &str = "<!--khalid-media--><template>";

/// Where an embed's player ends.
pub const MEDIA_END: &str = "</template><!--/khalid-media-->";

/// How an embed's card opens its form that allows External media for the
/// whole site. The server points it at `<mount>_consent/` and fills in the
/// page to come back to.
pub const MEDIA_FORM: &str = "<form class=\"embed-allow\" method=\"post\" action=\"/_consent/\"><input type=\"hidden\" name=\"back\" value=\"/\">";

/// `html` as a reader who allowed External media sees it: each embed's
/// card cut and its player in the page. A page with no embed comes back
/// unchanged.
#[must_use]
pub fn release_external_media(html: &str) -> String {
    if !html.contains(MEDIA_START) {
        return html.to_owned();
    }
    cut(html, MEDIA_CARD_START, MEDIA_CARD_END)
        .replace(MEDIA_START, "")
        .replace(MEDIA_END, "")
}

/// `html` for a feed, an email or any copy read outside the page: each
/// embed's card and player cut, leaving its caption, the title linked to
/// the provider's page.
#[must_use]
pub fn external_media_as_links(html: &str) -> String {
    if !html.contains(MEDIA_START) {
        return html.to_owned();
    }
    cut(
        &cut(html, MEDIA_CARD_START, MEDIA_CARD_END),
        MEDIA_START,
        MEDIA_END,
    )
}

/// `html` without each part from `start` to `end`, marks included. A part
/// never closed is cut to the end, so a player never loads by mistake.
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
