//! [`audit_json_ld`]: hand-written JSON-LD checked against the whole
//! schema.org vocabulary, beyond the names and shapes [`CustomSchema`]
//! refuses.
//!
//! Where [`CustomSchema`] stops at the first mistake, the audit reads any
//! JSON-LD it is given and lists every finding: a type or property schema.org
//! does not have, a property on a type it does not belong to, a value that is
//! not what the property expects (a date, a number, a web address, an
//! allowed enumeration value or a type that fits), an `@id` that points to
//! nothing, a name schema.org has replaced, and a value the page's visible
//! text never shows. A finding that has one clear fix carries it, such as
//! "change `pricecurrency` to `priceCurrency`".
//!
//! [`CustomSchema`]: crate::CustomSchema

mod google;
mod tree;
mod values;
mod visible;

use khalid_core::UserMessage;
use serde::Serialize;

use crate::SeoError;

/// Which report a finding belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SchemaReport {
    /// Whether the markup is correct schema.org.
    SchemaOrg,
    /// Whether Google can show the markup as a rich result.
    Google,
}

/// How much a finding matters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum FindingLevel {
    /// Search engines skip or misread what it is about.
    Error,
    /// Search engines may still read it, but it is weaker or risky.
    Warning,
}

/// What a fix changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum FixTarget {
    /// The name of the property at the path.
    Property,
    /// The `@type` at the path.
    Type,
    /// The text value at the path.
    Value,
}

/// One change that settles a finding, such as renaming `pricecurrency` to
/// `priceCurrency`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchemaFix {
    /// What the fix changes.
    pub target: FixTarget,
    /// Where, as in [`SchemaFinding::path`].
    pub path: String,
    /// The name or value as written.
    pub from: String,
    /// The name or value it becomes.
    pub to: String,
}

impl SchemaFix {
    /// The fix as a button says it, such as "Change pricecurrency to
    /// priceCurrency".
    pub fn label(&self) -> String {
        format!("Change {} to {}", self.from, self.to)
    }
}

/// One thing the audit found.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchemaFinding {
    /// The report it belongs to.
    pub report: SchemaReport,
    /// How much it matters.
    pub level: FindingLevel,
    /// Where it is, such as `@graph[0].offers.priceCurrency`, or empty for
    /// the whole markup.
    pub path: String,
    /// What is wrong, why it matters and how to fix it.
    pub message: UserMessage,
    /// The one change that settles it, when there is one.
    pub fix: Option<SchemaFix>,
}

/// What the audit knows besides the markup.
#[derive(Debug, Clone, Copy, Default)]
pub struct AuditContext<'a> {
    /// The text a reader sees on the page, or `None` to skip the check of
    /// values against it.
    pub visible_text: Option<&'a str>,
    /// `@id`s defined elsewhere on the page, such as the site's
    /// organisation, which the markup may point to.
    pub known_ids: &'a [String],
}

/// Every finding in the JSON-LD `text`, in the order they are found. Blank
/// text has none.
///
/// # Errors
///
/// [`SeoError::CustomSchemaSyntax`] when `text` is not JSON.
pub fn audit_json_ld(
    text: &str,
    context: &AuditContext<'_>,
) -> Result<Vec<SchemaFinding>, SeoError> {
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    let root = tree::read(text)?;
    let mut audit = tree::Audit::default();
    audit.root(&root);
    let mut findings = audit.finish(context.known_ids);
    if let Some(visible) = context.visible_text {
        findings.extend(visible::unseen(&root, visible));
    }
    Ok(findings)
}

/// The two reports on one piece of markup: is it correct schema.org, and
/// can Google show it as a rich result.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct SchemaReports {
    /// The `@type` of each node at the top, in the order written.
    pub types: Vec<String>,
    /// Whether the markup is correct schema.org: names, values, nested
    /// types and references.
    pub schema_org: Vec<SchemaFinding>,
    /// Whether Google can show it: the properties Google needs and
    /// recommends, and values the page does not show.
    pub google: Vec<SchemaFinding>,
}

/// Both reports on the JSON-LD `text`. Blank text has empty reports.
///
/// # Errors
///
/// [`SeoError::CustomSchemaSyntax`] when `text` is not JSON.
pub fn schema_reports(text: &str, context: &AuditContext<'_>) -> Result<SchemaReports, SeoError> {
    if text.trim().is_empty() {
        return Ok(SchemaReports::default());
    }
    let root = tree::read(text)?;
    let mut audit = tree::Audit::default();
    audit.root(&root);
    let mut google = google::google_findings(&root);
    if let Some(visible) = context.visible_text {
        google.extend(visible::unseen(&root, visible));
    }
    Ok(SchemaReports {
        types: google::top_types(&root),
        schema_org: audit.finish(context.known_ids),
        google,
    })
}

/// A finding of `report` and `level` at `path`.
pub(crate) fn finding(
    (report, level): (SchemaReport, FindingLevel),
    path: &str,
    (what, why, fix): (String, &'static str, String),
) -> SchemaFinding {
    SchemaFinding {
        report,
        level,
        path: path.to_owned(),
        message: UserMessage::new(what, why, fix),
        fix: None,
    }
}
