//! The images beside a folder post, offered in sizes made by `khalid-media`
//! at `<mount>media/<key>/<width>.<ext>`, each body image drawn as a
//! `picture` whose `img` has `width`, `height`, `srcset`, `sizes` and `alt`,
//! so the page never shifts as it loads (RULES.md BL-WEB-5, KH-WEB-5).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use khalid_core::{AbsoluteUrl, Config, UserMessage};
use khalid_media::{DerivedImage, Format, MediaCache, MediaError, derive};
use khalid_render::{ImageCandidate, ImageSet, Loading, PageBody, RenderImage};
use khalid_seo::ShareImage;
use khalid_server::MediaType;

use crate::build_pages::{LanguagePages, Page};
use crate::post_files::{self, IMAGE, PostFile};
use crate::post_photos::{self, Shown};

/// The folder under the mount, and in the output folder, that holds every size.
const FOLDER: &str = "media/";

/// How wide a body image is drawn: the text column, which `--measure`
/// keeps under 48rem, or the whole screen when that is narrower.
const SIZES: &str = "(min-width: 48rem) 48rem, 100vw";

/// The text column of [`SIZES`] in CSS pixels. A body image wider than
/// this is drawn smaller than it is, so it links to its widest size.
const COLUMN: u32 = 768;

/// One size of an image, and where it goes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MediaFile {
    /// Where `khalid-media` keeps it, in the site's `data/media/`.
    pub(crate) path: PathBuf,
    /// Where `khalid build` writes it, relative to the output folder.
    pub(crate) file: String,
    /// Where `khalid serve` answers it, such as `/blog/media/3f2a9c1e0b57d0aa/800.webp`.
    pub(crate) address: String,
}

/// The width and height of each SVG drawing beside a post, by its address.
type Drawings = BTreeMap<String, (u32, u32)>;

/// Every size an image is offered in, as a page draws it.
#[derive(Debug)]
struct Picture {
    webp: ImageSet,
    fallback: ImageSet,
    width: u32,
    height: u32,
}

/// Makes the sizes of each image among `files` that the `site` folder's
/// `data/media/` lacks, off the async runtime (RULES.md BL-RUN-7), draws
/// the body images of `languages` showing one, and returns every size once.
///
/// The lead images and author photos drawn are added to `shown`, which may
/// hold those of an earlier render, and every card and name on the pages
/// gets its image from it, so a saved post's page rendered alone shows the
/// same images as the full render.
pub(crate) fn attach(
    site: &Path,
    config: &Config,
    files: &[PostFile],
    languages: &mut [LanguagePages],
    shown: &mut Shown,
) -> Result<Vec<MediaFile>, UserMessage> {
    let cache = MediaCache::new(site);
    let mut pictures = BTreeMap::new();
    let mut drawings = BTreeMap::new();
    let mut sizes = BTreeMap::new();
    for file in files {
        let kind = MediaType::for_file(&file.address);
        if matches!(kind, MediaType::Svg) {
            if let Some(size) = drawing_size(&post_files::read(site, file)?) {
                drawings.insert(file.address.clone(), size);
            }
            continue;
        }
        if !matches!(kind, MediaType::Jpeg | MediaType::Png | MediaType::Webp) {
            continue;
        }
        let bytes = post_files::read(site, file)?;
        let image = derive(&bytes, &cache).map_err(|error| unmade(&file.source, &error))?;
        let Some((picture, made)) = picture(config, &image) else {
            continue;
        };
        pictures.insert(file.address.clone(), picture);
        sizes.extend(made.into_iter().map(|size| (size.address.clone(), size)));
    }
    let found = !pictures.is_empty() || !drawings.is_empty();
    for language in languages.iter_mut().filter(|_| found) {
        for page in &mut language.pages {
            post_files::rewrite(page, |html| drawn(html, &pictures));
            if let PageBody::Post(post) = &mut page.body {
                post.originals = originals(post.body.as_str(), &pictures);
            }
            if let Some(image) = lead(config, page, (&pictures, &drawings)) {
                shown.add(page, image);
            }
        }
    }
    post_photos::attach(languages, shown);
    Ok(sizes.into_values().collect())
}

