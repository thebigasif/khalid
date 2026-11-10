//! Why a body's blocks could not be read. Kept apart from `block` to keep
//! each file short (BL-SIZE-1); `block` re-exports it.

use khalid_core::ParseUrlError;

use crate::directive::unknown_hint;

/// Why a body's blocks could not be read. Lines count from 1 in the body.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum BlockError {
    /// A directive name Khalid does not know.
    #[error("line {line}: there is no block called {name}; {}", unknown_hint(name))]
    UnknownBlock {
        /// The line of the opening `:::`.
        line: usize,
        /// The name as written.
        name: String,
    },
    /// The braces after the name do not hold `name=value` pairs.
    #[error(
        "line {line}: the attributes in braces cannot be read; write each as name=value, in double quotes when the value has a space"
    )]
    Attributes {
        /// The line of the opening `:::`.
        line: usize,
    },
    /// A required attribute is missing.
    #[error("line {line}: {problem}")]
    Missing {
        /// The line of the opening `:::`.
        line: usize,
        /// What is missing, in a sentence.
        problem: &'static str,
    },
    /// An attribute the block does not take.
    #[error(
        "line {line}: {} does not take {attribute}; it takes {takes}",
        a(block)
    )]
    UnknownAttribute {
        /// The line of the opening `:::`.
        line: usize,
        /// The block's name.
        block: &'static str,
        /// The attribute as written.
        attribute: String,
        /// The attributes the block takes.
        takes: &'static str,
    },
    /// A callout kind Khalid does not know.
    #[error(
        "line {line}: a callout's kind is note, tip, warning, summary, read or stat, not {kind}"
    )]
    CalloutKind {
        /// The line of the opening `:::`.
        line: usize,
        /// The kind as written.
        kind: String,
    },
    /// A block written as a container when it is a one-line leaf, or the
    /// other way round.
    #[error("line {line}: {problem}")]
    Form {
        /// The line of the block.
        line: usize,
        /// Which form to use instead, in a sentence.
        problem: String,
    },
    /// An attribute's value is not written the way the block needs it.
    #[error("line {line}: {problem}")]
    Value {
        /// The line of the block or of the wrong line in its body.
        line: usize,
        /// How to write it, in a sentence.
        problem: &'static str,
    },
    /// A button whose link could run a script or leave the web.
    #[error(
        "line {line}: a button links to a page, a #section, https:, http: or mailto:, not {href}"
    )]
    Link {
        /// The line of the block.
        line: usize,
        /// The link as written.
        href: String,
    },
    /// An attribute that must be a full web address is not one.
    #[error("line {line}: {}'s {attribute} is wrong: {source}", a(block))]
    Address {
        /// The line of the block.
        line: usize,
        /// The block's name.
        block: &'static str,
        /// The attribute that holds the address.
        attribute: &'static str,
        /// Why the address cannot be used.
        #[source]
        source: ParseUrlError,
    },
    /// A maths block whose formula cannot be drawn.
    #[error("line {line}: the maths cannot be drawn: {problem}")]
    Maths {
        /// The formula's first line.
        line: usize,
        /// What the maths reader found, in a phrase.
        problem: String,
    },
    /// An opening `:::` with no closing `:::`.
    #[error("line {line}: the {name} block opened here is never closed with a ::: line")]
    Unclosed {
        /// The line of the opening `:::`.
        line: usize,
        /// The block's name.
        name: String,
    },
}

impl BlockError {
    /// The body line the problem is on, counting from 1.
    #[must_use]
    pub fn line(&self) -> usize {
        match self {
            Self::UnknownBlock { line, .. }
            | Self::Attributes { line }
            | Self::Missing { line, .. }
            | Self::UnknownAttribute { line, .. }
            | Self::CalloutKind { line, .. }
            | Self::Form { line, .. }
            | Self::Link { line, .. }
            | Self::Address { line, .. }
            | Self::Value { line, .. }
            | Self::Maths { line, .. }
            | Self::Unclosed { line, .. } => *line,
        }
    }

    /// What is wrong, in a sentence, without the line.
    #[must_use]
    pub fn problem(&self) -> String {
        let text = self.to_string();
        let prefix = format!("line {}: ", self.line());
        text.strip_prefix(&prefix)
            .map_or_else(|| text.clone(), str::to_owned)
    }
}

/// `block` with "a" or "an" before it.
fn a(block: &str) -> String {
    let article = if block.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "an"
    } else {
        "a"
    };
    format!("{article} {block}")
}
