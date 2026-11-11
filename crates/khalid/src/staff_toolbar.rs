//! The toolbar on public pages for signed-in staff (plan section 4.6,
//! KH-EDT-6): on a post, a link to edit it in the admin, what the linter
//! says about it and how often it was read in the last 30 days.
//!
//! The bar is added on the server, so it needs no script. Only a request
//! with a live staff session gets it, and that answer is marked private
//! and never stored, so no cache hands one person's bar to a reader. A
//! reader's request passes through untouched.
//!
//! The lint result is kept for 5 minutes, or until the post is saved, so
//! moving between pages does not run the linter on every view.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, PoisonError};

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::extract::{Request, State};
use axum::http::header::{
    ACCEPT_ENCODING, CACHE_CONTROL, CONTENT_LENGTH, CONTENT_TYPE, COOKIE, ETAG, IF_NONE_MATCH, VARY,
};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Duration, TimeDelta, Utc};
use khalid_analytics::AnalyticsStore;
use khalid_api::content_store::{ContentStore, PostKey};
use khalid_api::post_store::PostStore;
use khalid_api::resources::{Collection, LintCounts};
use khalid_auth::{Accounts, SessionToken};
use khalid_core::{Clock, Config, Slug, SystemClock};
use khalid_db::{AnalyticsTable, Database, SiteId};

use crate::assets::texts;
use crate::site_content::LiveContent;
use crate::site_posts::SitePosts;
use crate::staff;

/// The toolbar's styles. A file of its own, since the pages' policy allows
/// only stylesheets from the site, and only staff ever load it. The binary
/// packs `staff_toolbar.css` with its other whole texts.
const STYLES: &str = "src/staff_toolbar.css";

/// The largest page the bar is added to.
const PAGE_LIMIT: usize = 4 * 1024 * 1024;

/// How long the bar reuses a post's lint result while the post is not
/// saved again. Other posts and the settings can change the answer too,
/// such as a link to a post that was deleted, so it is judged again after.
const LINT_FRESH: TimeDelta = TimeDelta::minutes(5);

/// What the bar reads: who is signed in, the posts and their numbers.
struct Toolbar {
    accounts: Accounts,
    content: Arc<dyn ContentStore>,
    posts: SitePosts,
    views: Arc<dyn AnalyticsStore>,
    clock: Arc<dyn Clock>,
    /// The mount folder, such as `/blog/`.
    mount: String,
    /// The last lint result of each post.
    lints: LintMemo,
    /// The admin's path, such as `/blog/admin`.
    admin: String,
}

/// `router`, the site's pages, with the bar on each page staff of site
/// `id` ask for, and the bar's stylesheet at `<mount>_toolbar.css`.
pub(crate) fn start(
    (database, id, config): (&Database, SiteId, &Config),
    (content, posts): (LiveContent, SitePosts),
    router: Router,
) -> Router {
    let toolbar = Toolbar {
        accounts: staff::accounts(database, id, config, None),
        content: Arc::new(content),
        posts,
        views: Arc::new(AnalyticsTable::new(database.clone(), id)),
        clock: Arc::new(SystemClock),
        mount: config.mount().to_owned(),
        lints: LintMemo::default(),
        admin: staff::admin_path(config),
    };
    router.layer(middleware::from_fn_with_state(Arc::new(toolbar), show))
}

