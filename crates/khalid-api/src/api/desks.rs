//! The Admin API's stores beside posts. Kept apart from `api.rs` so that
//! file stays under 600 lines (BL-SIZE-1).

use std::sync::Arc;

use khalid_core::LimitSettings;

use crate::ai::AiDesk;
use crate::analytics_desk::AnalyticsDesk;
use crate::audit_desk::AuditDesk;
use crate::comment_moderation::{CommentModeration, NoComments};
use crate::design_desk::DesignDesk;
use crate::draft_comments::DraftComments;
use crate::entry_store::EntryStore;
use crate::follow_ups::{FollowUpFinder, NoSearchConsole};
use crate::form_answers::FormAnswers;
use crate::media_api::MediaLibrary;
use crate::post_check::PostChecks;
use crate::post_trash::PostTrash;
use crate::redirect_desk::RedirectDesk;
use crate::resources::{AdminAuthor, AdminCategory, AdminPage, AdminSnippet, AdminTemplate};
use crate::schema_copy::SchemaCopier;
use crate::schema_template_store::SchemaTemplateStore;
use crate::serpdad_store::SerpdadStore;
use crate::settings_store::SettingsStore;
use crate::subscriber_desk::SubscriberDesk;
use crate::tag_store::TagStore;
use crate::uptime::UptimeReader;
use crate::webhook_store::WebhookStore;

/// The Admin API's stores beside posts, as `khalid serve` holds them.
pub struct AdminDesks {
    /// Readers' comments, moderated under `/api/admin/v1/comments/`.
    pub comments: Arc<dyn CommentModeration>,
    /// The media library, or `None` when the media routes answer that the
    /// site has none.
    pub media: Option<MediaLibrary>,
    /// The stored redirects and the 404 log.
    pub redirects: RedirectDesk,
    /// The site's webhooks, or `None` when the webhook routes list none
    /// and answer that the site keeps none.
    pub webhooks: Option<Arc<dyn WebhookStore>>,
    /// The audit log every write that succeeds is kept in, or `None` when
    /// nothing is logged and the audit route lists nothing.
    pub audit: Option<AuditDesk>,
    /// Where deleted posts wait, or `None` when the trash routes list none.
    pub trash: Option<Arc<dyn PostTrash>>,
    /// The post checks behind the editor's SEO panel, or `None` when the
    /// check route answers that this server makes none.
    pub checks: Option<Arc<dyn PostChecks>>,
    /// The site's schema templates, or `None` when their routes answer
    /// that this server keeps none.
    pub schema_templates: Option<Arc<dyn SchemaTemplateStore>>,
    /// What fetches a page to copy its JSON-LD, or `None` when the copy
    /// route answers that this server fetches no pages.
    pub schema_copier: Option<Arc<dyn SchemaCopier>>,
    /// The site's pages, or `None` when the page routes answer that this
    /// server keeps none.
    pub pages: Option<Arc<dyn EntryStore<AdminPage>>>,
    /// The site's snippets, or `None` when the snippet routes answer that
    /// this server keeps none.
    pub snippets: Option<Arc<dyn EntryStore<AdminSnippet>>>,
    /// The site's authors, or `None` when the author routes answer that
    /// this server keeps none.
    pub authors: Option<Arc<dyn EntryStore<AdminAuthor>>>,
    /// The site's tags, or `None` when the tag routes answer that this
    /// server keeps none.
    pub tags: Option<Arc<dyn TagStore>>,
    /// The site's categories, or `None` when the category routes answer
    /// that this server keeps none.
    pub categories: Option<Arc<dyn EntryStore<AdminCategory>>>,
    /// The site's core settings, or `None` when the settings routes answer
    /// that this server keeps none.
    pub settings: Option<Arc<dyn SettingsStore>>,
    /// The site's `theme/tokens.toml`, or `None` when the Design route
    /// answers that this server keeps none.
    pub design: Option<DesignDesk>,
    /// The built-in templates, or `None` when the template route answers
    /// that this server lists none.
    pub templates: Option<Arc<[AdminTemplate]>>,
    /// The views `khalid serve` counted, read under `/api/admin/v1/analytics/`.
    pub analytics: AnalyticsDesk,
    /// Staff comments on drafts, or `None` when the routes list none and
    /// answer that the site keeps none.
    pub draft_comments: Option<DraftComments>,
    /// The Search Console follow-ups, under
    /// `/api/admin/v1/search-console/follow-ups/`.
    pub follow_ups: Arc<dyn FollowUpFinder>,
    /// What visitors sent through form blocks, or `None` when the form
    /// routes answer that this server keeps none.
    pub forms: Option<Arc<dyn FormAnswers>>,
    /// The `SerpDad` connection, or `None` when the `SerpDad` route answers
    /// that the site keeps no database.
    pub serpdad: Option<Arc<dyn SerpdadStore>>,
    /// Who writes the editor's suggestions, under `/api/admin/v1/ai/`;
    /// [`AiDesk::off`] sends nothing anywhere.
    pub ai: AiDesk,
    /// How many requests each key and address may make a minute.
    pub limits: LimitSettings,
    /// Newsletter subscribers and sends, or `None` when the subscriber
    /// routes list none and the newsletter routes answer that the site
    /// cannot send.
    pub subscribers: Option<Arc<dyn SubscriberDesk>>,
    /// The speed and uptime watch's looks, or `None` when the watch route
    /// answers that this server keeps none.
    pub uptime: Option<Arc<dyn UptimeReader>>,
}

impl std::fmt::Debug for AdminDesks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdminDesks")
            .field("media", &self.media)
            .field("redirects", &self.redirects)
            .finish_non_exhaustive()
    }
}

impl AdminDesks {
    /// No comments, no media library, no stored redirects, no webhooks, no
    /// audit log, no trash, no post checks, no schema templates, no schema
    /// copier, no pages,
    /// no snippets, no authors, no tags, no categories, no settings, no design, no
    /// templates, no counted views, no comments on drafts, no Search Console connection,
    /// no form answers, no `SerpDad` connection, no writing suggestions, no
    /// subscribers and no watch looks, with the usual limits.
    pub fn none() -> Self {
        Self {
            comments: Arc::new(NoComments),
            media: None,
            redirects: RedirectDesk::none(),
            webhooks: None,
            audit: None,
            trash: None,
            checks: None,
            schema_templates: None,
            schema_copier: None,
            pages: None,
            snippets: None,
            authors: None,
            tags: None,
            categories: None,
            settings: None,
            design: None,
            templates: None,
            analytics: AnalyticsDesk::none(),
            draft_comments: None,
            follow_ups: Arc::new(NoSearchConsole),
            forms: None,
            serpdad: None,
            ai: AiDesk::off(),
            limits: LimitSettings::default(),
            subscribers: None,
            uptime: None,
        }
    }
}
