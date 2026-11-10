//! The scheduler in `khalid serve`: every few seconds it asks the publish
//! door to act on each post whose publish or unpublish time has come. It
//! never publishes on its own, so every post it takes live or down is
//! announced like any other (CTO answers 2026-10-27, section 4). In the
//! same round it moves each page, glossary term, case study and changelog
//! entry whose `publish_at` has come from `drafts/` to `content/`, which
//! the next render announces, and tries again each form answer email
//! whose next try has come (G40). Once an hour it sends the weekly Search
//! Console email when a week has passed since the last one.

mod entries;
mod weekly_email;

pub(crate) use weekly_email::WeeklyEmail;

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use khalid_api::publish_door::{PublishDoor, Ran};
use khalid_core::{Clock, SystemClock};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::MissedTickBehavior;

use crate::alerts::Alerting;
use crate::form_delivery::FormRetry;

/// How often the schedules are looked at, so a post goes live at most this
/// long after its time.
const EVERY: Duration = Duration::from_secs(5);

/// Rounds between two looks at the weekly email: an hour.
const HOURLY: u32 = 720;

/// The running scheduler, stopped before the database closes.
#[derive(Debug)]
pub(crate) struct Scheduling {
    stop: oneshot::Sender<()>,
    task: JoinHandle<()>,
}

impl Scheduling {
    /// Stops the scheduler once the round in progress has finished
    /// (RULES.md BL-RUN-10).
    pub(crate) async fn stop(self) {
        let _ = self.stop.send(());
        let _ = self.task.await;
    }
}

/// Starts the scheduler on `door` and the drafts of the site `folder`,
/// raising each post the linter refuses through `alerting`, trying again
/// each form email `forms` holds whose time has come, and sending the
/// `weekly` email when the site gets one. Must be called inside the tokio
/// runtime. The first round runs at once, so a time that passed while the
/// server was stopped is acted on at start.
pub(crate) fn start(
    (door, folder, forms): (&Arc<PublishDoor>, &Path, Option<Arc<FormRetry>>),
    (alerting, weekly): (Arc<Alerting>, Option<Arc<WeeklyEmail>>),
) -> Scheduling {
    let (stop, mut stopped) = oneshot::channel();
    let (door, folder) = (Arc::clone(door), folder.to_path_buf());
    let task = tokio::spawn(async move {
        let mut ticks = tokio::time::interval(EVERY);
        ticks.set_missed_tick_behavior(MissedTickBehavior::Delay);
        let mut rounds = 0_u32;
        loop {
            tokio::select! {
                _ = &mut stopped => return,
                _ = ticks.tick() => {}
            }
            match door.run_due().await {
                Ok(ran) => {
                    for one in &ran {
                        report(one);
                        if let Ran::Refused(id) = one {
                            alerting.refused(id).await;
                        }
                    }
                }
                Err(error) => tracing::warn!(
                    ?error,
                    "the schedules could not be read, so they are looked at again next round"
                ),
            }
            let site = folder.clone();
            let moved =
                tokio::task::spawn_blocking(move || entries::publish_due(&site, SystemClock.now()));
            for file in moved.await.unwrap_or_default() {
                tracing::info!(file, "a scheduled entry went live");
            }
            if let Some(forms) = &forms {
                forms.round().await;
            }
            // After the posts, so asking Google never holds a post back.
            if let Some(weekly) = weekly.as_ref().filter(|_| rounds.is_multiple_of(HOURLY)) {
                weekly.round().await;
            }
            rounds = rounds.wrapping_add(1);
        }
    });
    Scheduling { stop, task }
}

/// Logs what the scheduler did about one post.
fn report(ran: &Ran) {
    if let Ran::Refused(id) = ran {
        tracing::warn!(
            id,
            "the linter refused a scheduled post, so it stays a draft"
        );
    } else {
        tracing::info!(?ran, "the scheduler acted on a post whose time had come");
    }
}
