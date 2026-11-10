//! The Admin API's posts on a copy of the demo site.

use std::fs;
use std::path::{Path, PathBuf};

use chrono::{NaiveDate, TimeZone, Utc};
use khalid_api::post_store::{CreatePostError, PostStore, UpdatePostError};
use khalid_api::resources::{AdminPost, Alias, PostDraft, PostStatus};
use khalid_core::{Environment, FixedClock};
use serde_json::{Value, json};

use super::*;

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

/// A copy of the demo site and its posts, on 2026-10-26.
fn demo() -> (tempfile::TempDir, SitePosts) {
    let mut fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fixture.push("../../fixtures/demo-site");
    let temp = tempfile::tempdir().unwrap();
    copy_dir(&fixture, temp.path());
    let config = Config::load(temp.path(), &Environment::empty()).unwrap();

    let clock = FixedClock::new(Utc.with_ymd_and_hms(2026, 10, 26, 9, 0, 0).unwrap());
    let posts = SitePosts::new(temp.path(), config, clock);
    (temp, posts)
}

fn within(dir: &Path, name: &str) -> PathBuf {
    let mut path = dir.to_path_buf();
    path.push(name);
    path
}

/// A draft that passes every blocking rule once dated.
fn good_draft() -> PostDraft {
    let markdown = "Anchor text is the visible words of a link.\n\n\
        Read [why internal links matter](/blog/why-internal-links-matter/) first.";
    PostDraft::new("Choosing anchor text", None, markdown)
        .with_author(Some(Slug::try_new("asif").unwrap()))
}

#[test]
fn a_new_post_is_a_draft_file_made_once_per_key() {
    let (site, posts) = demo();
    let made = posts
        .0
        .create_now(&good_draft(), "1111222233334444", Some("k1"))
        .unwrap();
    assert_eq!(made.id(), "en.choosing-anchor-text");
    assert_eq!(made.status(), PostStatus::Draft);
    assert_eq!(made.author().map(Slug::as_str), Some("asif"));
    assert_eq!(made.updated_by(), Some("1111222233334444"));
    let file = within(site.path(), "drafts/en/choosing-anchor-text.md");
    let text = fs::read_to_string(&file).unwrap();
    let front =
        "---\ntitle: \"Choosing anchor text\"\nslug: choosing-anchor-text\nauthor: asif\n---\n";
    assert!(text.starts_with(front), "{text}");

    let again = posts
        .0
        .create_now(&good_draft(), "1111222233334444", Some("k1"))
        .unwrap();
    assert_eq!(again.id(), made.id());
    let taken = posts
        .0
        .create_now(&good_draft(), "1111222233334444", Some("k2"));
    assert!(
        matches!(taken, Err(CreatePostError::SlugTaken(_))),
        "{taken:?}"
    );
    let demo_slug = PostDraft::new("Why internal links matter", None, "Words.");
    let taken = posts.0.create_now(&demo_slug, "1111222233334444", None);
    assert!(
        matches!(taken, Err(CreatePostError::SlugTaken(_))),
        "a published slug is taken too"
    );
}

#[test]
fn a_draft_and_a_published_post_read_back_by_identity() {
    let (_site, posts) = demo();
    posts.0.create_now(&good_draft(), "k", None).unwrap();
    let draft = posts.0.get_now("en.choosing-anchor-text").unwrap().unwrap();
    assert_eq!(draft.markdown(), good_draft().markdown());
    assert_eq!(draft.published_at(), None);

    let live = posts
        .0
        .get_now("de.warum-interne-links-wichtig-sind")
        .unwrap()
        .unwrap();
    assert_eq!(live.status(), PostStatus::Published);
    assert_eq!(
        live.published_at(),
        Some(Utc.with_ymd_and_hms(2026, 9, 14, 0, 0, 0).unwrap())
    );
    assert!(
        !live.markdown().is_empty() && !live.markdown().contains("translation:"),
        "{}",
        live.markdown()
    );

    for unknown in [
        "en.no-such-post",
        "fr.why-internal-links-matter",
        "no-language",
        "en.Bad Slug",
    ] {
        assert!(posts.0.get_now(unknown).unwrap().is_none(), "{unknown}");
    }
}

