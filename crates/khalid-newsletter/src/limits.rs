//! How often one reader may use the sign-up form (RULES.md BL-SEC-5).
//! Each sign-up sends an email from the site's own account, so without a
//! limit one script could mail thousands of inboxes and spoil the site's
//! name with mail providers. The limits themselves live with the caller.

use std::fmt;
use std::net::IpAddr;
use std::time::Duration;

use axum::http::HeaderMap;

/// What the limits say about one more sign-up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SignUpCheck {
    /// The sign-up may go ahead, and it is counted.
    Allowed,
    /// The reader sent too many; they may try again after this long.
    Wait(Duration),
    /// The limits could not be checked, so the sign-up must not go ahead.
    Unknown,
}

/// Counts sign-ups per reader. The router asks it once for each sign-up
/// that names a valid address and list, before any email is sent.
pub trait SignUpLimits: Send + Sync + fmt::Debug {
    /// Counts a sign-up from the connection at `peer`, sent with
    /// `headers`, when the limits allow it. A refused sign-up is not
    /// counted.
    fn check(&self, peer: IpAddr, headers: &HeaderMap) -> SignUpCheck;
}
