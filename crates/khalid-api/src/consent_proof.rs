//! The consent proof the Admin API reads (R18, KTD8): every version of
//! what readers were shown, and how many answers allowed each optional
//! category per day. Neither holds an address or a reader identifier.

use chrono::NaiveDate;
use khalid_analytics::{AnalyticsError, BoxFuture};
use khalid_core::ConsentCategory;

/// The optional categories, in the order their counts are kept.
pub const OPTIONAL: [ConsentCategory; 4] = [
    ConsentCategory::Preferences,
    ConsentCategory::Analytics,
    ConsentCategory::Marketing,
    ConsentCategory::ExternalMedia,
];

/// One version of what readers were shown, with the days it was in force.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ShownVersion {
    /// The settings version that names it, as sixteen hex digits.
    pub version: String,
    /// The shown text and settings, as JSON.
    pub shown: String,
    /// The first day a reader was shown it.
    pub first_day: NaiveDate,
    /// The last day a reader was shown it so far.
    pub last_day: NaiveDate,
}

impl ShownVersion {
    /// `version`, showing `shown`, in force from `first_day` to `last_day`.
    pub fn new(version: &str, shown: &str, first_day: NaiveDate, last_day: NaiveDate) -> Self {
        Self {
            version: version.to_owned(),
            shown: shown.to_owned(),
            first_day,
            last_day,
        }
    }
}

/// How readers answered on one day under one version.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct DayChoices {
    /// The day of the answers.
    pub day: NaiveDate,
    /// The version the readers were shown.
    pub version: String,
    /// How many answers there were.
    pub choices: u64,
    allowed: [u64; 4],
}

impl DayChoices {
    /// `choices` answers on `day` under `version`, of which `allowed`
    /// gives how many allowed each optional category.
    pub fn new(
        day: NaiveDate,
        version: &str,
        choices: u64,
        allowed: impl Fn(ConsentCategory) -> u64,
    ) -> Self {
        Self {
            day,
            version: version.to_owned(),
            choices,
            allowed: OPTIONAL.map(allowed),
        }
    }

    /// How many of the answers allowed `category`. Necessary is always
    /// on, so for it this is every answer.
    pub fn allowed(&self, category: ConsentCategory) -> u64 {
        OPTIONAL
            .iter()
            .position(|optional| *optional == category)
            .and_then(|slot| self.allowed.get(slot).copied())
            .unwrap_or(self.choices)
    }
}

/// Where a site keeps its consent proof, read only.
pub trait ConsentProof: Send + Sync {
    /// Every version readers were shown, oldest first.
    ///
    /// # Errors
    ///
    /// [`AnalyticsError::Storage`] when the versions cannot be read.
    fn versions(&self) -> BoxFuture<'_, Result<Vec<ShownVersion>, AnalyticsError>>;

    /// The answers of each day from `first` to `last`, both included, by
    /// day and then by version.
    ///
    /// # Errors
    ///
    /// [`AnalyticsError::Storage`] when the counts cannot be read.
    fn counts(
        &self,
        first: NaiveDate,
        last: NaiveDate,
    ) -> BoxFuture<'_, Result<Vec<DayChoices>, AnalyticsError>>;
}

/// Consent proof held in memory and read back as given, for tests.
#[cfg(any(test, feature = "test-util"))]
#[derive(Debug, Clone, Default)]
pub struct MemoryConsentProof {
    versions: Vec<ShownVersion>,
    counts: Vec<DayChoices>,
}

#[cfg(any(test, feature = "test-util"))]
impl MemoryConsentProof {
    /// A proof holding `versions` and the answers in `counts`.
    pub fn new(versions: Vec<ShownVersion>, counts: Vec<DayChoices>) -> Self {
        Self { versions, counts }
    }
}

#[cfg(any(test, feature = "test-util"))]
impl ConsentProof for MemoryConsentProof {
    fn versions(&self) -> BoxFuture<'_, Result<Vec<ShownVersion>, AnalyticsError>> {
        Box::pin(async { Ok(self.versions.clone()) })
    }

    fn counts(
        &self,
        first: NaiveDate,
        last: NaiveDate,
    ) -> BoxFuture<'_, Result<Vec<DayChoices>, AnalyticsError>> {
        let held = self
            .counts
            .iter()
            .filter(|row| (first..=last).contains(&row.day));
        let counts = held.cloned().collect();
        Box::pin(async move { Ok(counts) })
    }
}