/// An edit as JSON a client sends.
fn edit(fields: Value) -> PostEdit {
    serde_json::from_value(fields).unwrap()
}

#[tokio::test]
async fn a_stale_edit_changes_nothing() {
    let (_site, posts) = demo();
    posts.create(good_draft(), "k", None).await.unwrap();
    let stale = edit(json!({"title": "New", "updated_at": "2026-01-01T00:00:00Z"}));
    let refused = posts.update("en.choosing-anchor-text", stale, "k").await;
    assert!(
        matches!(refused, Err(UpdatePostError::Stale { .. })),
        "{refused:?}"
    );
    let missing = edit(json!({"title": "New", "updated_at": "2026-01-01T00:00:00Z"}));
    let refused = posts.update("en.no-such-post", missing, "k").await;
    assert!(
        matches!(refused, Err(UpdatePostError::NotFound)),
        "{refused:?}"
    );
}

#[tokio::test]
async fn the_gate_keeps_a_weak_post_a_draft() {
    let (site, posts) = demo();
    let weak = PostDraft::new("A weak post", None, "No link and no author.");
    let made = posts.create(weak, "k", None).await.unwrap();
    let publish = edit(json!({"status": "published", "updated_at": made.updated_at()}));
    let refused = posts.update(made.id(), publish, "k").await;
    let Err(UpdatePostError::Refused(report)) = refused else {
        panic!("published a post the gate blocks: {refused:?}");
    };
    let rules: Vec<Value> = report
        .findings()
        .iter()
        .map(|finding| serde_json::to_value(finding).unwrap()["rule"].clone())
        .collect();
    assert!(rules.contains(&json!("no-author")), "{rules:?}");
    assert!(rules.contains(&json!("no-internal-link")), "{rules:?}");
    assert!(within(site.path(), "drafts/en/a-weak-post.md").is_file());
    assert!(!within(site.path(), "content/en/posts/2026-10-26-a-weak-post.md").exists());
}

#[tokio::test]
async fn a_good_draft_moves_into_content_dated_today() {
    let (site, posts) = demo();
    let made = posts.create(good_draft(), "k", None).await.unwrap();
    let publish = edit(json!({"status": "published", "updated_at": made.updated_at()}));
    let live = posts.update(made.id(), publish, "k").await.unwrap();
    assert_eq!(live.status(), PostStatus::Published);
    let midnight = Utc.with_ymd_and_hms(2026, 10, 26, 0, 0, 0).unwrap();
    assert_eq!(live.published_at(), Some(midnight));
    let file = within(
        site.path(),
        "content/en/posts/2026-10-26-choosing-anchor-text.md",
    );
    assert!(
        fs::read_to_string(file)
            .unwrap()
            .contains("\ndate: 2026-10-26\n")
    );
    assert!(!within(site.path(), "drafts/en/choosing-anchor-text.md").exists());
    let read = posts.get("en.choosing-anchor-text").await.unwrap().unwrap();
    assert_eq!(read.status(), PostStatus::Published);
}

#[tokio::test]
async fn editing_a_published_post_keeps_its_other_fields() {
    let (site, posts) = demo();
    let post = posts
        .get("en.why-internal-links-matter")
        .await
        .unwrap()
        .unwrap();
    let title = "Why internal links matter most";
    let change = edit(json!({"title": title, "updated_at": post.updated_at()}));
    posts.update(post.id(), change, "k").await.unwrap();
    let file = within(
        site.path(),
        "content/en/posts/2026-09-14-why-internal-links-matter.md",
    );
    let text = fs::read_to_string(file).unwrap();
    assert!(
        text.contains("title: \"Why internal links matter most\"\n"),
        "{text}"
    );
    assert!(text.contains("translation: internal-links\n"), "{text}");
    assert!(text.contains("\nAn internal link is a link"), "{text}");
}

