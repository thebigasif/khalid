//! The lock that keeps two audits of one site apart. `khalid audit` and
//! the nightly audit in `khalid serve` both build into `data/audit/site/`
//! and write the same files, so an audit that starts while another runs
//! stops at once and says so. The lock is the file `data/audit/running`,
//! which holds the time the audit started. A lock six hours old was left
//! by an audit that stopped without removing it, so it is taken over.

use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write as _};
use std::path::{Path, PathBuf};

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

    use super::AuditLock;

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
    fn a_lock_with_no_time_in_it_is_taken_over() {
        let folder = tempfile::tempdir().unwrap();
        fs::write(lock_file(folder.path()), "").unwrap();
        assert!(AuditLock::take(folder.path(), &at(0)).is_ok());
    }
}
