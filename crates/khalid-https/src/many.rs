//! HTTPS for many domains behind one listener, as `khalid serve --https`
//! asks for a sites folder: one account, and a certificate for each
//! domain, got and renewed on its own.

use std::sync::Arc;

use khalid_core::Clock;
use khalid_outbound::Outbound;
use tokio::net::TcpListener;
use tokio::sync::OnceCell;

use crate::start::{renewal, signed_in};
use crate::{
    Acme, AgreedToTerms, Certificates, Directory, Domain, HttpsError, HttpsFolder, TlsListener,
    port_80_among,
};

/// HTTPS for the sites of a sites folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManyHttps {
    /// The home site's domain, where port 80 sends a visitor whose name
    /// has no certificate yet.
    pub home: Domain,
    /// Where the one account key is kept.
    pub account: HttpsFolder,
    /// Where certificates come from.
    pub directory: Directory,
}

impl ManyHttps {
    /// Whether an account is saved, which means its owner agreed to the
    /// certificate authority's terms before.
    ///
    /// # Errors
    ///
    /// Returns the errors of [`HttpsFolder::account`].
    pub async fn has_account(&self) -> Result<bool, HttpsError> {
        Ok(self.account.account().await?.is_some())
    }

    /// Starts HTTPS with no certificate yet: port 80 on `plain` answers
    /// Let's Encrypt's file check and moves every visitor to HTTPS, and the
    /// listener returned, on `tls`, answers each domain
    /// [`Domains::keep`] is given. `agreed` is the owner's yes to the
    /// terms, needed only when no account is saved; the account is opened
    /// or signed in to once, in the background, so saved certificates are
    /// served while Let's Encrypt cannot be reached.
    ///
    /// # Errors
    ///
    /// Returns [`HttpsError::TermsNotAgreed`] when a new account is needed
    /// and `agreed` is `None`, the errors of [`HttpsFolder::account`], and
    /// [`HttpsError::Tls`] when TLS cannot start.
    pub async fn start(
        &self,
        outbound: &Outbound,
        agreed: Option<AgreedToTerms>,
        (tls, plain): (TcpListener, TcpListener),
    ) -> Result<(TlsListener, Domains), HttpsError> {
        if agreed.is_none() && !self.has_account().await? {
            return Err(HttpsError::TermsNotAgreed);
        }
        let certificates = Certificates::new();
        let listener = TlsListener::new(tls, certificates.clone())?;
        let router = port_80_among(&self.home, certificates.clone());
        tokio::spawn(async move {
            if let Err(error) = axum::serve(plain, router).await {
                tracing::warn!(%error, "port 80 stopped answering");
            }
        });
        let domains = Domains {
            outbound: outbound.clone(),
            agreed,
            account: self.account.clone(),
            directory: self.directory.clone(),
            acme: Arc::default(),
            certificates,
        };
        Ok((listener, domains))
    }
}

/// The domains a [`ManyHttps`] listener answers, to which each site adds
/// its own as it starts. It is cheap to clone, and clones share the one
/// account.
#[derive(Debug, Clone)]
pub struct Domains {
    outbound: Outbound,
    agreed: Option<AgreedToTerms>,
    account: HttpsFolder,
    directory: Directory,
    acme: Arc<OnceCell<Acme>>,
    certificates: Certificates,
}

impl Domains {
    /// Answers `domain` with the certificate saved in `folder`, gets one
    /// when there is none or it falls due, and keeps it renewed, reading
    /// the time from `clock`, until the task running it is dropped.
    ///
    /// A domain whose certificate cannot be got, such as one whose DNS
    /// does not lead here yet, is logged and tried again an hour later,
    /// and the other domains go on.
    pub async fn keep(self, domain: Domain, folder: HttpsFolder, clock: impl Clock) {
        match folder.certificate().await {
            Ok(Some(saved)) if saved.covers(domain.as_str()) => self.certificates.set(saved),
            Ok(_) => {}
            Err(error) => {
                tracing::warn!(%domain, %error, "the saved HTTPS certificate could not be read, so Khalid gets a new one");
            }
        }
        let at = (&self.account, &self.directory);
        let signing_in = signed_in(at, &self.outbound, self.agreed, &domain);
        let acme = self.acme.get_or_init(|| signing_in).await.clone();
        renewal(acme, &domain, &self.certificates, &folder)
            .keep_renewed(clock)
            .await;
    }

    /// The certificates the listener answers with.
    pub fn certificates(&self) -> &Certificates {
        &self.certificates
    }
}
