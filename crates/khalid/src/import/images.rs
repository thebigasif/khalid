//! `khalid import --download-images`: copies the images each imported post
//! or page shows, and its feature image, from the old site into the
//! entry's own folder, through the one outbound client (RULES.md
//! BL-RUN-8), so a blog keeps its pictures when the old site goes away.
//! The entry becomes `<name>/index.md`, its links name the files beside
//! it, and its feature image becomes its lead image.

use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use khalid_core::{AbsoluteUrl, UserMessage};
use khalid_import::PlannedFile;
use khalid_outbound::Outbound;

use crate::cli::Cli;
use crate::exit::Exit;
use crate::{atomic, outbound};

/// What the import did with the images its entries show.
#[derive(Debug, Default)]
pub(crate) struct Images {
    /// How many different images came in.
    pub(crate) fetched: usize,
    /// Each image left at its old address, with why, in the order found.
    pub(crate) kept: Vec<(String, String)>,
    /// The files of the entries whose feature image became their lead
    /// image, as the plan named them.
    pub(crate) leads: Vec<PathBuf>,
    /// The images to write, from the site folder, with their bytes.
    files: Vec<(PathBuf, Vec<u8>)>,
}

/// The images one entry shows: its file in the plan, its feature image,
/// and the old addresses in its body, each once.
struct Shown {
    file: usize,
    lead: Option<String>,
    addresses: Vec<String>,
}

/// Each entry in `files` that shows an image, with `leads`, the feature
/// images by the entry's file. Drafts are one file each, so they are
/// listed in `kept` instead.
fn shown(files: &[PlannedFile], leads: &BTreeMap<PathBuf, String>, kept: &mut Kept) -> Vec<Shown> {
    let mut shown = Vec::new();
    for (file, planned) in files.iter().enumerate() {
        let lead = leads.get(&planned.path).cloned();
        let mut addresses = addresses(&planned.text);
        if let Some(lead) = &lead {
            addresses.retain(|address| address != lead);
            addresses.insert(0, lead.clone());
        }
        if addresses.is_empty() {
            continue;
        }
        if is_entry(&planned.path) {
            shown.push(Shown {
                file,
                lead,
                addresses,
            });
        } else {
            for address in addresses {
                let why = "it is in a draft, which is one file, so publish it and import again";
                kept.push((address, why.to_owned()));
            }
        }
    }
    shown
}

type Kept = Vec<(String, String)>;

/// Whether `path` is a post or a page an import writes as one file,
/// `content/<language>/posts/<name>.md` or the same under `pages/`.
fn is_entry(path: &Path) -> bool {
    let parts: Vec<&OsStr> = path.iter().collect();
    matches!(
        parts.as_slice(),
        [content, _, collection, name]
            if *content == "content"
                && (*collection == "posts" || *collection == "pages")
                && Path::new(name).extension() == Some(OsStr::new("md"))
    )
}

/// How many different images an import of `files` would download.
pub(crate) fn count(files: &[PlannedFile], leads: &BTreeMap<PathBuf, String>) -> usize {
    let mut drafts = Kept::new();
    let mut all: Vec<String> = shown(files, leads, &mut drafts)
        .into_iter()
        .flat_map(|entry| entry.addresses)
        .collect();
    all.sort();
    all.dedup();
    all.len()
}

