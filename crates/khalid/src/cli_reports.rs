//! The command line of the commands that read reports about a site:
//! `khalid search-console`, `crawl`, `analytics` and `bing`. Kept apart from
//! `cli` to keep each file short (BL-SIZE-1); `cli` names them all.

use clap::Subcommand;
use khalid_analytics::CRAWL_DAYS_KEPT;
use khalid_core::Slug;

/// The Search Console commands, for the property `[search_console]` names.
#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub(crate) enum SearchConsoleCommand {
    /// Sign in to Google and check that the property can be read
    Check,
    /// Show each post's clicks, impressions, click-through rate and
    /// position for the last 28 days, posts in decay, and the week's
    /// winners and losers
    Report,
    /// Show one post's numbers, the searches that showed it, and whether
    /// Google has it in its index
    Post {
        /// The slug of the published post
        slug: Slug,
    },
    /// Ask Google whether each published post is in its index, up to 200
    /// posts a run, within Google's daily quota of 2,000
    Index,
    /// List the long searches of 8 words or more, the way people ask an
    /// AI assistant, that showed each post in the last 28 days, and
    /// whether a heading of the post answers each
    FollowUps,
    /// Email the week in Google search to `report_to` under `[search_console]`,
    /// through `[mail]`. Run it once a week from a timer
    Email,
}

/// The crawl reports. They read the visitor log in `data/site.db` and
/// count a crawler only from an address its company publishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Subcommand)]
pub(crate) enum CrawlCommand {
    /// Show how long each new post waited for its first crawl
    First,
    /// Show each search and AI crawler's visits and what it got
    Health,
    /// List the addresses that answered crawlers with an error
    Errors,
}

/// The analytics commands. They read `data/site.db`, so they work while
/// `khalid serve` runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Subcommand)]
pub(crate) enum AnalyticsCommand {
    /// Show how many readers each AI assistant, such as `ChatGPT` or
    /// Perplexity, sent in the last days, against the days before
    Ai {
        /// How many days to count back from today, from 1 to 366
        #[arg(long, default_value_t = 28, value_parser = clap::value_parser!(u32).range(1..=366))]
        days: u32,
    },
    /// List the pages AI bots read but no AI assistant sent a reader to
    Uncited {
        /// How many days back to look, today included, from 1 to 90
        #[arg(long, default_value_t = 30, value_parser = clap::value_parser!(u32).range(1..=i64::from(CRAWL_DAYS_KEPT)))]
        days: u32,
    },
}

/// The Bing Webmaster Tools commands, for the site at the address in
/// `site.toml`, as verified in Bing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Subcommand)]
pub(crate) enum BingCommand {
    /// Hand Bing the site's sitemap, so it reads every page from it
    Sitemap,
    /// Show Bing's last crawl day, the sitemaps it holds and the addresses
    /// it had trouble with
    Report,
}
