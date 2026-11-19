//! The lock that keeps two audits of one site apart. `khalid audit` and
//! the nightly audit in `khalid serve` both build into `data/audit/site/`
//! and write the same files, so an audit that starts while another runs
//! stops at once and says so. The lock is the file `data/audit/running`,
//! which holds the time the audit started. A lock six hours old was left
//! by an audit that stopped without removing it, so it is taken over.

use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write as _};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};

use chrono::{DateTime, TimeDelta, Utc};
use khalid_core::{Clock, UserMessage};

use crate::audit_message;

/// The lock file's name in `data/audit/`.
const NAME: &str = "running";

/// How old a lock is before it counts as left behind. A build and crawl
/// of a large site, with every outside link asked, takes minutes, not
/// hours.
const STALE: TimeDelta = TimeDelta::hours(6);

/// A held audit lock. Dropping it removes the file, so the next audit can
/// start.
#[derive(Debug)]
pub(crate) struct AuditLock {
    file: PathBuf,
}

impl AuditLock {
    /// Takes the lock in the audit's `folder` at the time `clock` gives.
    ///
    /// # Errors
    ///
    /// Returns the busy message when another audit holds a lock younger
    /// than six hours, or the write message when the folder cannot be
    /// written to.
    pub(crate) fn take(folder: &Path, clock: &dyn Clock) -> Result<Self, UserMessage> {
        fs::create_dir_all(folder).map_err(|error| audit_message::write(folder, &error))?;
        let mut file = folder.to_path_buf();
        file.push(NAME);
        let now = clock.now();
        match place(&file, now) {
            Err(error) if error.kind() == ErrorKind::AlreadyExists && stale(&file, now) => {
                // Gone already when another audit took it over first.
                let _ = fs::remove_file(&file);
                place(&file, now)
            }
            placed => placed,
        }
        .map_err(|error| match error.kind() {
            ErrorKind::AlreadyExists => audit_message::busy(&file),
            _ => audit_message::write(&file, &error),
        })?;
        Ok(Self { file })
    }
}

impl Drop for AuditLock {
    fn drop(&mut self) {
        // Nothing to do when it cannot go: the next audit takes it over
        // once it is six hours old.
        let _ = fs::remove_file(&self.file);
    }
}

/// The lock of the nightly audit in `khalid serve`, which the server can
/// let go of while the audit still runs. Stopping the server does not wait
/// for a build that can take minutes, and the thread left running ends
/// with the process, so without this the lock would stay for six hours.
#[derive(Debug, Default)]
pub(crate) struct AuditHold(Mutex<Hold>);

#[derive(Debug, Default)]
struct Hold {
    lock: Option<AuditLock>,
    stopped: bool,
}

/// The nightly audit's turn with an [`AuditHold`]. Dropping it lets go of
/// the lock, unless the server already did.
#[derive(Debug)]
pub(crate) struct Holding<'h>(&'h AuditHold);

impl AuditHold {
    /// Takes the lock in the audit's `folder` at the time `clock` gives,
    /// for as long as the returned [`Holding`] lives.
    ///
    /// # Errors
    ///
    /// Returns the stopped message once the server is stopping, and the
    /// messages of [`AuditLock::take`].
    pub(crate) fn take(
        &self,
        folder: &Path,
        clock: &dyn Clock,
    ) -> Result<Holding<'_>, UserMessage> {
        let mut hold = self.hold();
        if hold.stopped {
            return Err(audit_message::stopped());
        }
        hold.lock = Some(AuditLock::take(folder, clock)?);
        Ok(Holding(self))
    }

    /// Lets go of the lock and refuses every later turn, as the server
    /// stops.
    pub(crate) fn stop(&self) {
        let mut hold = self.hold();
        hold.stopped = true;
        hold.lock = None;
    }

    /// Whether the server is stopping, so the audit should end early.
    pub(crate) fn stopped(&self) -> bool {
        self.hold().stopped
    }

    fn hold(&self) -> MutexGuard<'_, Hold> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl Drop for Holding<'_> {
    fn drop(&mut self) {
        self.0.hold().lock = None;
    }
}

