//! A reader's consent, kept in the `khalid-consent` cookie as
//! `2.<rev>.<bits>.<day>.<flags>`, and the Global Privacy Control signal
//! their browser sends (privacy plan KTD3, KTD5, KTD9, ADR "consent has
//! categories").
//!
//! The cookie carries no identifier. Changing it by hand changes only that
//! reader's own pages, so it is not signed.

use axum::http::{HeaderMap, header};

/// The cookie that keeps a reader's choice.
pub const CONSENT_COOKIE: &str = "khalid-consent";

/// The header a browser sends, set to `1`, when its Global Privacy Control
/// is on.
pub const GPC_HEADER: &str = "sec-gpc";

/// The version the cookie's value starts with.
const VERSION: &str = "2";

/// The flag for "Marketing allowed despite GPC".
const OVER_GPC: u8 = 1;

/// The flag for "answered only External media".
const MEDIA_ONLY: u8 = 1 << 1;

/// A set of the four optional consent categories. Necessary is never in
/// it, because it is always on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Categories(u8);

impl Categories {
    /// No optional category.
    pub const NONE: Self = Self(0);
    /// Remembering a reader's settings.
    pub const PREFERENCES: Self = Self(1);
    /// Counting and measuring visits.
    pub const ANALYTICS: Self = Self(1 << 1);
    /// Ads and tracking across sites.
    pub const MARKETING: Self = Self(1 << 2);
    /// Players and frames from other sites.
    pub const EXTERNAL_MEDIA: Self = Self(1 << 3);
    /// Every optional category.
    pub const ALL: Self = Self(0b1111);

    /// Whether every category in `other` is in this set.
    pub fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// This set with `other` added.
    #[must_use]
    pub fn with(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// This set with `other` taken out.
    #[must_use]
    pub fn without(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// The bits the cookie writes, one per category.
    pub fn bits(self) -> u8 {
        self.0
    }

    /// The set the cookie's `bits` name, or `None` for a bit no category
    /// has.
    pub fn from_bits(bits: u8) -> Option<Self> {
        (bits <= Self::ALL.0).then_some(Self(bits))
    }
}

/// What a reader answered, as the cookie keeps it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Answer {
    /// The categories the reader allowed.
    pub allowed: Categories,
    /// The day of the answer, counted from 1 January 1970.
    pub day: u32,
    /// Whether the reader chose on the choices page while their browser
    /// sent GPC, turning on a category the signal keeps off, so their own
    /// choice stands over the signal.
    pub over_gpc: bool,
    /// Whether the reader only allowed External media from an embed card,
    /// so the banner still asks about the rest.
    pub media_only: bool,
}

impl Answer {
    /// An answer allowing `allowed`, given on `day`, with neither flag.
    pub fn new(allowed: Categories, day: u32) -> Self {
        Self {
            allowed,
            day,
            over_gpc: false,
            media_only: false,
        }
    }

    /// The cookie's value for this answer to the settings `rev` names.
    pub fn value(&self, rev: &str) -> String {
        let flags = u8::from(self.over_gpc) * OVER_GPC + u8::from(self.media_only) * MEDIA_ONLY;
        format!(
            "{VERSION}.{rev}.{}.{}.{flags}",
            self.allowed.bits(),
            self.day
        )
    }

    /// The answer a cookie's `value` holds, when it is version 2 and
    /// answers the settings `rev` names. An old `no` reads as an answer
    /// allowing nothing; an old `yes` never covered categories, so it reads
    /// as no answer, as does anything else.
    pub fn read(value: &str, rev: &str) -> Option<Self> {
        if value == "no" {
            return Some(Self::new(Categories::NONE, 0));
        }
        let mut parts = value.split('.');
        let (Some(VERSION), Some(answered), Some(bits), Some(day), Some(flags), None) = (
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
        ) else {
            return None;
        };
        if answered != rev {
            return None;
        }
        let allowed = Categories::from_bits(bits.parse().ok()?)?;
        let flags: u8 = flags.parse().ok()?;
        if flags > OVER_GPC + MEDIA_ONLY {
            return None;
        }
        Some(Self {
            allowed,
            day: day.parse().ok()?,
            over_gpc: flags & OVER_GPC != 0,
            media_only: flags & MEDIA_ONLY != 0,
        })
    }
}

/// One reader as one request shows them: their answer to the current
/// settings, if any, and whether their browser sends GPC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reader {
    answer: Option<Answer>,
    gpc: bool,
}

impl Reader {
    /// The reader the headers of `request` show, answering the settings
    /// `rev` names. A cookie for other settings counts as no answer, so
    /// the reader is asked again.
    pub fn of(request: &HeaderMap, rev: &str) -> Self {
        let answer = request
            .get_all(header::COOKIE)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .flat_map(|cookies| cookies.split(';'))
            .find_map(|pair| match pair.trim().split_once('=') {
                Some((CONSENT_COOKIE, value)) => Some(value),
                _ => None,
            })
            .and_then(|value| Answer::read(value, rev));
        let gpc = request
            .get(GPC_HEADER)
            .is_some_and(|value| value.as_bytes().trim_ascii() == b"1");
        Self { answer, gpc }
    }

    /// The reader's answer, if they gave one.
    pub fn answer(&self) -> Option<Answer> {
        self.answer
    }

    /// Whether the reader's browser sends GPC.
    pub fn gpc(&self) -> bool {
        self.gpc
    }

    /// Whether the reader has answered, if only from an embed card, so
    /// their pages are made for them alone.
    pub fn answered(&self) -> bool {
        self.answer.is_some()
    }

    /// The categories whose code and players the reader's pages carry:
    /// what they allowed, less `locked` while their browser sends GPC,
    /// unless they turned Marketing on despite it.
    pub fn released(&self, locked: Categories) -> Categories {
        match self.answer {
            None => Categories::NONE,
            Some(answer) if self.gpc && !answer.over_gpc => answer.allowed.without(locked),
            Some(answer) => answer.allowed,
        }
    }
}
