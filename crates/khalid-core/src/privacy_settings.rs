//! How the site asks for consent, from the `[privacy]` table of
//! `site.toml`, and the two hashes the consent cookie and the consent
//! proof key on (privacy plan U2, KTD3, KTD8, KTD12, KTD15).

use std::collections::BTreeMap;
use std::fmt;

use serde::Deserialize;

use crate::{CodeCategory, Locale, Mode, SiteCode};

/// The fewest months a reader's choice lasts.
const FEWEST_MONTHS: u8 = 1;

/// The most months a reader's choice lasts, the longest the CNIL accepts.
const MOST_MONTHS: u8 = 13;

/// How long a reader's choice lasts when the table does not say.
const DEFAULT_MONTHS: u8 = 6;

/// One of the five fixed groups a reader agrees to (privacy plan KTD2).
/// Owners tag their code into the optional ones but cannot add more.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ConsentCategory {
    /// What the site needs to work. Always on, never a switch.
    Necessary,
    /// Remembering a reader's settings.
    Preferences,
    /// Counting and measuring visits.
    Analytics,
    /// Ads and tracking across sites.
    Marketing,
    /// Players and frames from other sites, such as a `YouTube` video.
    ExternalMedia,
}

impl ConsentCategory {
    /// Every category in the order the banner lists them.
    pub const ALL: [Self; 5] = [
        Self::Necessary,
        Self::Preferences,
        Self::Analytics,
        Self::Marketing,
        Self::ExternalMedia,
    ];

    /// The name forms and settings use, such as `external-media`.
    pub fn key(self) -> &'static str {
        match self {
            Self::Necessary => "necessary",
            Self::Preferences => "preferences",
            Self::Analytics => "analytics",
            Self::Marketing => "marketing",
            Self::ExternalMedia => "external-media",
        }
    }

    /// Whether a reader may turn it on or off: every category but
    /// Necessary, and each starts off.
    pub fn is_optional(self) -> bool {
        self != Self::Necessary
    }
}

/// What a request's Global Privacy Control signal turns off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum GpcMode {
    /// Marketing stays off, as US law reads the signal.
    #[default]
    Marketing,
    /// Every optional category stays off, the stricter reading.
    RejectAll,
}

/// The owner's words for one language, each replacing Khalid's own.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PrivacyText {
    /// The banner's heading.
    pub title: Option<String>,
    /// The banner's sentence.
    pub banner: Option<String>,
    /// What Preferences covers.
    pub preferences: Option<String>,
    /// What Analytics covers.
    pub analytics: Option<String>,
    /// What Marketing covers.
    pub marketing: Option<String>,
    /// What External media covers.
    pub external_media: Option<String>,
}

/// Who runs the site, for the privacy page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PrivacyContact {
    /// The person or company responsible.
    pub name: Option<String>,
    /// Where readers write about their data.
    pub email: Option<String>,
    /// The postal address.
    pub address: Option<String>,
}

/// What the imprint, the Impressum German law asks a website for, shows
/// beyond the name, address and email of [`PrivacyContact`]. Each line is
/// left off the page when it is not set.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PrivacyImprint {
    /// A phone number, a second quick way to reach the owner.
    pub phone: Option<String>,
    /// The trade register entry, such as `Amtsgericht Berlin, HRB 12345`.
    pub register: Option<String>,
    /// The VAT identification number, such as `DE123456789`.
    pub vat_id: Option<String>,
    /// Who answers for the site's articles, with their address.
    pub responsible: Option<String>,
}

/// How the site asks readers for consent. Every key may be left out:
///
/// ```toml
/// [privacy]
/// lifetime_months = 6
/// gpc = "marketing"
/// counts = true
/// external_tool = "Cookiebot"
///
/// [privacy.text.de]
/// banner = "Dürfen wir Besuche zählen?"
///
/// [privacy.contact]
/// email = "privacy@example.com"
///
/// [privacy.imprint]
/// phone = "+49 30 1234567"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PrivacySettings {
    /// How many months a reader's choice lasts, 6 unless set, from 1 to 13.
    #[serde(default = "default_months")]
    pub lifetime_months: u8,
    /// What Global Privacy Control turns off.
    #[serde(default)]
    pub gpc: GpcMode,
    /// Whether choices are counted per day, as written.
    #[serde(default)]
    counts: Option<bool>,
    /// The certified consent tool the owner runs instead, such as
    /// `Cookiebot`. Khalid then holds no code and shows no banner.
    pub external_tool: Option<String>,
    /// The owner's words, per language.
    #[serde(default)]
    pub text: BTreeMap<Locale, PrivacyText>,
    /// Who runs the site.
    #[serde(default)]
    pub contact: PrivacyContact,
    /// The imprint's own lines. Set, even empty, it adds an imprint page.
    #[serde(default)]
    pub imprint: Option<PrivacyImprint>,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            lifetime_months: DEFAULT_MONTHS,
            gpc: GpcMode::default(),
            counts: None,
            external_tool: None,
            text: BTreeMap::new(),
            contact: PrivacyContact::default(),
            imprint: None,
        }
    }
}

fn default_months() -> u8 {
    DEFAULT_MONTHS
}

impl PrivacySettings {
    /// Whether choices are counted on a site run in `mode`: as written, or
    /// on for a server and off for a static host, which sees no choices.
    pub fn counts(&self, mode: Mode) -> bool {
        self.counts.unwrap_or(mode == Mode::Server)
    }

