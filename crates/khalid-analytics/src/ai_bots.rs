//! Bots per AI tool: how often each AI assistant's bots read the site,
//! beside how many readers the assistant sent in the same days.
//!
//! An assistant's company sends bots to read pages, for training, for its
//! search index or to answer a person right now. Reads alone say the
//! assistant knows the site; the readers it sent say whether it points
//! people to it.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};

use crate::uncited::{is_page, read_ok};
use crate::{AiTool, CrawlerKind, Crawls, ReferrerGroup, Views};

/// Each AI bot named for the assistant its company runs, by the name the
/// crawl log shows. An AI bot not listed here belongs to no assistant
/// Khalid names, such as `CCBot`.
pub const AI_BOT_MAKERS: &[(&str, AiTool)] = &[
    ("GPTBot", AiTool::ChatGpt),
    ("ChatGPT-User", AiTool::ChatGpt),
    ("OAI-SearchBot", AiTool::ChatGpt),
    ("ClaudeBot", AiTool::Claude),
    ("Claude-User", AiTool::Claude),
    ("Claude-SearchBot", AiTool::Claude),
    ("anthropic-ai", AiTool::Claude),
    ("PerplexityBot", AiTool::Perplexity),
    ("Perplexity-User", AiTool::Perplexity),
    ("MistralAI-User", AiTool::LeChat),
    ("Meta-ExternalAgent", AiTool::MetaAi),
    ("YouBot", AiTool::You),
];

/// One bot's reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BotReads {
    /// The bot's name as its user agent claimed it, such as `GPTBot`.
    pub bot: String,
    /// How many times it read a page.
    pub reads: u64,
}

/// What one assistant's bots read, and the readers the assistant sent.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ToolBots {
    /// The assistant, or `None` for AI bots of no assistant Khalid names.
    pub tool: Option<AiTool>,
    /// How many times its bots read a page.
    pub reads: u64,
    /// How many different pages its bots read.
    pub pages: u64,
    /// The page views of readers the assistant sent in the same days.
    pub sent: u64,
    /// Its bots, most reads first, then by name.
    pub bots: Vec<BotReads>,
}

/// One assistant's counts while the list is built.
#[derive(Debug, Default)]
struct Counting<'a> {
    reads: u64,
    pages: BTreeSet<&'a str>,
    bots: BTreeMap<&'a str, u64>,
}

/// The AI bots' reads in `crawls`, per assistant, with the readers each
/// assistant sent in `views`. Named assistants come first, most reads
/// first; the bots of no named assistant come last, as one row.
///
/// A read is counted as in [`crate::uncited`]: a page, not a file, whose
/// latest answer that day was a success or "not modified". An assistant
/// whose bots read nothing is left out. Give both lists for the same days.
pub fn ai_bots(crawls: &[Crawls], views: &[Views]) -> Vec<ToolBots> {
    let mut tools: BTreeMap<Option<AiTool>, Counting<'_>> = BTreeMap::new();
    let reads = crawls
        .iter()
        .filter(|row| row.kind == CrawlerKind::Ai && read_ok(row.last_status))
        .filter(|row| is_page(&row.path));
    for row in reads {
        let counting = tools.entry(maker(&row.crawler)).or_default();
        counting.reads = counting.reads.saturating_add(row.hits);
        counting.pages.insert(&row.path);
        let bot = counting.bots.entry(&row.crawler).or_default();
        *bot = bot.saturating_add(row.hits);
    }
    let mut listed: Vec<ToolBots> = tools
        .into_iter()
        .map(|(tool, counting)| finished(tool, counting, views))
        .collect();
    // Stable, so assistants read as often keep their own order.
    listed.sort_by_key(|row| (row.tool.is_none(), Reverse(row.reads)));
    listed
}

/// The assistant whose company runs the bot named `crawler`.
fn maker(crawler: &str) -> Option<AiTool> {
    AI_BOT_MAKERS
        .iter()
        .find(|(name, _)| *name == crawler)
        .map(|&(_, tool)| tool)
}

fn finished(tool: Option<AiTool>, counting: Counting<'_>, views: &[Views]) -> ToolBots {
    let sent = tool.map_or(0, |tool| {
        views
            .iter()
            .filter(|row| row.group == ReferrerGroup::Ai(tool))
            .fold(0_u64, |sum, row| sum.saturating_add(row.views))
    });
    let mut bots: Vec<BotReads> = counting
        .bots
        .into_iter()
        .map(|(bot, reads)| BotReads {
            bot: bot.to_owned(),
            reads,
        })
        .collect();
    // Stable, so bots read as often stay in name order.
    bots.sort_by_key(|bot| Reverse(bot.reads));
    ToolBots {
        tool,
        reads: counting.reads,
        pages: u64::try_from(counting.pages.len()).unwrap_or(u64::MAX),
        sent,
        bots,
    }
}
