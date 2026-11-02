//! What the linter reads from a post's body besides its parsed parts: the
//! wikilinks as written, the plain words of the rendered HTML, and the
//! lists and tables under each heading. Kept apart from `lint_input.rs` so
//! that file stays under 600 lines (BL-SIZE-1).

use khalid_lint::SectionShape;

/// The target of each `[[target]]` and `[[target|text]]` in `body`, as
/// written, in order. Code, fenced or inline, holds none.
pub(crate) fn wikilinks(body: &str) -> Vec<String> {
    let mut targets = Vec::new();
    let mut fence: Option<&str> = None;
    for line in body.lines() {
        let trimmed = line.trim_start();
        let marker = ["```", "~~~"].into_iter().find(|m| trimmed.starts_with(m));
        match (fence, marker) {
            (None, Some(marker)) => fence = Some(marker),
            (Some(open), Some(marker)) if open == marker => fence = None,
            (None, None) => {
                // Text between backticks is inline code: every other piece.
                for prose in line.split('`').step_by(2) {
                    let mut rest = prose;
                    while let Some((_, after)) = rest.split_once("[[") {
                        let Some((inside, tail)) = after.split_once("]]") else {
                            break;
                        };
                        let target = inside.split('|').next().unwrap_or_default().trim();
                        if !target.is_empty() {
                            targets.push(target.to_owned());
                        }
                        rest = tail;
                    }
                }
            }
            _ => {}
        }
    }
    targets
}

/// The elements whose end also ends a line of text, such as a heading or a
/// list item written without a full stop.
const BLOCKS: &[&str] = &[
    "p",
    "li",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "td",
    "th",
    "dt",
    "dd",
    "blockquote",
    "figcaption",
    "pre",
];

/// Whether the tag written `tag`, without its `<` and `>`, ends a line:
/// the end of a block, or a `br`.
fn ends_line(tag: &str) -> bool {
    let (closing, rest) = match tag.strip_prefix('/') {
        Some(rest) => (true, rest),
        None => (false, tag),
    };
    let name = rest.split([' ', '/']).next().unwrap_or_default();
    name.eq_ignore_ascii_case("br") || (closing && BLOCKS.contains(&name))
}

/// The words of rendered HTML with every tag and every entity, such as
/// `&quot;`, replaced by a space, so neither is read as a word. The end of
/// a block becomes a line break, so a heading never runs into the next
/// sentence. A formula's `MathML` is left out, so its letters and signs are
/// never read as words; a maths block's caption stays. A chart's drawing
/// and its table for screen readers are left out the same way: they are
/// the chart's data, and its caption, which stays, says what and when.
pub(crate) fn plain_text(html: &str) -> String {
    let mut text = String::with_capacity(html.len());
    let mut tag = String::new();
    let mut skip_until = None;
    let mut in_maths: Option<&str> = None;
    for c in html.chars() {
        match (skip_until, c) {
            (Some(end), _) if c == end => {
                skip_until = None;
                if end == '>' && in_maths.is_some_and(|closing| tag == closing) {
                    in_maths = None;
                } else if end == '>' && in_maths.is_none() {
                    in_maths = left_out(&tag);
                }
                text.push(if end == '>' && ends_line(&tag) {
                    '\n'
                } else {
                    ' '
                });
                tag.clear();
            }
            (Some('>'), _) => tag.push(c),
            (Some(_), _) => {}
            (None, '<') => skip_until = Some('>'),
            (None, '&') => skip_until = Some(';'),
            (None, _) if in_maths.is_some() => {}
            (None, _) => text.push(c),
        }
    }
    text
}

/// The closing tag of the element the opening `tag` starts when its words
/// are left out of the text: a formula, a chart's drawing or its table.
fn left_out(tag: &str) -> Option<&'static str> {
    match tag.split(' ').next() {
        Some("math") => Some("/math"),
        Some("svg") => Some("/svg"),
        Some("table") if tag.contains("class=\"visually-hidden\"") => Some("/table"),
        _ => None,
    }
}

