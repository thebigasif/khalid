//! Newsletter subscribers and newsletters as the Admin API sends them
//! (G39, plan 4.6).

use chrono::{DateTime, Utc};
use khalid_core::Slug;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::display;

/// Where a subscriber stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SubscriberStatus {
    /// Signed up, and has not opened the confirmation link yet. Gets no
    /// newsletter until they do.
    Pending,
    /// Opened the confirmation link, and gets their newsletters.
    Confirmed,
    /// Left through an unsubscribe link, and gets nothing more.
    Unsubscribed,
}

/// One subscriber in Admin API shape.
///
/// Exhaustive on purpose: the database fills it in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct AdminSubscriber {
    /// Its identity.
    pub id: i64,
    /// Where their email goes.
    #[schema(example = "lena@example.com")]
    pub email: String,
    /// Where they stand.
    pub status: SubscriberStatus,
    /// The newsletters they get, in alphabetical order. Empty once they
    /// unsubscribed.
    #[schema(value_type = Vec<String>, example = json!(["main", "weekly"]))]
    #[serde(serialize_with = "slugs")]
    pub lists: Vec<Slug>,
    /// Their labels, such as `paid`, in alphabetical order.
    #[schema(value_type = Vec<String>, example = json!(["paid"]))]
    #[serde(serialize_with = "slugs")]
    pub labels: Vec<Slug>,
    /// When they first signed up.
    pub created_at: DateTime<Utc>,
    /// When they opened the confirmation link, if they did.
    pub confirmed_at: Option<DateTime<Utc>>,
    /// When they unsubscribed, if they did.
    pub unsubscribed_at: Option<DateTime<Utc>>,
}

/// Which subscribers a list asks for. Every part left out matches everyone.
///
/// Exhaustive on purpose: the route fills it in and the database reads it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SubscriberFilter {
    /// Only subscribers who stand here.
    pub status: Option<SubscriberStatus>,
    /// Only subscribers who get this newsletter.
    pub list: Option<Slug>,
    /// Only subscribers who carry this label.
    pub label: Option<Slug>,
    /// Only subscribers whose address holds this text, in any case.
    pub search: Option<String>,
}

/// One page of subscribers, newest first, and how many match in all.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SubscriberPage {
    /// The subscribers on this page.
    pub subscribers: Vec<AdminSubscriber>,
    /// How many subscribers match the filter, on every page.
    pub total: u64,
}

impl SubscriberPage {
    /// `subscribers`, out of `total` that match.
    pub fn new(subscribers: Vec<AdminSubscriber>, total: u64) -> Self {
        Self { subscribers, total }
    }
}

/// One newsletter and how many readers it has.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
#[non_exhaustive]
pub struct AdminNewsletter {
    /// Its name, such as `main` or `weekly`.
    #[serde(serialize_with = "display")]
    #[schema(value_type = String, example = "weekly")]
    pub name: Slug,
    /// The readers who confirmed and get it.
    pub confirmed: u64,
    /// The readers who signed up and have not confirmed yet.
    pub pending: u64,
    /// A sign-up form to paste into another website, such as the main
    /// site. It works with scripts off.
    pub form: String,
}

impl AdminNewsletter {
    /// The newsletter `name` with `confirmed` and `pending` readers, joined
    /// through `form`.
    pub fn new(name: Slug, (confirmed, pending): (u64, u64), form: String) -> Self {
        Self {
            name,
            confirmed,
            pending,
            form,
        }
    }
}

/// The totals of one email that went out. Totals only: who got a copy is
/// never kept.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
#[non_exhaustive]
pub struct AdminSend {
    /// The key it went out under, such as `post:en/hello`.
    #[schema(example = "post:en/hello")]
    pub key: String,
    /// The newsletter.
    #[serde(serialize_with = "display")]
    #[schema(value_type = String, example = "main")]
    pub list: Slug,
    /// When it started going out.
    pub started_at: DateTime<Utc>,
    /// The copies sent.
    pub sent: u32,
    /// The copies the mail service refused. They are not sent again.
    pub failed: u32,
    /// The readers who unsubscribed with a link from it.
    pub unsubscribed: u32,
}

impl AdminSend {
    /// The email sent under `key` to `list` at `started_at`, with its
    /// `(sent, failed, unsubscribed)` totals.
    pub fn new(
        key: String,
        list: Slug,
        started_at: DateTime<Utc>,
        (sent, failed, unsubscribed): (u32, u32, u32),
    ) -> Self {
        Self {
            key,
            list,
            started_at,
            sent,
            failed,
            unsubscribed,
        }
    }
}

/// Whether the site can send a newsletter now, and if not, what it lacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SendReadiness {
    /// Mail and a postal address are set: a post can go out.
    Ready,
    /// `site.toml` has no `[mail]`, so no email can leave.
    NoMail,
    /// `[mail]` has no postal address, which every newsletter email shows.
    NoPostalAddress,
}

/// A published post to email to one newsletter.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct NewsletterSend {
    /// The post's identity, such as `en.hello`.
    #[schema(example = "en.hello")]
    pub post: String,
    /// Send only to subscribers who carry every one of these labels.
    #[serde(default)]
    #[schema(value_type = Vec<String>, example = json!(["paid"]))]
    pub labels: Vec<Slug>,
}

/// Writes slugs as a list of strings.
fn slugs<S: serde::Serializer>(slugs: &[Slug], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(slugs.iter().map(Slug::as_str))
}
