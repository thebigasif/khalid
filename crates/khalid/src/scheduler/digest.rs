//! The newsletter digest in `khalid serve` (backlog row 16, plan 5.6):
//! once an hour the scheduler sends each newsletter whose folder holds a
//! `digest.toml` the posts published since its last digest, once a day or
//! once a week at most, the same email `khalid newsletter digest` sends.
//! A site with no `[mail]` or no postal address gets no digest and no
//! timer; a round with nothing new sends nothing.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::{DateTime, Utc};
use khalid_core::{Clock, Config, SystemClock};
use khalid_db::{Database, SiteId, SubscriberTable};
use khalid_newsletter::{
    Mailer, Newsletter, NewsletterWords, PostalAddress, Segment, Sent, SubscriberStore,
};

use crate::newsletter_files::digest_plan::{self, DigestPlan};
use crate::{index, newsletter, newsletter_digest};

/// What the digests need, read once at start.
pub(crate) struct Digests {
    folder: PathBuf,
    config: Config,
    table: SubscriberTable,
    mailer: Arc<dyn Mailer>,
    postal: PostalAddress,
}

impl std::fmt::Debug for Digests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Digests").finish_non_exhaustive()
    }
}

impl Digests {
    /// The digests of the site in `folder` with `config`, whose readers are
    /// site `id` of `database`, sent through `mailer`, or `None` when the
    /// site cannot send a newsletter.
    pub(crate) fn new(
        (folder, config): (&Path, &Config),
        (database, id): (&Database, SiteId),
        mailer: Option<Arc<dyn Mailer>>,
    ) -> Option<Arc<Self>> {
        let mail = config.mail()?;
        let postal = mail
            .postal_address
            .as_deref()
            .and_then(PostalAddress::new)?;
        Some(Arc::new(Self {
            folder: folder.to_path_buf(),
            config: config.clone(),
            table: SubscriberTable::new(database.clone(), id),
            mailer: mailer?,
            postal,
        }))
    }

    /// Sends each digest due at `now` that has new posts, and logs what
    /// happened. A failure waits for the next round, an hour later.
    pub(crate) async fn round(&self, now: DateTime<Utc>) {
        let folder = self.folder.clone();
        let plans = tokio::task::spawn_blocking(move || digest_plan::plans(&folder)).await;
        let plans = plans.unwrap_or_default();
        if plans.is_empty() {
            return;
        }
        let sends = match self.table.sends().await {
            Ok(sends) => sends,
            Err(error) => {
                tracing::warn!(%error, "the newsletter's past emails could not be read, so the digests wait an hour");
                return;
            }
        };
        let due: Vec<(_, DigestPlan)> = plans
            .into_iter()
            .filter(|(list, plan)| plan.due(list, &sends, now))
            .collect();
        if due.is_empty() {
            return;
        }
        let (folder, config) = (self.folder.clone(), self.config.clone());
        let index = tokio::task::spawn_blocking(move || index::of(&folder, &config)).await;
        let Ok(Ok(index)) = index else {
            tracing::warn!("the posts could not be read, so the digests wait an hour");
            return;
        };
        let Some(words) = self.words() else {
            return;
        };
        for (list, plan) in due {
            let language = plan
                .language
                .as_ref()
                .unwrap_or_else(|| self.config.default_language());
            let today = now.date_naive();
            let issue = newsletter_digest::next_issue(
                (&self.config, &words),
                &index,
                &sends,
                (&list, language),
                today,
            );
            let Some(issue) = issue else { continue };
            let segment = Segment::new(list.clone(), plan.labels.iter().cloned());
            self.send(&issue, &segment, &words).await;
        }
    }

    /// The newsletter's words, with the site's own laid over them, or the
    /// built-in ones alone when the site's cannot be used, which is logged.
    fn words(&self) -> Option<NewsletterWords> {
        newsletter::words(&self.folder)
            .inspect_err(|message| {
                tracing::warn!(
                    what = message.what(),
                    "the site's newsletter words are left out"
                );
            })
            .ok()
            .or_else(|| NewsletterWords::load().ok())
    }

