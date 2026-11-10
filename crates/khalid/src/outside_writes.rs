//! Writes another process makes to the stores `khalid serve` keeps in
//! memory: stored redirects, the media library and readers' comments.
//! `khalid mcp` notes each one in `data/outside-writes`, and each round of
//! the scheduler reads the stores again once that note changes (ADR
//! "khalid serve reads again what khalid mcp writes").

use std::path::{Path, PathBuf};
use std::sync::Arc;

use khalid_core::SitePath;
use khalid_media::Library;

use crate::comment_pages::CommentsReload;
use crate::live_redirects::LiveRedirects;

/// The note, inside the site folder. `data/` holds only what Khalid makes.
const NOTE: &str = "data/outside-writes";

/// Notes that the stores of the site in `site` changed, so a running
/// `khalid serve` reads them again within one scheduler round. The note
/// is a count, so two writes in one clock tick still change it.
///
/// # Errors
///
/// Returns the error when the note cannot be read or written.
pub(crate) fn note(site: &Path) -> std::io::Result<()> {
    let path = SitePath::within(site, NOTE)
        .and_then(|path| path.resolve())
        .map_err(std::io::Error::other)?;
    let count = read(&path).parse::<u64>().unwrap_or(0).wrapping_add(1);
    std::fs::write(&path, count.to_string())
}

/// The note's text, or nothing when there is no note yet.
fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

/// The stores `khalid serve` reads again when the note changes.
pub(crate) struct Reloads {
    redirects: LiveRedirects,
    media: Arc<Library>,
    comments: Option<CommentsReload>,
    note: Option<PathBuf>,
    seen: String,
}

impl Reloads {
    /// The `redirects`, `media` and `comments` of the site in `site`, as
    /// they were read at start, with the note as it is now.
    pub(crate) fn new(
        site: &Path,
        (redirects, media, comments): (LiveRedirects, Arc<Library>, Option<CommentsReload>),
    ) -> Self {
        let note = SitePath::within(site, NOTE)
            .and_then(|path| path.resolve())
            .ok();
        let seen = note.as_deref().map(read).unwrap_or_default();
        Self {
            redirects,
            media,
            comments,
            note,
            seen,
        }
    }

    /// Reads every store again when the note changed since the last look.
    pub(crate) async fn round(&mut self) {
        let Some(path) = self.note.clone() else {
            return;
        };
        let now = tokio::task::spawn_blocking(move || read(&path))
            .await
            .unwrap_or_default();
        if now == self.seen {
            return;
        }
        self.seen = now;
        self.redirects.reload().await;
        if let Err(error) = self.media.reload().await {
            tracing::warn!(%error, "the media library keeps what it had");
        }
        if let Some(comments) = &self.comments {
            comments.reload().await;
        }
    }
}

impl std::fmt::Debug for Reloads {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reloads")
            .field("note", &self.note)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_note_changes_what_a_running_serve_reads() {
        let site = tempfile::tempdir().unwrap();
        let mut data = site.path().to_path_buf();
        data.push("data");
        std::fs::create_dir(&data).unwrap();
        let path = SitePath::within(site.path(), NOTE)
            .unwrap()
            .resolve()
            .unwrap();
        assert_eq!(read(&path), "");
        note(site.path()).unwrap();
        let first = read(&path);
        note(site.path()).unwrap();
        assert_ne!(read(&path), first);
    }
}
