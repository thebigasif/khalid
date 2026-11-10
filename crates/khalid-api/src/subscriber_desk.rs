//! Where the Admin API reads and changes newsletter subscribers, and sends
//! a post to a newsletter (G39, plan 4.6).
//!
//! Readers join only through the sign-up form and its confirmation email
//! (double opt-in), so no route adds a subscriber.

use std::error::Error;

use khalid_core::Slug;

use crate::content_store::BoxFuture;
use crate::resources::{
    AdminNewsletter, AdminSend, AdminSubscriber, NewsletterSend, SendReadiness, SubscriberFilter,
    SubscriberPage,
};

/// A site's subscribers and newsletters, for the Admin API. `khalid serve`
/// implements it over the site's database and `content/newsletters/`.
pub trait SubscriberDesk: Send + Sync {
    /// The subscribers `filter` matches, newest first, on the page that
    /// `at` gives as `(page, limit)`: page from 1, `limit` to a page.
    ///
    /// # Errors
    ///
    /// [`SubscriberError::Store`] when the subscribers cannot be read.
    fn list<'a>(
        &'a self,
        filter: &'a SubscriberFilter,
        at: (u32, u32),
    ) -> BoxFuture<'a, Result<SubscriberPage, SubscriberError>>;

    /// Gives subscriber `id` the label `label`, or takes it away, as
    /// `change` says, and returns them. Doing it twice changes nothing.
    ///
    /// # Errors
    ///
    /// [`SubscriberError::NotFound`] when no subscriber has that identity,
    /// [`SubscriberError::Unsubscribed`] when they left and a label is
    /// added, and [`SubscriberError::Store`] when it cannot be written.
    fn label<'a>(
        &'a self,
        id: i64,
        label: &'a Slug,
        change: LabelChange,
    ) -> BoxFuture<'a, Result<AdminSubscriber, SubscriberError>>;

    /// Removes subscriber `id` and everything kept about them, for a
    /// reader who asks to be forgotten.
    ///
    /// # Errors
    ///
    /// [`SubscriberError::NotFound`] when no subscriber has that identity,
    /// and [`SubscriberError::Store`] when it cannot be written.
    fn erase(&self, id: i64) -> BoxFuture<'_, Result<(), SubscriberError>>;

    /// The site's newsletters in alphabetical order, each with its counts.
    ///
    /// # Errors
    ///
    /// [`SubscriberError::Store`] when the subscribers cannot be read.
    fn newsletters(&self) -> BoxFuture<'_, Result<Vec<AdminNewsletter>, SubscriberError>>;

    /// Every email that went out, newest first.
    ///
    /// # Errors
    ///
    /// [`SubscriberError::Store`] when the numbers cannot be read.
    fn sends(&self) -> BoxFuture<'_, Result<Vec<AdminSend>, SubscriberError>>;

    /// Whether a post can be sent now.
    fn readiness(&self) -> SendReadiness;

    /// Starts emailing the post `send` names to newsletter `list`, once,
    /// and returns the key it goes out under. The copies leave after the
    /// answer; [`SubscriberDesk::sends`] shows their totals.
    ///
    /// # Errors
    ///
    /// [`SubscriberError::NotReady`] when the site cannot send,
    /// [`SubscriberError::NoList`] or [`SubscriberError::NoPost`] when the
    /// newsletter or post does not exist, [`SubscriberError::NotPublished`]
    /// when the post is not live, [`SubscriberError::SentBefore`] when it
    /// went to that newsletter before, and [`SubscriberError::Store`] when
    /// the store cannot be read.
    fn send<'a>(
        &'a self,
        list: &'a Slug,
        send: &'a NewsletterSend,
    ) -> BoxFuture<'a, Result<String, SubscriberError>>;
}

/// Whether a label is given or taken away.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelChange {
    /// Give the subscriber the label.
    Add,
    /// Take the label away.
    Remove,
}

/// Why a subscriber or newsletter request could not be done.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SubscriberError {
    /// No subscriber has that identity.
    #[error("no subscriber has that identity")]
    NotFound,
    /// The subscriber left, so they get no new label.
    #[error("the subscriber unsubscribed")]
    Unsubscribed,
    /// The site has no newsletter by that name.
    #[error("the site has no newsletter by that name")]
    NoList,
    /// The site has no post with that identity.
    #[error("the site has no post with that identity")]
    NoPost,
    /// The post is not published.
    #[error("the post is not published")]
    NotPublished,
    /// The post went to that newsletter before.
    #[error("the post went to that newsletter before")]
    SentBefore,
    /// The site cannot send email yet.
    #[error("the site cannot send email yet")]
    NotReady(SendReadiness),
    /// The subscribers could not be read or written.
    #[error("the subscribers could not be read or written")]
    Store(#[source] Box<dyn Error + Send + Sync>),
}

impl SubscriberError {
    /// The store's own `source`, kept as the cause.
    pub fn store(source: impl Error + Send + Sync + 'static) -> Self {
        Self::Store(Box::new(source))
    }
}
