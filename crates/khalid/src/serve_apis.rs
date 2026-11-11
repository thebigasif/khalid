//! The Content API and the Admin API as `khalid serve` wires them to one
//! site: every Admin API desk backed by the site's files and database, built
//! on the first request under `/api/`. Kept apart from `serve.rs` so that file stays
//! under 600 lines (BL-SIZE-1).

use std::path::Path;
use std::sync::Arc;

use axum::Router;
use khalid_api::ai::AiDesk;
use khalid_api::analytics_desk::AnalyticsDesk;
use khalid_api::api::{self, AdminDesks, AuditDesk};
use khalid_api::comment_moderation::CommentModeration;
use khalid_api::content_store::ContentStore;
use khalid_api::design_desk::DesignDesk;
use khalid_api::ghost::{self, GhostSite};
use khalid_api::keys::KeyStore;
use khalid_api::media_api::MediaLibrary;
use khalid_api::publish_door::PublishDoor;
use khalid_api::redirect_desk::RedirectDesk;
use khalid_api::resources::{AdminAuthor, AdminPage, AdminSnippet};
use khalid_api::schema_copy::SchemaCopier;
use khalid_api::session_check::SessionCheck;
use khalid_auth::SiteDirectory;
use khalid_core::{Config, Environment, SystemClock};
use khalid_db::{
    AnalyticsTable, AuditTable, ConsentProofTable, Database, FormTable, KeyTable, SerpdadTable,
    SiteId, SubscriberTable, WebhookTable,
};
use khalid_newsletter::Mailer;

use crate::account_author::SiteAuthorProfiles;
use crate::search_console_finder::SiteFollowUps;
use crate::site_categories::SiteCategories;
use crate::site_content::LiveContent;
use crate::site_entries::SiteEntries;
use crate::site_posts::SitePosts;
use crate::site_schema_templates::SiteSchemaTemplates;
use crate::site_settings::SiteSettings;
use crate::site_tags::SiteTags;
use crate::watch_report::SiteUptime;
use crate::{draft_comment_mail, late_api, look, mounted, staff};

pub(crate) mod ai_writer;
pub(crate) mod schema_copier;
mod subscribers;

/// The desks `khalid serve` keeps in memory for the Admin API: comment
/// moderation, the media library, stored redirects and the Search Console
/// follow-ups, the writer of the editor's suggestions and the page fetcher
/// that copies schema from a URL.
pub(crate) type Desks = (
    Arc<dyn CommentModeration>,
    MediaLibrary,
    RedirectDesk,
    SiteFollowUps,
    (AiDesk, Option<schema_copier::PageCopier>),
);

/// What [`api_router`] is built from: its four argument groups.
pub(crate) type ApiParts<'a> = (
    (LiveContent, Arc<PublishDoor>, SitePosts),
    Desks,
    (
        &'a Database,
        SiteId,
        (&'a Path, &'a Config, &'a Environment),
    ),
    (Arc<dyn SiteDirectory>, Option<Arc<dyn Mailer>>),
);

