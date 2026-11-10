//! Sitemap files per section and language, split at 50,000, and the
//! index of them (task 1.13, KH-SEO-5).

#![cfg(test)]

mod common;
mod sitemap_index_xsd;
mod sitemap_xsd;

use std::time::Duration;

use common::{at, demo_site, internal_links_post, internal_links_post_de, locale, path, url};
use khalid_seo::{PageKind, SeoPage, SitemapSet, UrlPath, Video, VideoSource, link_translations};
use sitemap_index_xsd::check_sitemapindex;
use sitemap_xsd::{check_urlset, google_extensions};

fn demo_pages() -> Vec<SeoPage> {
    let home = SeoPage::new(PageKind::Home, locale("en"), UrlPath::root(), "");
    let about = SeoPage::new(PageKind::Page, locale("en"), path("/about/"), "About");
    let mut tag = SeoPage::new(PageKind::Tag, locale("en"), path("/tag/links/"), "Links");
    tag.item_count = Some(4);
    vec![
        home,
        internal_links_post(),
        internal_links_post_de(),
        about,
        tag,
    ]
}

fn demo_set() -> SitemapSet {
    let site = demo_site();
    let mut pages = demo_pages();
    link_translations(&site, &mut pages).unwrap();
    SitemapSet::from_pages(&site, &pages).unwrap()
}

#[test]
fn each_section_and_language_gets_its_own_file() {
    let paths: Vec<String> = demo_set()
        .files()
        .iter()
        .map(|file| file.path.to_string())
        .collect();
    assert_eq!(
        paths,
        [
            "/sitemaps/posts-en-1.xml",
            "/sitemaps/posts-de-1.xml",
            "/sitemaps/pages-en-1.xml",
            "/sitemaps/archives-en-1.xml",
        ]
    );
}

#[test]
fn the_index_lists_every_file_of_the_set() {
    insta::assert_snapshot!(demo_set().index().to_xml());
}

#[test]
fn the_index_and_every_file_are_valid_against_the_schemas() {
    let set = demo_set();
    assert_eq!(check_sitemapindex(&set.index().to_xml()), Ok(4));
    for file in set.files() {
        assert!(
            check_urlset(&file.sitemap.to_xml(), google_extensions).is_ok(),
            "{}",
            file.path
        );
    }
}

#[test]
fn a_file_is_split_at_fifty_thousand_addresses() {
    let site = demo_site();
    let pages: Vec<SeoPage> = (0..50_001)
        .map(|n| {
            SeoPage::new(
                PageKind::Post,
                locale("en"),
                path(&format!("/note-{n}/")),
                "Note",
            )
        })
        .collect();
    let set = SitemapSet::from_pages(&site, &pages).unwrap();
    let sizes: Vec<(String, usize)> = set
        .files()
        .iter()
        .map(|file| (file.path.to_string(), file.sitemap.entries().len()))
        .collect();
    assert_eq!(
        sizes,
        [
            ("/sitemaps/posts-en-1.xml".to_owned(), 50_000),
            ("/sitemaps/posts-en-2.xml".to_owned(), 1),
        ]
    );
    let first = set.files()[0].sitemap.to_xml();
    assert_eq!(check_urlset(&first, google_extensions), Ok(50_000));
}

fn post_with_video() -> SeoPage {
    let mut post = internal_links_post();
    let mut video = Video::new(
        "Internal links in five minutes",
        "A short walk through adding internal links to an older post.",
        url("https://i.ytimg.com/vi/aqz-KE-bpKQ/hqdefault.jpg"),
        VideoSource::Player(url("https://www.youtube-nocookie.com/embed/aqz-KE-bpKQ")),
    );
    video.duration = Some(Duration::from_secs(312));
    video.uploaded = Some(at(2026, 9, 12, 10));
    post.videos = vec![video];
    post
}

#[test]
fn videos_get_their_own_file_listed_in_the_index() {
    let pages = [post_with_video(), internal_links_post_de()];
    let set = SitemapSet::from_pages(&demo_site(), &pages).unwrap();
    let paths: Vec<String> = set.files().iter().map(|f| f.path.to_string()).collect();
    assert_eq!(
        paths,
        [
            "/sitemaps/posts-en-1.xml",
            "/sitemaps/posts-de-1.xml",
            "/sitemaps/videos-en-1.xml",
        ]
    );
    let index = set.index().to_xml();
    assert!(
        index.contains("<loc>https://www.example.com/blog/sitemaps/videos-en-1.xml</loc>"),
        "{index}"
    );
    let posts = set.files()[0].sitemap.to_xml();
    assert!(!posts.contains("<video:video>"), "{posts}");
    let videos = set.files()[2].sitemap.to_xml();
    assert_eq!(check_urlset(&videos, google_extensions), Ok(1));
    insta::assert_snapshot!(videos);
}

#[test]
fn a_site_without_videos_writes_no_video_file() {
    let set = demo_set();
    assert!(
        set.files()
            .iter()
            .all(|file| !file.path.as_str().contains("videos")),
    );
}

#[test]
fn a_site_with_nothing_published_has_an_empty_valid_index() {
    let set = SitemapSet::from_pages(&demo_site(), &[]).unwrap();
    assert_eq!(set.files().len(), 0);
    assert_eq!(check_sitemapindex(&set.index().to_xml()), Ok(0));
}
