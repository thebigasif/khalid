//! Both APIs behind one router, and the one `OpenAPI` document.

use std::sync::Arc;

use axum::Router;
use axum::body::Bytes;
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::middleware;
use axum::routing::get;
use khalid_core::SystemClock;
use utoipa::OpenApi;
use utoipa::openapi::OpenApi as Document;
use utoipa_axum::router::OpenApiRouter;

use crate::admin_ai;
use crate::admin_key::{AdminDoor, require_admin_key};
use crate::analytics_desk::AnalyticsDesk;
use crate::api_limits::{self, ApiLimits};
use crate::api_version::negotiate_version;
use crate::comment_moderation::{CommentModeration, NoComments};
use crate::content_key::ContentDoor;
use crate::content_store::ContentStore;
use crate::draft_comments::DraftComments;
use crate::follow_ups::FollowUpFinder;
use crate::form_answers::FormAnswers;
use crate::key_schemes::KeySchemes;
use crate::keys::KeyStore;
use crate::media_api::{self, MediaLibrary};
use crate::post_store::PostStore;
use crate::post_trash::PostTrash;
use crate::problem::Problem;
use crate::publish_door::{PublishDoor, Silent};
use crate::redirect_desk::RedirectDesk;
use crate::resources::MediaItems;
use crate::serpdad_store::SerpdadStore;
use crate::session_check::{SessionCheck, StaffRoutes};
use crate::webhook_store::WebhookStore;
use crate::{
    admin_analytics, admin_api, admin_api_keys, admin_audit, admin_authors, admin_calendar,
    admin_categories, admin_check, admin_comments, admin_consent, admin_design,
    admin_draft_comments, admin_draft_threads, admin_follow_ups, admin_forms, admin_newsletters,
    admin_not_found, admin_pages, admin_redirect_csv, admin_redirects, admin_schema_templates,
    admin_serpdad, admin_site_settings, admin_snippets, admin_subscribers, admin_tags,
    admin_templates, admin_trash, admin_webhooks, api_version, content_api,
};

pub use crate::audit_desk::AuditDesk;

mod desks;

pub use desks::AdminDesks;

/// Where the `OpenAPI` document is served.
const DOCUMENT: &str = "/api/openapi.json";

/// The media type it is served as.
const JSON: &str = "application/json";

/// The document's title, the schemas every route shares and the ways a
/// key may be sent.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Khalid API",
        description = "The Content API and the Admin API of a Khalid blog. Each key, and each \
                       address that sends wrong keys, has a limit a minute; past it the answer \
                       is 429 with Retry-After."
    ),
    components(schemas(Problem, MediaItems)),
    modifiers(&KeySchemes)
)]
pub(crate) struct ApiDoc;

/// The staff routes and the session check of a site with staff accounts.
pub type Staff = (StaffRoutes, Arc<dyn SessionCheck>);

/// The Content API under `/api/content/v1/`, the Admin API under
/// `/api/admin/v1/`, and `/api/openapi.json`, with the document every
/// route declares (RULES.md BL-API-4). The document itself needs no key.
/// A post that goes live is announced to nobody; see [`router_with_door`].
pub fn router(
    content: Arc<dyn ContentStore>,
    posts: Arc<dyn PostStore>,
    keys: Arc<dyn KeyStore>,
) -> (Router, Document) {
    router_with_comments(content, posts, keys, Arc::new(NoComments))
}

/// [`router`] for a site that keeps readers' comments, moderated through
/// `comments` under `/api/admin/v1/comments/`. The document is the same.
pub fn router_with_comments(
    content: Arc<dyn ContentStore>,
    posts: Arc<dyn PostStore>,
    keys: Arc<dyn KeyStore>,
    comments: Arc<dyn CommentModeration>,
) -> (Router, Document) {
    let door = PublishDoor::new(posts, Arc::new(Silent));
    router_with_door_and_comments(content, Arc::new(door), keys, comments)
}