#[tokio::test]
async fn a_real_update_writes_today_and_its_note_into_the_file() {
    let (site, posts) = demo();
    let id = "en.why-internal-links-matter";
    let post = posts.get(id).await.unwrap().unwrap();
    assert_eq!(post.updated(), NaiveDate::from_ymd_opt(2026, 10, 20));
    assert_eq!(post.updated_note(), Some("Refreshed the examples."));
    let markdown = format!(
        "{}\n\nOne more example: a link from a guide.",
        post.markdown()
    );
    let note = "Added a guide example.";
    let change = edit(json!({
        "markdown": markdown,
        "real_update": {"note": note},
        "updated_at": post.updated_at(),
    }));
    let saved = posts.update(id, change, "k").await.unwrap();
    assert_eq!(saved.updated(), NaiveDate::from_ymd_opt(2026, 10, 26));
    let file = within(
        site.path(),
        "content/en/posts/2026-09-14-why-internal-links-matter.md",
    );
    let text = fs::read_to_string(file).unwrap();
    assert!(text.contains("\nupdated: 2026-10-26\n"), "{text}");
    assert!(
        text.contains("\nupdated_note: \"Added a guide example.\"\n"),
        "{text}"
    );
    let read = posts.get(id).await.unwrap().unwrap();
    assert_eq!(read.updated_note(), Some(note));
    let title = edit(json!({"title": "Internal links", "updated_at": read.updated_at()}));
    let saved = posts.update(id, title, "k").await.unwrap();
    assert_eq!(saved.updated(), NaiveDate::from_ymd_opt(2026, 10, 26));
}

#[tokio::test]
async fn linting_judges_a_post_and_writes_nothing() {
    let (site, posts) = demo();
    let live = posts.lint("en.how-to-write-a-title-tag").await.unwrap();
    assert!(
        !live.unwrap().blocks(),
        "a published demo post passes the gate"
    );
    assert!(posts.lint("en.no-such-post").await.unwrap().is_none());

    let weak = PostDraft::new("A weak post", None, "No link and no author.");
    posts.create(weak, "k", None).await.unwrap();
    let draft = within(site.path(), "drafts/en/a-weak-post.md");
    let before = fs::read_to_string(&draft).unwrap();
    let report = posts.lint("en.a-weak-post").await.unwrap().unwrap();
    assert!(report.blocks());
    assert_eq!(
        fs::read_to_string(&draft).unwrap(),
        before,
        "the draft is not dated"
    );
}

#[tokio::test]
async fn status_draft_takes_a_live_post_back_into_drafts() {
    let (site, posts) = demo();
    let post = posts
        .get("en.why-internal-links-matter")
        .await
        .unwrap()
        .unwrap();
    let down = edit(json!({"status": "draft", "updated_at": post.updated_at()}));
    let draft = posts.update(post.id(), down, "k").await.unwrap();
    assert_eq!(draft.status(), PostStatus::Draft);
    assert_eq!(draft.published_at(), None);
    let live = "content/en/posts/2026-09-14-why-internal-links-matter.md";
    assert!(!within(site.path(), live).exists());
    let text = fs::read_to_string(within(
        site.path(),
        "drafts/en/why-internal-links-matter.md",
    ));
    assert!(text.unwrap().contains("\nAn internal link is a link"));
    let read = posts.get(post.id()).await.unwrap().unwrap();
    assert_eq!(read.status(), PostStatus::Draft);
}

/// The list `filter` and `order` ask for, every row on one page.
fn query(filter: Option<&str>, order: &str) -> ListQuery {
    let mut raw = khalid_api::query::RawListQuery::default();
    raw.filter = filter.map(str::to_owned);
    raw.order = Some(order.to_owned());
    raw.limit = Some("all".to_owned());
    ListQuery::try_from(&raw).unwrap()
}

#[tokio::test]
async fn the_list_holds_drafts_and_live_posts_with_their_lint_counts() {
    let (_site, posts) = demo();
    let weak = PostDraft::new("A weak post", None, "No link and no author.");
    posts.create(weak, "k", None).await.unwrap();
    posts.create(good_draft(), "k", None).await.unwrap();
    let all = posts.list(&query(None, "id")).await.unwrap();
    let json = serde_json::to_value(all.items()).unwrap();
    let rows = json.as_array().unwrap();
    let row = |id: &str| rows.iter().find(|row| row["id"] == id).unwrap().clone();
    let weak = row("en.a-weak-post");
    assert_eq!(weak["status"], "draft", "{weak}");
    assert_eq!(weak["index_status"], "not_live");
    assert_eq!(weak["gate"], "refuses", "{weak}");
    assert!(weak["lint"]["blocks"].as_u64().unwrap() >= 2, "{weak}");
    let good = row("en.choosing-anchor-text");
    assert_eq!(good["lint"]["blocks"], 0, "a good draft would pass: {good}");
    let live = row("en.why-internal-links-matter");
    assert_eq!(live["status"], "published", "{live}");
    assert_eq!(live["lint"]["blocks"], 0, "{live}");
    assert_eq!(live["index_status"], "indexable", "{live}");
    let live_count = rows
        .iter()
        .filter(|row| row["status"] == "published")
        .count();
    assert_eq!(all.total(), u64::try_from(live_count + 2).unwrap());

    let blocked = posts
        .list(&query(Some("gate:refuses"), "id"))
        .await
        .unwrap();
    let ids: Vec<&str> = blocked.items().iter().map(PostRow::id).collect();
    assert_eq!(ids, ["en.a-weak-post"]);
}

