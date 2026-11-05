//! Which sizes and formats an image is offered in.

/// The widths a responsive image is offered in, in pixels. They cover a
/// 320 px phone at 1x up to a wide screen at 2x without large gaps, so the
/// browser never fetches much more than it draws.
const BREAKPOINTS: [u32; 8] = [320, 480, 640, 800, 1024, 1280, 1600, 2048];

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

/// The widest file the fallback format may have: a browser that reads
/// neither AVIF nor WebP is an old one, so it gets one file of this width or
/// less instead of every width, which would double the files each image
/// keeps on disk.
const FALLBACK_MAX: u32 = 1280;

/// The widths to make in the fallback format for an original `width`
/// pixels wide: one, the widest of [`widths`] at [`FALLBACK_MAX`] or below,
/// or the narrowest when every width is wider.
pub fn fallback_widths(width: u32) -> Vec<u32> {
    let all = widths(width);
    all.iter()
        .rev()
        .find(|&&candidate| candidate <= FALLBACK_MAX)
        .or_else(|| all.first())
        .copied()
        .into_iter()
        .collect()
}
