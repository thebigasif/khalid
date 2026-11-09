//! Self-hosted fonts as a reader's browser asks for them: the page links
//! the stylesheet, the stylesheet's `@font-face` names the file, and the
//! address it names, read against the stylesheet's own address, answers
//! with the font. Through `khalid serve` with and without a mount, and in
//! a static build (G20).

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use serde_json::Value;
use tempfile::TempDir;

/// The bytes of the stand-in font: a woff2 signature, then filler.
const FONT: &[u8] = b"wOF2 stand-in font bytes";

/// A running `khalid serve`, stopped when dropped.
struct Server {
    child: Child,
    host: String,
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// One answer: its status, its headers in lower case, and its bytes.
struct Answer {
    status: u16,
    head: String,
    body: Vec<u8>,
}

#[cfg(test)]
impl Answer {
    fn header(&self, name: &str) -> Option<&str> {
        self.head
            .lines()
            .find_map(|line| line.strip_prefix(&format!("{name}: ")))
    }

    fn text(&self) -> String {
        String::from_utf8(self.body.clone()).unwrap()
    }
}

#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

#[cfg(test)]
fn copy_dir(from: &Path, into: &Path) {
    fs::create_dir_all(into).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let mut target = into.to_path_buf();
        target.push(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap();
        }
    }
}

/// A copy of the demo site, mounted at `mount`, with one listed font, one
/// woff2 file the tokens do not list, and a private note in `theme/fonts/`.
#[cfg(test)]
fn site_with_font(mount: &str) -> TempDir {
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    let site = tempfile::tempdir().unwrap();
    copy_dir(&demo, site.path());
    let config = fs::read_to_string(at(site.path(), "site.toml")).unwrap();
    let config = config.replace("mount = \"/blog/\"", &format!("mount = \"{mount}\""));
    fs::write(at(site.path(), "site.toml"), config).unwrap();
    let fonts = at(site.path(), "theme/fonts");
    fs::create_dir_all(&fonts).unwrap();
    fs::write(at(&fonts, "inter-latin-400.woff2"), FONT).unwrap();
    fs::write(at(&fonts, "unlisted.woff2"), FONT).unwrap();
    fs::write(at(&fonts, "notes.txt"), b"private").unwrap();
    let tokens_file = at(site.path(), "theme/tokens.toml");
    let tokens = fs::read_to_string(&tokens_file).unwrap_or_default();
    fs::write(
        &tokens_file,
        format!(
            "{tokens}\n[[fonts.files]]\nfamily = \"Inter\"\nstyle = \"normal\"\n\
             weight = \"400\"\npath = \"inter-latin-400.woff2\"\nrange = \"U+0000-00FF\"\n"
        ),
    )
    .unwrap();
    site
}

#[cfg(test)]
fn start(site: &Path) -> Server {
    let mut child = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap(), "serve", "--listen"])
        .args(["127.0.0.1:0", "--json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut line = String::new();
    let stdout = child.stdout.take().unwrap();
    BufReader::new(stdout).read_line(&mut line).unwrap();
    let json: Value = serde_json::from_str(&line).unwrap_or_else(|_| panic!("{line}"));
    let address = json["address"].as_str().unwrap();
    let rest = address.strip_prefix("http://").unwrap();
    let host = rest.split('/').next().unwrap().to_owned();
    Server { child, host }
}

#[cfg(test)]
fn get(server: &Server, path: &str) -> Answer {
    let host = &server.host;
    let mut stream = TcpStream::connect(host).unwrap();
    let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).unwrap();
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).unwrap();
    let split = raw.windows(4).position(|w| w == b"\r\n\r\n").unwrap();
    let head = String::from_utf8(raw[..split].to_vec())
        .unwrap()
        .to_ascii_lowercase();
    Answer {
        status: head.split(' ').nth(1).unwrap().parse().unwrap(),
        head,
        body: raw[split + 4..].to_vec(),
    }
}