/// The demo's live post with its `slug:` line taken out, as a site owner
/// may write one: `khalid build` names it from its title.
fn without_slug(site: &Path) -> PathBuf {
    let file = within(
        site,
        "content/en/posts/2026-09-14-why-internal-links-matter.md",
    );
    let text = fs::read_to_string(&file).unwrap();
    fs::write(&file, text.replace("slug: why-internal-links-matter\n", "")).unwrap();
    file
}

#[tokio::test]
async fn a_live_post_with_no_slug_is_listed_and_read_by_its_title_slug() {
    let (site, posts) = demo();
    without_slug(site.path());
    let id = "en.why-internal-links-matter";
    let all = posts.list(&query(None, "id")).await.unwrap();
    let ids: Vec<&str> = all.items().iter().map(PostRow::id).collect();
    assert!(ids.contains(&id), "{ids:?}");
    let live = posts.get(id).await.unwrap().unwrap();
    assert_eq!(live.status(), PostStatus::Published);
    assert!(posts.lint(id).await.unwrap().is_some());
}

#[tokio::test]
async fn a_new_title_keeps_the_address_of_a_live_post_with_no_slug() {
    let (site, posts) = demo();
    let file = without_slug(site.path());
    let id = "en.why-internal-links-matter";
    let post = posts.get(id).await.unwrap().unwrap();
    let change = edit(json!({"title": "Internal links", "updated_at": post.updated_at()}));
    let saved = posts.update(id, change, "k").await.unwrap();
    assert_eq!(saved.id(), id);
    let text = fs::read_to_string(file).unwrap();
    assert!(
        text.contains("\nslug: why-internal-links-matter\n"),
        "{text}"
    );
    assert!(posts.get(id).await.unwrap().is_some());
}

/// The draft made from [`good_draft`], edited with `fields`.
async fn edited_draft(posts: &SitePosts, mut fields: Value) -> Result<AdminPost, UpdatePostError> {
    let made = posts.create(good_draft(), "k", None).await.unwrap();
    fields["updated_at"] = json!(made.updated_at());
    posts.update(made.id(), edit(fields), "k").await
}

#[tokio::test]
async fn settings_go_into_the_front_matter_and_read_back() {
    let (site, posts) = demo();
    let settings = json!({
        "date": "2026-11-02",
        "excerpt": "Words that say where a link goes.",
        "featured": true,
        "robots": "noindex",
        "canonical": "https://example.com/anchor-text/",
        "aliases": ["/2019/05/anchor-text/", "/old/"],
    });
    let saved = edited_draft(&posts, json!({"author": "mira", "settings": settings})).await;
    let saved = saved.unwrap();
    let text = fs::read_to_string(within(site.path(), "drafts/en/choosing-anchor-text.md"));
    let text = text.unwrap();
    for line in [
        "author: mira\n",
        "date: 2026-11-02\n",
        "description: \"Words that say where a link goes.\"\n",
        "featured: true\n",
        "robots: \"noindex, follow\"\n",
        "canonical: \"https://example.com/anchor-text/\"\n",
        "aliases: [\"/2019/05/anchor-text/\", \"/old/\"]\n",
    ] {
        assert!(text.contains(line), "{line} in {text}");
    }
    let read = posts.get(saved.id()).await.unwrap().unwrap();
    assert_eq!(read.settings(), saved.settings());
    assert_eq!(
        serde_json::to_value(read.settings()).unwrap()["featured"],
        true
    );
}

