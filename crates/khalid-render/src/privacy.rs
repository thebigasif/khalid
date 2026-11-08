//! The privacy page: what the site collects, why, for how long and who else
//! sees it, written from the site's settings and features (privacy plan R20).
//! The templates hold the words; these types hold only the facts.

use serde::Serialize;

/// The generated privacy page of one language, at `<mount>privacy/`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderPrivacy {
    /// The heading, shown as the page's one `h1`.
    pub title: String,
    /// What the page says, as `privacy_facts.html` draws it.
    pub facts: PrivacyFacts,
}

/// Every fact the privacy page states, the same in each language. An
/// owner's own page shows them where it writes `::privacy-facts`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrivacyFacts {
    /// The site's name, who runs it unless `contact` names someone.
    pub site: String,
    /// Who readers write to about their data. `None` until the owner
    /// fills it in; the page then points to the site itself.
    pub contact: Option<PrivacyContact>,
    /// Whether Khalid serves the site, so forms, comments and the
    /// newsletter can work. A static site has none of them.
    pub server: bool,
    /// The cookies the site may set, Necessary ones first.
    pub cookies: Vec<PrivacyCookie>,
    /// The companies whose code runs once a reader agrees, by category.
    pub vendors: Vec<PrivacyVendor>,
    /// The names of the site's scripts that set no cookie and track no
    /// one, which run for every reader.
    pub quiet: Vec<String>,
    /// The players an embed may load once a reader allows external media.
    pub embeds: Vec<String>,
    /// Whether readers may comment.
    pub comments: bool,
    /// Whether readers may subscribe to the newsletter.
    pub newsletter: bool,
    /// Whether the site has a form readers can send.
    pub forms: bool,
    /// How many days the visitor log keeps a visit, when it is on.
    pub visitor_log_days: Option<u8>,
    /// How many months a reader's choice lasts before they are asked again.
    pub consent_months: u8,
    /// Whether a Global Privacy Control signal refuses everything optional,
    /// not only Marketing.
    pub gpc_rejects_all: bool,
    /// The certified consent tool the owner uses instead of Khalid's.
    pub external_tool: Option<String>,
    /// The day the facts last changed, as `2026-11-08`.
    pub changed: String,
}

/// Who readers write to about their data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrivacyContact {
    /// A person's or a company's name.
    pub name: String,
    /// An email address that answers privacy questions.
    pub email: Option<String>,
    /// A postal address, on one line.
    pub address: Option<String>,
}

/// One cookie the site may set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrivacyCookie {
    /// The cookie's name, or its start followed by `*`.
    pub name: String,
    /// What it is for, which picks its words in the strings file.
    pub purpose: CookiePurpose,
    /// How long it lasts.
    pub lasts: Lasts,
}

/// How long a cookie lasts, as `{"unit": "days", "count": 30}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "unit", content = "count", rename_all = "kebab-case")]
pub enum Lasts {
    /// A number of days.
    Days(u16),
    /// A number of months.
    Months(u8),
}

/// What a cookie the site sets is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum CookiePurpose {
    /// Remembers a reader's privacy choices.
    Consent,
    /// Lets a subscriber or member read what is for them.
    Reader,
    /// Opens a post with a password.
    Unlock,
}

/// A company whose code the site runs once a reader agrees.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrivacyVendor {
    /// The company or the script's name, `None` when the owner gave none.
    pub name: Option<String>,
    /// The consent category it waits for.
    pub category: PrivacyCategory,
    /// The address of its own privacy policy.
    pub policy: Option<String>,
}

/// An optional consent category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum PrivacyCategory {
    /// Remembers settings a reader chose.
    Preferences,
    /// Counts visits.
    Analytics,
    /// Shows or measures advertising.
    Marketing,
}
