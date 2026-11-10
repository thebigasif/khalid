//! The speed and uptime watch (C6): `khalid serve` looks at the blog's
//! home page at its public address every few minutes, as a reader would,
//! through DNS, the certificate and any proxy in front. Each look is kept
//! for 30 days, and three failed looks in a row, or slow recent looks,
//! raise an alert. A server that is down cannot say so itself: its looks stop.

use std::time::Duration;

use chrono::{DateTime, Duration as Days, Utc};
use khalid_alerts::{Answer, Kind, Probe, site_down, slow_site};
use khalid_core::{AbsoluteUrl, Config};
use khalid_db::{Database, DbError, SiteId, WatchTable};
use khalid_outbound::{Outbound, OutboundError};
use tokio::time::Instant;

use crate::alerts::Alerting;
use crate::cli::ServeArgs;

/// Looks are kept this many days.
pub(crate) const KEPT_DAYS: i64 = 30;

/// The looks read back: three at the rarest watch, and a margin.
const READ_BACK: Days = Days::hours(4);

/// How often the watch looks for `args` and the site with `config`, or
/// `None`: every `[alerts] watch_minutes`, but never while the server
/// answers only this computer, as it does by default while writing. A
/// test build may ask with the hidden `--watch-every`.
pub(crate) fn every(args: &ServeArgs, config: &Config) -> Option<Duration> {
    #[cfg(feature = "test-util")]
    if let Some(seconds) = args.test.watch_every {
        return Some(Duration::from_secs(seconds));
    }
    let minutes = config.alerts().watch_minutes;
    let only_here = args.domain.is_none() && args.listen.ip().is_loopback();
    if minutes == 0 || only_here {
        return None;
    }
    Some(Duration::from_secs(u64::from(minutes) * 60))
}

/// The blog's home page at its public address, and where its looks go.
#[derive(Debug)]
pub(crate) struct SiteWatch {
    outbound: Outbound,
    page: AbsoluteUrl,
    table: WatchTable,
    slow_ms: u32,
    /// How often the settings ask the watch to look, which sizes the
    /// window a slow site is judged on.
    every: Days,
}

impl SiteWatch {
    /// The watch of the site with `config`, kept in site `id` of
    /// `database`, looking through `outbound`. `None` when the address and
    /// the mount make no address, which the settings check refuses first.
    pub(crate) fn new(
        config: &Config,
        outbound: &Outbound,
        (database, id): (&Database, SiteId),
    ) -> Option<Self> {
        let base = config.address().as_str().trim_end_matches('/');
        let page = AbsoluteUrl::parse(&format!("{base}{}", config.mount())).ok()?;
        Some(Self {
            outbound: outbound.clone(),
            page,
            table: WatchTable::new(database.clone(), id),
            slow_ms: config.alerts().slow_ms,
            every: Days::minutes(i64::from(config.alerts().watch_minutes)),
        })
    }

    /// Looks once, keeps the look, and raises what the recent looks show
    /// through `alerting`. A failure to keep or read looks is logged, and
    /// the next round tries again.
    pub(crate) async fn round(&self, alerting: &Alerting) {
        let at = alerting.now();
        let Ok(probe) = self
            .look(at)
            .await
            .inspect_err(|said| tracing::warn!("{said}"))
        else {
            return;
        };
        let page = self.page.as_str();
        let Ok(probes) = self.keep(&probe, at).await.inspect_err(|error| {
            tracing::warn!(%error, "the speed and uptime watch could not keep its looks");
        }) else {
            return;
        };
        alerting
            .raise(Kind::SiteDown, site_down(&probes, page))
            .await;
        let slow = slow_site(&probes, self.slow_ms, self.every, page, at);
        alerting.raise(Kind::SlowSite, slow).await;
    }