#[tokio::test]
async fn co_authors_image_review_and_share_cards_go_into_the_front_matter() {
    let (site, posts) = demo();
    let draft = within(site.path(), "drafts/en/choosing-anchor-text.md");
    for (settings, line) in [
        (json!({"co_authors": ["mira"]}), "co_authors: [mira]\n"),
        (json!({"review_state": "ready"}), "review_state: ready\n"),
        (
            json!({"feature_image": {"src": "/media/a.jpg", "alt": "A link"}}),
            "image: {src: \"/media/a.jpg\", alt: \"A link\"}\n",
        ),
        (
            json!({"share_cards": {"x": {"title": "Anchor text"}}}),
            "share_cards: {x: {title: \"Anchor text\"}}\n",
        ),
    ] {
        let saved = edited_draft(&posts, json!({"settings": settings})).await;
        let saved = saved.unwrap();
        let text = fs::read_to_string(&draft).unwrap();
        assert!(text.contains(line), "{line} in {text}");
        let read = posts.get(saved.id()).await.unwrap().unwrap();
        assert_eq!(read.settings(), saved.settings());
        fs::remove_file(&draft).unwrap();
    }
}

#[tokio::test]
async fn access_password_template_and_head_code_go_into_the_file() {
    let (site, posts) = demo();
    let mut templates = within(site.path(), "theme/templates");
    fs::create_dir_all(&templates).unwrap();
    templates.push("post-wide.html");
    fs::write(&templates, "{% extends \"post.html\" %}").unwrap();
    let code = "<script>\n// a comment\ngo(\"x\");\n</script>";
    let settings = json!({
        "access": "members",
        "password": "generated-for-this-test",
        "template": "wide",
        "head_code": code,
    });
    let made = posts.create(good_draft(), "k", None).await.unwrap();
    let mut change = edit(json!({"settings": settings, "updated_at": made.updated_at()}));
    change.hash_password().unwrap();
    let saved = posts.update(made.id(), change, "k").await.unwrap();
    let file = within(site.path(), "drafts/en/choosing-anchor-text.md");
    let text = fs::read_to_string(&file).unwrap();
    for line in [
        "access: members\n",
        "password_hash: \"$argon2id$",
        "template: wide\n",
        "head_code: \"<script>\\n// a comment\\ngo(\\\"x\\\");\\n</script>\"\n",
    ] {
        assert!(text.contains(line), "{line} in {text}");
    }
    assert!(!text.contains("generated-for-this-test"), "{text}");
    let read = posts.get(saved.id()).await.unwrap().unwrap();
    assert_eq!(read.settings(), saved.settings());
    assert_eq!(
        read.settings().head_code.as_ref().unwrap().to_string(),
        code
    );

    let clear = json!({"access": "public", "password": null, "template": null, "head_code": null});
    let mut change = edit(json!({"settings": clear, "updated_at": read.updated_at()}));
    change.hash_password().unwrap();
    posts.update(read.id(), change, "k").await.unwrap();
    let text = fs::read_to_string(&file).unwrap();
    for field in ["access:", "password_hash:", "template:", "head_code:"] {
        assert!(!text.contains(field), "{field} in {text}");
    }
}

#[tokio::test]
async fn a_template_the_theme_lacks_is_refused() {
    let (_site, posts) = demo();
    let refused = edited_draft(&posts, json!({"settings": {"template": "wide"}})).await;
    assert!(
        matches!(
            refused,
            Err(UpdatePostError::Unsupported {
                setting: "template",
                ..
            })
        ),
        "{refused:?}"
    );
}

