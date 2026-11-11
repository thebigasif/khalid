//! The Search Console numbers `khalid serve` finds for the Admin API: the
//! follow-ups (GEO M3), the report (G43), each post's month for the posts
//! table and the whole site's totals for the Analytics screen, with the
//! code `khalid
//! search-console follow-ups` and `report` run. The
//! connection is read once at start; the posts are read on each request,
//! so a heading saved since counts.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::NaiveDate;
use khalid_api::content_store::BoxFuture;
use khalid_api::follow_ups::{FollowUpError, FollowUpFinder, FollowUps};
use khalid_api::search_report::{SearchNumbers, SearchPosts, SearchReport, SearchTotals};
use khalid_content::{ContentTree, SiteContent, read_content};
use khalid_core::{Clock, Config, Environment, SystemClock, UserMessage};
use khalid_outbound::Outbound;
use khalid_search_console::{SearchConsole, Totals};

use crate::bing::Failed;
use crate::cli::Cli;
use crate::exit::Exit;
use crate::search_console::{self, last_days, last_month};
use crate::search_console_follow_ups::found;
use crate::search_console_report::{gather, gather_posts};

/// A gathering of numbers for the site in a folder, from Google up to a day.
type Gather<T> =
    fn((&Path, &Config, &SiteContent), (&SearchConsole, &Outbound, NaiveDate)) -> Result<T, Failed>;

/// The follow-ups of the site in one folder.
#[derive(Debug)]
pub(crate) struct SiteFollowUps {
    folder: PathBuf,
    config: Config,
    /// The connection and the client that reaches Google, or `None` when
    /// the site has no Search Console connection or no Google sign-in.
    connection: Option<Arc<(SearchConsole, Outbound)>>,
}

impl SiteFollowUps {
    /// The follow-ups of the site in `folder` with `config`, signing in
    /// with the secrets in `env`.
    pub(crate) fn new(folder: &Path, config: &Config, env: &Environment, cli: &Cli) -> Self {
        Self {
            folder: folder.to_path_buf(),
            config: config.clone(),
            connection: search_console::connect(config, env, cli).ok().map(Arc::new),
        }
    }

    /// The follow-ups Google shares for the last month whose numbers are
    /// final, per post of the site as it is now.
    async fn find(&self) -> Result<FollowUps, FollowUpError> {
        let Some((console, outbound)) = self.connection.as_deref() else {
            return Err(FollowUpError::NotConnected(not_connected()));
        };
        let month = last_month(SystemClock.now().date_naive());
        let searches = async { console.sign_in(outbound).await?.searches(month).await };
        let searches = searches.await.map_err(|error| {
            FollowUpError::Google(search_console::message(console.property(), &error))
        })?;
        let (folder, config) = (self.folder.clone(), self.config.clone());
        let property = console.property().to_owned();
        let read = tokio::task::spawn_blocking(move || {
            let content = ContentTree::walk(&folder)
                .and_then(|tree| read_content(&tree))
                .map_err(|error| FollowUpError::Internal(Box::new(error)))?;
            found((&folder, &config, &content), &searches, (&property, month))
                .map_err(|(message, _)| unreadable(&message))
        });
        read.await
            .map_err(|error| FollowUpError::Internal(Box::new(error)))?
    }

    /// What `work` gathers, as `khalid search-console report` does, for
    /// the site as it is now. Gathering asks Google, and the report reads
    /// the 404 log, on its own runtime, so it runs on a blocking thread.
    async fn gathered<T: Send + 'static>(&self, work: Gather<T>) -> Result<T, FollowUpError> {
        let Some(connection) = self.connection.clone() else {
            return Err(FollowUpError::NotConnected(not_connected()));
        };
        let (folder, config) = (self.folder.clone(), self.config.clone());
        let read = tokio::task::spawn_blocking(move || {
            let content = ContentTree::walk(&folder)
                .and_then(|tree| read_content(&tree))
                .map_err(|error| FollowUpError::Internal(Box::new(error)))?;
            let (console, outbound) = connection.as_ref();
            let today = SystemClock.now().date_naive();
            work((&folder, &config, &content), (console, outbound, today)).map_err(
                |(message, exit)| match exit {
                    Exit::Internal => unreadable(&message),
                    _ => FollowUpError::Google(message),
                },
            )
        });
        read.await
            .map_err(|error| FollowUpError::Internal(Box::new(error)))?
    }
}

impl SiteFollowUps {
    /// Every page's numbers added up for the last `days` final days, and
    /// for the `days` days before, in one sign-in and two requests.
    async fn totals_now(&self, days: u32) -> Result<SearchTotals, FollowUpError> {
        let Some((console, outbound)) = self.connection.as_deref() else {
            return Err(FollowUpError::NotConnected(not_connected()));
        };
        let window = last_days(SystemClock.now().date_naive(), days);
        let before = window.before().unwrap_or(window);
        let asked = async {
            let session = console.sign_in(outbound).await?;
            tokio::try_join!(session.pages(window), session.pages(before))
        };
        let (now, then) = asked.await.map_err(|error| {
            FollowUpError::Google(search_console::message(console.property(), &error))
        })?;
        Ok(SearchTotals {
            property: console.property().to_owned(),
            days,
            first: window.first,
            last: window.last,
            now: numbers(Totals::of(&now)),
            before: numbers(Totals::of(&then)),
        })
    }
}

/// `totals` in the Admin API's shape.
fn numbers(totals: Totals) -> SearchNumbers {
    SearchNumbers {
        clicks: totals.clicks,
        impressions: totals.impressions,
        ctr: totals.ctr,
        position: totals.position,
    }
}

impl FollowUpFinder for SiteFollowUps {
    fn follow_ups(&self) -> BoxFuture<'_, Result<FollowUps, FollowUpError>> {
        Box::pin(self.find())
    }

    fn report(&self) -> BoxFuture<'_, Result<SearchReport, FollowUpError>> {
        Box::pin(self.gathered(gather))
    }

    fn totals(&self, days: u32) -> BoxFuture<'_, Result<SearchTotals, FollowUpError>> {
        Box::pin(self.totals_now(days))
    }

    fn posts(&self) -> BoxFuture<'_, Result<SearchPosts, FollowUpError>> {
        Box::pin(self.gathered(gather_posts))
    }
}

/// The message when the site has no Search Console connection, or no
/// Google sign-in for it.
fn not_connected() -> UserMessage {
    UserMessage::new(
        "This site is not connected to Google Search Console, Google's report of how a site does in search.",
        "Khalid asks Google nothing until site.toml opts in under [search_console] and the server has the owner's Google sign-in.",
        "Follow the steps in docs/deploy.md under Google Search Console, then restart khalid serve.",
    )
}

/// The site's posts could not be listed, as `message` says.
fn unreadable(message: &UserMessage) -> FollowUpError {
    FollowUpError::Internal(std::io::Error::other(message.to_string()).into())
}
