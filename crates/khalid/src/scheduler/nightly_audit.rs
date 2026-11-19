//! The nightly site audit in `khalid serve` (plan 5.10): once an hour the
//! scheduler asks whether a day has passed since the last audit, by
//! `khalid audit` or by the server, and runs one when it has. It audits the
//! site built into `data/audit/site/`, offline, and keeps the score in
//! `data/audit/runs.jsonl` for the dashboard. Like `khalid audit`, it then
//! fires `audit.finished` and raises the blog's broken links to the owner.
//! Stopping the server does not wait for it: the lock goes at once and the
//! audit ends after the step in progress. A site with no pages has nothing
//! to crawl and gets no audit.

use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::{DateTime, TimeDelta, Utc};
use khalid_alerts::Kind;
use khalid_audit::{Report, Run};
use khalid_content::{ContentTree, read_content};
use khalid_core::{Clock, Config, UserMessage};
use khalid_routes::Routes;
use tokio::runtime::Handle;

use crate::alerts::Alerting;
use crate::serve_https::ServeClock;
use crate::site_audit::AuditHold;
use crate::{audit_alert, content_message, site_audit};

/// How long after one audit the next falls due.
const DAY: TimeDelta = TimeDelta::hours(24);

/// Where an `audit.finished` delivery goes: the site's announcer in
/// `khalid serve`, a list in tests.
pub(crate) type Finished = Box<dyn Fn(&Report) + Send + Sync>;

/// What the nightly audit needs, read once at start.
pub(crate) struct NightlyAudit {
    folder: PathBuf,
    config: Config,
    routes: Routes,
    finished: Finished,
    hold: AuditHold,
}

impl fmt::Debug for NightlyAudit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NightlyAudit")
            .field("folder", &self.folder)
            .finish_non_exhaustive()
    }
}

impl NightlyAudit {
    /// The nightly audit of the site in `folder` with `config` and
    /// `routes`, telling `finished` about each run it keeps, or `None` for
    /// a site with no pages.
    pub(crate) fn new(
        folder: &Path,
        (config, routes): (&Config, &Routes),
        finished: Finished,
    ) -> Option<Arc<Self>> {
        config.mode().has_pages().then(|| {
            Arc::new(Self {
                folder: folder.to_path_buf(),
                config: config.clone(),
                routes: routes.clone(),
                finished,
                hold: AuditHold::default(),
            })
        })
    }

    /// Runs the audit when a day has passed since the last one by `clock`,
    /// logs what happened, and raises its broken links through
    /// `alerting`. It builds and reads the whole site, so it runs on a
    /// blocking thread (RULES.md BL-RUN-7).
    pub(crate) async fn round(self: &Arc<Self>, clock: ServeClock, alerting: &Alerting) {
        let audit = Arc::clone(self);
        let handle = Handle::current();
        let done = tokio::task::spawn_blocking(move || audit.run_if_due(&clock, &handle)).await;
        match done {
            Ok(Some(report)) => self.tell(&report, alerting).await,
            Ok(None) => {}
            Err(error) => tracing::warn!(%error, "the nightly site audit stopped early"),
        }
    }

    /// Lets go of the audit's lock and ends an audit in progress after its
    /// current step, as the server stops.
    pub(crate) fn stop(&self) {
        self.hold.stop();
    }

    /// Fires `audit.finished` about `report` and raises its broken links,
    /// unless the server is stopping.
    async fn tell(&self, report: &Report, alerting: &Alerting) {
        if self.hold.stopped() {
            return;
        }
        (self.finished)(report);
        alerting
            .raise(Kind::BrokenLinks, audit_alert::findings(report))
            .await;
    }

    /// Runs the audit when it is due, and returns its report.
    fn run_if_due(&self, clock: &dyn Clock, handle: &Handle) -> Option<Report> {
        if !due(&site_audit::runs(&self.folder), clock.now()) {
            return None;
        }
        match self.run(clock, handle) {
            Ok((run, report)) => {
                tracing::info!(
                    score = run.score,
                    errors = run.errors,
                    "the nightly site audit ran"
                );
                Some(report)
            }
            Err(_) if self.hold.stopped() => None,
            Err(message) => {
                tracing::warn!(
                    %message,
                    "the nightly site audit did not run, so it is tried again in an hour"
                );
                None
            }
        }
    }

    /// Reads the site's content as it is now and audits it.
    fn run(&self, clock: &dyn Clock, handle: &Handle) -> Result<(Run, Report), UserMessage> {
        let content = ContentTree::walk(&self.folder)
            .and_then(|tree| read_content(&tree))
            .map_err(|error| content_message::of(&self.folder, &error))?;
        let checked = (&self.config, &self.routes, &content);
        site_audit::nightly(&self.folder, checked, (clock, handle), &self.hold)
    }
}

