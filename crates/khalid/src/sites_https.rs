//! `khalid serve --https` on a sites folder: one listener on ports 443 and
//! 80 for every site, each site with its own certificate from Let's
//! Encrypt, kept in its own `data/https/`, and one account for them all in
//! the sites folder's `data/https/` (ADR "HTTPS for a sites folder").

use std::path::Path;

use khalid_core::{DomainName, UserMessage};
use khalid_https::{Domain, Domains, HttpsFolder, Listening, ManyHttps};

use crate::exit::Exit;
use crate::serve_https::{self, HttpsListen, Listen};
use crate::{https_message, serve_message};

/// Opens ports 443 and 80 for every site of the sites folder `root`,
/// asking for the owner's yes when no account is saved. Returns the
/// listener, the address visitors use for the home site, and the domains
/// each site adds itself to.
pub(crate) async fn open(
    root: &Path,
    home: &HttpsListen,
) -> Result<(Listening, String, Domains), (UserMessage, Exit)> {
    let many = ManyHttps {
        home: home.domain.clone(),
        account: HttpsFolder::new(serve_https::folder(root)),
        directory: home.directory.clone(),
    };
    let failed = |error| (https_message::start(&home.domain, &error), Exit::Failure);
    let agreed = if many.has_account().await.map_err(failed)? {
        None
    } else {
        Some(home.agree().await?)
    };
    let (listener, domains) = many
        .start(&home.outbound, agreed, home.bind().await?)
        .await
        .map_err(failed)?;
    let listening = Listening::Tls(listener);
    let address = listening
        .address(Some(&home.domain))
        .map_err(|_| (serve_message::internal(), Exit::Internal))?;
    Ok((listening, address, domains))
}

/// The certificate's domain for the site on `domain`, or `None` when
/// Let's Encrypt gives none for it, such as a name with no dot.
pub(crate) fn domain(domain: &DomainName) -> Option<Domain> {
    Domain::try_new(domain.as_str()).ok()
}

/// The site in `folder` among many over HTTPS for its own `domain`: its
/// settings, which the certificate alert reads, and the work that keeps
/// its certificate, kept in its own `data/https/`, on `domains`.
pub(crate) fn site(
    home: &HttpsListen,
    (folder, domain): (&Path, Domain),
    domains: &Domains,
) -> (Listen, impl Future<Output = ()> + Send + use<>) {
    let https = HttpsFolder::new(serve_https::folder(folder));
    let keeping = domains.clone().keep(domain.clone(), https, home.clock);
    (Listen::Https(Box::new(home.for_domain(domain))), keeping)
}
