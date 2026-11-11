//! The hourly prune in `khalid serve`: at start and then once an hour it
//! deletes the site's visitor rows that have aged out, such as 404s nobody
//! asked for in 90 days, finished webhook deliveries older than 30 days,
//! and sessions and sign-in links that can no longer be used, so the
//! database of a long-running site does not grow without end.

use std::time::Duration;

use khalid_core::Clock;
use khalid_db::{Database, Prune, SiteId};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::MissedTickBehavior;

/// How often the prune runs. Rows age in days, so once an hour is enough.
const EVERY: Duration = Duration::from_hours(1);

/// The running prune, stopped before the database closes.
#[derive(Debug)]
pub(crate) struct Pruning {
    stop: oneshot::Sender<()>,
    task: JoinHandle<()>,
}

impl Pruning {
    /// Starts the prune of the site `id` in `database` by the time `clock`
    /// gives. Must be called inside the tokio runtime. The first prune runs
    /// at once.
    pub(crate) fn start((database, id): (&Database, SiteId), clock: impl Clock + 'static) -> Self {
        let prune = Prune::new(database.clone(), id);
        let (stop, mut stopped) = oneshot::channel();
        let task = tokio::spawn(async move {
            let mut ticks = tokio::time::interval(EVERY);
            ticks.set_missed_tick_behavior(MissedTickBehavior::Delay);
            loop {
                tokio::select! {
                    _ = &mut stopped => return,
                    _ = ticks.tick() => {}
                }
                once(&prune, &clock).await;
            }
        });
        Self { stop, task }
    }

    /// Stops the prune once the one in progress has finished (RULES.md
    /// BL-RUN-10).
    pub(crate) async fn stop(self) {
        let _ = self.stop.send(());
        let _ = self.task.await;
    }
}

/// Runs one prune, logging what it deleted.
async fn once(prune: &Prune, clock: &impl Clock) {
    match prune.run(clock.now()).await {
        Ok(pruned) if pruned.total() > 0 => tracing::info!(
            misses = pruned.misses,
            deliveries = pruned.deliveries,
            sessions = pruned.sessions,
            links = pruned.links,
            "old visitor rows were deleted from the database"
        ),
        Ok(_) => {}
        Err(error) => tracing::warn!(
            ?error,
            "old visitor rows could not be deleted, so it is tried again in an hour"
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::Duration;

    use chrono::{TimeDelta, TimeZone, Utc};
    use khalid_api::not_found_log::NotFoundLog as _;
    use khalid_core::{FixedClock, Slug};
    use khalid_db::NotFoundTable;

    use super::*;

    #[tokio::test]
    async fn the_first_prune_runs_at_start_and_stopping_waits_for_it() {
        let dir = tempfile::tempdir().unwrap();
        let mut path = PathBuf::from(dir.path());
        path.push("site.db");
        let db = Database::open(&path).await.unwrap();
        db.migrate().await.unwrap();
        let id = db.site(&Slug::try_new("main").unwrap()).await.unwrap();
        let now = Utc.with_ymd_and_hms(2026, 11, 12, 9, 0, 0).unwrap();
        let log = NotFoundTable::new(db.clone(), id);
        log.record("/blog/wp-login.php", now - TimeDelta::days(100))
            .await
            .unwrap();
        log.record("/blog/old-post/", now).await.unwrap();

        let pruning = Pruning::start((&db, id), FixedClock::new(now));
        for _ in 0..100 {
            if log.misses(10).await.unwrap().len() == 1 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        pruning.stop().await;
        let kept = log.misses(10).await.unwrap();
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].path, "/blog/old-post/");
        db.close().await;
    }
}