/// The same router, publishing through `door`, which `khalid serve` shares
/// with its scheduler so every post that goes live is announced.
/// Its redirect routes keep nothing; see [`router_with_redirects`].
pub fn router_with_door(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
) -> (Router, Document) {
    router_with_door_and_comments(content, door, keys, Arc::new(NoComments))
}

/// The same router, with the redirect routes under
/// `/api/admin/v1/redirects/` reading and writing through `redirects`.
pub fn router_with_redirects(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    redirects: RedirectDesk,
) -> (Router, Document) {
    let desks = AdminDesks {
        redirects,
        ..AdminDesks::none()
    };
    assemble(content, door, keys, desks, None)
}

/// [`router_with_door`] for a site that keeps readers' comments, moderated
/// through `comments`. Without a media library, the media routes answer
/// that the site has none.
pub fn router_with_door_and_comments(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    comments: Arc<dyn CommentModeration>,
) -> (Router, Document) {
    let desks = AdminDesks {
        comments,
        ..AdminDesks::none()
    };
    assemble(content, door, keys, desks, None)
}

/// [`router_with_door_and_comments`] with the site's media library under
/// `/api/admin/v1/media/`.
pub fn router_with_media(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    comments: Arc<dyn CommentModeration>,
    media: MediaLibrary,
) -> (Router, Document) {
    let desks = AdminDesks {
        comments,
        media: Some(media),
        ..AdminDesks::none()
    };
    assemble(content, door, keys, desks, None)
}

/// [`router`] with staff accounts: `staff` routes join the Admin API, and
/// `sessions` lets a staff session cookie in wherever a key is needed.
/// Each of `staff.open` and `staff.signed_in` must hold at least one route.
pub fn router_with_staff(
    content: Arc<dyn ContentStore>,
    posts: Arc<dyn PostStore>,
    keys: Arc<dyn KeyStore>,
    staff: StaffRoutes,
    sessions: Arc<dyn SessionCheck>,
) -> (Router, Document) {
    let door = Arc::new(PublishDoor::new(posts, Arc::new(Silent)));
    let staff = Some((staff, sessions));
    assemble(content, door, keys, AdminDesks::none(), staff)
}

/// [`router_with_door_and_comments`] with staff accounts.
/// [`router_with_staff`] says what `staff` and `sessions` are.
pub fn router_with_door_comments_and_staff(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    comments: Arc<dyn CommentModeration>,
    staff: StaffRoutes,
    sessions: Arc<dyn SessionCheck>,
) -> (Router, Document) {
    let desks = AdminDesks {
        comments,
        ..AdminDesks::none()
    };
    assemble(content, door, keys, desks, Some((staff, sessions)))
}

/// [`router_with_door`] keeping each deleted post in `trash`, under
/// `/api/admin/v1/trash/`. `trash` is usually the store behind `door`.
pub fn router_with_trash(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    trash: Arc<dyn PostTrash>,
) -> (Router, Document) {
    let desks = AdminDesks {
        trash: Some(trash),
        ..AdminDesks::none()
    };
    assemble(content, door, keys, desks, None)
}

/// [`router_with_door`] with comments on drafts under
/// `/api/admin/v1/posts/{id}/draft-comments/`, kept in `comments`.
pub fn router_with_draft_comments(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    comments: DraftComments,
) -> (Router, Document) {
    let desks = AdminDesks {
        draft_comments: Some(comments),
        ..AdminDesks::none()
    };
    assemble(content, door, keys, desks, None)
}

/// [`router_with_door`] with the Search Console follow-ups found by
/// `finder`.
pub fn router_with_follow_ups(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    finder: Arc<dyn FollowUpFinder>,
) -> (Router, Document) {
    let desks = AdminDesks {
        follow_ups: finder,
        ..AdminDesks::none()
    };
    assemble(content, door, keys, desks, None)
}

/// [`router_with_door`] with the form routes under
/// `/api/admin/v1/forms/` reading the answers in `forms`.
pub fn router_with_forms(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    forms: Arc<dyn FormAnswers>,
) -> (Router, Document) {
    let desks = AdminDesks {
        forms: Some(forms),
        ..AdminDesks::none()
    };
    assemble(content, door, keys, desks, None)
}

