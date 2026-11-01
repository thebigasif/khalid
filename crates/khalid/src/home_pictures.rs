//! The lead images on the home page's cards when `khalid serve` answers it
//! before the full render: each drawn from the file beside its post, as
//! the full render draws it on the post's other cards, so the first home
//! page stays the page the full render makes.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use khalid_content::{Document, PostFront};
use khalid_core::{Config, UserMessage};
use khalid_render::PageBody;

use crate::build_pages::LanguagePages;
use crate::post_files::PostFile;
use crate::{post_images, post_photos};

/// The file a post's card shows as its picture, with its text alternative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Lead {
    file: PostFile,
    alt: String,
}

/// The lead images of posts, by the address of each post's page.
pub(crate) type Leads = BTreeMap<String, Lead>;

/// The lead image of the post `doc`, whose page is at `href`. `None` when
/// it has none, or is a lone file with no folder to hold one.
pub(crate) fn of(doc: &Document<PostFront>, href: &str) -> Option<Lead> {
    let image = doc.front.image.as_ref()?;
    let folder = doc.path.parent()?;
    let is_folder = doc.path.file_name().is_some_and(|name| name == "index.md");
    let name = image.src.trim_start_matches("./");
    let relative = !name.contains("://") && !name.starts_with('/') && !name.contains("..");
    if !is_folder || !relative {
        return None;
    }
    let mut source = PathBuf::from(folder);
    source.push(name);
    Some(Lead {
        file: PostFile {
            source,
            file: String::new(),
            address: format!("{href}{name}"),
        },
        alt: image.alt.clone(),
    })
}

/// Puts each card's lead image among `leads` on the home pages of
/// `languages`, drawn from the files in the `site` folder.
///
/// # Errors
///
/// The message for a lead image that cannot be read or made into sizes.
pub(crate) fn draw(
    site: &Path,
    config: &Config,
    leads: &Leads,
    languages: &mut [LanguagePages],
) -> Result<(), UserMessage> {
    let mut drawn = BTreeMap::new();
    for page in languages
        .iter_mut()
        .flat_map(|language| &mut language.pages)
    {
        let PageBody::Home(home) = &mut page.body else {
            continue;
        };
        for card in post_photos::home_cards(home) {
            let Some(lead) = leads.get(&card.href) else {
                continue;
            };
            if !drawn.contains_key(&card.href) {
                let image = post_images::card_image(site, config, &lead.file, &lead.alt)?;
                drawn.insert(card.href.clone(), image);
            }
            card.image = drawn.get(&card.href).cloned().flatten();
        }
    }
    Ok(())
}
