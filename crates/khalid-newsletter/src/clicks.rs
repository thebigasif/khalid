//! Click counting (G39): when a site counts clicks, each link in an email
//! leads through the blog first. Every reader's copy has the same link, so
//! a click counts against the email and the link, never a person.

use khalid_core::AbsoluteUrl;

/// Characters a link in plain text never holds, so it ends before them.
const ENDS_A_LINK: &[char] = &['<', '>', '"', '\'', '(', ')', '[', ']', '{', '}', '`'];

/// Characters that end a sentence rather than a link, when last.
const AFTER_A_LINK: &[char] = &['.', ',', ';', ':', '!', '?'];

/// The text of an email split into what stays and the links to count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Piece<'a> {
    /// Text that goes out as written.
    Text(&'a str),
    /// A web link, numbered from 1 by its first place in the text: the
    /// same address twice has one number.
    Link(u32),
}

/// `text` split into pieces, and each different link in it by its number,
/// in order. Only `http` and `https` links count.
pub(crate) fn pieces(text: &str) -> (Vec<Piece<'_>>, Vec<AbsoluteUrl>) {
    let (mut pieces, mut links): (Vec<Piece<'_>>, Vec<AbsoluteUrl>) = (Vec::new(), Vec::new());
    let mut rest = text;
    while let Some(start) = next_link(rest) {
        let (before, from) = rest.split_at(start);
        let length = from.find(|c: char| c.is_whitespace() || ENDS_A_LINK.contains(&c));
        let link = from.get(..length.unwrap_or(from.len())).unwrap_or(from);
        let link = link.trim_end_matches(AFTER_A_LINK);
        let Ok(url) = AbsoluteUrl::parse(link) else {
            let skip = from.len().min("http".len());
            pieces.push(Piece::Text(rest.get(..start + skip).unwrap_or(rest)));
            rest = from.get(skip..).unwrap_or_default();
            continue;
        };
        let number = links
            .iter()
            .position(|seen| seen == &url)
            .unwrap_or_else(|| {
                links.push(url);
                links.len().saturating_sub(1)
            });
        if !before.is_empty() {
            pieces.push(Piece::Text(before));
        }
        let number = u32::try_from(number).unwrap_or(u32::MAX).saturating_add(1);
        pieces.push(Piece::Link(number));
        rest = from.get(link.len()..).unwrap_or_default();
    }
    if !rest.is_empty() {
        pieces.push(Piece::Text(rest));
    }
    (pieces, links)
}

/// Where the next `http://` or `https://` link starts in `text`.
fn next_link(text: &str) -> Option<usize> {
    let secure = text.find("https://");
    let plain = text.find("http://");
    match (secure, plain) {
        (Some(secure), Some(plain)) => Some(secure.min(plain)),
        (found, None) | (None, found) => found,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn joined(pieces: &[Piece<'_>]) -> String {
        pieces
            .iter()
            .map(|piece| match piece {
                Piece::Text(text) => (*text).to_owned(),
                Piece::Link(number) => format!("<{number}>"),
            })
            .collect()
    }

    #[test]
    fn each_different_link_gets_a_number_and_the_text_around_it_stays() {
        let text = "Read it: https://www.example.com/blog/hello/.\n\
                    See [the guide](https://example.org/guide) and \
                    https://www.example.com/blog/hello/ again, or http://old.example.net";
        let (pieces, links) = pieces(text);
        assert_eq!(
            joined(&pieces),
            "Read it: <1>.\nSee [the guide](<2>) and <1> again, or <3>"
        );
        let links: Vec<_> = links.iter().map(AbsoluteUrl::as_str).collect();
        assert_eq!(links.len(), 3, "{links:?}");
        assert!(
            links[1].starts_with("https://example.org/guide"),
            "{links:?}"
        );
    }

    #[test]
    fn text_without_links_is_one_piece() {
        let (pieces, links) = pieces("No links here, only http talk.");
        assert_eq!(pieces, [Piece::Text("No links here, only http talk.")]);
        assert_eq!(links, []);
    }

    #[test]
    fn a_link_that_does_not_parse_stays_as_written() {
        let (pieces, links) = pieces("Broken: https:// and done");
        assert_eq!(joined(&pieces), "Broken: https:// and done");
        assert_eq!(links, []);
    }
}
