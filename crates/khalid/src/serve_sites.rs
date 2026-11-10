//! `khalid serve` on a sites folder: every site in `sites.toml` behind one
//! listener, routed by host name, and each site a super admin adds started
//! while the process runs (plan 4.4, ADR "many sites in one process").

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use khalid_auth::{Home, SiteDirectory};
use khalid_core::{DomainName, Environment, LogSettings, UserMessage};
use khalid_https::{Domains, Listening};
use khalid_outbound::Outbound;
use khalid_server::FolderLook;
use serde::Serialize;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

use crate::cli::{Cli, Output, ServeArgs};
use crate::exit::Exit;
use crate::serve::{self, GRACE, Keys};
use crate::serve_check::BodyCheck;
use crate::serve_https::{self, Listen};
use crate::site_folders::{NoPanel, SiteFolders};
use crate::site_hosts::{Among, Hosts};
use crate::site_list::{self, SiteList};
use crate::{
    alert_watch, memory, outbound, report, serve_message, server_log, sites_https, sites_message,
};

/// The JSON shape of the line `khalid serve --json` prints once it listens.
#[derive(Serialize)]
struct ServingBody<'a> {
    address: &'a str,
    sites: Vec<&'a str>,
}

/// What every site in the process shares.
struct Shared<'a> {
    root: &'a Path,
    cli: &'a Cli,
    runtime: &'a Runtime,
    env: &'a Environment,
    listen: Listen,
    outbound: Outbound,
    hosts: Hosts,
    home: Arc<dyn SiteDirectory>,
    output: Output,
    args: &'a ServeArgs,
}

/// Serves every site in the sites folder `root` on `--listen`, until
/// Ctrl+C or a stop signal stops them all.
pub(crate) fn run(root: &Path, cli: &Cli, args: &ServeArgs) -> Exit {
    let output = cli.output;
    // Each site's own `[log]` is read later, so the log starts plain at info.
    server_log::install(LogSettings::default(), output);
    if args.domain.is_some() {
        return report::failure(
            output,
            &sites_message::domain_in_sites_folder(),
            Exit::Usage,
        );
    }
    let list = match SiteList::read(root) {
        Ok(Some(list)) => list,
        Ok(None) => {
            return report::failure(
                output,
                &sites_message::not_a_sites_folder(root),
                Exit::Failure,
            );
        }
        Err(message) => return report::failure(output, &message, Exit::Failure),
    };
    let listen = match listen(cli, args, &list) {
        Ok(listen) => listen,
        Err(message) => return report::failure(output, &message, Exit::Failure),
    };
    memory::small_pages_only();
    let built = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_keep_alive(Duration::from_secs(1))
        .build();
    let (Ok(runtime), Ok(outbound)) = (built, outbound::client(cli)) else {
        return report::failure(output, &serve_message::internal(), Exit::Internal);
    };
    let (started, starts) = mpsc::unbounded_channel();
    let shared = Shared {
        root,
        cli,
        runtime: &runtime,
        env: &Environment::from_process(),
        listen,
        outbound,
        hosts: Hosts::default(),
        home: Arc::new(SiteFolders::new(root, Some(started))),
        output,
        args,
    };
    match serve_all(&shared, &list, starts) {
        Ok(()) if output == Output::Human => report::text("Stopped.\n", Exit::Success),
        Ok(()) => Exit::Success,
        Err((message, exit)) => report::failure(output, &message, exit),
    }
}

/// Where the sites answer: plain HTTP on `--listen`, or with `--https`
/// HTTPS on ports 443 and 80, port 80 sending visitors to the home site
/// when their name has no certificate yet.
fn listen(cli: &Cli, args: &ServeArgs, list: &SiteList) -> Result<Listen, UserMessage> {
    let Some(first) = list.domains().first().filter(|_| args.https) else {
        return Ok(Listen::Plain(args.listen));
    };
    let home = sites_https::domain(first).ok_or_else(|| sites_message::no_certificate(first))?;
    serve_https::https_listen(cli, args, &home).map(|https| Listen::Https(Box::new(https)))
}

/// Opens the sites folder's listener, and says where it answers and, over
/// HTTPS, the domains each site adds itself to.
async fn open(
    shared: &Shared<'_>,
) -> Result<(Listening, String, Option<Domains>), (UserMessage, Exit)> {
    match &shared.listen {
        Listen::Https(home) => {
            let (listener, address, domains) = sites_https::open(shared.root, home).await?;
            Ok((listener, address, Some(domains)))
        }
        Listen::Plain(_) => {
            let (listener, address) = serve_https::open(shared.root, &shared.listen).await?;
            Ok((listener, address, None))
        }
    }
}

