//! The Search Console report the Admin API answers (G43, KH-GRO-5): each
//! post's numbers for the last 28 days Google has final numbers for, the
//! posts whose clicks keep falling, pages that get search traffic though
//! readers cannot use them, and the week's winners and losers.
//! `khalid search-console report --json` prints the same shape. The
//! Analytics screen reads the whole site's totals for a period.

use chrono::NaiveDate;
use serde::Serialize;
use utoipa::ToSchema;

/// A site's Search Console numbers.
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub struct SearchReport {
    /// The Search Console property the numbers come from.
    #[schema(example = "sc-domain:example.com")]
    pub property: String,
    /// The first day of the month looked at.
    pub first: NaiveDate,
    /// The last day looked at: the last day whose numbers are final.
    pub last: NaiveDate,
    /// Every published post, the one with the most clicks first.
    pub posts: Vec<PostNumbers>,
    /// Pages whose clicks fell three 28-day months in a row.
    pub decaying: Vec<FallingPage>,
    /// Pages that still get search traffic though readers cannot use them.
    pub lost: Vec<LostPage>,
    /// Clicks on every page in the last 7 days.
    pub week_clicks: f64,
    /// Clicks on every page in the 7 days before.
    pub last_week_clicks: f64,
    /// The pages that gained the most clicks this week against last.
    pub winners: Vec<Mover>,
    /// The pages that lost the most clicks this week against last.
    pub losers: Vec<Mover>,
}

/// Each published post's numbers for the month and nothing else, which
/// the posts table shows beside each post. One request to Google finds
/// them, where the whole report needs six.
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub struct SearchPosts {
    /// The Search Console property the numbers come from.
    #[schema(example = "sc-domain:example.com")]
    pub property: String,
    /// The first day of the month looked at.
    pub first: NaiveDate,
    /// The last day looked at: the last day whose numbers are final.
    pub last: NaiveDate,
    /// Every published post, the one with the most clicks first.
    pub posts: Vec<PostNumbers>,
}

/// One post's numbers for the month.
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub struct PostNumbers {
    /// The post's identity, as in `/posts/{id}/`.
    #[schema(example = "en.what-a-sitemap-does")]
    pub id: String,
    /// The post's title.
    pub title: String,
    /// The post's full address.
    #[schema(example = "https://example.com/blog/what-a-sitemap-does/")]
    pub url: String,
    /// Clicks from Google search.
    pub clicks: f64,
    /// Times a search showed it.
    pub impressions: f64,
    /// Clicks for each time shown, from 0 to 1.
    pub ctr: f64,
    /// Its average position in results, 1 at the top, or `null` when no
    /// search showed it.
    pub position: Option<f64>,
}

/// A page whose clicks fell three months in a row.
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub struct FallingPage {
    /// The post's identity, or `null` for a page that is not a post.
    pub id: Option<String>,
    /// The post's title, or the page's address.
    pub title: String,
    /// The page's full address.
    pub url: String,
    /// Clicks in each 28-day month, oldest first.
    pub clicks: Vec<f64>,
    /// What to do about it.
    #[schema(example = "Refresh this post")]
    pub action: String,
    /// The go-deeper link, which asks why it dropped.
    pub go_deeper: String,
}

/// Why a page with search traffic cannot be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum LostReason {
    /// The address answers 404, not found.
    Missing,
    /// The post is marked `noindex`.
    Noindex,
}

/// A page that gets search traffic though readers cannot use it.
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub struct LostPage {
    /// The page's full address.
    pub url: String,
    /// Clicks from Google search in the month.
    pub clicks: f64,
    /// Times a search showed it in the month.
    pub impressions: f64,
    /// Why readers cannot use it.
    pub why: LostReason,
    /// How many requests the 404 log counted, for a missing page.
    pub not_found_hits: Option<u64>,
}

/// One page's clicks this week against last week.
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub struct Mover {
    /// The post's identity, or `null` for a page that is not a post.
    pub id: Option<String>,
    /// The post's title, or the page's address.
    pub title: String,
    /// The page's full address.
    pub url: String,
    /// Clicks last week.
    pub before: f64,
    /// Clicks this week.
    pub now: f64,
    /// The go-deeper link, which asks why it dropped, for a loser.
    pub go_deeper: Option<String>,
}

/// What Google search did for the whole site in a period, against the
/// period just as long before it.
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub struct SearchTotals {
    /// The Search Console property the numbers come from.
    #[schema(example = "sc-domain:example.com")]
    pub property: String,
    /// How many days the period has.
    #[schema(example = 28)]
    pub days: u32,
    /// The first day of the period.
    pub first: NaiveDate,
    /// The last day of the period: the last day whose numbers are final,
    /// about three days ago.
    pub last: NaiveDate,
    /// Every page together in the period.
    pub now: SearchNumbers,
    /// Every page together in the days just before.
    pub before: SearchNumbers,
}

/// Every page's numbers added up.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, ToSchema)]
pub struct SearchNumbers {
    /// Clicks from Google search.
    pub clicks: f64,
    /// Times a search showed a page.
    pub impressions: f64,
    /// Clicks for each time shown, from 0 to 1.
    pub ctr: f64,
    /// The average position in results, 1 at the top, or `null` when no
    /// search showed a page.
    pub position: Option<f64>,
}
