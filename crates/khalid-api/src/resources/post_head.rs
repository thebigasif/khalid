//! The SEO head of a post's page, for a front end that renders the post
//! itself (headless mode, KH-INS-5).

use std::borrow::Cow;
use std::sync::Arc;

use brotli::enc::BrotliEncoderParams;
use khalid_seo::{HeadTag, HeadTags, SchemaGraph};
use serde::ser::Error as _;
use serde::{Deserialize, Serialize, Serializer};
use utoipa::ToSchema;

/// How the theme's page starts its JSON-LD, which [`HeadParts::json_ld`]
/// leaves out.
const SCRIPT_OPEN: &str = "<script type=\"application/ld+json\">";

/// How it ends.
const SCRIPT_CLOSE: &str = "</script>\n";

/// Brotli quality for a packed head. A head is a few kilobytes that
/// repeat the site's addresses, so quality 5 makes it about a third of
/// its JSON in tens of microseconds.
const QUALITY: i32 = 5;

/// The Brotli window for a packed head, as a power of two: 128 KB, the
/// smallest the `brotli` crate has a small hasher for at quality 5, and
/// far more than a head needs.
const WINDOW: i32 = 17;

/// What a post's page carries in its `<head>` for search engines and share
/// cards: the tags in the order the theme writes them, then the JSON-LD
/// graph. A front end that prints each tag in order, then the graph in a
/// `<script type="application/ld+json">` tag, has the theme's head.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostHead(Arc<Held>);

/// How a head is held: as its parts, or packed for a store that keeps a
/// head for every post and sends few of them.
#[derive(Debug, PartialEq, Eq)]
enum Held {
    /// The tags and the graph, as made.
    Parts(HeadParts),
    /// The parts as JSON, compressed with Brotli: a third of the memory
    /// on an idle server, decoded when a front end asks for the post.
    Packed(Box<[u8]>),
}

/// A post page's head: its tags in the order the theme writes them, then
/// its JSON-LD graph. A post holds them behind a shared pointer, so the
/// store copies no text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct HeadParts {
    /// The title, meta and link tags, in order.
    tags: Vec<HeadElement>,
    /// The schema graph as compact JSON, with each less-than sign escaped,
    /// so it can go inside a script tag as it is.
    #[schema(example = "{\"@context\":\"https://schema.org\",\"@graph\":[]}")]
    json_ld: String,
}

/// One tag of a post page's head. Values are plain text: a front end
/// escapes them as it writes the tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "tag", rename_all = "lowercase")]
#[non_exhaustive]
pub enum HeadElement {
    /// `<title>text</title>`.
    Title {
        /// The page title.
        text: String,
    },
    /// `<meta name="..." content="...">`, or with `property` for Open Graph.
    Meta {
        /// The `name` attribute, such as `description` or `twitter:card`.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// The `property` attribute, such as `og:title`.
        #[serde(skip_serializing_if = "Option::is_none")]
        property: Option<String>,
        /// The `content` attribute.
        content: String,
    },
    /// `<link rel="..." hreflang="..." href="...">`, `hreflang` only on a
    /// language version.
    Link {
        /// The `rel` attribute, such as `canonical` or `alternate`.
        rel: String,
        /// The language of an `alternate` version, or `x-default`.
        #[serde(skip_serializing_if = "Option::is_none")]
        hreflang: Option<String>,
        /// The full address.
        href: String,
    },
}

impl PostHead {
    /// The head of a page with these SEO `tags` and schema `graph`, as
    /// `khalid-seo` writes them into the theme's page.
    pub fn new(tags: &HeadTags, graph: &SchemaGraph) -> Self {
        let script = graph.to_script_tag();
        let json_ld = script
            .strip_prefix(SCRIPT_OPEN)
            .and_then(|rest| rest.strip_suffix(SCRIPT_CLOSE))
            .unwrap_or_default()
            .to_owned();
        let tags = tags.as_slice().iter().filter_map(element).collect();
        Self(Arc::new(Held::Parts(HeadParts { tags, json_ld })))
    }

    /// The same head packed small, for a store that keeps one for every
    /// post: it reads and prints as before, and is decoded each time.
    /// A head already packed, or one that cannot be packed, is shared.
    #[must_use]
    pub fn packed(&self) -> Self {
        let Held::Parts(parts) = &*self.0 else {
            return self.clone();
        };
        let params = BrotliEncoderParams {
            quality: QUALITY,
            lgwin: WINDOW,
            ..BrotliEncoderParams::default()
        };
        let mut output = Vec::new();
        let packed = serde_json::to_vec(parts).ok().and_then(|json| {
            brotli::BrotliCompress(&mut json.as_slice(), &mut output, &params).ok()
        });
        match packed {
            Some(_) => Self(Arc::new(Held::Packed(output.into_boxed_slice()))),
            None => self.clone(),
        }
    }

    /// Whether the head is held packed.
    pub fn is_packed(&self) -> bool {
        matches!(*self.0, Held::Packed(_))
    }

    /// The tags and the graph, decoded first when the head is packed.
    /// `None` only when packed bytes cannot be read back, which
    /// [`PostHead::packed`] never writes.
    pub fn parts(&self) -> Option<Cow<'_, HeadParts>> {
        match &*self.0 {
            Held::Parts(parts) => Some(Cow::Borrowed(parts)),
            Held::Packed(bytes) => {
                let mut json = Vec::new();
                brotli::BrotliDecompress(&mut &bytes[..], &mut json).ok()?;
                serde_json::from_slice(&json).ok().map(Cow::Owned)
            }
        }
    }
}

impl HeadParts {
    /// The tags, in order.
    pub fn tags(&self) -> &[HeadElement] {
        &self.tags
    }

    /// The schema graph as compact JSON, safe inside a script tag.
    pub fn json_ld(&self) -> &str {
        &self.json_ld
    }
}

impl Serialize for PostHead {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let parts = self.parts();
        let parts =
            parts.ok_or_else(|| S::Error::custom("a packed post head could not be read"))?;
        parts.serialize(serializer)
    }
}

/// `tag` in API shape. `None` for a kind of tag this version does not know.
fn element(tag: &HeadTag) -> Option<HeadElement> {
    let text = |value: &str| Some(value.to_owned());
    Some(match tag {
        HeadTag::Title(title) => HeadElement::Title {
            text: title.clone(),
        },
        HeadTag::Meta { name, content } => HeadElement::Meta {
            name: text(name),
            property: None,
            content: content.clone(),
        },
        HeadTag::Property { property, content } => HeadElement::Meta {
            name: None,
            property: text(property),
            content: content.clone(),
        },
        HeadTag::Link { rel, href } => HeadElement::Link {
            rel: (*rel).to_owned(),
            hreflang: None,
            href: href.as_str().to_owned(),
        },
        HeadTag::Alternate { hreflang, href } => HeadElement::Link {
            rel: "alternate".to_owned(),
            hreflang: Some(hreflang.clone()),
            href: href.as_str().to_owned(),
        },
        _ => return None,
    })
}
