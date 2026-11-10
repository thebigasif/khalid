//! The certificates the server answers with, shared between the listener
//! and whatever renews them.

use std::collections::BTreeMap;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard};

use rustls::server::{ClientHello, ResolvesServerCert};
use rustls::sign::CertifiedKey;

use crate::challenges::ACME_TLS;
use crate::{Certificate, Challenges};

/// The certificates the TLS listener answers with, one for each domain it
/// serves. A new one, such as a renewal, takes effect from the next
/// connection, without a restart.
///
/// A visitor gets the certificate for the name their browser asks for. A
/// connection that names none, or a name no certificate covers, gets the
/// first certificate set, so a server with one domain answers as before.
///
/// While Let's Encrypt checks a domain over TLS-ALPN-01, its check gets
/// the answer from [`Certificates::challenges`] instead, and every other
/// visitor still gets the site's certificate.
///
/// It is cheap to clone, and every clone sees the same certificates.
#[derive(Debug, Clone, Default)]
pub struct Certificates {
    served: Arc<RwLock<Served>>,
    challenges: Challenges,
}

/// The certificates by the lower-case names they cover, and the name of
/// the first one set.
#[derive(Debug, Default)]
struct Served {
    by_name: BTreeMap<String, Certificate>,
    first: Option<String>,
}

impl Certificates {
    /// No certificate yet. Connections fail their handshake until one is
    /// set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Answers every name `certificate` covers with it from now on, in
    /// place of the one before.
    pub fn set(&self, certificate: Certificate) {
        // A certificate that names no domain is kept under no name, so it
        // still answers connections that name none.
        let names = match certificate.names() {
            [] => vec![String::new()],
            names => names.to_vec(),
        };
        let mut served = self.served.write().unwrap_or_else(PoisonError::into_inner);
        if served.first.is_none() {
            served.first = names.first().cloned();
        }
        if let Some((last, others)) = names.split_last() {
            for name in others {
                served.by_name.insert(name.clone(), certificate.clone());
            }
            served.by_name.insert(last.clone(), certificate);
        }
    }

    /// The certificate a connection that names no domain is answered
    /// with: the first one set, if there is one.
    pub fn current(&self) -> Option<Certificate> {
        let served = self.read();
        let first = served.first.as_ref()?;
        served.by_name.get(first).cloned()
    }

    /// The certificate for `domain`, such as `blog.example.com`, if one
    /// covers it.
    pub fn get(&self, domain: &str) -> Option<Certificate> {
        self.read()
            .by_name
            .get(&domain.to_ascii_lowercase())
            .cloned()
    }

    /// The answers to the checks Let's Encrypt is making, shared with the
    /// port 80 router.
    pub fn challenges(&self) -> &Challenges {
        &self.challenges
    }

    fn read(&self) -> RwLockReadGuard<'_, Served> {
        self.served.read().unwrap_or_else(PoisonError::into_inner)
    }
}

impl ResolvesServerCert for Certificates {
    fn resolve(&self, hello: ClientHello<'_>) -> Option<Arc<CertifiedKey>> {
        let is_check = hello
            .alpn()
            .is_some_and(|mut offered| offered.any(|protocol| protocol == ACME_TLS));
        if is_check {
            return hello
                .server_name()
                .and_then(|name| self.challenges.tls_answer(name));
        }
        hello
            .server_name()
            .and_then(|name| self.get(name))
            .or_else(|| self.current())
            .map(|certificate| certificate.certified_key())
    }
}
