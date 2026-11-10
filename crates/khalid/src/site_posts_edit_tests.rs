//! The Admin API's posts on a copy of the demo site: a post's category,
//! tags and slug, and who last changed it.

use super::*;

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