    /// The key that is wrong and why, on a site run in `mode`.
    pub(crate) fn mistake(&self, mode: Mode) -> Option<(&'static str, &'static str)> {
        if !(FEWEST_MONTHS..=MOST_MONTHS).contains(&self.lifetime_months) {
            return Some((
                "lifetime_months",
                "a reader's choice lasts 1 to 13 months, so use a number from 1 to 13",
            ));
        }
        if self.counts == Some(true) && mode == Mode::Static {
            return Some((
                "counts",
                "a static host sends the pages without Khalid, so there are no choices to count; set counts = false, or mode = \"server\"",
            ));
        }
        if self
            .external_tool
            .as_ref()
            .is_some_and(|name| name.trim().is_empty())
        {
            return Some((
                "external_tool",
                "name the certified consent tool, such as Cookiebot, or remove external_tool",
            ));
        }
        if self
            .contact
            .email
            .as_ref()
            .is_some_and(|email| !email.contains('@'))
        {
            return Some((
                "email",
                "the contact email needs an @, like privacy@example.com",
            ));
        }
        self.imprint_mistake()
    }

    /// The contact detail an imprint needs that `[privacy.contact]` lacks:
    /// German law asks for a name, a postal address and an email.
    fn imprint_mistake(&self) -> Option<(&'static str, &'static str)> {
        self.imprint.as_ref()?;
        let blank = |value: &Option<String>| value.as_ref().is_none_or(|v| v.trim().is_empty());
        let contact = &self.contact;
        if blank(&contact.name) {
            return Some((
                "name",
                "the imprint names who runs the site; add name under [privacy.contact], or remove [privacy.imprint]",
            ));
        }
        if blank(&contact.address) {
            return Some((
                "address",
                "the imprint needs a postal address; add address under [privacy.contact], or remove [privacy.imprint]",
            ));
        }
        if blank(&contact.email) {
            return Some((
                "email",
                "the imprint needs an email address; add email under [privacy.contact], or remove [privacy.imprint]",
            ));
        }
        None
    }

    /// The short hash a reader's consent cookie carries (KTD3): the
    /// optional categories that hold `code`, their vendors, External media
    /// with the fixed list of `embeds` Khalid can show, and the lifetime.
    /// When it changes, readers are asked again. Wording does not change
    /// it, and nor does publishing, since `embeds` is every provider and
    /// not the ones posts use.
    pub fn rev(&self, code: &SiteCode, embeds: &[&str]) -> ConsentRev {
        let mut hash = Fnv::new();
        self.write_rev(&mut hash, code, embeds);
        let [a, b, c, d, ..] = hash.finish().to_be_bytes();
        ConsentRev(u32::from_be_bytes([a, b, c, d]))
    }

    /// The hash of everything a reader is shown (KTD8): what [`Self::rev`]
    /// covers, plus the owner's words in each language, the vendors'
    /// policy links, the GPC mode, the outside tool and `built_in`, the
    /// release's own banner words. A wording change adds a version to the
    /// consent proof without asking readers again.
    pub fn version(&self, code: &SiteCode, embeds: &[&str], built_in: &str) -> SettingsVersion {
        let mut hash = Fnv::new();
        self.write_rev(&mut hash, code, embeds);
        for entry in &code.entries {
            hash.field(entry.policy.as_ref().map_or("", |url| url.as_str()));
        }
        hash.field(match self.gpc {
            GpcMode::Marketing => "gpc-marketing",
            GpcMode::RejectAll => "gpc-reject-all",
        });
        hash.field(self.external_tool.as_deref().unwrap_or(""));
        for (locale, text) in &self.text {
            hash.field(locale.as_str());
            for words in [
                &text.title,
                &text.banner,
                &text.preferences,
                &text.analytics,
                &text.marketing,
                &text.external_media,
            ] {
                hash.field(words.as_deref().unwrap_or(""));
            }
        }
        hash.field(built_in);
        SettingsVersion(hash.finish())
    }

    /// Feeds what [`Self::rev`] covers to `hash`, in a fixed order.
    fn write_rev(&self, hash: &mut Fnv, code: &SiteCode, embeds: &[&str]) {
        hash.field(&self.lifetime_months.to_string());
        for category in CodeCategory::ALL {
            if category == CodeCategory::NoTracking {
                continue;
            }
            let held = code
                .entries
                .iter()
                .filter(|e| e.category == category && !e.is_empty());
            let mut vendors: Vec<&str> = held.map(|e| e.vendor.as_deref().unwrap_or("")).collect();
            if vendors.is_empty() {
                continue;
            }
            vendors.sort_unstable();
            vendors.dedup();
            hash.field(category.consent().key());
            for vendor in vendors {
                hash.field(vendor);
            }
        }
        hash.field(ConsentCategory::ExternalMedia.key());
        for provider in embeds {
            hash.field(provider);
        }
    }
}

/// The short hash of the consent settings a reader answered, written as
/// eight hex digits (KTD3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConsentRev(u32);

impl fmt::Display for ConsentRev {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08x}", self.0)
    }
}

/// The hash of everything a reader was shown, written as sixteen hex
/// digits, which keys a settings version in the consent proof (KTD8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SettingsVersion(u64);

impl fmt::Display for SettingsVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

/// FNV-1a over 64 bits. Both hashes live in readers' cookies and in the
/// proof tables, so they must not move with a Rust release, as the
/// standard library's hasher may.
struct Fnv(u64);

impl Fnv {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0100_0000_01b3;

    fn new() -> Self {
        Self(Self::OFFSET)
    }

    /// Adds `text` with its length first, so no two lists of fields run
    /// together into the same bytes.
    fn field(&mut self, text: &str) {
        let length = u64::try_from(text.len()).unwrap_or(u64::MAX);
        for byte in length.to_le_bytes().iter().chain(text.as_bytes()) {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
    }

    fn finish(&self) -> u64 {
        self.0
    }
}
