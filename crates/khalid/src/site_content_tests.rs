//! The Content API's store on a running site, checked on the demo site.

use std::time::Duration;

use khalid_api::query::RawListQuery;
use khalid_content::{ContentTree, read_content};
use khalid_core::Environment;

use super::*;

fn demo() -> (PathBuf, Config, Catalog) {
    let mut folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    folder.push("../../fixtures/demo-site");
    let config = Config::load(&folder, &Environment::empty()).unwrap();
    let content = read_content(&ContentTree::walk(&folder).unwrap()).unwrap();
    let catalog = Catalog::of(&folder, &config, &content).unwrap();
    (folder, config, catalog)
}

fn english() -> ListQuery {
    let mut raw = RawListQuery::default();
    raw.filter = Some("language:en".to_owned());
    ListQuery::try_from(&raw).unwrap()
}

#[tokio::test]
async fn the_demo_posts_come_back_from_the_index_with_their_bodies() {
    let (folder, config, catalog) = demo();
    let live = LiveContent::default();
    live.replace(Published::of(&folder, &config, &catalog).unwrap());

    let listed = live.list(Collection::Posts, &english()).await.unwrap();
    let json = serde_json::to_value(listed.items()).unwrap();
    assert_eq!(listed.total(), 5, "{json}");
    let first = &json[0];
    assert_eq!(first["id"], "en.planning-a-topic-cluster");
    assert_eq!(
        first["url"],
        "https://www.example.com/blog/planning-a-topic-cluster/"
    );
    assert_eq!(first["published_at"], "2026-10-12T00:00:00Z");
    assert!(first["html"].as_str().unwrap().contains("<p>"), "{first}");
    assert!(
        !first["plaintext"].as_str().unwrap().contains('<'),
        "{first}"
    );

    let german = PostKey::Id("de.warum-interne-links-wichtig-sind".to_owned());
    let found = live
        .find(Collection::Posts, &german)
        .await
        .unwrap()
        .unwrap();
    let found = serde_json::to_value(found).unwrap();
    assert_eq!(
        found["url"],
        "https://www.example.com/blog/de/warum-interne-links-wichtig-sind/"
    );
    assert!(found["markdown"].as_str().unwrap().len() > 10, "{found}");
}

#[tokio::test]
async fn the_store_holds_no_body_until_one_is_asked_for() {
    let (folder, config, catalog) = demo();
    let published = Published::of(&folder, &config, &catalog).unwrap();
    let held = published.store.list(Collection::Posts, &english()).await;
    let held = serde_json::to_value(held.unwrap().items()).unwrap();
    assert!(held[0].get("html").is_none(), "{held}");
    assert!(held[0].get("markdown").is_none(), "{held}");
}

#[tokio::test]
async fn a_change_makes_only_the_posts_it_names_again() {
    let (folder, config, mut catalog) = demo();
    let before = Published::of(&folder, &config, &catalog).unwrap();
    let mut changes = PostChanges::default();
    for row in &mut catalog.index.posts {
        row.title = Some(format!("Retitled {}", row.slug));
    }
    let edited = "content/en/posts/2026-09-28-what-a-sitemap-does.md";
    changes.written.insert(edited.to_owned());
    changes
        .removed
        .insert("content/en/posts/2026-10-12-planning-a-topic-cluster.md".to_owned());
    catalog
        .index
        .posts
        .retain(|row| !row.path.contains("planning"));
    let after = before.changed(&config, &catalog, &changes).unwrap();

    let listed = after.store.list(Collection::Posts, &english()).await;
    let titles: Vec<String> = serde_json::to_value(listed.unwrap().items())
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|post| post["title"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(titles.len(), 4, "{titles:?}");
    let retitled: Vec<&String> = titles
        .iter()
        .filter(|t| t.starts_with("Retitled"))
        .collect();
    assert_eq!(retitled, ["Retitled what-a-sitemap-does"], "{titles:?}");
}

#[tokio::test]
async fn a_body_whose_file_is_gone_is_an_error_not_an_empty_post() {
    let (_, config, catalog) = demo();
    let elsewhere = tempfile::tempdir().unwrap();
    let live = LiveContent::default();
    live.replace(Published::of(elsewhere.path(), &config, &catalog).unwrap());
    assert!(live.list(Collection::Posts, &english()).await.is_err());
    let german = PostKey::Id("de.warum-interne-links-wichtig-sind".to_owned());
    assert!(live.find(Collection::Posts, &german).await.is_err());
}

#[tokio::test]
async fn a_request_waits_for_the_first_store_instead_of_answering_empty() {
    let (folder, config, catalog) = demo();
    let live = LiveContent::default();
    let asking = live.clone();
    let answer = tokio::spawn(async move {
        let listed = asking.list(Collection::Posts, &english()).await;
        listed.map(|listed| listed.total())
    });
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(!answer.is_finished(), "answered before the store was ready");
    live.replace(Published::of(&folder, &config, &catalog).unwrap());
    assert_eq!(answer.await.unwrap().unwrap(), 5);
}

#[tokio::test]
async fn a_store_that_never_loaded_answers_an_error_but_a_served_one_stays() {
    let (folder, config, catalog) = demo();
    let live = LiveContent::default();
    live.fail();
    assert!(live.list(Collection::Posts, &english()).await.is_err());
    live.replace(Published::of(&folder, &config, &catalog).unwrap());
    live.fail();
    let listed = live.list(Collection::Posts, &english()).await.unwrap();
    assert_eq!(listed.total(), 5);
}

#[tokio::test]
async fn a_post_in_a_series_names_it_in_the_content_api() {
    let (folder, config, _) = demo();
    let mut content = read_content(&ContentTree::walk(&folder).unwrap()).unwrap();
    let text = "---\nseries:\n  name: Topic clusters from scratch\n  order: 2\n---\n";
    let part = parse_post(Path::new("a.md"), text).unwrap().front.series;
    content
        .posts
        .iter_mut()
        .find(|doc| doc.path.ends_with("2026-10-12-planning-a-topic-cluster.md"))
        .unwrap()
        .front
        .series = part;
    let catalog = Catalog::of(&folder, &config, &content).unwrap();
    let published = Published::of(&folder, &config, &catalog).unwrap();
    let listed = published.store.list(Collection::Posts, &english()).await;
    let json = serde_json::to_value(listed.unwrap().items()).unwrap();
    assert_eq!(
        json[0]["series"]["slug"], "topic-clusters-from-scratch",
        "{json}"
    );
    assert_eq!(json[0]["series"]["order"], 2);
    assert!(json[1].get("series").is_none(), "a post in no series");
}

#[test]
fn plaintext_drops_tags_and_reads_entities() {
    assert_eq!(
        plaintext("<p>Links &amp; <a href=\"/x/\">anchors</a></p>\n<p>&lt;3</p>"),
        "Links & anchors <3"
    );
}
