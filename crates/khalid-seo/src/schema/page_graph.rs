//! The whole graph of one page: the site nodes, the breadcrumbs and the
//! page node.

use khalid_core::{AbsoluteUrl, PageSchemaType};

use crate::schema::article::{article_nodes, person_node};
use crate::schema::blocks::block_nodes;
use crate::schema::glossary::{term_node, term_set_node};
use crate::schema::local_business::business_nodes;
use crate::schema::reviewer::reviewers;
use crate::schema::template::template_nodes;
use crate::schema::{image_node, node_id, publisher_id, site_home};
use crate::{
    AuthorProfile, PageKind, SchemaBlock, SchemaGraph, SchemaNode, SchemaValue, SeoError, SeoPage,
    SeoSite, breadcrumb_list, site_graph,
};

/// The schema.org type of a page of `kind`.
fn page_type(kind: PageKind) -> &'static str {
    match kind {
        PageKind::Tag
        | PageKind::Category
        | PageKind::Collection
        | PageKind::Date
        | PageKind::GlossaryIndex => "CollectionPage",
        PageKind::Author => "ProfilePage",
        PageKind::AiPage => "AboutPage",
        PageKind::Search => "SearchResultsPage",
        PageKind::Home
        | PageKind::Post
        | PageKind::Page
        | PageKind::GlossaryTerm
        | PageKind::NotFound => "WebPage",
    }
}

/// The `@id` of the page node, `<page>#webpage`, which an article names as
/// its `mainEntityOfPage`.
pub(crate) fn page_id(site: &SeoSite, page: &SeoPage) -> Result<AbsoluteUrl, SeoError> {
    node_id(&site.url_for(&page.language, &page.path)?, "webpage")
}

/// The `Person` an author page is about, at `<author page>#person`, the id
/// their posts point to: from the page's author profile, else its author
/// name, else its title.
fn profile_person(site: &SeoSite, page: &SeoPage) -> Result<SchemaNode, SeoError> {
    let url = site.url_for(&page.language, &page.path)?;
    let name = page.author.as_deref().unwrap_or(&page.title);
    let mut profile = page
        .author_profile
        .clone()
        .unwrap_or_else(|| AuthorProfile::new(name));
    profile.url = Some(url.clone());
    // An author of the site writes for its publisher.
    let publisher = SchemaValue::Reference(publisher_id(site)?);
    Ok(person_node(&profile, &url)?.with("worksFor", publisher))
}

fn page_node(site: &SeoSite, page: &SeoPage) -> Result<SchemaNode, SeoError> {
    let url = site.url_for(&page.language, &page.path)?;
    let name = if page.title.is_empty() {
        &site.name
    } else {
        &page.title
    };
    let kind = match page.page_type {
        Some(PageSchemaType::FaqPage) if has_faq_block(page) => page_type(page.kind),
        typed => typed.map_or(page_type(page.kind), PageSchemaType::as_str),
    };
    let mut node = SchemaNode::new(kind)
        .with_id(page_id(site, page)?)
        .with("name", SchemaValue::text(name))
        .with("url", SchemaValue::Url(url.clone()))
        .with(
            "isPartOf",
            SchemaValue::Reference(node_id(&site_home(site)?, "website")?),
        )
        .with("inLanguage", SchemaValue::text(page.language.as_str()))
        .with(
            "breadcrumb",
            SchemaValue::Reference(node_id(&url, "breadcrumb")?),
        );
    if !page.excerpt.is_empty() {
        node.set("description", SchemaValue::text(&page.excerpt));
    }
    if page.kind == PageKind::Author {
        let person = node_id(&url, "person")?;
        node.set("mainEntity", SchemaValue::Reference(person));
    }
    if let Some(image) = &page.share_image {
        node.set("primaryImageOfPage", image_node(image));
    }
    if let Some(published) = page.published {
        node.set("datePublished", SchemaValue::DateTime(published));
    }
    if let Some(modified) = page.modified {
        node.set("dateModified", SchemaValue::DateTime(modified));
    }
    if !page.speakable.is_empty() {
        let selectors = page
            .speakable
            .iter()
            .map(|s| SchemaValue::text(s))
            .collect();
        let speakable = SchemaNode::new("SpeakableSpecification")
            .with("cssSelector", SchemaValue::List(selectors));
        node.set("speakable", SchemaValue::Node(Box::new(speakable)));
    }
    Ok(node)
}