/// Opens the listener, starts each site on its own thread, then starts
/// each site added through the panel, until a stop signal.
fn serve_all(
    shared: &Shared<'_>,
    list: &SiteList,
    mut starts: mpsc::UnboundedReceiver<DomainName>,
) -> Result<(), (UserMessage, Exit)> {
    let (listener, address, domains) = shared.runtime.block_on(open(shared))?;
    let domains = domains.as_ref();
    announce(shared.output, &address, list.domains());
    let front = khalid_server::serve(listener, shared.hosts.router(), serve::stop(), GRACE);
    let front = shared.runtime.spawn(front);
    std::thread::scope(|scope| {
        let homes = std::iter::once(Home::Yes).chain(std::iter::repeat(Home::No));
        for (domain, home) in list.domains().iter().zip(homes) {
            scope.spawn(move || one_site(shared, (domain, domains), home));
        }
        while let Some(domain) = shared.runtime.block_on(next_start(&mut starts)) {
            if shared.output == Output::Human {
                report::text(
                    &format!("Serving {} too.\n", domain.as_str()),
                    Exit::Success,
                );
            }
            scope.spawn(move || one_site(shared, (&domain, domains), Home::No));
        }
    });
    match shared.runtime.block_on(front) {
        Ok(Ok(_)) => Ok(()),
        _ => Err((serve_message::stopped(), Exit::Failure)),
    }
}

/// The next site to start, or `None` once the process is told to stop.
async fn next_start(starts: &mut mpsc::UnboundedReceiver<DomainName>) -> Option<DomainName> {
    tokio::select! {
        domain = starts.recv() => domain,
        () = serve::stop() => None,
    }
}

/// Checks and serves the site on `domain` until the stop signal, over
/// HTTPS when the sites folder has `domains`. A site that cannot be
/// served is reported, and the others go on.
fn one_site(shared: &Shared<'_>, (domain, domains): (&DomainName, Option<&Domains>), home: Home) {
    let folder = site_list::folder(shared.root, domain);
    let look = FolderLook::take(&folder);
    let checked = crate::check(&folder, shared.env, crate::Bodies::ByTheCommand);
    let (config, routes, content, assets) = match checked {
        Ok(checked) => checked,
        Err(message) => {
            report::failure(shared.output, &message, Exit::Failure);
            return;
        }
    };
    let among = Among {
        hosts: shared.hosts.clone(),
        domain: domain.clone(),
        directory: match home {
            Home::Yes => Arc::clone(&shared.home),
            Home::No => Arc::new(NoPanel),
        },
    };
    let keys = Keys::read(shared.env, (&folder, &config), shared.cli);
    let listen = site_listen(shared, (&folder, domain), domains);
    let to = (
        &listen,
        &shared.outbound,
        (shared.output, alert_watch::every(shared.args, &config)),
        keys,
    );
    let content = Arc::new(content);
    let bodies = BodyCheck::start(&folder, &content, (&config, &routes));
    let start = (content, assets, look, bodies);
    let served = serve::serve_site(
        shared.runtime,
        (&folder, &config, &routes),
        start,
        to,
        Some(&among),
    );
    if let Err((message, _)) = served {
        report::failure(shared.output, &message, Exit::Failure);
    }
}

/// Where the site on `domain` in `folder` answers among many: over HTTPS
/// for its own domain when there are `domains`, whose certificate is then
/// kept from now on, and the sites folder's plain address otherwise.
fn site_listen(
    shared: &Shared<'_>,
    (folder, domain): (&Path, &DomainName),
    domains: Option<&Domains>,
) -> Listen {
    let (Listen::Https(home), Some(domains)) = (&shared.listen, domains) else {
        return shared.listen.clone();
    };
    let Some(name) = sites_https::domain(domain) else {
        report::failure(
            shared.output,
            &sites_message::no_certificate(domain),
            Exit::Failure,
        );
        return Listen::Plain(shared.args.listen);
    };
    let (listen, keeping) = sites_https::site(home, (folder, name), domains);
    shared.runtime.spawn(keeping);
    listen
}

/// Says where the sites answer. In JSON, this one line is the whole output.
fn announce(output: Output, address: &str, domains: &[DomainName]) {
    let sites: Vec<&str> = domains.iter().map(DomainName::as_str).collect();
    match output {
        Output::Human => report::text(
            &format!(
                "Serving {} sites at {address}: {}. Press Ctrl+C to stop.\n",
                sites.len(),
                sites.join(", ")
            ),
            Exit::Success,
        ),
        Output::Json => report::json(&ServingBody { address, sites }, Exit::Success),
    };
}
