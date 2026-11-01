//! Reading the layout blocks: pull quote, pros and cons, and the folding
//! table.

use std::num::NonZeroU8;

use super::Attributes;
use crate::block::markdown_blocks;
use crate::{BlockError, BlockKind, ProsConsBlock};

/// Reads a folding table: how many rows show, the line that opens the
/// rest, and one markdown table as the body.
pub(super) fn table(attributes: &Attributes, body: &[&str]) -> Result<BlockKind, BlockError> {
    attributes.only("table", &["fold", "more"], "fold and more")?;
    let fold = attributes.parsed(
        "fold",
        |text| text.parse::<NonZeroU8>().ok().filter(|fold| fold.get() <= 100),
        "a table's fold is how many rows show before the rest fold away, a whole number from 1 to 100",
    )?;
    let blocks = markdown_blocks(body);
    if !matches!(blocks.as_slice(), [only] if only.kind == BlockKind::Table) {
        return Err(BlockError::Value {
            line: attributes.line,
            problem: "a table block holds one markdown table: a header row, a line of dashes, then one row per line",
        });
    }
    Ok(BlockKind::FoldingTable {
        fold: fold.unwrap_or(DEFAULT_FOLD),
        more: attributes.get("more"),
    })
}

/// How many rows show when a table block does not say.
const DEFAULT_FOLD: NonZeroU8 = match NonZeroU8::new(8) {
    Some(fold) => fold,
    None => NonZeroU8::MIN,
};

/// Reads a pull quote: who said it, and its words as the body.
pub(super) fn pull_quote(attributes: &Attributes, body: &[&str]) -> Result<BlockKind, BlockError> {
    attributes.only("pullquote", &["by"], "by")?;
    if body.iter().all(|line| line.trim().is_empty()) {
        return Err(BlockError::Missing {
            line: attributes.line,
            problem: "a pull quote needs words, written between its opening and closing ::: lines",
        });
    }
    Ok(BlockKind::PullQuote {
        by: attributes.get("by"),
    })
}

/// Reads pros and cons: the two titles, then one `+ ` line per pro and one
/// `- ` line per con. Blank lines are fine.
pub(super) fn pros_cons(attributes: &Attributes, body: &[&str]) -> Result<BlockKind, BlockError> {
    attributes.only("proscons", &["pros", "cons"], "pros and cons")?;
    let (mut pros, mut cons) = (Vec::new(), Vec::new());
    for (number, text) in (attributes.line.saturating_add(1)..).zip(body) {
        let text = text.trim();
        if text.is_empty() {
            continue;
        }
        let item = |rest: &str| rest.trim().to_owned();
        if let Some(rest) = text.strip_prefix("+ ") {
            pros.push(item(rest));
        } else if let Some(rest) = text.strip_prefix("- ") {
            cons.push(item(rest));
        } else {
            return Err(BlockError::Value {
                line: number,
                problem: "each line of pros and cons starts with + for a pro or - for a con, then a space",
            });
        }
    }
    if pros.is_empty() && cons.is_empty() {
        return Err(BlockError::Missing {
            line: attributes.line,
            problem: "pros and cons need at least one line, + for a pro or - for a con",
        });
    }
    Ok(BlockKind::ProsCons(ProsConsBlock {
        pros_title: attributes.get("pros").unwrap_or_else(|| "Pros".to_owned()),
        cons_title: attributes.get("cons").unwrap_or_else(|| "Cons".to_owned()),
        pros,
        cons,
    }))
}
