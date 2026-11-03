//! What kind of file a page is, as the `Content-Type` header says it.

/// The kind of file a [`Page`](crate::Page) holds: text in UTF-8, or an
/// image or other file from beside a post.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum MediaType {
    /// A web page.
    Html,
    /// A sitemap, or any other XML file.
    Xml,
    /// A plain text file, such as `robots.txt` or `llms.txt`.
    PlainText,
    /// A JSON file, such as a JSON feed.
    Json,
    /// A stylesheet.
    Css,
    /// The reader script. [`MediaType::for_file`] never gives it, so a
    /// file a writer put beside a post is never served as a script.
    JavaScript,
    /// A PNG image.
    Png,
    /// A JPEG image.
    Jpeg,
    /// A WebP image.
    Webp,
    /// An AVIF image.
    Avif,
    /// A GIF image.
    Gif,
    /// An SVG drawing, which is text.
    Svg,
    /// A PDF document.
    Pdf,
    /// An MP4 video.
    Mp4,
    /// A `WebM` video.
    Webm,
    /// MP3 audio.
    Mp3,
    /// AAC audio in an MP4 container.
    M4a,
    /// Ogg audio.
    Ogg,
    /// WAV audio.
    Wav,
    /// FLAC audio.
    Flac,
    /// A table of values, such as the data a chart block draws.
    Csv,
    /// A ZIP archive.
    Zip,
    /// A WOFF2 web font, the site's own from `theme/fonts/`.
    Woff2,
    /// Any other file, sent as bytes for the reader to save.
    Binary,
}

impl MediaType {
    /// The value of the `Content-Type` header for this kind.
    pub fn as_header(self) -> &'static str {
        match self {
            Self::Html => "text/html; charset=utf-8",
            Self::Xml => "application/xml; charset=utf-8",
            Self::PlainText => "text/plain; charset=utf-8",
            Self::Json => "application/json",
            Self::Css => "text/css; charset=utf-8",
            Self::JavaScript => "text/javascript; charset=utf-8",
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Webp => "image/webp",
            Self::Avif => "image/avif",
            Self::Gif => "image/gif",
            Self::Svg => "image/svg+xml",
            Self::Pdf => "application/pdf",
            Self::Mp4 => "video/mp4",
            Self::Webm => "video/webm",
            Self::Mp3 => "audio/mpeg",
            Self::M4a => "audio/mp4",
            Self::Ogg => "audio/ogg",
            Self::Wav => "audio/wav",
            Self::Flac => "audio/flac",
            Self::Csv => "text/csv; charset=utf-8",
            Self::Zip => "application/zip",
            Self::Woff2 => "font/woff2",
            Self::Binary => "application/octet-stream",
        }
    }

    /// The kind a file `name` has by its extension, in any case, such as
    /// [`MediaType::Png`] for `chart.PNG`. A name with no extension we
    /// know is [`MediaType::Binary`].
    pub fn for_file(name: &str) -> Self {
        let extension = name
            .rsplit_once('.')
            .map(|(_, extension)| extension.to_ascii_lowercase());
        match extension.as_deref() {
            Some("html" | "htm") => Self::Html,
            Some("xml") => Self::Xml,
            Some("txt" | "md") => Self::PlainText,
            Some("json") => Self::Json,
            Some("css") => Self::Css,
            Some("png") => Self::Png,
            Some("jpg" | "jpeg") => Self::Jpeg,
            Some("webp") => Self::Webp,
            Some("avif") => Self::Avif,
            Some("gif") => Self::Gif,
            Some("svg") => Self::Svg,
            Some("pdf") => Self::Pdf,
            Some("mp4") => Self::Mp4,
            Some("webm") => Self::Webm,
            Some("mp3") => Self::Mp3,
            Some("m4a") => Self::M4a,
            Some("ogg") => Self::Ogg,
            Some("wav") => Self::Wav,
            Some("flac") => Self::Flac,
            Some("csv") => Self::Csv,
            Some("zip") => Self::Zip,
            _ => Self::Binary,
        }
    }

    /// Whether Brotli can make this kind smaller. Images, sound, video,
    /// PDF documents, archives and unknown files are compressed already,
    /// or may be, so they are sent as they are.
    pub(crate) fn compresses(self) -> bool {
        matches!(
            self,
            Self::Html
                | Self::Xml
                | Self::PlainText
                | Self::Json
                | Self::Css
                | Self::JavaScript
                | Self::Svg
                | Self::Csv
        )
    }

    /// Whether a `GET` for this kind may ask for part of it with `Range`,
    /// as a browser does to play a video or to go on with a download.
    /// Text is never sent in part, since its answer may be compressed or
    /// made for one reader.
    pub(crate) fn takes_ranges(self) -> bool {
        !self.compresses()
    }
}
