//! Blocks that lay out a post's own words: pros and cons and columns side
//! by side (templates spec 4.4). They carry no schema.

/// One card of a `::::columns` block, written `:::column{title="..."}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    /// The words over the card, when it has some.
    pub title: Option<String>,
    /// The card's markdown.
    pub text: String,
}

/// `:::proscons`: what is good and what is not about one thing, as two
/// lists. One `+ ` line per pro and one `- ` line per con.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProsConsBlock {
    /// The words over the pros, `Pros` unless the block says otherwise.
    pub pros_title: String,
    /// The words over the cons, `Cons` unless the block says otherwise.
    pub cons_title: String,
    /// Each pro's markdown, in order.
    pub pros: Vec<String>,
    /// Each con's markdown, in order.
    pub cons: Vec<String>,
}
