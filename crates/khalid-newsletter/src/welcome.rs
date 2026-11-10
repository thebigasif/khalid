//! Welcome emails (task 5.9, G39): what a newsletter sends a reader the
//! moment they join, and the steps of its welcome sequence that follow
//! some days later.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use khalid_core::Slug;

use crate::{BoxFuture, PostalAddress};

/// The longest wait for a step of a welcome sequence, in days: a year.
pub const LAST_DAY: u16 = 365;

/// One step of a newsletter's welcome sequence, as its writer wrote it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Welcome {
    day: u16,
    subject: String,
    text: String,
}

impl Welcome {
    /// Reads `markdown`, whose first line is a `# ` heading that becomes
    /// the subject and whose rest is the text, as the step sent the moment
    /// a reader joins. `None` when the first line is not such a heading or
    /// the text is empty.
    pub fn read(markdown: &str) -> Option<Self> {
        let markdown = markdown.trim_start_matches('\u{feff}').trim_start();
        let (first, rest) = markdown.split_once('\n').unwrap_or((markdown, ""));
        let subject = first.strip_prefix("# ")?.trim();
        let text = rest.trim();
        (!subject.is_empty() && !text.is_empty()).then(|| Self {
            day: 0,
            subject: subject.to_owned(),
            text: format!("{text}\n"),
        })
    }

    /// The day a file called `name` is sent on: 0 for `welcome.md`, and 3
    /// for `welcome-day-3.md`. `None` for any other name, or a day of 0 or
    /// past [`LAST_DAY`].
    pub fn day_of_file(name: &str) -> Option<u16> {
        if name == "welcome.md" {
            return Some(0);
        }
        let digits = name.strip_prefix("welcome-day-")?.strip_suffix(".md")?;
        if digits.starts_with('0') || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        digits
            .parse()
            .ok()
            .filter(|day| (1..=LAST_DAY).contains(day))
    }

    /// The same step, sent `day` days after the reader joined.
    #[must_use]
    pub fn on_day(self, day: u16) -> Self {
        Self { day, ..self }
    }

    /// How many days after joining it is sent: 0 for at once.
    pub fn day(&self) -> u16 {
        self.day
    }

    /// The subject line.
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// The text, before the footer every newsletter email gets.
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// Where the welcome steps are read from each time they are needed, so a
/// writer's edit reaches the next reader without a restart.
pub trait WelcomeFiles: fmt::Debug + Send + Sync {
    /// The welcome steps of `list` as they are now, by day, each day once.
    fn steps<'a>(&'a self, list: &'a Slug) -> BoxFuture<'a, Vec<Welcome>>;
}

/// Steps held in memory, for tests and for programs that read them once.
impl WelcomeFiles for BTreeMap<Slug, Vec<Welcome>> {
    fn steps<'a>(&'a self, list: &'a Slug) -> BoxFuture<'a, Vec<Welcome>> {
        let mut steps = self.get(list).cloned().unwrap_or_default();
        steps.sort_by_key(Welcome::day);
        steps.dedup_by_key(|step| step.day);
        Box::pin(std::future::ready(steps))
    }
}

/// The welcome sequences of a site's newsletters, and the postal address
/// they end with: no welcome goes out without one.
#[derive(Debug, Clone)]
pub struct Welcomes {
    files: Arc<dyn WelcomeFiles>,
    postal: PostalAddress,
}

impl Welcomes {
    /// The welcome steps `files` holds, each ending with `postal`.
    pub fn new(files: impl WelcomeFiles + 'static, postal: PostalAddress) -> Self {
        Self {
            files: Arc::new(files),
            postal,
        }
    }

    /// The welcome steps of `list` as they are now, by day.
    pub async fn of(&self, list: &Slug) -> Vec<Welcome> {
        self.files.steps(list).await
    }

    /// The postal address they end with.
    pub fn postal(&self) -> &PostalAddress {
        &self.postal
    }
}
