//! The check that the blog says who stands behind it: an About or
//! contact page a reader can reach (GEO P5).

pub mod support;

use khalid_audit::{Check, Crawl, Limit, Severity, Thresholds, audit, crawl};
use support::{HOME, Memory, crawled, html, page, url};

/// Where the about check put its issue, if anywhere.
#[cfg(test)]
fn about_missing(crawl: &Crawl) -> Vec<String> {
    audit(crawl, Thresholds::default())
        .iter()
        .filter(|issue| issue.check() == Check::AboutMissing)
        .map(|issue| issue.url().as_str().to_owned())
        .collect()
}

#[tokio::test]
async fn a_blog_with_no_about_or_contact_page_is_warned_once_on_its_home_page() {
    let crawl = crawled(&[
        (HOME, html(r#"<a href="/blog/a-post/">A post</a>"#)),
        ("https://www.example.com/blog/a-post/", html("")),
    ])
    .await;
    assert_eq!(about_missing(&crawl), [HOME]);
    assert_eq!(Check::AboutMissing.severity(), Severity::Warning);
    assert_eq!(Check::AboutMissing.id(), "about-missing");
}

#[tokio::test]
async fn an_about_page_in_the_blog_or_a_contact_page_on_the_main_site_is_enough() {
    let inside = crawled(&[
        (HOME, html(r#"<a href="/blog/about/">About</a>"#)),
        ("https://www.example.com/blog/about/", html("")),
    ])
    .await;
    assert_eq!(about_missing(&inside), Vec::<String>::new());
    let main_site = crawled(&[(HOME, html(r#"<a href="/contact-us/">Contact</a>"#))]).await;
    assert_eq!(about_missing(&main_site), Vec::<String>::new());
}

#[tokio::test]
async fn a_blog_on_a_subdomain_may_link_to_the_about_page_of_its_main_site() {
    let home = "https://blog.example.com/";
    for about in [
        "https://example.com/about/",
        "https://www.example.com/contact/",
    ] {
        let source = Memory::new(&[(home, html(&format!(r#"<a href="{about}">About</a>"#)))]);
        let crawl = crawl(&source, &url(home), Limit::default()).await;
        assert_eq!(about_missing(&crawl), Vec::<String>::new(), "{about}");
    }
    let source = Memory::new(&[(
        home,
        html(r#"<a href="https://notexample.com/about/">About</a>"#),
    )]);
    let crawl = crawl(&source, &url(home), Limit::default()).await;
    assert_eq!(about_missing(&crawl), [home]);
}

#[tokio::test]
async fn a_page_marked_as_about_in_its_schema_is_enough_whatever_its_address() {
    let marked = r#"<title>Who we are</title><script type="application/ld+json">{"@context":"https://schema.org","@graph":[{"@type":"AboutPage","name":"Who we are"}]}</script>"#;
    let crawl = crawled(&[
        (HOME, html(r#"<a href="/blog/who-we-are/">Who we are</a>"#)),
        ("https://www.example.com/blog/who-we-are/", page(marked, "")),
    ])
    .await;
    assert_eq!(about_missing(&crawl), Vec::<String>::new());
}

#[tokio::test]
async fn another_sites_about_page_or_a_broken_one_does_not_count() {
    let crawl = crawled(&[(
        HOME,
        html(r#"<a href="https://other.example.org/about/">Them</a> <a href="/blog/contact/">Contact</a>"#),
    )])
    .await;
    assert_eq!(about_missing(&crawl), [HOME]);
}