/// Creates the lock `file` holding `now`, failing with `AlreadyExists`
/// when another audit holds it.
fn place(file: &Path, now: DateTime<Utc>) -> std::io::Result<()> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file)?
        .write_all(now.to_rfc3339().as_bytes())
}

/// Whether the lock in `file` was left behind: it is six hours old at
/// `now`, or holds no time to read.
fn stale(file: &Path, now: DateTime<Utc>) -> bool {
    fs::read_to_string(file)
        .ok()
        .and_then(|text| DateTime::parse_from_rfc3339(text.trim()).ok())
        .is_none_or(|at| now - at.with_timezone(&Utc) >= STALE)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use chrono::{DateTime, TimeDelta, Utc};
    use khalid_core::FixedClock;

    use super::{AuditHold, AuditLock};

    /// A fixed moment, since tests never read the computer's clock.
    fn noon() -> DateTime<Utc> {
        DateTime::from_timestamp(1_794_484_800, 0).unwrap_or_default()
    }

    fn at(hours: i64) -> FixedClock {
        FixedClock::new(noon() + TimeDelta::hours(hours))
    }

    fn lock_file(folder: &Path) -> PathBuf {
        let mut file = folder.to_path_buf();
        file.push("running");
        file
    }

    #[test]
    fn a_second_audit_waits_until_the_first_lets_go() {
        let folder = tempfile::tempdir().unwrap();
        let first = AuditLock::take(folder.path(), &at(0)).unwrap();
        let refused = AuditLock::take(folder.path(), &at(1)).unwrap_err();
        assert!(
            refused
                .what()
                .contains("Another audit of this site is running"),
            "{refused:?}"
        );
        drop(first);
        assert!(!lock_file(folder.path()).exists(), "letting go removes it");
        assert!(AuditLock::take(folder.path(), &at(1)).is_ok());
    }

    #[test]
    fn a_lock_left_for_six_hours_is_taken_over() {
        let folder = tempfile::tempdir().unwrap();
        fs::write(lock_file(folder.path()), noon().to_rfc3339()).unwrap();
        assert!(AuditLock::take(folder.path(), &at(5)).is_err());
        let taken = AuditLock::take(folder.path(), &at(6)).unwrap();
        let held = fs::read_to_string(lock_file(folder.path())).unwrap();
        assert_eq!(held, (noon() + TimeDelta::hours(6)).to_rfc3339());
        drop(taken);
    }

    #[test]
    fn stopping_the_server_lets_go_of_the_nightly_lock_while_its_audit_runs() {
        let folder = tempfile::tempdir().unwrap();
        let hold = AuditHold::default();
        let holding = hold.take(folder.path(), &at(0)).unwrap();
        assert!(lock_file(folder.path()).exists());
        hold.stop();
        assert!(hold.stopped());
        assert!(!lock_file(folder.path()).exists(), "the lock went at stop");
        assert!(
            AuditLock::take(folder.path(), &at(0)).is_ok(),
            "khalid audit can start"
        );
        drop(holding);
        assert!(
            hold.take(folder.path(), &at(1)).is_err(),
            "no audit after stop"
        );
    }

    #[test]
    fn a_hold_lets_go_when_its_audit_ends() {
        let folder = tempfile::tempdir().unwrap();
        let hold = AuditHold::default();
        drop(hold.take(folder.path(), &at(0)).unwrap());
        assert!(!lock_file(folder.path()).exists());
        assert!(!hold.stopped());
    }

    #[test]
    fn a_lock_with_no_time_in_it_is_taken_over() {
        let folder = tempfile::tempdir().unwrap();
        fs::write(lock_file(folder.path()), "").unwrap();
        assert!(AuditLock::take(folder.path(), &at(0)).is_ok());
    }
}
