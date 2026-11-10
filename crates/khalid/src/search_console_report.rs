//! `khalid search-console report`: each post's numbers for the last 28
//! days Google has final numbers for, posts in decay, and the week's
//! winners and losers, as sentences or JSON.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::Path;

use chrono::NaiveDate;
use khalid_api::search_report::{FallingPage, Mover, PostNumbers, SearchReport};
use khalid_content::SiteContent;
use khalid_core::{Config, Indexing, Locale, Slug};
use khalid_outbound::Outbound;
use khalid_search_console::{
    Change, Decay, PageNumbers, Question, SearchConsole, Window, decaying, go_deeper, movers,
};

use crate::bing::Failed;
use crate::cli::Output;
use crate::exit::Exit;
use crate::search_console::{ask, last_month};
use crate::search_console_lost;
use crate::{build_pages, index, report};

/// How many winners and how many losers a report lists.
const MOVERS: usize = 5;

/// A dated post readers can open, as Search Console names it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LivePost {
    /// Its identity in the Admin API, such as `en.a`.
    pub(crate) id: String,
    pub(crate) title: String,
    /// Its full address, such as `https://www.example.com/blog/a/`.
    pub(crate) address: String,
    /// Whether its front matter keeps it out of search results.
    pub(crate) noindex: bool,
    pub(crate) slug: Slug,
    /// The language it is written in.
    pub(crate) locale: Locale,
    /// Its file in the content index's form, such as
    /// `content/en/posts/a.md`.
    pub(crate) path: String,
}

/// Reads the numbers of the checked site's posts from `console` up to
/// `today` and prints them.
pub(crate) fn run(
    site: (&Path, &Config, &SiteContent),
    asked: (&SearchConsole, &Outbound, NaiveDate),
    output: Output,
) -> Result<Exit, Failed> {
    let body = gather(site, asked)?;
    Ok(match output {
        Output::Json => report::json(&body, Exit::Success),
        Output::Human => report::text(&sentences(&body), Exit::Success),
    })
}

/// Reads the numbers of the checked site's posts from `console` up to
/// `today`, with the site's 404 log.
pub(crate) fn gather(
    (folder, config, content): (&Path, &Config, &SiteContent),
    (console, outbound, today): (&SearchConsole, &Outbound, NaiveDate),
) -> Result<SearchReport, Failed> {
    let posts = live_posts(folder, config, content)?;
    let misses = search_console_lost::misses(folder)?;
    let month = last_month(today);
    let week = Window::ending(month.last, 7).unwrap_or(month);
    let before = week.before().unwrap_or(week);
    let earlier = |window: Window| window.before().unwrap_or(window);
    let (one, two) = (earlier(month), earlier(earlier(month)));
    let three = earlier(two);
    let (this_week, last_week, history) = ask(console, async {
        let session = console.sign_in(outbound).await?;
        let (now, then, h3, h2, h1, h0) = tokio::try_join!(
            session.pages(week),
            session.pages(before),
            session.pages(three),
            session.pages(two),
            session.pages(one),
            session.pages(month),
        )?;
        Ok((now, then, vec![h3, h2, h1, h0]))
    })?;
    let pages = history.last().map_or(&[][..], Vec::as_slice);
    let mut rows: Vec<PostNumbers> = posts.iter().map(|post| numbers(post, pages)).collect();
    rows.sort_by(|a, b| b.clicks.total_cmp(&a.clicks).then(a.title.cmp(&b.title)));
    let week_movers = movers(&last_week, &this_week, MOVERS);
    let root = config.address().as_str().trim_end_matches('/');
    let clicks = |rows: &[PageNumbers]| rows.iter().map(|row| row.clicks).sum();
    Ok(SearchReport {
        lost: search_console_lost::lost(root, &posts, pages, &misses),
        property: console.property().to_owned(),
        first: month.first,
        last: month.last,
        posts: rows,
        week_clicks: clicks(&this_week),
        last_week_clicks: clicks(&last_week),
        decaying: decaying(&history)
            .iter()
            .map(|decay| decay_body(&posts, decay))
            .collect(),
        winners: week_movers
            .winners
            .iter()
            .map(|c| mover(&posts, c, None))
            .collect(),
        losers: week_movers
            .losers
            .iter()
            .map(|c| mover(&posts, c, Some(Question::WhyItDropped)))
            .collect(),
    })
}

