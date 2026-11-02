//! A series: posts written to be read in order, as a post in one shows it.

use serde::Serialize;

use crate::Link;

/// The series a post is one part of: where it stands, the parts on either
/// side, and every part in order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderSeries {
    /// The series' name, such as `Topic clusters from scratch`.
    pub name: String,
    /// The series' own page, which lists every part.
    pub href: String,
    /// This post's place, counted from 1.
    pub part: usize,
    /// How many parts the series has.
    pub total: usize,
    /// The part before this one, if any.
    pub previous: Option<Link>,
    /// The part after this one, if any.
    pub next: Option<Link>,
    /// Every part in order, this one included.
    pub parts: Vec<Link>,
}
