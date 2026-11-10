//! Git mode's push hook on `khalid serve`: a git host that was pushed to
//! asks the blog to pull, and the pages are rendered again from what came
//! in (plan section 4.6, ADR "git mode: a branch per draft").

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError};

use axum::Router;
use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::post;
use hmac::{Hmac, KeyInit, Mac};
use khalid_core::Secret;
use khalid_server::{LivePages, Render};
use sha2::Sha256;
use subtle::ConstantTimeEq;

use crate::pull;

/// Where the hook answers, beside the APIs.
pub(crate) const HOOK_PATH: &str = "/api/hooks/git";

/// GitHub's header: `sha256=` and the HMAC-SHA256 of the body, in hex.
const GITHUB_SIGNATURE: &str = "x-hub-signature-256";

/// GitLab's header: the secret itself.
const GITLAB_TOKEN: &str = "x-gitlab-token";

/// What the hook needs: the site folder, the secret, a lock so two pulls
/// never run at once, and the pages to render again after a pull.
#[derive(Clone)]
struct Hook<R> {
    site: PathBuf,
    secret: Arc<Secret<String>>,
    pulling: Arc<Mutex<()>>,
    shown: (R, LivePages),
}

/// The push hook for the site in `site`, or no route at all when the
/// environment holds no `secret`. A pull that moved files renders the
/// `shown` pages again with their renderer.
pub(crate) fn router<R: Render + Clone>(
    site: &Path,
    secret: Option<Secret<String>>,
    shown: &(R, LivePages),
) -> Router {
    let Some(secret) = secret else {
        return Router::new();
    };
    let hook = Hook {
        site: site.to_path_buf(),
        secret: Arc::new(secret),
        pulling: Arc::new(Mutex::new(())),
        shown: shown.clone(),
    };
    Router::new()
        .route(HOOK_PATH, post(pushed::<R>))
        .with_state(hook)
}

/// Answers a signed push with 202 and pulls in the background, since a host
/// waits about ten seconds for an answer. Anything unsigned is a 401.
async fn pushed<R: Render + Clone>(
    State(hook): State<Hook<R>>,
    headers: HeaderMap,
    body: Bytes,
) -> (StatusCode, &'static str) {
    if !signed(&headers, &body, hook.secret.expose()) {
        return (
            StatusCode::UNAUTHORIZED,
            "The push was not signed with KHALID_GIT_HOOK_SECRET. Set the same secret on the git host's webhook.\n",
        );
    }
    tokio::spawn(pull_and_show(hook));
    (StatusCode::ACCEPTED, "Pulling.\n")
}

/// Pulls, then renders the pages again when the pull moved files. The file
/// watch sees them too, but not when the pull lands before the watch is
/// ready, so the hook does not wait for it (backlog row 33).
async fn pull_and_show<R: Render + Clone>(hook: Hook<R>) {
    let (site, pulling) = (hook.site.clone(), Arc::clone(&hook.pulling));
    let pulled = tokio::task::spawn_blocking(move || {
        let _one_at_a_time = pulling.lock().unwrap_or_else(PoisonError::into_inner);
        pull::pulled(&site)
    })
    .await;
    // Git's own words may hold the remote's address with a token in it, so
    // a failure only says where to look (BL-RUN-5).
    let Ok(Ok(moved)) = pulled else {
        return tracing::warn!(
            "the push hook could not pull; run khalid pull in the site folder to see why"
        );
    };
    tracing::info!(files = moved.files, "the push hook pulled");
    if moved.files == 0 {
        return;
    }
    // A save shown while this renders is newer than what it read, so it
    // stays, and the watch renders the pull after it.
    let (renderer, live) = &hook.shown;
    if let Err(error) = live.refresh_unless_saved(renderer).await {
        tracing::warn!(
            ?error,
            "the push hook pulled but the pages could not be rendered, so readers keep the last good pages"
        );
    }
}

/// Whether `headers` sign `body` with `secret`, the way GitHub or GitLab
/// does, compared in constant time.
fn signed(headers: &HeaderMap, body: &[u8], secret: &str) -> bool {
    let header = |name| headers.get(name).and_then(|value| value.to_str().ok());
    if let Some(signature) = header(GITHUB_SIGNATURE) {
        let Some(hex) = signature.strip_prefix("sha256=") else {
            return false;
        };
        let (Some(sent), Ok(mut mac)) = (
            bytes(hex),
            Hmac::<Sha256>::new_from_slice(secret.as_bytes()),
        ) else {
            return false;
        };
        mac.update(body);
        return mac.verify_slice(&sent).is_ok();
    }
    header(GITLAB_TOKEN).is_some_and(|token| bool::from(token.as_bytes().ct_eq(secret.as_bytes())))
}

