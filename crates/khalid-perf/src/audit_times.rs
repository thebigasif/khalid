//! How long `khalid audit` takes over a whole generated site, timed from
//! outside the process as `scripts/perf.sh` times a full rebuild.

use std::path::Path;
use std::process::{Command, Stdio};

use khalid_core::Clock;

use crate::{AuditError, percentile};

/// The time of each full audit of a site, sorted, in seconds.
#[derive(Debug, Clone, PartialEq)]
pub struct AuditTimes {
    sorted_s: Vec<f64>,
}

impl AuditTimes {
    /// Times already taken, in seconds, in any order.
    pub fn from_s(mut times: Vec<f64>) -> Self {
        times.sort_by(f64::total_cmp);
        Self { sorted_s: times }
    }

    /// Runs `binary --site SITE --json audit --offline` `runs` times on
    /// the site of `posts` posts, timing each with `clock`. The audit
    /// builds the site, crawls every page and scores it each time, as an
    /// owner's `khalid audit` does; `--offline` keeps links to other sites
    /// unchecked, so the internet's speed is not measured. An audit that
    /// finds an error exits with 1 and still counts, because it did the
    /// whole work; one that stops without a report does not.
    ///
    /// # Errors
    ///
    /// [`AuditError::Io`] when the binary cannot start,
    /// [`AuditError::NoReport`] when the audit gives no report, and
    /// [`AuditError::TooFewPages`] when it crawled fewer pages than the
    /// site has posts.
    pub fn measure(
        binary: &Path,
        (site, posts): (&Path, usize),
        runs: usize,
        clock: &dyn Clock,
    ) -> Result<Self, AuditError> {
        let mut times = Vec::with_capacity(runs);
        for _ in 0..runs {
            let start = clock.now();
            let done = Command::new(binary)
                .arg("--site")
                .arg(site)
                .args(["--json", "audit", "--offline"])
                .stdin(Stdio::null())
                .stderr(Stdio::null())
                .output()
                .map_err(|source| AuditError::Io {
                    path: binary.to_path_buf(),
                    source,
                })?;
            let nanos = (clock.now() - start).num_nanoseconds().unwrap_or(i64::MAX);
            let said = String::from_utf8_lossy(&done.stdout);
            let status = done.status.code().unwrap_or(-1);
            let (0 | 1, Some(pages)) = (status, crawled_pages(&said)) else {
                return Err(AuditError::NoReport {
                    status,
                    said: said.trim().to_owned(),
                });
            };
            if pages < posts {
                return Err(AuditError::TooFewPages { pages, posts });
            }
            times.push(f64::from(u32::try_from(nanos / 1_000).unwrap_or(u32::MAX)) / 1e6);
        }
        Ok(Self::from_s(times))
    }

    /// How many audits were timed.
    pub fn samples(&self) -> usize {
        self.sorted_s.len()
    }

    /// The median time of one full audit.
    pub fn median_s(&self) -> f64 {
        percentile(&self.sorted_s, 50)
    }

    /// The measure as the JSON object `scripts/perf.sh` merges into
    /// `perf/current.json`, with the name `perf/baseline.json` uses.
    pub fn to_json(&self) -> String {
        format!("{{\"site_audit_1k_s\": {:.3}}}\n", self.median_s())
    }
}

/// The pages a `khalid --json audit` report says it crawled, from its
/// first `"pages"` field, which comes before any issue. `None` when the
/// output is not a report, such as an error body.
pub fn crawled_pages(report: &str) -> Option<usize> {
    let (_, after) = report.split_once("\"pages\":")?;
    let digits: String = after
        .trim_start()
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
    digits.parse().ok()
}
