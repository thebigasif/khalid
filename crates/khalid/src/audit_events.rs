//! The `audit.finished` webhook `khalid audit` fires once a run is kept
//! (audit brief G25). The delivery carries the score and the counts; the
//! issues themselves stay in `data/audit/`, where `khalid audit --json`
//! and the CSV files list them.

use std::path::Path;

use khalid_audit::{Report, Severity};
use khalid_core::Config;
use khalid_webhooks::Event;
use serde::Serialize;
use tokio::runtime::Runtime;

use crate::announce::SiteAnnounce;
use crate::cli::Cli;
use crate::{announce, db, outbound, scheduler, serve};

/// The resource an `audit.finished` delivery carries.
#[derive(Debug, Serialize)]
pub(crate) struct Finished {
    audit: Summary,
}

/// One audit run in numbers.
#[derive(Debug, Serialize)]
struct Summary {
    /// The health score out of 100.
    score: u8,
    pages: usize,
    errors: usize,
    warnings: usize,
    notices: usize,
}

/// What an `audit.finished` delivery about `audit` carries. The nightly
/// audit in `khalid serve` fires the same through the server's announcer.
pub(crate) fn finished(audit: &Report) -> Finished {
    Finished {
        audit: Summary {
            score: audit.score(),
            pages: audit.pages(),
            errors: audit.count(Severity::Error),
            warnings: audit.count(Severity::Warning),
            notices: audit.count(Severity::Notice),
        },
    }
}

/// The nightly audit's `audit.finished`, fired through `telling`, the
/// running server's announcer.
pub(crate) fn to(telling: &SiteAnnounce) -> scheduler::Finished {
    let telling = telling.clone();
    Box::new(move |audit: &Report| telling.fire(Event::AuditFinished, finished(audit)))
}

/// Fires `audit.finished` about `audit` to the webhooks of the site in
/// `folder`, with `config`, and waits for the deliveries. A site with no
/// database has no webhooks, and none is made for it. A database that
/// cannot be opened is logged and leaves the audit's result alone, since
/// a failed delivery is kept for `khalid serve` to try again.
pub(crate) fn fire(
    (folder, config): (&Path, &Config),
    audit: &Report,
    cli: &Cli,
    runtime: &Runtime,
) {
    if !db::data_file(folder).is_ok_and(|file| file.exists()) {
        return;
    }
    let Ok(outbound) = outbound::client(cli) else {
        tracing::warn!("audit.finished was not sent: the web client could not start");
        return;
    };
    let (database, site) = match serve::open(folder, runtime) {
        Ok(opened) => opened,
        Err(message) => {
            tracing::warn!(%message, "audit.finished was not sent");
            return;
        }
    };
    runtime.block_on(async {
        match announce::once(config, (&database, site), &outbound) {
            Ok((telling, announcing)) => {
                telling.fire(Event::AuditFinished, finished(audit));
                announcing.stop().await;
            }
            Err(message) => tracing::warn!(%message, "audit.finished was not sent"),
        }
        database.close().await;
    });
}
