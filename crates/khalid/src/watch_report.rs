//! `khalid watch`: how fast and how reliably the blog answered at its
//! public address over the last day and the last 30 days, from the looks
//! `khalid serve` keeps (C6). A long gap between looks is time nobody
//! watched, as when the server itself was down. [`SiteUptime`] answers the
//! same report under `/api/admin/v1/watch/` (KH-API-5).

use std::fmt::Write as _;
use std::path::Path;
use std::sync::Arc;

use chrono::{DateTime, Duration, SecondsFormat, Utc};
use khalid_alerts::{Answer, Probe};
use khalid_api::content_store::BoxFuture;
use khalid_api::uptime::{
    UptimeError, UptimeReader, UptimeReport, WatchLook, WatchSpan, WatchedPage,
};
use khalid_core::{Clock, Slug};
use khalid_db::{Database, DbError, SiteId, WatchTable};

use crate::cli::Output;
use crate::db::MAIN;
use crate::exit::Exit;
use crate::site_watch::KEPT_DAYS;
use crate::{db, db_message, report};

/// Said when the watch has no looks yet.
const NO_LOOKS: &str = "The watch has not looked at the blog yet.\nkhalid serve looks at the public address every few minutes once it answers more than this computer.\nSet how often with watch_minutes in [alerts] of site.toml.\n";

/// The watch's looks of one site in the database, for the Admin API.
pub(crate) struct SiteUptime {
    table: WatchTable,
    clock: Arc<dyn Clock>,
}

impl SiteUptime {
    /// The looks of site `id` in `database`, reported at the time `clock`
    /// gives.
    pub(crate) fn new(database: &Database, id: SiteId, clock: Arc<dyn Clock>) -> Self {
        Self {
            table: WatchTable::new(database.clone(), id),
            clock,
        }
    }
}

impl UptimeReader for SiteUptime {
    fn report(&self) -> BoxFuture<'_, Result<UptimeReport, UptimeError>> {
        Box::pin(async move {
            let now = self.clock.now();
            let pages = pages(&self.table, now).await;
            let pages = pages.map_err(|error| UptimeError::Internal(Box::new(error)))?;
            Ok(UptimeReport { pages })
        })
    }
}

/// Prints the report for the `site` folder's database at the time `clock`
/// gives.
pub(crate) fn run(site: &Path, clock: &dyn Clock, output: Output) -> Exit {
    let path = match db::data_file(site) {
        Ok(path) => path,
        Err(message) => return report::failure(output, &message, Exit::Failure),
    };
    let (Ok(main), Ok(runtime)) = (
        Slug::try_new(MAIN),
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build(),
    ) else {
        return report::failure(output, &db_message::internal(), Exit::Internal);
    };
    let now = clock.now();
    match runtime.block_on(read(&path, &main, now)) {
        Ok(pages) if output == Output::Json => report::json(&UptimeReport { pages }, Exit::Success),
        Ok(pages) if pages.is_empty() => report::text(NO_LOOKS, Exit::Success),
        Ok(pages) => report::text(&human(&pages), Exit::Success),
        Err(error) => report::failure(output, &db_message::of(&path, &error), Exit::Failure),
    }
}

/// Every watched page's report at `now`, from the database at `path`.
async fn read(path: &Path, handle: &Slug, now: DateTime<Utc>) -> Result<Vec<WatchedPage>, DbError> {
    let db = Database::open(path).await?;
    let read = async {
        db.migrate().await?;
        let table = WatchTable::new(db.clone(), db.site(handle).await?);
        pages(&table, now).await
    };
    let reports = read.await;
    db.close().await;
    reports
}

/// Every page `table` holds looks at, reported at `now`.
async fn pages(table: &WatchTable, now: DateTime<Utc>) -> Result<Vec<WatchedPage>, DbError> {
    let mut reports = Vec::new();
    for page in table.pages().await? {
        let looks = table.since(&page, now - Duration::days(KEPT_DAYS)).await?;
        let day: Vec<Probe> = looks
            .iter()
            .filter(|look| look.at > now - Duration::days(1))
            .copied()
            .collect();
        reports.push(WatchedPage {
            last_day: span(&day, now),
            last_30_days: span(&looks, now),
            last_look: looks.last().map(look),
            page,
        });
    }
    Ok(reports)
}

/// What `looks`, oldest first, show up to `now`.
fn span(looks: &[Probe], now: DateTime<Utc>) -> WatchSpan {
    let mut times: Vec<u32> = looks
        .iter()
        .filter(|look| look.is_up())
        .map(|look| look.millis)
        .collect();
    times.sort_unstable();
    let ends = looks.iter().map(|look| look.at).chain([now]);
    let gaps = ends.clone().zip(ends.skip(1)).map(|(a, b)| b - a);
    WatchSpan {
        looks: looks.len(),
        answered: times.len(),
        middle_ms: times.get(times.len() / 2).copied(),
        slowest_ms: times.last().copied(),
        longest_gap_minutes: gaps.max().map_or(0, |gap| gap.num_minutes()),
    }
}

/// The newest look as the report shows it.
fn look(probe: &Probe) -> WatchLook {
    let status = match probe.answer {
        Answer::Status(status) => Some(status),
        Answer::Unreached => None,
    };
    WatchLook {
        at: probe.at.to_rfc3339_opts(SecondsFormat::Secs, true),
        status,
        millis: probe.millis,
    }
}

/// The report in sentences, one block per page.
fn human(pages: &[WatchedPage]) -> String {
    let mut text = String::new();
    for page in pages {
        let _ = writeln!(text, "{}", page.page);
        let _ = writeln!(text, "  Last 24 hours: {}", said(&page.last_day));
        let _ = writeln!(text, "  Last 30 days: {}", said(&page.last_30_days));
        if let Some(last) = &page.last_look {
            let got = last.status.map_or_else(
                || "got no answer".to_owned(),
                |status| format!("got status {status}"),
            );
            let _ = writeln!(
                text,
                "  Last look: {}, {got} in {} ms.",
                last.at, last.millis
            );
        }
    }
    text
}

/// One span in a sentence, such as `12 looks, 100.0% answered, middle
/// 120 ms, slowest 300 ms, longest gap 5 minutes.`
fn said(span: &WatchSpan) -> String {
    let Some(per_mille) = span.answered.saturating_mul(1000).checked_div(span.looks) else {
        return "no looks.".to_owned();
    };
    let share = format!("{}.{}%", per_mille / 10, per_mille % 10);
    let times = match (span.middle_ms, span.slowest_ms) {
        (Some(middle), Some(slowest)) => format!(", middle {middle} ms, slowest {slowest} ms"),
        _ => String::new(),
    };
    let gap = span.longest_gap_minutes;
    format!(
        "{} looks, {share} answered{times}, longest gap {gap} minutes.",
        span.looks
    )
}
