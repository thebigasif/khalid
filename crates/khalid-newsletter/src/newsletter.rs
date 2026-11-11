//! The sign-up flow: a form makes a subscriber wait, one email asks them,
//! and a link confirms or removes them. Then the issues that go out to them.

use std::sync::Arc;

use chrono::TimeDelta;
use khalid_core::{AbsoluteUrl, Clock, Locale, Slug};

use crate::clicks::{self, Piece};
use crate::{
    BackPath, ConfirmationLinks, Confirmed, EmailAddress, GenerateTokenError, Issue, Lists, Mail,
    MailError, Mailer, Member, NewsletterWords, PostalAddress, ReaderPass, Recipient, Segment,
    SendId, SendTotals, Sent, SignUp, SignedUp, SiteLinks, Stop, StoreError, SubscriberId,
    SubscriberStore, SubscriberToken, Unsubscribed, Welcome, Welcomes, WordsError,
};

/// How long after its day a later welcome step may still go out, so a
/// server stopped for a few days catches up, and a reader who joined long
/// ago is not sent a sequence written since.
const LATE_FOR: TimeDelta = TimeDelta::days(7);

/// The site the emails come from: its name, its language and where its
/// pages answer.
#[derive(Debug, Clone)]
pub struct NewsletterSite {
    /// The site's name, as the emails name it.
    pub name: String,
    /// The language the emails are written in.
    pub locale: Locale,
    /// Where the links in the emails lead.
    pub links: SiteLinks,
}

/// One site's newsletter: its subscribers, how mail leaves, and its clock
/// (RULES.md BL-RUN-1). Cloning shares all three.
#[derive(Debug, Clone)]
pub struct Newsletter {
    store: Arc<dyn SubscriberStore>,
    mailer: Arc<dyn Mailer>,
    clock: Arc<dyn Clock>,
    site: Arc<(NewsletterSite, NewsletterWords)>,
    pass: Option<ReaderPass>,
    lists: Arc<Lists>,
    welcomes: Option<Arc<Welcomes>>,
    count_clicks: bool,
}

impl Newsletter {
    /// The newsletter of `site`, keeping subscribers in `store` and
    /// sending through `mailer`.
    ///
    /// # Errors
    ///
    /// [`WordsError`] when a built-in words file is not valid, which this
    /// crate's tests rule out.
    pub fn new(
        store: Arc<dyn SubscriberStore>,
        mailer: Arc<dyn Mailer>,
        clock: Arc<dyn Clock>,
        site: NewsletterSite,
    ) -> Result<Self, WordsError> {
        let words = NewsletterWords::load()?;
        Ok(Self {
            store,
            mailer,
            clock,
            site: Arc::new((site, words)),
            pass: None,
            lists: Arc::new(Lists::main_only()),
            welcomes: None,
            count_clicks: false,
        })
    }

    /// The same newsletter, taking sign-ups for `lists` instead of `main`
    /// alone.
    #[must_use]
    pub fn with_lists(self, lists: Lists) -> Self {
        Self {
            lists: Arc::new(lists),
            ..self
        }
    }

    /// The site's newsletters.
    pub fn lists(&self) -> &Lists {
        &self.lists
    }

    /// This newsletter, giving each reader who confirms the cookie `pass`
    /// makes, so they can read gated content.
    #[must_use]
    pub fn with_pass(mut self, pass: ReaderPass) -> Self {
        self.pass = Some(pass);
        self
    }

    /// The reader pass, when gated content opens for subscribers.
    pub fn pass(&self) -> Option<&ReaderPass> {
        self.pass.as_ref()
    }

