//! Imported posts the publish gate blocks. Readers do not see such a post,
//! so a redirect to it would end on the not-found page. Its old addresses
//! wait in its `aliases` instead, which go live with the post the moment it
//! passes, at the address it then has (KH-API-7, KH-SEO-8, ADR "a blocked
//! imported post holds its old addresses").

use std::path::{Path, PathBuf};

use khalid_content::{ContentTree, Document, PostFront, read_content};
use khalid_core::{Config, SystemClock, UserMessage};
use khalid_routes::{PostAddress, Routes};
use khalid_seo::{Matching, RedirectEntry, write_redirects_csv};

use crate::build_pages::{named, path_of, routed};
use crate::post_text::{PostText, quoted};
use crate::{atomic, build_gate, content_message};

/// The file that keeps the old addresses outside the blog's folder that
/// lead to a held post, from the site folder.
pub(crate) const HELD_CSV: &str = "import/held-redirects.csv";

/// What the import held back.
#[derive(Debug, Default)]
pub(crate) struct Held {
    /// Each imported post the gate blocks, from the site folder.
    pub(crate) posts: Vec<PathBuf>,
    /// How many old addresses wait in those posts' aliases.
    pub(crate) aliased: usize,
    /// How many old addresses wait in [`HELD_CSV`], because they lie
    /// outside the blog's folder, where an alias cannot be.
    pub(crate) outside: usize,
}

/// Judges the site in `site` as `khalid build` does, once the import wrote
/// `written`, and holds each blocked imported post's redirects: every
/// exact row of `entries` that leads to such a post is taken out, and its
/// old address goes into the post's aliases, or into [`HELD_CSV`] when it
/// lies outside the blog's folder.
///
/// # Errors
///
/// What, why and fix when the content, the lint settings or a post's file
/// cannot be read or written.
pub(crate) fn hold(
    site: &Path,
    (config, routes): (&Config, &Routes),
    written: &[PathBuf],
    entries: &mut Vec<RedirectEntry>,
) -> Result<Held, UserMessage> {
    let content = ContentTree::walk(site)
        .and_then(|tree| read_content(&tree))
        .map_err(|error| content_message::of(site, &error))?;
    let refused = build_gate::screen(site, (config, routes, &content), &SystemClock)?;
    let origin = config.address().as_str().trim_end_matches('/');
    let mut held = Held::default();
    let mut outside = Vec::new();
    let blocked = content.posts.iter().filter(|doc| {
        written.contains(&doc.path) && refused.iter().any(|post| post.path == doc.path)
    });
    for doc in blocked {
        let address = address_of(config, routes, doc)?;
        let (leads, rest): (Vec<_>, Vec<_>) = entries
            .drain(..)
            .partition(|entry| leads_to(origin, entry, &address));
        *entries = rest;
        let (inside, beyond): (Vec<_>, Vec<_>) = leads
            .into_iter()
            .partition(|entry| entry.columns().from.starts_with(config.mount()));
        let mut aliases = doc.front.aliases.clone();
        for entry in &inside {
            let from = entry.columns().from;
            if !aliases.contains(&from) {
                aliases.push(from);
            }
        }
        if !inside.is_empty() {
            alias(site, &doc.path, &aliases)?;
        }
        held.aliased += inside.len();
        outside.extend(beyond);
        held.posts.push(doc.path.clone());
    }
    held.outside = outside.len();
    if !outside.is_empty() {
        let mut file = site.to_path_buf();
        file.push(HELD_CSV);
        atomic::replace(&file, &write_redirects_csv(&outside))
            .map_err(|error| not_written(&file, &error))?;
    }
    Ok(held)
}

/// The path from the site root the post `doc` would have once published,
/// such as `/keep-a-starter-alive/`.
fn address_of(
    config: &Config,
    routes: &Routes,
    doc: &Document<PostFront>,
) -> Result<String, UserMessage> {
    let front = &doc.front;
    let (_, slug) = named(config, doc, front.slug.as_ref(), front.title.as_deref())?;
    let entry = PostAddress::new(&doc.collection, &doc.locale, &slug, front.date);
    let rest = routed(config, routes, &doc.locale, &entry, &doc.path)?;
    Ok(path_of(config, &doc.locale, &rest))
}

/// Whether `entry` sends one path to `address` on the site at `origin`.
fn leads_to(origin: &str, entry: &RedirectEntry, address: &str) -> bool {
    let columns = entry.columns();
    let to = columns.to.as_deref().unwrap_or_default();
    columns.matching == Matching::Exact && to.strip_prefix(origin).unwrap_or(to) == address
}

/// Writes `aliases` into the front matter of the post at `path`, from the
/// site folder.
fn alias(site: &Path, path: &Path, aliases: &[String]) -> Result<(), UserMessage> {
    let mut file = site.to_path_buf();
    file.push(path);
    let text = std::fs::read_to_string(&file).map_err(|error| not_written(&file, &error))?;
    let unread = std::io::Error::other("its header cannot be read");
    let mut text = PostText::parse(&text).ok_or_else(|| not_written(&file, &unread))?;
    let paths: Vec<String> = aliases.iter().map(|alias| quoted(alias)).collect();
    text.set("aliases", &format!("[{}]", paths.join(", ")));
    atomic::replace(&file, &text.to_text()).map_err(|error| not_written(&file, &error))
}

fn not_written(file: &Path, error: &std::io::Error) -> UserMessage {
    UserMessage::new(
        format!("Khalid could not write {}: {error}.", file.display()),
        "The posts came in, but no redirect was stored.",
        "Check that the site folder can be written to, then run `khalid lint` and store the redirects with `khalid redirects import`.",
    )
}
