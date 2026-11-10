//! The home page's optional parts: the owner's numbers under the title,
//! tabs per topic that work as anchor links, and rows of posts with a
//! video or a sound player.

use std::collections::BTreeMap;

use khalid_render::{
    HomeSectionKind, HomeSectionPosts, PageBody, PageKind, RenderHome, RenderPack, Renderer,
    TemplateOption,
};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::listing::{first_page, main_of};
use crate::post::part;

fn home() -> RenderHome {
    let posts = first_page().posts;
    RenderHome {
        listing: first_page(),
        sections: Vec::new(),
        picks: Vec::new(),
        topics: Vec::new(),
        categories: Vec::new(),
        tabs: ["Technical SEO", "Writing"]
            .iter()
            .map(|title| HomeSectionPosts {
                title: (*title).to_owned(),
                href: format!("/blog/category/{}/", title.to_lowercase().replace(' ', "-")),
                posts: posts.clone(),
                kind: HomeSectionKind::Posts,
            })
            .collect(),
        videos: posts.iter().take(1).cloned().collect(),
        podcasts: posts.iter().skip(1).cloned().collect(),
        podcast_player: None,
    }
}

fn render(options: BTreeMap<String, TemplateOption>) -> String {
    let pack = RenderPack {
        name: "trial",
        templates: Vec::new(),
        options,
        cta: None,
    };
    let site = tempfile::tempdir().unwrap();
    let renderer = Renderer::with_pack(site.path(), &pack).unwrap();
    let meta = meta("Field Notes on SEO", "/blog/", PageKind::Website);
    let body = PageBody::Home(home());
    renderer.render(&demo_site("en"), &meta, &body).unwrap()
}

fn on(keys: &[&str]) -> BTreeMap<String, TemplateOption> {
    let mut options: BTreeMap<String, TemplateOption> = keys
        .iter()
        .map(|key| ((*key).to_owned(), TemplateOption::Switch(true)))
        .collect();
    let stats = vec![
        "140 | small business blogs studied".to_owned(),
        "9,200 | posts read".to_owned(),
    ];
    options.insert("hero_stats".to_owned(), TemplateOption::Lines(stats));
    options
}

#[test]
fn the_owner_numbers_sit_under_the_title_and_each_row_shows_when_on() {
    let page = render(on(&["topic_tabs", "video_section", "podcast_section"]));
    assert_valid_page(&page);
    insta::assert_snapshot!(part(&page, r#"<dl class="hero-stats">"#, "</dl>"));
    insta::assert_snapshot!(part(&page, r#"<ul class="topic-tab-list">"#, "</ul>"));
    let main = main_of(&page);
    assert!(
        main.contains(
            r#"<section class="topic-panel" id="topic-2" aria-labelledby="topic-2-title">"#
        )
    );
    assert!(
        main.contains(r#"<h2 id="home-videos">Videos</h2>"#),
        "{main}"
    );
    assert!(main.contains(r#"<h2 id="home-podcasts">Podcast episodes</h2>"#));
}

#[test]
fn every_part_is_off_until_the_template_turns_it_on() {
    let page = render(BTreeMap::new());
    let main = main_of(&page);
    for class in ["hero-stats", "topic-tabs", "media-row"] {
        assert!(!main.contains(class), "{class} in {main}");
    }
}
