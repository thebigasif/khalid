//! Bots per AI tool: how often each assistant's bots read the site, beside
//! the readers the assistant sent.

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use khalid_analytics::{
    AiTool, BotReads, Crawler, CrawlerKind, Crawls, ReferrerGroup, Views, ai_bots,
};

#[cfg(test)]
fn at(day: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, day, 9, 0, 0).unwrap()
}

#[cfg(test)]
fn read(path: &str, crawler: &str, hits: u64, status: u16) -> Crawls {
    let kind = Crawler::named(crawler).map_or(CrawlerKind::Other, Crawler::kind);
    Crawls {
        day: at(27).date_naive(),
        path: path.to_owned(),
        crawler: crawler.to_owned(),
        kind,
        hits,
        last_seen: at(27),
        last_status: status,
    }
}

#[cfg(test)]
fn sent(tool: AiTool, views: u64) -> Views {
    let day = NaiveDate::from_ymd_opt(2026, 10, 27).unwrap();
    Views::new(day, "/blog/a/", ReferrerGroup::Ai(tool), views)
}

#[cfg(test)]
fn bot(name: &str, reads: u64) -> BotReads {
    BotReads {
        bot: name.to_owned(),
        reads,
    }
}

#[test]
fn each_assistants_bots_are_counted_together_beside_the_readers_it_sent() {
    let crawls = [
        read("/blog/a/", "GPTBot", 5, 200),
        read("/blog/b/", "GPTBot", 2, 304),
        read("/blog/a/", "OAI-SearchBot", 3, 200),
        read("/blog/a/", "ClaudeBot", 4, 200),
    ];
    let views = [sent(AiTool::ChatGpt, 6), sent(AiTool::Gemini, 9)];
    let tools = ai_bots(&crawls, &views);
    let summary: Vec<_> = tools
        .iter()
        .map(|t| (t.tool, t.reads, t.pages, t.sent))
        .collect();
    assert_eq!(
        summary,
        [
            (Some(AiTool::ChatGpt), 10, 2, 6),
            (Some(AiTool::Claude), 4, 1, 0),
        ]
    );
    assert_eq!(
        tools.first().map(|t| t.bots.clone()),
        Some(vec![bot("GPTBot", 7), bot("OAI-SearchBot", 3)])
    );
}

#[test]
fn bots_of_no_assistant_come_last_and_failed_reads_files_and_other_bots_are_left_out() {
    let crawls = [
        read("/blog/a/", "CCBot", 9, 200),
        read("/blog/a/", "Bytespider", 1, 200),
        read("/blog/a/", "PerplexityBot", 9, 200),
        read("/blog/gone/", "PerplexityBot", 7, 404),
        read("/robots.txt", "PerplexityBot", 7, 200),
        read("/blog/a/", "Googlebot", 50, 200),
    ];
    let tools = ai_bots(&crawls, &[]);
    let summary: Vec<_> = tools.iter().map(|t| (t.tool, t.reads)).collect();
    assert_eq!(summary, [(Some(AiTool::Perplexity), 9), (None, 10)]);
    assert_eq!(
        tools.last().map(|t| t.bots.clone()),
        Some(vec![bot("CCBot", 9), bot("Bytespider", 1)])
    );
}

#[test]
fn no_ai_bot_reads_means_an_empty_list() {
    assert_eq!(ai_bots(&[], &[sent(AiTool::Claude, 3)]).len(), 0);
}

#[test]
fn every_bot_named_for_an_assistant_is_a_known_ai_bot() {
    for (name, tool) in khalid_analytics::AI_BOT_MAKERS {
        let crawler = Crawler::named(name);
        assert_eq!(crawler.map(Crawler::kind), Some(CrawlerKind::Ai), "{name}");
        assert_ne!(*tool, AiTool::Unnamed, "{name}");
    }
}
