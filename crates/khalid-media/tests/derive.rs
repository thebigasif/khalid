//! Every size and format of an image, made once and kept by content key.

use std::io::Cursor;

use image::{ImageFormat, Rgb, RgbImage};
use khalid_media::{Avif, Format, MediaCache, derive, derive_in_background, derive_with};

/// A 900 by 600 JPEG photo stand-in, made in the test.
#[cfg(test)]
fn photo(shade: u8) -> Vec<u8> {
    photo_sized(900, 600, shade)
}

/// A JPEG photo stand-in `width` by `height` pixels, made in the test.
#[cfg(test)]
fn photo_sized(width: u32, height: u32, shade: u8) -> Vec<u8> {
    let image = RgbImage::from_fn(width, height, |x, y| {
        Rgb([
            u8::try_from(x % 256).unwrap(),
            u8::try_from(y % 256).unwrap(),
            shade,
        ])
    });
    let mut bytes = Cursor::new(Vec::new());
    image.write_to(&mut bytes, ImageFormat::Jpeg).unwrap();
    bytes.into_inner()
}

#[test]
fn every_width_is_made_in_webp_and_one_in_the_original_format() {
    let site = tempfile::tempdir().unwrap();
    let derived = derive(&photo(90), &MediaCache::new(site.path())).unwrap();
    assert_eq!((derived.width, derived.height), (900, 600));
    let made: Vec<(Format, u32, u32)> = derived
        .files
        .iter()
        .map(|file| (file.variant.format, file.variant.width, file.variant.height))
        .collect();
    assert_eq!(
        made,
        [
            (Format::Webp, 320, 213),
            (Format::Webp, 480, 320),
            (Format::Webp, 640, 427),
            (Format::Webp, 800, 533),
            (Format::Webp, 900, 600),
            (Format::Jpeg, 900, 600),
        ]
    );
    assert_eq!(derived.encoded, 6);
    for file in &derived.files {
        let bytes = std::fs::read(&file.path).unwrap();
        let decoded = image::load_from_memory(&bytes).unwrap();
        assert_eq!(
            decoded.width(),
            file.variant.width,
            "{}",
            file.path.display()
        );
    }
}

#[test]
fn a_site_that_asks_for_avif_gets_every_width_in_avif_first() {
    let site = tempfile::tempdir().unwrap();
    let cache = MediaCache::new(site.path());
    let derived = derive_with(&photo(90), &cache, Avif::Make).unwrap();
    let avif: Vec<u32> = derived
        .files
        .iter()
        .filter(|file| file.variant.format == Format::Avif)
        .map(|file| file.variant.width)
        .collect();
    assert_eq!(avif, [320, 480, 640, 800, 900]);
    assert_eq!(derived.files[0].variant.format, Format::Avif);
    assert_eq!(derived.encoded, 11);
    for file in derived
        .files
        .iter()
        .filter(|f| f.variant.format == Format::Avif)
    {
        let bytes = std::fs::read(&file.path).unwrap();
        // An AVIF file opens with an ISO box naming its brand, `ftypavif`.
        assert_eq!(&bytes[4..12], b"ftypavif", "{}", file.path.display());
        assert!(file.path.to_string_lossy().ends_with(".avif"));
    }
    let again = derive_with(&photo(90), &cache, Avif::Make).unwrap();
    assert_eq!(again.encoded, 0, "every file came from the cache");
    assert_eq!(Format::Avif.media_type(), "image/avif");
}

#[test]
fn a_site_that_asks_for_avif_alone_gets_no_webp_and_one_middle_fallback() {
    let site = tempfile::tempdir().unwrap();
    let cache = MediaCache::new(site.path());
    let derived = derive_with(&photo_sized(1100, 700, 90), &cache, Avif::Only).unwrap();
    let made: Vec<(Format, u32)> = derived
        .files
        .iter()
        .map(|file| (file.variant.format, file.variant.width))
        .collect();
    assert_eq!(
        made,
        [
            (Format::Avif, 320),
            (Format::Avif, 480),
            (Format::Avif, 640),
            (Format::Avif, 800),
            (Format::Avif, 1024),
            (Format::Avif, 1100),
            (Format::Jpeg, 1024),
        ]
    );
    assert_eq!(derived.encoded, 7);
    let jpeg = derived.files.last().unwrap();
    let decoded = image::load_from_memory(&std::fs::read(&jpeg.path).unwrap()).unwrap();
    assert_eq!((decoded.width(), decoded.height()), (1024, 652));
}

#[test]
fn an_unchanged_image_is_never_encoded_twice() {
    let site = tempfile::tempdir().unwrap();
    let cache = MediaCache::new(site.path());
    let first = derive(&photo(90), &cache).unwrap();
    let again = derive(&photo(90), &cache).unwrap();
    assert_eq!(again.encoded, 0, "every file came from the cache");
    assert_eq!(again.files, first.files);
}

#[test]
fn a_changed_image_gets_new_files_beside_the_old() {
    let site = tempfile::tempdir().unwrap();
    let cache = MediaCache::new(site.path());
    let before = derive(&photo(90), &cache).unwrap();
    let after = derive(&photo(200), &cache).unwrap();
    assert_ne!(before.key, after.key);
    assert_eq!(after.encoded, 6);
    assert!(
        before.files.iter().all(|file| file.path.is_file()),
        "old pages keep working"
    );
}

#[tokio::test]
async fn images_are_made_off_the_async_runtime() {
    let site = tempfile::tempdir().unwrap();
    let cache = MediaCache::new(site.path());
    let derived = derive_in_background(photo(90), cache).await.unwrap();
    assert_eq!(derived.encoded, 6);
}

#[test]
fn a_file_that_is_not_an_image_makes_nothing() {
    let site = tempfile::tempdir().unwrap();
    assert!(derive(b"%PDF-1.7 not an image", &MediaCache::new(site.path())).is_err());
    let mut data = site.path().to_path_buf();
    data.push("data");
    assert!(!data.exists(), "nothing was written");
}
