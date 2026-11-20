//! Subscribers and newsletters through the Admin API on a running `khalid
//! serve` (G39, plan 4.6): a reader who signed up and confirmed is
//! listed, gets a label, gets a post sent to their label once, and is
//! forgotten on request.

mod common;

use std::fs;
use std::thread;
use std::time::Duration;

use common::{Server, demo_copy, new_key, start};
use khalid_mail::StubSmtp;
use serde_json::json;

/// A key that manages subscribers.
const OWNER: [&str; 4] = ["--name", "Owner", "--scope", "subscribers:write"];

/// The routes under the demo's mount, where the admin calls them.
const SUBSCRIBERS: &str = "/blog/api/admin/v1/subscribers/";
const NEWSLETTERS: &str = "/blog/api/admin/v1/newsletters/";
const SENDS: &str = "/blog/api/admin/v1/newsletters/main/sends/";

/// `[mail]` through `stub`, with the postal address every newsletter shows.
#[cfg(test)]
fn mail_settings(stub: &StubSmtp) -> String {
    let port = stub.port();
    format!(
        "\n[mail]\nfrom = \"news@example.com\"\nname = \"Field Notes\"\n\
         postal_address = \"1 Park Row, Leeds LS1 5AB, UK\"\n\n[mail.smtp]\n\
         host = \"127.0.0.1\"\nport = {port}\nsecurity = \"none\"\nprivate_host = true\n"
    )
}

/// Signs `email` up with the plain form and opens the link the stub got.
#[cfg(test)]
fn subscribe(server: &Server, stub: &StubSmtp, email: &str) {
    let form = format!("email={}&homepage=", email.replace('@', "%40"));
    let typed = Some("application/x-www-form-urlencoded");
    let (status, page) = server.send("POST", "/blog/subscribe", &[], (form.as_bytes(), typed));
    assert_eq!(status, 200, "{page}");
    let received = stub.received();
    let message = received
        .last()
        .unwrap()
        .replace("=\n", "")
        .replace("=3D", "=");
    let (_, rest) = message.split_once("subscribe/confirm?token=").unwrap();
    let token = rest.split(char::is_whitespace).next().unwrap();
    let confirm = format!("token={token}");
    let route = "/blog/subscribe/confirm";
    let (status, page) = server.send("POST", route, &[], (confirm.as_bytes(), typed));
    assert_eq!(status, 200, "{page}");
}

/// Whether the stub holds `count` emails within 20 seconds.
#[cfg(test)]
fn received(stub: &StubSmtp, count: usize) -> bool {
    (0..200).any(|_| {
        thread::sleep(Duration::from_millis(100));
        stub.received().len() >= count
    })
}

#[test]
fn a_confirmed_reader_is_labelled_sent_a_post_once_and_forgotten() {
    let stub = StubSmtp::start().unwrap();
    let site = demo_copy();
    let mut settings = site.path().to_path_buf();
    settings.push("site.toml");
    let text = format!(
        "{}{}",
        fs::read_to_string(&settings).unwrap(),
        mail_settings(&stub)
    );
    fs::write(&settings, text).unwrap();
    let server = start(site.path(), &new_key(site.path(), &OWNER));
    assert!(server.answers("/blog/", 200));
    subscribe(&server, &stub, "grace@example.com");
    subscribe(&server, &stub, "ada@example.com");

    let (status, listed) = server.admin("GET", SUBSCRIBERS, None);
    assert_eq!(status, 200, "{listed}");
    assert_eq!(listed["meta"]["pagination"]["total"], 2);
    let grace = &listed["subscribers"][1];
    assert_eq!(grace["email"], "grace@example.com");
    assert_eq!(
        (grace["status"].as_str(), &grace["lists"]),
        (Some("confirmed"), &json!(["main"]))
    );
    let id = grace["id"].as_i64().unwrap();
    let (status, labelled) = server.admin("PUT", &format!("{SUBSCRIBERS}{id}/labels/paid/"), None);
    assert_eq!(status, 200, "{labelled}");

    let (_, newsletters) = server.admin("GET", NEWSLETTERS, None);
    assert_eq!(newsletters["sending"], "ready", "{newsletters}");
    let main = &newsletters["newsletters"][0];
    assert_eq!(
        (main["name"].as_str(), main["confirmed"].as_u64()),
        (Some("main"), Some(2))
    );
    assert!(main["form"].as_str().unwrap().contains("<form"), "{main}");

    let count = format!("{NEWSLETTERS}main/readers/?labels=paid");
    let (status, counted) = server.admin("GET", &count, None);
    assert_eq!(status, 200, "{counted}");
    assert_eq!(counted["readers"], 1, "the count shown before the send");

    let before = stub.received().len();
    let send = json!({"post": "en.why-internal-links-matter", "labels": ["paid"]});
    let (status, started) = server.admin("POST", SENDS, Some(&send));
    assert_eq!(status, 202, "{started}");
    assert_eq!(started["key"], "post:en/why-internal-links-matter");
    assert!(received(&stub, before + 1), "the post never left");
    let mails = stub.received();
    assert_eq!(mails.len(), before + 1, "only the labelled reader gets it");
    assert!(
        mails[before].contains("To: grace@example.com"),
        "{}",
        mails[before]
    );
    let (status, again) = server.admin("POST", SENDS, Some(&send));
    assert_eq!(status, 409, "{again}");

    let (status, _) = server.admin("DELETE", &format!("{SUBSCRIBERS}{id}/"), None);
    assert_eq!(status, 204);
    let (_, listed) = server.admin("GET", SUBSCRIBERS, None);
    assert_eq!(listed["meta"]["pagination"]["total"], 1, "{listed}");
    assert_eq!(listed["subscribers"][0]["email"], "ada@example.com");
}

