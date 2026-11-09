//! Where a site keeps its media library, from the `[media]` table of
//! `site.toml`.

use std::fmt;

use serde::Deserialize;

use crate::{AbsoluteUrl, DomainName, Secret};

/// The media library's settings. Leave the table out to keep every file on
/// disk in the site's `media/` folder. Add `[media.s3]` to keep the files in
/// an S3-compatible bucket, such as Cloudflare R2, instead:
///
/// ```toml
/// [media.s3]
/// endpoint = "https://0c4a2b.r2.cloudflarestorage.com"
/// bucket = "field-notes-media"
/// region = "auto"
/// prefix = "blog/"
/// public_address = "https://media.example.com/"
/// ```
///
/// The bucket's two keys never go in the file. They come from the
/// environment, see [`S3Keys`].
///
/// Photo and GIF search in the editor are off until turned on here. Their
/// keys come from the environment too, `KHALID_UNSPLASH_ACCESS_KEY` and
/// `KHALID_GIPHY_API_KEY`. A chosen picture loads from Unsplash or GIPHY,
/// so readers' browsers fetch it from there:
///
/// ```toml
/// [media]
/// unsplash = true
/// gifs = true
/// ```
///
/// Pictures are offered in WebP and their own format. `avif = true` adds
/// AVIF, which is smaller and slower to make. `avif = "only"` offers AVIF
/// in place of WebP, which nearly halves the pictures a site keeps, see
/// [`AvifSetting`]:
///
/// ```toml
/// [media]
/// avif = "only"
/// ```
///
/// An `::embed` shows a page in a frame only when it is a known player or
/// its host is listed here, such as a calculator the site owner runs:
///
/// ```toml
/// [media]
/// embed_domains = ["tools.example.com"]
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct MediaSettings {
    /// The bucket, or `None` to keep files on disk.
    pub s3: Option<S3Settings>,
    /// Whether writers can find free photos on Unsplash from the editor.
    pub unsplash: bool,
    /// Whether writers can find GIFs on GIPHY from the editor.
    pub gifs: bool,
    /// Whether pictures are offered in AVIF, before WebP or in its place.
    /// AVIF is about half the size of JPEG, but takes about a second to
    /// make for each wide picture, so it is off until a site asks with
    /// `avif = true` or `avif = "only"`. Each picture is made once and kept.
    pub avif: AvifSetting,
    /// The domains whose pages an `::embed` may show in a frame, beside
    /// the video and sound players Khalid knows, such as an interactive
    /// tool on `tools.example.com`. Each is an exact host, and only these
    /// join the Content Security Policy's `frame-src`.
    pub embed_domains: Vec<DomainName>,
}

/// Whether pictures are offered in AVIF, written as `avif` under `[media]`
/// in `site.toml`: `false`, `true` or `"only"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum AvifSetting {
    /// `avif = false`, or no `avif`: WebP and the picture's own format.
    #[default]
    Off,
    /// `avif = true`: AVIF first, then WebP and the picture's own format.
    BesideWebp,
    /// `avif = "only"`: AVIF and the picture's own format, without WebP.
    /// Every current browser reads AVIF; the few that do not get one file
    /// in the picture's own format, at most 1024 px wide, which is also
    /// the image the page shares on social networks.
    Only,
}

impl<'de> Deserialize<'de> for AvifSetting {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(AvifVisitor)
    }
}

/// Reads `avif` as written: a switch, or the word `only`.
struct AvifVisitor;

impl serde::de::Visitor<'_> for AvifVisitor {
    type Value = AvifSetting;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("true, false or \"only\"")
    }

    fn visit_bool<E: serde::de::Error>(self, on: bool) -> Result<AvifSetting, E> {
        Ok(if on {
            AvifSetting::BesideWebp
        } else {
            AvifSetting::Off
        })
    }

    fn visit_str<E: serde::de::Error>(self, word: &str) -> Result<AvifSetting, E> {
        if word == "only" {
            Ok(AvifSetting::Only)
        } else {
            Err(E::invalid_value(serde::de::Unexpected::Str(word), &self))
        }
    }
}

/// An S3-compatible bucket that holds the media library's files.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct S3Settings {
    /// The service's address, such as `https://s3.eu-central-1.amazonaws.com`.
    pub endpoint: AbsoluteUrl,
    /// The bucket's name.
    pub bucket: String,
    /// The region the bucket is in. R2 takes `auto`, the default.
    #[serde(default = "auto_region")]
    pub region: String,
    /// Put before every file's name in the bucket, such as `blog/`, so one
    /// bucket can serve several sites. Empty by default.
    #[serde(default)]
    pub prefix: String,
    /// Where readers fetch the files, such as the bucket's public address
    /// or a CDN in front of it.
    pub public_address: AbsoluteUrl,
}

fn auto_region() -> String {
    "auto".to_owned()
}

impl S3Settings {
    /// The first rule the settings break, as the key it is under and what is
    /// wrong, or `None` when they are fine.
    pub(crate) fn mistake(&self) -> Option<(&'static str, &'static str)> {
        let bucket_name = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-';
        if !(3..=63).contains(&self.bucket.len()) || !self.bucket.chars().all(bucket_name) {
            return Some((
                "bucket",
                "a bucket name is 3 to 63 lowercase letters, digits and hyphens",
            ));
        }
        if self.region.is_empty() || self.region.chars().any(char::is_whitespace) {
            return Some((
                "region",
                "the region is one word, such as auto or eu-central-1",
            ));
        }
        let prefix_ok = self.prefix.is_empty()
            || (self.prefix.ends_with('/')
                && !self.prefix.starts_with('/')
                && self.prefix.split('/').all(|part| part != ".."));
        if !prefix_ok {
            return Some((
                "prefix",
                "the prefix is empty, or a folder name ending in /, such as blog/",
            ));
        }
        None
    }
}

/// The two keys that let Khalid write to the bucket, read from the
/// environment so they never sit in `site.toml` or a backup of it.
#[derive(Debug)]
#[non_exhaustive]
pub struct S3Keys {
    /// The access key's identity, from `KHALID_S3_ACCESS_KEY_ID`.
    pub access_key_id: Secret<String>,
    /// The secret that signs each request, from `KHALID_S3_SECRET_ACCESS_KEY`.
    pub secret_access_key: Secret<String>,
}