    /// Keeps `probe`, drops looks older than 30 days at `at`, and reads
    /// back the recent ones, oldest first.
    async fn keep(&self, probe: &Probe, at: DateTime<Utc>) -> Result<Vec<Probe>, DbError> {
        let page = self.page.as_str();
        self.table.record(page, probe).await?;
        self.table.forget_before(at - Days::days(KEPT_DAYS)).await?;
        self.table.since(page, at - READ_BACK).await
    }

    /// One look at the page, started `at`: its status and how long the
    /// whole answer took. When the guard refuses the address, because it
    /// is on a private network, no reader outside sees it, so there is
    /// nothing to watch: the error says so in plain words, for the log.
    async fn look(&self, at: DateTime<Utc>) -> Result<Probe, String> {
        let started = Instant::now();
        let answer = match self.outbound.get(&self.page).await {
            Ok(fetched) => Answer::Status(fetched.status()),
            Err(OutboundError::Blocked { .. }) => {
                return Err(format!(
                    "The speed and uptime watch did not look at {}, because it is a private address no reader outside can reach. Set address in site.toml to the blog's public address to watch it.",
                    self.page
                ));
            }
            Err(_) => Answer::Unreached,
        };
        let millis = u32::try_from(started.elapsed().as_millis()).unwrap_or(u32::MAX);
        Ok(Probe { at, answer, millis })
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use clap::Parser;
    use khalid_core::{Environment, Slug};
    use khalid_outbound::Limits;

    use super::*;
    use crate::cli::{Cli, Command};

    const SITE: &str = "name = \"Field Notes\"\naddress = \"https://www.example.com\"\ndefault_language = \"en\"\nlanguages = [\"en\"]\n";

    fn every_for(line: &[&str], alerts: &str) -> Option<Duration> {
        let text = format!("{SITE}{alerts}");
        let config = Config::from_toml(Path::new("site.toml"), &text, &Environment::empty());
        let cli = Cli::try_parse_from(line).unwrap();
        let Some(Command::Serve(args)) = cli.command else {
            panic!("not serve");
        };
        every(&args, &config.unwrap())
    }

    #[test]
    fn the_watch_looks_only_when_the_server_answers_more_than_this_computer() {
        let public = ["khalid", "serve", "--listen", "0.0.0.0:8080"];
        assert_eq!(every_for(&public, ""), Some(Duration::from_secs(300)));
        let rare = "\n[alerts]\nwatch_minutes = 60\n";
        assert_eq!(every_for(&public, rare), Some(Duration::from_secs(3600)));
        assert_eq!(every_for(&public, "\n[alerts]\nwatch_minutes = 0\n"), None);
        let https = ["khalid", "serve", "--domain", "blog.example.com"];
        assert_eq!(every_for(&https, ""), Some(Duration::from_secs(300)));
        assert_eq!(every_for(&["khalid", "serve"], ""), None, "writing here");
        let here = ["khalid", "serve", "--listen", "[::1]:8080"];
        assert_eq!(every_for(&here, ""), None);
    }

    #[tokio::test]
    async fn a_private_address_is_not_looked_at_and_the_log_says_why() {
        let dir = tempfile::tempdir().unwrap();
        let mut path = dir.path().to_path_buf();
        path.push("site.db");
        let db = Database::open(&path).await.unwrap();
        db.migrate().await.unwrap();
        let site = db.site(&Slug::try_new("main").unwrap()).await.unwrap();
        let text = SITE.replace("https://www.example.com", "http://192.168.1.20");
        let env = Environment::empty();
        let config = Config::from_toml(Path::new("site.toml"), &text, &env).unwrap();
        let outbound = Outbound::new(Limits::default()).unwrap();
        let watch = SiteWatch::new(&config, &outbound, (&db, site)).unwrap();

        let said = watch.look(DateTime::UNIX_EPOCH).await.unwrap_err();
        assert_eq!(
            said,
            "The speed and uptime watch did not look at http://192.168.1.20/, because it is a private address no reader outside can reach. Set address in site.toml to the blog's public address to watch it."
        );
    }
}