async fn show(State(toolbar): State<Arc<Toolbar>>, mut request: Request, next: Next) -> Response {
    let path = request.uri().path().to_owned();
    if path == toolbar.stylesheet() {
        let headers = [(CONTENT_TYPE, "text/css; charset=utf-8")];
        return (headers, texts::text(STYLES)).into_response();
    }
    let token = request
        .headers()
        .get(COOKIE)
        .and_then(|value| value.to_str().ok());
    let token = token.and_then(SessionToken::from_cookie_header);
    if request.method() != Method::GET || !toolbar.signed_in(token).await {
        return next.run(request).await;
    }
    // The bar is added to the page's own bytes, so ask for them plain and
    // whole: no compressed copy and no "not modified".
    request.headers_mut().remove(ACCEPT_ENCODING);
    request.headers_mut().remove(IF_NONE_MATCH);
    let response = next.run(request).await;
    let html = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|media| media.starts_with("text/html"));
    if response.status() != StatusCode::OK || !html {
        return response;
    }
    let (mut parts, body) = response.into_parts();
    let Ok(page) = to_bytes(body, PAGE_LIMIT).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let page = String::from_utf8_lossy(&page);
    let bar = toolbar.bar(&path).await;
    let shown = match page.rfind("</body>") {
        Some(end) => {
            let (head, tail) = page.split_at(end);
            format!("{head}{bar}{tail}")
        }
        None => format!("{page}{bar}"),
    };
    parts.headers.remove(ETAG);
    parts.headers.remove(CONTENT_LENGTH);
    let private = HeaderValue::from_static("private, no-store");
    parts.headers.insert(CACHE_CONTROL, private);
    parts
        .headers
        .insert(VARY, HeaderValue::from_static("cookie"));
    Response::from_parts(parts, Body::from(shown))
}

impl Toolbar {
    fn stylesheet(&self) -> String {
        format!("{}_toolbar.css", self.mount)
    }

    /// Whether `token`, from the request's cookie, is a live staff session.
    async fn signed_in(&self, token: Option<SessionToken>) -> bool {
        match token {
            Some(token) => self.accounts.session(&token).await.is_ok(),
            None => false,
        }
    }

    /// The identity of the published post at `path`, as the Admin API
    /// names it, such as `en.hello`.
    async fn post_at(&self, path: &str) -> Option<String> {
        let last = path.trim_end_matches('/').rsplit('/').next()?;
        let slug = Slug::try_new_unicode(last).ok()?;
        let key = PostKey::Slug(slug);
        let found = self.content.find(Collection::Posts, &key);
        let post = found.await.ok().flatten()?;
        let here = post.url().as_str().ends_with(path);
        here.then(|| format!("{}.{}", post.language(), post.slug()))
    }

    /// The bar for the page at `path`.
    async fn bar(&self, path: &str) -> String {
        let admin = &self.admin;
        let opening = format!(
            "<link rel=\"stylesheet\" href=\"{}\"><aside class=\"khalid-toolbar\" \
             aria-label=\"Staff toolbar\"><p>You are signed in. Readers do not see this bar.</p>",
            self.stylesheet()
        );
        let Some(id) = self.post_at(path).await else {
            return format!("{opening}<a href=\"{admin}/\">Open the admin</a></aside>");
        };
        let lint = self.lint_of(&id).await;
        let views = self.views_of(path).await;
        let id = escaped(&id);
        format!(
            "{opening}<a href=\"{admin}/posts/{id}\">Edit this post</a>\
             <p>{lint}</p><p>{views}</p></aside>"
        )
    }

    /// What the linter says about post `id`, in words: remembered while
    /// the post is unsaved and the answer fresh, or else judged now.
    async fn lint_of(&self, id: &str) -> String {
        let now = self.clock.now();
        let saved = self.posts.get(id).await.ok().flatten();
        let saved = saved.map(|post| post.updated_at());
        if let Some(saved) = saved
            && let Some(words) = self.lints.recall(id, saved, now)
        {
            return words;
        }
        let Ok(Some(report)) = self.posts.lint(id).await else {
            return "The linter could not check this post just now.".to_owned();
        };
        let words = lint_words(LintCounts::of(&report));
        if let Some(saved) = saved {
            self.lints.keep(id, saved, now, &words);
        }
        words
    }