    /// Signs `email` up for `list` from the gated content at `back`, as
    /// [`sign_up`](Self::sign_up) does, with `back` in the confirmation
    /// link. An address confirmed before gets a pass link to `back`
    /// instead, so a subscriber on a new device can read too.
    ///
    /// # Errors
    ///
    /// [`NewsletterError`] as [`sign_up`](Self::sign_up) returns it.
    pub async fn sign_up_to_read(
        &self,
        email: EmailAddress,
        list: Slug,
        back: &BackPath,
    ) -> Result<SignedUp, NewsletterError> {
        let signed = self.signed_up(email.clone(), list, Some(back)).await?;
        let confirmed = matches!(signed, SignedUp::AlreadyConfirmed | SignedUp::JoinedNow(_));
        if let (true, Some(pass)) = (confirmed, &self.pass) {
            let (site, words) = &*self.site;
            let mail = words.pass(&site.locale, &site.name, email, &pass.link(back));
            self.mailer.send(&mail).await?;
        }
        Ok(signed)
    }

    /// Signs `email` up for `list`. When a confirmation is due, sends it,
    /// with a link that confirms and a link that removes the address.
    /// The reader is told the same thing whatever the answer, so the form
    /// never reveals who subscribes.
    ///
    /// # Errors
    ///
    /// [`NewsletterError`] when there is no randomness for a token, the
    /// store fails, or the transport does not take the email. A failed
    /// email can be asked for again after ten minutes.
    pub async fn sign_up(
        &self,
        email: EmailAddress,
        list: Slug,
    ) -> Result<SignedUp, NewsletterError> {
        self.signed_up(email, list, None).await
    }

    /// Signs up as [`sign_up`](Self::sign_up) says, with `back` in the link.
    async fn signed_up(
        &self,
        email: EmailAddress,
        list: Slug,
        back: Option<&BackPath>,
    ) -> Result<SignedUp, NewsletterError> {
        let (confirm, remove) = (SubscriberToken::generate()?, SubscriberToken::generate()?);
        let at = self.clock.now();
        let asked = SignUp::new(email.clone(), list.clone(), confirm.hash(), at);
        let signed = self.store.sign_up(&asked).await?;
        if let SignedUp::JoinedNow(subscriber) = signed {
            self.welcome(&Recipient::new(subscriber, email.clone()), &[list])
                .await;
        }
        if let SignedUp::ConfirmationDue(subscriber) = signed {
            self.store
                .add_unsubscribe(subscriber, &remove.hash(), at)
                .await?;
            let (site, words) = &*self.site;
            let links = ConfirmationLinks {
                site: &site.links,
                confirm: &confirm,
                remove: &remove,
                back,
            };
            let mail = words.confirmation(&site.locale, &site.name, email, links);
            self.mailer.send(&mail).await?;
        }
        Ok(signed)
    }

    /// The same newsletter, sending the first of `welcomes` to each reader
    /// the moment they join, and the later steps through
    /// [`send_due_welcomes`](Self::send_due_welcomes).
    #[must_use]
    pub fn with_welcomes(self, welcomes: Welcomes) -> Self {
        Self {
            welcomes: Some(Arc::new(welcomes)),
            ..self
        }
    }

    /// Sends `subscriber`, confirmed a moment ago, the first welcome email
    /// of each newsletter they joined that has one.
    async fn welcome_confirmed(&self, subscriber: SubscriberId) {
        if self.welcomes.is_none() {
            return;
        }
        match self.store.subscriber(subscriber).await {
            Ok(Some(Member {
                recipient, lists, ..
            })) => self.welcome(&recipient, &lists).await,
            Ok(None) => {}
            Err(error) => tracing::warn!(%error, "a welcome email was not sent"),
        }
    }

    /// Sends `recipient`, who joined each of `lists` a moment ago, the
    /// step of its welcome sequence sent at once, when it has one.
    async fn welcome(&self, recipient: &Recipient, lists: &[Slug]) {
        let Some(welcomes) = &self.welcomes else {
            return;
        };
        for list in lists {
            let steps = welcomes.of(list).await;
            if let Some(first) = steps.first().filter(|step| step.day() == 0) {
                self.welcome_step(welcomes, recipient, (list, first)).await;
            }
        }
    }

