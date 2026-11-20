//! The link previews a writer sets by hand in `share_cards:`, one for sites
//! that read Open Graph, such as Facebook, and one for X, as the page head
//! writes them.

use khalid_content::{ShareCard, ShareCards};
use khalid_core::{AbsoluteUrl, Config};
use khalid_seo::{CardOverride, CardOverrides};

use crate::post_files::library_photo;

/// The cards of the page at `path`, such as `/blog/about/`, as its front
/// matter sets them in `cards`.
pub(crate) fn overrides(config: &Config, path: &str, cards: &ShareCards) -> CardOverrides {
    let mut overrides = CardOverrides::default();
    overrides.open_graph = one(config, path, cards.facebook.as_ref());
    overrides.x = one(config, path, cards.x.as_ref());
    overrides
}

/// One card, or an empty one when the writer set none.
fn one(config: &Config, path: &str, card: Option<&ShareCard>) -> CardOverride {
    let Some(card) = card else {
        return CardOverride::default();
    };
    let image = card
        .image
        .as_deref()
        .and_then(|src| image(config, path, src));
    CardOverride::new(card.title.clone(), card.description.clone(), image)
}

/// The full address of a card's image `src`: a full address as it is, a
/// media library path or a path from the site's root on the site's host,
/// and any other name a file beside the page at `path`. `None`, with a
/// warning, when no address can be made of it.
fn image(config: &Config, path: &str, src: &str) -> Option<AbsoluteUrl> {
    if src.starts_with("https://") || src.starts_with("http://") {
        return AbsoluteUrl::parse(src).ok();
    }
    let at = library_photo::address(config, src)
        .or_else(|| src.starts_with('/').then(|| src.to_owned()))
        .unwrap_or_else(|| format!("{path}{}", src.trim_start_matches("./")));
    let origin = config.address().as_str().trim_end_matches('/');
    let url = AbsoluteUrl::parse(&format!("{origin}{at}")).ok();
    if url.is_none() {
        tracing::warn!(image = %src, page = %path, "a share card's image has no address readers could fetch, so the card uses the page's own image");
    }
    url
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use khalid_core::Environment;

    use super::*;

    fn demo() -> Config {
        let mut folder = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        folder.push("../../fixtures/demo-site");
        Config::load(&folder, &Environment::empty()).unwrap()
    }

    fn cards(yaml: &str) -> ShareCards {
        let text = format!("---\nshare_cards: {yaml}\n---\n");
        khalid_content::parse_post(Path::new("a.md"), &text)
            .unwrap()
            .front
            .share_cards
    }

    #[test]
    fn each_network_gets_its_own_card() {
        let set = cards(
            "{x: {title: On X}, facebook: {title: On Facebook, description: Short, image: https://cdn.example.com/f.png}}",
        );
        let made = overrides(&demo(), "/blog/about/", &set);
        assert_eq!(made.x.title.as_deref(), Some("On X"));
        assert_eq!((made.x.description, made.x.image), (None, None));
        assert_eq!(made.open_graph.title.as_deref(), Some("On Facebook"));
        assert_eq!(made.open_graph.description.as_deref(), Some("Short"));
        let image = made.open_graph.image.map(|url| url.as_str().to_owned());
        assert_eq!(image.as_deref(), Some("https://cdn.example.com/f.png"));
    }

    #[test]
    fn an_image_beside_the_page_or_from_the_root_gets_the_sites_host() {
        let config = demo();
        let at = |src| image(&config, "/blog/about/", src).map(|url| url.as_str().to_owned());
        assert_eq!(
            at("./card.png").as_deref(),
            Some("https://www.example.com/blog/about/card.png")
        );
        assert_eq!(
            at("/images/card.png").as_deref(),
            Some("https://www.example.com/images/card.png")
        );
    }
}