/// The address of the stylesheet the page `html` links.
#[cfg(test)]
fn stylesheet_of(html: &str) -> String {
    let link = html
        .split("<link")
        .find(|tag| tag.contains("rel=\"stylesheet\""))
        .unwrap_or_else(|| panic!("no stylesheet link:\n{html}"));
    link.split("href=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap()
        .to_owned()
}

/// The first font address the stylesheet `css` names in an `@font-face`.
#[cfg(test)]
fn font_url_in(css: &str) -> String {
    let face = css
        .split("@font-face")
        .find(|rule| rule.contains("format(\"woff2\")"))
        .unwrap_or_else(|| panic!("no self-hosted face:\n{css}"));
    face.split("url(\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap()
        .to_owned()
}

/// `reference` read against the address `base`, as a browser reads a
/// `url()` in a stylesheet: a path from the root stays, anything else is
/// joined to `base`'s folder, with `..` and `.` taken out.
#[cfg(test)]
fn resolve(base: &str, reference: &str) -> String {
    assert!(
        !reference.contains("://"),
        "a font on another host: {reference}"
    );
    let joined = if reference.starts_with('/') {
        reference.to_owned()
    } else {
        let (folder, _) = base.rsplit_once('/').unwrap();
        format!("{folder}/{reference}")
    };
    let mut parts: Vec<&str> = Vec::new();
    for part in joined.split('/').skip(1) {
        match part {
            ".." => {
                parts.pop();
            }
            "." => {}
            _ => parts.push(part),
        }
    }
    format!("/{}", parts.join("/"))
}

/// The served blog at `mount` answers the font its stylesheet names, as a
/// font kept for a year, and nothing else in `theme/fonts/`.
#[cfg(test)]
fn serves_the_font_its_stylesheet_names(mount: &str) {
    let site = site_with_font(mount);
    let server = start(site.path());

    let home = get(&server, mount);
    assert_eq!(home.status, 200, "{}", home.head);
    let sheet = stylesheet_of(&home.text());
    let css = get(&server, &sheet);
    assert_eq!(css.status, 200, "{sheet}");
    let font = resolve(&sheet, &font_url_in(&css.text()));
    assert_eq!(font, format!("{mount}assets/fonts/inter-latin-400.woff2"));

    let answer = get(&server, &font);
    assert_eq!(answer.status, 200, "{font}: {}", answer.head);
    assert_eq!(answer.header("content-type"), Some("font/woff2"));
    assert_eq!(
        answer.header("cache-control"),
        Some("public, max-age=31536000, immutable")
    );
    assert_eq!(answer.body, FONT);

    for private in ["notes.txt", "unlisted.woff2", "../tokens.toml"] {
        let path = format!("{mount}assets/fonts/{private}");
        assert_eq!(get(&server, &path).status, 404, "{path} stays private");
    }
}

#[test]
fn a_served_blog_in_a_folder_answers_the_font_its_stylesheet_names() {
    serves_the_font_its_stylesheet_names("/blog/");
}

#[test]
fn a_served_blog_at_the_root_answers_the_font_its_stylesheet_names() {
    serves_the_font_its_stylesheet_names("/");
}

#[test]
fn a_static_build_holds_the_font_where_its_stylesheet_points() {
    let site = site_with_font("/blog/");
    let out = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.path().to_str().unwrap(), "build", "--out"])
        .arg(out.path())
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    // The output folder is the blog's folder, `/blog/`, on the host.
    let file = |address: &str| at(out.path(), address.strip_prefix("/blog/").unwrap());
    let home = fs::read_to_string(at(out.path(), "index.html")).unwrap();
    let sheet = stylesheet_of(&home);
    let css = fs::read_to_string(file(&sheet)).unwrap();
    let font = resolve(&sheet, &font_url_in(&css));
    assert_eq!(font, "/blog/assets/fonts/inter-latin-400.woff2");
    assert_eq!(fs::read(file(&font)).unwrap(), FONT);
    assert!(
        !at(out.path(), "assets/fonts/notes.txt").exists(),
        "only woff2 files are copied"
    );
    assert!(
        !at(out.path(), "assets/fonts/unlisted.woff2").exists(),
        "only the files the tokens list are copied"
    );
}