    /// How often the page at `path` was read in the last 30 days, in words.
    async fn views_of(&self, path: &str) -> String {
        let today = self.clock.now().date_naive();
        let first = today - Duration::days(29);
        match self.views.views(first, today).await {
            Ok(rows) => {
                let read: u64 = rows
                    .iter()
                    .filter(|r| r.path == path)
                    .map(|r| r.views)
                    .sum();
                let times = if read == 1 { "time" } else { "times" };
                format!("Read {read} {times} in the last 30 days.")
            }
            Err(_) => "The numbers could not be read just now.".to_owned(),
        }
    }
}

/// The bar's last lint result for each post, so moving between pages does
/// not run the linter on every view. It holds one short line per post the
/// staff looked at.
#[derive(Debug, Default)]
struct LintMemo(Mutex<BTreeMap<String, Remembered>>);

/// One post's lint result and when it was found.
#[derive(Debug)]
struct Remembered {
    /// When the post's file was last saved.
    saved: DateTime<Utc>,
    /// When the linter judged it.
    at: DateTime<Utc>,
    /// What it found, in words.
    words: String,
}

impl LintMemo {
    /// What the linter said about post `id` as saved at `saved`, while it
    /// is less than [`LINT_FRESH`] old at `now`.
    fn recall(&self, id: &str, saved: DateTime<Utc>, now: DateTime<Utc>) -> Option<String> {
        let memo = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let kept = memo.get(id)?;
        let fresh = kept.saved == saved && now.signed_duration_since(kept.at) < LINT_FRESH;
        fresh.then(|| kept.words.clone())
    }

    /// Keeps `words`, what the linter said at `now` about post `id` as
    /// saved at `saved`.
    fn keep(&self, id: &str, saved: DateTime<Utc>, now: DateTime<Utc>, words: &str) {
        let mut memo = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let words = words.to_owned();
        memo.insert(
            id.to_owned(),
            Remembered {
                saved,
                at: now,
                words,
            },
        );
    }
}

/// What the linter found, in words.
fn lint_words(counts: LintCounts) -> String {
    let warnings = match counts.warnings() {
        0 => String::new(),
        1 => ", with 1 warning".to_owned(),
        n => format!(", with {n} warnings"),
    };
    match counts.blocks() {
        0 => format!("Passes the lint check{warnings}."),
        1 => format!("1 problem would stop this post going live{warnings}."),
        n => format!("{n} problems would stop this post going live{warnings}."),
    }
}

/// `text` with the characters HTML gives meaning to escaped.
fn escaped(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn the_lint_result_reads_as_a_sentence() {
        let said = |blocks, warnings| lint_words(LintCounts::new(blocks, warnings));
        assert_eq!(said(0, 0), "Passes the lint check.");
        assert_eq!(said(0, 1), "Passes the lint check, with 1 warning.");
        assert_eq!(said(1, 0), "1 problem would stop this post going live.");
        let both = "2 problems would stop this post going live, with 3 warnings.";
        assert_eq!(said(2, 3), both);
        assert_eq!(escaped("en.a<b>&\"'"), "en.a&lt;b&gt;&amp;&quot;&#39;");
    }

    #[test]
    fn a_lint_result_is_reused_until_the_post_changes_or_5_minutes_pass() {
        let at = |minute, second| {
            Utc.with_ymd_and_hms(2026, 11, 10, 9, minute, second)
                .unwrap()
        };
        let memo = LintMemo::default();
        let saved = at(0, 0);
        assert_eq!(memo.recall("en.a", saved, at(1, 0)), None);
        memo.keep("en.a", saved, at(1, 0), "Passes the lint check.");
        let kept = Some("Passes the lint check.".to_owned());
        assert_eq!(memo.recall("en.a", saved, at(5, 59)), kept);
        assert_eq!(memo.recall("en.b", saved, at(2, 0)), None, "another post");
        assert_eq!(memo.recall("en.a", at(3, 0), at(3, 0)), None, "saved again");
        assert_eq!(memo.recall("en.a", saved, at(6, 0)), None, "5 minutes on");
    }
}
