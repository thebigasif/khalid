//! A post saved while a starting server still renders the rest of its site
//! shows at once and stays shown: the start render keeps it on top, and a
//! full render of the save is never thrown away for the start render's
//! sake, only for a newer save's.

use std::sync::Arc;

use khalid_server::{LivePages, MediaType, Page, PagePath, Pages, Render, Rendered};

const POST: &str = "/blog/what-a-sitemap-does/";

#[cfg(test)]
fn page(path: &str, body: &'static str) -> Page {
    Page::new(PagePath::try_new(path).unwrap(), MediaType::Html, body)
}

#[cfg(test)]
fn home() -> Page {
    page("/blog/", "<h1>Generated Field Notes</h1>")
}

#[cfg(test)]
fn whole(post: &'static str) -> Pages {
    Pages::new(vec![home(), page(POST, post)], "<h1>Page not found</h1>").unwrap()
}

#[cfg(test)]
fn starting() -> LivePages {
    LivePages::new(Pages::first(vec![home()]).unwrap())
}

#[cfg(test)]
fn body(live: &LivePages, path: &str) -> String {
    let pages = live.current();
    let page = pages.get(path).expect("the page is served");
    String::from_utf8(page.body().unwrap().to_vec()).unwrap()
}

/// Renders the post as `post`, and while it renders does `meanwhile` to
/// the pages being served, as the start render or a newer save would.
#[derive(Debug)]
struct Beside {
    live: LivePages,
    meanwhile: fn(&LivePages),
    post: &'static str,
}

#[cfg(test)]
impl Render for Beside {
    type Error = std::io::Error;

    fn render(&self) -> Result<Rendered, Self::Error> {
        (self.meanwhile)(&self.live);
        let pages = vec![home(), page(POST, self.post)];
        Ok(Rendered::new(pages, "<h1>Page not found</h1>"))
    }
}

#[test]
fn a_page_saved_during_the_start_stays_when_the_rest_arrives() {
    let live = starting();
    live.show_saved(vec![page(POST, "<p>Saved during the start.</p>")]);
    assert!(live.serve_whole(whole("<p>As the start read it.</p>")));
    assert!(live.current().is_whole(), "the rest of the site is served");
    assert_eq!(body(&live, POST), "<p>Saved during the start.</p>");
    assert_eq!(body(&live, "/blog/"), "<h1>Generated Field Notes</h1>");
}

#[test]
fn the_start_render_never_replaces_a_whole_set() {
    let live = starting();
    let first = live.current();
    assert!(live.replace_if_current(&first, whole("<p>Newer.</p>")));
    assert!(
        !live.serve_whole(whole("<p>As the start read it.</p>")),
        "a whole set read after the start is newer"
    );
    assert_eq!(body(&live, POST), "<p>Newer.</p>");
}

#[tokio::test]
async fn a_full_render_lands_although_the_start_render_finished_beside_it() {
    let live = starting();
    let renderer = Arc::new(Beside {
        live: live.clone(),
        meanwhile: |live| {
            live.serve_whole(whole("<p>As the start read it.</p>"));
        },
        post: "<p>Edited.</p>",
    });
    let served = live.refresh_unless_saved(&renderer).await.unwrap();
    assert!(served, "the start render is older than the edit");
    assert_eq!(body(&live, POST), "<p>Edited.</p>");
}

#[tokio::test]
async fn a_full_render_is_stale_once_a_newer_save_shows() {
    let live = starting();
    let renderer = Arc::new(Beside {
        live: live.clone(),
        meanwhile: |live| live.show_saved(vec![page(POST, "<p>Saved again.</p>")]),
        post: "<p>Edited.</p>",
    });
    let served = live.refresh_unless_saved(&renderer).await.unwrap();
    assert!(!served, "the newer save wins, and the render runs again");
    assert_eq!(body(&live, POST), "<p>Saved again.</p>");
}
