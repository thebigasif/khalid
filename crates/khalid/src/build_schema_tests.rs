//! A post's blocks reach its JSON-LD graph, and the graph has everything
//! Google requires.

use std::path::{Path, PathBuf};

use khalid_content::render;
use khalid_core::{Config, Environment, Locale, SlugMode};
use khalid_seo::{IssueLevel, PageKind, SchemaGraph, check_graph, page_graph};

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

::business{name=\"Larkspur Studio\" type=\"ProfessionalService\" street=\"4 Tannery Yard\" locality=\"Bristol\" postcode=\"BS1 6QA\" country=\"GB\" href=\"/about/\"}

:::event{name=\"Internal links clinic\" start=\"2026-11-12T18:00+01:00\" online=\"https://meet.example.com/clinic\" price=\"0\" currency=\"GBP\" image=\"clinic.jpg\"}
An hour of fixing internal links.
:::

:::course{name=\"SEO writing\"}
Seven lessons on **writing** posts that rank.
:::

::book{name=\"Everybody Writes\" author=\"Ann Handley\" isbn=\"978-1-119-85487-9\"}

:::dataset{name=\"Content benchmark\" license=\"https://creativecommons.org/licenses/by/4.0/\" download=\"benchmark.csv\"}
Word counts of 140 blogs.
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
        "ProfessionalService",
        "Event",
        "Course",
        "Book",
        "Dataset",
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
    for written in [
        "\"url\": \"https://www.example.com/about/\"",
        "\"startDate\": \"2026-11-12T17:00:00Z\"",
        "https://www.example.com/blog/blocks/clinic.jpg",
        "\"description\": \"Seven lessons on writing posts that rank.\"",
        "https://www.example.com/blog/blocks/benchmark.csv",
    ] {
        assert!(json.contains(written), "{written} in {json}");
    }
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

/// The `VideoObject` nodes `seo` writes, and the issues that would hold
/// its page back.
fn video_nodes(seo: &khalid_seo::SeoPage) -> (Vec<khalid_seo::SchemaNode>, Vec<String>) {
    let config = Config::load(&demo(), &Environment::empty()).unwrap();
    let site = build_seo::site(&config).unwrap();
    let graph = page_graph(&site, seo).unwrap();
    let blocking = check_graph(&graph)
        .into_iter()
        .filter(|issue| issue.level() == IssueLevel::Blocks)
        .map(|issue| format!("{} {}", issue.kind(), issue.property()))
        .collect();
    let videos = graph
        .nodes()
        .iter()
        .filter(|node| node.kind() == "VideoObject");
    (videos.cloned().collect(), blocking)
}

#[test]
fn an_embed_with_its_upload_date_is_a_video_object_and_google_finds_nothing_missing() {
    let seo = embeds_post(
        "\
::embed{src=\"https://www.youtube.com/watch?v=dQw4w9WgXcQ\" title=\"How search works\" uploaded=\"2024-05-16\"}

::embed{src=\"https://vimeo.com/76979871\" title=\"A short film\"}
",
        Some("https://www.example.com/blog/media/film.jpg"),
    );
    assert_eq!(seo.videos.len(), 2, "both reach the video sitemap");
    let (videos, blocking) = video_nodes(&seo);
    assert_eq!(blocking, Vec::<String>::new());
    let json = SchemaGraph::new(videos).to_json();
    assert_eq!(json.matches("\"VideoObject\"").count(), 1, "{json}");
    for written in [
        "\"uploadDate\": \"2024-05-16T00:00:00Z\"",
        "\"embedUrl\": \"https://www.youtube-nocookie.com/embed/dQw4w9WgXcQ\"",
        "\"thumbnailUrl\": \"https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg\"",
        "\"description\": \"Two short films about search.\"",
    ] {
        assert!(json.contains(written), "{written} in {json}");
    }
    assert!(!json.contains("vimeo"), "no date, no node: {json}");
}

#[test]
fn a_video_file_takes_its_own_date_else_the_posts_else_writes_no_node() {
    let file = "::video{src=\"/media/tour.mp4\" title=\"A tour\" poster=\"tour.jpg\"}\n";
    let seo = embeds_post(file, None);
    assert_eq!(seo.videos.len(), 1, "the sitemap still lists it");
    assert_eq!(video_nodes(&seo).0, [], "a page with no date");

    let own = "::video{src=\"/media/tour.mp4\" title=\"A tour\" poster=\"tour.jpg\" uploaded=\"2023-02-01\"}\n";
    let seo = embeds_post(own, None);
    let json = SchemaGraph::new(video_nodes(&seo).0).to_json();
    assert!(
        json.contains("\"uploadDate\": \"2023-02-01T00:00:00Z\""),
        "{json}"
    );
    let day = seo.videos[0].uploaded.map(|date| date.to_rfc3339());
    assert_eq!(day.as_deref(), Some("2023-02-01T00:00:00+00:00"), "sitemap");
}

/// The channel showcase's first post plays its own video file: it writes a
/// `VideoObject` dated by the post, and Google finds nothing missing. The
/// channel's `site.toml` only overlays the showcase's, so the demo site's
/// settings stand in.
#[test]
fn the_channel_showcase_video_writes_its_node() {
    let config = Config::load(&demo(), &Environment::empty()).unwrap();
    let locale = Locale::try_new("en").unwrap();
    let mut path = demo();
    path.set_file_name("showcase-publications");
    path.push("channel/content/en/posts/2026-07-08-first-bowl-centring-clay/index.md");
    let text = std::fs::read_to_string(&path).unwrap();
    let body = text.splitn(3, "---\n").nth(2).unwrap();
    let rendered = render(body, &locale, SlugMode::Ascii).unwrap();
    let rest = "first-bowl-centring-clay/";
    let mut seo = build_seo::page(PageKind::Post, &locale, rest, rest).unwrap();
    let day = chrono::NaiveDate::from_ymd_opt(2026, 7, 8).unwrap();
    seo.published = Some(build_seo::midnight(day));
    let by = (&rendered.structured[..], &rendered.links[..]);
    fill(
        &mut seo,
        &config,
        ("/blog/first-bowl-centring-clay/", "Noor"),
        by,
    )
    .unwrap();

    let (videos, blocking) = video_nodes(&seo);
    assert_eq!(blocking, Vec::<String>::new());
    assert_eq!(videos.len(), 1);
    insta::assert_snapshot!(SchemaGraph::new(videos).to_json());
}

/// The showcase pages whose blocks write schema: the file under the
/// showcase's `content/`, the page's address under the blog, and its kind.
const SHOWCASE: [(&str, &str, PageKind); 4] = [
    (
        "en/posts/2026-05-05-seo-writing-course/index.md",
        "seo-writing-course/",
        PageKind::Post,
    ),
    (
        "en/posts/2026-10-09-content-benchmark-data/index.md",
        "content-benchmark-data/",
        PageKind::Post,
    ),
    (
        "en/posts/2026-08-04-editing-checklist/index.md",
        "editing-checklist/",
        PageKind::Post,
    ),
    ("en/pages/about/index.md", "about/", PageKind::Page),
];

/// The types the showcase's cards write.
const CARD_TYPES: [&str; 5] = ["Event", "Course", "Book", "Dataset", "ProfessionalService"];

#[test]
fn the_showcase_cards_write_their_nodes_and_google_finds_nothing_missing() {
    let mut site_dir = demo();
    site_dir.set_file_name("showcase-site");
    let config = Config::load(&site_dir, &Environment::empty()).unwrap();
    let site = build_seo::site(&config).unwrap();
    let locale = Locale::try_new("en").unwrap();
    for (file, rest, kind) in SHOWCASE {
        let mut path = site_dir.clone();
        path.push("content");
        path.push(file);
        let text = std::fs::read_to_string(&path).unwrap();
        let body = text.splitn(3, "---\n").nth(2).unwrap();
        let rendered = render(body, &locale, SlugMode::Ascii).unwrap();
        let mut seo = build_seo::page(kind, &locale, rest, rest).unwrap();
        seo.author = Some("Maren Okafor".to_owned());
        seo.published = Some(build_seo::midnight(chrono::NaiveDate::default()));
        let address = format!("/blog/{rest}");
        let body = (&rendered.structured[..], &rendered.links[..]);
        fill(&mut seo, &config, (&address, "Maren Okafor"), body).unwrap();

        let graph = page_graph(&site, &seo).unwrap();
        let blocking: Vec<_> = check_graph(&graph)
            .into_iter()
            .filter(|issue| issue.level() == IssueLevel::Blocks)
            .map(|issue| format!("{} {}", issue.kind(), issue.property()))
            .collect();
        assert_eq!(blocking, Vec::<String>::new(), "{file}");
        let cards: Vec<_> = graph
            .nodes()
            .iter()
            .filter(|node| CARD_TYPES.contains(&node.kind()))
            .cloned()
            .collect();
        assert!(!cards.is_empty(), "{file}");
        let name = rest.trim_end_matches('/');
        insta::assert_snapshot!(name, SchemaGraph::new(cards).to_json());
    }
}