/// Draws the lead image of a post, or the photo of an author, at the top
/// of `page`, when it is a file beside the page's file with its sizes among
/// `pictures`, or a drawing among `drawings`: fetched at once with high
/// priority, as the one large image at the top (BL-WEB-5). A picture is
/// also shared as the page's Open Graph and X image; a drawing is not, as
/// the networks do not show one. Returns what it drew. An image anywhere
/// else is left out until it has sizes, so no page shifts while it loads.
fn lead(
    config: &Config,
    page: &mut Page,
    (pictures, drawings): (&BTreeMap<String, Picture>, &Drawings),
) -> Option<RenderImage> {
    let image = page.lead.as_ref()?;
    let name = image.src.trim_start_matches("./");
    let address = format!("{}{name}", page.meta.path);
    let picture = pictures.get(&address);
    let (width, height, webp, fallback) = match (picture, drawings.get(&address)) {
        (Some(picture), _) => (
            picture.width,
            picture.height,
            Some(picture.webp.clone()),
            picture.fallback.clone(),
        ),
        (None, Some(&(width, height))) => {
            let drawing = ImageCandidate {
                url: address,
                width,
            };
            (width, height, None, ImageSet::new(drawing, Vec::new()))
        }
        (None, None) => {
            tracing::warn!(image = %image.src, page = %page.meta.path, "the lead image is not a file beside its page, so it is left out");
            return None;
        }
    };
    let drawn = RenderImage {
        alt: image.alt.clone(),
        caption: image.caption.clone(),
        width,
        height,
        sizes: SIZES.to_owned(),
        avif: None,
        webp,
        fallback,
        loading: Loading::Eager,
    };
    match &mut page.body {
        PageBody::Post(post) => post.image = Some(drawn.clone()),
        PageBody::Author(author) => author.image = Some(drawn.clone()),
        _ => return None,
    }
    let Some(picture) = picture else {
        return Some(drawn);
    };
    let origin = config.address().as_str().trim_end_matches('/');
    let widest = picture.fallback.widest();
    let shared = widest.and_then(|file| {
        let url = AbsoluteUrl::parse(&format!("{origin}{}", file.url)).ok()?;
        let scaled = u64::from(picture.height).checked_mul(u64::from(file.width))?;
        let height = u32::try_from(scaled.checked_div(u64::from(picture.width))?).ok()?;
        Some(ShareImage::new(url, file.width, height, &image.alt))
    });
    page.seo.share_image = shared;
    Some(drawn)
}

/// The picture `image` makes, and its files. `None` without the WebP and
/// fallback files `derive` always makes.
fn picture(config: &Config, image: &DerivedImage) -> Option<(Picture, Vec<MediaFile>)> {
    let mut webp = Vec::new();
    let mut fallback = Vec::new();
    let mut made = Vec::with_capacity(image.files.len());
    for derived in &image.files {
        let file = format!("{FOLDER}{}/{}", image.key, derived.variant.file_name());
        let address = format!("{}{file}", config.mount());
        let candidate = ImageCandidate {
            url: address.clone(),
            width: derived.variant.width,
        };
        if derived.variant.format == Format::Webp {
            webp.push(candidate);
        } else {
            fallback.push(candidate);
        }
        made.push(MediaFile {
            path: derived.path.clone(),
            file,
            address,
        });
    }
    let set = |mut candidates: Vec<ImageCandidate>| {
        let first = candidates.pop()?;
        Some(ImageSet::new(first, candidates))
    };
    let picture = Picture {
        webp: set(webp)?,
        fallback: set(fallback)?,
        width: image.width,
        height: image.height,
    };
    Some((picture, made))
}

/// The size an SVG drawing asks to be shown at, in CSS pixels: the `width`
/// and `height` of its `svg` element, or else those of its `viewBox`.
/// `None` when it gives neither, as the page could then shift.
fn drawing_size(bytes: &[u8]) -> Option<(u32, u32)> {
    let text = std::str::from_utf8(bytes).ok()?;
    let (_, rest) = text.split_once("<svg")?;
    let (tag, _) = rest.split_once('>')?;
    let whole = |value: &str| {
        let number: f64 = value.trim().trim_end_matches("px").parse().ok()?;
        let rounded = number.round();
        (1.0..=f64::from(u16::MAX))
            .contains(&rounded)
            .then(|| format!("{rounded}").parse().ok())
            .flatten()
    };
    let width = value_of(tag, "width").and_then(whole);
    let height = value_of(tag, "height").and_then(whole);
    if let (Some(width), Some(height)) = (width, height) {
        return Some((width, height));
    }
    let view: Vec<&str> = value_of(tag, "viewBox")?
        .split([' ', ','])
        .filter(|it| !it.is_empty())
        .collect();
    match view.as_slice() {
        [_, _, width, height] => Some((whole(width)?, whole(height)?)),
        _ => None,
    }
}

