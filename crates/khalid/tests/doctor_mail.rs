//! `khalid doctor` checks mail: `[mail]` is valid and the secret it names
//! is in the environment, which doctor never prints (G38, KH-OPS-2). Every
//! secret here is made up for the test.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;
use tempfile::TempDir;

/// Every variable that holds a mail secret, removed before each run.
const SECRETS: [&str; 3] = [
    "KHALID_SMTP_PASSWORD",
    "KHALID_RESEND_API_KEY",
    "KHALID_POSTMARK_TOKEN",
];

/// `name` inside `dir`.
#[cfg(test)]
fn within(dir: &Path, name: &str) -> PathBuf {
    let mut path = dir.to_path_buf();
    path.push(name);
    path
}

/// The demo site's settings with `more` added, in a fresh folder.
#[cfg(test)]
fn site_with(more: &str) -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    let settings = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/demo-site/site.toml"
    );
    let settings = format!("{}\n{more}", fs::read_to_string(settings).unwrap());
    fs::write(within(dir.path(), "site.toml"), settings).unwrap();
    fs::create_dir(within(dir.path(), "content")).unwrap();
    dir
}

/// `[mail]` through an SMTP relay on loopback at `port`, signing in.
#[cfg(test)]
fn smtp(port: u16) -> String {
    format!(
        "[mail]\nfrom = \"news@example.com\"\nforms_to = \"hello@example.com\"\n\n\
         [mail.smtp]\nhost = \"127.0.0.1\"\nport = {port}\nsecurity = \"none\"\n\
         private_host = true\nusername = \"news@example.com\"\n"
    )
}

/// Runs `khalid doctor --offline --json` with `args` on `site` with only
/// the secrets in `secrets` set; returns the exit code, the mail check,
/// the settings check and all it printed.
#[cfg(test)]
fn doctor(
    site: &Path,
    secrets: &[(&str, &str)],
    args: &[&str],
) -> (Option<i32>, Value, Value, String) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_khalid"));
    command
        .arg("--site")
        .arg(site)
        .args(["doctor", "--offline", "--json"])
        .args(args);
    for name in SECRETS {
        command.env_remove(name);
    }
    command.envs(secrets.iter().copied());
    let output = command.output().unwrap();
    let said = String::from_utf8_lossy(&output.stdout).into_owned();
    let body: Value = serde_json::from_str(&said).unwrap_or_else(|_| panic!("{said}"));
    let find = |name: &str| {
        body["checks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|check| check["check"] == name)
            .unwrap_or_else(|| panic!("no {name} check in {body}"))
            .clone()
    };
    (output.status.code(), find("mail"), find("settings"), said)
}

#[test]
fn a_site_without_mail_is_fine_and_told_what_it_misses() {
    let site = site_with("");
    let (code, mail, _, said) = doctor(site.path(), &[], &[]);
    assert_eq!(code, Some(0), "{said}");
    assert_eq!(mail["status"], "ok", "{mail}");
    let summary = mail["summary"].as_str().unwrap();
    assert!(summary.contains("sends no email"), "{summary}");
    assert!(summary.contains("[mail]"), "{summary}");
}

#[test]
fn a_missing_mail_secret_is_one_plain_problem_under_mail() {
    let site = site_with(&smtp(25));
    let (code, mail, settings, said) = doctor(site.path(), &[], &[]);
    assert_eq!(code, Some(1), "{said}");
    assert_eq!(
        settings["status"], "ok",
        "the rest of the settings load: {settings}"
    );
    assert_eq!(mail["status"], "problem", "{mail}");
    let problem = &mail["problem"];
    assert_eq!(
        problem["what"],
        "Mail is set to go through the SMTP server 127.0.0.1, but KHALID_SMTP_PASSWORD is not set."
    );
    assert!(
        problem["why"]
            .as_str()
            .unwrap()
            .contains("refuses to start"),
        "{problem}"
    );
    let fix = problem["fix"].as_str().unwrap();
    assert!(fix.starts_with("Set KHALID_SMTP_PASSWORD"), "{fix}");
    assert!(
        said.contains("\"problems\":1"),
        "only the mail check: {said}"
    );
}

#[test]
fn a_good_secret_passes_without_being_shown() {
    let site = site_with(&smtp(25));
    let password = format!("pw-made-for-this-test-{}", std::process::id());
    let secrets = [("KHALID_SMTP_PASSWORD", password.as_str())];
    let (code, mail, _, said) = doctor(site.path(), &secrets, &[]);
    assert_eq!(code, Some(0), "{said}");
    assert_eq!(mail["status"], "ok", "{mail}");
    assert_eq!(
        mail["summary"],
        "Mail goes out through the SMTP server 127.0.0.1 from news@example.com, \
         with KHALID_SMTP_PASSWORD set."
    );
    assert!(
        !said.contains(&password),
        "doctor printed the password: {said}"
    );
}
