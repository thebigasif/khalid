//! The admin app, built from `admin/` by Vite and embedded in the binary,
//! answered under `admin/` in the blog's folder, such as `/admin/` or
//! `/blog/admin/` (plan section 4.6). It talks only to the Admin and
//! Content APIs, which `khalid serve` answers beside it.

use std::sync::Arc;

use axum::Router;
use axum::body::Bytes;
use axum::extract::Path;
use axum::http::{StatusCode, Uri, header};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;

/// One file of the built admin, by its path under `admin/dist/`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Asset {
    /// Where it sits under `admin/dist/`, such as `assets/index-4f2a.js`.
    pub path: &'static str,
    /// The file's bytes.
    pub body: &'static [u8],
}

mod built {
    include!(concat!(env!("OUT_DIR"), "/admin_files.rs"));
}

/// The admin's own policy: its scripts, styles and API calls come from the
/// blog itself, and no other site may frame it. Pictures may also come
/// from Unsplash and GIPHY, where photo and GIF search find them.
const POLICY: &str = "default-src 'self'; script-src 'self'; style-src 'self'; \
    img-src 'self' data: https://images.unsplash.com https://*.giphy.com; \
    connect-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'";

/// Vite names every file under `assets/` by a hash of its content, so a
/// browser may keep it; the page itself is checked on every visit.
const FOR_A_YEAR: &str = "public, max-age=31536000, immutable";
const CHECK_EACH_TIME: &str = "no-cache";

const NOT_BUILT: &str = "<!doctype html>\n<html lang=\"en\">\n<meta charset=\"utf-8\">\n\
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
    <title>The admin is not in this copy of Khalid</title>\n\
    <h1>The admin is not in this copy of Khalid</h1>\n\
    <p>This program was built without the admin app, so there is nothing to show here. \
    The blog and its APIs work as usual.</p>\n\
    <p>To add it, run <code>npm ci</code> and <code>npm run build</code> in the \
    <code>admin</code> folder, then build Khalid again.</p>\n";

/// The files `npm run build` left in `admin/dist/` when this binary was
/// built; none when it had not run.
pub fn built() -> &'static [Asset] {
    built::FILES
}

/// The blog an admin belongs to, as its page names it, so the sign-in
/// screen shows whose admin this is before anyone signs in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Blog<'a> {
    /// The folder the blog sits under on its host, such as `/blog/`, or
    /// `/` for a blog that is the whole site.
    pub mount: &'a str,
    /// The blog's name, as `site.toml` gives it.
    pub name: &'a str,
    /// The full address of the main site's logo, when there is one the
    /// admin's own policy lets it show.
    pub logo: Option<&'a str>,
    /// The theme's accent as `#rrggbb`, when it reads well on white, so
    /// the admin's buttons and links wear the blog's own colour.
    pub accent: Option<&'a str>,
}

/// The routes that answer the admin of `blog` under its mount, such as
/// `/blog/`, or `/` for a blog that is the whole site: `<mount>admin`
/// moves to `<mount>admin/`, a built file answers as itself, and any other
/// address under `<mount>admin/` gets the app's page, whose router shows
/// the screen for it. The page names the mount, so the app finds its
/// files, screens and APIs there, and the blog's name and logo. Under a
/// mount, an address under `/admin/` moves to the same one under
/// `<mount>admin/`.
pub fn router(files: &'static [Asset], blog: Blog<'_>) -> Router {
    let mount = blog.mount;
    let home = Arc::<str>::from(format!("{mount}admin/"));
    let shown = Arc::new(page_in(files, blog));
    let (to_home, at_home, anywhere) = (Arc::clone(&home), Arc::clone(&shown), shown);
    let router = Router::new()
        .route(
            &format!("{mount}admin"),
            get(move || async move { Redirect::permanent(&to_home) }),
        )
        .route(
            &home,
            get(move || async move { page(Option::as_ref(&at_home)) }),
        )
        .route(
            &format!("{home}{{*path}}"),
            get(move |Path(path): Path<String>| async move {
                file(files, Option::as_ref(&anywhere), &path)
            }),
        );
    if mount == "/" {
        return router;
    }
    let moved = move |uri: Uri| async move {
        let rest = uri.path().strip_prefix("/admin/").unwrap_or_default();
        let query = uri
            .query()
            .map(|query| format!("?{query}"))
            .unwrap_or_default();
        Redirect::permanent(&format!("{home}{rest}{query}"))
    };
    router
        .route("/admin", get(moved.clone()))
        .route("/admin/", get(moved.clone()))
        .route("/admin/{*path}", get(moved))
}

