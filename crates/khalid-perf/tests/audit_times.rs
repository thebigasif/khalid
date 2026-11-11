//! The audit measure times whole audits, counts one that finds errors,
//! refuses one that gives no report or skips posts, and reports the median
//! against the "Full site audit, 1,000 posts" budget. A shell script plays
//! `khalid`, so these run where `/bin/sh` does.
#![cfg(unix)]

use std::path::Path;

use khalid_core::SystemClock;
use khalid_perf::{AuditError, AuditTimes, crawled_pages};

/// A stand-in for `khalid` that prints `said` and exits with `status`,
/// whatever it is asked.
#[cfg(test)]
fn stand_in(dir: &Path, said: &str, status: u8) -> std::path::PathBuf {
    use std::os::unix::fs::PermissionsExt as _;
    let mut path = dir.to_path_buf();
    path.push("khalid");
    let script = format!("#!/bin/sh\nprintf '%s\\n' '{said}'\nexit {status}\n");
    std::fs::write(&path, script).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    path
}

#[test]
fn each_run_is_timed_and_the_median_is_reported() {
    let dir = tempfile::tempdir().unwrap();
    let khalid = stand_in(dir.path(), r#"{"score":97,"pages":12,"errors":0}"#, 0);
    let times = AuditTimes::measure(&khalid, (dir.path(), 10), 3, &SystemClock).unwrap();
    assert_eq!(times.samples(), 3);
    assert!(times.median_s() > 0.0);
}

#[test]
fn an_audit_that_finds_errors_still_counts() {
    let dir = tempfile::tempdir().unwrap();
    let khalid = stand_in(dir.path(), r#"{"score":60,"pages":12,"errors":4}"#, 1);
    let times = AuditTimes::measure(&khalid, (dir.path(), 10), 1, &SystemClock).unwrap();
    assert_eq!(times.samples(), 1);
}

#[test]
fn an_audit_that_stops_without_a_report_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let khalid = stand_in(dir.path(), r#"{"error":"the site did not build"}"#, 1);
    let error = AuditTimes::measure(&khalid, (dir.path(), 10), 1, &SystemClock).unwrap_err();
    assert!(
        matches!(error, AuditError::NoReport { status: 1, .. }),
        "{error}"
    );
}

#[test]
fn an_audit_that_khalid_could_not_do_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let khalid = stand_in(dir.path(), r#"{"score":0,"pages":12}"#, 3);
    let error = AuditTimes::measure(&khalid, (dir.path(), 10), 1, &SystemClock).unwrap_err();
    assert!(
        matches!(error, AuditError::NoReport { status: 3, .. }),
        "{error}"
    );
}

#[test]
fn an_audit_that_crawled_fewer_pages_than_posts_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let khalid = stand_in(dir.path(), r#"{"score":100,"pages":3}"#, 0);
    let error = AuditTimes::measure(&khalid, (dir.path(), 10), 1, &SystemClock).unwrap_err();
    assert!(
        matches!(
            error,
            AuditError::TooFewPages {
                pages: 3,
                posts: 10
            }
        ),
        "{error}"
    );
}

#[test]
fn a_binary_that_cannot_start_is_named() {
    let dir = tempfile::tempdir().unwrap();
    let mut missing = dir.path().to_path_buf();
    missing.push("no-such-khalid");
    let error = AuditTimes::measure(&missing, (dir.path(), 10), 1, &SystemClock).unwrap_err();
    assert!(matches!(error, AuditError::Io { .. }), "{error}");
}

#[test]
fn the_pages_come_from_the_report_not_from_an_issue() {
    let report = r#"{"score":88,"pages":1043,"issues":[{"detail":"\"pages\":2"}]}"#;
    assert_eq!(crawled_pages(report), Some(1043));
    assert_eq!(crawled_pages(r#"{"error":"no"}"#), None);
    assert_eq!(crawled_pages(""), None);
}

#[test]
fn the_json_names_the_baseline_measure() {
    let json = AuditTimes::from_s(vec![4.5, 2.25, 3.0]).to_json();
    assert_eq!(json, "{\"site_audit_1k_s\": 3.000}\n");
}