#[tokio::test]
async fn category_and_tags_go_into_the_file_and_filter_the_list() {
    let (site, posts) = demo();
    let settings = json!({"category": "on-page-seo", "tags": ["anchor text", "Internal links"]});
    let saved = edited_draft(&posts, json!({"settings": settings})).await;
    let saved = saved.unwrap();
    let file = within(site.path(), "drafts/en/choosing-anchor-text.md");
    let text = fs::read_to_string(&file).unwrap();
    for line in [
        "category: on-page-seo\n",
        "tags: [\"anchor text\", \"Internal links\"]\n",
    ] {
        assert!(text.contains(line), "{line} in {text}");
    }
    let read = posts.get(saved.id()).await.unwrap().unwrap();
    assert_eq!(read.settings(), saved.settings());
    for filter in ["category:on-page-seo", "tag:'anchor text'"] {
        let found = posts.list(&query(Some(filter), "id")).await.unwrap();
        let ids: Vec<&str> = found.items().iter().map(PostRow::id).collect();
        assert!(ids.contains(&saved.id()), "{filter}: {ids:?}");
    }

    let clear =
        json!({"settings": {"category": null, "tags": []}, "updated_at": read.updated_at()});
    posts.update(read.id(), edit(clear), "k").await.unwrap();
    let text = fs::read_to_string(&file).unwrap();
    for field in ["category:", "tags:"] {
        assert!(!text.contains(field), "{field} in {text}");
    }
}

#[tokio::test]
async fn a_category_the_site_lacks_is_refused() {
    let (_site, posts) = demo();
    let refused = edited_draft(&posts, json!({"settings": {"category": "recipes"}})).await;
    assert!(
        matches!(
            refused,
            Err(UpdatePostError::Unsupported {
                setting: "category",
                ..
            })
        ),
        "{refused:?}"
    );
}

#[tokio::test]
async fn a_new_slug_moves_a_draft_to_its_new_file() {
    let (site, posts) = demo();
    let moved = edited_draft(&posts, json!({"slug": "anchor-text-guide"})).await;
    let moved = moved.unwrap();
    assert_eq!(moved.id(), "en.anchor-text-guide");
    assert!(!within(site.path(), "drafts/en/choosing-anchor-text.md").exists());
    let text = fs::read_to_string(within(site.path(), "drafts/en/anchor-text-guide.md"));
    assert!(text.unwrap().contains("\nslug: anchor-text-guide\n"));

    let taken = edited_draft(&posts, json!({"slug": "anchor-text-guide"})).await;
    assert!(
        matches!(taken, Err(UpdatePostError::SlugTaken(_))),
        "{taken:?}"
    );
    let live = posts
        .get("en.why-internal-links-matter")
        .await
        .unwrap()
        .unwrap();
    let change = edit(json!({"settings": {"date": null}, "updated_at": live.updated_at()}));
    let refused = posts.update(live.id(), change, "k").await;
    assert!(matches!(
        refused,
        Err(UpdatePostError::Unsupported {
            setting: "date",
            ..
        })
    ));
}

#[tokio::test]
async fn the_last_editor_is_read_back_until_the_file_changes_by_hand() {
    let (site, posts) = demo();
    let made = posts.create(good_draft(), "maria", None).await.unwrap();
    let read = posts.get(made.id()).await.unwrap().unwrap();
    assert_eq!(read.updated_by(), Some("maria"), "the maker is the editor");

    let retitle = edit(json!({"title": "Picking anchor text", "updated_at": made.updated_at()}));
    let saved = posts.update(made.id(), retitle, "asif").await.unwrap();
    let live = edit(json!({"status": "published", "updated_at": saved.updated_at()}));
    let live = posts.update(made.id(), live, "scheduler").await.unwrap();
    let read = posts.get(made.id()).await.unwrap().unwrap();
    assert_eq!(read.updated_by(), Some("scheduler"), "kept at its new file");
    assert_eq!(row_of(&posts, made.id()).await["updated_by"], "scheduler");
    let stale = edit(json!({"title": "Old copy", "updated_at": saved.updated_at()}));
    let refused = posts.update(made.id(), stale, "asif").await;
    assert!(
        matches!(&refused, Err(UpdatePostError::Stale { updated_by: Some(by), .. }) if by == "scheduler"),
        "a stale edit names who changed it: {refused:?}"
    );

    let file = within(
        site.path(),
        "content/en/posts/2026-10-26-choosing-anchor-text.md",
    );
    let text = fs::read_to_string(&file).unwrap();
    fs::write(&file, text.replace("Picking", "Choosing")).unwrap();
    let later = std::time::SystemTime::from(live.updated_at()) + std::time::Duration::from_secs(5);
    fs::File::options()
        .write(true)
        .open(&file)
        .unwrap()
        .set_modified(later)
        .unwrap();
    let read = posts.get(made.id()).await.unwrap().unwrap();
    assert_eq!(
        read.updated_by(),
        None,
        "a change by hand has no known editor"
    );
    assert_eq!(row_of(&posts, made.id()).await["updated_by"], Value::Null);
}

