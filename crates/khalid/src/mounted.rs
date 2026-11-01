//! A blog under a folder of another site, its mount such as `/blog/`,
//! answers everything there, since the proxy in front passes on only that
//! folder (KH-INS-3). The APIs keep their routes at `/api/`, where
//! `khalid mcp` and the other commands reach them in process. So a request
//! under `<mount>api/` reaches them there, and the paths they answer with
//! are given back under the mount. `/api/` itself still answers, for a
//! program that reaches Khalid without the proxy.

use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{HeaderValue, Request, StatusCode, Uri, header};
use axum::response::{IntoResponse, Response};
use khalid_core::Config;
use serde_json::{Value, json};
use tower::ServiceExt as _;
use tower::service_fn;

/// Where the APIs answer from the host's root.
const API: &str = "/api/";

/// Where the `OpenAPI` document is served.
const DOCUMENT: &str = "/api/openapi.json";

/// The routes whose answers name a screen of the admin as `admin_path`, a
/// path from the host's root.
const NAMING_THE_ADMIN: [&str; 2] = ["/api/admin/v1/posts/", "/api/admin/v1/serpdad/"];

/// The blog's address with its mount, such as `https://example.com/blog/`,
/// where every address Khalid gives out starts.
pub(crate) fn blog_address(config: &Config) -> String {
    let address = config.address().as_str().trim_end_matches('/');
    format!("{address}{}", config.mount())
}

/// `app`, answering under the mount of the blog in `config` what it
/// answers under `/api/`, and naming the blog under its mount as the
/// server in the `OpenAPI` document. At `/`, `app` as it is.
pub(crate) fn router(config: &Config, app: Router) -> Router {
    let mount = config.mount();
    if mount == "/" {
        return app;
    }
    let prefix = Arc::<str>::from(mount.trim_end_matches('/'));
    let server = Arc::<str>::from(blog_address(config).trim_end_matches('/'));
    let answer = service_fn(move |request: Request<Body>| {
        let (app, prefix, server) = (app.clone(), Arc::clone(&prefix), Arc::clone(&server));
        async move {
            let (mut parts, body) = request.into_parts();
            let at_root = under_api(&parts.uri, &prefix);
            if let Some(uri) = &at_root {
                parts.uri.clone_from(uri);
            }
            let path = parts.uri.path();
            let (names_admin, document) = (NAMING_THE_ADMIN.contains(&path), path == DOCUMENT);
            let mut answer = app.oneshot(Request::from_parts(parts, body)).await?;
            if at_root.is_some() {
                answer = moved_under(answer, &prefix);
                if names_admin {
                    answer = admin_under(answer, &prefix).await;
                }
            }
            if document {
                answer = with_server(answer, &server).await;
            }
            Ok(answer)
        }
    });
    Router::new().fallback_service(answer)
}

/// The address at `/api/` that `uri` under `<prefix>/api/` stands for.
fn under_api(uri: &Uri, prefix: &str) -> Option<Uri> {
    let rest = uri.path().strip_prefix(prefix)?.strip_prefix(API)?;
    let query = uri
        .query()
        .map(|query| format!("?{query}"))
        .unwrap_or_default();
    format!("{API}{rest}{query}").parse().ok()
}

/// `answer` with the path in its `Location`, if it has one, under `prefix`.
fn moved_under(mut answer: Response, prefix: &str) -> Response {
    let moved = answer.headers().get(header::LOCATION).and_then(|to| {
        let to = to.to_str().ok()?;
        let root = to.starts_with('/') && !to.starts_with("//");
        root.then(|| HeaderValue::try_from(format!("{prefix}{to}")).ok())?
    });
    if let Some(moved) = moved {
        answer.headers_mut().insert(header::LOCATION, moved);
    }
    answer
}

/// `answer` with each `admin_path` in its JSON under `prefix`.
async fn admin_under(answer: Response, prefix: &str) -> Response {
    let (mut parts, body) = answer.into_parts();
    // The answer was made in memory, so reading it whole adds no limit.
    let Ok(bytes) = to_bytes(body, usize::MAX).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let named = format!("\"admin_path\":\"{prefix}/admin/");
    let text = String::from_utf8_lossy(&bytes).replace("\"admin_path\":\"/admin/", &named);
    parts.headers.remove(header::CONTENT_LENGTH);
    Response::from_parts(parts, Body::from(text))
}

