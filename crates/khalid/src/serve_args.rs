//! What `khalid serve` takes on the command line.

use std::net::SocketAddr;

use clap::{ArgGroup, Args};
use khalid_https::Domain;

/// What `khalid serve` takes: plain HTTP on an address, HTTPS for a
/// domain with its own certificate, or HTTPS for every site of a sites
/// folder. The flags that tune HTTPS need `--domain` or `--https`.
#[derive(Debug, Args)]
#[command(group(ArgGroup::new("tls").args(["domain", "https"])))]
pub(crate) struct ServeArgs {
    /// The address and port to answer on over plain HTTP
    #[arg(long, value_name = "ADDRESS", default_value = "127.0.0.1:8080")]
    pub(crate) listen: SocketAddr,
    /// Make the site first when the folder has none, at the address in `KHALID_ADDRESS`
    #[arg(long)]
    pub(crate) init: bool,
    /// Answer over HTTPS for this domain on ports 443 and 80, with a free
    /// certificate from Let's Encrypt that Khalid gets and renews
    #[arg(long, value_name = "DOMAIN", conflicts_with = "listen")]
    pub(crate) domain: Option<Domain>,
    /// In a sites folder, answer every site over HTTPS on ports 443 and
    /// 80, each with its own free certificate from Let's Encrypt
    #[arg(long, conflicts_with = "listen")]
    pub(crate) https: bool,
    /// Agree to Let's Encrypt's subscriber agreement unasked, as in a service or a container
    #[arg(long, requires = "tls")]
    pub(crate) accept_terms: bool,
    /// The address for HTTPS instead of port 443 on every address
    #[arg(long, value_name = "ADDRESS", requires = "tls", hide = true)]
    pub(crate) https_listen: Option<SocketAddr>,
    /// The address for plain HTTP instead of port 80 on every address
    #[arg(long, value_name = "ADDRESS", requires = "tls", hide = true)]
    pub(crate) http_listen: Option<SocketAddr>,
    /// Another ACME directory than Let's Encrypt's, such as its staging one
    #[arg(long, value_name = "URL", requires = "tls", hide = true)]
    pub(crate) acme_directory: Option<String>,
    /// Test builds only: the hidden timings.
    #[cfg(feature = "test-util")]
    #[command(flatten)]
    pub(crate) test: crate::serve_https::ServeTestArgs,
}

impl ServeArgs {
    /// Whether the server answers over HTTPS, for one domain or many.
    pub(crate) fn serves_https(&self) -> bool {
        self.domain.is_some() || self.https
    }
}
