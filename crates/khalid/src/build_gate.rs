//! The publish gate in `khalid build` and `khalid serve`: a post the gate
//! blocks is left out of the site, and the rest of the site goes live, so
//! one old post never empties a site and the two commands always agree on
//! what is published (KH-LNT-1).
//!
//! A post with no date is a draft. Neither command shows it, so the gate
//! leaves it alone too. A case study or changelog entry under `content/`
//! is always shown, with or without a date, so the gate judges every one;
//! its drafts wait under `drafts/`, which neither command reads.

use std::borrow::Cow;
use std::path::{Path, PathBuf};

use khalid_content::SiteContent;
use khalid_core::{Clock, Config, UserMessage};
use khalid_lint::{Linter, LocationKind, PublishGate, Report, Verdict};
use khalid_routes::Routes;

use crate::lint_input::{self, LintInput};
use crate::{categories, lint, lint_message, site_briefs};

/// A dated post, a case study or a changelog entry the gate keeps off the
/// site, and the report saying why.
#[derive(Debug)]
pub(crate) struct Refused {
    /// The post's file, relative to the site folder.
    pub(crate) path: PathBuf,
    /// Every finding about the post, warnings included.
    pub(crate) report: Report,
}

/// Every dated post, case study and changelog entry of `content` the gate
/// refuses, judged with the `site`
/// folder's `lint.toml`, briefs and categories and the content's fact
/// registry, on today's date from `clock`.
///
/// # Errors
///
/// What, why and fix when `lint.toml`, a brief or `categories.toml` cannot
/// be read.
pub(crate) fn screen(
    site: &Path,
    (config, routes, content): (&Config, &Routes, &SiteContent),
    clock: &dyn Clock,
) -> Result<Vec<Refused>, UserMessage> {
    let settings = lint::lint_settings(site)?;
    let named = categories::of(site)?;
    lint::reviewed_categories(&settings, &named, content)?;
    let linter = Linter::new(&settings, clock).map_err(|error| lint_message::of(&error))?;
    let input = LintInput::new(content, config, routes.clone())
        .with_briefs(site_briefs::load(site)?)
        .with_categories(named);
    Ok(refused(&linter, content, config, &input))
}

/// Each dated post in `content` that `gate` refuses, in the order the
/// posts were read, then each case study and changelog entry it refuses.
/// Empty when everything may go live. Each post first gets the quick yes
/// or no, and only a refused post the full verdict with its warnings, so a
/// 10,000-post site stays inside its budget. Posts are held to the
/// content's fact registry and to their briefs in `input`.
pub(crate) fn refused(
    gate: &dyn PublishGate,
    content: &SiteContent,
    config: &Config,
    input: &LintInput,
) -> Vec<Refused> {
    let (site, documents) = lint_input::lint_site(content, config, input);
    let posts = (site.posts.iter().zip(documents))
        .filter(|(post, _)| post.date.is_some() && gate.refuses(post, &site))
        .map(|(post, document)| (gate.verdict(post, &site), document.path.as_path()));
    let entries = (lint_input::entry_files(content, config).into_iter())
        .map(|(entry, path)| (gate.entry_verdict(&entry), path));
    (posts.chain(entries))
        .filter_map(|(verdict, path)| match verdict {
            Verdict::Refuse { report } => Some(Refused {
                path: path.to_path_buf(),
                report,
            }),
            Verdict::Publish { .. } => None,
        })
        .collect()
}

/// `content` without the `refused` posts, case studies and changelog
/// entries. Borrowed as it is when the gate refused none.
pub(crate) fn published<'c>(content: &'c SiteContent, refused: &[Refused]) -> Cow<'c, SiteContent> {
    if refused.is_empty() {
        return Cow::Borrowed(content);
    }
    let kept_file = |path: &Path| !refused.iter().any(|left| left.path == path);
    let mut kept = content.clone();
    kept.posts.retain(|doc| kept_file(&doc.path));
    kept.case_studies.retain(|doc| kept_file(&doc.path));
    kept.changelog.retain(|doc| kept_file(&doc.path));
    Cow::Owned(kept)
}

/// `content` as `khalid serve` shows it: without the posts the gate
/// refuses, as [`screen`] judges them, so readers see what `khalid build`
/// writes. Logs each post left out.
///
/// # Errors
///
/// What, why and fix when the site's lint settings cannot be read.
pub(crate) fn for_readers<'c>(
    site: &Path,
    (config, routes, content): (&Config, &Routes, &'c SiteContent),
    clock: &dyn Clock,
) -> Result<Cow<'c, SiteContent>, UserMessage> {
    let refused = screen(site, (config, routes, content), clock)?;
    for post in &refused {
        let post = post.path.display();
        tracing::warn!(%post, "a post is not ready to publish, so readers do not see it; `khalid lint` says why");
    }
    Ok(published(content, &refused))
}

/// Every finding of each post in `refused`, then what was left out, why
/// and the fix, for a person to read after a build. Empty when nothing was
/// left out.
pub(crate) fn human(refused: &[Report]) -> String {
    let mut text = String::new();
    if refused.is_empty() {
        return text;
    }
    for finding in refused.iter().flat_map(Report::findings) {
        lint::write_finding(&mut text, finding);
    }
    let only_posts = (refused.iter().flat_map(Report::findings))
        .all(|finding| finding.location().kind == LocationKind::Post);
    let noun = if only_posts { "post" } else { "page" };
    let count = match refused.len() {
        1 => format!("1 {noun}, because it is"),
        pages => format!("{pages} {noun}s, because they are"),
    };
    format!(
        "{text}Left out {count} not ready to publish. The blocks are listed above.\nFix each block and build again, or move its file under drafts/<language>/ to keep it a draft.\n"
    )
}