/// The `OpenAPI` document in `answer`, naming `server` as where the APIs
/// answer. Anything else, such as a failure, is left as it is.
async fn with_server(answer: Response, server: &str) -> Response {
    if answer.status() != StatusCode::OK {
        return answer;
    }
    let (mut parts, body) = answer.into_parts();
    let Ok(bytes) = to_bytes(body, usize::MAX).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let mut document: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    let Some(fields) = document.as_object_mut() else {
        return Response::from_parts(parts, Body::from(bytes));
    };
    fields.insert("servers".to_owned(), json!([{ "url": server }]));
    let Ok(bytes) = serde_json::to_vec(&document) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    parts.headers.remove(header::CONTENT_LENGTH);
    Response::from_parts(parts, Body::from(bytes))
}

#[cfg(test)]
mod tests {
    use axum::routing::get;

    use super::*;

    fn config(mount: &str) -> Config {
        let text = format!(
            "name = \"Blog\"\naddress = \"https://example.com\"\nmount = \"{mount}\"\n\
             mode = \"server\"\ndefault_language = \"en\"\nlanguages = [\"en\"]\n"
        );
        let env = khalid_core::Environment::empty();
        Config::from_toml(std::path::Path::new("site.toml"), &text, &env).unwrap()
    }

    fn app() -> Router {
        let made = || async {
            (
                StatusCode::CREATED,
                [(header::LOCATION, "/api/admin/v1/pages/a/")],
            )
        };
        let serpdad = || async { r#"{"drafts":[{"admin_path":"/admin/posts/en.a"}]}"# };
        Router::new()
            .route("/api/admin/v1/pages/", get(made))
            .route("/api/admin/v1/serpdad/", get(serpdad))
            .route(DOCUMENT, get(|| async { r#"{"openapi":"3.1.0"}"# }))
            .route("/blog/", get(|| async { "home" }))
    }

    async fn get_at(router: &Router, path: &str) -> (StatusCode, Option<String>, String) {
        let request = Request::get(path).body(Body::empty()).unwrap();
        let answer = router.clone().oneshot(request).await.unwrap();
        let location = answer
            .headers()
            .get(header::LOCATION)
            .map(|to| to.to_str().unwrap().to_owned());
        let status = answer.status();
        let body = to_bytes(answer.into_body(), usize::MAX).await.unwrap();
        (status, location, String::from_utf8(body.to_vec()).unwrap())
    }

    #[test]
    fn the_blog_address_ends_with_its_mount() {
        assert_eq!(blog_address(&config("/blog/")), "https://example.com/blog/");
        assert_eq!(blog_address(&config("/")), "https://example.com/");
    }

    #[tokio::test]
    async fn under_a_mount_the_apis_answer_there_and_name_it() {
        let router = router(&config("/blog/"), app());
        let (status, location, _) = get_at(&router, "/blog/api/admin/v1/pages/?x=1").await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(location.as_deref(), Some("/blog/api/admin/v1/pages/a/"));
        let (_, _, drafts) = get_at(&router, "/blog/api/admin/v1/serpdad/").await;
        assert_eq!(
            drafts,
            r#"{"drafts":[{"admin_path":"/blog/admin/posts/en.a"}]}"#
        );
        let (_, _, document) = get_at(&router, "/blog/api/openapi.json").await;
        let document: Value = serde_json::from_str(&document).unwrap();
        assert_eq!(document["servers"][0]["url"], "https://example.com/blog");
        assert_eq!(get_at(&router, "/blog/").await.2, "home");
    }

    #[tokio::test]
    async fn the_apis_still_answer_at_the_root_with_root_paths() {
        let router = router(&config("/blog/"), app());
        let (_, location, _) = get_at(&router, "/api/admin/v1/pages/").await;
        assert_eq!(location.as_deref(), Some("/api/admin/v1/pages/a/"));
        let (_, _, drafts) = get_at(&router, "/api/admin/v1/serpdad/").await;
        assert!(drafts.contains("\"/admin/posts/en.a\""), "{drafts}");
    }

    #[tokio::test]
    async fn at_the_root_nothing_changes() {
        let router = router(&config("/"), app());
        let (_, location, _) = get_at(&router, "/api/admin/v1/pages/").await;
        assert_eq!(location.as_deref(), Some("/api/admin/v1/pages/a/"));
        assert_eq!(get_at(&router, DOCUMENT).await.2, r#"{"openapi":"3.1.0"}"#);
        assert_eq!(
            get_at(&router, "/blog/api/openapi.json").await.0,
            StatusCode::NOT_FOUND
        );
    }
}
