//! A site's staff accounts as `khalid serve` signs people in: accounts,
//! sessions, passkeys and sign-in links in the site's database (plan
//! section 4.6).

use std::sync::Arc;

use khalid_api::content_store::BoxFuture;
use khalid_auth::{
    Accounts, Email, LINK_LIFETIME, LinkSender, Links, PasskeySite, Passkeys, PasswordHashing,
    SendLinkError,
};
use khalid_core::{Config, Secret, SystemClock};
use khalid_db::{AccountTable, Database, LinkTable, PasskeyTable, SessionTable, SiteId};
use khalid_newsletter::{EmailAddress, Mail, Mailer};

use crate::mounted;

/// The staff accounts of `site` in `db`, with passkeys for the address in
/// `config` when it has a host name. Sign-in links are mailed through
/// `mailer` when the site sends mail; links made by `khalid staff owner`
/// work either way.
pub(crate) fn accounts(
    db: &Database,
    site: SiteId,
    config: &Config,
    mailer: Option<Arc<dyn Mailer>>,
) -> Accounts {
    let store = Arc::new(AccountTable::new(db.clone(), site));
    let sessions = Arc::new(SessionTable::new(db.clone(), site));
    let links = Links {
        store: Arc::new(LinkTable::new(db.clone(), site)),
        sender: mailer.map(|mailer| -> Arc<dyn LinkSender> {
            Arc::new(LinkMail {
                mailer,
                site: config.name().to_owned(),
            })
        }),
        admin: admin_address(config),
    };
    let accounts = Accounts::new(
        store,
        sessions,
        PasswordHashing::new(),
        Arc::new(SystemClock),
    )
    .with_links(links);
    // An address such as http://127.0.0.1/ has no host name for a passkey
    // to belong to; the site then answers that it has no passkeys.
    match PasskeySite::for_address(config.address()) {
        Ok(passkey_site) => {
            let store = Arc::new(PasskeyTable::new(db.clone(), site));
            accounts.with_passkeys(Passkeys::new(store, passkey_site))
        }
        Err(_) => accounts,
    }
}

/// Mails sign-in links through the site's mail sender, in the background,
/// so the answer comes as fast whether or not the email has an account.
struct LinkMail {
    mailer: Arc<dyn Mailer>,
    /// The site's name, for the subject line.
    site: String,
}

impl LinkSender for LinkMail {
    fn send<'a>(
        &'a self,
        to: &'a Email,
        link: Secret<String>,
    ) -> BoxFuture<'a, Result<(), SendLinkError>> {
        let mailer = Arc::clone(&self.mailer);
        let mail = link_mail(&self.site, to, &link);
        tokio::spawn(async move {
            let Some(mail) = mail else {
                tracing::warn!("a sign-in link was not mailed: the account's email takes no mail");
                return;
            };
            // Only the error's own words: a transport's cause may name the
            // address, which never goes to the log (BL-RUN-5).
            if let Err(error) = mailer.send(&mail).await {
                tracing::warn!(%error, "a sign-in link could not be mailed");
            }
        });
        Box::pin(async { Ok(()) })
    }
}

/// The email that carries `link` to `to` for the site named `site`, or
/// `None` when `to` is not an address mail can go to.
fn link_mail(site: &str, to: &Email, link: &Secret<String>) -> Option<Mail> {
    let to = EmailAddress::parse(to.as_str()).ok()?;
    let minutes = LINK_LIFETIME.num_minutes();
    let subject = format!("Your sign-in link for {site}");
    let text = format!(
        "Open this link to sign in to the admin of {site}:\n\n{}\n\n\
         It works once, for {minutes} minutes. If you did not ask for it, \
         ignore this email: nobody can sign in without the link.\n",
        link.expose()
    );
    Some(Mail::new(to, subject, text))
}

/// Where the admin is served: its folder, `admin` unless `site.toml` sets
/// `admin_folder`, under the blog's address with its mount, such as
/// `https://example.com/blog/admin` (plan section 4.6).
pub(crate) fn admin_address(config: &Config) -> String {
    format!("{}{}", mounted::blog_address(config), config.admin_folder())
}

/// The admin's path from the host's root, with no slash at the end, such
/// as `/blog/admin` or `/studio`.
pub(crate) fn admin_path(config: &Config) -> String {
    format!("{}{}", config.mount(), config.admin_folder())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_link_mail_names_the_site_and_how_long_the_link_works() {
        let to: Email = "mira@example.com".parse().unwrap();
        let link = Secret::new("https://example.com/blog/admin/sign-in/link#token=t".to_owned());
        let mail = link_mail("Field Notes", &to, &link).unwrap();
        assert_eq!(mail.to.to_string(), "mira@example.com");
        assert_eq!(mail.subject, "Your sign-in link for Field Notes");
        assert_eq!(
            mail.text,
            "Open this link to sign in to the admin of Field Notes:\n\n\
             https://example.com/blog/admin/sign-in/link#token=t\n\n\
             It works once, for 15 minutes. If you did not ask for it, \
             ignore this email: nobody can sign in without the link.\n"
        );
    }
}
