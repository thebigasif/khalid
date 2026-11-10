//! A post's blocks reach its JSON-LD graph, and the graph has everything
//! Google requires.

use std::path::{Path, PathBuf};

use khalid_content::render;
use khalid_core::{Config, Environment, Locale, SlugMode};
use khalid_seo::{IssueLevel, PageKind, check_graph, page_graph};

use super::fill;
use crate::build_seo;

/// A body with one of each block that writes its own schema, and a link
/// to a Wikipedia article.
const BODY: &str = "\
Read about [search engine optimization](https://en.wikipedia.org/wiki/Search_engine_optimization) first.

::::faq
:::question{text=\"What is a title tag?\"}
The blue link in search results.
:::
::::

::::howto{name=\"Add an internal link\" time=\"10:00\"}
:::step{name=\"Find an older post\"}
Search your site for the topic.
:::
::::

:::review{item=\"Ahrefs Webmaster Tools\" type=\"SoftwareApplication\" rating=\"4.5\" best=\"5\"}
Fast and free for your own site.
:::

::::recipe{name=\"Oat bars\" image=\"oat-bars.jpg\" prep=\"15:00\" cook=\"30:00\"}
:::ingredients
- 250 g rolled oats
:::

:::method
1. Bake at 180 C for 30 minutes.
:::
::::

::video{src=\"/media/site-tour.mp4\" title=\"A tour of the site\" poster=\"site-tour.jpg\" duration=\"2:05\"}

:::product{name=\"Site Audit Kit\" image=\"audit-kit.png\" price=\"49.00\" currency=\"USD\" rating=\"4.5\" href=\"/shop/audit-kit/\" button=\"Buy the kit\"}
Every check a small site needs.
:::

:::product{name=\"Plain notebook\"}
Paper for notes.
:::
";

/// The demo site's folder.
fn demo() -> PathBuf {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    path.push("../../fixtures/demo-site");
    path
}

#[test]
fn every_block_reaches_the_graph_and_google_finds_nothing_missing() {
    let config = Config::load(&demo(), &Environment::empty()).unwrap();
    let locale = Locale::try_new("en").unwrap();
    let rendered = render(BODY, &locale, SlugMode::Ascii).unwrap();
    let mut seo = build_seo::page(PageKind::Post, &locale, "blocks/", "Blocks").unwrap();
    seo.published = Some(build_seo::midnight(chrono::NaiveDate::default()));
    fill(
        &mut seo,
        &config,
        ("/blog/blocks/", "Asif"),
        (&rendered.structured, &rendered.links),
    )
    .unwrap();

    let site = build_seo::site(&config).unwrap();
    let graph = page_graph(&site, &seo).unwrap();
    let json = graph.to_json();
    for kind in [
        "FAQPage",
        "HowTo",
        "Review",
        "Recipe",
        "VideoObject",
        "Product",
    ] {
        assert!(
            json.contains(&format!("\"@type\": \"{kind}\"")),
            "{kind} in {json}"
        );
    }
    assert!(json.contains("\"mentions\""), "{json}");
    assert!(json.contains("https://en.wikipedia.org/wiki/Search_engine_optimization"));
    assert!(
        json.contains("https://www.example.com/blog/blocks/oat-bars.jpg"),
        "{json}"
    );
    assert!(
        json.contains("https://www.example.com/media/site-tour.mp4"),
        "{json}"
    );
    assert!(
        json.contains("\"name\": \"Asif\""),
        "the post's author reviews: {json}"
    );
    assert_eq!(seo.videos.len(), 1, "for the video sitemap");
    assert!(json.contains("\"price\": \"49.00\""), "{json}");
    assert!(
        json.contains("https://www.example.com/shop/audit-kit/"),
        "{json}"
    );
    assert!(
        !json.contains("Plain notebook"),
        "a card with no price and no score writes no Product: {json}"
    );

    let blocking: Vec<_> = check_graph(&graph)
        .into_iter()
        .filter(|issue| issue.level() == IssueLevel::Blocks)
        .map(|issue| format!("{} {}", issue.kind(), issue.property()))
        .collect();
    assert_eq!(blocking, Vec::<String>::new(), "{json}");
}

/// A post at `/blog/films/` with `body`, its summary and its `lead`
/// image filled in, as the build fills them.
fn embeds_post(body: &str, lead: Option<&str>) -> khalid_seo::SeoPage {
    let config = Config::load(&demo(), &Environment::empty()).unwrap();
    let locale = Locale::try_new("en").unwrap();
    let rendered = render(body, &locale, SlugMode::Ascii).unwrap();
    let mut seo = build_seo::page(PageKind::Post, &locale, "films/", "Films").unwrap();
    seo.excerpt = "Two short films about search.".to_owned();
    seo.images = lead
        .map(|src| khalid_core::AbsoluteUrl::parse(src).unwrap())
        .into_iter()
        .collect();
    let body = (&rendered.structured[..], &rendered.links[..]);
    fill(&mut seo, &config, ("/blog/films/", "Asif"), body).unwrap();
    seo
}

#[test]
fn a_video_embed_reaches_the_video_sitemap_by_its_player() {
    let seo = embeds_post(
        "::embed{src=\"https://www.youtube.com/watch?v=dQw4w9WgXcQ\" title=\"How search works\"}\n",
        None,
    );
    let [video] = seo.videos.as_slice() else {
        panic!("one video: {:?}", seo.videos);
    };
    assert_eq!(video.title, "How search works");
    assert_eq!(video.description, "Two short films about search.");
    assert_eq!(
        video.thumbnail.as_str(),
        "https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg"
    );
    assert_eq!(
        video.player().map(khalid_core::AbsoluteUrl::as_str),
        Some("https://www.youtube-nocookie.com/embed/dQw4w9WgXcQ")
    );
    assert_eq!(video.file(), None);
    assert_eq!(video.uploaded, None, "the host's upload date is unknown");
    assert!(
        seo.blocks.is_empty(),
        "no VideoObject without an upload date"
    );
}

#[test]
fn a_vimeo_embed_shows_the_lead_image_and_waits_for_one() {
    let vimeo = "::embed{src=\"https://vimeo.com/76979871\" title=\"A short film\"}\n";
    let lead = "https://www.example.com/blog/media/film.jpg";
    let seo = embeds_post(vimeo, Some(lead));
    assert_eq!(seo.videos.len(), 1);
    assert_eq!(seo.videos[0].thumbnail.as_str(), lead);
    let seo = embeds_post(vimeo, None);
    assert_eq!(seo.videos, [], "Google refuses a video with no still");
}
