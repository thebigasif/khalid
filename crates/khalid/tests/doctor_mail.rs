//! `khalid doctor` checks mail: `[mail]` is valid, the secret it names is
//! in the environment, which doctor never prints, and with
//! `--send-test-mail` one email goes out through the site's own mailer
//! (G38, KH-OPS-2). Every secret here is made up for the test, and the mail
//! server is a stand-in on loopback.

use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::TimeDelta;
use khalid_core::{Clock, Slug, SystemClock};
use khalid_db::{Database, Failure, FormMailTable};
use khalid_forms::FormId;
use khalid_mail::StubSmtp;
use khalid_newsletter::{EmailAddress, Mail};
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
fn a_good_secret_passes_and_the_test_email_arrives_without_the_secret_shown() {
    let stub = StubSmtp::start().unwrap();
    let site = site_with(&smtp(stub.port()));
    let password = format!("pw-made-for-this-test-{}", std::process::id());
    let secrets = [("KHALID_SMTP_PASSWORD", password.as_str())];

    let (code, mail, _, said) = doctor(site.path(), &secrets, &[]);
    assert_eq!(code, Some(0), "{said}");
    assert_eq!(mail["status"], "ok", "{mail}");
    assert!(
        stub.received().is_empty(),
        "no email without --send-test-mail"
    );

    let (code, mail, _, said) = doctor(site.path(), &secrets, &["--send-test-mail"]);
    assert_eq!(code, Some(0), "{said}");
    let summary = mail["summary"].as_str().unwrap();
    assert!(
        summary.contains("A test email went to hello@example.com."),
        "{summary}"
    );
    assert!(
        !said.contains(&password),
        "doctor printed the password: {said}"
    );
    let emails = stub.received();
    assert_eq!(emails.len(), 1, "{emails:?}");
    assert!(emails[0].contains("To: hello@example.com"), "{}", emails[0]);
    assert!(
        emails[0].contains("Subject: A test email from khalid doctor"),
        "{}",
        emails[0]
    );
}

#[test]
fn a_test_email_that_fails_is_a_problem_that_says_why() {
    let closed = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = closed.local_addr().unwrap().port();
    drop(closed);
    let site = site_with(&smtp(port));
    let secrets = [("KHALID_SMTP_PASSWORD", "made-for-this-test")];
    let (code, mail, _, said) = doctor(site.path(), &secrets, &["--send-test-mail"]);
    assert_eq!(code, Some(1), "{said}");
    let what = mail["problem"]["what"].as_str().unwrap();
    assert!(
        what.starts_with("The test email to hello@example.com could not be sent: "),
        "{what}"
    );
    assert!(
        what.contains("the SMTP server did not take the email"),
        "{what}"
    );
    let fix = mail["problem"]["fix"].as_str().unwrap();
    assert!(fix.contains("[mail.smtp]"), "{fix}");
}

#[tokio::test]
async fn a_form_email_khalid_gave_up_on_is_worth_a_look() {
    let site = site_with(&smtp(25));
    let mut file = within(site.path(), "data");
    fs::create_dir_all(&file).unwrap();
    file.push("site.db");
    let db = Database::open(&file).await.unwrap();
    db.migrate().await.unwrap();
    let site_id = db.site(&Slug::try_new("main").unwrap()).await.unwrap();
    let to = EmailAddress::parse("hello@example.com").unwrap();
    let mail = Mail::new(
        to,
        "New answer".to_owned(),
        "name: Grace Hopper\n".to_owned(),
    );
    let failure = Failure {
        at: SystemClock.now() - TimeDelta::hours(1),
        error: "the email could not be sent: the SMTP server did not take the email",
        next_at: None,
    };
    let table = FormMailTable::new(db.clone(), site_id);
    let contact = FormId::try_new("contact").unwrap();
    table.keep(&contact, &mail, failure).await.unwrap();
    db.close().await;

    let secrets = [("KHALID_SMTP_PASSWORD", "made-for-this-test")];
    let (code, _, _, said) = doctor(site.path(), &secrets, &[]);
    assert_eq!(code, Some(0), "worth a look, not a problem: {said}");
    let body: Value = serde_json::from_str(&said).unwrap();
    let warning = body["warnings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|warning| warning["what"].as_str().unwrap().contains("gave up"))
        .unwrap_or_else(|| panic!("no given-up note in {said}"));
    assert_eq!(
        warning["what"],
        "Khalid gave up emailing 1 form answer to you in the last 30 days, after a day of failed tries."
    );
    assert!(
        warning["fix"]
            .as_str()
            .unwrap()
            .contains("did not take the email"),
        "{warning}"
    );
    assert!(
        !said.contains("Grace Hopper"),
        "doctor printed an answer: {said}"
    );
}