    /// Sends `recipient` the welcome `step` of `list`, unless it went to
    /// them before, and says whether it went now. A step that cannot go
    /// out is logged and left, never tried again: the reader is
    /// subscribed all the same.
    async fn welcome_step(
        &self,
        welcomes: &Welcomes,
        recipient: &Recipient,
        (list, step): (&Slug, &Welcome),
    ) -> bool {
        let (site, words) = &*self.site;
        let sent = async {
            let at = self.clock.now();
            let id = recipient.id;
            if !self.store.claim_welcome(id, list, step.day(), at).await? {
                return Ok(false);
            }
            let remove = SubscriberToken::generate()?;
            self.store.add_unsubscribe(id, &remove.hash(), at).await?;
            let link = site.links.unsubscribe(&remove);
            let footer = words.footer(&site.locale, &site.name, &link, welcomes.postal());
            let text = format!("{}{footer}", step.text());
            let to = recipient.email.clone();
            let mail = Mail::new(to, step.subject().to_owned(), text);
            self.mailer.send(&mail).await?;
            Ok::<_, NewsletterError>(true)
        };
        sent.await.unwrap_or_else(|error| {
            tracing::warn!(%error, "a welcome email was not sent");
            false
        })
    }

    /// Sends each later step of every welcome sequence to the readers it
    /// has fallen due for, once, and gives how many went out. A step more
    /// than a week past its day is skipped, so a sequence written long
    /// after a reader joined does not reach them in a burst. Run it every
    /// hour or so.
    ///
    /// # Errors
    ///
    /// [`NewsletterError::Store`] when the store cannot be read. The steps
    /// sent before the failure stay sent.
    pub async fn send_due_welcomes(&self) -> Result<u32, NewsletterError> {
        let Some(welcomes) = &self.welcomes else {
            return Ok(0);
        };
        let now = self.clock.now();
        let due = (now - LATE_FOR, now);
        let mut sent = 0_u32;
        for list in self.lists.iter() {
            for step in welcomes.of(list).await.iter().filter(|step| step.day() > 0) {
                for recipient in self.store.welcomes_due(list, step.day(), due).await? {
                    if self.welcome_step(welcomes, &recipient, (list, step)).await {
                        sent = sent.saturating_add(1);
                    }
                }
            }
        }
        Ok(sent)
    }

    /// Confirms the subscriber whose link holds `token`, as typed in the
    /// link, and sends the welcome emails of the newsletters they joined.
    /// Text that is not a token confirms nobody.
    ///
    /// # Errors
    ///
    /// [`NewsletterError::Store`] when the store fails.
    pub async fn confirm(&self, token: &str) -> Result<Confirmed, NewsletterError> {
        let Ok(token) = SubscriberToken::parse(token) else {
            return Ok(Confirmed::Unknown);
        };
        let confirmed = self.store.confirm(&token.hash(), self.clock.now()).await?;
        if let Confirmed::Now(subscriber) = confirmed {
            self.welcome_confirmed(subscriber).await;
        }
        Ok(confirmed)
    }

    /// Unsubscribes the subscriber whose link holds `token`, as typed in
    /// the link. Text that is not a token unsubscribes nobody.
    ///
    /// # Errors
    ///
    /// [`NewsletterError::Store`] when the store fails.
    pub async fn unsubscribe(&self, token: &str) -> Result<Unsubscribed, NewsletterError> {
        let Ok(token) = SubscriberToken::parse(token) else {
            return Ok(Unsubscribed::Unknown);
        };
        Ok(self
            .store
            .unsubscribe(&token.hash(), self.clock.now())
            .await?)
    }

