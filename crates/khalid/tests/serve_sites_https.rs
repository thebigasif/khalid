//! `khalid serve --https` on a sites folder (backlog row 34, plan 4.4 and
//! 4.7): one process answers each site over HTTPS with its own saved
//! certificate, port 80 moves each visitor to their own site, and
//! `--domain`, which is for one site, is refused with the reason.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::Duration;

use khalid_core::AbsoluteUrl;
use khalid_outbound::{Limits, Outbound, Reach, Resolver};
use serde_json::Value;

/// The home site and a second one.
const SITES: [&str; 2] = ["example.com", "news.example.com"];

/// A running `khalid serve`, stopped when dropped.
struct Server {
    child: Child,
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[cfg(test)]
fn khalid(site: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_khalid"));
    command.arg("--site").arg(site);
    command
}

#[cfg(test)]
fn run(site: &Path, args: &[&str]) -> Output {
    khalid(site)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .unwrap()
}

#[cfg(test)]
fn site_folder(root: &Path, domain: &str) -> PathBuf {
    let mut path = root.to_path_buf();
    path.push("sites");
    path.push(domain);
    path
}

/// A sites folder with both sites, each with a saved self-signed
/// certificate for its domain in its own `data/https`, and those
/// certificates as PEM.
#[cfg(test)]
fn sites_with_certificates() -> (tempfile::TempDir, Vec<String>) {
    let root = tempfile::tempdir().unwrap();
    let mut roots = Vec::new();
    for domain in SITES {
        let added = run(root.path(), &["sites", "add", domain]);
        assert_eq!(added.status.code(), Some(0), "{added:?}");
        let made = rcgen::generate_simple_self_signed(vec![domain.to_owned()]).unwrap();
        let mut path = site_folder(root.path(), domain);
        path.push("data/https");
        fs::create_dir_all(&path).unwrap();
        path.push("certificate.pem");
        fs::write(&path, made.cert.pem()).unwrap();
        path.set_file_name("key.pem");
        fs::write(&path, made.signing_key.serialize_pem()).unwrap();
        roots.push(made.cert.pem());
    }
    (root, roots)
}

/// A port nothing listens on right now.
#[cfg(test)]
fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

/// The answer port 80 gives to a visitor asking for `host`.
#[cfg(test)]
fn on_port_80(port: u16, host: &str) -> String {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    let request = format!("GET /?ref=mail HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).unwrap();
    let mut answer = String::new();
    stream.read_to_string(&mut answer).unwrap();
    answer
}

#[test]
fn each_site_is_served_over_https_with_its_own_certificate() {
    let (root, roots) = sites_with_certificates();
    let ports = (free_port(), free_port());
    let mut child = khalid(root.path())
        .args(["serve", "--https", "--accept-terms", "--json"])
        .args(["--https-listen", &format!("127.0.0.1:{}", ports.0)])
        .args(["--http-listen", &format!("127.0.0.1:{}", ports.1)])
        .args(["--acme-directory", "http://127.0.0.1:9/dir"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut line = String::new();
    BufReader::new(child.stdout.take().unwrap())
        .read_line(&mut line)
        .unwrap();
    let _server = Server { child };
    let json: Value = serde_json::from_str(&line).unwrap_or_else(|_| panic!("{line}"));
    assert_eq!(json["address"], format!("https://example.com:{}", ports.0));

    let runtime = tokio::runtime::Runtime::new().unwrap();
    for (domain, root) in SITES.iter().zip(&roots) {
        let hosts = vec![((*domain).to_owned(), vec!["127.0.0.1".parse().unwrap()])];
        let resolver = Resolver::for_tests(hosts, Reach::PublicAndLoopback);
        // Trusting this site's certificate alone, so the answer proves the
        // server picked it for this name.
        let outbound = Outbound::trusting(Limits::default(), resolver, root.as_bytes()).unwrap();
        let home = AbsoluteUrl::parse(&format!("https://{domain}:{}/", ports.0)).unwrap();
        let answered = (0..300).find_map(|_| {
            let got = runtime.block_on(outbound.get(&home));
            match got {
                Ok(page) if page.status() == 200 => Some(page),
                _ => {
                    thread::sleep(Duration::from_millis(100));
                    None
                }
            }
        });
        let page = answered.unwrap_or_else(|| panic!("{domain} never answered over HTTPS"));
        assert!(
            String::from_utf8_lossy(page.body()).contains("<h1"),
            "{domain}"
        );
    }

    let news = on_port_80(ports.1, "news.example.com");
    assert!(news.starts_with("HTTP/1.1 308"), "{news}");
    assert!(
        news.contains("location: https://news.example.com/?ref=mail"),
        "{news}"
    );
    let unknown = on_port_80(ports.1, "shop.example.com");
    assert!(
        unknown.contains("location: https://example.com/?ref=mail"),
        "{unknown}"
    );
}

#[test]
fn a_sites_folder_refuses_domain_and_names_https() {
    let (root, _) = sites_with_certificates();

    let domain = run(root.path(), &["serve", "--domain", "example.com"]);

    let said = String::from_utf8_lossy(&domain.stderr);
    assert_eq!(domain.status.code(), Some(2), "{said}");
    assert!(said.contains("khalid serve --https"), "{said}");
}