/// [`router_with_door`] with the analytics routes under
/// `/api/admin/v1/analytics/` reading the views in `analytics`.
pub fn router_with_analytics(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    analytics: AnalyticsDesk,
) -> (Router, Document) {
    let desks = AdminDesks {
        analytics,
        ..AdminDesks::none()
    };
    assemble(content, door, keys, desks, None)
}

/// [`router_with_door`] keeping every write that succeeds in `audit`'s
/// log, which `/api/admin/v1/audit/` lists.
pub fn router_with_audit(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    audit: AuditDesk,
) -> (Router, Document) {
    let desks = AdminDesks {
        audit: Some(audit),
        ..AdminDesks::none()
    };
    assemble(content, door, keys, desks, None)
}

/// [`router_with_door`] with every store in `desks`, and no staff.
pub fn router_with_desks(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    desks: AdminDesks,
) -> (Router, Document) {
    assemble(content, door, keys, desks, None)
}

/// [`router_with_door`] with the webhook routes under
/// `/api/admin/v1/webhooks/` reading and writing through `webhooks`.
pub fn router_with_webhooks(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    webhooks: Arc<dyn WebhookStore>,
) -> (Router, Document) {
    let desks = AdminDesks {
        webhooks: Some(webhooks),
        ..AdminDesks::none()
    };
    assemble(content, door, keys, desks, None)
}

/// [`router_with_door`] with the `SerpDad` route under
/// `/api/admin/v1/serpdad/` keeping its connection in `serpdad`.
pub fn router_with_serpdad(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    serpdad: Option<Arc<dyn SerpdadStore>>,
) -> (Router, Document) {
    let desks = AdminDesks {
        serpdad,
        ..AdminDesks::none()
    };
    assemble(content, door, keys, desks, None)
}

/// The whole router as `khalid serve` runs it: every store in `desks`,
/// and `staff`, the staff routes and the session check that
/// [`router_with_staff`] describes.
pub fn router_for_serve(
    content: Arc<dyn ContentStore>,
    door: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    desks: AdminDesks,
    staff: Staff,
) -> (Router, Document) {
    assemble(content, door, keys, desks, Some(staff))
}

