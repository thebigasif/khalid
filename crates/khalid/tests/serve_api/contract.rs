//! The `OpenAPI` contract on a running `khalid serve`: the document it
//! serves is the SDK's, every documented route reaches a handler, and
//! every API address that answers is documented (KH-API-4).

use std::fs;
use std::path::PathBuf;

use serde_json::Value;

use super::{demo_copy, exchange, new_key, start};

/// The resources `docs/standards/api.md` names, and `navigation`, as the
/// contract test in `khalid-api` tries them.
const RESOURCES: [&str; 18] = [
    "posts",
    "pages",
    "authors",
    "tags",
    "categories",
    "collections",
    "glossary",
    "case-studies",
    "changelog",
    "media",
    "redirects",
    "snippets",
    "forms",
    "subscribers",
    "webhooks",
    "settings",
    "site",
    "navigation",
];

/// The methods a route could answer.
const METHODS: [&str; 5] = ["GET", "POST", "PUT", "PATCH", "DELETE"];

/// Whether an answer came from an API handler: those answer JSON, a
/// problem, or a CSV export, and every other address gets the site's HTML
/// pages.
#[cfg(test)]
fn from_a_handler(media: &str) -> bool {
    media.starts_with("application/json")
        || media.starts_with("application/problem+json")
        || media.starts_with("text/csv")
}

/// Brief B-API-4's done line: `khalid serve` answers `/api/openapi.json`
/// without a key, it is the document the SDK is generated from, and the
/// contract holds on the running server: every documented route reaches a
/// handler, and every API address that answers is documented.
#[test]
fn khalid_serve_serves_the_openapi_document_its_routes_keep() {
    let site = demo_copy();
    let content_key = new_key(site.path(), &["--kind", "content", "--name", "Web page"]);
    let admin_key = new_key(site.path(), &["--name", "Contract", "--scope", "admin:all"]);
    let server = start(site.path());

    let (status, media, body) = exchange(&server, "GET", "/api/openapi.json", &[], None);
    assert_eq!(
        (status, media.as_str()),
        (200, "application/json"),
        "{body}"
    );
    let mut document: Value = serde_json::from_str(&body).unwrap();
    // The SDK's copy fits every blog; the served one names where this
    // blog's APIs answer, under the demo's mount.
    let servers = document.as_object_mut().unwrap().remove("servers");
    let blog = serde_json::json!([{ "url": "https://www.example.com/blog" }]);
    assert_eq!(servers, Some(blog));
    let mut sdk = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    sdk.push("../../sdk/typescript/openapi.json");
    let committed: Value = serde_json::from_str(&fs::read_to_string(sdk).unwrap()).unwrap();
    assert!(
        document == committed,
        "the served document is not sdk/typescript/openapi.json"
    );

    let content = format!("Khalid-Content {content_key}");
    let admin = format!("Khalid {admin_key}");
    let reached = |method: &str, path: &str| {
        let key = if path.starts_with("/api/admin/") {
            &admin
        } else {
            &content
        };
        let headers = [("Authorization", key.as_str())];
        let empty = serde_json::json!({});
        let (status, media, _) = exchange(&server, method, path, &headers, Some(&empty));
        // A 204 has no body to name its type; only a handler answers one.
        status == 204 || from_a_handler(&media)
    };
    let concrete = |path: &str| {
        path.replace("{id}", "contract-id")
            .replace("{slug}", "contract-slug")
    };
    let paths = document["paths"].as_object().unwrap();
    assert!(paths.len() > 30, "{}", paths.len());
    for (path, item) in paths {
        for method in METHODS {
            let documented = item.get(method.to_lowercase()).is_some();
            let answered = reached(method, &concrete(path));
            assert_eq!(answered, documented, "{method} {path}");
        }
    }
    for base in ["/api/content/v1", "/api/admin/v1"] {
        for resource in RESOURCES {
            for suffix in ["/", "/contract-id/", "/slug/contract-slug/"] {
                let address = format!("{base}/{resource}{suffix}");
                for method in METHODS {
                    if reached(method, &address) {
                        let listed = paths.iter().any(|(path, item)| {
                            concrete(path) == address && item.get(method.to_lowercase()).is_some()
                        });
                        assert!(listed, "{method} {address} answers but is not documented");
                    }
                }
            }
        }
    }
}
