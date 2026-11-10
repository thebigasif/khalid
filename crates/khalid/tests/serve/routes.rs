//! `routes.toml` on a running `khalid serve`: every list page answers where
//! it is routed, and an alias moves with its post.

use std::fs;

use super::{demo_copy, get, put, start};

#[test]
fn serve_answers_every_list_page_where_routes_toml_puts_it() {
    let site = demo_copy();
    put(
        site.path(),
        "routes.toml",
        "[taxonomies]\ncategory = \"topic\"\n\n\
         [collections.news]\npermalink = \"/news/{year}/{month}/{slug}/\"\n\n\
         [channels.from-lena]\npath = \"/from-lena/\"\nfilter = { author = \"lena-vogt\" }\n",
    );
    put(
        site.path(),
        "content/en/news/_index.md",
        "---\ntitle: News\n---\n\nWhat changed on Field Notes, month by month.\n",
    );
    put(
        site.path(),
        "content/en/news/launch.md",
        "---\ntitle: Field Notes is live\nslug: launch\ndate: 2026-09-03\nauthor: asif\n---\n\n\
         The blog is live. Start at [the home page](/blog/).\n",
    );
    let server = start(site.path());
    for (path, text) in [
        ("/blog/archive/", "<h1>Archive</h1>"),
        ("/blog/de/archive/", "lang=\"de\""),
        ("/blog/authors/", "<h1>Authors</h1>"),
        ("/blog/sitemap/", "<h1>Sitemap</h1>"),
        ("/blog/topic/technical-seo/", "<h1>Technical SEO</h1>"),
        ("/blog/from-lena/", "<h1>From lena</h1>"),
        ("/blog/news/", "month by month."),
        ("/blog/news/2026/", "<h1>2026</h1>"),
        ("/blog/news/2026/09/", "Field Notes is live</a>"),
    ] {
        let answer = get(&server, path);
        assert_eq!(answer.status, 200, "{path}");
        assert!(answer.body.contains(text), "{path}:\n{}", answer.body);
    }
    assert_eq!(get(&server, "/blog/category/technical-seo/").status, 404);
}

#[test]
fn serve_moves_an_alias_to_where_routes_toml_puts_the_post() {
    let site = demo_copy();
    let mut routes = site.path().to_path_buf();
    routes.push("routes.toml");
    fs::write(
        &routes,
        "[collections.posts]\npermalink = \"/{year}/{slug}/\"\n",
    )
    .unwrap();
    let mut file = site.path().to_path_buf();
    file.push("content/en/posts/2026-09-28-what-a-sitemap-does.md");
    let post = fs::read_to_string(&file).unwrap();
    let alias = "translation: sitemap\naliases:\n  - /blog/sitemaps-explained/\n";
    fs::write(&file, post.replace("translation: sitemap\n", alias)).unwrap();
    let server = start(site.path());
    let moved = get(&server, "/blog/sitemaps-explained/");
    assert_eq!(moved.status, 301, "the alias moved to its post");
    assert_eq!(moved.location, "/blog/2026/what-a-sitemap-does/");
    assert_eq!(get(&server, &moved.location).status, 200);
}