/// The list row of post `id`, as JSON.
async fn row_of(posts: &SitePosts, id: &str) -> Value {
    let rows = posts.list(&query(None, "id")).await.unwrap();
    let row = rows.items().iter().find(|row| row.id() == id).unwrap();
    serde_json::to_value(row).unwrap()
}

/// The aliases of `post`, as paths.
fn aliases_of(post: &AdminPost) -> Vec<&str> {
    post.settings().aliases.iter().map(Alias::as_str).collect()
}

#[tokio::test]
async fn a_live_post_takes_a_new_slug_and_its_old_address_redirects() {
    let (site, posts) = demo();
    let file = "content/en/posts/2026-09-14-why-internal-links-matter.md";
    let file = within(site.path(), file);
    let live = posts.get("en.why-internal-links-matter").await.unwrap();
    let live = live.unwrap();
    let change = edit(json!({"slug": "internal-links", "updated_at": live.updated_at()}));
    let moved = posts.update(live.id(), change, "k").await.unwrap();
    assert_eq!(moved.id(), "en.internal-links");
    assert_eq!(moved.status(), PostStatus::Published);
    assert_eq!(aliases_of(&moved), ["/blog/why-internal-links-matter/"]);
    let text = fs::read_to_string(&file).unwrap();
    assert!(text.contains("\nslug: internal-links\n"), "{text}");
    assert!(
        text.contains("\"/blog/why-internal-links-matter/\""),
        "{text}"
    );
    let gone = posts.get("en.why-internal-links-matter").await.unwrap();
    assert!(gone.is_none());

    let back = json!({"slug": "why-internal-links-matter", "updated_at": moved.updated_at()});
    let back = posts.update(moved.id(), edit(back), "k").await.unwrap();
    assert_eq!(aliases_of(&back), ["/blog/internal-links/"]);

    let taken = json!({"slug": "choosing-anchor-text", "updated_at": back.updated_at()});
    posts.create(good_draft(), "k", None).await.unwrap();
    let taken = posts.update(back.id(), edit(taken), "k").await;
    assert!(
        matches!(taken, Err(UpdatePostError::SlugTaken(_))),
        "{taken:?}"
    );
}

#[test]
fn a_trash_key_names_one_folder_inside_trash_and_never_climbs() {
    let (_site, posts) = demo();
    let names = |key: &str| posts.0.names_one_copy(key);
    assert!(names("20261031T093000Z-en.hello"));
    for climbing in ["..", "20261031T093000Z-x/../..", "a/b", "/etc", ""] {
        assert!(!names(climbing), "{climbing}");
    }
}

#[tokio::test]
async fn the_trash_keeps_copies_trash_days_long_or_for_good_at_0() {
    let (site, posts) = demo();
    let put = |key: &str| {
        let file = within(site.path(), &format!("trash/{key}/drafts/en/old.md"));
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, "---\ntitle: Old\nslug: old\n---\n\nWords.\n").unwrap();
    };
    // The clock says 2026-10-26 09:00, so 30 days back is 2026-09-26 09:00.
    put("20260926T085959Z-en.old");
    put("20260926T090001Z-en.kept");
    let emptied = posts.empty_old_trash().await.unwrap();
    assert_eq!(emptied, ["20260926T085959Z-en.old"]);
    assert!(within(site.path(), "trash/20260926T090001Z-en.kept").is_dir());

    let toml = within(site.path(), "site.toml");
    let text = fs::read_to_string(&toml).unwrap();
    fs::write(&toml, format!("trash_days = 0\n{text}")).unwrap();
    let config = Config::load(site.path(), &Environment::empty()).unwrap();
    let clock = FixedClock::new(Utc.with_ymd_and_hms(2036, 1, 1, 0, 0, 0).unwrap());
    let forever = SitePosts::new(site.path(), config, clock);
    assert_eq!(
        forever.empty_old_trash().await.unwrap(),
        Vec::<String>::new()
    );
    assert!(within(site.path(), "trash/20260926T090001Z-en.kept").is_dir());
}
