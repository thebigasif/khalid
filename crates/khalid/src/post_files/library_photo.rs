//! A lead image or author photo from the media library, such as
//! `/blog/media/library/3f2a9c1e0b57d0aa/lena.png`, the address the admin's
//! photo picker saves: published with the pages and drawn in sizes, just
//! like a file beside the content file.

use std::path::{Path, PathBuf};

use khalid_core::Config;
use khalid_media::{ContentKey, LIBRARY_ADDRESS};

use super::{self as post_files, PostFile};
use crate::build_pages::LanguagePages;

/// The library address `src` names, below the blog's mount, such as
/// `media/library/3f2a9c1e0b57d0aa/lena.png`. `None` for anything else.
/// The mount, a leading `/` or neither may come first.
fn named(config: &Config, src: &str) -> Option<(ContentKey, String)> {
    let rest = src
        .strip_prefix(config.mount())
        .or_else(|| src.strip_prefix('/'))
        .unwrap_or(src);
    let rest = rest.strip_prefix(LIBRARY_ADDRESS)?.strip_prefix('/')?;
    let (key, name) = rest.split_once('/')?;
    let plain = !name.is_empty() && !name.starts_with('.') && !name.contains(['/', '\\']);
    let key = ContentKey::parse(key)?;
    plain.then(|| (key, name.to_owned()))
}

/// Where readers fetch the library file `src` names, such as
/// `/blog/media/library/3f2a9c1e0b57d0aa/lena.png`, or `None` when `src`
/// names no library file.
pub(crate) fn address(config: &Config, src: &str) -> Option<String> {
    let (key, name) = named(config, src)?;
    Some(format!("{}{LIBRARY_ADDRESS}/{key}/{name}", config.mount()))
}

/// The library file `src` names, as a file to publish, when it is in the
/// `site` folder's `media/`. A file kept in a bucket, or gone, is `None`:
/// the page leaves the image out rather than reserve space for nothing.
pub(crate) fn file(site: &Path, config: &Config, src: &str) -> Option<PostFile> {
    let (key, name) = named(config, src)?;
    let below = format!("{LIBRARY_ADDRESS}/{key}/{name}");
    let file = PostFile {
        source: PathBuf::from(format!("media/{key}/{name}")),
        address: format!("{}{below}", config.mount()),
        file: below,
    };
    let found = post_files::on_disk(site, &file).is_ok_and(|path| path.is_file());
    if !found {
        tracing::warn!(image = %src, "the media library has no such file on disk, so the image is left out");
    }
    found.then_some(file)
}

/// The library files the lead images and author photos of `languages`
/// name, each once, in page order, for the `site` folder to publish.
pub(crate) fn files(site: &Path, config: &Config, languages: &[LanguagePages]) -> Vec<PostFile> {
    let mut files: Vec<PostFile> = Vec::new();
    let leads = languages
        .iter()
        .flat_map(|language| &language.pages)
        .filter_map(|page| page.lead.as_ref());
    for lead in leads {
        if let Some(file) = file(site, config, &lead.src)
            && !files.contains(&file)
        {
            files.push(file);
        }
    }
    files
}

#[cfg(test)]
mod tests {
    use khalid_core::Environment;

    use super::*;

    fn config() -> Config {
        let text = "name = \"Field notes\"\naddress = \"https://www.example.com\"\nmount = \"/blog/\"\ndefault_language = \"en\"\nlanguages = [\"en\"]\n";
        Config::from_toml(Path::new("site.toml"), text, &Environment::empty()).unwrap()
    }

    #[test]
    fn a_library_address_is_read_with_or_without_the_mount() {
        let config = config();
        let want = "/blog/media/library/3f2a9c1e0b57d0aa/lena.png";
        for src in [
            "/blog/media/library/3f2a9c1e0b57d0aa/lena.png",
            "/media/library/3f2a9c1e0b57d0aa/lena.png",
            "media/library/3f2a9c1e0b57d0aa/lena.png",
        ] {
            assert_eq!(address(&config, src).as_deref(), Some(want), "{src}");
        }
    }

    #[test]
    fn anything_else_is_not_a_library_file() {
        let config = config();
        for src in [
            "lena.png",
            "/blog/media/3f2a9c1e0b57d0aa/800.webp",
            "/blog/media/library/not-a-key/lena.png",
            "/blog/media/library/3f2a9c1e0b57d0aa/../site.toml",
            "/blog/media/library/3f2a9c1e0b57d0aa/",
            "https://cdn.example.com/media/library/3f2a9c1e0b57d0aa/lena.png",
        ] {
            assert_eq!(address(&config, src), None, "{src}");
        }
    }
}
