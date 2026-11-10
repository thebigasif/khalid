//! The Admin API's subscribers and newsletters as `khalid serve` keeps
//! them: readers in the site's database, newsletters from
//! `content/newsletters/`, and a post sent the way `khalid newsletter
//! send` sends it (G39, plan 4.6).

use std::path::{Path, PathBuf};
use std::sync::Arc;

use khalid_api::content_store::BoxFuture;
use khalid_api::post_store::PostStore;
use khalid_api::resources::{
    AdminNewsletter, AdminSend, AdminSubscriber, NewsletterSend, PostStatus, SendReadiness,
    SubscriberFilter, SubscriberPage, SubscriberStatus,
};
use khalid_api::subscriber_desk::{LabelChange, SubscriberDesk, SubscriberError};
use khalid_core::{Clock, Config, Slug, SystemClock};
use khalid_db::SubscriberTable;
use khalid_newsletter::{
    Lists, Mailer, Newsletter, Pages, PostalAddress, Segment, SubscriberStore,
};

use crate::site_posts::SitePosts;
use crate::{newsletter, newsletter_files, newsletter_send};

/// One site's subscribers and newsletters, for the Admin API.
#[derive(Clone)]
pub(crate) struct SiteSubscribers {
    table: SubscriberTable,
    folder: PathBuf,
    config: Config,
    posts: SitePosts,
    mailer: Option<Arc<dyn Mailer>>,
}

impl SiteSubscribers {
    /// The subscribers in `table` of the site in `folder` with `config`,
    /// sending the posts in `posts` through `mailer` when the site sends
    /// mail.
    pub(crate) fn new(
        table: SubscriberTable,
        (folder, config): (&Path, &Config),
        posts: SitePosts,
        mailer: Option<Arc<dyn Mailer>>,
    ) -> Self {
        Self {
            table,
            folder: folder.to_path_buf(),
            config: config.clone(),
            posts,
            mailer,
        }
    }

    /// The site's newsletters, read from its folder off the async threads.
    async fn lists(&self) -> Result<Lists, SubscriberError> {
        let folder = self.folder.clone();
        let read = tokio::task::spawn_blocking(move || newsletter_files::lists(&folder));
        read.await.map_err(SubscriberError::store)
    }

    /// The mailer and postal address a send needs, or what the site lacks.
    fn sender(&self) -> Result<(Arc<dyn Mailer>, PostalAddress), SendReadiness> {
        let mail = self.config.mail().ok_or(SendReadiness::NoMail)?;
        let mailer = self.mailer.clone().ok_or(SendReadiness::NoMail)?;
        let postal = mail.postal_address.as_deref().and_then(PostalAddress::new);
        Ok((mailer, postal.ok_or(SendReadiness::NoPostalAddress)?))
    }

    /// Checks that `send` names a published post not sent before, then
    /// sends it to `segment` in the background, and answers its key.
    async fn start(
        &self,
        segment: Segment,
        send: &NewsletterSend,
        (mailer, postal): (Arc<dyn Mailer>, PostalAddress),
    ) -> Result<String, SubscriberError> {
        let post = self.posts.get(&send.post).await;
        let post = post.map_err(SubscriberError::store)?;
        let post = post.ok_or(SubscriberError::NoPost)?;
        if post.status() != PostStatus::Published {
            return Err(SubscriberError::NotPublished);
        }
        let name = send.post.replacen('.', "/", 1);
        let issue = newsletter_send::issue_of(&self.config, &post, &name);
        let key = issue.key().to_owned();
        let sends = self.table.sends().await.map_err(SubscriberError::store)?;
        if sends.iter().any(|sent| sent.key == key) {
            return Err(SubscriberError::SentBefore);
        }
        let clock: Arc<dyn Clock> = Arc::new(SystemClock);
        let store = Arc::new(self.table.clone());
        let site = newsletter::site(&self.config);
        let flow = Newsletter::new(store, mailer, clock, site).map_err(SubscriberError::store)?;
        tokio::spawn(async move {
            // The key alone: an error's cause may name an address (BL-RUN-5).
            if let Err(error) = flow.send(&issue, &segment, &postal).await {
                tracing::warn!(%error, key = issue.key(), "a newsletter could not finish sending");
            }
        });
        Ok(key)
    }
}

impl SubscriberDesk for SiteSubscribers {
    fn list<'a>(
        &'a self,
        filter: &'a SubscriberFilter,
        at: (u32, u32),
    ) -> BoxFuture<'a, Result<SubscriberPage, SubscriberError>> {
        Box::pin(async move {
            self.table
                .page(filter, at)
                .await
                .map_err(SubscriberError::store)
        })
    }

    fn label<'a>(
        &'a self,
        id: i64,
        label: &'a Slug,
        change: LabelChange,
    ) -> BoxFuture<'a, Result<AdminSubscriber, SubscriberError>> {
        Box::pin(async move {
            let at = SystemClock.now();
            let changed = self.table.change_label(id, label, (change, at)).await;
            let subscriber = changed.map_err(SubscriberError::store)?;
            let subscriber = subscriber.ok_or(SubscriberError::NotFound)?;
            let left = subscriber.status == SubscriberStatus::Unsubscribed;
            if left && change == LabelChange::Add {
                return Err(SubscriberError::Unsubscribed);
            }
            Ok(subscriber)
        })
    }

    fn erase(&self, id: i64) -> BoxFuture<'_, Result<(), SubscriberError>> {
        Box::pin(async move {
            let erased = self.table.erase(id).await.map_err(SubscriberError::store)?;
            erased.then_some(()).ok_or(SubscriberError::NotFound)
        })
    }

    fn newsletters(&self) -> BoxFuture<'_, Result<Vec<AdminNewsletter>, SubscriberError>> {
        Box::pin(async move {
            let lists = self.lists().await?;
            let counts = self.table.list_counts().await;
            let counts = counts.map_err(SubscriberError::store)?;
            let pages = Pages::new(newsletter::site(&self.config));
            let pages = pages.map_err(SubscriberError::store)?;
            Ok(lists
                .iter()
                .map(|list| {
                    let counted = counts.get(list.as_str()).copied().unwrap_or_default();
                    AdminNewsletter::new(list.clone(), counted, pages.embed(list))
                })
                .collect())
        })
    }

    fn sends(&self) -> BoxFuture<'_, Result<Vec<AdminSend>, SubscriberError>> {
        Box::pin(async move {
            let sends = self.table.sends().await.map_err(SubscriberError::store)?;
            Ok(sends
                .into_iter()
                .map(|sent| {
                    let totals = (sent.totals.sent, sent.totals.failed, sent.unsubscribed);
                    AdminSend::new(sent.key, sent.list, sent.started_at, totals)
                })
                .collect())
        })
    }

    fn readiness(&self) -> SendReadiness {
        self.sender().err().unwrap_or(SendReadiness::Ready)
    }

    fn send<'a>(
        &'a self,
        list: &'a Slug,
        send: &'a NewsletterSend,
    ) -> BoxFuture<'a, Result<String, SubscriberError>> {
        Box::pin(async move {
            let sender = self.sender().map_err(SubscriberError::NotReady)?;
            let lists = self.lists().await?;
            let list = lists
                .find(Some(list.as_str()))
                .ok_or(SubscriberError::NoList)?;
            let segment = Segment::new(list, send.labels.iter().cloned());
            self.start(segment, send, sender).await
        })
    }
}
