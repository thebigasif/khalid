//! Saving the file of a page, glossary term, case study or changelog entry
//! on a running `khalid serve` fires its `.published`, `.published.edited`
//! and `.deleted` webhooks to a stand-in receiver on loopback, signed with
//! each webhook's secret, moving one to `drafts/` fires `.unpublished`, a
//! draft given, moved or cleared of a `publish_at` time fires `.scheduled`, a
//! saved `site.toml` or theme file fires `site.changed` once the server uses
//! it, and a post that names a tag new to the site fires `tag.added` (audit
//! brief G25, KH-API-3).

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use chrono::{SecondsFormat, TimeDelta};
use khalid_core::{AbsoluteUrl, Clock, Secret, Slug, SystemClock};
use khalid_db::{Database, WebhookTable};
use khalid_webhooks::{Event, SIGNATURE_HEADER, SigningKey, Verifier};
use serde_json::{Value, json};
use tempfile::TempDir;
use tokio::time::Instant;
use wiremock::matchers::method;
use wiremock::{Mock, MockServer, ResponseTemplate};

/// The stand-in host for the webhook receiver, on loopback.
const HOSTS: &str = "hooks.test=127.0.0.1";

/// How long a delivery may take to arrive. CI shares a busy server, where a
/// debug build renders the demo site slowly, so the wait is generous; a
/// delivery that arrives ends it at once.
const DEADLINE: Duration = Duration::from_secs(120);

/// A page no demo post links to, so the site renders with or without it.
const PAGE: &str = "---\ntitle: Contact Field Notes\nslug: contact\n---\n\nWrite to us.\n";

/// A glossary term no demo post links to.
const TERM: &str =
    "---\nterm: Crawl budget\nslug: crawl-budget\n---\n\nHow many pages a crawler reads.\n";

/// A case study no demo post links to.
const STUDY: &str = "---\ntitle: How a cafe got found\nslug: harbour-cafe\ndate: 2026-10-21\n---\n\nIt listed its menu.\n";

/// A changelog entry no demo post links to.
const CHANGELOG: &str = "---\ntitle: What changed in November\ndate: 2026-11-02\nversion: 2026.11\n---\n\n- Added a contact page.\n";

/// A running `khalid serve`, stopped when dropped.
struct Server(Child);

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// A copy of the demo site with a webhook for each of `events` sending to
/// a receiver at `/<event>`, and the running server. Gives back the
/// receiver, the site, the server and each webhook's secret.
#[cfg(test)]
async fn serving(events: &[Event]) -> (MockServer, TempDir, Server, Vec<Secret<String>>) {
    let receiver = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&receiver)
        .await;
    let mut demo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    demo.push("../../fixtures/demo-site/.");
    let site = tempfile::tempdir().unwrap();
    let copied = Command::new("cp")
        .arg("-R")
        .args([&demo, site.path()])
        .status();
    assert!(copied.unwrap().success());
    let mut file = site.path().to_path_buf();
    file.push("data");
    fs::create_dir_all(&file).unwrap();
    file.push("site.db");
    let db = Database::open(&file).await.unwrap();
    db.migrate().await.unwrap();
    let id = db.site(&Slug::try_new("main").unwrap()).await.unwrap();
    let table = WebhookTable::new(db.clone(), id, SystemClock);
    let mut secrets = Vec::new();
    for &event in events {
        let port = receiver.address().port();
        let url = AbsoluteUrl::parse(&format!("http://hooks.test:{port}/{event}")).unwrap();
        secrets.push(table.create(&url, event).await.unwrap().1);
    }
    db.close().await;
    let server = start(site.path());
    (receiver, site, server, secrets)
}

