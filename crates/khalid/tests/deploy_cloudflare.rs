//! The Cloudflare starter in `deploy/cloudflare/`, which the "Deploy to
//! Cloudflare" button and the Pages steps in `docs/deploy/cloudflare.md`
//! both use: it lints and audits clean with this binary, and Cloudflare
//! serves the folder `khalid build` writes.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;
use tempfile::TempDir;

/// Where the starter lives.
const STARTER: &str = "deploy/cloudflare";

/// The path of a file in the repository.
#[cfg(test)]
fn repo_file(relative: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../..");
    path.push(relative);
    path
}

#[cfg(test)]
fn read(relative: &str) -> String {
    fs::read_to_string(repo_file(relative)).unwrap()
}

#[cfg(test)]
fn within(dir: &Path, relative: &str) -> PathBuf {
    let mut path = dir.to_path_buf();
    path.push(relative);
    path
}

/// A copy of the starter as git holds it, without what a local build left.
#[cfg(test)]
fn starter_copy() -> TempDir {
    let temp = tempfile::tempdir().unwrap();
    copy_dir(&repo_file(STARTER), temp.path());
    temp
}

#[cfg(test)]
fn copy_dir(from: &Path, into: &Path) {
    fs::create_dir_all(into).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        if ["public", "data", ".khalid", ".wrangler"].contains(&name.as_str()) {
            continue;
        }
        let target = within(into, &name);
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap();
        }
    }
}

#[cfg(test)]
fn khalid(site: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap()])
        .args(args)
        .env_remove("KHALID_ADDRESS")
        .output()
        .unwrap()
}

/// `wrangler.jsonc` as JSON: its comments are whole lines.
#[cfg(test)]
fn wrangler() -> Value {
    let text = read(&format!("{STARTER}/wrangler.jsonc"));
    let json: Vec<&str> = text
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect();
    serde_json::from_str(&json.join("\n")).unwrap()
}

#[test]
fn the_starter_lints_with_no_blocks_and_no_warnings() {
    let site = starter_copy();
    let lint = khalid(site.path(), &["lint"]);
    let stdout = String::from_utf8_lossy(&lint.stdout);
    assert!(lint.status.success(), "{lint:?}");
    assert!(stdout.contains("0 blocks and 0 warnings."), "{stdout}");
    assert!(
        stdout.contains("Every post scores 100 out of 100."),
        "{stdout}"
    );
}

#[test]
fn the_starter_audits_at_100_with_no_errors_or_warnings() {
    let site = starter_copy();
    let audit = khalid(site.path(), &["audit", "--offline", "--json"]);
    let stdout = String::from_utf8_lossy(&audit.stdout);
    let body: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}"));
    let loud: Vec<&Value> = body["issues"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|issue| issue["severity"] == "error" || issue["severity"] == "warning")
        .collect();
    assert_eq!(loud, Vec::<&Value>::new(), "{stdout}");
    assert_eq!(body["score"], 100, "{stdout}");
    assert_eq!(audit.status.code(), Some(0));
}

/// Cloudflare runs `npm run build`, then serves the assets folder that
/// `wrangler.jsonc` names, with the build's `404.html` for a missing page.
#[test]
fn wrangler_serves_the_folder_the_build_writes() {
    let site = starter_copy();
    let built = khalid(site.path(), &["build"]);
    assert!(built.status.success(), "{built:?}");
    let config = wrangler();
    let assets = &config["assets"];
    let folder = assets["directory"].as_str().unwrap();
    assert_eq!(folder, "./public");
    let served = within(site.path(), folder);
    assert!(within(&served, "index.html").is_file());
    assert!(within(&served, "welcome-new-blog/index.html").is_file());
    assert_eq!(assets["not_found_handling"], "404-page");
    assert!(within(&served, "404.html").is_file());
    assert!(config.get("main").is_none(), "the Worker has no code");
    let package: Value = serde_json::from_str(&read(&format!("{STARTER}/package.json"))).unwrap();
    assert_eq!(package["scripts"]["build"], "sh build.sh");
    let script = read(&format!("{STARTER}/build.sh"));
    let last = script.lines().rfind(|line| !line.is_empty());
    assert_eq!(last, Some("\"$bin/khalid\" --site \"$here\" build"));
}

/// The build installs this release, with the install script from the same
/// tag, which checks the download against the release's checksums.
#[test]
fn the_build_installs_the_pinned_release_with_the_install_script() {
    let script = read(&format!("{STARTER}/build.sh"));
    let version = format!("version=\"v{}\"", env!("CARGO_PKG_VERSION"));
    assert!(script.lines().any(|line| line == version), "{script}");
    assert!(script.contains(
        "installer=\"https://raw.githubusercontent.com/thebigasif/khalid/$version/scripts/install.sh\""
    ));
    assert!(script.contains("curl -fsSL -o \"$here/.khalid/install.sh\" \"$installer\""));
    assert!(
        script.contains("sh \"$here/.khalid/install.sh\" --version \"$version\" --dir \"$bin\"")
    );
    let install = read("scripts/install.sh");
    for option in ["--version)", "--dir)", "SHA256SUMS"] {
        assert!(install.contains(option), "install.sh has no {option}");
    }
    let ignored = read(&format!("{STARTER}/.gitignore"));
    for folder in ["/public/", "/data/", "/.khalid/"] {
        assert!(ignored.lines().any(|line| line == folder), "{folder}");
    }
}

/// A build variable sets the address without editing `site.toml`, as the
/// guide says for Workers Builds and Pages.
#[test]
fn khalid_address_sets_the_built_pages_address() {
    let site = starter_copy();
    let built = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.path().to_str().unwrap(), "build"])
        .env("KHALID_ADDRESS", "https://my-blog.pages.dev")
        .output()
        .unwrap();
    assert!(built.status.success(), "{built:?}");
    let home = fs::read_to_string(within(site.path(), "public/index.html")).unwrap();
    assert!(
        home.contains("href=\"https://my-blog.pages.dev/\""),
        "{home}"
    );
}