/// Substack's `email_list` file: Lena reads free, Omar pays, Ines turned
/// email off, and one row has no address that takes mail.
const SUBSTACK: &str = "email,active_subscription,expiry,plan,email_disabled,created_at\n\
lena@example.com,false,,other,false,2021-03-04T05:06:07.000Z\n\
omar@example.com,true,2027-01-01T00:00:00.000Z,yearly,false,2022-01-01T00:00:00.000Z\n\
ines@example.org,false,,other,true,2020-05-06T07:08:09.000Z\n\
not-an-address,false,,other,false,\n";

#[test]
fn readers_come_in_with_their_status_and_no_email_and_go_out_as_csv() {
    let stub = StubSmtp::start().unwrap();
    let site = demo_copy();
    let mut settings = site.path().to_path_buf();
    settings.push("site.toml");
    let mut text = fs::read_to_string(&settings).unwrap();
    text.push_str(&mail_settings(&stub));
    fs::write(&settings, text).unwrap();
    let key = new_key(site.path(), &OWNER);
    let server = start(site.path(), &key);
    assert!(server.answers("/blog/", 200));
    let auth = format!("Khalid {key}");
    let auth = [("Authorization", auth.as_str())];
    let csv = |method, path: &str, body: &str| {
        let typed = Some("text/csv");
        server.send(method, path, &auth, (body.as_bytes(), typed))
    };

    let import = format!("{SUBSCRIBERS}import/");
    let (status, answer) = csv("POST", &import, SUBSTACK);
    assert_eq!(status, 200, "{answer}");
    let answer: serde_json::Value = serde_json::from_str(&answer).unwrap();
    let skipped = json!([{"line": 5, "why": "its address cannot take mail"}]);
    let expected = json!({"source": "substack", "added": 3, "already": 0, "skipped": skipped});
    assert_eq!(answer, expected);
    let (_, listed) = server.admin("GET", &format!("{SUBSCRIBERS}?label=paid"), None);
    assert_eq!(
        listed["subscribers"][0]["email"], "omar@example.com",
        "{listed}"
    );
    let (_, listed) = server.admin("GET", &format!("{SUBSCRIBERS}?status=unsubscribed"), None);
    assert_eq!(
        listed["subscribers"][0]["email"], "ines@example.org",
        "{listed}"
    );

    let kim = json!({"email": "Kim@Example.com", "status": "pending"});
    let (status, added) = server.admin("POST", SUBSCRIBERS, Some(&kim));
    assert_eq!(status, 201, "{added}");
    assert_eq!(added["subscribers"][0]["lists"], json!(["main"]), "{added}");
    let (status, again) = server.admin("POST", SUBSCRIBERS, Some(&kim));
    assert_eq!(status, 409, "{again}");

    let (status, exported) = csv("GET", &format!("{SUBSCRIBERS}export/"), "");
    assert_eq!(status, 200, "{exported}");
    assert!(
        exported.starts_with("email,status,created_at,lists,labels\nlena@example.com,confirmed,2021-03-04T05:06:07+00:00,main,\n"),
        "{exported}"
    );
    assert!(
        exported.contains("\nines@example.org,unsubscribed,"),
        "{exported}"
    );
    let (status, answer) = csv("POST", &import, &exported);
    assert_eq!(status, 200, "{answer}");
    assert!(answer.contains("\"added\":0,\"already\":4"), "{answer}");
    assert_eq!(stub.received(), Vec::<String>::new(), "no email goes out");
}
