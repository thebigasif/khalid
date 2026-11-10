//! HTTPS for many domains behind one listener, as a sites folder serves
//! them: one account, a saved certificate served as it is, and a new one
//! got for a domain that has none, from a small test service that checks
//! the answers for real.

#[cfg(test)]
#[expect(
    dead_code,
    reason = "these tests order with the TLS check alone and open no bare account"
)]
mod acme_service;
#[cfg(test)]
mod common;

use std::time::Duration;

use acme_service::{Offer, start_ordering};
use common::tls_handshake;
use khalid_core::FixedClock;
use khalid_https::{AgreedToTerms, Directory, Domain, HttpsError, HttpsFolder, Issued, ManyHttps};
use khalid_outbound::{Limits, Outbound, Reach, Resolver};
use tokio::net::TcpListener;

/// The home site, whose certificate is saved already.
const HOME: &str = "blog.test";

/// A site added later, with no certificate yet.
const NEWS: &str = "news.test";

/// The outbound client, reaching the test service on loopback.
#[cfg(test)]
fn outbound() -> Outbound {
    let resolver = Resolver::for_tests(Vec::new(), Reach::PublicAndLoopback);
    Outbound::with_resolver(Limits::default(), resolver).unwrap()
}

/// Two loopback listeners, for TLS and port 80, and their ports.
#[cfg(test)]
async fn listeners() -> ((TcpListener, TcpListener), (u16, u16)) {
    let tls = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let plain = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let ports = (
        tls.local_addr().unwrap().port(),
        plain.local_addr().unwrap().port(),
    );
    ((tls, plain), ports)
}

#[cfg(test)]
fn many(directory: &str, account: &HttpsFolder) -> ManyHttps {
    ManyHttps {
        home: Domain::try_new(HOME).unwrap(),
        account: account.clone(),
        directory: Directory::Other(directory.to_owned()),
    }
}

#[tokio::test]
async fn each_domain_gets_its_own_certificate_and_a_saved_one_is_served_as_it_is() {
    let (listeners, ports) = listeners().await;
    let service = start_ordering(NEWS, Offer::TlsAlpn, ports).await;
    let (root, home_site, news_site) = (
        tempfile::tempdir().unwrap(),
        tempfile::tempdir().unwrap(),
        tempfile::tempdir().unwrap(),
    );
    let account = HttpsFolder::new(root.path().to_path_buf());
    let made = rcgen::generate_simple_self_signed(vec![HOME.to_owned()]).unwrap();
    let saved = Issued {
        chain: made.cert.pem(),
        key: made.signing_key.serialize_pem(),
    };
    let home_folder = HttpsFolder::new(home_site.path().to_path_buf());
    home_folder.save_certificate(&saved).await.unwrap();
    let news_folder = HttpsFolder::new(news_site.path().to_path_buf());

    let (listener, domains) = many(&service.directory, &account)
        .start(&outbound(), Some(AgreedToTerms), listeners)
        .await
        .unwrap();
    tokio::spawn(async move { axum::serve(listener, axum::Router::new()).await });
    let clock = FixedClock::new("2026-10-27T00:00:00Z".parse().unwrap());
    let sites = [(HOME, home_folder), (NEWS, news_folder.clone())];
    let keeping: Vec<_> = sites
        .into_iter()
        .map(|(name, folder)| {
            let keep = domains
                .clone()
                .keep(Domain::try_new(name).unwrap(), folder, clock);
            tokio::spawn(keep)
        })
        .collect();
    let mut waited = 0;
    while domains.certificates().get(NEWS).is_none() && waited < 200 {
        tokio::time::sleep(Duration::from_millis(100)).await;
        waited += 1;
    }
    let (home_answer, _) = tls_handshake(ports.0, HOME, b"http/1.1").await.unwrap();
    keeping.iter().for_each(tokio::task::JoinHandle::abort);

    assert_eq!(home_answer, made.cert.der().to_vec(), "the saved one");
    assert!(
        domains
            .certificates()
            .get(NEWS)
            .is_some_and(|c| c.covers(NEWS))
    );
    assert!(news_folder.certificate().await.unwrap().is_some(), "saved");
    assert_eq!(service.issued(), 1, "only the domain with none ordered");
    assert_eq!(service.accounts(), 1, "one account for every domain");
    assert!(account.account().await.unwrap().is_some());
}

#[tokio::test]
async fn no_account_is_opened_for_many_domains_without_the_owners_yes() {
    let (listeners, _) = listeners().await;
    let root = tempfile::tempdir().unwrap();
    let account = HttpsFolder::new(root.path().to_path_buf());

    let started = many("http://127.0.0.1:9/dir", &account)
        .start(&outbound(), None, listeners)
        .await;

    assert!(matches!(started, Err(HttpsError::TermsNotAgreed)));
}