/// Every dated post of the site, in the content index's order.
pub(crate) fn live_posts(
    folder: &Path,
    config: &Config,
    content: &SiteContent,
) -> Result<Vec<LivePost>, Failed> {
    let rows =
        index::of_content(folder, config, content).map_err(|message| (message, Exit::Failure))?;
    let hidden: BTreeSet<String> = content
        .posts
        .iter()
        .filter(|doc| matches!(&doc.front.robots, Some(r) if r.indexing == Indexing::NoIndex))
        .filter_map(|doc| index::slashed(&doc.path))
        .collect();
    let root = config.address().as_str().trim_end_matches('/');
    let posts = rows.posts.iter().filter(|row| row.date.is_some());
    Ok(posts
        .map(|row| LivePost {
            id: format!("{}.{}", row.locale, row.slug),
            title: row.title.clone().unwrap_or_else(|| row.slug.to_string()),
            address: format!(
                "{root}{}",
                build_pages::path_of(config, &row.locale, &format!("{}/", row.slug))
            ),
            noindex: hidden.contains(&row.path),
            slug: row.slug.clone(),
            locale: row.locale.clone(),
            path: row.path.clone(),
        })
        .collect())
}

/// `post`'s numbers among `pages`, or none when no search showed it.
fn numbers(post: &LivePost, pages: &[PageNumbers]) -> PostNumbers {
    let found = pages.iter().find(|page| page.key == post.address);
    PostNumbers {
        id: post.id.clone(),
        title: post.title.clone(),
        url: post.address.clone(),
        clicks: found.map_or(0.0, |page| page.clicks),
        impressions: found.map_or(0.0, |page| page.impressions),
        ctr: found.map_or(0.0, |page| page.ctr),
        position: found.map(|page| page.position),
    }
}

/// `change` named by its post's title among `posts`, or by its address,
/// with a link that asks `question` when one is given.
fn mover(posts: &[LivePost], change: &Change, question: Option<Question>) -> Mover {
    let post = posts.iter().find(|post| post.address == change.key);
    Mover {
        id: post.map(|post| post.id.clone()),
        title: post.map_or_else(|| change.key.clone(), |post| post.title.clone()),
        url: change.key.clone(),
        before: change.before,
        now: change.now,
        go_deeper: question.map(|question| go_deeper(question, &change.key)),
    }
}

/// `decay` named by its post's title among `posts`, or by its address.
fn decay_body(posts: &[LivePost], decay: &Decay) -> FallingPage {
    let post = posts.iter().find(|post| post.address == decay.key);
    FallingPage {
        id: post.map(|post| post.id.clone()),
        title: post.map_or_else(|| decay.key.clone(), |post| post.title.clone()),
        url: decay.key.clone(),
        clicks: decay.clicks.clone(),
        action: "Refresh this post".to_owned(),
        go_deeper: go_deeper(Question::WhyItDropped, &decay.key),
    }
}

/// The report as sentences for a person.
fn sentences(body: &SearchReport) -> String {
    let mut text = format!(
        "Search Console numbers for {}, {} to {}:\n",
        body.property, body.first, body.last
    );
    if body.posts.is_empty() {
        text.push_str("The site has no published posts yet.\n");
    }
    for post in &body.posts {
        // Writing to a String cannot fail, so the results are dropped.
        let _ = match post.position {
            Some(position) => writeln!(
                text,
                "- {}: {:.0} clicks, {:.0} impressions, {:.1}% clicked, average position {position:.1}.",
                post.title,
                post.clicks,
                post.impressions,
                post.ctr * 100.0
            ),
            None => writeln!(text, "- {}: no search showed it.", post.title),
        };
        let _ = writeln!(text, "  {}", post.url);
    }
    if !body.decaying.is_empty() {
        text.push_str("\nThese lost clicks three months in a row. Refresh this post:\n");
    }
    for decay in &body.decaying {
        let months: Vec<String> = decay.clicks.iter().map(|c| format!("{c:.0}")).collect();
        let _ = writeln!(
            text,
            "- {}: {} clicks a month, oldest first.\n  Go deeper in SerpDad: {}",
            decay.title,
            months.join(", "),
            decay.go_deeper
        );
    }
    search_console_lost::sentences(&body.lost, &mut text);
    for (heading, list) in [
        ("gained the most clicks", &body.winners),
        ("lost the most clicks", &body.losers),
    ] {
        if !list.is_empty() {
            let _ = writeln!(text, "\nThis week against last week, these {heading}:");
        }
        for change in list {
            let _ = writeln!(
                text,
                "- {}: {:.0} clicks, from {:.0}.",
                change.title, change.now, change.before
            );
            if let Some(link) = &change.go_deeper {
                let _ = writeln!(text, "  Go deeper in SerpDad: {link}");
            }
        }
    }
    text
}
