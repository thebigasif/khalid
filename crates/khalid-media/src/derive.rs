//! Making every size and format of an image, in parallel, once.

use std::path::PathBuf;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{ContentKey, Format, MediaCache, MediaError, Original, Variant, widths};

/// Everything made from one original: its key, its size and its files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedImage {
    /// The key every file is named by.
    pub key: ContentKey,
    /// The original's width in pixels.
    pub width: u32,
    /// The original's height in pixels.
    pub height: u32,
    /// Every file, AVIF first when asked for, then WebP, then the fallback,
    /// each from narrowest to widest.
    pub files: Vec<DerivedFile>,
    /// How many files this call encoded. Zero when the cache had them all.
    pub encoded: usize,
}

/// One file made from an original.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedFile {
    /// Its format and size.
    pub variant: Variant,
    /// Where it is kept.
    pub path: PathBuf,
}

/// Makes every variant of `original` that `cache` does not already have,
/// encoding them in parallel on rayon's pool. Files already in the cache
/// are kept as they are: the key is a hash of the original's bytes, so a
/// file with the same name is the same file (survey lesson 5).
///
/// This blocks while it works. From async code, call [`derive_in_background`].
///
/// # Errors
///
/// Returns the first [`MediaError`] from decoding, encoding or writing.
pub fn derive(original: &[u8], cache: &MediaCache) -> Result<DerivedImage, MediaError> {
    derive_with(original, cache, Avif::Skip)
}

/// Whether to make AVIF files too. AVIF is about half the size of JPEG but
/// takes about a second for a wide image, so a site asks for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Avif {
    /// WebP and the original's format only.
    #[default]
    Skip,
    /// AVIF first, then WebP and the original's format.
    Make,
}

/// Makes every variant of `original` as [`derive`](fn@derive) does, with the
/// AVIF files first when `avif` asks for them.
///
/// # Errors
///
/// Returns the first [`MediaError`] from decoding, encoding or writing.
pub fn derive_with(
    original: &[u8],
    cache: &MediaCache,
    avif: Avif,
) -> Result<DerivedImage, MediaError> {
    let key = ContentKey::of(original);
    let image = Original::decode(original)?;
    let avif = (avif == Avif::Make).then_some(Format::Avif);
    let variants: Vec<Variant> = avif
        .into_iter()
        .chain([Format::Webp, image.fallback()])
        .flat_map(|format| {
            widths(image.width())
                .into_iter()
                .map(move |width| (format, width))
        })
        .map(|(format, width)| image.variant(format, width))
        .collect();
    let made: Vec<(DerivedFile, bool)> = variants
        .into_par_iter()
        .map(|variant| {
            let path = cache.path(&key, &variant)?;
            if path.is_file() {
                return Ok((DerivedFile { variant, path }, false));
            }
            let bytes = image.encode(&variant)?;
            let path = cache.write(&key, &variant, &bytes)?;
            Ok((DerivedFile { variant, path }, true))
        })
        .collect::<Result<_, MediaError>>()?;
    let encoded = made.iter().filter(|(_, fresh)| *fresh).count();
    Ok(DerivedImage {
        key,
        width: image.width(),
        height: image.height(),
        files: made.into_iter().map(|(file, _)| file).collect(),
        encoded,
    })
}

/// Runs [`derive`](fn@derive) on a blocking thread, so the async runtime keeps
/// serving requests while images are made (RULES.md BL-RUN-7).
///
/// # Errors
///
/// Returns what [`derive`](fn@derive) returns, or [`MediaError::Worker`] if the
/// blocking thread could not finish.
pub async fn derive_in_background(
    original: Vec<u8>,
    cache: MediaCache,
) -> Result<DerivedImage, MediaError> {
    tokio::task::spawn_blocking(move || derive(&original, &cache))
        .await
        .map_err(|source| MediaError::Worker { source })?
}