    /// Sends `email` no more newsletters, as the mail service asked because
    /// of `why`, and keeps the sign-up form from mailing it again. Returns
    /// whether the address was a subscriber's.
    ///
    /// # Errors
    ///
    /// [`NewsletterError::Store`] when the store fails.
    pub async fn stop(&self, email: &EmailAddress, why: Stop) -> Result<bool, NewsletterError> {
        Ok(self.store.stop(email, why, self.clock.now()).await?)
    }
}

impl Newsletter {
    /// Emails `issue` to everyone in `segment`, once: an issue whose key
    /// went out before sends nothing. Each copy ends with the reader's own
    /// one-click unsubscribe link and the site's `postal` address. A copy
    /// the transport refuses is counted and the rest still go.
    ///
    /// # Errors
    ///
    /// [`NewsletterError`] when there is no randomness for a link or the
    /// store fails. The copies sent before a failure stay sent, and the
    /// key stays used, so a second try never sends anyone a copy twice.
    pub async fn send(
        &self,
        issue: &Issue,
        segment: &Segment,
        postal: &PostalAddress,
    ) -> Result<Sent, NewsletterError> {
        let at = self.clock.now();
        let Some(send) = self
            .store
            .start_send(issue.key(), segment.list(), at)
            .await?
        else {
            return Ok(Sent::Before);
        };
        let (site, words) = &*self.site;
        let body = self.counted(send, issue.text()).await?;
        let mut totals = SendTotals::default();
        for recipient in self.store.audience(segment).await? {
            let remove = SubscriberToken::generate()?;
            self.store
                .add_send_unsubscribe(send, recipient.id, &remove.hash(), at)
                .await?;
            let link = site.links.unsubscribe(&remove);
            let footer = words.footer(&site.locale, &site.name, &link, postal);
            let text = format!("{body}{footer}");
            let mail = Mail::new(recipient.email, issue.subject().to_owned(), text);
            match self.mailer.send(&mail).await {
                Ok(()) => totals.sent = totals.sent.saturating_add(1),
                Err(error) => {
                    tracing::warn!(%error, "a newsletter copy was not sent");
                    totals.failed = totals.failed.saturating_add(1);
                }
            }
        }
        self.store
            .finish_send(send, totals, self.clock.now())
            .await?;
        Ok(Sent::Now(totals))
    }

    /// The same newsletter, counting clicks on the links in each email it
    /// sends, as `[mail] count_clicks = true` asks (G39).
    #[must_use]
    pub fn with_click_counting(self) -> Self {
        Self {
            count_clicks: true,
            ..self
        }
    }

    /// `text` of `send` as every copy carries it: when clicks are counted,
    /// each web link is stored and leads through the blog first.
    async fn counted(&self, send: SendId, text: &str) -> Result<String, NewsletterError> {
        if !self.count_clicks {
            return Ok(text.to_owned());
        }
        let (pieces, links) = clicks::pieces(text);
        for (position, url) in (1_u32..).zip(&links) {
            self.store.add_link(send, position, url).await?;
        }
        let links = &self.site.0.links;
        Ok(pieces
            .iter()
            .map(|piece| match piece {
                Piece::Text(text) => (*text).to_owned(),
                Piece::Link(position) => links.click(send, *position),
            })
            .collect())
    }

    /// Counts a click on link number `position` of `send`, and gives where
    /// it leads, or `None` when no email had that link.
    ///
    /// # Errors
    ///
    /// [`NewsletterError::Store`] when the store fails.
    pub async fn click(
        &self,
        send: SendId,
        position: u32,
    ) -> Result<Option<AbsoluteUrl>, NewsletterError> {
        Ok(self.store.click(send, position).await?)
    }
}

/// Why a sign-up, a confirmation or an unsubscribe did not happen.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum NewsletterError {
    /// The operating system gave no randomness for a link.
    #[error(transparent)]
    Random(#[from] GenerateTokenError),
    /// The subscriber list could not be read or written.
    #[error(transparent)]
    Store(#[from] StoreError),
    /// The transport did not take the confirmation email.
    #[error(transparent)]
    Mail(#[from] MailError),
}
