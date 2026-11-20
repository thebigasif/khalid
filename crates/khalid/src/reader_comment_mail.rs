//! The email each new reader's comment sends to the site's owners and
//! admins, so a comment never waits unseen for approval.

use std::sync::Arc;

use khalid_api::keys::Role;
use khalid_auth::{Account, AccountStore};
use khalid_comments::{Comment, CommentAuthor};
use khalid_core::Config;
use khalid_db::{AccountTable, Database, SiteId};
use khalid_newsletter::{EmailAddress, Mail, Mailer};

use crate::staff;

/// Emails the owners and admins of a site about each new reader comment.
#[derive(Clone)]
pub(crate) struct ReaderCommentMail {
    accounts: AccountTable,
    mailer: Arc<dyn Mailer>,
    /// Where the admin's Comments screen is.
    screen: String,
}

impl std::fmt::Debug for ReaderCommentMail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReaderCommentMail")
            .field("screen", &self.screen)
            .finish_non_exhaustive()
    }
}

impl ReaderCommentMail {
    /// The mail of site `id`, sent through `mailer`, or `None` when the
    /// site sends no mail or its `[comments]` says `notify = false`.
    pub(crate) fn of(
        (database, id, config): (&Database, SiteId, &Config),
        mailer: Option<Arc<dyn Mailer>>,
    ) -> Option<Self> {
        let mailer = mailer.filter(|_| config.comments().notify)?;
        Some(Self {
            accounts: AccountTable::new(database.clone(), id),
            mailer,
            screen: format!("{}/comments", staff::admin_address(config)),
        })
    }

    /// Emails, in the background, about `comment` on the page at `page`, so
    /// the reader never waits on the mail server. A staff reply sends
    /// nothing, and the reader's email never goes into the mail.
    pub(crate) fn send(&self, comment: &Comment, page: &str) {
        let CommentAuthor::Reader { name, .. } = &comment.author else {
            return;
        };
        let Ok(runtime) = tokio::runtime::Handle::try_current() else {
            return;
        };
        let (accounts, mailer) = (self.accounts.clone(), Arc::clone(&self.mailer));
        let said = (name.as_str().to_owned(), comment.text.as_str().to_owned());
        let at = (page.to_owned(), self.screen.clone());
        runtime.spawn(async move {
            let all = match accounts.list().await {
                Ok(all) => all,
                Err(error) => {
                    return tracing::warn!(
                        %error,
                        "the staff accounts could not be read to email a comment"
                    );
                }
            };
            for mail in mails(&all, (&said.0, &said.1), (&at.0, &at.1)) {
                // Only the error's own words: a transport's cause may name
                // the address, which never goes to the log (BL-RUN-5).
                if let Err(error) = mailer.send(&mail).await {
                    tracing::warn!(%error, "a reader's comment could not be emailed");
                }
            }
        });
    }
}

/// The emails about a comment by `name` saying `text` on the page at
/// `page`: one to each owner and admin, with where to approve it.
fn mails(
    accounts: &[Account],
    (name, text): (&str, &str),
    (page, screen): (&str, &str),
) -> Vec<Mail> {
    let subject = format!("New comment from {name}, waiting for your approval");
    let body = format!(
        "{name} commented on {page}:\n\n{text}\n\n\
         It shows under the post once you approve it. Approve it, or mark it as spam, on the Comments screen: {screen}\n"
    );
    accounts
        .iter()
        .filter(|account| matches!(account.role(), Role::Owner | Role::Admin))
        .filter_map(|account| EmailAddress::parse(account.email().as_str()).ok())
        .map(|to| Mail::new(to, subject.clone(), body.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use khalid_auth::AccountId;

    use super::*;

    fn account(row: i64, email: &str, role: Role) -> Account {
        let name = email.split('@').next().unwrap_or_default().to_owned();
        Account::new(AccountId::from_row(row), email.parse().unwrap(), name, role)
    }

    #[test]
    fn owners_and_admins_hear_of_a_reader_comment() {
        let staff = [
            account(1, "owner@example.com", Role::Owner),
            account(2, "ada@example.com", Role::Admin),
            account(3, "mira@example.com", Role::Editor),
        ];
        let page = "https://example.com/blog/rye/";
        let screen = "https://example.com/blog/admin/comments";
        let sent = mails(&staff, ("Lena", "Very clear, thanks."), (page, screen));
        let to: Vec<String> = sent.iter().map(|mail| mail.to.to_string()).collect();
        assert_eq!(to, ["owner@example.com", "ada@example.com"]);
        assert_eq!(
            sent[0].subject,
            "New comment from Lena, waiting for your approval"
        );
        assert_eq!(
            sent[0].text,
            "Lena commented on https://example.com/blog/rye/:\n\nVery clear, thanks.\n\n\
             It shows under the post once you approve it. Approve it, or mark it as spam, \
             on the Comments screen: https://example.com/blog/admin/comments\n"
        );
    }
}
