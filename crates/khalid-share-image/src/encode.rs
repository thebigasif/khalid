//! The drawn card as PNG bytes. A card has one background, one text colour,
//! one accent and the smoothed edges between them, so it nearly always fits
//! in a palette of 256 colours. An indexed PNG keeps every pixel exactly.
//! Against the RGBA PNG it replaced, a card takes about 37% fewer bytes and
//! half the time to draw (docs/standards/performance.md, "Share cards").

use std::collections::BTreeMap;
use std::io::Write;

use png::{BitDepth, ColorType, Compression, Encoder, Filter};
use tiny_skia::Pixmap;

use crate::ShareImageError;

/// Encodes `pixmap` as an indexed PNG when it has at most 256 colours, and
/// as an 8-bit RGB PNG otherwise. A card is drawn opaque, so no alpha
/// channel is written.
///
/// # Errors
///
/// Returns [`ShareImageError::Encode`] when the PNG encoder fails.
pub(crate) fn encode(pixmap: &Pixmap) -> Result<Vec<u8>, ShareImageError> {
    let rgb: Vec<[u8; 3]> = pixmap
        .pixels()
        .iter()
        .map(|pixel| {
            let color = pixel.demultiply();
            [color.red(), color.green(), color.blue()]
        })
        .collect();
    let mut bytes = Vec::new();
    let mut encoder = Encoder::new(&mut bytes, pixmap.width(), pixmap.height());
    encoder.set_depth(BitDepth::Eight);
    encoder.set_compression(Compression::Balanced);
    encoder.set_filter(Filter::Adaptive);
    if let Some((palette, indices)) = palette(&rgb) {
        encoder.set_color(ColorType::Indexed);
        encoder.set_palette(palette);
        write(encoder, &indices)?;
    } else {
        encoder.set_color(ColorType::Rgb);
        write(encoder, rgb.as_flattened())?;
    }
    Ok(bytes)
}

/// The palette, three bytes per colour in order of first use, and each
/// pixel's place in it. `None` when there are more than 256 colours.
fn palette(rgb: &[[u8; 3]]) -> Option<(Vec<u8>, Vec<u8>)> {
    let mut places: BTreeMap<[u8; 3], u8> = BTreeMap::new();
    let mut palette = Vec::new();
    let mut indices = Vec::with_capacity(rgb.len());
    // Most pixels repeat the one before, so that colour is checked first.
    let mut last: Option<([u8; 3], u8)> = None;
    for color in rgb {
        let place = if let Some((_, place)) = last.filter(|(seen, _)| seen == color) {
            place
        } else if let Some(&place) = places.get(color) {
            place
        } else {
            let place = u8::try_from(places.len()).ok()?;
            places.insert(*color, place);
            palette.extend_from_slice(color);
            place
        };
        last = Some((*color, place));
        indices.push(place);
    }
    Some((palette, indices))
}

/// Writes the header and `data` with `encoder`.
fn write<W: Write>(encoder: Encoder<'_, W>, data: &[u8]) -> Result<(), ShareImageError> {
    let mut writer = encoder
        .write_header()
        .map_err(|_| ShareImageError::Encode)?;
    writer
        .write_image_data(data)
        .map_err(|_| ShareImageError::Encode)?;
    writer.finish().map_err(|_| ShareImageError::Encode)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use png::{ColorType, Decoder};
    use tiny_skia::{Color, Pixmap};

    use super::encode;

    /// The colour type and RGB pixels of `png`, palette entries looked up.
    fn decode(png: &[u8]) -> (ColorType, Vec<u8>) {
        let mut reader = Decoder::new(Cursor::new(png)).read_info().unwrap();
        let color_type = reader.info().color_type;
        let palette = reader
            .info()
            .palette
            .as_ref()
            .map(|palette| palette.to_vec());
        let mut buffer = vec![0; reader.output_buffer_size().unwrap()];
        let frame = reader.next_frame(&mut buffer).unwrap();
        buffer.truncate(frame.buffer_size());
        let rgb = match palette {
            Some(palette) => buffer
                .iter()
                .flat_map(|&place| {
                    let start = usize::from(place) * 3;
                    palette[start..start + 3].to_vec()
                })
                .collect(),
            None => buffer,
        };
        (color_type, rgb)
    }

    /// The RGB bytes of every pixel of `pixmap`.
    fn pixels(pixmap: &Pixmap) -> Vec<u8> {
        pixmap
            .pixels()
            .iter()
            .flat_map(|pixel| {
                let color = pixel.demultiply();
                [color.red(), color.green(), color.blue()]
            })
            .collect()
    }

    #[test]
    fn a_card_of_few_colours_is_indexed_and_keeps_every_pixel() {
        let mut pixmap = Pixmap::new(40, 20).unwrap();
        pixmap.fill(Color::from_rgba8(0xfb, 0xf7, 0xf0, 255));
        for (place, pixel) in pixmap.pixels_mut().iter_mut().enumerate().take(300) {
            let shade = u8::try_from(place % 200).unwrap();
            *pixel = tiny_skia::ColorU8::from_rgba(shade, 0x1d, 0x0e, 255).premultiply();
        }

        let png = encode(&pixmap).unwrap();

        let (color_type, rgb) = decode(&png);
        assert_eq!(color_type, ColorType::Indexed);
        assert_eq!(rgb, pixels(&pixmap));
    }

    #[test]
    fn a_card_of_more_than_256_colours_is_rgb_and_keeps_every_pixel() {
        let mut pixmap = Pixmap::new(40, 20).unwrap();
        for (place, pixel) in pixmap.pixels_mut().iter_mut().enumerate() {
            let [low, high, ..] = place.to_le_bytes();
            *pixel = tiny_skia::ColorU8::from_rgba(low, high, 0x80, 255).premultiply();
        }

        let png = encode(&pixmap).unwrap();

        let (color_type, rgb) = decode(&png);
        assert_eq!(color_type, ColorType::Rgb);
        assert_eq!(rgb, pixels(&pixmap));
    }
}
