//! Which sizes and formats an image is offered in.

/// The widths a responsive image is offered in, in pixels. They cover a
/// 320 px phone at 1x up to a wide screen at 2x without large gaps, so the
/// browser never fetches much more than it draws. 1152 is for phones at
/// three device pixels per CSS pixel: a card 344 to 384 px wide needs 1032
/// to 1152 pixels, and without it the browser fetched the 1280 px file.
const BREAKPOINTS: [u32; 9] = [320, 480, 640, 800, 1024, 1152, 1280, 1600, 2048];

/// A file format the site serves images in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Format {
    /// AVIF: about half the size of JPEG, read by current browsers, and slow
    /// to make, so a site asks for it (ADR "AVIF images are made with ravif").
    Avif,
    /// WebP: smaller than JPEG and PNG, read by every current browser.
    Webp,
    /// JPEG, for photos, when the original was a JPEG.
    Jpeg,
    /// PNG, for graphics, when the original was a PNG.
    Png,
}

impl Format {
    /// The file extension, such as `webp`.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Avif => "avif",
            Self::Webp => "webp",
            Self::Jpeg => "jpg",
            Self::Png => "png",
        }
    }

    /// The media type, such as `image/webp`.
    pub fn media_type(self) -> &'static str {
        match self {
            Self::Avif => "image/avif",
            Self::Webp => "image/webp",
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
        }
    }
}

/// One derived file: a format at a width, with the height that keeps the
/// original's shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Variant {
    /// The file format.
    pub format: Format,
    /// The width in pixels.
    pub width: u32,
    /// The height in pixels.
    pub height: u32,
}

impl Variant {
    /// The file name inside the original's folder, such as `800.webp`.
    pub fn file_name(&self) -> String {
        format!("{}.{}", self.width, self.format.extension())
    }
}

/// The widths to make for an original `width` pixels wide: every breakpoint
/// below it, then the original width itself, capped at the widest
/// breakpoint. An image is never scaled up, and a small image is offered at
/// its own size only.
pub fn widths(width: u32) -> Vec<u32> {
    let mut widths: Vec<u32> = BREAKPOINTS
        .into_iter()
        .filter(|&breakpoint| breakpoint < width)
        .collect();
    if width > 0 {
        widths.push(width.min(BREAKPOINTS.last().copied().unwrap_or(width)));
    }
    widths.dedup();
    widths
}

/// The widest file the fallback format may have. A browser that reads
/// neither AVIF nor WebP is an old one, so it gets one file of this width or
/// less instead of every width, which would double the files each image
/// keeps on disk. The same file is the image a post shares on social
/// networks, which ask for at least 1200 px.
const FALLBACK_MAX: u32 = 1280;

/// The widths to make in the fallback format for an original `width`
/// pixels wide: one, the widest of [`widths`] at 1280 px or below, or the
/// narrowest when every width is wider.
pub fn fallback_widths(width: u32) -> Vec<u32> {
    one_width_up_to(width, FALLBACK_MAX)
}

/// The widest fallback file when a site offers AVIF in place of
/// WebP (`avif = "only"`).
/// Every current browser then takes AVIF, so the fallback serves only the
/// few that read neither format. One middle width keeps the picture sharp
/// enough on their screens at a third of the bytes of the 1280 px file
/// times two formats.
const FALLBACK_BESIDE_AVIF_MAX: u32 = 1024;

/// The widths to make in the fallback format beside AVIF alone, without
/// WebP, for an original `width` pixels wide: one, the widest of [`widths`]
/// at 1024 px or below, or the narrowest when every width is wider.
pub fn fallback_widths_beside_avif(width: u32) -> Vec<u32> {
    one_width_up_to(width, FALLBACK_BESIDE_AVIF_MAX)
}

/// The widest of [`widths`] for `width` that is `max` or narrower, or the
/// narrowest when every width is wider, as a list of one or none.
fn one_width_up_to(width: u32, max: u32) -> Vec<u32> {
    let all = widths(width);
    all.iter()
        .rev()
        .find(|&&candidate| candidate <= max)
        .or_else(|| all.first())
        .copied()
        .into_iter()
        .collect()
}