/// Whether an audit is due at `now`: none has run yet in `runs`, or the
/// last ran a day or more before.
fn due(runs: &[Run], now: DateTime<Utc>) -> bool {
    runs.last().is_none_or(|last| now - last.at >= DAY)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::sync::{Arc, Mutex, mpsc};

    use chrono::{DateTime, TimeDelta, Utc};
    use khalid_alerts::{Alert, Kind, MemoryRaised, Raised};
    use khalid_audit::{Report, Run};
    use khalid_core::{Clock, Config, Environment, FixedClock};
    use khalid_routes::Routes;

    use super::{Finished, NightlyAudit, due};
    use crate::alerts::{Alerting, Fire};
    use crate::{audit_events, serve, site_audit};

    /// A run kept at `at`, as the history file reads it back.
    fn run_at(at: DateTime<Utc>) -> Run {
        let line = format!(
            r#"{{"at":"{}","score":100,"pages":1,"errors":0,"warnings":0,"notices":0}}"#,
            at.to_rfc3339()
        );
        khalid_audit::history(&line).remove(0)
    }

    /// A fixed moment, since tests never read the computer's clock.
    fn noon() -> DateTime<Utc> {
        DateTime::from_timestamp(1_794_484_800, 0).unwrap_or_default()
    }

    #[test]
    fn a_site_never_audited_is_due() {
        assert!(due(&[], noon()));
    }

    #[test]
    fn an_audit_waits_a_day_after_the_last() {
        let last = noon();
        let runs = [run_at(last - TimeDelta::days(3)), run_at(last)];
        assert!(!due(&runs, last + TimeDelta::hours(23)));
        assert!(due(&runs, last + TimeDelta::hours(24)));
    }

    #[test]
    fn the_server_audits_the_demo_site_once_a_day_and_keeps_its_score() {
        let site = demo_copy();
        let config = Config::load(site.path(), &Environment::empty()).unwrap();
        let routes = Routes::load(site.path()).unwrap();
        let audit = NightlyAudit::new(site.path(), (&config, &routes), quiet()).unwrap();
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();
        let at = |hours| FixedClock::new(noon() + TimeDelta::hours(hours));

        audit.run_if_due(&at(0), runtime.handle());
        audit.run_if_due(&at(23), runtime.handle());
        let runs = site_audit::runs(site.path());
        assert_eq!(runs.len(), 1, "a second audit waits a day");
        assert_eq!(
            (runs[0].at, runs[0].score, runs[0].errors),
            (noon(), 100, 0)
        );
        assert!(runs[0].pages > 0);
        let mut issues = site.path().to_path_buf();
        issues.push("data/audit/issues.csv");
        assert!(issues.exists(), "the issues are kept as CSV too");

        audit.run_if_due(&at(24), runtime.handle());
        assert_eq!(site_audit::runs(site.path()).len(), 2);
    }

    #[test]
    fn the_nightly_audit_waits_while_khalid_audit_runs() {
        let site = demo_copy();
        let config = Config::load(site.path(), &Environment::empty()).unwrap();
        let routes = Routes::load(site.path()).unwrap();
        let audit = NightlyAudit::new(site.path(), (&config, &routes), quiet()).unwrap();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let mut lock = site.path().to_path_buf();
        lock.push("data/audit");
        fs::create_dir_all(&lock).unwrap();
        lock.push("running");
        fs::write(&lock, noon().to_rfc3339()).unwrap();

        audit.run_if_due(&FixedClock::new(noon()), runtime.handle());
        assert_eq!(site_audit::runs(site.path()).len(), 0, "no audit ran");
        assert!(lock.exists(), "the other audit keeps its lock");
    }

    #[test]
    fn the_nightly_audit_fires_audit_finished_and_raises_a_broken_link_once() {
        let site = demo_copy();
        let mut post = site.path().to_path_buf();
        post.push("content/en/posts/2026-09-14-why-internal-links-matter.md");
        let text = fs::read_to_string(&post).unwrap();
        let broken = text.replacen(
            "Start with one link per post.",
            "Start with one link per post, such as [our old guide](/blog/old-guide/).",
            1,
        );
        assert_ne!(
            text, broken,
            "the sentence to change is gone from the demo post"
        );
        fs::write(&post, broken).unwrap();
        let config = Config::load(site.path(), &Environment::empty()).unwrap();
        let routes = Routes::load(site.path()).unwrap();
        let scores = Arc::new(Mutex::new(Vec::new()));
        let kept = Arc::clone(&scores);
        let finished: Finished = Box::new(move |report: &Report| {
            let body = serde_json::to_value(audit_events::finished(report)).unwrap();
            kept.lock().unwrap().push(body);
        });
        let audit = NightlyAudit::new(site.path(), (&config, &routes), finished).unwrap();
        let (alerting, raised) = alerting(&config);
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        for hours in [0, 24] {
            let clock = FixedClock::new(noon() + TimeDelta::hours(hours));
            let report = audit.run_if_due(&clock, runtime.handle()).unwrap();
            runtime.block_on(audit.tell(&report, &alerting));
        }

        let scores = scores.lock().unwrap().clone();
        assert_eq!(scores.len(), 2, "audit.finished after each run");
        assert!(
            scores[0]["audit"]["errors"].as_u64().unwrap() > 0,
            "{}",
            scores[0]
        );
        let raised = raised.lock().unwrap().clone();
        assert_eq!(raised.len(), 1, "the same broken link is raised once");
        assert_eq!(raised[0].kind(), Kind::BrokenLinks);
        let line = &raised[0].findings()[0].line;
        assert!(line.contains("/blog/old-guide/"), "{line}");
    }

    #[test]
    fn a_stopping_server_starts_no_audit_and_tells_no_one() {
        let site = demo_copy();
        let config = Config::load(site.path(), &Environment::empty()).unwrap();
        let routes = Routes::load(site.path()).unwrap();
        let told = Arc::new(Mutex::new(0_u32));
        let kept = Arc::clone(&told);
        let finished: Finished = Box::new(move |_: &Report| *kept.lock().unwrap() += 1);
        let audit = NightlyAudit::new(site.path(), (&config, &routes), finished).unwrap();
        let (alerting, raised) = alerting(&config);
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let report = audit.run_if_due(&FixedClock::new(noon()), runtime.handle());
        audit.stop();
        runtime.block_on(audit.tell(&report.unwrap(), &alerting));
        let a_day_on = FixedClock::new(noon() + TimeDelta::hours(24));
        assert!(audit.run_if_due(&a_day_on, runtime.handle()).is_none());
        assert_eq!(
            site_audit::runs(site.path()).len(),
            1,
            "no second audit ran"
        );
        assert_eq!(
            *told.lock().unwrap(),
            0,
            "an audit that ends at stop is not told"
        );
        assert!(raised.lock().unwrap().is_empty(), "nor raised");
    }

    #[test]
    fn stopping_the_server_does_not_wait_for_an_audit_build() {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .build()
            .unwrap();
        // Stands in for a build of a large site: it runs until the test
        // lets it end, so a stop that waited for it would never return.
        let (release, building) = mpsc::channel::<()>();
        drop(runtime.spawn_blocking(move || building.recv()));
        serve::wind_down(runtime);
        drop(release);
    }

    /// Alerting for `config` that keeps each alert it fires in a list.
    fn alerting(config: &Config) -> (Alerting, Arc<Mutex<Vec<Alert>>>) {
        let fired = Arc::new(Mutex::new(Vec::new()));
        let kept = Arc::clone(&fired);
        let fire: Fire = Box::new(move |alert: &Alert| kept.lock().unwrap().push(alert.clone()));
        let clock: Arc<dyn Clock> = Arc::new(FixedClock::new(noon()));
        let raised: Arc<dyn Raised> = Arc::new(MemoryRaised::default());
        (Alerting::new(config, (raised, clock), None, fire), fired)
    }

    /// An `audit.finished` that goes nowhere.
    fn quiet() -> Finished {
        Box::new(|_: &Report| {})
    }

    /// The demo site in a folder of its own, so the audit's `data/` stays
    /// out of the fixture.
    fn demo_copy() -> tempfile::TempDir {
        let mut demo = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        demo.push("../../fixtures/demo-site");
        let site = tempfile::tempdir().unwrap();
        copy_dir(&demo, site.path());
        site
    }

    fn copy_dir(from: &Path, into: &Path) {
        fs::create_dir_all(into).unwrap();
        for entry in fs::read_dir(from).unwrap() {
            let entry = entry.unwrap();
            if entry.file_name() == "data" {
                continue;
            }
            let mut target = into.to_path_buf();
            target.push(entry.file_name());
            if entry.file_type().unwrap().is_dir() {
                copy_dir(&entry.path(), &target);
            } else {
                fs::copy(entry.path(), &target).unwrap();
            }
        }
    }
}
