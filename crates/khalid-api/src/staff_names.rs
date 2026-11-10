//! The names the admin shows for whoever changed something: a staff
//! account by its name, a key by the name it was made with (backlog row
//! 12, briefs G50 and G52, KH-EDT-2 and KH-EDT-6).
//!
//! Records keep the identity, which never changes. The name is looked up
//! each time it is shown, so a renamed account shows its new name in
//! presence, versions, stale-edit messages and the audit log alike.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use crate::keys::KeyStore;
use crate::session_check::SessionCheck;

/// A name for each identity a key or a staff account carries.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StaffNames(BTreeMap<String, String>);

impl StaffNames {
    /// The names in `named`, each by its identity. A later name for the
    /// same identity wins; a blank name is left out.
    pub fn new(named: impl IntoIterator<Item = (String, String)>) -> Self {
        let named = named
            .into_iter()
            .filter(|(_, name)| !name.trim().is_empty());
        Self(named.collect())
    }

    /// The name of `id`, or `id` itself when no key or account has it,
    /// such as a revoked key or a deleted account.
    pub fn of<'a>(&'a self, id: &'a str) -> &'a str {
        self.0.get(id).map_or(id, String::as_str)
    }
}

/// Where the names come from: the stored keys, then the staff accounts
/// when the site has them, whose names win. The Admin API door hands one
/// to every route, and only the routes that show a name read it.
#[derive(Clone)]
pub(crate) struct NameBook {
    keys: Arc<dyn KeyStore>,
    sessions: Option<Arc<dyn SessionCheck>>,
}

impl NameBook {
    /// The book over `keys` and, if the site has accounts, `sessions`.
    pub(crate) fn new(keys: Arc<dyn KeyStore>, sessions: Option<Arc<dyn SessionCheck>>) -> Self {
        Self { keys, sessions }
    }

    /// Every name known now. A store that cannot be read gives no names,
    /// so the identities show instead and the request still succeeds.
    pub(crate) async fn names(&self) -> StaffNames {
        let keys = match self.keys.list().await {
            Ok(keys) => keys,
            Err(error) => {
                let error: &(dyn std::error::Error + 'static) = &error;
                tracing::warn!(error, "the key names could not be read");
                Vec::new()
            }
        };
        let keys = keys
            .into_iter()
            .map(|key| (key.id.as_str().to_owned(), key.name));
        let accounts = match &self.sessions {
            Some(sessions) => sessions.names().await.unwrap_or_else(|error| {
                let error: &(dyn std::error::Error + 'static) = &error;
                tracing::warn!(error, "the staff names could not be read");
                Vec::new()
            }),
            None => Vec::new(),
        };
        let accounts = accounts
            .into_iter()
            .map(|(id, name)| (id.as_str().to_owned(), name));
        StaffNames::new(keys.chain(accounts))
    }
}

impl fmt::Debug for NameBook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NameBook").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_known_identity_shows_its_name_and_an_unknown_one_itself() {
        let names = StaffNames::new([
            ("1111222233334444".to_owned(), "SerpDad".to_owned()),
            ("0000000000000001".to_owned(), "Maria Lopez".to_owned()),
            ("0000000000000002".to_owned(), "  ".to_owned()),
        ]);
        assert_eq!(names.of("0000000000000001"), "Maria Lopez");
        assert_eq!(names.of("1111222233334444"), "SerpDad");
        assert_eq!(names.of("0000000000000002"), "0000000000000002");
        assert_eq!(names.of("9999aaaabbbbcccc"), "9999aaaabbbbcccc");
    }

    #[test]
    fn a_later_name_for_the_same_identity_wins() {
        let names = StaffNames::new([
            ("0000000000000001".to_owned(), "a key".to_owned()),
            ("0000000000000001".to_owned(), "Maria Lopez".to_owned()),
        ]);
        assert_eq!(names.of("0000000000000001"), "Maria Lopez");
    }
}
