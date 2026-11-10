//! robots.txt with per-bot rules (task 1.13, KH-SEO-7).
//!
//! Every check goes through `robotstxt`, a port of Google's own parser and
//! matcher, so the tests ask what Googlebot would do, not what the text says.

#![cfg(test)]

mod common;

use common::{demo_site, internal_links_post, internal_links_post_de, path};
use khalid_seo::{
    AI_ANSWER_CRAWLERS, AI_TRAINING_CRAWLERS, BotAccess, RobotsLine, RobotsPath, RobotsTxt,
    SeoError, SeoSite, SiteSearch, UserAgent,
};
use robotstxt::DefaultMatcher;

fn agent(token: &str) -> UserAgent {
    UserAgent::try_new(token).unwrap()
}

/// The demo site with a private preview folder, AI training crawlers
/// blocked and Claude's search crawler let in.
fn demo_policy_site() -> SeoSite {
    let mut site = demo_site();
    site.crawlers.disallow = vec![RobotsPath::try_new("/preview/").unwrap()];
    site.crawlers
        .bots
        .insert(agent("GPTBot"), BotAccess::Blocked);
    site.crawlers
        .bots
        .insert(agent("CCBot"), BotAccess::Blocked);
    site.crawlers
        .bots
        .insert(agent("Claude-SearchBot"), BotAccess::Allowed);
    site
}

fn allowed(robots: &str, bot: &str, address: &str) -> bool {
    DefaultMatcher::default().one_agent_allowed_by_robots(robots, bot, address)
}

#[test]
fn the_demo_robots_txt() {
    let robots = RobotsTxt::for_site(&demo_policy_site()).unwrap();
    insta::assert_snapshot!(robots.to_string());
}

#[test]
fn every_bot_may_read_posts_unless_blocked() {
    let robots = RobotsTxt::for_site(&demo_policy_site())
        .unwrap()
        .to_string();
    let site = demo_policy_site();
    for post in [internal_links_post(), internal_links_post_de()] {
        let address = site.url_for(&post.language, &post.path).unwrap();
        let address = address.as_str();
        assert!(allowed(&robots, "Googlebot", address), "{address}");
        assert!(allowed(&robots, "Claude-SearchBot", address), "{address}");
        assert!(!allowed(&robots, "GPTBot", address), "{address}");
        assert!(!allowed(&robots, "CCBot", address), "{address}");
    }
}

#[test]
fn a_path_blocked_for_all_is_blocked_for_a_named_bot_too() {
    let robots = RobotsTxt::for_site(&demo_policy_site())
        .unwrap()
        .to_string();
    let preview = "https://www.example.com/blog/preview/draft-on-links/";
    assert!(!allowed(&robots, "Googlebot", preview));
    assert!(!allowed(&robots, "Claude-SearchBot", preview));
}

#[test]
fn a_blog_in_a_folder_says_nothing_about_the_rest_of_the_site() {
    let robots = RobotsTxt::for_site(&demo_policy_site())
        .unwrap()
        .to_string();
    assert!(allowed(
        &robots,
        "GPTBot",
        "https://www.example.com/pricing/"
    ));
}

#[test]
fn search_results_are_kept_from_crawlers_unless_the_site_lets_them_in() {
    let mut site = demo_policy_site();
    site.search = SiteSearch::On(path("/search/"));
    let robots = RobotsTxt::for_site(&site).unwrap().to_string();
    let results = "https://www.example.com/blog/search/?q=links";
    assert!(!allowed(&robots, "Googlebot", results), "{robots}");
    assert!(!allowed(&robots, "Claude-SearchBot", results), "{robots}");
    site.crawlers.disallow_search = false;
    let robots = RobotsTxt::for_site(&site).unwrap().to_string();
    assert!(allowed(&robots, "Googlebot", results), "{robots}");
}

#[test]
fn extra_lines_go_after_the_groups_and_before_the_sitemap() {
    let mut site = demo_site();
    site.crawlers.extra_lines = vec![
        RobotsLine::try_new("# Ask before you copy").unwrap(),
        RobotsLine::try_new("Crawl-delay: 5").unwrap(),
    ];
    let robots = RobotsTxt::for_site(&site).unwrap().to_string();
    assert!(
        robots.contains(
            "# Ask before you copy\nCrawl-delay: 5\n\nSitemap: https://www.example.com/blog/sitemap.xml\n"
        ),
        "{robots}"
    );
    for bad in ["Crawl-delay: 5\nDisallow: /", "", "Crawl\tdelay"] {
        assert!(RobotsLine::try_new(bad).is_err(), "{bad:?}");
    }
}

