//! Reading the chart block: its kind, caption and numbers.

use super::Attributes;
use crate::chart::{self, ChartBlock};
use crate::{BlockError, BlockKind};

/// Reads a chart: its kind, title, unit and file, and the lines of its
/// `body`, or no lines when `src` names a file beside the post.
pub(super) fn chart(attributes: &Attributes, body: &[&str]) -> Result<BlockKind, BlockError> {
    attributes.only(
        "chart",
        &["type", "title", "unit", "src"],
        "type, title, unit and src",
    )?;
    let kind = chart::kind(attributes.get("type").as_deref()).ok_or(BlockError::Value {
        line: attributes.line,
        problem: "a chart's type is bar, column or line",
    })?;
    let title = attributes.need(
        "title",
        "a chart needs title, one line that says what the numbers show",
    )?;
    let src = attributes.get("src");
    if src.as_deref().is_some_and(|src| !chart::is_file_name(src)) {
        return Err(BlockError::Value {
            line: attributes.line,
            problem: "a chart's src is a CSV file beside the post, such as traffic.csv",
        });
    }
    let first = attributes.line.saturating_add(1);
    let table =
        chart::table(body, first).map_err(|(line, problem)| BlockError::Value { line, problem })?;
    if table.rows.is_empty() && src.is_none() {
        return Err(BlockError::Missing {
            line: attributes.line,
            problem: "a chart needs numbers: a header line and one line per row between its ::: lines, or src, a CSV file beside the post",
        });
    }
    Ok(BlockKind::Chart(ChartBlock {
        kind,
        title,
        unit: attributes.get("unit"),
        src,
        table,
    }))
}
