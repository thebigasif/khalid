//! The page set readers see now, replaced as a whole when the site changes.

use std::sync::Arc;

use tokio::sync::watch;

use crate::{Page, Pages, Render, ServerError, render_pages};

/// The pages being served, shared by every request and replaced as a whole
/// after a change, so a reader never sees half a rebuild. Cloning shares
/// the same set.
#[derive(Debug, Clone)]
pub struct LivePages(Arc<watch::Sender<Arc<Pages>>>);

impl LivePages {
    /// Serves `pages` until they are replaced.
    pub fn new(pages: Pages) -> Self {
        Self(Arc::new(watch::Sender::new(Arc::new(pages))))
    }

    /// The pages being served now.
    pub fn current(&self) -> Arc<Pages> {
        Arc::clone(&self.0.borrow())
    }

    /// Each time the pages are replaced from now on, for work that follows
    /// a render, such as handing freed memory back.
    pub fn replacements(&self) -> Replacements {
        Replacements(self.0.subscribe())
    }

    /// The pages being served once they are every page of the site: at
    /// once, unless the server started with [`Pages::first`] and the rest
    /// is still being rendered.
    pub async fn whole(&self) -> Arc<Pages> {
        self.once(Pages::is_whole).await
    }

    /// The pages being served once `ready` says yes to them: at once when
    /// it does now. A request held for the rest of the site waits so for
    /// its own page, which a post saved during the start brings sooner.
    pub async fn once(&self, ready: impl Fn(&Pages) -> bool) -> Arc<Pages> {
        let mut served = self.0.subscribe();
        match served.wait_for(|pages| ready(pages)).await {
            Ok(pages) => Arc::clone(&pages),
            // The sender lives as long as `self`, so it cannot be closed.
            Err(_) => self.current(),
        }
    }

    /// Renders the site again with `renderer` and serves the new pages.
    /// Pages whose bytes did not change keep their Brotli copy.
    ///
    /// # Errors
    ///
    /// Returns the errors of [`render_pages`]. The pages being served do
    /// not change, so a broken edit never takes the site down.
    pub async fn refresh<R: Render + Clone>(&self, renderer: &R) -> Result<(), ServerError> {
        let pages = render_pages(renderer, Some(self.current())).await?;
        self.0.send_replace(Arc::new(pages));
        Ok(())
    }

    /// Serves the `saved` pages in place of the pages at their addresses,
    /// and every other page as it is (see [`Pages::with_saved`]).
    pub fn show_saved(&self, saved: Vec<Page>) {
        self.0
            .send_modify(|pages| *pages = Arc::new(pages.with_saved(saved)));
    }

    /// Renders the site again with `renderer`, like [`LivePages::refresh`],
    /// but serves the new pages only when no newer save showed while it
    /// rendered through [`LivePages::show_saved`]. Says whether the new
    /// pages are served.
    ///
    /// Pages replaced meanwhile by a render of older content, such as the
    /// start render or its Brotli copies, do not hold the new pages back:
    /// on a large site the start render can take as long as this one, and
    /// waiting for both would hold an edit back for two full renders.
    ///
    /// # Errors
    ///
    /// Returns the errors of [`render_pages`]; the pages being served do
    /// not change.
    pub async fn refresh_unless_saved<R: Render + Clone>(
        &self,
        renderer: &R,
    ) -> Result<bool, ServerError> {
        let before = self.current();
        let saves = before.saves();
        let pages = render_pages(renderer, Some(before)).await?;
        Ok(self.0.send_if_modified(|served| {
            if served.saves() != saves {
                return false;
            }
            *served = Arc::new(pages);
            true
        }))
    }

    /// Serves the whole site's `pages`, rendered from the content read at
    /// start, in place of the first pages, with every page saved since kept
    /// on top. Says whether they are served: not when a whole set is
    /// served already, since a full render after a save read newer content.
    pub fn serve_whole(&self, pages: Pages) -> bool {
        self.0.send_if_modified(|served| {
            if served.is_whole() {
                return false;
            }
            *served = Arc::new(pages.under_saved(served));
            true
        })
    }

    /// Adds a Brotli copy to every page being served, on a blocking thread,
    /// then serves the compressed set. Start the server with
    /// [`Pages::plain`] and call this once it listens, so the first reader
    /// does not wait for compression.
    ///
    /// When the pages were replaced meanwhile, by an edit, the compressed
    /// set is dropped: the newer pages were compressed when they were built.
    ///
    /// # Errors
    ///
    /// Returns [`ServerError::Worker`] when the compressing thread stopped.
    /// The plain pages keep being served.
    pub async fn compress(&self) -> Result<(), ServerError> {
        let plain = self.current();
        let work = Arc::clone(&plain);
        let compressed = tokio::task::spawn_blocking(move || work.compressed())
            .await
            .map_err(|source| ServerError::Worker { source })?;
        self.replace_if_current(&plain, compressed);
        Ok(())
    }

    /// Serves `pages` in place of `expected`, unless something else
    /// replaced `expected` first. Says whether `pages` are served now.
    pub fn replace_if_current(&self, expected: &Arc<Pages>, pages: Pages) -> bool {
        self.0.send_if_modified(|served| {
            if !Arc::ptr_eq(served, expected) {
                return false;
            }
            *served = Arc::new(pages);
            true
        })
    }
}

/// The replacements of a [`LivePages`] set, one at a time, from
/// [`LivePages::replacements`]. It holds no pages itself.
#[derive(Debug)]
pub struct Replacements(watch::Receiver<Arc<Pages>>);

impl Replacements {
    /// Completes once the pages were replaced since the last call, or since
    /// this was made; several replacements in between count as one. Once
    /// every [`LivePages`] is gone no replacement can come, and it never
    /// completes.
    pub async fn next(&mut self) {
        if self.0.changed().await.is_err() {
            std::future::pending::<()>().await;
        }
    }
}

impl From<Pages> for LivePages {
    fn from(pages: Pages) -> Self {
        Self::new(pages)
    }
}