/// The bytes `hex` spells, or `None` when it is not hex.
fn bytes(hex: &str) -> Option<Vec<u8>> {
    let digits = hex.as_bytes();
    if !digits.len().is_multiple_of(2) {
        return None;
    }
    digits
        .chunks(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).ok()?, 16).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use axum::http::Request;
    use khalid_server::{Pages, Rendered};
    use tower::ServiceExt;

    use super::*;

    fn github(body: &[u8], secret: &str) -> HeaderMap {
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(body);
        let hex = mac
            .finalize()
            .into_bytes()
            .iter()
            .fold(String::new(), |mut hex, b| {
                let _ = std::fmt::Write::write_fmt(&mut hex, format_args!("{b:02x}"));
                hex
            });
        let mut headers = HeaderMap::new();
        headers.insert(GITHUB_SIGNATURE, format!("sha256={hex}").parse().unwrap());
        headers
    }

    #[test]
    fn a_github_signature_counts_only_for_its_body_and_secret() {
        let secret = "made-for-this-test";
        assert!(signed(&github(b"{}", secret), b"{}", secret));
        assert!(!signed(&github(b"{}", secret), b"{\"a\":1}", secret));
        assert!(!signed(&github(b"{}", "another"), b"{}", secret));
        assert!(!signed(&HeaderMap::new(), b"{}", secret));
        let mut odd = HeaderMap::new();
        odd.insert(GITHUB_SIGNATURE, "sha256=zz".parse().unwrap());
        assert!(!signed(&odd, b"{}", secret));
    }

    #[test]
    fn a_gitlab_token_counts_only_when_it_is_the_secret() {
        let secret = "made-for-this-test";
        let mut headers = HeaderMap::new();
        headers.insert(GITLAB_TOKEN, secret.parse().unwrap());
        assert!(signed(&headers, b"{}", secret));
        headers.insert(GITLAB_TOKEN, "made-for-this-tesT".parse().unwrap());
        assert!(!signed(&headers, b"{}", secret));
    }

    /// Counts its renders, each a site with no pages.
    #[derive(Clone, Default)]
    struct Counted(Arc<AtomicUsize>);

    impl Render for Counted {
        type Error = std::io::Error;

        fn render(&self) -> Result<Rendered, Self::Error> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(Rendered::new(Vec::new(), "<h1>Page not found</h1>"))
        }
    }

    fn git(folder: &Path, args: &[&str]) {
        let ran = std::process::Command::new("git")
            .args(["-c", "user.name=Writer", "-c", "user.email=w@example.com"])
            .arg("-C")
            .arg(folder)
            .args(args)
            .output()
            .unwrap();
        assert!(ran.status.success(), "git {args:?}");
    }

    fn within(folder: &Path, name: &str) -> PathBuf {
        let mut path = folder.to_path_buf();
        path.push(name);
        path
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn a_pull_that_moved_files_is_rendered_without_the_file_watch() {
        let (origin, writer, blog) = (
            tempfile::tempdir().unwrap(),
            tempfile::tempdir().unwrap(),
            tempfile::tempdir().unwrap(),
        );
        let origin_path = origin.path().to_str().unwrap();
        git(origin.path(), &["init", "--bare", "--quiet", "-b", "main"]);
        git(writer.path(), &["clone", "--quiet", origin_path, "."]);
        fs::write(within(writer.path(), "site.toml"), "").unwrap();
        git(writer.path(), &["add", "."]);
        git(writer.path(), &["commit", "--quiet", "-m", "A site"]);
        git(writer.path(), &["push", "--quiet", "origin", "HEAD:main"]);
        git(blog.path(), &["clone", "--quiet", origin_path, "."]);
        fs::write(within(writer.path(), "post.md"), "A post.\n").unwrap();
        git(writer.path(), &["add", "."]);
        git(writer.path(), &["commit", "--quiet", "-m", "A post"]);
        git(writer.path(), &["push", "--quiet", "origin", "HEAD:main"]);

        let secret = "made-for-this-test";
        let renders = Counted::default();
        let live = LivePages::new(Pages::new(Vec::new(), "").unwrap());
        let shown = (renders.clone(), live);
        let hook = router(blog.path(), Some(Secret::new(secret.to_owned())), &shown);
        let push = Request::post(HOOK_PATH)
            .header(GITLAB_TOKEN, secret)
            .body(axum::body::Body::from("{}"))
            .unwrap();
        assert_eq!(hook.oneshot(push).await.unwrap().status(), 202);
        let rendered = async {
            while renders.0.load(Ordering::SeqCst) == 0 {
                tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            }
        };
        let waited = tokio::time::timeout(std::time::Duration::from_secs(20), rendered).await;
        assert!(waited.is_ok(), "the pulled post was rendered");
        assert!(within(blog.path(), "post.md").is_file(), "the blog pulled");
    }
}
