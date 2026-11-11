//! Settings held in memory, for tests.

use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard, PoisonError};

use khalid_core::{Clock, SystemClock};
use serde_json::{Map, Value};

use super::{SettingsError, SettingsStore};
use crate::content_store::BoxFuture;
use crate::entry_store::EntryStoreError;
use crate::resources::{AdminSettings, SettingsChange};

/// Settings held in memory, for tests. A change applies at once, so a
/// restart is never needed; it refuses a default language missing from
/// the languages, a consent lifetime outside 1 to 13 months and a code
/// category an owner may not pick, as the checks at start do.
#[derive(Debug)]
pub struct MemorySettings {
    settings: Mutex<AdminSettings>,
    clock: Box<dyn Clock>,
}

impl MemorySettings {
    /// A store holding `settings`, stamping changes with the system clock.
    pub fn new(settings: AdminSettings) -> Self {
        Self {
            settings: Mutex::new(settings),
            clock: Box::new(SystemClock),
        }
    }

    /// The same store, stamping changes with `clock`.
    #[must_use]
    pub fn with_clock(self, clock: impl Clock + 'static) -> Self {
        Self {
            clock: Box::new(clock),
            ..self
        }
    }

    fn settings(&self) -> MutexGuard<'_, AdminSettings> {
        self.settings.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn apply(&self, change: SettingsChange) -> Result<AdminSettings, SettingsError> {
        let mut settings = self.settings();
        if settings.updated_at != change.updated_at {
            let updated_at = settings.updated_at;
            return Err(SettingsError::Stale { updated_at });
        }
        let mut next = settings.clone();
        next.name = change.name.unwrap_or(next.name);
        next.address = change.address.unwrap_or(next.address);
        next.mount = change.mount.unwrap_or(next.mount);
        next.default_language = change.default_language.unwrap_or(next.default_language);
        next.languages = change.languages.unwrap_or(next.languages);
        next.template = change.template.unwrap_or(next.template);
        if let Some(cta) = change.cta {
            next.cta = cta.into_setting();
        }
        set_parts(&mut next.seo, change.seo);
        set_parts(&mut next.privacy, change.privacy);
        next.code = change.code.unwrap_or(next.code);
        next.home_sections = change.home_sections.unwrap_or(next.home_sections);
        if let Some(nav) = change.nav {
            next.nav.primary = nav.primary.unwrap_or(next.nav.primary);
            next.nav.footer = nav.footer.unwrap_or(next.nav.footer);
        }
        if !next.languages.contains(&next.default_language) {
            return Err(SettingsError::Refused {
                field: "default_language".to_owned(),
                problem: "the default language must be one of the languages".to_owned(),
            });
        }
        if let Some((field, problem)) = privacy_mistake(&next) {
            return Err(SettingsError::Refused {
                field: field.to_owned(),
                problem: problem.to_owned(),
            });
        }
        next.updated_at = self.clock.now();
        *settings = next.clone();
        Ok(next)
    }
}

/// Puts each sent part into `table`, removing each sent as `None`.
fn set_parts(table: &mut Map<String, Value>, parts: Option<BTreeMap<String, Option<Value>>>) {
    for (part, sent) in parts.unwrap_or_default() {
        match sent {
            Some(value) => table.insert(part, value),
            None => table.remove(&part),
        };
    }
}

/// The key and problem of a privacy setting the checks at start refuse:
/// a lifetime outside 1 to 13 months, or code in a category that is not
/// one of the four an owner may pick.
fn privacy_mistake(settings: &AdminSettings) -> Option<(&'static str, &'static str)> {
    let months = settings.privacy.get("lifetime_months").map(Value::as_u64);
    if months.is_some_and(|months| !(1..=13).contains(&months.unwrap_or(0))) {
        return Some((
            "lifetime_months",
            "a reader's choice lasts 1 to 13 months, so use a number from 1 to 13",
        ));
    }
    let categories = ["preferences", "analytics", "marketing", "no-tracking"];
    let wrong = (settings.code.iter())
        .filter_map(|entry| entry.get("category"))
        .any(|category| {
            !category
                .as_str()
                .is_some_and(|word| categories.contains(&word))
        });
    wrong.then_some((
        "category",
        "use preferences, analytics, marketing or no-tracking",
    ))
}

/// Memory answers at once, so each future is ready when returned.
impl SettingsStore for MemorySettings {
    fn read(&self) -> BoxFuture<'_, Result<AdminSettings, EntryStoreError>> {
        Box::pin(std::future::ready(Ok(self.settings().clone())))
    }

    fn change(
        &self,
        change: SettingsChange,
    ) -> BoxFuture<'_, Result<AdminSettings, SettingsError>> {
        Box::pin(std::future::ready(self.apply(change)))
    }
}