/// Downloads every image the entries in `files` show, once each, moves
/// each entry that got one into its own folder, and points its links at
/// the files. A failed image keeps its old address and is listed with why.
pub(crate) fn bring_in(
    files: &mut [PlannedFile],
    leads: &BTreeMap<PathBuf, String>,
    site: &Path,
    cli: &Cli,
) -> Result<Images, (UserMessage, Exit)> {
    let mut images = Images::default();
    let shown = shown(files, leads, &mut images.kept);
    let mut fetched: BTreeMap<&str, Result<(Vec<u8>, &str), String>> = BTreeMap::new();
    let no_client = |error: &dyn std::fmt::Display| {
        let message = UserMessage::new(
            format!("Khalid could not get ready to download the images: {error}."),
            "Nothing was imported, so the site is as it was.",
            "Run the command again, or leave out --download-images to keep the old links.",
        );
        (message, Exit::Internal)
    };
    let client = outbound::client(cli).map_err(|error| no_client(&error))?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| no_client(&error))?;
    for address in shown.iter().flat_map(|entry| &entry.addresses) {
        if !fetched.contains_key(address.as_str()) {
            let got = runtime.block_on(fetch(&client, address));
            images.fetched += usize::from(got.is_ok());
            if let Err(why) = &got {
                images.kept.push((address.clone(), why.clone()));
            }
            fetched.insert(address, got);
        }
    }
    for entry in &shown {
        let Some(planned) = files.get_mut(entry.file) else {
            continue;
        };
        let mut old = site.to_path_buf();
        old.push(&planned.path);
        let got: Vec<(&String, &(Vec<u8>, &str))> = entry
            .addresses
            .iter()
            .filter_map(|address| Some((address, fetched.get(address.as_str())?.as_ref().ok()?)))
            .collect();
        // A file left by an earlier import stays where the write finds it.
        if got.is_empty() || old.exists() {
            continue;
        }
        let folder = planned.path.with_extension("");
        let mut names = Vec::new();
        for (address, (bytes, kind)) in got {
            let name = unique(&file_name(address, kind), &names);
            planned.text = relinked(&planned.text, address, &name);
            if entry.lead.as_ref() == Some(address) && !planned.text.contains("\nimage:") {
                planned.text = with_lead(&planned.text, &name);
                images.leads.push(planned.path.clone());
            }
            let mut path = folder.clone();
            path.push(&name);
            images.files.push((path, bytes.clone()));
            names.push(name);
        }
        planned.path = folder;
        planned.path.push("index.md");
    }
    Ok(images)
}

/// Writes the downloaded images into the site at `site`, never over a file,
/// and returns their paths from the site folder.
pub(crate) fn write(site: &Path, images: &Images) -> Result<Vec<PathBuf>, (UserMessage, Exit)> {
    let mut written = Vec::new();
    for (path, bytes) in &images.files {
        let mut folder = site.to_path_buf();
        folder.push(path.parent().unwrap_or(Path::new("")));
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        atomic::create_bytes(&folder, &name, bytes).map_err(|error| {
            let message = UserMessage::new(
                format!(
                    "The posts came in, but the image {} was not written: {error}.",
                    path.display()
                ),
                "The post that shows it has a broken image until it is there.",
                "Check that the site folder can be written to, then put the image there by hand.",
            );
            (message, Exit::Failure)
        })?;
        written.push(path.clone());
    }
    Ok(written)
}

/// The image at `address`, with the file type its first bytes show, or
/// why it was not taken, in a few words.
async fn fetch(client: &Outbound, address: &str) -> Result<(Vec<u8>, &'static str), String> {
    let url = AbsoluteUrl::parse(address).map_err(|error| error.to_string())?;
    let fetched = client.get(&url).await.map_err(|error| error.to_string())?;
    if fetched.status() != 200 {
        return Err(format!("it answered {}", fetched.status()));
    }
    let bytes = fetched.body();
    let kind = picture_kind(bytes).ok_or("it is not a JPEG, PNG, GIF, WebP or AVIF image")?;
    Ok((bytes.to_vec(), kind))
}

/// The file type of a picture by its first bytes. SVG is left out, as a
/// drawing can carry scripts.
fn picture_kind(bytes: &[u8]) -> Option<&'static str> {
    let kinds: [(&[u8], usize, &str); 6] = [
        (&[0xFF, 0xD8, 0xFF], 0, "jpg"),
        (b"\x89PNG\r\n\x1a\n", 0, "png"),
        (b"GIF8", 0, "gif"),
        (b"WEBP", 8, "webp"),
        (b"ftypavif", 4, "avif"),
        (b"ftypavis", 4, "avif"),
    ];
    kinds.into_iter().find_map(|(mark, at, kind)| {
        let end = at.checked_add(mark.len())?;
        (bytes.get(at..end) == Some(mark)).then_some(kind)
    })
}

/// The old addresses of the images in `markdown`: each `![alt](address)`
/// and each figure's `src`, on the web, each once, in the order found.
fn addresses(markdown: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    for line in markdown.lines() {
        let mut candidates = Vec::new();
        for (_, after) in line
            .split("![")
            .skip(1)
            .filter_map(|part| part.split_once("]("))
        {
            candidates.extend(after.split([')', ' ']).next());
        }
        if line
            .trim_start()
            .trim_start_matches(':')
            .starts_with("figure{")
        {
            candidates.extend(
                line.split("src=\"")
                    .nth(1)
                    .and_then(|rest| rest.split('"').next()),
            );
        }
        for address in candidates {
            let web = address.starts_with("https://") || address.starts_with("http://");
            if web && !found.iter().any(|seen| seen == address) {
                found.push(address.to_owned());
            }
        }
    }
    found
}