/// Every router above: the Content API, and the Admin API publishing
/// through `posts` and keeping what `desks` hold. With `staff`, its routes
/// join the Admin API and a staff session counts wherever a key is needed.
fn assemble(
    content: Arc<dyn ContentStore>,
    posts: Arc<PublishDoor>,
    keys: Arc<dyn KeyStore>,
    desks: AdminDesks,
    staff: Option<Staff>,
) -> (Router, Document) {
    let (routes, sessions) = staff.unzip();
    let limits = Arc::new(ApiLimits::new(SystemClock, desks.limits));
    let door = AdminDoor {
        keys: Arc::clone(&keys),
        sessions,
        limits: Arc::clone(&limits),
        audit: desks.audit.clone(),
    };
    let content_door = ContentDoor {
        keys: Arc::clone(&keys),
        limits,
    };
    let redirects = desks.redirects;
    let ai = admin_ai::AiRoute {
        desk: desks.ai,
        serpdad: desks.serpdad.clone(),
        posts: Arc::clone(&posts),
    };
    let serpdad = admin_serpdad::Serpdad::new(desks.serpdad, Arc::clone(&keys));
    let mut admin = admin_api::routes(Arc::clone(&posts), serpdad.clone(), door.clone())
        .merge(admin_audit::routes(desks.audit, door.clone()))
        .merge(admin_calendar::routes(Arc::clone(&posts), door.clone()))
        .merge(admin_draft_comments::routes(
            Arc::clone(&posts),
            desks.draft_comments.clone(),
            door.clone(),
        ))
        .merge(admin_draft_threads::routes(
            posts,
            desks.draft_comments,
            door.clone(),
        ))
        .merge(admin_comments::routes(desks.comments, door.clone()))
        .merge(admin_subscribers::routes(
            desks.subscribers.clone(),
            door.clone(),
        ))
        .merge(admin_newsletters::routes(desks.subscribers, door.clone()))
        .merge(media_api::routes(desks.media, door.clone()))
        .merge(admin_not_found::routes(redirects.clone(), door.clone()))
        .merge(admin_redirect_csv::routes(redirects.clone(), door.clone()))
        .merge(admin_redirects::routes(redirects, door.clone()))
        .merge(admin_trash::routes(desks.trash, door.clone()))
        .merge(admin_webhooks::routes(desks.webhooks, door.clone()))
        .merge(admin_check::routes(desks.checks, door.clone()))
        .merge(admin_schema_templates::routes(
            desks.schema_templates,
            door.clone(),
        ))
        .merge(admin_pages::routes(desks.pages, door.clone()))
        .merge(admin_snippets::routes(desks.snippets, door.clone()))
        .merge(admin_authors::routes(desks.authors, door.clone()))
        .merge(admin_tags::routes(desks.tags, door.clone()))
        .merge(admin_categories::routes(desks.categories, door.clone()))
        .merge(admin_site_settings::routes(desks.settings, door.clone()))
        .merge(admin_templates::routes(desks.templates, door.clone()))
        .merge(admin_design::routes(desks.design, door.clone()))
        .merge(admin_api_keys::routes(keys, door.clone()))
        .merge(admin_consent::routes(desks.analytics.clone(), door.clone()))
        .merge(admin_analytics::routes(desks.analytics, door.clone()))
        .merge(admin_follow_ups::routes(desks.follow_ups, door.clone()))
        .merge(admin_forms::routes(desks.forms, door.clone()))
        .merge(admin_serpdad::routes(serpdad, door.clone()))
        .merge(admin_ai::routes(ai, door.clone()));
    if let Some(staff) = routes {
        let signed_in = staff
            .signed_in
            .route_layer(middleware::from_fn_with_state(door, require_admin_key));
        let staff = staff
            .open
            .merge(signed_in)
            .route_layer(middleware::from_fn(negotiate_version));
        admin = admin.merge(staff);
    }
    let api = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest(
            content_api::BASE,
            content_api::routes(content, content_door),
        )
        .nest(admin_api::BASE, admin);
    finish(api)
}

/// Splits `api` into the router and its document, lists
/// `Accept-Version` on every operation, and serves the document.
///
/// The document is served as the JSON it turns into, made once. Kept as a
/// struct, a copy of the whole document went into each copy of the route
/// the router makes as it is put together, and stayed there.
pub(crate) fn finish(api: OpenApiRouter) -> (Router, Document) {
    let (router, mut document) = api.split_for_parts();
    api_version::document(&mut document);
    api_limits::document(&mut document);
    let router = match serde_json::to_vec(&document) {
        Ok(json) => {
            let json = Bytes::from(json);
            router.route(
                DOCUMENT,
                get(move || async move { ([(CONTENT_TYPE, JSON)], json) }),
            )
        }
        Err(error) => {
            tracing::error!(%error, "the OpenAPI document could not be written as JSON");
            router.route(
                DOCUMENT,
                get(|| async { StatusCode::INTERNAL_SERVER_ERROR }),
            )
        }
    };
    (router, document)
}

#[cfg(test)]
mod tests {
    use axum::body::{Body, to_bytes};
    use axum::http::Request;
    use tower::ServiceExt as _;

    use super::*;

    #[tokio::test]
    async fn the_document_is_served_as_the_json_it_turns_into() {
        let (router, document) = finish(OpenApiRouter::with_openapi(ApiDoc::openapi()));
        let request = Request::get(DOCUMENT).body(Body::empty()).unwrap();
        let answer = router.oneshot(request).await.unwrap();
        assert_eq!(answer.status(), StatusCode::OK);
        assert_eq!(answer.headers()[CONTENT_TYPE], JSON);
        let body = to_bytes(answer.into_body(), usize::MAX).await.unwrap();
        assert_eq!(body, serde_json::to_vec(&document).unwrap());
    }
}
