//! Where `khalid serve` last answered over plain HTTP for a site, kept in
//! `data/serve-address`, so `khalid staff owner` names the same port in
//! its link for this computer without being told.

use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

/// The file in `data/` that holds the address, such as `127.0.0.1:8080`.
const FILE: &str = "serve-address";

/// `data/serve-address` in the `site` folder.
fn file(site: &Path) -> PathBuf {
    let mut path = site.to_path_buf();
    path.push("data");
    path.push(FILE);
    path
}

/// Keeps `address` as where `site` answers now. A failed write only costs
/// the owner link its guess, so it is logged and serving goes on. A plain
/// write, not a flushed one, keeps the start fast: a torn file reads as no
/// address, and the link falls back to the usual port.
pub(crate) fn record(site: &Path, address: SocketAddr) {
    let path = file(site);
    let written = path
        .parent()
        .map_or(Ok(()), fs::create_dir_all)
        .and_then(|()| fs::write(&path, format!("{address}\n")));
    if let Err(error) = written {
        tracing::warn!(path = %path.display(), %error, "could not keep where serve answers");
    }
}

/// Where `khalid serve` last answered for `site`, or `None` when it has
/// not run there or the file does not hold an address.
pub(crate) fn last(site: &Path) -> Option<SocketAddr> {
    fs::read_to_string(file(site)).ok()?.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_address_kept_is_the_one_read_back() {
        let site = tempfile::tempdir().unwrap();
        assert_eq!(last(site.path()), None, "serve never ran here");
        let address: SocketAddr = "127.0.0.1:18291".parse().unwrap();
        record(site.path(), address);
        assert_eq!(last(site.path()), Some(address));
    }
}