#[test]
fn the_default_lets_everyone_in_and_names_the_sitemap() {
    let mut site = demo_site();
    site.mount = path("/");
    let robots = RobotsTxt::for_site(&site).unwrap().to_string();
    assert!(allowed(
        &robots,
        "Googlebot",
        "https://www.example.com/anything/"
    ));
    assert!(
        robots.contains("Sitemap: https://www.example.com/sitemap.xml"),
        "{robots}"
    );
}

#[test]
fn bot_names_and_paths_are_checked() {
    assert!(matches!(
        UserAgent::try_new("Bad Bot"),
        Err(SeoError::UserAgent { .. })
    ));
    assert!(matches!(
        UserAgent::try_new(""),
        Err(SeoError::UserAgent { .. })
    ));
    assert!(matches!(
        RobotsPath::try_new("preview/"),
        Err(SeoError::RobotsPath { .. })
    ));
    assert!(matches!(
        RobotsPath::try_new("/a b/"),
        Err(SeoError::RobotsPath { .. })
    ));
    assert!(RobotsPath::try_new("/*.pdf$").is_ok());
}

/// The demo site with the "search yes, AI training no" switch on.
fn training_blocked_site() -> SeoSite {
    let mut site = demo_site();
    site.crawlers.ai_training = BotAccess::Blocked;
    site
}

const POST: &str = "https://www.example.com/blog/on-links/";

#[test]
fn ai_training_crawlers_are_allowed_by_default() {
    let robots = RobotsTxt::for_site(&demo_site()).unwrap().to_string();
    insta::assert_snapshot!(robots);
    for crawler in AI_TRAINING_CRAWLERS.iter().chain(&AI_ANSWER_CRAWLERS) {
        assert!(allowed(&robots, crawler.token, POST), "{}", crawler.token);
    }
}

#[test]
fn the_switch_blocks_training_crawlers_and_keeps_search_and_answers() {
    let robots = RobotsTxt::for_site(&training_blocked_site())
        .unwrap()
        .to_string();
    insta::assert_snapshot!(robots);
    for crawler in &AI_TRAINING_CRAWLERS {
        assert!(!allowed(&robots, crawler.token, POST), "{}", crawler.token);
    }
    for crawler in &AI_ANSWER_CRAWLERS {
        assert!(allowed(&robots, crawler.token, POST), "{}", crawler.token);
    }
    for search in ["Googlebot", "Bingbot", "Applebot"] {
        assert!(allowed(&robots, search, POST), "{search}");
    }
}

#[test]
fn an_answer_crawler_blocked_by_hand_joins_the_training_crawlers() {
    let mut site = training_blocked_site();
    site.crawlers
        .bots
        .insert(agent("PerplexityBot"), BotAccess::Blocked);
    let robots = RobotsTxt::for_site(&site).unwrap().to_string();
    insta::assert_snapshot!(robots);
    assert!(!allowed(&robots, "PerplexityBot", POST));
    assert!(!allowed(&robots, "GPTBot", POST));
    assert!(allowed(&robots, "Claude-SearchBot", POST));
}

#[test]
fn a_training_crawler_allowed_by_name_stays_allowed() {
    let mut site = training_blocked_site();
    site.crawlers
        .bots
        .insert(agent("CCBot"), BotAccess::Allowed);
    site.crawlers
        .bots
        .insert(agent("bytespider"), BotAccess::Allowed);
    let robots = RobotsTxt::for_site(&site).unwrap().to_string();
    assert!(allowed(&robots, "CCBot", POST), "{robots}");
    assert!(allowed(&robots, "Bytespider", POST), "{robots}");
    assert!(!allowed(&robots, "GPTBot", POST), "{robots}");
}

#[test]
fn the_two_lists_are_valid_tokens_and_share_none() {
    for crawler in AI_TRAINING_CRAWLERS.iter().chain(&AI_ANSWER_CRAWLERS) {
        assert!(
            UserAgent::try_new(crawler.token).is_ok(),
            "{}",
            crawler.token
        );
        assert!(!crawler.about.is_empty(), "{}", crawler.token);
    }
    for training in &AI_TRAINING_CRAWLERS {
        let twice = (AI_ANSWER_CRAWLERS.iter())
            .any(|answer| answer.token.eq_ignore_ascii_case(training.token));
        assert!(!twice, "{}", training.token);
    }
}
