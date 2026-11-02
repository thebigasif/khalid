//! The author profile a staff account writes as: found or made when the
//! account is, so a new person has a byline and a page from the start.

use std::sync::Arc;

use khalid_api::content_store::BoxFuture;

use super::Accounts;
use crate::{Account, AccountStore};

/// Finds or makes the author profile a person writes as. `khalid serve`
/// and `khalid staff owner` keep profiles as author files through the
/// Admin API's own author store.
pub trait AuthorProfiles: Send + Sync {
    /// The identity, such as `en.asif`, of the profile for a person called
    /// `name`: the one whose slug is made from the name, made with that
    /// name when none has the slug. `None` when no profile could be found
    /// or made; the account then stays unlinked, and the reason is logged
    /// by the implementation.
    fn profile_for<'a>(&'a self, name: &'a str) -> BoxFuture<'a, Option<String>>;
}

impl Accounts {
    /// These accounts, each new one linked to an author profile from
    /// `profiles`.
    #[must_use]
    pub fn with_author_profiles(mut self, profiles: Arc<dyn AuthorProfiles>) -> Self {
        self.profiles = Some(profiles);
        self
    }

    /// `account`, linked to the author profile named after it, made when
    /// missing. An account already linked, a site with no profiles, or a
    /// profile that could not be made or saved gives `account` back as it
    /// was: signing in never depends on a byline.
    pub async fn link_author(&self, account: Account) -> Account {
        let Some(profiles) = self
            .profiles
            .as_ref()
            .filter(|_| account.author().is_none())
        else {
            return account;
        };
        link_to_profile(self.store.as_ref(), profiles.as_ref(), account).await
    }
}

/// `account` linked in `store` to its profile from `profiles`, as
/// [`Accounts::link_author`] links it, for a caller with only a store, such
/// as `khalid staff owner`.
pub async fn link_to_profile(
    store: &dyn AccountStore,
    profiles: &dyn AuthorProfiles,
    account: Account,
) -> Account {
    let Some(author) = profiles.profile_for(account.name()).await else {
        return account;
    };
    match store.set_author(account.id(), &author).await {
        Ok(true) => account.with_author(Some(author)),
        Ok(false) | Err(_) => account,
    }
}
