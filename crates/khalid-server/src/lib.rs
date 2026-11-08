//! The web server for the public site.
//!
//! This crate owns answering HTTP. It holds the rendered pages in memory
//! ([`Pages`]), each at a clean [`PagePath`] with its [`MediaType`], and the
//! [`router`] serves them, redirecting the addresses that differ only by a
//! trailing slash or `index.html`, and sending each [`Moved`] address on
//! with 301. Every response carries the
//! [`SecurityHeaders`], with a Content Security Policy whose nonce is new
//! for each request (BL-SEC-2). On a site that asks readers for consent,
//! [`router_with_consent`] answers through [`filled`], which makes each
//! page for the [`Reader`], with the [`HeldCode`] of each category they
//! allowed. [`serve`] answers until told to stop, then
//! stops gracefully (BL-RUN-10). Beside the requests it runs two
//! [`Background`] jobs: [`watch`], which renders the pages again when a file
//! changes, and [`retry_deliveries`], which sends due webhooks again.
//!
//! It must not know about markdown, templates or the content tree. Pages
//! arrive already rendered, because the render path does no work at request
//! time (RULES.md BL-PERF-2); a request copies bytes.

mod background;
mod clean_url;
mod compress;
mod consent;
mod consent_cookie;
mod etag;
mod live_pages;
mod media_type;
mod moved;
mod page;
mod page_path;
mod pages;
mod range;
mod render;
mod respond;
mod retry;
mod security;
mod serve;
mod server_error;
mod watch;
mod watch_events;

pub use background::Background;
pub use consent::{
    ASK_END, ASK_START, CHOSEN_END, CHOSEN_START, ConsentSite, GPC_END, GPC_START, HELD_END,
    HELD_START, HeldCode, MEDIA_END, MEDIA_START, NO_GPC_END, NO_GPC_START, SAVED_END, SAVED_QUERY,
    SAVED_START, SITE_BODY_SLOT, SITE_HEAD_SLOT, SavedNotice, filled, under_gpc,
};
pub use consent_cookie::{Answer, CONSENT_COOKIE, Categories, GPC_HEADER, Reader};
pub use live_pages::{LivePages, Replacements};
pub use media_type::MediaType;
pub use moved::Moved;
pub use page::{NONCE_SLOT, Page};
pub use page_path::{PagePath, ParsePagePathError};
pub use pages::Pages;
pub use range::ranged;
pub use render::{Render, Rendered, render_pages};
pub use respond::{drawn_for, router, router_with_consent};
pub use retry::retry_deliveries;
pub use security::SecurityHeaders;
pub use serve::{Stopped, serve};
pub use server_error::ServerError;
pub use watch::{FolderLook, poll, watch};