    /// Sends one digest `issue` to `segment`, worded with `words`.
    async fn send(
        &self,
        issue: &khalid_newsletter::Issue,
        segment: &Segment,
        words: &NewsletterWords,
    ) {
        let clock: Arc<dyn Clock> = Arc::new(SystemClock);
        let store: Arc<dyn SubscriberStore> = Arc::new(self.table.clone());
        let site = newsletter::site(&self.config);
        let Ok(flow) = Newsletter::new(store, Arc::clone(&self.mailer), clock, site) else {
            return;
        };
        let flow = flow.with_words(words.clone());
        let counting = self.config.mail().is_some_and(|mail| mail.count_clicks);
        let flow = if counting {
            flow.with_click_counting()
        } else {
            flow
        };
        let list = segment.list().as_str();
        // The key alone: an error's cause may name an address (BL-RUN-5).
        match flow.send(issue, segment, &self.postal).await {
            Ok(Sent::Now(totals)) => {
                tracing::info!(
                    list,
                    sent = totals.sent,
                    failed = totals.failed,
                    key = issue.key(),
                    "a newsletter digest went out"
                );
            }
            Ok(_) => {}
            Err(error) => {
                tracing::warn!(%error, key = issue.key(), "a newsletter digest could not finish sending");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use khalid_core::{Environment, SitePath, Slug};
    use khalid_newsletter::{EmailAddress, MemoryMailer, SignUp, SubscriberToken};

    use super::*;

    const SITE: &str = "name = \"Field Notes\"\naddress = \"https://www.example.com/blog/\"\ndefault_language = \"en\"\nlanguages = [\"en\"]\n\n[mail]\nfrom = \"news@example.com\"\npostal_address = \"Field Notes Ltd, 1 Park Row, Leeds\"\n\n[mail.resend]\n";

    #[cfg(test)]
    fn write(site: &Path, path: &str, text: &str) {
        let file = SitePath::within(site, path).unwrap();
        fs::create_dir_all(file.as_path().parent().unwrap()).unwrap();
        fs::write(file.as_path(), text).unwrap();
    }

    /// A site with one post dated `today`, a digest each week for `main`,
    /// and one confirmed reader.
    #[cfg(test)]
    async fn site(today: chrono::NaiveDate) -> (tempfile::TempDir, Config, Database, SiteId) {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "site.toml", SITE);
        let post = format!(
            "---\ntitle: Fresh news on crawling\nslug: fresh-news\ndate: {today}\nauthor: lena\n---\n\nNew pages first.\n"
        );
        write(dir.path(), "content/en/posts/fresh-news.md", &post);
        write(
            dir.path(),
            "content/newsletters/main/digest.toml",
            "every = \"week\"\n",
        );
        let env = Environment::from_pairs(&[(Environment::RESEND_API_KEY, "made-for-this-test")]);
        let config = Config::from_toml(Path::new("site.toml"), SITE, &env).unwrap();
        let file = SitePath::within(dir.path(), "site.db").unwrap();
        let db = Database::open(file.as_path()).await.unwrap();
        db.migrate().await.unwrap();
        let id = db.site(&Slug::try_new("main").unwrap()).await.unwrap();
        let table = SubscriberTable::new(db.clone(), id);
        let token = SubscriberToken::generate().unwrap();
        let email = EmailAddress::parse("grace@example.com").unwrap();
        let main = Slug::try_new("main").unwrap();
        let now = SystemClock.now();
        table
            .sign_up(&SignUp::new(email, main, token.hash(), now))
            .await
            .unwrap();
        table.confirm(&token.hash(), now).await.unwrap();
        (dir, config, db, id)
    }

    #[tokio::test]
    async fn a_due_digest_goes_out_once_and_then_waits_its_week() {
        let now = SystemClock.now();
        let (dir, config, db, id) = site(now.date_naive()).await;
        let mailer = MemoryMailer::default();
        let mail: Arc<dyn Mailer> = Arc::new(mailer.clone());
        let digests = Digests::new((dir.path(), &config), (&db, id), Some(mail)).unwrap();
        digests.round(now).await;
        let sent = mailer.sent();
        assert_eq!(sent.len(), 1, "one reader, one copy");
        assert!(
            sent[0].text.contains("Fresh news on crawling"),
            "{}",
            sent[0].text
        );
        assert!(
            sent[0].text.contains("/blog/fresh-news/"),
            "{}",
            sent[0].text
        );
        digests.round(now).await;
        assert_eq!(mailer.sent().len(), 1, "sent this week already");
        db.close().await;
    }

    #[tokio::test]
    async fn a_site_without_a_postal_address_gets_no_digests() {
        let text = SITE.replace(
            "postal_address = \"Field Notes Ltd, 1 Park Row, Leeds\"\n",
            "",
        );
        let env = Environment::from_pairs(&[(Environment::RESEND_API_KEY, "made-for-this-test")]);
        let config = Config::from_toml(Path::new("site.toml"), &text, &env).unwrap();
        let (dir, _, db, id) = site(SystemClock.now().date_naive()).await;
        let mail: Arc<dyn Mailer> = Arc::new(MemoryMailer::default());
        assert!(Digests::new((dir.path(), &config), (&db, id), Some(mail)).is_none());
        db.close().await;
    }
}