/// A file name for the image at `address` of type `kind`: the last part of
/// its path in small letters, digits and hyphens.
fn file_name(address: &str, kind: &str) -> String {
    let last = address.split(['?', '#']).next().unwrap_or_default();
    let last = last.rsplit('/').next().unwrap_or_default();
    let stem = last.rsplit_once('.').map_or(last, |(stem, _)| stem);
    let mut name = String::new();
    for character in stem.chars().map(|c| c.to_ascii_lowercase()) {
        if character.is_ascii_alphanumeric() {
            name.push(character);
        } else if !name.is_empty() && !name.ends_with('-') {
            name.push('-');
        }
    }
    let name = name.trim_end_matches('-');
    let name = if name.is_empty() || name == "index" {
        "image"
    } else {
        name
    };
    format!("{name}.{kind}")
}

/// `name`, or with a number before its type when `taken` holds it.
fn unique(name: &str, taken: &[String]) -> String {
    let (stem, kind) = name.rsplit_once('.').unwrap_or((name, ""));
    let mut candidate = name.to_owned();
    let mut number = 1;
    while taken.contains(&candidate) {
        number += 1;
        candidate = format!("{stem}-{number}.{kind}");
    }
    candidate
}

/// `markdown` with each image link and figure that names `address` naming
/// the file `name` instead.
fn relinked(markdown: &str, address: &str, name: &str) -> String {
    markdown
        .replace(&format!("]({address})"), &format!("]({name})"))
        .replace(&format!("]({address} "), &format!("]({name} "))
        .replace(&format!("src=\"{address}\""), &format!("src=\"{name}\""))
}

/// `file` with `image` set to the file `name`, described by the entry's
/// title, at the end of its front matter.
fn with_lead(file: &str, name: &str) -> String {
    let Some((front, body)) = file
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
    else {
        return file.to_owned();
    };
    let Some(title) = front.lines().find_map(|line| line.strip_prefix("title: ")) else {
        return file.to_owned();
    };
    format!("---\n{front}\nimage:\n  src: \"{name}\"\n  alt: {title}\n---\n{body}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_links_and_figures_are_found_once_each() {
        let markdown = "Text ![A jar](https://old.example.com/a.jpg \"Jar\") and [a link](https://old.example.com/b.jpg).\n\
            :::figure{src=\"https://old.example.com/c.png\" alt=\"C\"}\nCaption\n:::\n\
            ![Again](https://old.example.com/a.jpg) ![Here](d.jpg)\n";
        assert_eq!(
            addresses(markdown),
            [
                "https://old.example.com/a.jpg",
                "https://old.example.com/c.png"
            ]
        );
    }

    #[test]
    fn a_file_name_comes_from_the_path_and_the_bytes() {
        assert_eq!(
            file_name("https://x.example/content/images/Rye_Loaf.JPG?w=2", "png"),
            "rye-loaf.png"
        );
        assert_eq!(file_name("https://x.example/", "jpg"), "image.jpg");
        assert_eq!(
            unique("a.png", &["a.png".to_owned(), "a-2.png".to_owned()]),
            "a-3.png"
        );
    }

    #[test]
    fn only_raster_pictures_are_taken() {
        assert_eq!(picture_kind(&[0xFF, 0xD8, 0xFF, 0xE0]), Some("jpg"));
        assert_eq!(picture_kind(b"RIFF\0\0\0\0WEBPVP8 "), Some("webp"));
        assert_eq!(
            picture_kind(b"<svg xmlns=\"http://www.w3.org/2000/svg\"/>"),
            None
        );
        assert_eq!(picture_kind(b"<html>"), None);
    }

    #[test]
    fn the_lead_goes_at_the_end_of_the_front_matter() {
        let file = "---\ntitle: \"Rye\"\nslug: \"rye\"\n---\nBody\n";
        assert_eq!(
            with_lead(file, "rye.png"),
            "---\ntitle: \"Rye\"\nslug: \"rye\"\nimage:\n  src: \"rye.png\"\n  alt: \"Rye\"\n---\nBody\n"
        );
    }
}