/// The whole schema graph of `page`: the publisher, the `WebSite`, the
/// `BreadcrumbList`, then the page node, typed by the page's kind:
/// `CollectionPage` for archives, `ProfilePage` for an author,
/// `SearchResultsPage` for search, `WebPage` for the rest. A post adds its
/// article node, typed by its [`ArticleType`](crate::ArticleType), and the
/// `Person` who wrote it. An author page adds the `Person` it is about, its
/// `mainEntity`. A glossary term adds its `DefinedTerm`, the
/// glossary's A to Z page its `DefinedTermSet`, and the home page one node
/// per location of a local business. Then come the nodes of the page's
/// blocks, such as an FAQ, the nodes of the site's
/// [`SchemaTemplate`](crate::SchemaTemplate)s that cover the page, and last
/// the [`CustomSchema`](crate::CustomSchema) of the site, then of the page.
/// A template or custom node of a type a block on the page writes is left
/// out, and a page typed `FAQPage` with an FAQ block is a `WebPage`, so
/// nothing a block describes is written twice.
///
/// # Errors
///
/// [`SeoError::Url`] when an address cannot be built.
pub fn page_graph(site: &SeoSite, page: &SeoPage) -> Result<SchemaGraph, SeoError> {
    let mut graph = site_graph(site)?;
    graph.push(breadcrumb_list(site, page)?);
    let (reviewed_by, reviewers) = reviewers(page, &site.url_for(&page.language, &page.path)?)?;
    let mut node = page_node(site, page)?;
    if let Some(reviewed_by) = reviewed_by {
        node.set("reviewedBy", reviewed_by);
    }
    graph.push(node);
    let mut extra = match page.kind {
        PageKind::Post => article_nodes(site, page, page_id(site, page)?)?,
        PageKind::GlossaryTerm => vec![term_node(site, page)?],
        PageKind::GlossaryIndex => vec![term_set_node(site, page)?],
        PageKind::Home => business_nodes(site)?,
        PageKind::Author => vec![profile_person(site, page)?],
        _ => Vec::new(),
    };
    // An author who also reviewed the post is already one Person node.
    for person in reviewers {
        if !extra.iter().any(|node| node.id() == person.id()) {
            extra.push(person);
        }
    }
    let blocks = block_nodes(site, page)?;
    // A thing a block describes is written once: the block's node, which
    // matches what the reader sees, wins over a template or a schema field
    // node of the same type (ADR "a block wins over the schema field").
    let from_blocks: Vec<String> = blocks.iter().map(|n| n.kind().to_owned()).collect();
    let unwritten = |node: &SchemaNode| !from_blocks.iter().any(|kind| kind == node.kind());
    let custom = site
        .custom_schema
        .nodes()
        .iter()
        .chain(page.custom_schema.nodes())
        .cloned();
    for node in extra {
        graph.push(node);
    }
    for node in blocks {
        // Two location blocks that name one place share its node, which
        // the home page already holds.
        let id = node.id();
        if !graph.nodes().iter().any(|n| id.is_some() && n.id() == id) {
            graph.push(node);
        }
    }
    let others = template_nodes(site, page)?.into_iter().chain(custom);
    for node in others.filter(unwritten) {
        graph.push(node);
    }
    Ok(graph)
}

/// Whether `page` has an FAQ block with questions, whose `FAQPage` node
/// then stands in for a page typed `FAQPage` in its front matter.
fn has_faq_block(page: &SeoPage) -> bool {
    page.blocks
        .iter()
        .any(|block| matches!(block, SchemaBlock::Faq(items) if !items.is_empty()))
}