/// The lists and tables under each heading of `html`, in heading order:
/// what sits between one `<h1>` to `<h6>` and the next. The plain text
/// the rules read has lost the bullets and pipes, so W6 reads these.
pub(crate) fn section_shapes(html: &str) -> Vec<Vec<SectionShape>> {
    let mut sections = Vec::new();
    let mut rest = html.get(next_heading(html).unwrap_or(html.len())..);
    while let Some(section) = rest.and_then(|from| from.get(1..)) {
        let end = next_heading(section).unwrap_or(section.len());
        let inside = section.get(..end).unwrap_or_default();
        let mut holds = Vec::new();
        if inside.contains("<ul") || inside.contains("<ol") {
            holds.push(SectionShape::List);
        }
        if inside.contains("<table") {
            holds.push(SectionShape::Table);
        }
        sections.push(holds);
        rest = section.get(end..).filter(|left| !left.is_empty());
    }
    sections
}

/// Where the next heading tag of `html` starts.
fn next_heading(html: &str) -> Option<usize> {
    ["<h1", "<h2", "<h3", "<h4", "<h5", "<h6"]
        .iter()
        .filter_map(|tag| html.find(tag))
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_heading_holds_the_lists_and_tables_before_the_next() {
        let html = "<p>Intro</p><ul><li>not under a heading</li></ul>\
                    <h2>Types</h2><ul><li>a</li></ul><h3>Versus</h3><p>none</p>\
                    <h2>Both</h2><ol><li>a</li></ol><table><tr><td>b</td></tr></table>";
        assert_eq!(
            section_shapes(html),
            [
                vec![SectionShape::List],
                vec![],
                vec![SectionShape::List, SectionShape::Table],
            ]
        );
    }

    #[test]
    fn plain_text_drops_tags_and_entities() {
        let html = "<p>Write a <em>&quot;title&quot;</em> tag &amp; test it.</p>";
        let text = plain_text(html);
        let words: Vec<&str> = text.split_whitespace().collect();
        assert_eq!(words, ["Write", "a", "title", "tag", "test", "it."]);
    }

    #[test]
    fn plain_text_ends_a_line_at_each_block_only() {
        let html = "<h2 id=\"why\">Why links</h2><ul><li>One <a href=\"/a/\">link</a></li><li>Two<br/>three</li></ul>";
        let lines: Vec<String> = plain_text(html)
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
            .filter(|line| !line.is_empty())
            .collect();
        assert_eq!(lines, ["Why links", "One link", "Two", "three"]);
    }

    #[test]
    fn plain_text_leaves_out_formulas_and_keeps_their_captions() {
        let html = "<p>Growth <math><mi>x</mi></math> compounds.</p>\n<figure class=\"maths\">\n<math display=\"block\"><mi>A</mi><mo>=</mo><mi>P</mi></math>\n<figcaption>Compound growth</figcaption>\n</figure>\n";
        let words: Vec<String> = plain_text(html)
            .split_whitespace()
            .map(str::to_owned)
            .collect();
        assert_eq!(words, ["Growth", "compounds.", "Compound", "growth"]);
    }

    #[test]
    fn plain_text_leaves_out_a_charts_drawing_and_table_and_keeps_its_caption() {
        let html = "<figure class=\"chart chart-bar\">\n<figcaption>Visits in 2026</figcaption>\n<svg class=\"chart-drawing\" width=\"100%\" height=\"50\" aria-hidden=\"true\"><text>Home</text><text>4.2k</text></svg>\n<table class=\"visually-hidden\">\n<caption>Visits in 2026</caption>\n<tr><th>Home</th><td>4.2k</td></tr>\n</table>\n</figure>\n<table><tr><td>Kept</td></tr></table>";
        let words: Vec<String> = plain_text(html)
            .split_whitespace()
            .map(str::to_owned)
            .collect();
        assert_eq!(words, ["Visits", "in", "2026", "Kept"]);
    }

    #[test]
    fn wikilinks_are_read_as_written_outside_code() {
        let body = "See [[why-links]] and [[link-text|good link text]].\n\n\
                    Write `[[not-a-link]]` like this.\n\n```\n[[in-a-fence]]\n```\n\n\
                    ~~~\n```\n[[still-fenced]]\n~~~\n\nLast [[ seo ]] and [[]] and [[open";
        assert_eq!(wikilinks(body), ["why-links", "link-text", "seo"]);
    }
}
