//! Embed blocks and the provider allowlist (task 1.7, KH-CON-5).

use khalid_content::{
    BlockKind, EMBED_PROVIDERS, EmbedProvider, Gated, Wikilinks, parse_blocks, print_blocks,
    render, render_framed,
};
use khalid_core::{AbsoluteUrl, DomainName, Locale, SlugMode};

#[cfg(test)]
fn round_trip(markdown: &str) -> BlockKind {
    let blocks = parse_blocks(markdown).unwrap();
    assert_eq!(blocks.len(), 1, "{markdown:?} gave {blocks:?}");
    assert_eq!(print_blocks(&blocks), markdown);
    blocks[0].kind.clone()
}

#[cfg(test)]
fn html(markdown: &str) -> String {
    render(markdown, &Locale::try_new("en").unwrap(), SlugMode::Ascii)
        .unwrap()
        .html
}

#[cfg(test)]
fn error(markdown: &str) -> String {
    parse_blocks(markdown).unwrap_err().to_string()
}

#[cfg(test)]
fn url(text: &str) -> AbsoluteUrl {
    AbsoluteUrl::parse(text).unwrap()
}

#[test]
fn an_embed_round_trips() {
    assert_eq!(
        round_trip(
            "::embed{src=\"https://www.youtube.com/watch?v=dQw4w9WgXcQ\" title=\"How search works\"}\n"
        ),
        BlockKind::Embed {
            src: url("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            title: "How search works".to_owned(),
            height: None,
        }
    );
}

#[test]
fn renders_each_provider_in_a_frame_and_any_other_address_as_a_link() {
    insta::assert_snapshot!(html(
        "\
::embed{src=\"https://www.youtube.com/watch?v=dQw4w9WgXcQ\" title=\"How search works\"}

::embed{src=\"https://youtu.be/dQw4w9WgXcQ\" title=\"How search works\"}

::embed{src=\"https://vimeo.com/76979871\" title=\"A short film\"}

::embed{src=\"https://open.spotify.com/episode/4rOoJ6Egrf8K2IrywzwOMk\" title=\"An SEO podcast\"}

::embed{src=\"https://soundcloud.com/forss/flickermood\" title=\"Flickermood\"}

::embed{src=\"https://codepen.io/chriscoyier/pen/gfdDu\" title=\"A layout demo\"}

::embed{src=\"https://www.loom.com/share/0281766fa2d04bb788eaf19e65135184\" title=\"A site tour\"}

::embed{src=\"https://example.com/talk\" title=\"A talk elsewhere\"}
"
    ));
}

/// Safari refuses a sandbox token it does not know, `allow-presentation`,
/// and logs an error on every page with a player. Fullscreen comes from
/// the `allow` list, which every browser reads.
#[test]
fn a_player_frame_carries_only_sandbox_tokens_every_browser_knows() {
    let page = html(
        "::embed{src=\"https://www.youtube.com/watch?v=dQw4w9WgXcQ\" title=\"How search works\"}\n",
    );
    assert!(
        page.contains("sandbox=\"allow-scripts allow-same-origin allow-popups\""),
        "{page}"
    );
    assert!(!page.contains("allow-presentation"), "{page}");
    assert!(
        page.contains("allow=\"encrypted-media; fullscreen; picture-in-picture\""),
        "{page}"
    );
}

#[test]
fn an_embed_joins_the_links() {
    let rendered = render(
        "::embed{src=\"https://vimeo.com/1\" title=\"A film\"}\n",
        &Locale::try_new("en").unwrap(),
        SlugMode::Ascii,
    )
    .unwrap();
    let links: Vec<(&str, &str)> = rendered
        .links
        .iter()
        .map(|link| (link.href.as_str(), link.text.as_str()))
        .collect();
    assert_eq!(links, [("https://vimeo.com/1", "A film")]);
}

#[test]
fn the_allowlist_names_every_frame_origin_once() {
    let origins: Vec<&str> = EMBED_PROVIDERS
        .iter()
        .map(|provider| provider.frame_origin())
        .collect();
    assert_eq!(
        origins,
        [
            "https://www.youtube-nocookie.com",
            "https://player.vimeo.com",
            "https://open.spotify.com",
            "https://w.soundcloud.com",
            "https://codepen.io",
            "https://www.loom.com",
        ]
    );
    assert_eq!(EMBED_PROVIDERS[0], EmbedProvider::YouTube);
}

#[test]
fn an_embed_needs_a_title() {
    assert_eq!(
        error("::embed{src=\"https://vimeo.com/1\"}\n"),
        "line 1: an embed needs title, the name a screen reader says for the player"
    );
}

/// `markdown` rendered on a site that allows frames from `domains`.
#[cfg(test)]
fn framed(markdown: &str, domains: &[&str]) -> String {
    let domains: Vec<DomainName> = domains
        .iter()
        .map(|domain| DomainName::try_new(domain).unwrap())
        .collect();
    let en = Locale::try_new("en").unwrap();
    let gated = Gated::Locked { back: None };
    render_framed(
        markdown,
        &en,
        SlugMode::Ascii,
        &Wikilinks::new(),
        gated,
        &domains,
    )
    .unwrap()
    .html
}

const CALCULATOR: &str = "::embed{src=\"https://tools.example.com/roi?plan=pro\" title=\"Content ROI calculator\" height=\"560\"}\n";

#[test]
fn an_embed_round_trips_with_its_height_and_refuses_a_wrong_one() {
    assert_eq!(
        round_trip(CALCULATOR),
        BlockKind::Embed {
            src: url("https://tools.example.com/roi?plan=pro"),
            title: "Content ROI calculator".to_owned(),
            height: Some(560),
        }
    );
    for height in ["50", "3000", "tall"] {
        let markdown =
            format!("::embed{{src=\"https://a.example\" title=\"T\" height=\"{height}\"}}\n");
        assert_eq!(
            error(&markdown),
            "line 1: an embed's height is the frame's height in pixels, a whole number from 120 to 2000"
        );
    }
}

#[test]
fn a_page_on_an_allowed_domain_shows_in_a_lazy_sandboxed_frame_of_its_height() {
    let html = framed(CALCULATOR, &["tools.example.com"]);
    assert_eq!(
        html,
        "<figure class=\"embed embed-site\">\n<iframe src=\"https://tools.example.com/roi?plan=pro\" title=\"Content ROI calculator\" width=\"560\" height=\"560\" loading=\"lazy\" referrerpolicy=\"strict-origin-when-cross-origin\" sandbox=\"allow-scripts allow-same-origin allow-forms allow-popups\"></iframe>\n<figcaption><a href=\"https://tools.example.com/roi?plan=pro\">Content ROI calculator</a></figcaption>\n</figure>\n"
    );
    let no_height = framed(
        "::embed{src=\"https://tools.example.com/a\" title=\"A tool\"}\n",
        &["tools.example.com"],
    );
    assert!(no_height.contains(" height=\"480\" "), "{no_height}");
}

#[test]
fn a_page_on_any_other_domain_or_over_http_stays_a_link() {
    let link = "<p class=\"bookmark\"><a href=";
    assert!(html(CALCULATOR).starts_with(link), "no allowlist");
    assert!(
        framed(CALCULATOR, &["example.com"]).starts_with(link),
        "a parent domain"
    );
    assert!(framed(CALCULATOR, &["evil-tools.example.com"]).starts_with(link));
    let http = "::embed{src=\"http://tools.example.com/a\" title=\"T\"}\n";
    assert!(
        framed(http, &["tools.example.com"]).starts_with(link),
        "http"
    );
}
