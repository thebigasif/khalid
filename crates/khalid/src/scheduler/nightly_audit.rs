//! The nightly site audit in `khalid serve` (plan 5.10): once an hour the
//! scheduler asks whether a day has passed since the last audit, by
//! `khalid audit` or by the server, and runs one when it has. It audits the
//! site built into `data/audit/site/`, offline, and keeps the score in
//! `data/audit/runs.jsonl` for the dashboard. A site with no pages has
//! nothing to crawl and gets no audit.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::{DateTime, TimeDelta, Utc};
use khalid_audit::Run;
use khalid_content::{ContentTree, read_content};
use khalid_core::{Clock, Config, UserMessage};
use khalid_routes::Routes;
use tokio::runtime::Handle;

use crate::serve_https::ServeClock;
use crate::{content_message, site_audit};

/// How long after one audit the next falls due.
const DAY: TimeDelta = TimeDelta::hours(24);

/// What the nightly audit needs, read once at start.
#[derive(Debug)]
pub(crate) struct NightlyAudit {
    folder: PathBuf,
    config: Config,
    routes: Routes,
}

impl NightlyAudit {
    /// The nightly audit of the site in `folder` with `config` and
    /// `routes`, or `None` for a site with no pages.
    pub(crate) fn new(folder: &Path, config: &Config, routes: &Routes) -> Option<Arc<Self>> {
        config.mode().has_pages().then(|| {
            Arc::new(Self {
                folder: folder.to_path_buf(),
                config: config.clone(),
                routes: routes.clone(),
            })
        })
    }

    /// Runs the audit when a day has passed since the last one by `clock`,
    /// and logs what happened. It builds and reads the whole site, so it
    /// runs on a blocking thread (RULES.md BL-RUN-7).
    pub(crate) async fn round(self: &Arc<Self>, clock: ServeClock) {
        let audit = Arc::clone(self);
        let handle = Handle::current();
        let done = tokio::task::spawn_blocking(move || audit.run_if_due(&clock, &handle)).await;
        if let Err(error) = done {
            tracing::warn!(%error, "the nightly site audit stopped early");
        }
    }

    /// Runs the audit when it is due.
    fn run_if_due(&self, clock: &dyn Clock, handle: &Handle) {
        if !due(&site_audit::runs(&self.folder), clock.now()) {
            return;
        }
        match self.run(clock, handle) {
            Ok(run) => tracing::info!(
                score = run.score,
                errors = run.errors,
                "the nightly site audit ran"
            ),
            Err(message) => tracing::warn!(
                %message,
                "the nightly site audit did not run, so it is tried again in an hour"
            ),
        }
    }

    /// Reads the site's content as it is now and audits it.
    fn run(&self, clock: &dyn Clock, handle: &Handle) -> Result<Run, UserMessage> {
        let content = ContentTree::walk(&self.folder)
            .and_then(|tree| read_content(&tree))
            .map_err(|error| content_message::of(&self.folder, &error))?;
        let checked = (&self.config, &self.routes, &content);
        site_audit::nightly(&self.folder, checked, clock, handle)
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

    use chrono::{DateTime, TimeDelta, Utc};
    use khalid_audit::Run;
    use khalid_core::{Config, Environment, FixedClock};
    use khalid_routes::Routes;

    use super::{NightlyAudit, due};
    use crate::site_audit;

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
        let audit = NightlyAudit::new(site.path(), &config, &routes).unwrap();
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
        let audit = NightlyAudit::new(site.path(), &config, &routes).unwrap();
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
