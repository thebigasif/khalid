//! The meta description of a page whose writer has no field for one, made
//! from the page's own first paragraph: a glossary term is described by
//! its definition. Without it the term's page has no description, which
//! search results and Lighthouse's SEO check both miss.

/// The most characters a description holds, the linter's default for its
/// `long-description` rule.
const MOST: usize = 160;

/// The first paragraph of the rendered `html` as plain text, cut after the
/// last whole sentence that fits in [`MOST`] characters. A first sentence
/// longer than that is cut after a word and ends with `…`. Empty when
/// `html` holds no text.
pub(crate) fn summary(html: &str) -> String {
    let text = paragraph_text(html);
    let mut summary = String::new();
    let mut sentences_end = 0;
    let mut length = 0;
    for word in text.split_whitespace() {
        let added = word.chars().count() + usize::from(length > 0);
        if length + added > MOST {
            return match summary.get(..sentences_end) {
                Some(sentences) if sentences_end > 0 => sentences.to_owned(),
                _ => format!("{summary}…"),
            };
        }
        if length > 0 {
            summary.push(' ');
        }
        summary.push_str(word);
        length += added;
        if word.ends_with(['.', '!', '?']) {
            sentences_end = summary.len();
        }
    }
    summary
}

/// The text of the first `<p>` of `html`, or of all of it when it has none,
/// with its tags left out and its entities read.
fn paragraph_text(html: &str) -> String {
    let start = html.find("<p>").or_else(|| html.find("<p ")).unwrap_or(0);
    let rest = html.get(start..).unwrap_or(html);
    let inner = rest
        .get(..rest.find("</p>").unwrap_or(rest.len()))
        .unwrap_or(rest);
    let mut text = String::with_capacity(inner.len());
    let mut in_tag = false;
    for c in inner.chars() {
        match c {
            '<' => in_tag = true,
            '>' if in_tag => in_tag = false,
            _ if !in_tag => text.push(c),
            _ => {}
        }
    }
    decoded(&text)
}

/// `text` with each entity, such as `&amp;` or `&#39;`, as its character.
/// An `&` that starts no entity stays as it is.
fn decoded(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find('&') {
        out.push_str(rest.get(..at).unwrap_or_default());
        let after = rest.get(at + 1..).unwrap_or_default();
        let entity = after
            .find(';')
            .and_then(|end| after.get(..end))
            .and_then(|name| character(name).map(|c| (c, name.len())));
        if let Some((c, length)) = entity {
            out.push(c);
            rest = after.get(length + 1..).unwrap_or_default();
        } else {
            out.push('&');
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// The character an entity named `name`, without its `&` and `;`, stands
/// for, among those the Markdown renderer writes.
fn character(name: &str) -> Option<char> {
    match name {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        "nbsp" => Some(' '),
        _ => {
            let number = name.strip_prefix('#')?;
            let code = match number.strip_prefix(['x', 'X']) {
                Some(hex) => u32::from_str_radix(hex, 16).ok()?,
                None => number.parse().ok()?,
            };
            char::from_u32(code)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::summary;

    #[test]
    fn keeps_the_whole_sentences_of_the_first_paragraph_that_fit() {
        let html = "<p>One <a href=\"/blog/x/\">short</a> sentence. Another one.</p>\n\
                    <p>A second paragraph.</p>";
        assert_eq!(summary(html), "One short sentence. Another one.");
        let long = format!("<p>First sentence. {}.</p>", "word ".repeat(40).trim_end());
        assert_eq!(summary(&long), "First sentence.");
    }

    #[test]
    fn cuts_a_first_sentence_too_long_for_results_after_a_word() {
        let html = format!("<p>{}</p>", "word ".repeat(40));
        let cut = summary(&html);
        assert!(cut.ends_with("word…"), "{cut}");
        assert!(cut.chars().count() <= 161, "{cut}");
    }

    #[test]
    fn reads_entities_and_leaves_out_tags() {
        let html = "<p>Tom &amp; Jerry&#39;s &quot;<em>best</em>&quot; &lt;tip&gt; &#x2013; R&D &bogus;.</p>";
        assert_eq!(
            summary(html),
            "Tom & Jerry's \"best\" <tip> \u{2013} R&D &bogus;."
        );
    }

    #[test]
    fn is_empty_without_text() {
        assert_eq!(summary(""), "");
        assert_eq!(
            summary("<figure><img src=\"/a.png\" alt=\"\"></figure>"),
            ""
        );
    }
}
