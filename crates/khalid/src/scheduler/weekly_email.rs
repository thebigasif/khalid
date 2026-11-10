//! The weekly Search Console email in `khalid serve` (G43, plan 5.11):
//! once an hour the scheduler asks whether a week has passed since the
//! email last went out, and sends it when it has, the same email
//! `khalid search-console email` sends. A site that names no `report_to`,
//! has no `[mail]` or no Google sign-in gets no email and no timer.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use khalid_content::{ContentTree, SiteContent, read_content};
use khalid_core::{Clock, Config, Environment, SystemClock};
use khalid_newsletter::{EmailAddress, Mailer};
use khalid_outbound::Outbound;
use khalid_search_console::SearchConsole;

use crate::cli::Cli;
use crate::search_console;
use crate::search_console_email::{due, mail_of, send};

/// What the weekly email needs, read once at start.
#[derive(Debug)]
pub(crate) struct WeeklyEmail {
    folder: PathBuf,
    config: Config,
    to: EmailAddress,
    mailer: Arc<dyn Mailer>,
    console: SearchConsole,
    outbound: Outbound,
}

impl WeeklyEmail {
    /// The weekly email of the site in `folder` with `config`, signed in
    /// with the secrets in `env`, or `None` when the site is not set up
    /// to get one.
    pub(crate) fn new(
        folder: &Path,
        config: &Config,
        env: &Environment,
        cli: &Cli,
    ) -> Option<Arc<Self>> {
        let (console, outbound) = search_console::connect(config, env, cli).ok()?;
        let (to, mailer) = mail_of(config, env, &outbound).ok()?;
        Some(Arc::new(Self {
            folder: folder.to_path_buf(),
            config: config.clone(),
            to,
            mailer,
            console,
            outbound,
        }))
    }

    /// Sends the email when it is due, and logs what happened. Asks Google
    /// and reads files, so it runs on a blocking thread (RULES.md
    /// BL-RUN-7).
    pub(crate) async fn round(self: &Arc<Self>) {
        let weekly = Arc::clone(self);
        let done = tokio::task::spawn_blocking(move || weekly.send_if_due()).await;
        if let Err(error) = done {
            tracing::warn!(%error, "the weekly Search Console email stopped early");
        }
    }

    /// Sends the email when a week has passed since it last went out.
    fn send_if_due(&self) {
        let today = SystemClock.now().date_naive();
        if !due(&self.folder, today) {
            return;
        }
        let content = match self.content() {
            Ok(content) => content,
            Err(error) => {
                tracing::warn!(%error, "the posts could not be read, so the weekly email waits an hour");
                return;
            }
        };
        let site = (self.folder.as_path(), &self.config, &content);
        let asked = (&self.console, &self.outbound, today);
        match send(site, (&self.to, self.mailer.as_ref()), asked) {
            Ok(()) => tracing::info!(to = %self.to, "the weekly Search Console email went out"),
            Err((message, _)) => tracing::warn!(
                %message,
                "the weekly Search Console email was not sent, so it is tried again in an hour"
            ),
        }
    }

    /// The site's content as it is now.
    fn content(&self) -> Result<SiteContent, khalid_content::ContentError> {
        ContentTree::walk(&self.folder).and_then(|tree| read_content(&tree))
    }
}
