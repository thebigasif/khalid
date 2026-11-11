//! What `khalid build` sends `IndexNow`: each live post that is new or has
//! a new `updated:` date since the last build, as the record in `data/`
//! has it, when the site has a key and a public address (task C8, ADR
//! "`IndexNow` without sitemap pings"). A static site has no server to send
//! them, so the build does.

use std::collections::BTreeMap;
use std::net::IpAddr;
use std::path::{Path, PathBuf};

use khalid_content::SiteContent;
use khalid_core::{AbsoluteUrl, Config, IndexNowSettings, UserMessage};
use khalid_seo::{IndexNow, Retries};

use crate::announce::indexnow_record::{FIRST_RUN, Record};
use crate::cli::Cli;
use crate::{build_message, build_seo, index, outbound, site_addresses};

/// What the build tells about `IndexNow`: a line for people, and how many
/// addresses were sent, when any request went out.
#[derive(Debug)]
pub(crate) struct Told {
    pub(crate) line: String,
    pub(crate) sent: Option<usize>,
}

impl Told {
    fn line(line: &str) -> Self {
        let line = line.to_owned();
        Self { line, sent: None }
    }
}

/// Sends `IndexNow` the news in the published `content` of the `site`
/// folder, without the posts in `left_out`, which the gate kept out. `None`
/// when the site has no `IndexNow` key; nothing is sent when its address
/// is not public. The record is kept only once the news is sent, so a
/// failed request is sent again at the next build.
pub(crate) fn tell(
    (site, config): (&Path, &Config),
    content: &SiteContent,
    left_out: &[PathBuf],
    cli: &Cli,
) -> Option<Told> {
    let settings = config.seo().indexnow.as_ref()?;
    let address = config.address();
    if !public(address) {
        return Some(Told::line(&format!(
            "IndexNow: nothing sent, because the site's address {address} is not public, so search engines cannot read pages there. Set address in site.toml to the public address to send them."
        )));
    }
    let now = match record(site, config, content, left_out) {
        Ok(now) => now,
        Err(message) => return Some(Told::line(&format!("IndexNow: {}", message.what()))),
    };
    let Some(before) = Record::read(site) else {
        return Some(keep(site, &now, Told::line(FIRST_RUN)));
    };
    let base = config.address().as_str().trim_end_matches('/');
    let urls: Vec<AbsoluteUrl> = now
        .news_since(&before)
        .into_iter()
        .filter_map(|path| AbsoluteUrl::parse(&format!("{base}{path}")).ok())
        .collect();
    if urls.is_empty() {
        let none = "Sent 0 addresses to IndexNow: no post is new or has a new updated: date.";
        return Some(keep(site, &now, Told::line(none)));
    }
    Some(match send(config, settings, &urls, cli) {
        Ok(sent) => {
            let noun = if sent == 1 { "address" } else { "addresses" };
            let line = format!(
                "Sent {sent} {noun} to IndexNow, for the posts that are new or have a new updated: date."
            );
            keep(
                site,
                &now,
                Told {
                    sent: Some(sent),
                    ..Told::line(&line)
                },
            )
        }
        Err(message) => Told::line(&format!(
            "IndexNow: {} {} The build is fine, and the next build sends these addresses again.",
            message.what(),
            message.why()
        )),
    })
}

/// Keeps `now` as what `IndexNow` heard, and says `told`, with a note when
/// the record could not be written.
fn keep(site: &Path, now: &Record, mut told: Told) -> Told {
    if let Err(error) = now.write(site) {
        told.line = format!(
            "{}\nIndexNow: the list in data/indexnow-sent.txt could not be written ({error}), so the next build may send these addresses again.",
            told.line
        );
    }
    told
}

/// The record of the live posts in `content`, without those `left_out`, at
/// the addresses `khalid serve` gives them.
fn record(
    site: &Path,
    config: &Config,
    content: &SiteContent,
    left_out: &[PathBuf],
) -> Result<Record, UserMessage> {
    let mut published = content.clone();
    published.posts.retain(|doc| !left_out.contains(&doc.path));
    let index = index::of_content(site, config, &published)?;
    let addresses = site_addresses::of(config, &index);
    let updated: BTreeMap<_, _> = published
        .posts
        .iter()
        .filter_map(|doc| Some((index::slashed(&doc.path)?, doc.front.last_updated()?)))
        .collect();
    Ok(Record::of(&addresses, &updated))
}

/// Sends `urls` to `IndexNow` with `settings`, through the one outbound
/// client, and says how many were sent.
fn send(
    config: &Config,
    settings: &IndexNowSettings,
    urls: &[AbsoluteUrl],
    cli: &Cli,
) -> Result<usize, UserMessage> {
    let indexnow = IndexNow::new(&build_seo::site(config)?, settings)
        .map_err(|error| build_message::seo(&error))?;
    let outbound = outbound::client(cli).map_err(|error| build_message::indexnow(&error))?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| build_message::indexnow(&error))?;
    let retries = Retries::default();
    let sent = runtime.block_on(indexnow.send(&outbound, &retries, urls));
    sent.map(|sent| sent.urls)
        .map_err(|error| build_message::indexnow(&error))
}

/// Whether search engines can reach `address`: not `localhost`, a private
/// or loopback address, or a name kept for local networks and tests.
fn public(address: &AbsoluteUrl) -> bool {
    let host = address.host().trim_start_matches('[').trim_end_matches(']');
    if let Ok(ip) = host.parse::<IpAddr>() {
        return khalid_outbound::is_public(ip);
    }
    let local = [".localhost", ".local", ".internal", ".test", ".lan"];
    host != "localhost" && !local.iter().any(|end| host.ends_with(end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_an_address_search_engines_can_reach_is_public() {
        let public = |url: &str| public(&AbsoluteUrl::parse(url).unwrap());
        assert!(public("https://www.example.com/blog/"));
        assert!(public("https://203.0.114.9/"));
        assert!(!public("http://localhost:8080/"));
        assert!(!public("http://127.0.0.1:8080/"));
        assert!(!public("http://192.168.1.4/"));
        assert!(!public("http://[::1]:8080/"));
        assert!(!public("https://bakery.test/"));
        assert!(!public("https://blog.local/"));
    }
}
