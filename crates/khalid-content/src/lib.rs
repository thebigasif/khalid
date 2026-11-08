//! The files that are Khalid's source of truth (RULES.md BL-FS-1).
//!
//! This crate owns reading content: front matter, markdown and the content
//! tree of language folders, collections and posts. It must not know about
//! templates, HTTP or the database; it hands typed content to the crates that
//! render, serve and index it, and never writes derived data under
//! `content/` (BL-FS-2).

mod block;
mod body_check;
mod chart;
mod content_error;
mod content_hash;
mod directive;
mod embed;
mod form;
mod front_matter;
mod layout;
mod maths;
mod post;
mod post_fields;
mod render;
mod schemas;
mod site_content;
mod structured;
mod tree;
mod wikilink;

pub use block::{
    Block, BlockError, BlockKind, CalloutKind, GalleryImage, Visibility, parse_blocks, print_blocks,
};
pub use body_check::check_bodies;
pub use chart::{
    ChartBlock, ChartKind, ChartRow, ChartTable, ChartValue, MAX_CHART_ROWS, MAX_CHART_SERIES,
    with_chart_files,
};
pub use content_error::ContentError;
pub use content_hash::ContentHash;
pub use embed::{EMBED_PROVIDERS, EmbedProvider};
pub use form::{FieldKind, FormField};
pub use layout::{Column, ProsConsBlock};
pub use post::{
    MAX_RELATED, MAX_SECONDARY_KEYWORDS, MAX_STOCK_TICKERS, Post, PostFront, parse_post,
};
pub use post_fields::{
    LeadImage, MAX_FACT_LABEL, MAX_FACT_VALUE, MAX_FACTS, PostFact, PostSeries, ReviewState,
    ShareCard, ShareCards, Sitemap, TopicCluster, TopicRole,
};
pub use render::{
    CTA_MARK, ENGINE_TEXT, Gated, Heading, Image, LOCATION_MARK, Link, PRIVACY_FACTS_MARK,
    Rendered, render, render_framed, render_gated, render_linked,
};
pub use schemas::{
    AuthorFront, CaseStudyFront, ChangelogFront, GlossaryFront, IndexFront, PageFront, PageHub,
    SnippetFront,
};
pub use site_content::{Document, SiteContent, read_content};
pub use structured::{
    BusinessBlock, HowToBlock, ProductBlock, Question, REVIEW_TYPES, ReaderRatings, RecipeBlock,
    ReviewBlock, Step, StructuredBlock, VideoBlock,
};
pub use tree::{Collection, ContentTree, Entry, Language};
pub use wikilink::{GLOSSARY, WikiTarget, Wikilinks};
