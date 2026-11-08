//! The consent banner and the privacy choices page (privacy plan U4,
//! KTD10). Both are plain forms that post to the consent route, so they
//! work with scripts off on a running server.

use serde::Serialize;

/// Where the note that the reader's Global Privacy Control signal was
/// honoured starts, inside a `template` no browser shows. The banner and
/// the choices page carry it; the server removes the two marks for a
/// request that sends the signal, so the note shows.
pub const GPC_START: &str = "<!--khalid-gpc--><template>";

/// Where the Global Privacy Control note ends.
pub const GPC_END: &str = "</template><!--/khalid-gpc-->";

/// The addresses the banner links and posts to, which templates read as
/// `banner`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConsentBanner {
    /// Where the form posts, such as `/blog/_consent/`.
    pub action: String,
    /// The page the reader is on, which the answer sends them back to.
    pub back: String,
    /// The privacy choices page in the page's language, such as
    /// `/blog/privacy-choices/`.
    pub choices: String,
    /// The privacy page in the page's language, such as `/blog/privacy/`,
    /// once the site has one: until then nothing links it.
    pub privacy: Option<String>,
    /// Who keeps the answer: the server, through a form, or a static
    /// site's consent script, through plain buttons in a banner hidden
    /// until the script shows it.
    pub keeper: ConsentKeeper,
}

/// Who keeps a reader's consent, which picks how the banner and the
/// choices page ask (privacy plan KTD1, KTD7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum ConsentKeeper {
    /// A running server, which reads the plain forms, so they work with
    /// scripts off.
    Server,
    /// The consent script of a static site, which answers plain buttons.
    /// A static build locks every form, and with scripts off nothing
    /// optional loads, so there is nothing to ask.
    Script,
    /// Nobody: a static site with no code that waits and no embed asks
    /// nothing, so its choices page says so.
    Nobody,
}

/// Which part of the banner to draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum ConsentPart {
    /// The question a reader who has not answered sees: Accept all, Reject
    /// all and Choose.
    Ask,
    /// The one line a reader who answered sees, with a link to change it.
    Saved,
}

/// The privacy choices page of one language, at `privacy-choices/`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderPrivacyChoices {
    /// The heading, shown as the page's one `h1`.
    pub title: String,
    /// Where the form posts, such as `/blog/_consent/`.
    pub action: String,
    /// The privacy page in the page's language, once the site has one.
    pub privacy: Option<String>,
    /// Who keeps the choice. A static host keeps it only with scripts on,
    /// so its page says so to a reader without them.
    pub keeper: ConsentKeeper,
    /// One row per category, in the order the page shows them.
    pub categories: Vec<ConsentRow>,
}

/// One category on the choices page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConsentRow {
    /// The name the form sends, such as `external-media`.
    pub key: String,
    /// The category's name, shown as its `legend`.
    pub name: String,
    /// One sentence on what the category covers.
    pub note: String,
    /// Who the category shares data with, such as `YouTube`.
    pub vendors: Vec<String>,
    /// Whether the reader may switch it. Necessary is always on.
    pub optional: bool,
}