/// The Content API and the Admin API as `khalid serve` runs them, with the
/// comment moderation, media library, stored redirects, Search Console
/// follow-ups, webhooks, form answers, `SerpDad` connection, watch looks and staff
/// accounts of site `id`, the post checks, schema templates, pages, snippets, authors, tags,
/// categories and settings of the site in `site`, the super-admin routes
/// over `panel`, and comments on drafts and newsletters emailed through
/// `mailer` when the site sends mail. The settings and schema templates
/// read `site.toml` with `env`, the environment the server started with.
pub(crate) fn api_router(
    (content, door, posts): (LiveContent, Arc<PublishDoor>, SitePosts),
    (comments, media, redirects, follow_ups, (ai, schema_copier)): Desks,
    (database, id, (site, config, env)): (&Database, SiteId, (&Path, &Config, &Environment)),
    (panel, mailer): (Arc<dyn SiteDirectory>, Option<Arc<dyn Mailer>>),
) -> Router {
    let table = SubscriberTable::new(database.clone(), id);
    let readers =
        subscribers::SiteSubscribers::new(table, (site, config), posts.clone(), mailer.clone());
    let profiles = Arc::new(SiteAuthorProfiles::new(site, config));
    let accounts = staff::accounts(database, id, config).with_author_profiles(profiles);
    let sessions: Arc<dyn SessionCheck> = Arc::new(accounts.clone());
    let routes = khalid_auth::staff_routes(accounts.clone());
    let staff = (khalid_auth::with_sites(routes, accounts, panel), sessions);
    let desks = AdminDesks {
        comments,
        media: Some(media),
        redirects,
        webhooks: Some(Arc::new(WebhookTable::new(
            database.clone(),
            id,
            SystemClock,
        ))),
        trash: Some(Arc::new(posts.clone())),
        audit: Some(AuditDesk::new(
            Arc::new(AuditTable::new(database.clone(), id)),
            SystemClock,
        )),
        // The same posts the door publishes, judged with the live settings.
        checks: Some(Arc::new(posts)),
        schema_templates: Some(Arc::new(SiteSchemaTemplates::new(site, config, env))),
        schema_copier: schema_copier.map(|copier| -> Arc<dyn SchemaCopier> { Arc::new(copier) }),
        pages: Some(Arc::new(SiteEntries::<AdminPage>::new(site, config))),
        snippets: Some(Arc::new(SiteEntries::<AdminSnippet>::new(site, config))),
        authors: Some(Arc::new(SiteEntries::<AdminAuthor>::new(site, config))),
        tags: Some(Arc::new(SiteTags::new(site, config))),
        categories: Some(Arc::new(SiteCategories::new(site, config))),
        settings: Some(Arc::new(SiteSettings::new(site, config, env))),
        design: Some(DesignDesk::new(site, look::pack_tokens(site, env))),
        templates: Some(Arc::from(look::admin_templates())),
        analytics: AnalyticsDesk::new(
            Arc::new(AnalyticsTable::new(database.clone(), id)),
            Arc::new(SystemClock),
        )
        .with_consent(Arc::new(ConsentProofTable::new(database.clone(), id))),
        draft_comments: Some(draft_comment_mail::desk((database, id, config), mailer)),
        follow_ups: Arc::new(follow_ups),
        forms: Some(Arc::new(FormTable::new(database.clone(), id))),
        serpdad: Some(Arc::new(SerpdadTable::new(
            database.clone(),
            id,
            SystemClock,
        ))),
        ai,
        limits: *config.limits(),
        subscribers: Some(Arc::new(readers)),
        uptime: Some(Arc::new(SiteUptime::new(
            database,
            id,
            Arc::new(SystemClock),
        ))),
    };
    let keys: Arc<dyn KeyStore> = Arc::new(KeyTable::new(database.clone(), id, SystemClock));
    let content: Arc<dyn ContentStore> = Arc::new(content);
    let ghost_api = config.ghost_content_api().then(|| {
        let site = GhostSite::from_config(config);
        ghost::router(Arc::clone(&content), Arc::clone(&keys), site)
    });
    let (api, _) = api::router_for_serve(content, door, keys, desks, staff);
    match ghost_api {
        Some(ghost_api) => api.merge(ghost_api),
        None => api,
    }
}

/// Where the APIs answer for the site in `config`: `/api/`, and Ghost's
/// Content API when the site switches it on (`docs/ghost-api.md`). Off,
/// a request there never builds the APIs and is any unknown address.
fn api_roots(config: &Config) -> &'static [&'static str] {
    if config.ghost_content_api() {
        &[late_api::UNDER, late_api::GHOST]
    } else {
        &[late_api::UNDER]
    }
}

/// `app` with the router of [`api_router`] and git mode's push `hook`
/// answering under `/api/`, and under the blog's mount (see [`mounted`]),
/// built on a blocking thread when the first
/// request under `/api/` arrives (ADR "the APIs are built on their first
/// request"). An address
/// under `/api/` the APIs lack goes to `app`, as it did when the routers
/// were merged before the first answer.
pub(crate) fn with_apis(
    app: Router,
    (apis, desks, at, helpers): ApiParts<'_>,
    hook: Router,
) -> Router {
    let (database, id, (site, config, env)) = at;
    let (database, site, config) = (database.clone(), site.to_path_buf(), config.clone());
    let (rest, env) = (app.clone(), env.clone());
    let blog = config.clone();
    let app = late_api::router_under(app, api_roots(&blog), move || {
        let at = (&database, id, (site.as_path(), &config, &env));
        api_router(apis, desks, at, helpers)
            .merge(hook)
            .fallback_service(rest)
    });
    mounted::router(&blog, app)
}
