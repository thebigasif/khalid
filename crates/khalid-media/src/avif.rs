//! AVIF encoding with `ravif`, kept apart so the slow encoder's settings
//! live in one place (ADR "AVIF images are made with ravif, when a site
//! asks for them").

use ravif::{Encoder, Img, RGBA8};

use crate::MediaError;

/// The encoder's speed, from 1, slowest, to 10. Speed 8 makes files about
/// 40% smaller than 10 for under three times the time; slower speeds gave
/// almost nothing more on photos.
const SPEED: u8 = 8;

/// `pixels`, RGBA rows `width` by `height`, as an AVIF file at `quality`
/// from 1 to 100.
///
/// # Errors
///
/// [`MediaError::EncodeAvif`] when the size does not match the pixels or
/// the encoder fails.
pub(crate) fn encode(
    pixels: &[u8],
    (width, height): (u32, u32),
    quality: f32,
) -> Result<Vec<u8>, MediaError> {
    let refused = |reason: String| MediaError::EncodeAvif { reason };
    let (rgba, _) = pixels.as_chunks::<4>();
    let rgba: Vec<RGBA8> = rgba
        .iter()
        .map(|&[red, green, blue, alpha]| RGBA8::new(red, green, blue, alpha))
        .collect();
    let width = usize::try_from(width).map_err(|error| refused(error.to_string()))?;
    let height = usize::try_from(height).map_err(|error| refused(error.to_string()))?;
    if width.checked_mul(height) != Some(rgba.len()) {
        return Err(refused("the size does not match the pixels".to_owned()));
    }
    Encoder::new()
        .with_quality(quality)
        .with_speed(SPEED)
        .encode_rgba(Img::new(rgba.as_slice(), width, height))
        .map(|encoded| encoded.avif_file)
        .map_err(|error| refused(error.to_string()))
}