/// The app's page as a blog under `mount` serves it, or `None` when this
/// binary holds no admin. Vite names the built files from the page's own
/// folder, which differs on each screen's address, so they are named from
/// the host's root instead, and the mount goes in a `khalid-mount` meta,
/// after the blog's name, logo and accent in their own.
fn page_in(files: &[Asset], blog: Blog<'_>) -> Option<Bytes> {
    let index = files.iter().find(|asset| asset.path == "index.html")?;
    let html = String::from_utf8_lossy(index.body);
    let mount = blog.mount;
    let folder = format!("=\"{mount}admin/");
    let name = meta("khalid-name", blog.name);
    let logo = blog
        .logo
        .map(|logo| meta("khalid-logo", logo))
        .unwrap_or_default();
    let accent = blog
        .accent
        .map(|accent| meta("khalid-accent", accent))
        .unwrap_or_default();
    let named =
        format!("{name}{logo}{accent}<meta name=\"khalid-mount\" content=\"{mount}\">\n</head>");
    let html = html
        .replace("=\"./", &folder)
        .replace("=\"/admin/", &folder)
        .replacen("</head>", &named, 1);
    Some(Bytes::from(html))
}

/// One `<meta>` line, its content written so no name can end the attribute.
fn meta(name: &str, content: &str) -> String {
    let content = content
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    format!("<meta name=\"{name}\" content=\"{content}\">\n")
}

fn file(files: &[Asset], shown: Option<&Bytes>, path: &str) -> Response {
    match files.iter().find(|asset| asset.path == path) {
        Some(asset) if path.starts_with("assets/") => answer(asset, FOR_A_YEAR),
        Some(_) if path == "index.html" => page(shown),
        Some(asset) => answer(asset, CHECK_EACH_TIME),
        None if path.starts_with("assets/") => StatusCode::NOT_FOUND.into_response(),
        None => page(shown),
    }
}

fn page(shown: Option<&Bytes>) -> Response {
    let headers = headers("text/html; charset=utf-8", CHECK_EACH_TIME);
    match shown {
        Some(html) => (headers, html.clone()).into_response(),
        None => (StatusCode::SERVICE_UNAVAILABLE, headers, NOT_BUILT).into_response(),
    }
}

fn answer(asset: &Asset, cache: &'static str) -> Response {
    (headers(content_type(asset.path), cache), asset.body).into_response()
}

fn headers(
    content_type: &'static str,
    cache: &'static str,
) -> [(header::HeaderName, &'static str); 5] {
    [
        (header::CONTENT_TYPE, content_type),
        (header::CACHE_CONTROL, cache),
        (header::CONTENT_SECURITY_POLICY, POLICY),
        (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
        (header::REFERRER_POLICY, "no-referrer"),
    ]
}

/// The type of a built file, from its extension.
fn content_type(path: &str) -> &'static str {
    let extension = path.rsplit_once('.').map_or("", |(_, extension)| extension);
    match extension {
        "html" => "text/html; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ico" => "image/x-icon",
        "woff2" => "font/woff2",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    include!("build_warning.rs");

    #[test]
    fn the_build_warning_says_what_why_and_fix_like_the_admin_page() {
        let [what, why, fix] = BUILD_WARNING;
        assert!(what.contains("admin/dist/"), "{what}");
        assert!(why.contains("/admin/"), "{why}");
        for step in ["npm ci", "npm run build"] {
            assert!(fix.contains(step), "{fix}");
            assert!(super::NOT_BUILT.contains(step), "the page names {step} too");
        }
        for line in BUILD_WARNING {
            assert!(line.ends_with('.'), "{line}");
            assert!(line.split_whitespace().count() < 25, "{line}");
            assert!(!line.contains('\u{2014}'), "{line}");
        }
    }
}
