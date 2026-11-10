//! Form answer emails that fail are tried again (G40): the first failure
//! keeps the email in `form_mail`, and the scheduler's round sends each
//! one whose time has come, with the webhooks' backoff, so a mail server
//! down for most of a day still delivers. After the last try Khalid gives
//! up, says so in the log, and `khalid doctor` lists it for 30 days.

use std::sync::Arc;

use chrono::TimeDelta;
use khalid_core::{Clock, SystemClock};
use khalid_db::{Database, Failure, FormMailTable, SiteId, WaitingMail};
use khalid_forms::FormId;
use khalid_newsletter::{Mail, Mailer};
use khalid_webhooks::RetrySchedule;

use crate::server_log;

/// How long an email Khalid gave up on stays listed.
const GIVEN_UP_KEPT: TimeDelta = TimeDelta::days(30);

/// Sends form answer emails, keeping each that fails for another try.
#[derive(Debug)]
pub(crate) struct FormRetry {
    mailer: Arc<dyn Mailer>,
    table: FormMailTable,
    clock: Arc<dyn Clock>,
    schedule: RetrySchedule,
}

impl FormRetry {
    /// Sends through `mailer`, keeping failures in `table`, at the times
    /// `clock` gives, with the webhooks' backoff.
    pub(crate) fn new(
        mailer: Arc<dyn Mailer>,
        table: FormMailTable,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            mailer,
            table,
            clock,
            schedule: RetrySchedule::default(),
        }
    }

    /// The retry `khalid serve` runs for site `id` of `database`, sending
    /// through `mailer` by the system clock, or `None` when the site sends
    /// no mail.
    pub(crate) fn for_site(
        mailer: Option<Arc<dyn Mailer>>,
        (database, id): (&Database, SiteId),
    ) -> Option<Arc<Self>> {
        let table = FormMailTable::new(database.clone(), id);
        mailer.map(|mailer| Arc::new(Self::new(mailer, table, Arc::new(SystemClock))))
    }

    /// Sends `mail`, the answer to `form`, and keeps it for another try
    /// when that fails.
    pub(crate) async fn send(&self, form: &FormId, mail: &Mail) {
        let Err(error) = self.mailer.send(mail).await else {
            return;
        };
        let error = server_log::causes(&error);
        let at = self.clock.now();
        let next_at = self.schedule.next(1, at);
        tracing::warn!(%error, %form, "a form's answers could not be emailed, so Khalid tries again later");
        let failure = Failure {
            at,
            error: &error,
            next_at,
        };
        if let Err(error) = self.table.keep(form, mail, failure).await {
            tracing::warn!(%error, %form, "a form's unsent email could not be kept for another try");
        }
    }

    /// Tries again each email whose time has come, and forgets those given
    /// up on more than 30 days ago.
    pub(crate) async fn round(&self) {
        let now = self.clock.now();
        match self.table.due(now).await {
            Ok(due) => {
                for waiting in due {
                    self.again(waiting).await;
                }
            }
            Err(error) => {
                tracing::warn!(%error, "unsent form emails could not be read, so they are tried next round");
            }
        }
        if let Err(error) = self.table.forget_given_up_before(now - GIVEN_UP_KEPT).await {
            tracing::warn!(%error, "old unsent form emails could not be dropped");
        }
    }

    /// Tries `waiting` once more.
    async fn again(&self, waiting: WaitingMail) {
        let (form, tries) = (&waiting.form, waiting.failures + 1);
        let written = match self.mailer.send(&waiting.mail).await {
            Ok(()) => {
                tracing::info!(%form, tries, "a form's answers were emailed after an earlier try failed");
                self.table.sent(waiting.id).await
            }
            Err(error) => {
                let error = server_log::causes(&error);
                let at = self.clock.now();
                let next_at = self.schedule.next(tries, at);
                if next_at.is_none() {
                    tracing::warn!(%error, %form, tries, "a form's answers could not be emailed, and Khalid stopped trying");
                } else {
                    tracing::warn!(%error, %form, tries, "a form's answers could not be emailed again, so Khalid tries later");
                }
                let failure = Failure {
                    at,
                    error: &error,
                    next_at,
                };
                self.table.failed_again(waiting.id, failure).await
            }
        };
        if let Err(error) = written {
            tracing::warn!(%error, %form, "a form's unsent email could not be updated");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Mutex;

    use chrono::{DateTime, TimeZone, Utc};
    use khalid_core::Slug;

    use khalid_newsletter::{BoxFuture, EmailAddress, MailError};

    use super::*;

    /// A clock a test moves by hand.
    #[derive(Debug)]
    struct HandClock(Mutex<DateTime<Utc>>);

    impl HandClock {
        fn set(&self, at: DateTime<Utc>) {
            *self.0.lock().unwrap() = at;
        }
    }

    impl Clock for HandClock {
        fn now(&self) -> DateTime<Utc> {
            *self.0.lock().unwrap()
        }
    }

    /// A transport that refuses the first `refusals` emails, then takes
    /// every one.
    #[derive(Debug, Default)]
    struct Flaky {
        refusals: Mutex<usize>,
        tries: Mutex<usize>,
        sent: Mutex<Vec<Mail>>,
    }

    impl Mailer for Flaky {
        fn send<'a>(&'a self, mail: &'a Mail) -> BoxFuture<'a, Result<(), MailError>> {
            Box::pin(async move {
                *self.tries.lock().unwrap() += 1;
                let mut refusals = self.refusals.lock().unwrap();
                if *refusals > 0 {
                    *refusals -= 1;
                    return Err(MailError::new(std::io::Error::other("refused for a test")));
                }
                self.sent.lock().unwrap().push(mail.clone());
                Ok(())
            })
        }
    }

    fn at(minutes: i64) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 11, 10, 9, 0, 0).unwrap() + TimeDelta::minutes(minutes)
    }

    fn mail() -> Mail {
        let to = EmailAddress::parse("hello@example.com").unwrap();
        Mail::new(
            to,
            "New answer to the contact form".to_owned(),
            "name: Grace Hopper\n".to_owned(),
        )
    }

    /// A retry over a fresh database, a hand clock at [`at`]`(0)` and a
    /// transport that refuses `refusals` emails first.
    async fn retry(
        refusals: usize,
    ) -> (
        tempfile::TempDir,
        FormRetry,
        Arc<Flaky>,
        Arc<HandClock>,
        FormMailTable,
    ) {
        let dir = tempfile::tempdir().unwrap();
        let mut path = PathBuf::from(dir.path());
        path.push("site.db");
        let db = Database::open(&path).await.unwrap();
        db.migrate().await.unwrap();
        let site = db.site(&Slug::try_new("main").unwrap()).await.unwrap();
        let table = FormMailTable::new(db, site);
        let flaky = Arc::new(Flaky {
            refusals: Mutex::new(refusals),
            ..Flaky::default()
        });
        let clock = Arc::new(HandClock(Mutex::new(at(0))));
        let mailer: Arc<dyn Mailer> = Arc::<Flaky>::clone(&flaky);
        let made = FormRetry::new(mailer, table.clone(), Arc::<HandClock>::clone(&clock));
        (dir, made, flaky, clock, table)
    }

    #[tokio::test]
    async fn an_email_that_fails_twice_is_sent_on_the_third_try() {
        let (_dir, retry, flaky, clock, _) = retry(2).await;
        let form = FormId::try_new("contact").unwrap();
        retry.send(&form, &mail()).await;
        assert!(flaky.sent.lock().unwrap().is_empty(), "the first try fails");

        clock.set(at(0) + TimeDelta::seconds(59));
        retry.round().await;
        assert_eq!(*flaky.tries.lock().unwrap(), 1, "not tried before its time");

        clock.set(at(1));
        retry.round().await;
        assert_eq!(*flaky.tries.lock().unwrap(), 2, "the second try fails");

        clock.set(at(5));
        retry.round().await;
        assert_eq!(
            *flaky.tries.lock().unwrap(),
            2,
            "5 minutes after the second, not the first"
        );
        clock.set(at(6));
        retry.round().await;
        assert_eq!(
            *flaky.sent.lock().unwrap(),
            [mail()],
            "sent on the third try"
        );

        clock.set(at(60 * 24));
        retry.round().await;
        assert_eq!(*flaky.tries.lock().unwrap(), 3, "never sent twice");
    }

    #[tokio::test]
    async fn after_the_last_try_khalid_gives_up_and_forgets_it_30_days_later() {
        let (_dir, retry, flaky, clock, table) = retry(usize::MAX).await;
        let form = FormId::try_new("contact").unwrap();
        retry.send(&form, &mail()).await;
        let mut now = at(0);
        for failures in 1..=6 {
            now = retry.schedule.next(failures, now).unwrap();
            clock.set(now);
            retry.round().await;
        }
        assert_eq!(*flaky.tries.lock().unwrap(), 7, "seven tries in all");
        assert!(now - at(0) < TimeDelta::hours(24), "within a day");
        assert_eq!(table.given_up_since(at(0)).await.unwrap().len(), 1);

        clock.set(now + TimeDelta::days(31));
        retry.round().await;
        assert_eq!(*flaky.tries.lock().unwrap(), 7, "never tried again");
        assert_eq!(table.given_up_since(at(0)).await.unwrap(), []);
    }

    #[tokio::test]
    async fn an_email_sent_at_once_keeps_nothing() {
        let (_dir, retry, flaky, clock, table) = retry(0).await;
        retry
            .send(&FormId::try_new("contact").unwrap(), &mail())
            .await;
        assert_eq!(flaky.sent.lock().unwrap().len(), 1);
        clock.set(at(60));
        assert_eq!(table.due(at(60)).await.unwrap(), []);
    }
}
