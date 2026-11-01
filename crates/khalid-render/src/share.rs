//! Share links under a post: a plain link to each network's own share
//! address, so they work with scripts off and nothing reaches a network
//! until a reader follows one (templates spec 4.2). No script, no image
//! and no counter comes from a network.

use minijinja::Value;
use minijinja::value::Serde;
use serde::Serialize;

/// The key of every network a share row may link to, in the order of the
/// docs. `khalid-theme` offers the same list as a template option.
pub const SHARE_NETWORKS: [&str; 8] = [
    "x",
    "linkedin",
    "facebook",
    "bluesky",
    "reddit",
    "hacker-news",
    "whatsapp",
    "email",
];

/// The name and share address of the network `key`, or `None` for a key
/// no network has. `{url}` and `{title}` are filled in, encoded.
fn network(key: &str) -> Option<(&'static str, &'static str)> {
    Some(match key {
        "x" => ("X", "https://x.com/intent/post?url={url}&text={title}"),
        "linkedin" => (
            "LinkedIn",
            "https://www.linkedin.com/sharing/share-offsite/?url={url}",
        ),
        "facebook" => (
            "Facebook",
            "https://www.facebook.com/sharer/sharer.php?u={url}",
        ),
        "bluesky" => (
            "Bluesky",
            "https://bsky.app/intent/compose?text={title}%20{url}",
        ),
        "reddit" => (
            "Reddit",
            "https://www.reddit.com/submit?url={url}&title={title}",
        ),
        "hacker-news" => (
            "Hacker News",
            "https://news.ycombinator.com/submitlink?u={url}&t={title}",
        ),
        "whatsapp" => ("WhatsApp", "https://wa.me/?text={title}%20{url}"),
        "email" => ("Email", "mailto:?subject={title}&body={url}"),
        _ => return None,
    })
}

/// One share link as templates read it.
#[derive(Debug, Serialize)]
struct ShareLink {
    /// The network's key, such as `email`, which a template may name in
    /// its own words.
    key: String,
    /// The network's own name, such as `LinkedIn`.
    name: &'static str,
    /// The share address with the page in it.
    href: String,
}

/// The template function `share_links(url, title, networks)`: a link for
/// each key of `networks`, in order, that shares the page at `url` titled
/// `title`. A key no network has is left out; the theme checks the keys
/// before any page renders.
#[expect(
    clippy::needless_pass_by_value,
    reason = "minijinja hands a template function a list argument as an owned Vec"
)]
pub(crate) fn share_links(url: &str, title: &str, networks: Vec<String>) -> Value {
    let (url, title) = (encode(url), encode(title));
    let link = |key: &String| {
        let (name, address) = network(key)?;
        let href = address.replace("{url}", &url).replace("{title}", &title);
        Some(ShareLink {
            key: key.clone(),
            name,
            href,
        })
    };
    Value::from(Serde(networks.iter().filter_map(link).collect::<Vec<_>>()))
}

/// `text` with every byte but letters, digits and `-._~` written as `%XX`,
/// so it is safe in any part of an address.
fn encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
            out.push(char::from(byte));
        } else {
            out.push('%');
            for nibble in [byte >> 4, byte & 0x0f] {
                let digit = char::from_digit(u32::from(nibble), 16).unwrap_or('0');
                out.push(digit.to_ascii_uppercase());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_everything_but_the_unreserved_characters() {
        assert_eq!(encode("a-b_c.d~e"), "a-b_c.d~e");
        assert_eq!(encode("What, & why?"), "What%2C%20%26%20why%3F");
        assert_eq!(encode("https://x.io/a/"), "https%3A%2F%2Fx.io%2Fa%2F");
        assert_eq!(encode("é"), "%C3%A9");
    }

    #[test]
    fn every_network_has_an_address_that_takes_the_page() {
        for key in SHARE_NETWORKS {
            let (_, address) = network(key).unwrap();
            assert!(address.contains("{url}"), "{key}");
        }
        assert_eq!(network("myspace"), None);
    }
}
