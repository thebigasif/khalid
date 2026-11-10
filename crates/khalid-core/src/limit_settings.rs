//! How many requests the APIs and the search page take a minute before
//! they answer 429, from the `[limits]` table of `site.toml` (KH-OPS-5).

use serde::Deserialize;

/// The most requests a minute any limit may allow.
const MOST: u32 = 100_000;

/// How many requests each limit allows a minute. Every key is optional,
/// and the usual numbers suit one site on one small server:
///
/// ```toml
/// [limits]
/// admin_per_minute = 600
/// content_per_minute = 6000
/// wrong_keys_per_minute = 10
/// search_per_minute = 120
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct LimitSettings {
    /// Admin API requests one integration key, staff token or staff
    /// session may make, 600 unless set.
    pub admin_per_minute: u32,
    /// Content API requests one content key may make, 6000 unless set,
    /// since every reader of a headless front end shares that key.
    pub content_per_minute: u32,
    /// Wrong keys one address may send before it is paused, 10 unless set.
    pub wrong_keys_per_minute: u32,
    /// Searches one reader's address may make on the site's search page,
    /// 120 unless set.
    pub search_per_minute: u32,
}

impl Default for LimitSettings {
    fn default() -> Self {
        Self {
            admin_per_minute: 600,
            content_per_minute: 6000,
            wrong_keys_per_minute: 10,
            search_per_minute: 120,
        }
    }
}

impl LimitSettings {
    /// The key that is wrong and why.
    pub(crate) fn mistake(&self) -> Option<(&'static str, &'static str)> {
        let keys = [
            ("admin_per_minute", self.admin_per_minute),
            ("content_per_minute", self.content_per_minute),
            ("wrong_keys_per_minute", self.wrong_keys_per_minute),
            ("search_per_minute", self.search_per_minute),
        ];
        let (key, _) = keys
            .into_iter()
            .find(|&(_, count)| !(1..=MOST).contains(&count))?;
        Some((
            key,
            "a limit counts 1 to 100000 requests a minute, so use a number in that range",
        ))
    }
}
