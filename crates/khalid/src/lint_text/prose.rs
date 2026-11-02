//! The writer's own prose in a rendered post, for the readability rules:
//! the plain text without code, quotes, tables and the parts Khalid writes
//! itself, such as the table of contents or a bookmark card's summary of
//! another site. A long code line is not a long sentence, and a quoted
//! source's passive voice is not the writer's.

use super::plain_text;

/// Elements whose text is not the writer's prose: code, quotes, tables,
/// navigation, forms and maths.
const SKIPPED: &[&str] = &[
    "aside",
    "blockquote",
    "code",
    "form",
    "kbd",
    "math",
    "nav",
    "pre",
    "samp",
    "script",
    "style",
    "table",
];

/// The left-out elements that sit inside a sentence.
const INLINE: &[&str] = &["code", "kbd", "math", "samp"];

/// Classes of the parts Khalid writes itself, or copies from elsewhere:
/// a bookmark card, a file link, a pull quote that repeats the text, the
/// note on gated content and the table of contents.
const SKIPPED_CLASSES: &[&str] = &["bookmark", "file", "gated-note", "pullquote", "toc"];

/// The plain text of the prose in `html`, one line per paragraph, heading
/// or list item, as [`plain_text`] gives it. Each element left out ends a
/// line, so the text around it never runs together.
pub(crate) fn prose_text(html: &str) -> String {
    let mut kept = String::with_capacity(html.len());
    let mut skipping: Option<(String, usize)> = None;
    let mut rest = html;
    while let Some((before, after)) = rest.split_once('<') {
        let Some((tag, tail)) = after.split_once('>') else {
            break;
        };
        rest = tail;
        let closing = tag.starts_with('/');
        let lone = tag.ends_with('/');
        let name = tag
            .trim_start_matches('/')
            .split([' ', '/', '\n', '\t'])
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if let Some((open, depth)) = &mut skipping {
            if *open == name && !lone {
                *depth = if closing {
                    depth.saturating_sub(1)
                } else {
                    depth.saturating_add(1)
                };
            }
            if *depth == 0 {
                skipping = None;
            }
            continue;
        }
        kept.push_str(before);
        if !closing && !lone && (SKIPPED.contains(&name.as_str()) || skipped_class(tag)) {
            // Inline code drops out of its sentence; a block ends a line.
            let inline = INLINE.contains(&name.as_str());
            kept.push_str(if inline { " " } else { "<br>" });
            skipping = Some((name, 1));
        } else {
            kept.push('<');
            kept.push_str(tag);
            kept.push('>');
        }
    }
    if skipping.is_none() {
        kept.push_str(rest);
    }
    plain_text(&kept)
}

/// Whether the opening tag `tag` has a class Khalid writes itself.
fn skipped_class(tag: &str) -> bool {
    let Some((_, after)) = tag.split_once("class=\"") else {
        return false;
    };
    let classes = after.split('"').next().unwrap_or_default();
    classes
        .split_whitespace()
        .any(|class| SKIPPED_CLASSES.contains(&class))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(html: &str) -> Vec<String> {
        prose_text(html)
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
            .filter(|line| !line.is_empty())
            .collect()
    }

    #[test]
    fn code_quotes_and_tables_are_left_out_and_prose_stays() {
        let html = "<p>Run the <code>khalid build</code> command.</p>\
                    <pre><code>let x = 1;\nlet y = 2;</code></pre>\
                    <blockquote><p>Links are <em>written</em> by people.</p></blockquote>\
                    <table><tr><td>Zeile</td></tr></table><p>Fertig.</p>";
        assert_eq!(lines(html), ["Run the command.", "Fertig."]);
    }

    #[test]
    fn the_parts_khalid_writes_itself_are_left_out() {
        let html = "<nav class=\"toc\" aria-label=\"Contents\"><ol><li>Why</li></ol></nav>\
                    <h2 id=\"why\">Why</h2><figure class=\"bookmark\"><div><p>Their summary.</p></div></figure>\
                    <p class=\"gated-note\">Subscribe to read on.</p><p>Our own words.</p>";
        assert_eq!(lines(html), ["Why", "Our own words."]);
    }
}