/// The value of the attribute `name` in the start tag `tag`, quoted with
/// either quote.
fn value_of<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    tag.match_indices(name).find_map(|(at, _)| {
        let (before, after) = tag.split_at(at);
        let spaced = before.chars().next_back().is_some_and(char::is_whitespace);
        let value = after
            .strip_prefix(name)?
            .trim_start()
            .strip_prefix('=')?
            .trim_start();
        let quote = value.chars().next().filter(|c| matches!(c, '"' | '\''))?;
        let (value, _) = value.strip_prefix(quote)?.split_once(quote)?;
        spaced.then_some(value)
    })
}

/// `html` with each image whose address is a key of `pictures` drawn as
/// its picture, keeping the image's `alt`.
fn drawn(html: &str, pictures: &BTreeMap<String, Picture>) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(at) = rest.find(IMAGE) {
        let (before, image) = rest.split_at(at);
        out.push_str(before);
        let after = image.strip_prefix(IMAGE).unwrap_or(image);
        let shown = after
            .split_once('"')
            .and_then(|(address, _)| pictures.get(address));
        if let (Some(picture), Some((tag, tail))) = (shown, image.split_once('>')) {
            draw(&mut out, picture, alt(tag));
            rest = tail;
        } else {
            out.push_str(IMAGE);
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// For each picture drawn in `html` that is wider than the text column and
/// not inside a link, the address its image shows, as `html` writes it,
/// and the address of its widest size, which click to enlarge links to.
fn originals(html: &str, pictures: &BTreeMap<String, Picture>) -> BTreeMap<String, String> {
    let wide = pictures.values().filter(|picture| picture.width > COLUMN);
    let widest = wide.filter_map(|picture| picture.fallback.widest());
    widest
        .map(|size| (attribute(&size.url), size.url.clone()))
        .filter(|(shown, _)| {
            let at = html.find(&format!("{IMAGE}{shown}\""));
            at.is_some_and(|at| !inside_link(html.split_at(at).0))
        })
        .collect()
}

/// Whether `before` ends inside an `a` element.
fn inside_link(before: &str) -> bool {
    let opened = before.rfind("<a ").or_else(|| before.rfind("<a>"));
    opened.is_some_and(|open| before.rfind("</a>").is_none_or(|close| close < open))
}

/// The `alt` of an `img` tag as written, already escaped.
fn alt(tag: &str) -> &str {
    tag.split_once(" alt=\"")
        .and_then(|(_, rest)| rest.split_once('"'))
        .map_or("", |(alt, _)| alt)
}

/// Writes `picture` with `alt`, in the shape of the templates' `picture` macro.
fn draw(out: &mut String, picture: &Picture, alt: &str) {
    let webp = attribute(&picture.webp.to_srcset());
    let fallback = &picture.fallback;
    let src = attribute(fallback.widest().map_or("", |widest| widest.url.as_str()));
    let srcset = attribute(&fallback.to_srcset());
    let (width, height) = (picture.width, picture.height);
    let _ = write!(
        out,
        "<picture>\n<source type=\"image/webp\" srcset=\"{webp}\" sizes=\"{SIZES}\">\n\
         <img src=\"{src}\" srcset=\"{srcset}\" sizes=\"{SIZES}\" width=\"{width}\" \
         height=\"{height}\" alt=\"{alt}\" loading=\"lazy\" decoding=\"async\">\n</picture>"
    );
}

/// `text` safe inside a double-quoted attribute. Addresses here are made
/// from the mount, a hex key and a width, and the mount may hold any text.
fn attribute(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// The message when an image beside a post cannot be made into sizes.
fn unmade(path: &Path, error: &MediaError) -> UserMessage {
    UserMessage::new(
        format!(
            "Khalid could not make the sizes of {}: {error}.",
            path.display()
        ),
        "Every image is offered in sizes that fit each screen, so a broken image stops the page.",
        "Open the image in an editor and save it again as JPEG, PNG or WebP, or remove it, then try again.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHART: &str = "/blog/slow/page-weight.png";

    fn set(mount: &str, extension: &str) -> ImageSet {
        let url = format!("{mount}media/3f2a/");
        let at = |width| ImageCandidate {
            url: format!("{url}{width}.{extension}"),
            width,
        };
        ImageSet::new(at(320), vec![at(800)])
    }

    fn pictures(mount: &str) -> BTreeMap<String, Picture> {
        let picture = Picture {
            webp: set(mount, "webp"),
            fallback: set(mount, "png"),
            width: 800,
            height: 450,
        };
        BTreeMap::from([(CHART.to_owned(), picture)])
    }

    #[test]
    fn a_shown_image_becomes_a_picture_with_its_five_attributes() {
        let html = format!(
            "<p>Before</p>\n<img src=\"{CHART}\" alt=\"A bar chart\" loading=\"lazy\" decoding=\"async\" />\n<p>After</p>"
        );
        assert_eq!(
            drawn(&html, &pictures("/blog/")),
            "<p>Before</p>\n<picture>\n\
             <source type=\"image/webp\" srcset=\"/blog/media/3f2a/320.webp 320w, /blog/media/3f2a/800.webp 800w\" sizes=\"(min-width: 48rem) 48rem, 100vw\">\n\
             <img src=\"/blog/media/3f2a/800.png\" srcset=\"/blog/media/3f2a/320.png 320w, /blog/media/3f2a/800.png 800w\" sizes=\"(min-width: 48rem) 48rem, 100vw\" width=\"800\" height=\"450\" alt=\"A bar chart\" loading=\"lazy\" decoding=\"async\">\n\
             </picture>\n<p>After</p>"
        );
    }

    #[test]
    fn any_other_image_is_left_as_it_is() {
        let html = "<img src=\"/blog/slow/other.png\" alt=\"\" /><img src=\"https://cdn.example.com/a.png\" alt=\"\" />";
        assert_eq!(drawn(html, &pictures("/blog/")), html);
    }

    #[test]
    fn a_picture_wider_than_the_column_links_to_its_widest_size() {
        let pictures = pictures("/blog/");
        let html = drawn(&format!("<img src=\"{CHART}\" alt=\"\" />"), &pictures);
        let widest = "/blog/media/3f2a/800.png".to_owned();
        assert_eq!(
            originals(&html, &pictures),
            BTreeMap::from([(widest.clone(), widest)])
        );
    }

    #[test]
    fn a_picture_inside_a_link_or_as_narrow_as_the_column_links_nowhere() {
        let mut pictures = pictures("/blog/");
        let image = format!("<img src=\"{CHART}\" alt=\"\" />");
        let linked = drawn(&format!("<a href=\"/blog/\">{image}</a>"), &pictures);
        assert_eq!(originals(&linked, &pictures), BTreeMap::new());
        for picture in pictures.values_mut() {
            picture.width = COLUMN;
        }
        assert_eq!(
            originals(&drawn(&image, &pictures), &pictures),
            BTreeMap::new()
        );
    }

    #[test]
    fn a_drawing_gives_its_size_from_its_width_and_height_or_its_view_box() {
        let sized = |svg: &str| drawing_size(svg.as_bytes());
        let both = "<?xml version=\"1.0\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 10 10\"\n  width=\"640px\" height='300'>";
        assert_eq!(sized(both), Some((640, 300)));
        assert_eq!(
            sized("<svg viewBox=\"0,0,800.4,450\"><rect/></svg>"),
            Some((800, 450))
        );
        assert_eq!(
            sized("<svg stroke-width=\"2\" viewBox=\"0 0 64 30\">"),
            Some((64, 30))
        );
        for none in [
            "<svg width=\"100%\" height=\"100%\">",
            "<svg>",
            "<svg viewBox=\"0 0 0 30\">",
            "not a drawing",
        ] {
            assert_eq!(sized(none), None, "{none}");
        }
    }

    #[test]
    fn a_mount_with_markup_is_escaped_in_every_address() {
        let html = format!("<img src=\"{CHART}\" alt=\"\" />");
        let drawn = drawn(&html, &pictures("/a\"b&c/"));
        assert!(
            !drawn.contains("a\"b") && drawn.contains("/a&quot;b&amp;c/"),
            "{drawn}"
        );
    }
}
