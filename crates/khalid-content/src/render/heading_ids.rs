//! The ids written on each heading of a body, and the list of headings.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Mutex, PoisonError};

use comrak::adapters::{HeadingAdapter, HeadingMeta};
use comrak::nodes::Sourcepos;
use khalid_core::{Locale, Slug, SlugMode};

use super::Heading;

/// Writes `<hN id="...">` and records each heading as it is written.
pub(super) struct HeadingIds {
    pub(super) locale: Locale,
    pub(super) slugs: SlugMode,
    pub(super) state: Mutex<IdState>,
}

/// The headings written so far, and how often each id has been used.
#[derive(Default)]
pub(super) struct IdState {
    pub(super) headings: Vec<Heading>,
    seen: BTreeMap<String, usize>,
}

impl HeadingAdapter for HeadingIds {
    fn enter(
        &self,
        output: &mut dyn fmt::Write,
        heading: &HeadingMeta,
        _: Option<Sourcepos>,
    ) -> fmt::Result {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let base = Slug::from_title(&heading.content, &self.locale, self.slugs)
            .map_or_else(|_| "section".to_owned(), |slug| slug.as_str().to_owned());
        let count = state.seen.entry(base.clone()).or_insert(0);
        *count += 1;
        let id = if *count == 1 {
            base
        } else {
            format!("{base}-{count}")
        };
        write!(output, "<h{} id=\"{id}\">", heading.level)?;
        state.headings.push(Heading {
            level: heading.level,
            id,
            text: heading.content.clone(),
        });
        Ok(())
    }

    fn exit(&self, output: &mut dyn fmt::Write, heading: &HeadingMeta) -> fmt::Result {
        write!(output, "</h{}>", heading.level)
    }
}
