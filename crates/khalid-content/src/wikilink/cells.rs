//! Wikilinks in table cells. A table splits its rows at every `|` before
//! it reads links, so `[[slug|text]]` in a cell would become two cells.
//! GitHub keeps a cell whole where its `|` is written `\|`, and comrak
//! reads `[[slug\|text]]` in a cell as the labelled link. So each `|`
//! inside `[[ ]]` on a table row is written `\|` before the parse.

use std::borrow::Cow;

/// `markdown` with each `|` inside `[[ ]]` on a table row written `\|`,
/// so the link keeps its label and the table its columns. A `|` already
/// written `\|`, a `|` outside `[[ ]]`, and every line outside a table are
/// left as they are. Lines and their numbers do not change.
pub(crate) fn guard_cells(markdown: &str) -> Cow<'_, str> {
    if !markdown.contains("[[") {
        return Cow::Borrowed(markdown);
    }
    let lines: Vec<&str> = markdown.split_inclusive('\n').collect();
    let rows = table_rows(&lines);
    if !rows.contains(&true) {
        return Cow::Borrowed(markdown);
    }
    let mut guarded = String::with_capacity(markdown.len().saturating_add(8));
    for (line, row) in lines.into_iter().zip(rows) {
        if row {
            escape_links(line, &mut guarded);
        } else {
            guarded.push_str(line);
        }
    }
    Cow::Owned(guarded)
}

/// Whether each of `lines` is a row of a table: the header above a
/// delimiter row such as `| --- | :-: |`, the delimiter row, and every
/// line after it up to a blank line. Fenced code holds no table.
fn table_rows(lines: &[&str]) -> Vec<bool> {
    let mut rows = vec![false; lines.len()];
    let mut fence: Option<&str> = None;
    let mut in_table = false;
    for (at, line) in lines.iter().enumerate() {
        let text = unquoted(line);
        let marker = ["```", "~~~"].into_iter().find(|m| text.starts_with(m));
        if let Some(open) = fence {
            if marker == Some(open) {
                fence = None;
            }
            continue;
        }
        if marker.is_some() {
            fence = marker;
            in_table = false;
        } else if text.trim().is_empty() {
            in_table = false;
        } else if in_table {
            set(&mut rows, at);
        } else if is_delimiter(text) {
            let header = at.checked_sub(1);
            let above = header.and_then(|h| lines.get(h)).map(|h| unquoted(h));
            if above.is_some_and(|h| h.contains('|')) {
                in_table = true;
                set(&mut rows, at);
                if let Some(header) = header {
                    set(&mut rows, header);
                }
            }
        }
    }
    rows
}

/// Marks line `at` of `rows` as a table row.
fn set(rows: &mut [bool], at: usize) {
    if let Some(row) = rows.get_mut(at) {
        *row = true;
    }
}

/// `line` without its indent and the `>` marks of a quote around it.
fn unquoted(line: &str) -> &str {
    line.trim_start_matches(|c: char| c == '>' || c.is_whitespace())
}

/// Whether `text` is a table's delimiter row: only `|`, `-`, `:` and
/// spaces, with at least one `|` and one `-`.
fn is_delimiter(text: &str) -> bool {
    let text = text.trim();
    text.contains('|')
        && text.contains('-')
        && text
            .chars()
            .all(|c| matches!(c, '|' | '-' | ':' | ' ' | '\t'))
}

/// Appends `line` to `out` with each unescaped `|` inside `[[ ]]` written
/// `\|`.
fn escape_links(line: &str, out: &mut String) {
    let mut rest = line;
    while let Some((before, after)) = rest.split_once("[[") {
        let Some((inside, tail)) = after.split_once("]]") else {
            break;
        };
        out.push_str(before);
        out.push_str("[[");
        let mut escaped = false;
        for c in inside.chars() {
            if c == '|' && !escaped {
                out.push('\\');
            }
            out.push(c);
            escaped = c == '\\' && !escaped;
        }
        out.push_str("]]");
        rest = tail;
    }
    out.push_str(rest);
}

#[cfg(test)]
mod tests {
    use super::guard_cells;

    #[test]
    fn a_pipe_inside_a_link_on_a_table_row_is_escaped() {
        let text = "| a | b |\n|---|---|\n| [[x|y]] | [[z\\|w]] |\n\nLast [[x|y]] here.\n";
        assert_eq!(
            guard_cells(text),
            "| a | b |\n|---|---|\n| [[x\\|y]] | [[z\\|w]] |\n\nLast [[x|y]] here.\n"
        );
    }

    #[test]
    fn a_table_in_a_quote_and_the_header_row_are_guarded() {
        let text = "> | [[x|y]] | b |\n> | :-- | --: |\n> | 1 | 2 |\n";
        assert_eq!(
            guard_cells(text),
            "> | [[x\\|y]] | b |\n> | :-- | --: |\n> | 1 | 2 |\n"
        );
    }

    #[test]
    fn text_without_a_table_or_in_a_fence_is_left_alone() {
        for text in [
            "a | b [[x|y]]\n---\n",
            "```\n| [[x|y]] |\n|---|\n```\n",
            "| a | b |\n|---|---|\n| 1 | 2 |\n",
            "Heading [[x|y]]\n---\n",
        ] {
            assert!(
                matches!(guard_cells(text), std::borrow::Cow::Borrowed(_)),
                "{text}"
            );
        }
    }
}
