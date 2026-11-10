//! Shared building blocks for every Khalid crate.
//!
//! This crate owns the types the other crates agree on: the message a person
//! reads ([`UserMessage`]), validated identifiers such as [`Slug`], [`Locale`]
//! and [`AbsoluteUrl`], the [`Clock`], the [`Config`], the [`Secret`] wrapper,
//! [`SitePath`], [`RobotsDirectives`], which front matter and the SEO
//! outputs share, the [`IndexNowKey`] that settings and the SEO pings
//! share, and a post's [`PostAccess`], [`TemplateName`], [`PasswordHash`]
//! and checked [`HeadCode`], which front matter, the Admin API and the
//! server share, the site's own [`SiteCode`], each entry tagged with a
//! [`CodeCategory`], and the [`PrivacySettings`] that decide how readers
//! are asked. It must not know about HTTP or SQL. It depends on no other Khalid crate, and its only
//! I/O is reading the system clock and resolving a [`SitePath`] on disk.

mod absolute_url;
mod admin_folder;
mod ai_page_settings;
mod ai_settings;
mod alert_settings;
mod clock;
mod config;
mod environment;
mod geo_point;
mod head_code;
mod indexnow_key;
mod limit_settings;
mod link_settings;
mod link_tracking;
mod locale;
mod mail_settings;
mod media_settings;
mod post_access;
mod privacy_settings;
mod reader;
mod robots;
mod schema_types;
mod search_console_settings;
mod secret;
mod seo_breadcrumb_settings;
mod seo_head_settings;
mod seo_publisher_settings;
mod seo_search_settings;
mod seo_settings;
mod site_code;
mod site_nav;
mod site_path;
mod slug;
mod slug_from_title;
mod template_choice;
mod term_seo;
pub mod toml_location;
mod user_message;
mod visitor_log_settings;

pub use absolute_url::{AbsoluteUrl, ParseUrlError};
pub use admin_folder::{AdminFolder, ParseAdminFolderError};
pub use ai_page_settings::{AiPageSettings, AiPageText, LlmsTxtSettings};
pub use ai_settings::{AiProvider, AiSettings};
pub use alert_settings::AlertSettings;
#[cfg(any(test, feature = "test-util"))]
pub use clock::FixedClock;
pub use clock::{Clock, SystemClock};
pub use config::{Config, ConfigError, Mode, SlugMode};
pub use environment::Environment;
pub use geo_point::{GeoPoint, GeoPointError};
pub use head_code::{HeadCode, HeadCodeError};
pub use indexnow_key::{IndexNowKey, ParseIndexNowKeyError};
pub use limit_settings::LimitSettings;
pub use link_settings::{DomainName, ExternalRel, LinkOpening, LinkSettings, ParseDomainNameError};
pub use link_tracking::{LinkTracking, ParseTagValueError, TagValue};
pub use locale::{Locale, ParseLocaleError};
pub use mail_settings::{
    MailSettings, MailTransport, PostmarkSettings, SmtpSecurity, SmtpSettings,
};
pub use media_settings::{AvifSetting, MediaSettings, S3Keys, S3Settings};
pub use post_access::{
    ParsePasswordHashError, ParseTemplateNameError, PasswordHash, PostAccess, TemplateName,
};
pub use privacy_settings::{
    ConsentCategory, ConsentRev, GpcMode, PrivacyContact, PrivacyImprint, PrivacySettings,
    PrivacyText, SettingsVersion,
};
pub use reader::{HomeSection, Reader, ReaderSwitch};
pub use robots::{
    Following, ImagePreview, Indexing, ParseRobotsError, PreviewLimit, Restriction,
    RobotsDirectives,
};
pub use schema_types::{ArticleSchemaType, PageSchemaType};
pub use search_console_settings::{SearchConsoleProperty, SearchConsoleSettings};
pub use secret::Secret;
pub use seo_breadcrumb_settings::{BreadcrumbSettings, BreadcrumbTaxonomy};
pub use seo_head_settings::{
    AiTraining, CrawlerSettings, FeedContentSetting, FeedKind, FeedSettings, ImageSettings,
    SocialSettings, TemplateSettings, TitleSettings, VerificationSettings,
};
pub use seo_publisher_settings::{
    HoursSettings, LocalSettings, LocationSettings, PublisherSettings, PublisherType,
    SchemaMapSettings, SchemaTemplateSettings, WeekdaySetting,
};
pub use seo_search_settings::SearchFilterSettings;
pub use seo_settings::{ArchiveIndex, ArchiveSetting, IndexNowSettings, SeoSettings};
pub use site_code::{BodyCode, BodyCodeError, CodeCategory, CodeEntry, Consent, SiteCode};
pub use site_nav::{
    Announcement, FooterGroup, MenuLink, NavSettings, OneLine, SiteHref, SiteNavError,
};
pub use site_path::{SitePath, SitePathError};
pub use slug::{ParseSlugError, Slug};
pub use template_choice::{CallToAction, OptionValue, PackName, SourcePlace, TemplateChoice};
pub use term_seo::TermSeo;
pub use user_message::UserMessage;
pub use visitor_log_settings::VisitorLogSettings;