/// Starts `khalid serve`, reaching the stand-in host on loopback, and
/// waits until it listens.
#[cfg(test)]
fn start(site: &Path) -> Server {
    let mut child = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap()])
        .args(["--test-hosts", HOSTS])
        .args(["serve", "--listen", "127.0.0.1:0", "--json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut line = String::new();
    BufReader::new(child.stdout.take().unwrap())
        .read_line(&mut line)
        .unwrap();
    assert!(line.contains("address"), "{line}");
    Server(child)
}

/// The one delivery of `event`, waited for until [`DEADLINE`] and checked
/// against `secret`, as JSON.
#[cfg(test)]
async fn the_delivery(receiver: &MockServer, event: Event, secret: &Secret<String>) -> Value {
    let route = format!("/{event}");
    let deadline = Instant::now() + DEADLINE;
    let hooks = loop {
        let all = receiver.received_requests().await.unwrap();
        let hooks: Vec<_> = all.into_iter().filter(|r| r.url.path() == route).collect();
        if !hooks.is_empty() || Instant::now() >= deadline {
            break hooks;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    };
    assert_eq!(hooks.len(), 1, "{event} fires once");
    let hook = &hooks[0];
    assert_eq!(hook.headers["khalid-event"], event.as_str());
    let signature = hook.headers[SIGNATURE_HEADER].to_str().unwrap();
    let verifier = Verifier::new(SigningKey::try_new(secret).unwrap());
    verifier
        .verify(signature, &hook.body, &SystemClock)
        .unwrap();
    let body: Value = serde_json::from_slice(&hook.body).unwrap();
    assert_eq!(body["event"], event.as_str(), "{body}");
    body
}

/// Adds the file `name` with `text` to the site, changes it, then removes
/// it, and checks each of `events` fires in turn with the entry under
/// `key`, named `id`.
#[cfg(test)]
async fn add_change_remove(
    events: [Event; 3],
    (name, text): (&str, &str),
    (key, id): (&str, &str),
) {
    let (receiver, site, _server, secrets) = serving(&events).await;
    let mut file = site.path().to_path_buf();
    file.push(name);
    fs::write(&file, text).unwrap();
    let body = the_delivery(&receiver, events[0], &secrets[0]).await;
    assert_eq!(body[key]["id"], id, "{body}");
    assert_eq!(body[key]["file"], name, "{body}");

    fs::write(&file, format!("{text}\nOne more line.\n")).unwrap();
    let body = the_delivery(&receiver, events[1], &secrets[1]).await;
    assert_eq!(body[key]["id"], id, "{body}");

    fs::remove_file(&file).unwrap();
    let body = the_delivery(&receiver, events[2], &secrets[2]).await;
    assert_eq!(body[key]["id"], id, "the entry as it was: {body}");

    // Nothing fired twice, such as a second `.published` from a render of
    // the content read before the save.
    let all = receiver.received_requests().await.unwrap();
    let paths: Vec<&str> = all.iter().map(|r| r.url.path()).collect();
    assert_eq!(paths.len(), 3, "{paths:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_saved_page_fires_page_published_edited_and_deleted() {
    let events = [
        Event::PagePublished,
        Event::PagePublishedEdited,
        Event::PageDeleted,
    ];
    let file = ("content/en/pages/contact.md", PAGE);
    add_change_remove(events, file, ("page", "en.contact")).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn a_saved_glossary_term_fires_its_three_events() {
    let events = [
        Event::GlossaryTermPublished,
        Event::GlossaryTermPublishedEdited,
        Event::GlossaryTermDeleted,
    ];
    let file = ("content/en/glossary/crawl-budget.md", TERM);
    add_change_remove(events, file, ("glossary_term", "en.crawl-budget")).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn a_saved_case_study_fires_its_three_events() {
    let events = [
        Event::CaseStudyPublished,
        Event::CaseStudyPublishedEdited,
        Event::CaseStudyDeleted,
    ];
    let file = ("content/en/case-studies/harbour-cafe.md", STUDY);
    add_change_remove(events, file, ("case_study", "en.harbour-cafe")).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn a_saved_changelog_entry_fires_its_three_events() {
    let events = [
        Event::ChangelogEntryPublished,
        Event::ChangelogEntryPublishedEdited,
        Event::ChangelogEntryDeleted,
    ];
    let file = ("content/en/changelog/2026-11.md", CHANGELOG);
    add_change_remove(events, file, ("changelog_entry", "en.2026-11")).await;
}

/// Writes `text` as a new draft at `drafts/<name>`, moves it to the same
/// place under `content/`, then back, and checks `published` fires, then
/// `unpublished`, with the entry under `key`, named `id`, and that nothing
/// is deleted.
#[cfg(test)]
async fn publish_and_unpublish(
    [published, unpublished, deleted]: [Event; 3],
    (name, text): (&str, &str),
    (key, id): (&str, &str),
) {
    let events = [published, unpublished, deleted];
    let (receiver, site, _server, secrets) = serving(&events).await;
    let mut draft = site.path().to_path_buf();
    draft.push(format!("drafts/{name}"));
    fs::create_dir_all(draft.parent().unwrap()).unwrap();
    fs::write(&draft, text).unwrap();
    let mut live = site.path().to_path_buf();
    live.push(format!("content/{name}"));
    fs::rename(&draft, &live).unwrap();
    let body = the_delivery(&receiver, published, &secrets[0]).await;
    assert_eq!(body[key]["id"], id, "{body}");

    fs::rename(&live, &draft).unwrap();
    let body = the_delivery(&receiver, unpublished, &secrets[1]).await;
    assert_eq!(body[key]["id"], id, "the entry as it was: {body}");
    assert_eq!(body[key]["file"], format!("content/{name}"), "{body}");
    let all = receiver.received_requests().await.unwrap();
    let paths: Vec<&str> = all.iter().map(|r| r.url.path()).collect();
    assert_eq!(paths.len(), 2, "nothing was deleted: {paths:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_page_moved_to_drafts_fires_page_unpublished() {
    let events = [
        Event::PagePublished,
        Event::PageUnpublished,
        Event::PageDeleted,
    ];
    let file = ("en/pages/contact.md", PAGE);
    publish_and_unpublish(events, file, ("page", "en.contact")).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn a_glossary_term_moved_to_drafts_fires_glossary_term_unpublished() {
    let events = [
        Event::GlossaryTermPublished,
        Event::GlossaryTermUnpublished,
        Event::GlossaryTermDeleted,
    ];
    let file = ("en/glossary/crawl-budget.md", TERM);
    publish_and_unpublish(events, file, ("glossary_term", "en.crawl-budget")).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn a_case_study_moved_to_drafts_fires_case_study_unpublished() {
    let events = [
        Event::CaseStudyPublished,
        Event::CaseStudyUnpublished,
        Event::CaseStudyDeleted,
    ];
    let file = ("en/case-studies/harbour-cafe.md", STUDY);
    publish_and_unpublish(events, file, ("case_study", "en.harbour-cafe")).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn a_changelog_entry_moved_to_drafts_fires_changelog_entry_unpublished() {
    let events = [
        Event::ChangelogEntryPublished,
        Event::ChangelogEntryUnpublished,
        Event::ChangelogEntryDeleted,
    ];
    let file = ("en/changelog/2026-11.md", CHANGELOG);
    publish_and_unpublish(events, file, ("changelog_entry", "en.2026-11")).await;
}

/// The bodies of the first `count` deliveries of `event`, in the order they
/// came, waited for until [`DEADLINE`], each checked against `secret`.
#[cfg(test)]
async fn deliveries(
    receiver: &MockServer,
    (event, count): (Event, usize),
    secret: &Secret<String>,
) -> Vec<Value> {
    let route = format!("/{event}");
    let deadline = Instant::now() + DEADLINE;
    let hooks = loop {
        let all = receiver.received_requests().await.unwrap();
        let hooks: Vec<_> = all.into_iter().filter(|r| r.url.path() == route).collect();
        if hooks.len() >= count || Instant::now() >= deadline {
            break hooks;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    };
    let verifier = Verifier::new(SigningKey::try_new(secret).unwrap());
    let checked = hooks.iter().map(|hook| {
        let signature = hook.headers[SIGNATURE_HEADER].to_str().unwrap();
        verifier
            .verify(signature, &hook.body, &SystemClock)
            .unwrap();
        serde_json::from_slice::<Value>(&hook.body).unwrap()
    });
    checked.collect()
}

/// `text` with `publish_at: <at>` added to its front matter.
#[cfg(test)]
fn at(text: &str, at: &str) -> String {
    text.replacen("---\n", &format!("---\npublish_at: {at}\n"), 1)
}

/// Writes `text` as a draft at `drafts/<name>` with a publish time, moves
/// the time, then takes it away, and checks `scheduled` fires once for
/// each with the entry under `key`, named `id`, and the time or `null`.
#[cfg(test)]
async fn schedule_move_cancel(
    scheduled: Event,
    (name, text): (&str, &str),
    (key, id): (&str, &str),
) {
    let (receiver, site, _server, secrets) = serving(&[scheduled]).await;
    let mut draft = site.path().to_path_buf();
    draft.push(format!("drafts/{name}"));
    fs::create_dir_all(draft.parent().unwrap()).unwrap();
    let times = ["2099-01-05T09:00:00Z", "2099-01-06T10:30:00+01:00"];
    for (n, time) in times.iter().enumerate() {
        fs::write(&draft, at(text, time)).unwrap();
        deliveries(&receiver, (scheduled, n + 1), &secrets[0]).await;
    }
    fs::write(&draft, text).unwrap();
    let bodies = deliveries(&receiver, (scheduled, 3), &secrets[0]).await;

    let told: Vec<&Value> = bodies.iter().map(|body| &body[key]["publish_at"]).collect();
    let (nine, half_nine) = (json!("2099-01-05T09:00:00Z"), json!("2099-01-06T09:30:00Z"));
    assert_eq!(told, [&nine, &half_nine, &Value::Null], "{bodies:?}");
    for body in &bodies {
        assert_eq!(body["event"], scheduled.as_str(), "{body}");
        assert_eq!(body[key]["id"], id, "{body}");
        assert_eq!(body[key]["file"], format!("drafts/{name}"), "{body}");
    }
    let all = receiver.received_requests().await.unwrap();
    assert_eq!(all.len(), 3, "each change fired once");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_page_draft_scheduled_moved_and_cancelled_fires_page_scheduled() {
    let file = ("en/pages/contact.md", PAGE);
    schedule_move_cancel(Event::PageScheduled, file, ("page", "en.contact")).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn a_glossary_term_draft_scheduled_fires_glossary_term_scheduled() {
    let file = ("en/glossary/crawl-budget.md", TERM);
    let entry = ("glossary_term", "en.crawl-budget");
    schedule_move_cancel(Event::GlossaryTermScheduled, file, entry).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn a_case_study_draft_scheduled_fires_case_study_scheduled() {
    let file = ("en/case-studies/harbour-cafe.md", STUDY);
    let entry = ("case_study", "en.harbour-cafe");
    schedule_move_cancel(Event::CaseStudyScheduled, file, entry).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn a_changelog_draft_scheduled_fires_changelog_entry_scheduled() {
    let file = ("en/changelog/2026-11.md", CHANGELOG);
    let entry = ("changelog_entry", "en.2026-11");
    schedule_move_cancel(Event::ChangelogEntryScheduled, file, entry).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn a_page_draft_goes_live_at_its_time_and_fires_page_published_once() {
    let events = [Event::PageScheduled, Event::PagePublished];
    let (receiver, site, _server, secrets) = serving(&events).await;
    let mut draft = site.path().to_path_buf();
    draft.push("drafts/en/pages/contact.md");
    fs::create_dir_all(draft.parent().unwrap()).unwrap();
    // Far enough ahead that serve sees the draft waiting first.
    let soon = SystemClock.now() + TimeDelta::seconds(10);
    let soon = soon.to_rfc3339_opts(SecondsFormat::Secs, true);
    fs::write(&draft, at(PAGE, &soon)).unwrap();

    let scheduled = deliveries(&receiver, (events[0], 1), &secrets[0]).await;
    assert_eq!(scheduled[0]["page"]["publish_at"], soon.as_str());
    let body = the_delivery(&receiver, events[1], &secrets[1]).await;
    assert_eq!(body["page"]["id"], "en.contact", "{body}");
    assert_eq!(
        body["page"]["file"], "content/en/pages/contact.md",
        "{body}"
    );
    let mut live = site.path().to_path_buf();
    live.push("content/en/pages/contact.md");
    assert_eq!(fs::read_to_string(&live).unwrap(), PAGE, "no publish_at");
    assert!(!draft.exists(), "the draft moved");

    tokio::time::sleep(Duration::from_secs(6)).await;
    let all = receiver.received_requests().await.unwrap();
    let paths: Vec<&str> = all.iter().map(|r| r.url.path()).collect();
    assert_eq!(paths, ["/page.scheduled", "/page.published"], "once each");
}

/// Saves `site.toml` with `change` made to its text, and gives back the
/// `settings` of the one `site.changed` delivery.
#[cfg(test)]
async fn site_toml_saved(change: impl FnOnce(String) -> String) -> Value {
    let (receiver, site, _server, secrets) = serving(&[Event::SiteChanged]).await;
    let mut file = site.path().to_path_buf();
    file.push("site.toml");
    let text = fs::read_to_string(&file).unwrap();
    fs::write(&file, change(text)).unwrap();
    let body = the_delivery(&receiver, Event::SiteChanged, &secrets[0]).await;
    body["settings"].clone()
}

#[tokio::test(flavor = "multi_thread")]
async fn a_saved_site_toml_fires_site_changed_naming_the_settings_only() {
    let settings = site_toml_saved(|text| {
        let old = "name = \"Field Notes on SEO\"\n";
        assert!(text.contains(old), "{text}");
        text.replacen(old, "name = \"Field Notes Weekly\"\n", 1)
    })
    .await;
    assert_eq!(settings["file"], "site.toml", "{settings}");
    assert_eq!(settings["files"], serde_json::json!(["site.toml"]));
    assert_eq!(settings["changed"], serde_json::json!(["name"]));
    assert_eq!(
        settings["restart_needed"], false,
        "the pages use a new name at once"
    );
    assert!(!settings.to_string().contains("Weekly"), "no value");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_start_only_setting_fires_site_changed_with_restart_needed() {
    let settings = site_toml_saved(|text| format!("{text}\npreview_link_days = 3\n")).await;
    assert_eq!(
        settings["changed"],
        serde_json::json!(["preview_link_days"])
    );
    assert_eq!(settings["restart_needed"], true, "{settings}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_saved_theme_file_fires_site_changed_once_it_is_in_use() {
    let (receiver, site, _server, secrets) = serving(&[Event::SiteChanged]).await;
    let mut file = site.path().to_path_buf();
    file.push("theme");
    fs::create_dir_all(&file).unwrap();
    file.push("site.css");
    fs::write(&file, ".site-title { letter-spacing: 0.01em; }\n").unwrap();

    let body = the_delivery(&receiver, Event::SiteChanged, &secrets[0]).await;
    let settings = &body["settings"];
    assert_eq!(settings["files"], serde_json::json!(["theme/site.css"]));
    assert_eq!(settings["changed"], serde_json::json!([]), "{body}");
    assert_eq!(settings["restart_needed"], false, "{body}");
    assert!(settings.get("file").is_none(), "site.toml is as it was");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_post_that_names_a_new_tag_fires_tag_added() {
    let (receiver, site, _server, secrets) = serving(&[Event::TagAdded]).await;
    let mut file = site.path().to_path_buf();
    file.push("content/en/posts/2026-09-14-why-internal-links-matter.md");
    let text = fs::read_to_string(&file).unwrap();
    let tagged = "category: on-page-seo\ntags: [crawl budget]\n";
    fs::write(&file, text.replacen("category: on-page-seo\n", tagged, 1)).unwrap();

    let body = the_delivery(&receiver, Event::TagAdded, &secrets[0]).await;
    assert_eq!(body["tag"]["id"], "en.crawl-budget", "{body}");
    assert_eq!(body["tag"]["name"], "crawl budget", "{body}");
    assert_eq!(body["tag"]["language"], "en", "{body}");
}
