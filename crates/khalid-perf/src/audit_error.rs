//! What can go wrong while timing `khalid audit`.

use std::io;
use std::path::PathBuf;

/// `khalid audit` could not be started, or did not audit the site.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AuditError {
    /// The binary could not be started.
    #[error("could not start {}", path.display())]
    Io {
        /// The binary.
        path: PathBuf,
        /// The cause.
        #[source]
        source: io::Error,
    },
    /// The audit stopped without a report: it could not build or crawl the
    /// site, so its time says nothing about the budget.
    #[error("the audit stopped without a report (exit status {status}): {said}")]
    NoReport {
        /// The exit status, or -1 when a signal ended the process.
        status: i32,
        /// What the audit wrote to standard output.
        said: String,
    },
    /// The audit crawled fewer pages than the site has posts, so it did
    /// not do the whole work the budget names.
    #[error("the audit crawled {pages} pages, fewer than the {posts} posts of the site")]
    TooFewPages {
        /// The pages the audit crawled.
        pages: usize,
        /// The posts the site was written with.
        posts: usize,
    },
}
