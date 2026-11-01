//! The content index a rebuild writes, read from the site's content tree.

use std::path::{Component, Path};

use khalid_content::{ContentHash, ContentTree, SiteContent, read_content};
use khalid_core::{Config, UserMessage};
use khalid_db::{ContentIndex, IndexedAuthor, IndexedPost};

use crate::build_pages::named;
use crate::{categories, content_message};

/// The collection whose entries are posts. Other collections get their own
/// schemas and join the index with them.
pub(crate) const POSTS: &str = "posts";

/// Every post and author of every language in the `site` folder, and the
/// categories in its `categories.toml`, ready for the database.
pub(crate) fn of(site: &Path, config: &Config) -> Result<ContentIndex, UserMessage> {
    let content = |error| content_message::of(site, &error);
    let tree = ContentTree::walk(site).map_err(content)?;
    let read = read_content(&tree).map_err(content)?;
    of_content(site, config, &read)
}

/// The same index as [`of`], from the content already `read` from the
/// `site` folder, so no post is read twice. A post or an author with no
/// `slug:` takes the one `khalid build` makes from its title or name, so
/// the server and the built site give it the same address.
pub(crate) fn of_content(
    site: &Path,
    config: &Config,
    read: &SiteContent,
) -> Result<ContentIndex, UserMessage> {
    let mut index = ContentIndex::default();
    for doc in read.posts.iter().filter(|doc| doc.collection == POSTS) {
        let front = &doc.front;
        let at = format!("{}/{}", site.display(), doc.path.display());
        let path = slashed(&doc.path).ok_or_else(|| unreadable_name(&at))?;
        let (_, slug) = named(config, doc, front.slug.as_ref(), front.title.as_deref())?;
        let mut post = IndexedPost::new(&path, doc.locale.clone(), POSTS, slug, bytes(doc.hash));
        post.title.clone_from(&front.title);
        post.description.clone_from(&front.description);
        post.date = front.date;
        post.author.clone_from(&front.author);
        post.category.clone_from(&front.category);
        post.translation.clone_from(&front.translation);
        index.posts.push(post);
    }
    for doc in &read.authors {
        let at = format!("{}/{}", site.display(), doc.path.display());
        let path = slashed(&doc.path).ok_or_else(|| unreadable_name(&at))?;
        let front = &doc.front;
        let (name, slug) = named(config, doc, front.slug.as_ref(), front.name.as_deref())?;
        let author = IndexedAuthor::new(&path, doc.locale.clone(), slug, &name, bytes(doc.hash));
        index.authors.push(author);
    }
    index.categories = categories::of(site)?;
    Ok(index)
}

/// The entry's path with `/` between folders, the form the index stores on
/// every system, or `None` when a name is not valid UTF-8.
pub(crate) fn slashed(path: &Path) -> Option<String> {
    let parts: Option<Vec<&str>> = path
        .components()
        .map(|part| match part {
            Component::Normal(name) => name.to_str(),
            _ => None,
        })
        .collect();
    parts.map(|parts| parts.join("/"))
}

/// The message for a file name that is not valid UTF-8.
fn unreadable_name(at: &str) -> UserMessage {
    UserMessage::new(
        format!("The name of {at} has characters Khalid cannot read."),
        "Every file under content/ needs a name that is valid text to be indexed.",
        "Rename the file using letters, digits and hyphens.",
    )
}

/// A content hash as bytes. `ContentHash` shows itself as 64 lowercase hex
/// digits and has no byte accessor yet.
fn bytes(hash: ContentHash) -> [u8; 32] {
    let hex = hash.to_string();
    let mut bytes = [0; 32];
    for (byte, pair) in bytes.iter_mut().zip(hex.as_bytes().chunks(2)) {
        let digits = std::str::from_utf8(pair).unwrap_or_default();
        *byte = u8::from_str_radix(digits, 16).unwrap_or_default();
    }
    bytes
}
